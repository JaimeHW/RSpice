#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30730_e44333, assign30730_e44333_d_n0, assign30730_e44333_d_n2, assign30730_e44333_d_n6, assign30730_e44333_d_n7, assign30730_e44333_d_n10, assign30730_e44333_d_n11, assign30730_e44333_d_n12, assign30730_e44333_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign30730_e44323: f64 = (locals.var_fac1p2__blk934 * locals.var_beta);
        let assign30730_e44325: f64 = (assign30730_e44323 / 2.0);
        let assign30730_e44328: f64 = (locals.var_tx__blk908).sqrt();
        let assign30730_e44329: f64 = (1.0 - assign30730_e44328);
        let assign30730_e44330: f64 = (assign30730_e44325 * assign30730_e44329);
        let assign30730_e44331: f64 = (locals.var_vgpld__blk935 + assign30730_e44330);
        (assign30730_e44331, (locals.var_vgpld__blk935_dn0 + ((((locals.var_fac1p2__blk934_dn0 * locals.var_beta) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn0 / (2.0 * assign30730_e44328)))))), (locals.var_vgpld__blk935_dn2 + ((((locals.var_fac1p2__blk934_dn2 * locals.var_beta) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn2 / (2.0 * assign30730_e44328)))))), (locals.var_vgpld__blk935_dn6 + ((((locals.var_fac1p2__blk934_dn6 * locals.var_beta) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn6 / (2.0 * assign30730_e44328)))))), (locals.var_vgpld__blk935_dn7 + ((((locals.var_fac1p2__blk934_dn7 * locals.var_beta) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn7 / (2.0 * assign30730_e44328)))))), (locals.var_vgpld__blk935_dn10 + (((((locals.var_fac1p2__blk934_dn10 * locals.var_beta) + (locals.var_fac1p2__blk934 * locals.var_beta_dn10)) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn10 / (2.0 * assign30730_e44328)))))), (locals.var_vgpld__blk935_dn11 + ((((locals.var_fac1p2__blk934_dn11 * locals.var_beta) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn11 / (2.0 * assign30730_e44328)))))), (locals.var_vgpld__blk935_dn12 + ((((locals.var_fac1p2__blk934_dn12 * locals.var_beta) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn12 / (2.0 * assign30730_e44328)))))), (locals.var_vgpld__blk935_dn17 + ((((locals.var_fac1p2__blk934_dn17 * locals.var_beta) / 2.0) * assign30730_e44329) + (assign30730_e44325 * (-(locals.var_tx__blk908_dn17 / (2.0 * assign30730_e44328)))))),)
    } else {
        (locals.var_ps0_inia__blk950, locals.var_ps0_inia__blk950_dn0, locals.var_ps0_inia__blk950_dn2, locals.var_ps0_inia__blk950_dn6, locals.var_ps0_inia__blk950_dn7, locals.var_ps0_inia__blk950_dn10, locals.var_ps0_inia__blk950_dn11, locals.var_ps0_inia__blk950_dn12, locals.var_ps0_inia__blk950_dn17,)
    }
};
        locals.var_ps0_inia__blk950 = assign30730_e44333;
        locals.var_ps0_inia__blk950_dn0 = assign30730_e44333_d_n0;
        locals.var_ps0_inia__blk950_dn2 = assign30730_e44333_d_n2;
        locals.var_ps0_inia__blk950_dn6 = assign30730_e44333_d_n6;
        locals.var_ps0_inia__blk950_dn7 = assign30730_e44333_d_n7;
        locals.var_ps0_inia__blk950_dn10 = assign30730_e44333_d_n10;
        locals.var_ps0_inia__blk950_dn11 = assign30730_e44333_d_n11;
        locals.var_ps0_inia__blk950_dn12 = assign30730_e44333_d_n12;
        locals.var_ps0_inia__blk950_dn17 = assign30730_e44333_d_n17;
        locals.var_ps0_inia__blk950_rv = 0.0;

        let (assign30740_e44349, assign30740_e44349_d_n0, assign30740_e44349_d_n2, assign30740_e44349_d_n6, assign30740_e44349_d_n7, assign30740_e44349_d_n10, assign30740_e44349_d_n11, assign30740_e44349_d_n12, assign30740_e44349_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign30740_e44346: f64 = (locals.var_ps0_inia__blk950 + locals.var_vxbgmtcl__blk925);
        let assign30740_e44347: f64 = (locals.var_beta * assign30740_e44346);
        (assign30740_e44347, (locals.var_beta * (locals.var_ps0_inia__blk950_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign30740_e44346) + (locals.var_beta * (locals.var_ps0_inia__blk950_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk950_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign30740_e44349;
        locals.var_chi__blk947_dn0 = assign30740_e44349_d_n0;
        locals.var_chi__blk947_dn2 = assign30740_e44349_d_n2;
        locals.var_chi__blk947_dn6 = assign30740_e44349_d_n6;
        locals.var_chi__blk947_dn7 = assign30740_e44349_d_n7;
        locals.var_chi__blk947_dn10 = assign30740_e44349_d_n10;
        locals.var_chi__blk947_dn11 = assign30740_e44349_d_n11;
        locals.var_chi__blk947_dn12 = assign30740_e44349_d_n12;
        locals.var_chi__blk947_dn17 = assign30740_e44349_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let (assign30750_e44363, assign30750_e44363_d_n0, assign30750_e44363_d_n2, assign30750_e44363_d_n6, assign30750_e44363_d_n7, assign30750_e44363_d_n10, assign30750_e44363_d_n11, assign30750_e44363_d_n12, assign30750_e44363_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign30750_e44360: f64 = (-locals.var_chi__blk947);
        let assign30750_e44361: f64 = (assign30750_e44360).exp();
        (assign30750_e44361, (assign30750_e44361 * (-locals.var_chi__blk947_dn0)), (assign30750_e44361 * (-locals.var_chi__blk947_dn2)), (assign30750_e44361 * (-locals.var_chi__blk947_dn6)), (assign30750_e44361 * (-locals.var_chi__blk947_dn7)), (assign30750_e44361 * (-locals.var_chi__blk947_dn10)), (assign30750_e44361 * (-locals.var_chi__blk947_dn11)), (assign30750_e44361 * (-locals.var_chi__blk947_dn12)), (assign30750_e44361 * (-locals.var_chi__blk947_dn17)),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign30750_e44363;
        locals.var_ty__blk909_dn0 = assign30750_e44363_d_n0;
        locals.var_ty__blk909_dn2 = assign30750_e44363_d_n2;
        locals.var_ty__blk909_dn6 = assign30750_e44363_d_n6;
        locals.var_ty__blk909_dn7 = assign30750_e44363_d_n7;
        locals.var_ty__blk909_dn10 = assign30750_e44363_d_n10;
        locals.var_ty__blk909_dn11 = assign30750_e44363_d_n11;
        locals.var_ty__blk909_dn12 = assign30750_e44363_d_n12;
        locals.var_ty__blk909_dn17 = assign30750_e44363_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign30760_e44391, assign30760_e44391_d_n0, assign30760_e44391_d_n2, assign30760_e44391_d_n6, assign30760_e44391_d_n7, assign30760_e44391_d_n10, assign30760_e44391_d_n11, assign30760_e44391_d_n12, assign30760_e44391_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign30760_e44378: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign30760_e44379: f64 = (locals.var_beta * assign30760_e44378);
        let assign30760_e44381: f64 = (assign30760_e44379 - 1.0);
        let assign30760_e44383: f64 = (assign30760_e44381 + locals.var_ty__blk909);
        let assign30760_e44384: f64 = (4.0 * assign30760_e44383);
        let assign30760_e44387: f64 = (locals.var_fac1p2__blk934 * locals.var_beta2);
        let assign30760_e44388: f64 = (assign30760_e44384 / assign30760_e44387);
        let assign30760_e44389: f64 = (1.0 + assign30760_e44388);
        (assign30760_e44389, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0)) + locals.var_ty__blk909_dn0)) * assign30760_e44387) - (assign30760_e44384 * (locals.var_fac1p2__blk934_dn0 * locals.var_beta2))) / (assign30760_e44387 * assign30760_e44387)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2)) + locals.var_ty__blk909_dn2)) * assign30760_e44387) - (assign30760_e44384 * (locals.var_fac1p2__blk934_dn2 * locals.var_beta2))) / (assign30760_e44387 * assign30760_e44387)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6)) + locals.var_ty__blk909_dn6)) * assign30760_e44387) - (assign30760_e44384 * (locals.var_fac1p2__blk934_dn6 * locals.var_beta2))) / (assign30760_e44387 * assign30760_e44387)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7)) + locals.var_ty__blk909_dn7)) * assign30760_e44387) - (assign30760_e44384 * (locals.var_fac1p2__blk934_dn7 * locals.var_beta2))) / (assign30760_e44387 * assign30760_e44387)), ((((4.0 * (((locals.var_beta_dn10 * assign30760_e44378) + (locals.var_beta * (locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10))) + locals.var_ty__blk909_dn10)) * assign30760_e44387) - (assign30760_e44384 * ((locals.var_fac1p2__blk934_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk934 * locals.var_beta2_dn10)))) / (assign30760_e44387 * assign30760_e44387)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11)) + locals.var_ty__blk909_dn11)) * assign30760_e44387) - (assign30760_e44384 * (locals.var_fac1p2__blk934_dn11 * locals.var_beta2))) / (assign30760_e44387 * assign30760_e44387)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12)) + locals.var_ty__blk909_dn12)) * assign30760_e44387) - (assign30760_e44384 * (locals.var_fac1p2__blk934_dn12 * locals.var_beta2))) / (assign30760_e44387 * assign30760_e44387)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17)) + locals.var_ty__blk909_dn17)) * assign30760_e44387) - (assign30760_e44384 * (locals.var_fac1p2__blk934_dn17 * locals.var_beta2))) / (assign30760_e44387 * assign30760_e44387)),)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign30760_e44391;
        locals.var_tx__blk908_dn0 = assign30760_e44391_d_n0;
        locals.var_tx__blk908_dn2 = assign30760_e44391_d_n2;
        locals.var_tx__blk908_dn6 = assign30760_e44391_d_n6;
        locals.var_tx__blk908_dn7 = assign30760_e44391_d_n7;
        locals.var_tx__blk908_dn10 = assign30760_e44391_d_n10;
        locals.var_tx__blk908_dn11 = assign30760_e44391_d_n11;
        locals.var_tx__blk908_dn12 = assign30760_e44391_d_n12;
        locals.var_tx__blk908_dn17 = assign30760_e44391_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let assign30770_e44395: f64 = (10.0 * 2.220446049250313e-16);
        let assign30770_e44396: f64 = if locals.var_tx__blk908 < assign30770_e44395 { 1.0 } else { 0.0 };
        locals.var_guard1009 = assign30770_e44396;
        locals.var_guard1009_rv = 0.0;

        let (assign30780_e44412, assign30780_e44412_d_n0, assign30780_e44412_d_n2, assign30780_e44412_d_n6, assign30780_e44412_d_n7, assign30780_e44412_d_n10, assign30780_e44412_d_n11, assign30780_e44412_d_n12, assign30780_e44412_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        let assign30780_e44410: f64 = (10.0 * 2.220446049250313e-16);
        (assign30780_e44410, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign30780_e44412;
        locals.var_tx__blk908_dn0 = assign30780_e44412_d_n0;
        locals.var_tx__blk908_dn2 = assign30780_e44412_d_n2;
        locals.var_tx__blk908_dn6 = assign30780_e44412_d_n6;
        locals.var_tx__blk908_dn7 = assign30780_e44412_d_n7;
        locals.var_tx__blk908_dn10 = assign30780_e44412_d_n10;
        locals.var_tx__blk908_dn11 = assign30780_e44412_d_n11;
        locals.var_tx__blk908_dn12 = assign30780_e44412_d_n12;
        locals.var_tx__blk908_dn17 = assign30780_e44412_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign30790_e44435, assign30790_e44435_d_n0, assign30790_e44435_d_n2, assign30790_e44435_d_n6, assign30790_e44435_d_n7, assign30790_e44435_d_n10, assign30790_e44435_d_n11, assign30790_e44435_d_n12, assign30790_e44435_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign30790_e44425: f64 = (locals.var_fac1p2__blk934 * locals.var_beta);
        let assign30790_e44427: f64 = (assign30790_e44425 / 2.0);
        let assign30790_e44430: f64 = (locals.var_tx__blk908).sqrt();
        let assign30790_e44431: f64 = (1.0 - assign30790_e44430);
        let assign30790_e44432: f64 = (assign30790_e44427 * assign30790_e44431);
        let assign30790_e44433: f64 = (locals.var_vgpld__blk935 + assign30790_e44432);
        (assign30790_e44433, (locals.var_vgpld__blk935_dn0 + ((((locals.var_fac1p2__blk934_dn0 * locals.var_beta) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn0 / (2.0 * assign30790_e44430)))))), (locals.var_vgpld__blk935_dn2 + ((((locals.var_fac1p2__blk934_dn2 * locals.var_beta) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn2 / (2.0 * assign30790_e44430)))))), (locals.var_vgpld__blk935_dn6 + ((((locals.var_fac1p2__blk934_dn6 * locals.var_beta) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn6 / (2.0 * assign30790_e44430)))))), (locals.var_vgpld__blk935_dn7 + ((((locals.var_fac1p2__blk934_dn7 * locals.var_beta) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn7 / (2.0 * assign30790_e44430)))))), (locals.var_vgpld__blk935_dn10 + (((((locals.var_fac1p2__blk934_dn10 * locals.var_beta) + (locals.var_fac1p2__blk934 * locals.var_beta_dn10)) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn10 / (2.0 * assign30790_e44430)))))), (locals.var_vgpld__blk935_dn11 + ((((locals.var_fac1p2__blk934_dn11 * locals.var_beta) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn11 / (2.0 * assign30790_e44430)))))), (locals.var_vgpld__blk935_dn12 + ((((locals.var_fac1p2__blk934_dn12 * locals.var_beta) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn12 / (2.0 * assign30790_e44430)))))), (locals.var_vgpld__blk935_dn17 + ((((locals.var_fac1p2__blk934_dn17 * locals.var_beta) / 2.0) * assign30790_e44431) + (assign30790_e44427 * (-(locals.var_tx__blk908_dn17 / (2.0 * assign30790_e44430)))))),)
    } else {
        (locals.var_ps0_inia__blk950, locals.var_ps0_inia__blk950_dn0, locals.var_ps0_inia__blk950_dn2, locals.var_ps0_inia__blk950_dn6, locals.var_ps0_inia__blk950_dn7, locals.var_ps0_inia__blk950_dn10, locals.var_ps0_inia__blk950_dn11, locals.var_ps0_inia__blk950_dn12, locals.var_ps0_inia__blk950_dn17,)
    }
};
        locals.var_ps0_inia__blk950 = assign30790_e44435;
        locals.var_ps0_inia__blk950_dn0 = assign30790_e44435_d_n0;
        locals.var_ps0_inia__blk950_dn2 = assign30790_e44435_d_n2;
        locals.var_ps0_inia__blk950_dn6 = assign30790_e44435_d_n6;
        locals.var_ps0_inia__blk950_dn7 = assign30790_e44435_d_n7;
        locals.var_ps0_inia__blk950_dn10 = assign30790_e44435_d_n10;
        locals.var_ps0_inia__blk950_dn11 = assign30790_e44435_d_n11;
        locals.var_ps0_inia__blk950_dn12 = assign30790_e44435_d_n12;
        locals.var_ps0_inia__blk950_dn17 = assign30790_e44435_d_n17;
        locals.var_ps0_inia__blk950_rv = 0.0;

        let (assign30800_e44451, assign30800_e44451_d_n0, assign30800_e44451_d_n2, assign30800_e44451_d_n6, assign30800_e44451_d_n7, assign30800_e44451_d_n10, assign30800_e44451_d_n11, assign30800_e44451_d_n12, assign30800_e44451_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign30800_e44448: f64 = (locals.var_ps0_inia__blk950 + locals.var_vxbgmtcl__blk925);
        let assign30800_e44449: f64 = (locals.var_beta * assign30800_e44448);
        (assign30800_e44449, (locals.var_beta * (locals.var_ps0_inia__blk950_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign30800_e44448) + (locals.var_beta * (locals.var_ps0_inia__blk950_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk950_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign30800_e44451;
        locals.var_chi__blk947_dn0 = assign30800_e44451_d_n0;
        locals.var_chi__blk947_dn2 = assign30800_e44451_d_n2;
        locals.var_chi__blk947_dn6 = assign30800_e44451_d_n6;
        locals.var_chi__blk947_dn7 = assign30800_e44451_d_n7;
        locals.var_chi__blk947_dn10 = assign30800_e44451_d_n10;
        locals.var_chi__blk947_dn11 = assign30800_e44451_d_n11;
        locals.var_chi__blk947_dn12 = assign30800_e44451_d_n12;
        locals.var_chi__blk947_dn17 = assign30800_e44451_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let assign30810_e44454: f64 = if locals.var_chi__blk947 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1010 = assign30810_e44454;
        locals.var_guard1010_rv = 0.0;

        let (assign30830_e44499,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30830_e44483: f64 = (9.0 * 1.414213562373095);
        let assign30830_e44484: f64 = (1.0 / assign30830_e44483);
        let assign30830_e44488: f64 = (7.0 * 0.049787068367863944);
        let assign30830_e44489: f64 = (5.0 + assign30830_e44488);
        let assign30830_e44493: f64 = (2.0 + 0.049787068367863944);
        let assign30830_e44494: f64 = (assign30830_e44493).sqrt();
        let assign30830_e44495: f64 = (54.0 * assign30830_e44494);
        let assign30830_e44496: f64 = (assign30830_e44489 / assign30830_e44495);
        let assign30830_e44497: f64 = (assign30830_e44484 - assign30830_e44496);
        (assign30830_e44497,)
    } else {
        (locals.var_ta__blk951,)
    }
};
        locals.var_ta__blk951 = assign30830_e44499;
        locals.var_ta__blk951_rv = 0.0;

        let (assign30840_e44526,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30840_e44513: f64 = (1.0 + 0.049787068367863944);
        let assign30840_e44517: f64 = (2.0 + 0.049787068367863944);
        let assign30840_e44518: f64 = (assign30840_e44517).sqrt();
        let assign30840_e44519: f64 = (2.0 * assign30840_e44518);
        let assign30840_e44520: f64 = (assign30840_e44513 / assign30840_e44519);
        let assign30840_e44523: f64 = (1.414213562373095 / 3.0);
        let assign30840_e44524: f64 = (assign30840_e44520 - assign30840_e44523);
        (assign30840_e44524,)
    } else {
        (locals.var_tb__blk952,)
    }
};
        locals.var_tb__blk952 = assign30840_e44526;
        locals.var_tb__blk952_rv = 0.0;

        let (assign30850_e44548, assign30850_e44548_d_n0, assign30850_e44548_d_n2, assign30850_e44548_d_n6, assign30850_e44548_d_n7, assign30850_e44548_d_n10, assign30850_e44548_d_n11, assign30850_e44548_d_n12, assign30850_e44548_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30850_e44540: f64 = (1.0 / 1.414213562373095);
        let assign30850_e44544: f64 = (locals.var_beta * locals.var_fac1__blk933);
        let assign30850_e44545: f64 = (1.0 / assign30850_e44544);
        let assign30850_e44546: f64 = (assign30850_e44540 + assign30850_e44545);
        (assign30850_e44546, (-((locals.var_beta * locals.var_fac1__blk933_dn0) / (assign30850_e44544 * assign30850_e44544))), (-((locals.var_beta * locals.var_fac1__blk933_dn2) / (assign30850_e44544 * assign30850_e44544))), (-((locals.var_beta * locals.var_fac1__blk933_dn6) / (assign30850_e44544 * assign30850_e44544))), (-((locals.var_beta * locals.var_fac1__blk933_dn7) / (assign30850_e44544 * assign30850_e44544))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk933) + (locals.var_beta * locals.var_fac1__blk933_dn10)) / (assign30850_e44544 * assign30850_e44544))), (-((locals.var_beta * locals.var_fac1__blk933_dn11) / (assign30850_e44544 * assign30850_e44544))), (-((locals.var_beta * locals.var_fac1__blk933_dn12) / (assign30850_e44544 * assign30850_e44544))), (-((locals.var_beta * locals.var_fac1__blk933_dn17) / (assign30850_e44544 * assign30850_e44544))),)
    } else {
        (locals.var_tc__blk953, locals.var_tc__blk953_dn0, locals.var_tc__blk953_dn2, locals.var_tc__blk953_dn6, locals.var_tc__blk953_dn7, locals.var_tc__blk953_dn10, locals.var_tc__blk953_dn11, locals.var_tc__blk953_dn12, locals.var_tc__blk953_dn17,)
    }
};
        locals.var_tc__blk953 = assign30850_e44548;
        locals.var_tc__blk953_dn0 = assign30850_e44548_d_n0;
        locals.var_tc__blk953_dn2 = assign30850_e44548_d_n2;
        locals.var_tc__blk953_dn6 = assign30850_e44548_d_n6;
        locals.var_tc__blk953_dn7 = assign30850_e44548_d_n7;
        locals.var_tc__blk953_dn10 = assign30850_e44548_d_n10;
        locals.var_tc__blk953_dn11 = assign30850_e44548_d_n11;
        locals.var_tc__blk953_dn12 = assign30850_e44548_d_n12;
        locals.var_tc__blk953_dn17 = assign30850_e44548_d_n17;
        locals.var_tc__blk953_rv = 0.0;

        let (assign30860_e44567, assign30860_e44567_d_n0, assign30860_e44567_d_n2, assign30860_e44567_d_n6, assign30860_e44567_d_n7, assign30860_e44567_d_n10, assign30860_e44567_d_n11, assign30860_e44567_d_n12, assign30860_e44567_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30860_e44562: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign30860_e44563: f64 = (-assign30860_e44562);
        let assign30860_e44565: f64 = (assign30860_e44563 / locals.var_fac1__blk933);
        (assign30860_e44565, ((((-(locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn0)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn2)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn6)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn7)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn10)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn11)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn12)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17)) * locals.var_fac1__blk933) - (assign30860_e44563 * locals.var_fac1__blk933_dn17)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)),)
    } else {
        (locals.var_td__blk954, locals.var_td__blk954_dn0, locals.var_td__blk954_dn2, locals.var_td__blk954_dn6, locals.var_td__blk954_dn7, locals.var_td__blk954_dn10, locals.var_td__blk954_dn11, locals.var_td__blk954_dn12, locals.var_td__blk954_dn17,)
    }
};
        locals.var_td__blk954 = assign30860_e44567;
        locals.var_td__blk954_dn0 = assign30860_e44567_d_n0;
        locals.var_td__blk954_dn2 = assign30860_e44567_d_n2;
        locals.var_td__blk954_dn6 = assign30860_e44567_d_n6;
        locals.var_td__blk954_dn7 = assign30860_e44567_d_n7;
        locals.var_td__blk954_dn10 = assign30860_e44567_d_n10;
        locals.var_td__blk954_dn11 = assign30860_e44567_d_n11;
        locals.var_td__blk954_dn12 = assign30860_e44567_d_n12;
        locals.var_td__blk954_dn17 = assign30860_e44567_d_n17;
        locals.var_td__blk954_rv = 0.0;

        let (assign30870_e44609, assign30870_e44609_d_n0, assign30870_e44609_d_n2, assign30870_e44609_d_n6, assign30870_e44609_d_n7, assign30870_e44609_d_n10, assign30870_e44609_d_n11, assign30870_e44609_d_n12, assign30870_e44609_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30870_e44581: f64 = (locals.var_tb__blk952 * locals.var_tb__blk952);
        let assign30870_e44583: f64 = (assign30870_e44581 * locals.var_tb__blk952);
        let assign30870_e44586: f64 = (27.0 * locals.var_ta__blk951);
        let assign30870_e44588: f64 = (assign30870_e44586 * locals.var_ta__blk951);
        let assign30870_e44590: f64 = (assign30870_e44588 * locals.var_ta__blk951);
        let assign30870_e44591: f64 = (assign30870_e44583 / assign30870_e44590);
        let assign30870_e44594: f64 = (locals.var_tb__blk952 * locals.var_tc__blk953);
        let assign30870_e44597: f64 = (6.0 * locals.var_ta__blk951);
        let assign30870_e44599: f64 = (assign30870_e44597 * locals.var_ta__blk951);
        let assign30870_e44600: f64 = (assign30870_e44594 / assign30870_e44599);
        let assign30870_e44601: f64 = (assign30870_e44591 - assign30870_e44600);
        let assign30870_e44605: f64 = (2.0 * locals.var_ta__blk951);
        let assign30870_e44606: f64 = (locals.var_td__blk954 / assign30870_e44605);
        let assign30870_e44607: f64 = (assign30870_e44601 + assign30870_e44606);
        (assign30870_e44607, ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn0) / assign30870_e44599)) + (locals.var_td__blk954_dn0 / assign30870_e44605)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn2) / assign30870_e44599)) + (locals.var_td__blk954_dn2 / assign30870_e44605)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn6) / assign30870_e44599)) + (locals.var_td__blk954_dn6 / assign30870_e44605)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn7) / assign30870_e44599)) + (locals.var_td__blk954_dn7 / assign30870_e44605)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn10) / assign30870_e44599)) + (locals.var_td__blk954_dn10 / assign30870_e44605)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn11) / assign30870_e44599)) + (locals.var_td__blk954_dn11 / assign30870_e44605)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn12) / assign30870_e44599)) + (locals.var_td__blk954_dn12 / assign30870_e44605)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn17) / assign30870_e44599)) + (locals.var_td__blk954_dn17 / assign30870_e44605)),)
    } else {
        (locals.var_tq__blk955, locals.var_tq__blk955_dn0, locals.var_tq__blk955_dn2, locals.var_tq__blk955_dn6, locals.var_tq__blk955_dn7, locals.var_tq__blk955_dn10, locals.var_tq__blk955_dn11, locals.var_tq__blk955_dn12, locals.var_tq__blk955_dn17,)
    }
};
        locals.var_tq__blk955 = assign30870_e44609;
        locals.var_tq__blk955_dn0 = assign30870_e44609_d_n0;
        locals.var_tq__blk955_dn2 = assign30870_e44609_d_n2;
        locals.var_tq__blk955_dn6 = assign30870_e44609_d_n6;
        locals.var_tq__blk955_dn7 = assign30870_e44609_d_n7;
        locals.var_tq__blk955_dn10 = assign30870_e44609_d_n10;
        locals.var_tq__blk955_dn11 = assign30870_e44609_d_n11;
        locals.var_tq__blk955_dn12 = assign30870_e44609_d_n12;
        locals.var_tq__blk955_dn17 = assign30870_e44609_d_n17;
        locals.var_tq__blk955_rv = 0.0;

        let (assign30880_e44637, assign30880_e44637_d_n0, assign30880_e44637_d_n2, assign30880_e44637_d_n6, assign30880_e44637_d_n7, assign30880_e44637_d_n10, assign30880_e44637_d_n11, assign30880_e44637_d_n12, assign30880_e44637_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30880_e44623: f64 = (3.0 * locals.var_ta__blk951);
        let assign30880_e44625: f64 = (assign30880_e44623 * locals.var_tc__blk953);
        let assign30880_e44628: f64 = (locals.var_tb__blk952 * locals.var_tb__blk952);
        let assign30880_e44629: f64 = (assign30880_e44625 - assign30880_e44628);
        let assign30880_e44632: f64 = (9.0 * locals.var_ta__blk951);
        let assign30880_e44634: f64 = (assign30880_e44632 * locals.var_ta__blk951);
        let assign30880_e44635: f64 = (assign30880_e44629 / assign30880_e44634);
        (assign30880_e44635, ((assign30880_e44623 * locals.var_tc__blk953_dn0) / assign30880_e44634), ((assign30880_e44623 * locals.var_tc__blk953_dn2) / assign30880_e44634), ((assign30880_e44623 * locals.var_tc__blk953_dn6) / assign30880_e44634), ((assign30880_e44623 * locals.var_tc__blk953_dn7) / assign30880_e44634), ((assign30880_e44623 * locals.var_tc__blk953_dn10) / assign30880_e44634), ((assign30880_e44623 * locals.var_tc__blk953_dn11) / assign30880_e44634), ((assign30880_e44623 * locals.var_tc__blk953_dn12) / assign30880_e44634), ((assign30880_e44623 * locals.var_tc__blk953_dn17) / assign30880_e44634),)
    } else {
        (locals.var_tp__blk956, locals.var_tp__blk956_dn0, locals.var_tp__blk956_dn2, locals.var_tp__blk956_dn6, locals.var_tp__blk956_dn7, locals.var_tp__blk956_dn10, locals.var_tp__blk956_dn11, locals.var_tp__blk956_dn12, locals.var_tp__blk956_dn17,)
    }
};
        locals.var_tp__blk956 = assign30880_e44637;
        locals.var_tp__blk956_dn0 = assign30880_e44637_d_n0;
        locals.var_tp__blk956_dn2 = assign30880_e44637_d_n2;
        locals.var_tp__blk956_dn6 = assign30880_e44637_d_n6;
        locals.var_tp__blk956_dn7 = assign30880_e44637_d_n7;
        locals.var_tp__blk956_dn10 = assign30880_e44637_d_n10;
        locals.var_tp__blk956_dn11 = assign30880_e44637_d_n11;
        locals.var_tp__blk956_dn12 = assign30880_e44637_d_n12;
        locals.var_tp__blk956_dn17 = assign30880_e44637_d_n17;
        locals.var_tp__blk956_rv = 0.0;

        let (assign30890_e44660, assign30890_e44660_d_n0, assign30890_e44660_d_n2, assign30890_e44660_d_n6, assign30890_e44660_d_n7, assign30890_e44660_d_n10, assign30890_e44660_d_n11, assign30890_e44660_d_n12, assign30890_e44660_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30890_e44651: f64 = (locals.var_tq__blk955 * locals.var_tq__blk955);
        let assign30890_e44654: f64 = (locals.var_tp__blk956 * locals.var_tp__blk956);
        let assign30890_e44656: f64 = (assign30890_e44654 * locals.var_tp__blk956);
        let assign30890_e44657: f64 = (assign30890_e44651 + assign30890_e44656);
        let assign30890_e44658: f64 = (assign30890_e44657).sqrt();
        (assign30890_e44658, ((((locals.var_tq__blk955_dn0 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn0)) + ((((locals.var_tp__blk956_dn0 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn0)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn0))) / (2.0 * assign30890_e44658)), ((((locals.var_tq__blk955_dn2 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn2)) + ((((locals.var_tp__blk956_dn2 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn2)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn2))) / (2.0 * assign30890_e44658)), ((((locals.var_tq__blk955_dn6 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn6)) + ((((locals.var_tp__blk956_dn6 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn6)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn6))) / (2.0 * assign30890_e44658)), ((((locals.var_tq__blk955_dn7 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn7)) + ((((locals.var_tp__blk956_dn7 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn7)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn7))) / (2.0 * assign30890_e44658)), ((((locals.var_tq__blk955_dn10 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn10)) + ((((locals.var_tp__blk956_dn10 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn10)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn10))) / (2.0 * assign30890_e44658)), ((((locals.var_tq__blk955_dn11 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn11)) + ((((locals.var_tp__blk956_dn11 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn11)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn11))) / (2.0 * assign30890_e44658)), ((((locals.var_tq__blk955_dn12 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn12)) + ((((locals.var_tp__blk956_dn12 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn12)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn12))) / (2.0 * assign30890_e44658)), ((((locals.var_tq__blk955_dn17 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn17)) + ((((locals.var_tp__blk956_dn17 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn17)) * locals.var_tp__blk956) + (assign30890_e44654 * locals.var_tp__blk956_dn17))) / (2.0 * assign30890_e44658)),)
    } else {
        (locals.var_t5__blk904, locals.var_t5__blk904_dn0, locals.var_t5__blk904_dn2, locals.var_t5__blk904_dn6, locals.var_t5__blk904_dn7, locals.var_t5__blk904_dn10, locals.var_t5__blk904_dn11, locals.var_t5__blk904_dn12, locals.var_t5__blk904_dn17,)
    }
};
        locals.var_t5__blk904 = assign30890_e44660;
        locals.var_t5__blk904_dn0 = assign30890_e44660_d_n0;
        locals.var_t5__blk904_dn2 = assign30890_e44660_d_n2;
        locals.var_t5__blk904_dn6 = assign30890_e44660_d_n6;
        locals.var_t5__blk904_dn7 = assign30890_e44660_d_n7;
        locals.var_t5__blk904_dn10 = assign30890_e44660_d_n10;
        locals.var_t5__blk904_dn11 = assign30890_e44660_d_n11;
        locals.var_t5__blk904_dn12 = assign30890_e44660_d_n12;
        locals.var_t5__blk904_dn17 = assign30890_e44660_d_n17;
        locals.var_t5__blk904_rv = 0.0;

        let (assign30900_e44679, assign30900_e44679_d_n0, assign30900_e44679_d_n2, assign30900_e44679_d_n6, assign30900_e44679_d_n7, assign30900_e44679_d_n10, assign30900_e44679_d_n11, assign30900_e44679_d_n12, assign30900_e44679_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30900_e44673: f64 = (-locals.var_tq__blk955);
        let assign30900_e44675: f64 = (assign30900_e44673 + locals.var_t5__blk904);
        let assign30900_e44677: f64 = (assign30900_e44675).powf(0.3333333333333333);
        (assign30900_e44677, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn0) + locals.var_t5__blk904_dn0))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn0) + locals.var_t5__blk904_dn0) / assign30900_e44675))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn2) + locals.var_t5__blk904_dn2))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn2) + locals.var_t5__blk904_dn2) / assign30900_e44675))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn6) + locals.var_t5__blk904_dn6))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn6) + locals.var_t5__blk904_dn6) / assign30900_e44675))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn7) + locals.var_t5__blk904_dn7))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn7) + locals.var_t5__blk904_dn7) / assign30900_e44675))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn10) + locals.var_t5__blk904_dn10))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn10) + locals.var_t5__blk904_dn10) / assign30900_e44675))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn11) + locals.var_t5__blk904_dn11))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn11) + locals.var_t5__blk904_dn11) / assign30900_e44675))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn12) + locals.var_t5__blk904_dn12))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn12) + locals.var_t5__blk904_dn12) / assign30900_e44675))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30900_e44675).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn17) + locals.var_t5__blk904_dn17))) } } else { (assign30900_e44677 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn17) + locals.var_t5__blk904_dn17) / assign30900_e44675))) },)
    } else {
        (locals.var_tu__blk957, locals.var_tu__blk957_dn0, locals.var_tu__blk957_dn2, locals.var_tu__blk957_dn6, locals.var_tu__blk957_dn7, locals.var_tu__blk957_dn10, locals.var_tu__blk957_dn11, locals.var_tu__blk957_dn12, locals.var_tu__blk957_dn17,)
    }
};
        locals.var_tu__blk957 = assign30900_e44679;
        locals.var_tu__blk957_dn0 = assign30900_e44679_d_n0;
        locals.var_tu__blk957_dn2 = assign30900_e44679_d_n2;
        locals.var_tu__blk957_dn6 = assign30900_e44679_d_n6;
        locals.var_tu__blk957_dn7 = assign30900_e44679_d_n7;
        locals.var_tu__blk957_dn10 = assign30900_e44679_d_n10;
        locals.var_tu__blk957_dn11 = assign30900_e44679_d_n11;
        locals.var_tu__blk957_dn12 = assign30900_e44679_d_n12;
        locals.var_tu__blk957_dn17 = assign30900_e44679_d_n17;
        locals.var_tu__blk957_rv = 0.0;

        let (assign30910_e44698, assign30910_e44698_d_n0, assign30910_e44698_d_n2, assign30910_e44698_d_n6, assign30910_e44698_d_n7, assign30910_e44698_d_n10, assign30910_e44698_d_n11, assign30910_e44698_d_n12, assign30910_e44698_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30910_e44693: f64 = (locals.var_tq__blk955 + locals.var_t5__blk904);
        let assign30910_e44695: f64 = (assign30910_e44693).powf(0.3333333333333333);
        let assign30910_e44696: f64 = (-assign30910_e44695);
        (assign30910_e44696, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn0 + locals.var_t5__blk904_dn0))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn0 + locals.var_t5__blk904_dn0) / assign30910_e44693))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn2 + locals.var_t5__blk904_dn2))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn2 + locals.var_t5__blk904_dn2) / assign30910_e44693))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn6 + locals.var_t5__blk904_dn6))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn6 + locals.var_t5__blk904_dn6) / assign30910_e44693))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn7 + locals.var_t5__blk904_dn7))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn7 + locals.var_t5__blk904_dn7) / assign30910_e44693))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn10 + locals.var_t5__blk904_dn10))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn10 + locals.var_t5__blk904_dn10) / assign30910_e44693))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn11 + locals.var_t5__blk904_dn11))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn11 + locals.var_t5__blk904_dn11) / assign30910_e44693))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn12 + locals.var_t5__blk904_dn12))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn12 + locals.var_t5__blk904_dn12) / assign30910_e44693))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30910_e44693).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn17 + locals.var_t5__blk904_dn17))) } } else { (assign30910_e44695 * (0.3333333333333333 * ((locals.var_tq__blk955_dn17 + locals.var_t5__blk904_dn17) / assign30910_e44693))) }),)
    } else {
        (locals.var_tv__blk958, locals.var_tv__blk958_dn0, locals.var_tv__blk958_dn2, locals.var_tv__blk958_dn6, locals.var_tv__blk958_dn7, locals.var_tv__blk958_dn10, locals.var_tv__blk958_dn11, locals.var_tv__blk958_dn12, locals.var_tv__blk958_dn17,)
    }
};
        locals.var_tv__blk958 = assign30910_e44698;
        locals.var_tv__blk958_dn0 = assign30910_e44698_d_n0;
        locals.var_tv__blk958_dn2 = assign30910_e44698_d_n2;
        locals.var_tv__blk958_dn6 = assign30910_e44698_d_n6;
        locals.var_tv__blk958_dn7 = assign30910_e44698_d_n7;
        locals.var_tv__blk958_dn10 = assign30910_e44698_d_n10;
        locals.var_tv__blk958_dn11 = assign30910_e44698_d_n11;
        locals.var_tv__blk958_dn12 = assign30910_e44698_d_n12;
        locals.var_tv__blk958_dn17 = assign30910_e44698_d_n17;
        locals.var_tv__blk958_rv = 0.0;

        let (assign30920_e44720, assign30920_e44720_d_n0, assign30920_e44720_d_n2, assign30920_e44720_d_n6, assign30920_e44720_d_n7, assign30920_e44720_d_n10, assign30920_e44720_d_n11, assign30920_e44720_d_n12, assign30920_e44720_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30920_e44712: f64 = (locals.var_tu__blk957 + locals.var_tv__blk958);
        let assign30920_e44716: f64 = (3.0 * locals.var_ta__blk951);
        let assign30920_e44717: f64 = (locals.var_tb__blk952 / assign30920_e44716);
        let assign30920_e44718: f64 = (assign30920_e44712 - assign30920_e44717);
        (assign30920_e44718, (locals.var_tu__blk957_dn0 + locals.var_tv__blk958_dn0), (locals.var_tu__blk957_dn2 + locals.var_tv__blk958_dn2), (locals.var_tu__blk957_dn6 + locals.var_tv__blk958_dn6), (locals.var_tu__blk957_dn7 + locals.var_tv__blk958_dn7), (locals.var_tu__blk957_dn10 + locals.var_tv__blk958_dn10), (locals.var_tu__blk957_dn11 + locals.var_tv__blk958_dn11), (locals.var_tu__blk957_dn12 + locals.var_tv__blk958_dn12), (locals.var_tu__blk957_dn17 + locals.var_tv__blk958_dn17),)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign30920_e44720;
        locals.var_tx__blk908_dn0 = assign30920_e44720_d_n0;
        locals.var_tx__blk908_dn2 = assign30920_e44720_d_n2;
        locals.var_tx__blk908_dn6 = assign30920_e44720_d_n6;
        locals.var_tx__blk908_dn7 = assign30920_e44720_d_n7;
        locals.var_tx__blk908_dn10 = assign30920_e44720_d_n10;
        locals.var_tx__blk908_dn11 = assign30920_e44720_d_n11;
        locals.var_tx__blk908_dn12 = assign30920_e44720_d_n12;
        locals.var_tx__blk908_dn17 = assign30920_e44720_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign30930_e44738, assign30930_e44738_d_n0, assign30930_e44738_d_n2, assign30930_e44738_d_n6, assign30930_e44738_d_n7, assign30930_e44738_d_n10, assign30930_e44738_d_n11, assign30930_e44738_d_n12, assign30930_e44738_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30930_e44734: f64 = (locals.var_tx__blk908 * locals.var_beta_inv);
        let assign30930_e44736: f64 = (assign30930_e44734 - locals.var_vxbgmtcl__blk925);
        (assign30930_e44736, ((locals.var_tx__blk908_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn0), ((locals.var_tx__blk908_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn2), ((locals.var_tx__blk908_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn6), ((locals.var_tx__blk908_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn7), (((locals.var_tx__blk908_dn10 * locals.var_beta_inv) + (locals.var_tx__blk908 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk925_dn10), ((locals.var_tx__blk908_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn11), ((locals.var_tx__blk908_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn12), ((locals.var_tx__blk908_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_ps0_inia__blk950, locals.var_ps0_inia__blk950_dn0, locals.var_ps0_inia__blk950_dn2, locals.var_ps0_inia__blk950_dn6, locals.var_ps0_inia__blk950_dn7, locals.var_ps0_inia__blk950_dn10, locals.var_ps0_inia__blk950_dn11, locals.var_ps0_inia__blk950_dn12, locals.var_ps0_inia__blk950_dn17,)
    }
};
        locals.var_ps0_inia__blk950 = assign30930_e44738;
        locals.var_ps0_inia__blk950_dn0 = assign30930_e44738_d_n0;
        locals.var_ps0_inia__blk950_dn2 = assign30930_e44738_d_n2;
        locals.var_ps0_inia__blk950_dn6 = assign30930_e44738_d_n6;
        locals.var_ps0_inia__blk950_dn7 = assign30930_e44738_d_n7;
        locals.var_ps0_inia__blk950_dn10 = assign30930_e44738_d_n10;
        locals.var_ps0_inia__blk950_dn11 = assign30930_e44738_d_n11;
        locals.var_ps0_inia__blk950_dn12 = assign30930_e44738_d_n12;
        locals.var_ps0_inia__blk950_dn17 = assign30930_e44738_d_n17;
        locals.var_ps0_inia__blk950_rv = 0.0;

        let (assign30940_e44756, assign30940_e44756_d_n0, assign30940_e44756_d_n2, assign30940_e44756_d_n6, assign30940_e44756_d_n7, assign30940_e44756_d_n10, assign30940_e44756_d_n11, assign30940_e44756_d_n12, assign30940_e44756_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        let assign30940_e44753: f64 = (locals.var_ps0_inia__blk950 + locals.var_vxbgmtcl__blk925);
        let assign30940_e44754: f64 = (locals.var_beta * assign30940_e44753);
        (assign30940_e44754, (locals.var_beta * (locals.var_ps0_inia__blk950_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign30940_e44753) + (locals.var_beta * (locals.var_ps0_inia__blk950_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk950_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign30940_e44756;
        locals.var_chi__blk947_dn0 = assign30940_e44756_d_n0;
        locals.var_chi__blk947_dn2 = assign30940_e44756_d_n2;
        locals.var_chi__blk947_dn6 = assign30940_e44756_d_n6;
        locals.var_chi__blk947_dn7 = assign30940_e44756_d_n7;
        locals.var_chi__blk947_dn10 = assign30940_e44756_d_n10;
        locals.var_chi__blk947_dn11 = assign30940_e44756_d_n11;
        locals.var_chi__blk947_dn12 = assign30940_e44756_d_n12;
        locals.var_chi__blk947_dn17 = assign30940_e44756_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let assign30950_e44759: f64 = if p.p41 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1011 = assign30950_e44759;
        locals.var_guard1011_rv = 0.0;

        let (assign30970_e44793, assign30970_e44793_d_n0, assign30970_e44793_d_n2, assign30970_e44793_d_n6, assign30970_e44793_d_n7, assign30970_e44793_d_n10, assign30970_e44793_d_n11, assign30970_e44793_d_n12, assign30970_e44793_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign30970_e44789: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign30970_e44791: f64 = (assign30970_e44789 + 0.1);
        (assign30970_e44791, (locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0), (locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2), (locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6), (locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7), (locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10), (locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11), (locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12), (locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_vgpld_shift__blk959, locals.var_vgpld_shift__blk959_dn0, locals.var_vgpld_shift__blk959_dn2, locals.var_vgpld_shift__blk959_dn6, locals.var_vgpld_shift__blk959_dn7, locals.var_vgpld_shift__blk959_dn10, locals.var_vgpld_shift__blk959_dn11, locals.var_vgpld_shift__blk959_dn12, locals.var_vgpld_shift__blk959_dn17,)
    }
};
        locals.var_vgpld_shift__blk959 = assign30970_e44793;
        locals.var_vgpld_shift__blk959_dn0 = assign30970_e44793_d_n0;
        locals.var_vgpld_shift__blk959_dn2 = assign30970_e44793_d_n2;
        locals.var_vgpld_shift__blk959_dn6 = assign30970_e44793_d_n6;
        locals.var_vgpld_shift__blk959_dn7 = assign30970_e44793_d_n7;
        locals.var_vgpld_shift__blk959_dn10 = assign30970_e44793_d_n10;
        locals.var_vgpld_shift__blk959_dn11 = assign30970_e44793_d_n11;
        locals.var_vgpld_shift__blk959_dn12 = assign30970_e44793_d_n12;
        locals.var_vgpld_shift__blk959_dn17 = assign30970_e44793_d_n17;
        locals.var_vgpld_shift__blk959_rv = 0.0;

        let (assign30980_e44813, assign30980_e44813_d_n0, assign30980_e44813_d_n2, assign30980_e44813_d_n6, assign30980_e44813_d_n7, assign30980_e44813_d_n10, assign30980_e44813_d_n11, assign30980_e44813_d_n12, assign30980_e44813_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign30980_e44807: f64 = (-locals.var_vxbgmtcl__blk925);
        let assign30980_e44808: f64 = (locals.var_beta * assign30980_e44807);
        let assign30980_e44809: f64 = (assign30980_e44808).exp();
        let assign30980_e44811: f64 = (assign30980_e44809 + 1e-50);
        (assign30980_e44811, (assign30980_e44809 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn0))), (assign30980_e44809 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn2))), (assign30980_e44809 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn6))), (assign30980_e44809 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn7))), (assign30980_e44809 * ((locals.var_beta_dn10 * assign30980_e44807) + (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn10)))), (assign30980_e44809 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn11))), (assign30980_e44809 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn12))), (assign30980_e44809 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk966, locals.var_exp_bvbs__blk966_dn0, locals.var_exp_bvbs__blk966_dn2, locals.var_exp_bvbs__blk966_dn6, locals.var_exp_bvbs__blk966_dn7, locals.var_exp_bvbs__blk966_dn10, locals.var_exp_bvbs__blk966_dn11, locals.var_exp_bvbs__blk966_dn12, locals.var_exp_bvbs__blk966_dn17,)
    }
};
        locals.var_exp_bvbs__blk966 = assign30980_e44813;
        locals.var_exp_bvbs__blk966_dn0 = assign30980_e44813_d_n0;
        locals.var_exp_bvbs__blk966_dn2 = assign30980_e44813_d_n2;
        locals.var_exp_bvbs__blk966_dn6 = assign30980_e44813_d_n6;
        locals.var_exp_bvbs__blk966_dn7 = assign30980_e44813_d_n7;
        locals.var_exp_bvbs__blk966_dn10 = assign30980_e44813_d_n10;
        locals.var_exp_bvbs__blk966_dn11 = assign30980_e44813_d_n11;
        locals.var_exp_bvbs__blk966_dn12 = assign30980_e44813_d_n12;
        locals.var_exp_bvbs__blk966_dn17 = assign30980_e44813_d_n17;
        locals.var_exp_bvbs__blk966_rv = 0.0;

        let (assign30990_e44829, assign30990_e44829_d_n0, assign30990_e44829_d_n2, assign30990_e44829_d_n6, assign30990_e44829_d_n7, assign30990_e44829_d_n10, assign30990_e44829_d_n11, assign30990_e44829_d_n12, assign30990_e44829_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign30990_e44827: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign30990_e44827, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign30990_e44829;
        locals.var_t0__blk899_dn0 = assign30990_e44829_d_n0;
        locals.var_t0__blk899_dn2 = assign30990_e44829_d_n2;
        locals.var_t0__blk899_dn6 = assign30990_e44829_d_n6;
        locals.var_t0__blk899_dn7 = assign30990_e44829_d_n7;
        locals.var_t0__blk899_dn10 = assign30990_e44829_d_n10;
        locals.var_t0__blk899_dn11 = assign30990_e44829_d_n11;
        locals.var_t0__blk899_dn12 = assign30990_e44829_d_n12;
        locals.var_t0__blk899_dn17 = assign30990_e44829_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign31000_e44845, assign31000_e44845_d_n0, assign31000_e44845_d_n2, assign31000_e44845_d_n6, assign31000_e44845_d_n7, assign31000_e44845_d_n10, assign31000_e44845_d_n11, assign31000_e44845_d_n12, assign31000_e44845_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31000_e44843: f64 = (locals.var_t0__blk899 * locals.var_t0__blk899);
        (assign31000_e44843, ((locals.var_t0__blk899_dn0 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn0)), ((locals.var_t0__blk899_dn2 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn2)), ((locals.var_t0__blk899_dn6 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn6)), ((locals.var_t0__blk899_dn7 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn7)), ((locals.var_t0__blk899_dn10 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn10)), ((locals.var_t0__blk899_dn11 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn11)), ((locals.var_t0__blk899_dn12 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn12)), ((locals.var_t0__blk899_dn17 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn17)),)
    } else {
        (locals.var_cnst1over__blk960, locals.var_cnst1over__blk960_dn0, locals.var_cnst1over__blk960_dn2, locals.var_cnst1over__blk960_dn6, locals.var_cnst1over__blk960_dn7, locals.var_cnst1over__blk960_dn10, locals.var_cnst1over__blk960_dn11, locals.var_cnst1over__blk960_dn12, locals.var_cnst1over__blk960_dn17,)
    }
};
        locals.var_cnst1over__blk960 = assign31000_e44845;
        locals.var_cnst1over__blk960_dn0 = assign31000_e44845_d_n0;
        locals.var_cnst1over__blk960_dn2 = assign31000_e44845_d_n2;
        locals.var_cnst1over__blk960_dn6 = assign31000_e44845_d_n6;
        locals.var_cnst1over__blk960_dn7 = assign31000_e44845_d_n7;
        locals.var_cnst1over__blk960_dn10 = assign31000_e44845_d_n10;
        locals.var_cnst1over__blk960_dn11 = assign31000_e44845_d_n11;
        locals.var_cnst1over__blk960_dn12 = assign31000_e44845_d_n12;
        locals.var_cnst1over__blk960_dn17 = assign31000_e44845_d_n17;
        locals.var_cnst1over__blk960_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31010_e44861, assign31010_e44861_d_n0, assign31010_e44861_d_n2, assign31010_e44861_d_n6, assign31010_e44861_d_n7, assign31010_e44861_d_n10, assign31010_e44861_d_n11, assign31010_e44861_d_n12, assign31010_e44861_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31010_e44859: f64 = (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966);
        (assign31010_e44859, ((locals.var_cnst1over__blk960_dn0 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn0)), ((locals.var_cnst1over__blk960_dn2 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn2)), ((locals.var_cnst1over__blk960_dn6 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn6)), ((locals.var_cnst1over__blk960_dn7 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn7)), ((locals.var_cnst1over__blk960_dn10 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn10)), ((locals.var_cnst1over__blk960_dn11 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn11)), ((locals.var_cnst1over__blk960_dn12 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn12)), ((locals.var_cnst1over__blk960_dn17 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn17)),)
    } else {
        (locals.var_gammachi__blk961, locals.var_gammachi__blk961_dn0, locals.var_gammachi__blk961_dn2, locals.var_gammachi__blk961_dn6, locals.var_gammachi__blk961_dn7, locals.var_gammachi__blk961_dn10, locals.var_gammachi__blk961_dn11, locals.var_gammachi__blk961_dn12, locals.var_gammachi__blk961_dn17,)
    }
};
        locals.var_gammachi__blk961 = assign31010_e44861;
        locals.var_gammachi__blk961_dn0 = assign31010_e44861_d_n0;
        locals.var_gammachi__blk961_dn2 = assign31010_e44861_d_n2;
        locals.var_gammachi__blk961_dn6 = assign31010_e44861_d_n6;
        locals.var_gammachi__blk961_dn7 = assign31010_e44861_d_n7;
        locals.var_gammachi__blk961_dn10 = assign31010_e44861_d_n10;
        locals.var_gammachi__blk961_dn11 = assign31010_e44861_d_n11;
        locals.var_gammachi__blk961_dn12 = assign31010_e44861_d_n12;
        locals.var_gammachi__blk961_dn17 = assign31010_e44861_d_n17;
        locals.var_gammachi__blk961_rv = 0.0;

        let (assign31020_e44877, assign31020_e44877_d_n0, assign31020_e44877_d_n2, assign31020_e44877_d_n6, assign31020_e44877_d_n7, assign31020_e44877_d_n10, assign31020_e44877_d_n11, assign31020_e44877_d_n12, assign31020_e44877_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31020_e44875: f64 = (locals.var_beta2 * locals.var_fac1p2__blk934);
        (assign31020_e44875, (locals.var_beta2 * locals.var_fac1p2__blk934_dn0), (locals.var_beta2 * locals.var_fac1p2__blk934_dn2), (locals.var_beta2 * locals.var_fac1p2__blk934_dn6), (locals.var_beta2 * locals.var_fac1p2__blk934_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk934) + (locals.var_beta2 * locals.var_fac1p2__blk934_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk934_dn11), (locals.var_beta2 * locals.var_fac1p2__blk934_dn12), (locals.var_beta2 * locals.var_fac1p2__blk934_dn17),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign31020_e44877;
        locals.var_t0__blk899_dn0 = assign31020_e44877_d_n0;
        locals.var_t0__blk899_dn2 = assign31020_e44877_d_n2;
        locals.var_t0__blk899_dn6 = assign31020_e44877_d_n6;
        locals.var_t0__blk899_dn7 = assign31020_e44877_d_n7;
        locals.var_t0__blk899_dn10 = assign31020_e44877_d_n10;
        locals.var_t0__blk899_dn11 = assign31020_e44877_d_n11;
        locals.var_t0__blk899_dn12 = assign31020_e44877_d_n12;
        locals.var_t0__blk899_dn17 = assign31020_e44877_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign31030_e44893, assign31030_e44893_d_n0, assign31030_e44893_d_n2, assign31030_e44893_d_n6, assign31030_e44893_d_n7, assign31030_e44893_d_n10, assign31030_e44893_d_n11, assign31030_e44893_d_n12, assign31030_e44893_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31030_e44891: f64 = (locals.var_beta * locals.var_vgpld_shift__blk959);
        (assign31030_e44891, (locals.var_beta * locals.var_vgpld_shift__blk959_dn0), (locals.var_beta * locals.var_vgpld_shift__blk959_dn2), (locals.var_beta * locals.var_vgpld_shift__blk959_dn6), (locals.var_beta * locals.var_vgpld_shift__blk959_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift__blk959) + (locals.var_beta * locals.var_vgpld_shift__blk959_dn10)), (locals.var_beta * locals.var_vgpld_shift__blk959_dn11), (locals.var_beta * locals.var_vgpld_shift__blk959_dn12), (locals.var_beta * locals.var_vgpld_shift__blk959_dn17),)
    } else {
        (locals.var_psi__blk962, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, locals.var_psi__blk962_dn10, locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    }
};
        locals.var_psi__blk962 = assign31030_e44893;
        locals.var_psi__blk962_dn0 = assign31030_e44893_d_n0;
        locals.var_psi__blk962_dn2 = assign31030_e44893_d_n2;
        locals.var_psi__blk962_dn6 = assign31030_e44893_d_n6;
        locals.var_psi__blk962_dn7 = assign31030_e44893_d_n7;
        locals.var_psi__blk962_dn10 = assign31030_e44893_d_n10;
        locals.var_psi__blk962_dn11 = assign31030_e44893_d_n11;
        locals.var_psi__blk962_dn12 = assign31030_e44893_d_n12;
        locals.var_psi__blk962_dn17 = assign31030_e44893_d_n17;
        locals.var_psi__blk962_rv = 0.0;

        let (assign31040_e44923, assign31040_e44923_d_n0, assign31040_e44923_d_n2, assign31040_e44923_d_n6, assign31040_e44923_d_n7, assign31040_e44923_d_n10, assign31040_e44923_d_n11, assign31040_e44923_d_n12, assign31040_e44923_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31040_e44907: f64 = (locals.var_gammachi__blk961 * locals.var_t0__blk899);
        let assign31040_e44910: f64 = (locals.var_psi__blk962 * locals.var_psi__blk962);
        let assign31040_e44911: f64 = (assign31040_e44907 + assign31040_e44910);
        let assign31040_e44912: f64 = (assign31040_e44911).ln();
        let assign31040_e44915: f64 = (locals.var_cnst1over__blk960 * locals.var_t0__blk899);
        let assign31040_e44916: f64 = (assign31040_e44915).ln();
        let assign31040_e44917: f64 = (assign31040_e44912 - assign31040_e44916);
        let assign31040_e44920: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk925);
        let assign31040_e44921: f64 = (assign31040_e44917 + assign31040_e44920);
        (assign31040_e44921, ((((((locals.var_gammachi__blk961_dn0 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn0)) + ((locals.var_psi__blk962_dn0 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn0))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn0 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn0)) / assign31040_e44915)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn0)), ((((((locals.var_gammachi__blk961_dn2 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn2)) + ((locals.var_psi__blk962_dn2 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn2))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn2 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn2)) / assign31040_e44915)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn2)), ((((((locals.var_gammachi__blk961_dn6 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn6)) + ((locals.var_psi__blk962_dn6 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn6))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn6 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn6)) / assign31040_e44915)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn6)), ((((((locals.var_gammachi__blk961_dn7 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn7)) + ((locals.var_psi__blk962_dn7 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn7))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn7 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn7)) / assign31040_e44915)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn7)), ((((((locals.var_gammachi__blk961_dn10 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn10)) + ((locals.var_psi__blk962_dn10 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn10))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn10 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn10)) / assign31040_e44915)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk925) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn10))), ((((((locals.var_gammachi__blk961_dn11 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn11)) + ((locals.var_psi__blk962_dn11 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn11))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn11 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn11)) / assign31040_e44915)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn11)), ((((((locals.var_gammachi__blk961_dn12 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn12)) + ((locals.var_psi__blk962_dn12 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn12))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn12 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn12)) / assign31040_e44915)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn12)), ((((((locals.var_gammachi__blk961_dn17 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn17)) + ((locals.var_psi__blk962_dn17 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn17))) / assign31040_e44911) - (((locals.var_cnst1over__blk960_dn17 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn17)) / assign31040_e44915)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi_1__blk963, locals.var_chi_1__blk963_dn0, locals.var_chi_1__blk963_dn2, locals.var_chi_1__blk963_dn6, locals.var_chi_1__blk963_dn7, locals.var_chi_1__blk963_dn10, locals.var_chi_1__blk963_dn11, locals.var_chi_1__blk963_dn12, locals.var_chi_1__blk963_dn17,)
    }
};
        locals.var_chi_1__blk963 = assign31040_e44923;
        locals.var_chi_1__blk963_dn0 = assign31040_e44923_d_n0;
        locals.var_chi_1__blk963_dn2 = assign31040_e44923_d_n2;
        locals.var_chi_1__blk963_dn6 = assign31040_e44923_d_n6;
        locals.var_chi_1__blk963_dn7 = assign31040_e44923_d_n7;
        locals.var_chi_1__blk963_dn10 = assign31040_e44923_d_n10;
        locals.var_chi_1__blk963_dn11 = assign31040_e44923_d_n11;
        locals.var_chi_1__blk963_dn12 = assign31040_e44923_d_n12;
        locals.var_chi_1__blk963_dn17 = assign31040_e44923_d_n17;
        locals.var_chi_1__blk963_rv = 0.0;

        let (assign31050_e44941, assign31050_e44941_d_n0, assign31050_e44941_d_n2, assign31050_e44941_d_n6, assign31050_e44941_d_n7, assign31050_e44941_d_n10, assign31050_e44941_d_n11, assign31050_e44941_d_n12, assign31050_e44941_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31050_e44937: f64 = (locals.var_psi__blk962 - locals.var_chi_1__blk963);
        let assign31050_e44939: f64 = (assign31050_e44937 - 1.0);
        (assign31050_e44939, (locals.var_psi__blk962_dn0 - locals.var_chi_1__blk963_dn0), (locals.var_psi__blk962_dn2 - locals.var_chi_1__blk963_dn2), (locals.var_psi__blk962_dn6 - locals.var_chi_1__blk963_dn6), (locals.var_psi__blk962_dn7 - locals.var_chi_1__blk963_dn7), (locals.var_psi__blk962_dn10 - locals.var_chi_1__blk963_dn10), (locals.var_psi__blk962_dn11 - locals.var_chi_1__blk963_dn11), (locals.var_psi__blk962_dn12 - locals.var_chi_1__blk963_dn12), (locals.var_psi__blk962_dn17 - locals.var_chi_1__blk963_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign31050_e44941;
        locals.var_tmf1_dn0 = assign31050_e44941_d_n0;
        locals.var_tmf1_dn2 = assign31050_e44941_d_n2;
        locals.var_tmf1_dn6 = assign31050_e44941_d_n6;
        locals.var_tmf1_dn7 = assign31050_e44941_d_n7;
        locals.var_tmf1_dn10 = assign31050_e44941_d_n10;
        locals.var_tmf1_dn11 = assign31050_e44941_d_n11;
        locals.var_tmf1_dn12 = assign31050_e44941_d_n12;
        locals.var_tmf1_dn17 = assign31050_e44941_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign31060_e44959, assign31060_e44959_d_n0, assign31060_e44959_d_n2, assign31060_e44959_d_n6, assign31060_e44959_d_n7, assign31060_e44959_d_n10, assign31060_e44959_d_n11, assign31060_e44959_d_n12, assign31060_e44959_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31060_e44955: f64 = (4.0 * locals.var_psi__blk962);
        let assign31060_e44957: f64 = assign31060_e44955;
        (assign31060_e44957, (4.0 * locals.var_psi__blk962_dn0), (4.0 * locals.var_psi__blk962_dn2), (4.0 * locals.var_psi__blk962_dn6), (4.0 * locals.var_psi__blk962_dn7), (4.0 * locals.var_psi__blk962_dn10), (4.0 * locals.var_psi__blk962_dn11), (4.0 * locals.var_psi__blk962_dn12), (4.0 * locals.var_psi__blk962_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31060_e44959;
        locals.var_tmf2_dn0 = assign31060_e44959_d_n0;
        locals.var_tmf2_dn2 = assign31060_e44959_d_n2;
        locals.var_tmf2_dn6 = assign31060_e44959_d_n6;
        locals.var_tmf2_dn7 = assign31060_e44959_d_n7;
        locals.var_tmf2_dn10 = assign31060_e44959_d_n10;
        locals.var_tmf2_dn11 = assign31060_e44959_d_n11;
        locals.var_tmf2_dn12 = assign31060_e44959_d_n12;
        locals.var_tmf2_dn17 = assign31060_e44959_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31070_e44979, assign31070_e44979_d_n0, assign31070_e44979_d_n2, assign31070_e44979_d_n6, assign31070_e44979_d_n7, assign31070_e44979_d_n10, assign31070_e44979_d_n11, assign31070_e44979_d_n12, assign31070_e44979_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let (assign31070_e44977, assign31070_e44977_d_n0, assign31070_e44977_d_n2, assign31070_e44977_d_n6, assign31070_e44977_d_n7, assign31070_e44977_d_n10, assign31070_e44977_d_n11, assign31070_e44977_d_n12, assign31070_e44977_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign31070_e44976: f64 = (-locals.var_tmf2);
                (assign31070_e44976, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign31070_e44977, assign31070_e44977_d_n0, assign31070_e44977_d_n2, assign31070_e44977_d_n6, assign31070_e44977_d_n7, assign31070_e44977_d_n10, assign31070_e44977_d_n11, assign31070_e44977_d_n12, assign31070_e44977_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31070_e44979;
        locals.var_tmf2_dn0 = assign31070_e44979_d_n0;
        locals.var_tmf2_dn2 = assign31070_e44979_d_n2;
        locals.var_tmf2_dn6 = assign31070_e44979_d_n6;
        locals.var_tmf2_dn7 = assign31070_e44979_d_n7;
        locals.var_tmf2_dn10 = assign31070_e44979_d_n10;
        locals.var_tmf2_dn11 = assign31070_e44979_d_n11;
        locals.var_tmf2_dn12 = assign31070_e44979_d_n12;
        locals.var_tmf2_dn17 = assign31070_e44979_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31080_e44998, assign31080_e44998_d_n0, assign31080_e44998_d_n2, assign31080_e44998_d_n6, assign31080_e44998_d_n7, assign31080_e44998_d_n10, assign31080_e44998_d_n11, assign31080_e44998_d_n12, assign31080_e44998_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31080_e44993: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign31080_e44995: f64 = (assign31080_e44993 + locals.var_tmf2);
        let assign31080_e44996: f64 = (assign31080_e44995).sqrt();
        (assign31080_e44996, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31080_e44996)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31080_e44996)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31080_e44996)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31080_e44996)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31080_e44996)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31080_e44996)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31080_e44996)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31080_e44996)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31080_e44998;
        locals.var_tmf2_dn0 = assign31080_e44998_d_n0;
        locals.var_tmf2_dn2 = assign31080_e44998_d_n2;
        locals.var_tmf2_dn6 = assign31080_e44998_d_n6;
        locals.var_tmf2_dn7 = assign31080_e44998_d_n7;
        locals.var_tmf2_dn10 = assign31080_e44998_d_n10;
        locals.var_tmf2_dn11 = assign31080_e44998_d_n11;
        locals.var_tmf2_dn12 = assign31080_e44998_d_n12;
        locals.var_tmf2_dn17 = assign31080_e44998_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31090_e45018, assign31090_e45018_d_n0, assign31090_e45018_d_n2, assign31090_e45018_d_n6, assign31090_e45018_d_n7, assign31090_e45018_d_n10, assign31090_e45018_d_n11, assign31090_e45018_d_n12, assign31090_e45018_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31090_e45014: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign31090_e45015: f64 = (1.0 + assign31090_e45014);
        let assign31090_e45016: f64 = (0.5 * assign31090_e45015);
        (assign31090_e45016, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31090_e45018;
        locals.var_t1__blk900_dn0 = assign31090_e45018_d_n0;
        locals.var_t1__blk900_dn2 = assign31090_e45018_d_n2;
        locals.var_t1__blk900_dn6 = assign31090_e45018_d_n6;
        locals.var_t1__blk900_dn7 = assign31090_e45018_d_n7;
        locals.var_t1__blk900_dn10 = assign31090_e45018_d_n10;
        locals.var_t1__blk900_dn11 = assign31090_e45018_d_n11;
        locals.var_t1__blk900_dn12 = assign31090_e45018_d_n12;
        locals.var_t1__blk900_dn17 = assign31090_e45018_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31100_e45042, assign31100_e45042_d_n0, assign31100_e45042_d_n2, assign31100_e45042_d_n6, assign31100_e45042_d_n7, assign31100_e45042_d_n10, assign31100_e45042_d_n11, assign31100_e45042_d_n12, assign31100_e45042_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31100_e45035: f64 = 2.0;
        let assign31100_e45036: f64 = (locals.var_tmf1 + assign31100_e45035);
        let assign31100_e45038: f64 = (assign31100_e45036 / locals.var_tmf2);
        let assign31100_e45039: f64 = (1.0 - assign31100_e45038);
        let assign31100_e45040: f64 = (0.5 * assign31100_e45039);
        (assign31100_e45040, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31100_e45036 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign31100_e45042;
        locals.var_t2__blk901_dn0 = assign31100_e45042_d_n0;
        locals.var_t2__blk901_dn2 = assign31100_e45042_d_n2;
        locals.var_t2__blk901_dn6 = assign31100_e45042_d_n6;
        locals.var_t2__blk901_dn7 = assign31100_e45042_d_n7;
        locals.var_t2__blk901_dn10 = assign31100_e45042_d_n10;
        locals.var_t2__blk901_dn11 = assign31100_e45042_d_n11;
        locals.var_t2__blk901_dn12 = assign31100_e45042_d_n12;
        locals.var_t2__blk901_dn17 = assign31100_e45042_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign31110_e45062, assign31110_e45062_d_n0, assign31110_e45062_d_n2, assign31110_e45062_d_n6, assign31110_e45062_d_n7, assign31110_e45062_d_n10, assign31110_e45062_d_n11, assign31110_e45062_d_n12, assign31110_e45062_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31110_e45058: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign31110_e45059: f64 = (0.5 * assign31110_e45058);
        let assign31110_e45060: f64 = (locals.var_psi__blk962 - assign31110_e45059);
        (assign31110_e45060, (locals.var_psi__blk962_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi__blk962_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi__blk962_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi__blk962_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi__blk962_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi__blk962_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi__blk962_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi__blk962_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1__blk963, locals.var_chi_1__blk963_dn0, locals.var_chi_1__blk963_dn2, locals.var_chi_1__blk963_dn6, locals.var_chi_1__blk963_dn7, locals.var_chi_1__blk963_dn10, locals.var_chi_1__blk963_dn11, locals.var_chi_1__blk963_dn12, locals.var_chi_1__blk963_dn17,)
    }
};
        locals.var_chi_1__blk963 = assign31110_e45062;
        locals.var_chi_1__blk963_dn0 = assign31110_e45062_d_n0;
        locals.var_chi_1__blk963_dn2 = assign31110_e45062_d_n2;
        locals.var_chi_1__blk963_dn6 = assign31110_e45062_d_n6;
        locals.var_chi_1__blk963_dn7 = assign31110_e45062_d_n7;
        locals.var_chi_1__blk963_dn10 = assign31110_e45062_d_n10;
        locals.var_chi_1__blk963_dn11 = assign31110_e45062_d_n11;
        locals.var_chi_1__blk963_dn12 = assign31110_e45062_d_n12;
        locals.var_chi_1__blk963_dn17 = assign31110_e45062_d_n17;
        locals.var_chi_1__blk963_rv = 0.0;

        let (assign31120_e45078, assign31120_e45078_d_n0, assign31120_e45078_d_n2, assign31120_e45078_d_n6, assign31120_e45078_d_n7, assign31120_e45078_d_n10, assign31120_e45078_d_n11, assign31120_e45078_d_n12, assign31120_e45078_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31120_e45076: f64 = (locals.var_psi__blk962 - locals.var_chi_1__blk963);
        (assign31120_e45076, (locals.var_psi__blk962_dn0 - locals.var_chi_1__blk963_dn0), (locals.var_psi__blk962_dn2 - locals.var_chi_1__blk963_dn2), (locals.var_psi__blk962_dn6 - locals.var_chi_1__blk963_dn6), (locals.var_psi__blk962_dn7 - locals.var_chi_1__blk963_dn7), (locals.var_psi__blk962_dn10 - locals.var_chi_1__blk963_dn10), (locals.var_psi__blk962_dn11 - locals.var_chi_1__blk963_dn11), (locals.var_psi__blk962_dn12 - locals.var_chi_1__blk963_dn12), (locals.var_psi__blk962_dn17 - locals.var_chi_1__blk963_dn17),)
    } else {
        (locals.var_psi__blk962, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, locals.var_psi__blk962_dn10, locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    }
};
        locals.var_psi__blk962 = assign31120_e45078;
        locals.var_psi__blk962_dn0 = assign31120_e45078_d_n0;
        locals.var_psi__blk962_dn2 = assign31120_e45078_d_n2;
        locals.var_psi__blk962_dn6 = assign31120_e45078_d_n6;
        locals.var_psi__blk962_dn7 = assign31120_e45078_d_n7;
        locals.var_psi__blk962_dn10 = assign31120_e45078_d_n10;
        locals.var_psi__blk962_dn11 = assign31120_e45078_d_n11;
        locals.var_psi__blk962_dn12 = assign31120_e45078_d_n12;
        locals.var_psi__blk962_dn17 = assign31120_e45078_d_n17;
        locals.var_psi__blk962_rv = 0.0;

        let (assign31130_e45096, assign31130_e45096_d_n0, assign31130_e45096_d_n2, assign31130_e45096_d_n6, assign31130_e45096_d_n7, assign31130_e45096_d_n10, assign31130_e45096_d_n11, assign31130_e45096_d_n12, assign31130_e45096_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31130_e45093: f64 = (locals.var_beta * 0.1);
        let assign31130_e45094: f64 = (locals.var_psi__blk962 + assign31130_e45093);
        (assign31130_e45094, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, (locals.var_psi__blk962_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    } else {
        (locals.var_psi__blk962, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, locals.var_psi__blk962_dn10, locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    }
};
        locals.var_psi__blk962 = assign31130_e45096;
        locals.var_psi__blk962_dn0 = assign31130_e45096_d_n0;
        locals.var_psi__blk962_dn2 = assign31130_e45096_d_n2;
        locals.var_psi__blk962_dn6 = assign31130_e45096_d_n6;
        locals.var_psi__blk962_dn7 = assign31130_e45096_d_n7;
        locals.var_psi__blk962_dn10 = assign31130_e45096_d_n10;
        locals.var_psi__blk962_dn11 = assign31130_e45096_d_n11;
        locals.var_psi__blk962_dn12 = assign31130_e45096_d_n12;
        locals.var_psi__blk962_dn17 = assign31130_e45096_d_n17;
        locals.var_psi__blk962_rv = 0.0;

        let (assign31140_e45126, assign31140_e45126_d_n0, assign31140_e45126_d_n2, assign31140_e45126_d_n6, assign31140_e45126_d_n7, assign31140_e45126_d_n10, assign31140_e45126_d_n11, assign31140_e45126_d_n12, assign31140_e45126_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31140_e45110: f64 = (locals.var_gammachi__blk961 * locals.var_t0__blk899);
        let assign31140_e45113: f64 = (locals.var_psi__blk962 * locals.var_psi__blk962);
        let assign31140_e45114: f64 = (assign31140_e45110 + assign31140_e45113);
        let assign31140_e45115: f64 = (assign31140_e45114).ln();
        let assign31140_e45118: f64 = (locals.var_cnst1over__blk960 * locals.var_t0__blk899);
        let assign31140_e45119: f64 = (assign31140_e45118).ln();
        let assign31140_e45120: f64 = (assign31140_e45115 - assign31140_e45119);
        let assign31140_e45123: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk925);
        let assign31140_e45124: f64 = (assign31140_e45120 + assign31140_e45123);
        (assign31140_e45124, ((((((locals.var_gammachi__blk961_dn0 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn0)) + ((locals.var_psi__blk962_dn0 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn0))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn0 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn0)) / assign31140_e45118)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn0)), ((((((locals.var_gammachi__blk961_dn2 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn2)) + ((locals.var_psi__blk962_dn2 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn2))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn2 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn2)) / assign31140_e45118)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn2)), ((((((locals.var_gammachi__blk961_dn6 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn6)) + ((locals.var_psi__blk962_dn6 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn6))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn6 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn6)) / assign31140_e45118)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn6)), ((((((locals.var_gammachi__blk961_dn7 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn7)) + ((locals.var_psi__blk962_dn7 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn7))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn7 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn7)) / assign31140_e45118)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn7)), ((((((locals.var_gammachi__blk961_dn10 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn10)) + ((locals.var_psi__blk962_dn10 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn10))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn10 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn10)) / assign31140_e45118)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk925) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn10))), ((((((locals.var_gammachi__blk961_dn11 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn11)) + ((locals.var_psi__blk962_dn11 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn11))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn11 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn11)) / assign31140_e45118)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn11)), ((((((locals.var_gammachi__blk961_dn12 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn12)) + ((locals.var_psi__blk962_dn12 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn12))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn12 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn12)) / assign31140_e45118)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn12)), ((((((locals.var_gammachi__blk961_dn17 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn17)) + ((locals.var_psi__blk962_dn17 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn17))) / assign31140_e45114) - (((locals.var_cnst1over__blk960_dn17 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn17)) / assign31140_e45118)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi_b__blk964, locals.var_chi_b__blk964_dn0, locals.var_chi_b__blk964_dn2, locals.var_chi_b__blk964_dn6, locals.var_chi_b__blk964_dn7, locals.var_chi_b__blk964_dn10, locals.var_chi_b__blk964_dn11, locals.var_chi_b__blk964_dn12, locals.var_chi_b__blk964_dn17,)
    }
};
        locals.var_chi_b__blk964 = assign31140_e45126;
        locals.var_chi_b__blk964_dn0 = assign31140_e45126_d_n0;
        locals.var_chi_b__blk964_dn2 = assign31140_e45126_d_n2;
        locals.var_chi_b__blk964_dn6 = assign31140_e45126_d_n6;
        locals.var_chi_b__blk964_dn7 = assign31140_e45126_d_n7;
        locals.var_chi_b__blk964_dn10 = assign31140_e45126_d_n10;
        locals.var_chi_b__blk964_dn11 = assign31140_e45126_d_n11;
        locals.var_chi_b__blk964_dn12 = assign31140_e45126_d_n12;
        locals.var_chi_b__blk964_dn17 = assign31140_e45126_d_n17;
        locals.var_chi_b__blk964_rv = 0.0;

        let (assign31150_e45140, assign31150_e45140_d_n0, assign31150_e45140_d_n2, assign31150_e45140_d_n6, assign31150_e45140_d_n7, assign31150_e45140_d_n10, assign31150_e45140_d_n11, assign31150_e45140_d_n12, assign31150_e45140_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    } else {
        (locals.var_chi_a__blk965, locals.var_chi_a__blk965_dn0, locals.var_chi_a__blk965_dn2, locals.var_chi_a__blk965_dn6, locals.var_chi_a__blk965_dn7, locals.var_chi_a__blk965_dn10, locals.var_chi_a__blk965_dn11, locals.var_chi_a__blk965_dn12, locals.var_chi_a__blk965_dn17,)
    }
};
        locals.var_chi_a__blk965 = assign31150_e45140;
        locals.var_chi_a__blk965_dn0 = assign31150_e45140_d_n0;
        locals.var_chi_a__blk965_dn2 = assign31150_e45140_d_n2;
        locals.var_chi_a__blk965_dn6 = assign31150_e45140_d_n6;
        locals.var_chi_a__blk965_dn7 = assign31150_e45140_d_n7;
        locals.var_chi_a__blk965_dn10 = assign31150_e45140_d_n10;
        locals.var_chi_a__blk965_dn11 = assign31150_e45140_d_n11;
        locals.var_chi_a__blk965_dn12 = assign31150_e45140_d_n12;
        locals.var_chi_a__blk965_dn17 = assign31150_e45140_d_n17;
        locals.var_chi_a__blk965_rv = 0.0;

        let (assign31160_e45160, assign31160_e45160_d_n0, assign31160_e45160_d_n2, assign31160_e45160_d_n6, assign31160_e45160_d_n7, assign31160_e45160_d_n10, assign31160_e45160_d_n11, assign31160_e45160_d_n12, assign31160_e45160_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31160_e45154: f64 = (locals.var_chi_b__blk964 - locals.var_chi_a__blk965);
        let assign31160_e45157: f64 = (0.0008 * 75.0);
        let assign31160_e45158: f64 = (assign31160_e45154 - assign31160_e45157);
        (assign31160_e45158, (locals.var_chi_b__blk964_dn0 - locals.var_chi_a__blk965_dn0), (locals.var_chi_b__blk964_dn2 - locals.var_chi_a__blk965_dn2), (locals.var_chi_b__blk964_dn6 - locals.var_chi_a__blk965_dn6), (locals.var_chi_b__blk964_dn7 - locals.var_chi_a__blk965_dn7), (locals.var_chi_b__blk964_dn10 - locals.var_chi_a__blk965_dn10), (locals.var_chi_b__blk964_dn11 - locals.var_chi_a__blk965_dn11), (locals.var_chi_b__blk964_dn12 - locals.var_chi_a__blk965_dn12), (locals.var_chi_b__blk964_dn17 - locals.var_chi_a__blk965_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign31160_e45160;
        locals.var_tmf1_dn0 = assign31160_e45160_d_n0;
        locals.var_tmf1_dn2 = assign31160_e45160_d_n2;
        locals.var_tmf1_dn6 = assign31160_e45160_d_n6;
        locals.var_tmf1_dn7 = assign31160_e45160_d_n7;
        locals.var_tmf1_dn10 = assign31160_e45160_d_n10;
        locals.var_tmf1_dn11 = assign31160_e45160_d_n11;
        locals.var_tmf1_dn12 = assign31160_e45160_d_n12;
        locals.var_tmf1_dn17 = assign31160_e45160_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign31170_e45180, assign31170_e45180_d_n0, assign31170_e45180_d_n2, assign31170_e45180_d_n6, assign31170_e45180_d_n7, assign31170_e45180_d_n10, assign31170_e45180_d_n11, assign31170_e45180_d_n12, assign31170_e45180_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31170_e45174: f64 = (4.0 * locals.var_chi_b__blk964);
        let assign31170_e45177: f64 = (0.0008 * 75.0);
        let assign31170_e45178: f64 = (assign31170_e45174 * assign31170_e45177);
        (assign31170_e45178, ((4.0 * locals.var_chi_b__blk964_dn0) * assign31170_e45177), ((4.0 * locals.var_chi_b__blk964_dn2) * assign31170_e45177), ((4.0 * locals.var_chi_b__blk964_dn6) * assign31170_e45177), ((4.0 * locals.var_chi_b__blk964_dn7) * assign31170_e45177), ((4.0 * locals.var_chi_b__blk964_dn10) * assign31170_e45177), ((4.0 * locals.var_chi_b__blk964_dn11) * assign31170_e45177), ((4.0 * locals.var_chi_b__blk964_dn12) * assign31170_e45177), ((4.0 * locals.var_chi_b__blk964_dn17) * assign31170_e45177),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31170_e45180;
        locals.var_tmf2_dn0 = assign31170_e45180_d_n0;
        locals.var_tmf2_dn2 = assign31170_e45180_d_n2;
        locals.var_tmf2_dn6 = assign31170_e45180_d_n6;
        locals.var_tmf2_dn7 = assign31170_e45180_d_n7;
        locals.var_tmf2_dn10 = assign31170_e45180_d_n10;
        locals.var_tmf2_dn11 = assign31170_e45180_d_n11;
        locals.var_tmf2_dn12 = assign31170_e45180_d_n12;
        locals.var_tmf2_dn17 = assign31170_e45180_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31180_e45200, assign31180_e45200_d_n0, assign31180_e45200_d_n2, assign31180_e45200_d_n6, assign31180_e45200_d_n7, assign31180_e45200_d_n10, assign31180_e45200_d_n11, assign31180_e45200_d_n12, assign31180_e45200_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let (assign31180_e45198, assign31180_e45198_d_n0, assign31180_e45198_d_n2, assign31180_e45198_d_n6, assign31180_e45198_d_n7, assign31180_e45198_d_n10, assign31180_e45198_d_n11, assign31180_e45198_d_n12, assign31180_e45198_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign31180_e45197: f64 = (-locals.var_tmf2);
                (assign31180_e45197, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign31180_e45198, assign31180_e45198_d_n0, assign31180_e45198_d_n2, assign31180_e45198_d_n6, assign31180_e45198_d_n7, assign31180_e45198_d_n10, assign31180_e45198_d_n11, assign31180_e45198_d_n12, assign31180_e45198_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31180_e45200;
        locals.var_tmf2_dn0 = assign31180_e45200_d_n0;
        locals.var_tmf2_dn2 = assign31180_e45200_d_n2;
        locals.var_tmf2_dn6 = assign31180_e45200_d_n6;
        locals.var_tmf2_dn7 = assign31180_e45200_d_n7;
        locals.var_tmf2_dn10 = assign31180_e45200_d_n10;
        locals.var_tmf2_dn11 = assign31180_e45200_d_n11;
        locals.var_tmf2_dn12 = assign31180_e45200_d_n12;
        locals.var_tmf2_dn17 = assign31180_e45200_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31190_e45219, assign31190_e45219_d_n0, assign31190_e45219_d_n2, assign31190_e45219_d_n6, assign31190_e45219_d_n7, assign31190_e45219_d_n10, assign31190_e45219_d_n11, assign31190_e45219_d_n12, assign31190_e45219_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31190_e45214: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign31190_e45216: f64 = (assign31190_e45214 + locals.var_tmf2);
        let assign31190_e45217: f64 = (assign31190_e45216).sqrt();
        (assign31190_e45217, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31190_e45217)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31190_e45217)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31190_e45217)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31190_e45217)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31190_e45217)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31190_e45217)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31190_e45217)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31190_e45217)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign31190_e45219;
        locals.var_tmf2_dn0 = assign31190_e45219_d_n0;
        locals.var_tmf2_dn2 = assign31190_e45219_d_n2;
        locals.var_tmf2_dn6 = assign31190_e45219_d_n6;
        locals.var_tmf2_dn7 = assign31190_e45219_d_n7;
        locals.var_tmf2_dn10 = assign31190_e45219_d_n10;
        locals.var_tmf2_dn11 = assign31190_e45219_d_n11;
        locals.var_tmf2_dn12 = assign31190_e45219_d_n12;
        locals.var_tmf2_dn17 = assign31190_e45219_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign31200_e45239, assign31200_e45239_d_n0, assign31200_e45239_d_n2, assign31200_e45239_d_n6, assign31200_e45239_d_n7, assign31200_e45239_d_n10, assign31200_e45239_d_n11, assign31200_e45239_d_n12, assign31200_e45239_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31200_e45235: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign31200_e45236: f64 = (1.0 + assign31200_e45235);
        let assign31200_e45237: f64 = (0.5 * assign31200_e45236);
        (assign31200_e45237, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31200_e45239;
        locals.var_t1__blk900_dn0 = assign31200_e45239_d_n0;
        locals.var_t1__blk900_dn2 = assign31200_e45239_d_n2;
        locals.var_t1__blk900_dn6 = assign31200_e45239_d_n6;
        locals.var_t1__blk900_dn7 = assign31200_e45239_d_n7;
        locals.var_t1__blk900_dn10 = assign31200_e45239_d_n10;
        locals.var_t1__blk900_dn11 = assign31200_e45239_d_n11;
        locals.var_t1__blk900_dn12 = assign31200_e45239_d_n12;
        locals.var_t1__blk900_dn17 = assign31200_e45239_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31210_e45265, assign31210_e45265_d_n0, assign31210_e45265_d_n2, assign31210_e45265_d_n6, assign31210_e45265_d_n7, assign31210_e45265_d_n10, assign31210_e45265_d_n11, assign31210_e45265_d_n12, assign31210_e45265_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31210_e45256: f64 = (2.0 * 0.0008);
        let assign31210_e45258: f64 = (assign31210_e45256 * 75.0);
        let assign31210_e45259: f64 = (locals.var_tmf1 + assign31210_e45258);
        let assign31210_e45261: f64 = (assign31210_e45259 / locals.var_tmf2);
        let assign31210_e45262: f64 = (1.0 - assign31210_e45261);
        let assign31210_e45263: f64 = (0.5 * assign31210_e45262);
        (assign31210_e45263, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31210_e45259 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign31210_e45265;
        locals.var_t2__blk901_dn0 = assign31210_e45265_d_n0;
        locals.var_t2__blk901_dn2 = assign31210_e45265_d_n2;
        locals.var_t2__blk901_dn6 = assign31210_e45265_d_n6;
        locals.var_t2__blk901_dn7 = assign31210_e45265_d_n7;
        locals.var_t2__blk901_dn10 = assign31210_e45265_d_n10;
        locals.var_t2__blk901_dn11 = assign31210_e45265_d_n11;
        locals.var_t2__blk901_dn12 = assign31210_e45265_d_n12;
        locals.var_t2__blk901_dn17 = assign31210_e45265_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign31220_e45285, assign31220_e45285_d_n0, assign31220_e45285_d_n2, assign31220_e45285_d_n6, assign31220_e45285_d_n7, assign31220_e45285_d_n10, assign31220_e45285_d_n11, assign31220_e45285_d_n12, assign31220_e45285_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign31220_e45281: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign31220_e45282: f64 = (0.5 * assign31220_e45281);
        let assign31220_e45283: f64 = (locals.var_chi_b__blk964 - assign31220_e45282);
        (assign31220_e45283, (locals.var_chi_b__blk964_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b__blk964_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b__blk964_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b__blk964_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b__blk964_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b__blk964_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b__blk964_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b__blk964_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign31220_e45285;
        locals.var_chi__blk947_dn0 = assign31220_e45285_d_n0;
        locals.var_chi__blk947_dn2 = assign31220_e45285_d_n2;
        locals.var_chi__blk947_dn6 = assign31220_e45285_d_n6;
        locals.var_chi__blk947_dn7 = assign31220_e45285_d_n7;
        locals.var_chi__blk947_dn10 = assign31220_e45285_d_n10;
        locals.var_chi__blk947_dn11 = assign31220_e45285_d_n11;
        locals.var_chi__blk947_dn12 = assign31220_e45285_d_n12;
        locals.var_chi__blk947_dn17 = assign31220_e45285_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let (assign31230_e45301, assign31230_e45301_d_n0, assign31230_e45301_d_n2, assign31230_e45301_d_n6, assign31230_e45301_d_n7, assign31230_e45301_d_n10, assign31230_e45301_d_n11, assign31230_e45301_d_n12, assign31230_e45301_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign31230_e45297: f64 = (locals.var_chi__blk947 / locals.var_beta);
        let assign31230_e45299: f64 = (assign31230_e45297 - locals.var_vxbgmtcl__blk925);
        (assign31230_e45299, ((locals.var_chi__blk947_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn0), ((locals.var_chi__blk947_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn2), ((locals.var_chi__blk947_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn6), ((locals.var_chi__blk947_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn7), ((((locals.var_chi__blk947_dn10 * locals.var_beta) - (locals.var_chi__blk947 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk925_dn10), ((locals.var_chi__blk947_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn11), ((locals.var_chi__blk947_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn12), ((locals.var_chi__blk947_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_ps0ld__blk949, locals.var_ps0ld__blk949_dn0, locals.var_ps0ld__blk949_dn2, locals.var_ps0ld__blk949_dn6, locals.var_ps0ld__blk949_dn7, locals.var_ps0ld__blk949_dn10, locals.var_ps0ld__blk949_dn11, locals.var_ps0ld__blk949_dn12, locals.var_ps0ld__blk949_dn17,)
    }
};
        locals.var_ps0ld__blk949 = assign31230_e45301;
        locals.var_ps0ld__blk949_dn0 = assign31230_e45301_d_n0;
        locals.var_ps0ld__blk949_dn2 = assign31230_e45301_d_n2;
        locals.var_ps0ld__blk949_dn6 = assign31230_e45301_d_n6;
        locals.var_ps0ld__blk949_dn7 = assign31230_e45301_d_n7;
        locals.var_ps0ld__blk949_dn10 = assign31230_e45301_d_n10;
        locals.var_ps0ld__blk949_dn11 = assign31230_e45301_d_n11;
        locals.var_ps0ld__blk949_dn12 = assign31230_e45301_d_n12;
        locals.var_ps0ld__blk949_dn17 = assign31230_e45301_d_n17;
        locals.var_ps0ld__blk949_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31240_e45319, assign31240_e45319_d_n0, assign31240_e45319_d_n2, assign31240_e45319_d_n6, assign31240_e45319_d_n7, assign31240_e45319_d_n10, assign31240_e45319_d_n11, assign31240_e45319_d_n12, assign31240_e45319_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign31240_e45313: f64 = (locals.var_chi__blk947 - 1.0);
        let assign31240_e45315: f64 = (-locals.var_chi__blk947);
        let assign31240_e45316: f64 = (assign31240_e45315).exp();
        let assign31240_e45317: f64 = (assign31240_e45313 + assign31240_e45316);
        (assign31240_e45317, (locals.var_chi__blk947_dn0 + (assign31240_e45316 * (-locals.var_chi__blk947_dn0))), (locals.var_chi__blk947_dn2 + (assign31240_e45316 * (-locals.var_chi__blk947_dn2))), (locals.var_chi__blk947_dn6 + (assign31240_e45316 * (-locals.var_chi__blk947_dn6))), (locals.var_chi__blk947_dn7 + (assign31240_e45316 * (-locals.var_chi__blk947_dn7))), (locals.var_chi__blk947_dn10 + (assign31240_e45316 * (-locals.var_chi__blk947_dn10))), (locals.var_chi__blk947_dn11 + (assign31240_e45316 * (-locals.var_chi__blk947_dn11))), (locals.var_chi__blk947_dn12 + (assign31240_e45316 * (-locals.var_chi__blk947_dn12))), (locals.var_chi__blk947_dn17 + (assign31240_e45316 * (-locals.var_chi__blk947_dn17))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31240_e45319;
        locals.var_t1__blk900_dn0 = assign31240_e45319_d_n0;
        locals.var_t1__blk900_dn2 = assign31240_e45319_d_n2;
        locals.var_t1__blk900_dn6 = assign31240_e45319_d_n6;
        locals.var_t1__blk900_dn7 = assign31240_e45319_d_n7;
        locals.var_t1__blk900_dn10 = assign31240_e45319_d_n10;
        locals.var_t1__blk900_dn11 = assign31240_e45319_d_n11;
        locals.var_t1__blk900_dn12 = assign31240_e45319_d_n12;
        locals.var_t1__blk900_dn17 = assign31240_e45319_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let assign31250_e45323: f64 = (10.0 * 2.220446049250313e-16);
        let assign31250_e45324: f64 = if locals.var_t1__blk900 < assign31250_e45323 { 1.0 } else { 0.0 };
        locals.var_guard1012 = assign31250_e45324;
        locals.var_guard1012_rv = 0.0;

        let (assign31260_e45340, assign31260_e45340_d_n0, assign31260_e45340_d_n2, assign31260_e45340_d_n6, assign31260_e45340_d_n7, assign31260_e45340_d_n10, assign31260_e45340_d_n11, assign31260_e45340_d_n12, assign31260_e45340_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1012 != 0.0)) {
        let assign31260_e45338: f64 = (10.0 * 2.220446049250313e-16);
        (assign31260_e45338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31260_e45340;
        locals.var_t1__blk900_dn0 = assign31260_e45340_d_n0;
        locals.var_t1__blk900_dn2 = assign31260_e45340_d_n2;
        locals.var_t1__blk900_dn6 = assign31260_e45340_d_n6;
        locals.var_t1__blk900_dn7 = assign31260_e45340_d_n7;
        locals.var_t1__blk900_dn10 = assign31260_e45340_d_n10;
        locals.var_t1__blk900_dn11 = assign31260_e45340_d_n11;
        locals.var_t1__blk900_dn12 = assign31260_e45340_d_n12;
        locals.var_t1__blk900_dn17 = assign31260_e45340_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31270_e45353, assign31270_e45353_d_n0, assign31270_e45353_d_n2, assign31270_e45353_d_n6, assign31270_e45353_d_n7, assign31270_e45353_d_n10, assign31270_e45353_d_n11, assign31270_e45353_d_n12, assign31270_e45353_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign31270_e45351: f64 = (locals.var_t1__blk900).sqrt();
        (assign31270_e45351, (locals.var_t1__blk900_dn0 / (2.0 * assign31270_e45351)), (locals.var_t1__blk900_dn2 / (2.0 * assign31270_e45351)), (locals.var_t1__blk900_dn6 / (2.0 * assign31270_e45351)), (locals.var_t1__blk900_dn7 / (2.0 * assign31270_e45351)), (locals.var_t1__blk900_dn10 / (2.0 * assign31270_e45351)), (locals.var_t1__blk900_dn11 / (2.0 * assign31270_e45351)), (locals.var_t1__blk900_dn12 / (2.0 * assign31270_e45351)), (locals.var_t1__blk900_dn17 / (2.0 * assign31270_e45351)),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign31270_e45353;
        locals.var_t2__blk901_dn0 = assign31270_e45353_d_n0;
        locals.var_t2__blk901_dn2 = assign31270_e45353_d_n2;
        locals.var_t2__blk901_dn6 = assign31270_e45353_d_n6;
        locals.var_t2__blk901_dn7 = assign31270_e45353_d_n7;
        locals.var_t2__blk901_dn10 = assign31270_e45353_d_n10;
        locals.var_t2__blk901_dn11 = assign31270_e45353_d_n11;
        locals.var_t2__blk901_dn12 = assign31270_e45353_d_n12;
        locals.var_t2__blk901_dn17 = assign31270_e45353_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign31280_e45367, assign31280_e45367_d_n0, assign31280_e45367_d_n2, assign31280_e45367_d_n6, assign31280_e45367_d_n7, assign31280_e45367_d_n10, assign31280_e45367_d_n11, assign31280_e45367_d_n12, assign31280_e45367_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign31280_e45365: f64 = (locals.var_cnst0over__blk932 * locals.var_t2__blk901);
        (assign31280_e45365, ((locals.var_cnst0over__blk932_dn0 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn0)), ((locals.var_cnst0over__blk932_dn2 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn2)), ((locals.var_cnst0over__blk932_dn6 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn6)), ((locals.var_cnst0over__blk932_dn7 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn7)), ((locals.var_cnst0over__blk932_dn10 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn10)), ((locals.var_cnst0over__blk932_dn11 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn11)), ((locals.var_cnst0over__blk932_dn12 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn12)), ((locals.var_cnst0over__blk932_dn17 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign31280_e45367;
        locals.var_qbuld_dn0 = assign31280_e45367_d_n0;
        locals.var_qbuld_dn2 = assign31280_e45367_d_n2;
        locals.var_qbuld_dn6 = assign31280_e45367_d_n6;
        locals.var_qbuld_dn7 = assign31280_e45367_d_n7;
        locals.var_qbuld_dn10 = assign31280_e45367_d_n10;
        locals.var_qbuld_dn11 = assign31280_e45367_d_n11;
        locals.var_qbuld_dn12 = assign31280_e45367_d_n12;
        locals.var_qbuld_dn17 = assign31280_e45367_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign31290_e45383, assign31290_e45383_d_n0, assign31290_e45383_d_n2, assign31290_e45383_d_n6, assign31290_e45383_d_n7, assign31290_e45383_d_n10, assign31290_e45383_d_n11, assign31290_e45383_d_n12, assign31290_e45383_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) {
        let assign31290_e45380: f64 = (locals.var_vgpld__blk935 - locals.var_ps0ld__blk949);
        let assign31290_e45381: f64 = (locals.var_cox0__blk910 * assign31290_e45380);
        (assign31290_e45381, (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn0 - locals.var_ps0ld__blk949_dn0)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn2 - locals.var_ps0ld__blk949_dn2)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn6 - locals.var_ps0ld__blk949_dn6)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn7 - locals.var_ps0ld__blk949_dn7)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn10 - locals.var_ps0ld__blk949_dn10)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn11 - locals.var_ps0ld__blk949_dn11)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn12 - locals.var_ps0ld__blk949_dn12)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn17 - locals.var_ps0ld__blk949_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign31290_e45383;
        locals.var_qsuld_dn0 = assign31290_e45383_d_n0;
        locals.var_qsuld_dn2 = assign31290_e45383_d_n2;
        locals.var_qsuld_dn6 = assign31290_e45383_d_n6;
        locals.var_qsuld_dn7 = assign31290_e45383_d_n7;
        locals.var_qsuld_dn10 = assign31290_e45383_d_n10;
        locals.var_qsuld_dn11 = assign31290_e45383_d_n11;
        locals.var_qsuld_dn12 = assign31290_e45383_d_n12;
        locals.var_qsuld_dn17 = assign31290_e45383_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign31300_e45386: f64 = if p.p41 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1013 = assign31300_e45386;
        locals.var_guard1013_rv = 0.0;

        let (assign31310_e45404, assign31310_e45404_d_n0, assign31310_e45404_d_n2, assign31310_e45404_d_n6, assign31310_e45404_d_n7, assign31310_e45404_d_n10, assign31310_e45404_d_n11, assign31310_e45404_d_n12, assign31310_e45404_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31310_e45400: f64 = (-locals.var_vxbgmtcl__blk925);
        let assign31310_e45401: f64 = (locals.var_beta * assign31310_e45400);
        let assign31310_e45402: f64 = (assign31310_e45401).exp();
        (assign31310_e45402, (assign31310_e45402 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn0))), (assign31310_e45402 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn2))), (assign31310_e45402 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn6))), (assign31310_e45402 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn7))), (assign31310_e45402 * ((locals.var_beta_dn10 * assign31310_e45400) + (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn10)))), (assign31310_e45402 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn11))), (assign31310_e45402 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn12))), (assign31310_e45402 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk966, locals.var_exp_bvbs__blk966_dn0, locals.var_exp_bvbs__blk966_dn2, locals.var_exp_bvbs__blk966_dn6, locals.var_exp_bvbs__blk966_dn7, locals.var_exp_bvbs__blk966_dn10, locals.var_exp_bvbs__blk966_dn11, locals.var_exp_bvbs__blk966_dn12, locals.var_exp_bvbs__blk966_dn17,)
    }
};
        locals.var_exp_bvbs__blk966 = assign31310_e45404;
        locals.var_exp_bvbs__blk966_dn0 = assign31310_e45404_d_n0;
        locals.var_exp_bvbs__blk966_dn2 = assign31310_e45404_d_n2;
        locals.var_exp_bvbs__blk966_dn6 = assign31310_e45404_d_n6;
        locals.var_exp_bvbs__blk966_dn7 = assign31310_e45404_d_n7;
        locals.var_exp_bvbs__blk966_dn10 = assign31310_e45404_d_n10;
        locals.var_exp_bvbs__blk966_dn11 = assign31310_e45404_d_n11;
        locals.var_exp_bvbs__blk966_dn12 = assign31310_e45404_d_n12;
        locals.var_exp_bvbs__blk966_dn17 = assign31310_e45404_d_n17;
        locals.var_exp_bvbs__blk966_rv = 0.0;

        let (assign31320_e45420, assign31320_e45420_d_n0, assign31320_e45420_d_n2, assign31320_e45420_d_n6, assign31320_e45420_d_n7, assign31320_e45420_d_n10, assign31320_e45420_d_n11, assign31320_e45420_d_n12, assign31320_e45420_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31320_e45418: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign31320_e45418, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign31320_e45420;
        locals.var_t0__blk899_dn0 = assign31320_e45420_d_n0;
        locals.var_t0__blk899_dn2 = assign31320_e45420_d_n2;
        locals.var_t0__blk899_dn6 = assign31320_e45420_d_n6;
        locals.var_t0__blk899_dn7 = assign31320_e45420_d_n7;
        locals.var_t0__blk899_dn10 = assign31320_e45420_d_n10;
        locals.var_t0__blk899_dn11 = assign31320_e45420_d_n11;
        locals.var_t0__blk899_dn12 = assign31320_e45420_d_n12;
        locals.var_t0__blk899_dn17 = assign31320_e45420_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign31330_e45436, assign31330_e45436_d_n0, assign31330_e45436_d_n2, assign31330_e45436_d_n6, assign31330_e45436_d_n7, assign31330_e45436_d_n10, assign31330_e45436_d_n11, assign31330_e45436_d_n12, assign31330_e45436_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31330_e45434: f64 = (locals.var_t0__blk899 * locals.var_t0__blk899);
        (assign31330_e45434, ((locals.var_t0__blk899_dn0 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn0)), ((locals.var_t0__blk899_dn2 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn2)), ((locals.var_t0__blk899_dn6 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn6)), ((locals.var_t0__blk899_dn7 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn7)), ((locals.var_t0__blk899_dn10 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn10)), ((locals.var_t0__blk899_dn11 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn11)), ((locals.var_t0__blk899_dn12 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn12)), ((locals.var_t0__blk899_dn17 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn17)),)
    } else {
        (locals.var_cnst1over__blk960, locals.var_cnst1over__blk960_dn0, locals.var_cnst1over__blk960_dn2, locals.var_cnst1over__blk960_dn6, locals.var_cnst1over__blk960_dn7, locals.var_cnst1over__blk960_dn10, locals.var_cnst1over__blk960_dn11, locals.var_cnst1over__blk960_dn12, locals.var_cnst1over__blk960_dn17,)
    }
};
        locals.var_cnst1over__blk960 = assign31330_e45436;
        locals.var_cnst1over__blk960_dn0 = assign31330_e45436_d_n0;
        locals.var_cnst1over__blk960_dn2 = assign31330_e45436_d_n2;
        locals.var_cnst1over__blk960_dn6 = assign31330_e45436_d_n6;
        locals.var_cnst1over__blk960_dn7 = assign31330_e45436_d_n7;
        locals.var_cnst1over__blk960_dn10 = assign31330_e45436_d_n10;
        locals.var_cnst1over__blk960_dn11 = assign31330_e45436_d_n11;
        locals.var_cnst1over__blk960_dn12 = assign31330_e45436_d_n12;
        locals.var_cnst1over__blk960_dn17 = assign31330_e45436_d_n17;
        locals.var_cnst1over__blk960_rv = 0.0;

        let (assign31340_e45452, assign31340_e45452_d_n0, assign31340_e45452_d_n2, assign31340_e45452_d_n6, assign31340_e45452_d_n7, assign31340_e45452_d_n10, assign31340_e45452_d_n11, assign31340_e45452_d_n12, assign31340_e45452_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31340_e45450: f64 = (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966);
        (assign31340_e45450, ((locals.var_cnst1over__blk960_dn0 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn0)), ((locals.var_cnst1over__blk960_dn2 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn2)), ((locals.var_cnst1over__blk960_dn6 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn6)), ((locals.var_cnst1over__blk960_dn7 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn7)), ((locals.var_cnst1over__blk960_dn10 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn10)), ((locals.var_cnst1over__blk960_dn11 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn11)), ((locals.var_cnst1over__blk960_dn12 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn12)), ((locals.var_cnst1over__blk960_dn17 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn17)),)
    } else {
        (locals.var_cfs1__blk975, locals.var_cfs1__blk975_dn0, locals.var_cfs1__blk975_dn2, locals.var_cfs1__blk975_dn6, locals.var_cfs1__blk975_dn7, locals.var_cfs1__blk975_dn10, locals.var_cfs1__blk975_dn11, locals.var_cfs1__blk975_dn12, locals.var_cfs1__blk975_dn17,)
    }
};
        locals.var_cfs1__blk975 = assign31340_e45452;
        locals.var_cfs1__blk975_dn0 = assign31340_e45452_d_n0;
        locals.var_cfs1__blk975_dn2 = assign31340_e45452_d_n2;
        locals.var_cfs1__blk975_dn6 = assign31340_e45452_d_n6;
        locals.var_cfs1__blk975_dn7 = assign31340_e45452_d_n7;
        locals.var_cfs1__blk975_dn10 = assign31340_e45452_d_n10;
        locals.var_cfs1__blk975_dn11 = assign31340_e45452_d_n11;
        locals.var_cfs1__blk975_dn12 = assign31340_e45452_d_n12;
        locals.var_cfs1__blk975_dn17 = assign31340_e45452_d_n17;
        locals.var_cfs1__blk975_rv = 0.0;

        let (assign31350_e45466,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk922,)
    }
};
        locals.var_flg_conv__blk922 = assign31350_e45466;
        locals.var_flg_conv__blk922_rv = 0.0;

        let (assign31360_e45480, assign31360_e45480_d_n0, assign31360_e45480_d_n2, assign31360_e45480_d_n6, assign31360_e45480_d_n7, assign31360_e45480_d_n10, assign31360_e45480_d_n11, assign31360_e45480_d_n12, assign31360_e45480_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
        locals.var_fs01__blk969 = assign31360_e45480;
        locals.var_fs01__blk969_dn0 = assign31360_e45480_d_n0;
        locals.var_fs01__blk969_dn2 = assign31360_e45480_d_n2;
        locals.var_fs01__blk969_dn6 = assign31360_e45480_d_n6;
        locals.var_fs01__blk969_dn7 = assign31360_e45480_d_n7;
        locals.var_fs01__blk969_dn10 = assign31360_e45480_d_n10;
        locals.var_fs01__blk969_dn11 = assign31360_e45480_d_n11;
        locals.var_fs01__blk969_dn12 = assign31360_e45480_d_n12;
        locals.var_fs01__blk969_dn17 = assign31360_e45480_d_n17;
        locals.var_fs01__blk969_rv = 0.0;

        let (assign31370_e45494, assign31370_e45494_d_n0, assign31370_e45494_d_n2, assign31370_e45494_d_n6, assign31370_e45494_d_n7, assign31370_e45494_d_n10, assign31370_e45494_d_n11, assign31370_e45494_d_n12, assign31370_e45494_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk973, locals.var_fs02__blk973_dn0, locals.var_fs02__blk973_dn2, locals.var_fs02__blk973_dn6, locals.var_fs02__blk973_dn7, locals.var_fs02__blk973_dn10, locals.var_fs02__blk973_dn11, locals.var_fs02__blk973_dn12, locals.var_fs02__blk973_dn17,)
    }
};
        locals.var_fs02__blk973 = assign31370_e45494;
        locals.var_fs02__blk973_dn0 = assign31370_e45494_d_n0;
        locals.var_fs02__blk973_dn2 = assign31370_e45494_d_n2;
        locals.var_fs02__blk973_dn6 = assign31370_e45494_d_n6;
        locals.var_fs02__blk973_dn7 = assign31370_e45494_d_n7;
        locals.var_fs02__blk973_dn10 = assign31370_e45494_d_n10;
        locals.var_fs02__blk973_dn11 = assign31370_e45494_d_n11;
        locals.var_fs02__blk973_dn12 = assign31370_e45494_d_n12;
        locals.var_fs02__blk973_dn17 = assign31370_e45494_d_n17;
        locals.var_fs02__blk973_rv = 0.0;

        let (assign31380_e45508,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign31380_e45508;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_115(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign31390_loop_guard: usize = 0;
        while {
            let assign31390_cond_e45523: f64 = (2.0 * 20.0);
            let assign31390_cond_e45525: f64 = (assign31390_cond_e45523 + 1.0);
            let assign31390_cond_e45527: f64 = if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_lp_s0 <= assign31390_cond_e45525)) { 1.0 } else { 0.0 };
            assign31390_cond_e45527 != 0.0
        } {
            assign31390_loop_guard += 1;
            assert!(assign31390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31390_body0_e45541, assign31390_body0_e45541_d_n0, assign31390_body0_e45541_d_n2, assign31390_body0_e45541_d_n6, assign31390_body0_e45541_d_n7, assign31390_body0_e45541_d_n10, assign31390_body0_e45541_d_n11, assign31390_body0_e45541_d_n12, assign31390_body0_e45541_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk971, locals.var_fb__blk971_dn0, locals.var_fb__blk971_dn2, locals.var_fb__blk971_dn6, locals.var_fb__blk971_dn7, locals.var_fb__blk971_dn10, locals.var_fb__blk971_dn11, locals.var_fb__blk971_dn12, locals.var_fb__blk971_dn17,)
    }
};
            locals.var_fb__blk971 = assign31390_body0_e45541;
            locals.var_fb__blk971_dn0 = assign31390_body0_e45541_d_n0;
            locals.var_fb__blk971_dn2 = assign31390_body0_e45541_d_n2;
            locals.var_fb__blk971_dn6 = assign31390_body0_e45541_d_n6;
            locals.var_fb__blk971_dn7 = assign31390_body0_e45541_d_n7;
            locals.var_fb__blk971_dn10 = assign31390_body0_e45541_d_n10;
            locals.var_fb__blk971_dn11 = assign31390_body0_e45541_d_n11;
            locals.var_fb__blk971_dn12 = assign31390_body0_e45541_d_n12;
            locals.var_fb__blk971_dn17 = assign31390_body0_e45541_d_n17;
            locals.var_fb__blk971_rv = 0.0;
            let (assign31390_body1_e45559, assign31390_body1_e45559_d_n0, assign31390_body1_e45559_d_n2, assign31390_body1_e45559_d_n6, assign31390_body1_e45559_d_n7, assign31390_body1_e45559_d_n10, assign31390_body1_e45559_d_n11, assign31390_body1_e45559_d_n12, assign31390_body1_e45559_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31390_body1_e45556: f64 = (locals.var_ps0ld__blk949 + locals.var_vxbgmtcl__blk925);
        let assign31390_body1_e45557: f64 = (locals.var_beta * assign31390_body1_e45556);
        (assign31390_body1_e45557, (locals.var_beta * (locals.var_ps0ld__blk949_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0ld__blk949_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0ld__blk949_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0ld__blk949_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign31390_body1_e45556) + (locals.var_beta * (locals.var_ps0ld__blk949_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0ld__blk949_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0ld__blk949_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0ld__blk949_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
            locals.var_chi__blk947 = assign31390_body1_e45559;
            locals.var_chi__blk947_dn0 = assign31390_body1_e45559_d_n0;
            locals.var_chi__blk947_dn2 = assign31390_body1_e45559_d_n2;
            locals.var_chi__blk947_dn6 = assign31390_body1_e45559_d_n6;
            locals.var_chi__blk947_dn7 = assign31390_body1_e45559_d_n7;
            locals.var_chi__blk947_dn10 = assign31390_body1_e45559_d_n10;
            locals.var_chi__blk947_dn11 = assign31390_body1_e45559_d_n11;
            locals.var_chi__blk947_dn12 = assign31390_body1_e45559_d_n12;
            locals.var_chi__blk947_dn17 = assign31390_body1_e45559_d_n17;
            locals.var_chi__blk947_rv = 0.0;
            let assign31390_body2_e45562: f64 = if locals.var_chi__blk947 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1014 = assign31390_body2_e45562;
            locals.var_guard1014_rv = 0.0;
            let (assign31390_body3_e45593, assign31390_body3_e45593_d_n0, assign31390_body3_e45593_d_n2, assign31390_body3_e45593_d_n6, assign31390_body3_e45593_d_n7, assign31390_body3_e45593_d_n10, assign31390_body3_e45593_d_n11, assign31390_body3_e45593_d_n12, assign31390_body3_e45593_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body3_e45578: f64 = (locals.var_chi__blk947 * locals.var_chi__blk947);
        let assign31390_body3_e45580: f64 = (assign31390_body3_e45578 * locals.var_chi__blk947);
        let assign31390_body3_e45584: f64 = (-0.07053654284009761);
        let assign31390_body3_e45587: f64 = (locals.var_chi__blk947 * 0.006115288895133179);
        let assign31390_body3_e45588: f64 = (assign31390_body3_e45584 + assign31390_body3_e45587);
        let assign31390_body3_e45589: f64 = (locals.var_chi__blk947 * assign31390_body3_e45588);
        let assign31390_body3_e45590: f64 = (0.29693154855771 + assign31390_body3_e45589);
        let assign31390_body3_e45591: f64 = (assign31390_body3_e45580 * assign31390_body3_e45590);
        (assign31390_body3_e45591, ((((((locals.var_chi__blk947_dn0 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn0)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn0)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn0 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn2 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn2)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn2)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn2 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn6 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn6)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn6)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn6 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn7 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn7)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn7)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn7 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn10 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn10)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn10)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn10 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn11 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn11)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn11)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn11 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn12 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn12)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn12)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn12 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn17 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn17)) * locals.var_chi__blk947) + (assign31390_body3_e45578 * locals.var_chi__blk947_dn17)) * assign31390_body3_e45590) + (assign31390_body3_e45580 * ((locals.var_chi__blk947_dn17 * assign31390_body3_e45588) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi__blk967, locals.var_fi__blk967_dn0, locals.var_fi__blk967_dn2, locals.var_fi__blk967_dn6, locals.var_fi__blk967_dn7, locals.var_fi__blk967_dn10, locals.var_fi__blk967_dn11, locals.var_fi__blk967_dn12, locals.var_fi__blk967_dn17,)
    }
};
            locals.var_fi__blk967 = assign31390_body3_e45593;
            locals.var_fi__blk967_dn0 = assign31390_body3_e45593_d_n0;
            locals.var_fi__blk967_dn2 = assign31390_body3_e45593_d_n2;
            locals.var_fi__blk967_dn6 = assign31390_body3_e45593_d_n6;
            locals.var_fi__blk967_dn7 = assign31390_body3_e45593_d_n7;
            locals.var_fi__blk967_dn10 = assign31390_body3_e45593_d_n10;
            locals.var_fi__blk967_dn11 = assign31390_body3_e45593_d_n11;
            locals.var_fi__blk967_dn12 = assign31390_body3_e45593_d_n12;
            locals.var_fi__blk967_dn17 = assign31390_body3_e45593_d_n17;
            locals.var_fi__blk967_rv = 0.0;
            let (assign31390_body4_e45628, assign31390_body4_e45628_d_n0, assign31390_body4_e45628_d_n2, assign31390_body4_e45628_d_n6, assign31390_body4_e45628_d_n7, assign31390_body4_e45628_d_n10, assign31390_body4_e45628_d_n11, assign31390_body4_e45628_d_n12, assign31390_body4_e45628_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body4_e45609: f64 = (locals.var_chi__blk947 * locals.var_chi__blk947);
        let assign31390_body4_e45612: f64 = (3.0 * 0.29693154855771);
        let assign31390_body4_e45616: f64 = (-0.07053654284009761);
        let assign31390_body4_e45617: f64 = (4.0 * assign31390_body4_e45616);
        let assign31390_body4_e45620: f64 = (locals.var_chi__blk947 * 5.0);
        let assign31390_body4_e45622: f64 = (assign31390_body4_e45620 * 0.006115288895133179);
        let assign31390_body4_e45623: f64 = (assign31390_body4_e45617 + assign31390_body4_e45622);
        let assign31390_body4_e45624: f64 = (locals.var_chi__blk947 * assign31390_body4_e45623);
        let assign31390_body4_e45625: f64 = (assign31390_body4_e45612 + assign31390_body4_e45624);
        let assign31390_body4_e45626: f64 = (assign31390_body4_e45609 * assign31390_body4_e45625);
        (assign31390_body4_e45626, ((((locals.var_chi__blk947_dn0 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn0)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn0 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn2 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn2)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn2 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn6 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn6)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn6 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn7 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn7)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn7 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn10 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn10)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn10 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn11 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn11)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn11 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn12 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn12)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn12 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn17 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn17)) * assign31390_body4_e45625) + (assign31390_body4_e45609 * ((locals.var_chi__blk947_dn17 * assign31390_body4_e45623) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi__blk968, locals.var_fi_dchi__blk968_dn0, locals.var_fi_dchi__blk968_dn2, locals.var_fi_dchi__blk968_dn6, locals.var_fi_dchi__blk968_dn7, locals.var_fi_dchi__blk968_dn10, locals.var_fi_dchi__blk968_dn11, locals.var_fi_dchi__blk968_dn12, locals.var_fi_dchi__blk968_dn17,)
    }
};
            locals.var_fi_dchi__blk968 = assign31390_body4_e45628;
            locals.var_fi_dchi__blk968_dn0 = assign31390_body4_e45628_d_n0;
            locals.var_fi_dchi__blk968_dn2 = assign31390_body4_e45628_d_n2;
            locals.var_fi_dchi__blk968_dn6 = assign31390_body4_e45628_d_n6;
            locals.var_fi_dchi__blk968_dn7 = assign31390_body4_e45628_d_n7;
            locals.var_fi_dchi__blk968_dn10 = assign31390_body4_e45628_d_n10;
            locals.var_fi_dchi__blk968_dn11 = assign31390_body4_e45628_d_n11;
            locals.var_fi_dchi__blk968_dn12 = assign31390_body4_e45628_d_n12;
            locals.var_fi_dchi__blk968_dn17 = assign31390_body4_e45628_d_n17;
            locals.var_fi_dchi__blk968_rv = 0.0;
            let (assign31390_body5_e45648, assign31390_body5_e45648_d_n0, assign31390_body5_e45648_d_n2, assign31390_body5_e45648_d_n6, assign31390_body5_e45648_d_n7, assign31390_body5_e45648_d_n10, assign31390_body5_e45648_d_n11, assign31390_body5_e45648_d_n12, assign31390_body5_e45648_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body5_e45644: f64 = (locals.var_cfs1__blk975 * locals.var_fi__blk967);
        let assign31390_body5_e45646: f64 = (assign31390_body5_e45644 * locals.var_fi__blk967);
        (assign31390_body5_e45646, ((((locals.var_cfs1__blk975_dn0 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn0)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn0)), ((((locals.var_cfs1__blk975_dn2 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn2)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn2)), ((((locals.var_cfs1__blk975_dn6 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn6)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn6)), ((((locals.var_cfs1__blk975_dn7 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn7)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn7)), ((((locals.var_cfs1__blk975_dn10 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn10)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn10)), ((((locals.var_cfs1__blk975_dn11 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn11)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn11)), ((((locals.var_cfs1__blk975_dn12 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn12)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn12)), ((((locals.var_cfs1__blk975_dn17 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn17)) * locals.var_fi__blk967) + (assign31390_body5_e45644 * locals.var_fi__blk967_dn17)),)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
            locals.var_fs01__blk969 = assign31390_body5_e45648;
            locals.var_fs01__blk969_dn0 = assign31390_body5_e45648_d_n0;
            locals.var_fs01__blk969_dn2 = assign31390_body5_e45648_d_n2;
            locals.var_fs01__blk969_dn6 = assign31390_body5_e45648_d_n6;
            locals.var_fs01__blk969_dn7 = assign31390_body5_e45648_d_n7;
            locals.var_fs01__blk969_dn10 = assign31390_body5_e45648_d_n10;
            locals.var_fs01__blk969_dn11 = assign31390_body5_e45648_d_n11;
            locals.var_fs01__blk969_dn12 = assign31390_body5_e45648_d_n12;
            locals.var_fs01__blk969_dn17 = assign31390_body5_e45648_d_n17;
            locals.var_fs01__blk969_rv = 0.0;
            let (assign31390_body6_e45672, assign31390_body6_e45672_d_n0, assign31390_body6_e45672_d_n2, assign31390_body6_e45672_d_n6, assign31390_body6_e45672_d_n7, assign31390_body6_e45672_d_n10, assign31390_body6_e45672_d_n11, assign31390_body6_e45672_d_n12, assign31390_body6_e45672_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body6_e45664: f64 = (locals.var_cfs1__blk975 * locals.var_beta);
        let assign31390_body6_e45666: f64 = (assign31390_body6_e45664 * 2.0);
        let assign31390_body6_e45668: f64 = (assign31390_body6_e45666 * locals.var_fi__blk967);
        let assign31390_body6_e45670: f64 = (assign31390_body6_e45668 * locals.var_fi_dchi__blk968);
        (assign31390_body6_e45670, ((((((locals.var_cfs1__blk975_dn0 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn0)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn0)), ((((((locals.var_cfs1__blk975_dn2 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn2)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn2)), ((((((locals.var_cfs1__blk975_dn6 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn6)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn6)), ((((((locals.var_cfs1__blk975_dn7 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn7)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn7)), (((((((locals.var_cfs1__blk975_dn10 * locals.var_beta) + (locals.var_cfs1__blk975 * locals.var_beta_dn10)) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn10)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn10)), ((((((locals.var_cfs1__blk975_dn11 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn11)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn11)), ((((((locals.var_cfs1__blk975_dn12 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn12)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn12)), ((((((locals.var_cfs1__blk975_dn17 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign31390_body6_e45666 * locals.var_fi__blk967_dn17)) * locals.var_fi_dchi__blk968) + (assign31390_body6_e45668 * locals.var_fi_dchi__blk968_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk970, locals.var_fs01_dps0__blk970_dn0, locals.var_fs01_dps0__blk970_dn2, locals.var_fs01_dps0__blk970_dn6, locals.var_fs01_dps0__blk970_dn7, locals.var_fs01_dps0__blk970_dn10, locals.var_fs01_dps0__blk970_dn11, locals.var_fs01_dps0__blk970_dn12, locals.var_fs01_dps0__blk970_dn17,)
    }
};
            locals.var_fs01_dps0__blk970 = assign31390_body6_e45672;
            locals.var_fs01_dps0__blk970_dn0 = assign31390_body6_e45672_d_n0;
            locals.var_fs01_dps0__blk970_dn2 = assign31390_body6_e45672_d_n2;
            locals.var_fs01_dps0__blk970_dn6 = assign31390_body6_e45672_d_n6;
            locals.var_fs01_dps0__blk970_dn7 = assign31390_body6_e45672_d_n7;
            locals.var_fs01_dps0__blk970_dn10 = assign31390_body6_e45672_d_n10;
            locals.var_fs01_dps0__blk970_dn11 = assign31390_body6_e45672_d_n11;
            locals.var_fs01_dps0__blk970_dn12 = assign31390_body6_e45672_d_n12;
            locals.var_fs01_dps0__blk970_dn17 = assign31390_body6_e45672_d_n17;
            locals.var_fs01_dps0__blk970_rv = 0.0;
            let (assign31390_body7_e45708, assign31390_body7_e45708_d_n0, assign31390_body7_e45708_d_n2, assign31390_body7_e45708_d_n6, assign31390_body7_e45708_d_n7, assign31390_body7_e45708_d_n10, assign31390_body7_e45708_d_n11, assign31390_body7_e45708_d_n12, assign31390_body7_e45708_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body7_e45690: f64 = (-0.117851130197758);
        let assign31390_body7_e45695: f64 = (-0.00163730162779191);
        let assign31390_body7_e45698: f64 = (locals.var_chi__blk947 * 6.36964918866352e-5);
        let assign31390_body7_e45699: f64 = (assign31390_body7_e45695 + assign31390_body7_e45698);
        let assign31390_body7_e45700: f64 = (locals.var_chi__blk947 * assign31390_body7_e45699);
        let assign31390_body7_e45701: f64 = (0.0178800506338833 + assign31390_body7_e45700);
        let assign31390_body7_e45702: f64 = (locals.var_chi__blk947 * assign31390_body7_e45701);
        let assign31390_body7_e45703: f64 = (assign31390_body7_e45690 + assign31390_body7_e45702);
        let assign31390_body7_e45704: f64 = (locals.var_chi__blk947 * assign31390_body7_e45703);
        let assign31390_body7_e45705: f64 = (0.707106781186548 + assign31390_body7_e45704);
        let assign31390_body7_e45706: f64 = (locals.var_chi__blk947 * assign31390_body7_e45705);
        (assign31390_body7_e45706, ((locals.var_chi__blk947_dn0 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn2 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn6 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn7 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn10 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn11 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn12 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn17 * assign31390_body7_e45705) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign31390_body7_e45703) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign31390_body7_e45701) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign31390_body7_e45699) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk971, locals.var_fb__blk971_dn0, locals.var_fb__blk971_dn2, locals.var_fb__blk971_dn6, locals.var_fb__blk971_dn7, locals.var_fb__blk971_dn10, locals.var_fb__blk971_dn11, locals.var_fb__blk971_dn12, locals.var_fb__blk971_dn17,)
    }
};
            locals.var_fb__blk971 = assign31390_body7_e45708;
            locals.var_fb__blk971_dn0 = assign31390_body7_e45708_d_n0;
            locals.var_fb__blk971_dn2 = assign31390_body7_e45708_d_n2;
            locals.var_fb__blk971_dn6 = assign31390_body7_e45708_d_n6;
            locals.var_fb__blk971_dn7 = assign31390_body7_e45708_d_n7;
            locals.var_fb__blk971_dn10 = assign31390_body7_e45708_d_n10;
            locals.var_fb__blk971_dn11 = assign31390_body7_e45708_d_n11;
            locals.var_fb__blk971_dn12 = assign31390_body7_e45708_d_n12;
            locals.var_fb__blk971_dn17 = assign31390_body7_e45708_d_n17;
            locals.var_fb__blk971_rv = 0.0;
            let (assign31390_body8_e45750, assign31390_body8_e45750_d_n0, assign31390_body8_e45750_d_n2, assign31390_body8_e45750_d_n6, assign31390_body8_e45750_d_n7, assign31390_body8_e45750_d_n10, assign31390_body8_e45750_d_n11, assign31390_body8_e45750_d_n12, assign31390_body8_e45750_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body8_e45726: f64 = (-0.117851130197758);
        let assign31390_body8_e45727: f64 = (2.0 * assign31390_body8_e45726);
        let assign31390_body8_e45731: f64 = (3.0 * 0.0178800506338833);
        let assign31390_body8_e45735: f64 = (-0.00163730162779191);
        let assign31390_body8_e45736: f64 = (4.0 * assign31390_body8_e45735);
        let assign31390_body8_e45739: f64 = (locals.var_chi__blk947 * 5.0);
        let assign31390_body8_e45741: f64 = (assign31390_body8_e45739 * 6.36964918866352e-5);
        let assign31390_body8_e45742: f64 = (assign31390_body8_e45736 + assign31390_body8_e45741);
        let assign31390_body8_e45743: f64 = (locals.var_chi__blk947 * assign31390_body8_e45742);
        let assign31390_body8_e45744: f64 = (assign31390_body8_e45731 + assign31390_body8_e45743);
        let assign31390_body8_e45745: f64 = (locals.var_chi__blk947 * assign31390_body8_e45744);
        let assign31390_body8_e45746: f64 = (assign31390_body8_e45727 + assign31390_body8_e45745);
        let assign31390_body8_e45747: f64 = (locals.var_chi__blk947 * assign31390_body8_e45746);
        let assign31390_body8_e45748: f64 = (0.707106781186548 + assign31390_body8_e45747);
        (assign31390_body8_e45748, ((locals.var_chi__blk947_dn0 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn2 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn6 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn7 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn10 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn11 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn12 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn17 * assign31390_body8_e45746) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign31390_body8_e45744) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign31390_body8_e45742) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi__blk972, locals.var_fb_dchi__blk972_dn0, locals.var_fb_dchi__blk972_dn2, locals.var_fb_dchi__blk972_dn6, locals.var_fb_dchi__blk972_dn7, locals.var_fb_dchi__blk972_dn10, locals.var_fb_dchi__blk972_dn11, locals.var_fb_dchi__blk972_dn12, locals.var_fb_dchi__blk972_dn17,)
    }
};
            locals.var_fb_dchi__blk972 = assign31390_body8_e45750;
            locals.var_fb_dchi__blk972_dn0 = assign31390_body8_e45750_d_n0;
            locals.var_fb_dchi__blk972_dn2 = assign31390_body8_e45750_d_n2;
            locals.var_fb_dchi__blk972_dn6 = assign31390_body8_e45750_d_n6;
            locals.var_fb_dchi__blk972_dn7 = assign31390_body8_e45750_d_n7;
            locals.var_fb_dchi__blk972_dn10 = assign31390_body8_e45750_d_n10;
            locals.var_fb_dchi__blk972_dn11 = assign31390_body8_e45750_d_n11;
            locals.var_fb_dchi__blk972_dn12 = assign31390_body8_e45750_d_n12;
            locals.var_fb_dchi__blk972_dn17 = assign31390_body8_e45750_d_n17;
            locals.var_fb_dchi__blk972_rv = 0.0;
            let (assign31390_body9_e45773, assign31390_body9_e45773_d_n0, assign31390_body9_e45773_d_n2, assign31390_body9_e45773_d_n6, assign31390_body9_e45773_d_n7, assign31390_body9_e45773_d_n10, assign31390_body9_e45773_d_n11, assign31390_body9_e45773_d_n12, assign31390_body9_e45773_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body9_e45766: f64 = (locals.var_fb__blk971 * locals.var_fb__blk971);
        let assign31390_body9_e45768: f64 = (assign31390_body9_e45766 + locals.var_fs01__blk969);
        let assign31390_body9_e45770: f64 = (assign31390_body9_e45768 + 1e-50);
        let assign31390_body9_e45771: f64 = (assign31390_body9_e45770).sqrt();
        (assign31390_body9_e45771, ((((locals.var_fb__blk971_dn0 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn0)) + locals.var_fs01__blk969_dn0) / (2.0 * assign31390_body9_e45771)), ((((locals.var_fb__blk971_dn2 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn2)) + locals.var_fs01__blk969_dn2) / (2.0 * assign31390_body9_e45771)), ((((locals.var_fb__blk971_dn6 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn6)) + locals.var_fs01__blk969_dn6) / (2.0 * assign31390_body9_e45771)), ((((locals.var_fb__blk971_dn7 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn7)) + locals.var_fs01__blk969_dn7) / (2.0 * assign31390_body9_e45771)), ((((locals.var_fb__blk971_dn10 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn10)) + locals.var_fs01__blk969_dn10) / (2.0 * assign31390_body9_e45771)), ((((locals.var_fb__blk971_dn11 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn11)) + locals.var_fs01__blk969_dn11) / (2.0 * assign31390_body9_e45771)), ((((locals.var_fb__blk971_dn12 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn12)) + locals.var_fs01__blk969_dn12) / (2.0 * assign31390_body9_e45771)), ((((locals.var_fb__blk971_dn17 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn17)) + locals.var_fs01__blk969_dn17) / (2.0 * assign31390_body9_e45771)),)
    } else {
        (locals.var_fs02__blk973, locals.var_fs02__blk973_dn0, locals.var_fs02__blk973_dn2, locals.var_fs02__blk973_dn6, locals.var_fs02__blk973_dn7, locals.var_fs02__blk973_dn10, locals.var_fs02__blk973_dn11, locals.var_fs02__blk973_dn12, locals.var_fs02__blk973_dn17,)
    }
};
            locals.var_fs02__blk973 = assign31390_body9_e45773;
            locals.var_fs02__blk973_dn0 = assign31390_body9_e45773_d_n0;
            locals.var_fs02__blk973_dn2 = assign31390_body9_e45773_d_n2;
            locals.var_fs02__blk973_dn6 = assign31390_body9_e45773_d_n6;
            locals.var_fs02__blk973_dn7 = assign31390_body9_e45773_d_n7;
            locals.var_fs02__blk973_dn10 = assign31390_body9_e45773_d_n10;
            locals.var_fs02__blk973_dn11 = assign31390_body9_e45773_d_n11;
            locals.var_fs02__blk973_dn12 = assign31390_body9_e45773_d_n12;
            locals.var_fs02__blk973_dn17 = assign31390_body9_e45773_d_n17;
            locals.var_fs02__blk973_rv = 0.0;
            let (assign31390_body10_e45801, assign31390_body10_e45801_d_n0, assign31390_body10_e45801_d_n2, assign31390_body10_e45801_d_n6, assign31390_body10_e45801_d_n7, assign31390_body10_e45801_d_n10, assign31390_body10_e45801_d_n11, assign31390_body10_e45801_d_n12, assign31390_body10_e45801_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 != 0.0)) {
        let assign31390_body10_e45789: f64 = (locals.var_beta * locals.var_fb_dchi__blk972);
        let assign31390_body10_e45791: f64 = (assign31390_body10_e45789 * 2.0);
        let assign31390_body10_e45793: f64 = (assign31390_body10_e45791 * locals.var_fb__blk971);
        let assign31390_body10_e45795: f64 = (assign31390_body10_e45793 + locals.var_fs01_dps0__blk970);
        let assign31390_body10_e45798: f64 = (locals.var_fs02__blk973 + locals.var_fs02__blk973);
        let assign31390_body10_e45799: f64 = (assign31390_body10_e45795 / assign31390_body10_e45798);
        (assign31390_body10_e45799, ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn0) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn0)) + locals.var_fs01_dps0__blk970_dn0) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn0 + locals.var_fs02__blk973_dn0))) / (assign31390_body10_e45798 * assign31390_body10_e45798)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn2) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn2)) + locals.var_fs01_dps0__blk970_dn2) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn2 + locals.var_fs02__blk973_dn2))) / (assign31390_body10_e45798 * assign31390_body10_e45798)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn6) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn6)) + locals.var_fs01_dps0__blk970_dn6) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn6 + locals.var_fs02__blk973_dn6))) / (assign31390_body10_e45798 * assign31390_body10_e45798)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn7) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn7)) + locals.var_fs01_dps0__blk970_dn7) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn7 + locals.var_fs02__blk973_dn7))) / (assign31390_body10_e45798 * assign31390_body10_e45798)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi__blk972) + (locals.var_beta * locals.var_fb_dchi__blk972_dn10)) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn10)) + locals.var_fs01_dps0__blk970_dn10) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn10 + locals.var_fs02__blk973_dn10))) / (assign31390_body10_e45798 * assign31390_body10_e45798)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn11) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn11)) + locals.var_fs01_dps0__blk970_dn11) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn11 + locals.var_fs02__blk973_dn11))) / (assign31390_body10_e45798 * assign31390_body10_e45798)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn12) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn12)) + locals.var_fs01_dps0__blk970_dn12) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn12 + locals.var_fs02__blk973_dn12))) / (assign31390_body10_e45798 * assign31390_body10_e45798)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn17) * 2.0) * locals.var_fb__blk971) + (assign31390_body10_e45791 * locals.var_fb__blk971_dn17)) + locals.var_fs01_dps0__blk970_dn17) * assign31390_body10_e45798) - (assign31390_body10_e45795 * (locals.var_fs02__blk973_dn17 + locals.var_fs02__blk973_dn17))) / (assign31390_body10_e45798 * assign31390_body10_e45798)),)
    } else {
        (locals.var_fs02_dps0__blk974, locals.var_fs02_dps0__blk974_dn0, locals.var_fs02_dps0__blk974_dn2, locals.var_fs02_dps0__blk974_dn6, locals.var_fs02_dps0__blk974_dn7, locals.var_fs02_dps0__blk974_dn10, locals.var_fs02_dps0__blk974_dn11, locals.var_fs02_dps0__blk974_dn12, locals.var_fs02_dps0__blk974_dn17,)
    }
};
            locals.var_fs02_dps0__blk974 = assign31390_body10_e45801;
            locals.var_fs02_dps0__blk974_dn0 = assign31390_body10_e45801_d_n0;
            locals.var_fs02_dps0__blk974_dn2 = assign31390_body10_e45801_d_n2;
            locals.var_fs02_dps0__blk974_dn6 = assign31390_body10_e45801_d_n6;
            locals.var_fs02_dps0__blk974_dn7 = assign31390_body10_e45801_d_n7;
            locals.var_fs02_dps0__blk974_dn10 = assign31390_body10_e45801_d_n10;
            locals.var_fs02_dps0__blk974_dn11 = assign31390_body10_e45801_d_n11;
            locals.var_fs02_dps0__blk974_dn12 = assign31390_body10_e45801_d_n12;
            locals.var_fs02_dps0__blk974_dn17 = assign31390_body10_e45801_d_n17;
            locals.var_fs02_dps0__blk974_rv = 0.0;
            let assign31390_body11_e45804: f64 = if locals.var_chi__blk947 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard1015 = assign31390_body11_e45804;
            locals.var_guard1015_rv = 0.0;
            let (assign31390_body12_e45824, assign31390_body12_e45824_d_n0, assign31390_body12_e45824_d_n2, assign31390_body12_e45824_d_n6, assign31390_body12_e45824_d_n7, assign31390_body12_e45824_d_n10, assign31390_body12_e45824_d_n11, assign31390_body12_e45824_d_n12, assign31390_body12_e45824_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 != 0.0)) {
        let assign31390_body12_e45822: f64 = (locals.var_chi__blk947).exp();
        (assign31390_body12_e45822, (assign31390_body12_e45822 * locals.var_chi__blk947_dn0), (assign31390_body12_e45822 * locals.var_chi__blk947_dn2), (assign31390_body12_e45822 * locals.var_chi__blk947_dn6), (assign31390_body12_e45822 * locals.var_chi__blk947_dn7), (assign31390_body12_e45822 * locals.var_chi__blk947_dn10), (assign31390_body12_e45822 * locals.var_chi__blk947_dn11), (assign31390_body12_e45822 * locals.var_chi__blk947_dn12), (assign31390_body12_e45822 * locals.var_chi__blk947_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign31390_body12_e45824;
            locals.var_exp_chi_dn0 = assign31390_body12_e45824_d_n0;
            locals.var_exp_chi_dn2 = assign31390_body12_e45824_d_n2;
            locals.var_exp_chi_dn6 = assign31390_body12_e45824_d_n6;
            locals.var_exp_chi_dn7 = assign31390_body12_e45824_d_n7;
            locals.var_exp_chi_dn10 = assign31390_body12_e45824_d_n10;
            locals.var_exp_chi_dn11 = assign31390_body12_e45824_d_n11;
            locals.var_exp_chi_dn12 = assign31390_body12_e45824_d_n12;
            locals.var_exp_chi_dn17 = assign31390_body12_e45824_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign31390_body13_e45847, assign31390_body13_e45847_d_n0, assign31390_body13_e45847_d_n2, assign31390_body13_e45847_d_n6, assign31390_body13_e45847_d_n7, assign31390_body13_e45847_d_n10, assign31390_body13_e45847_d_n11, assign31390_body13_e45847_d_n12, assign31390_body13_e45847_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 != 0.0)) {
        let assign31390_body13_e45844: f64 = (locals.var_exp_chi - 1.0);
        let assign31390_body13_e45845: f64 = (locals.var_cfs1__blk975 * assign31390_body13_e45844);
        (assign31390_body13_e45845, ((locals.var_cfs1__blk975_dn0 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk975_dn2 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk975_dn6 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk975_dn7 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk975_dn10 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk975_dn11 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk975_dn12 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk975_dn17 * assign31390_body13_e45844) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
            locals.var_fs01__blk969 = assign31390_body13_e45847;
            locals.var_fs01__blk969_dn0 = assign31390_body13_e45847_d_n0;
            locals.var_fs01__blk969_dn2 = assign31390_body13_e45847_d_n2;
            locals.var_fs01__blk969_dn6 = assign31390_body13_e45847_d_n6;
            locals.var_fs01__blk969_dn7 = assign31390_body13_e45847_d_n7;
            locals.var_fs01__blk969_dn10 = assign31390_body13_e45847_d_n10;
            locals.var_fs01__blk969_dn11 = assign31390_body13_e45847_d_n11;
            locals.var_fs01__blk969_dn12 = assign31390_body13_e45847_d_n12;
            locals.var_fs01__blk969_dn17 = assign31390_body13_e45847_d_n17;
            locals.var_fs01__blk969_rv = 0.0;
            let (assign31390_body14_e45870, assign31390_body14_e45870_d_n0, assign31390_body14_e45870_d_n2, assign31390_body14_e45870_d_n6, assign31390_body14_e45870_d_n7, assign31390_body14_e45870_d_n10, assign31390_body14_e45870_d_n11, assign31390_body14_e45870_d_n12, assign31390_body14_e45870_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 != 0.0)) {
        let assign31390_body14_e45866: f64 = (locals.var_cfs1__blk975 * locals.var_beta);
        let assign31390_body14_e45868: f64 = (assign31390_body14_e45866 * locals.var_exp_chi);
        (assign31390_body14_e45868, (((locals.var_cfs1__blk975_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk975_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk975_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk975_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk975_dn10 * locals.var_beta) + (locals.var_cfs1__blk975 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk975_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk975_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk975_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign31390_body14_e45866 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk970, locals.var_fs01_dps0__blk970_dn0, locals.var_fs01_dps0__blk970_dn2, locals.var_fs01_dps0__blk970_dn6, locals.var_fs01_dps0__blk970_dn7, locals.var_fs01_dps0__blk970_dn10, locals.var_fs01_dps0__blk970_dn11, locals.var_fs01_dps0__blk970_dn12, locals.var_fs01_dps0__blk970_dn17,)
    }
};
            locals.var_fs01_dps0__blk970 = assign31390_body14_e45870;
            locals.var_fs01_dps0__blk970_dn0 = assign31390_body14_e45870_d_n0;
            locals.var_fs01_dps0__blk970_dn2 = assign31390_body14_e45870_d_n2;
            locals.var_fs01_dps0__blk970_dn6 = assign31390_body14_e45870_d_n6;
            locals.var_fs01_dps0__blk970_dn7 = assign31390_body14_e45870_d_n7;
            locals.var_fs01_dps0__blk970_dn10 = assign31390_body14_e45870_d_n10;
            locals.var_fs01_dps0__blk970_dn11 = assign31390_body14_e45870_d_n11;
            locals.var_fs01_dps0__blk970_dn12 = assign31390_body14_e45870_d_n12;
            locals.var_fs01_dps0__blk970_dn17 = assign31390_body14_e45870_d_n17;
            locals.var_fs01_dps0__blk970_rv = 0.0;
            let (assign31390_body15_e45893, assign31390_body15_e45893_d_n0, assign31390_body15_e45893_d_n2, assign31390_body15_e45893_d_n6, assign31390_body15_e45893_d_n7, assign31390_body15_e45893_d_n10, assign31390_body15_e45893_d_n11, assign31390_body15_e45893_d_n12, assign31390_body15_e45893_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 == 0.0)) {
        let assign31390_body15_e45890: f64 = (locals.var_beta * locals.var_ps0ld__blk949);
        let assign31390_body15_e45891: f64 = (assign31390_body15_e45890).exp();
        (assign31390_body15_e45891, (assign31390_body15_e45891 * (locals.var_beta * locals.var_ps0ld__blk949_dn0)), (assign31390_body15_e45891 * (locals.var_beta * locals.var_ps0ld__blk949_dn2)), (assign31390_body15_e45891 * (locals.var_beta * locals.var_ps0ld__blk949_dn6)), (assign31390_body15_e45891 * (locals.var_beta * locals.var_ps0ld__blk949_dn7)), (assign31390_body15_e45891 * ((locals.var_beta_dn10 * locals.var_ps0ld__blk949) + (locals.var_beta * locals.var_ps0ld__blk949_dn10))), (assign31390_body15_e45891 * (locals.var_beta * locals.var_ps0ld__blk949_dn11)), (assign31390_body15_e45891 * (locals.var_beta * locals.var_ps0ld__blk949_dn12)), (assign31390_body15_e45891 * (locals.var_beta * locals.var_ps0ld__blk949_dn17)),)
    } else {
        (locals.var_exp_bps0__blk976, locals.var_exp_bps0__blk976_dn0, locals.var_exp_bps0__blk976_dn2, locals.var_exp_bps0__blk976_dn6, locals.var_exp_bps0__blk976_dn7, locals.var_exp_bps0__blk976_dn10, locals.var_exp_bps0__blk976_dn11, locals.var_exp_bps0__blk976_dn12, locals.var_exp_bps0__blk976_dn17,)
    }
};
            locals.var_exp_bps0__blk976 = assign31390_body15_e45893;
            locals.var_exp_bps0__blk976_dn0 = assign31390_body15_e45893_d_n0;
            locals.var_exp_bps0__blk976_dn2 = assign31390_body15_e45893_d_n2;
            locals.var_exp_bps0__blk976_dn6 = assign31390_body15_e45893_d_n6;
            locals.var_exp_bps0__blk976_dn7 = assign31390_body15_e45893_d_n7;
            locals.var_exp_bps0__blk976_dn10 = assign31390_body15_e45893_d_n10;
            locals.var_exp_bps0__blk976_dn11 = assign31390_body15_e45893_d_n11;
            locals.var_exp_bps0__blk976_dn12 = assign31390_body15_e45893_d_n12;
            locals.var_exp_bps0__blk976_dn17 = assign31390_body15_e45893_d_n17;
            locals.var_exp_bps0__blk976_rv = 0.0;
            let (assign31390_body16_e45917, assign31390_body16_e45917_d_n0, assign31390_body16_e45917_d_n2, assign31390_body16_e45917_d_n6, assign31390_body16_e45917_d_n7, assign31390_body16_e45917_d_n10, assign31390_body16_e45917_d_n11, assign31390_body16_e45917_d_n12, assign31390_body16_e45917_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 == 0.0)) {
        let assign31390_body16_e45914: f64 = (locals.var_exp_bps0__blk976 - locals.var_exp_bvbs__blk966);
        let assign31390_body16_e45915: f64 = (locals.var_cnst1over__blk960 * assign31390_body16_e45914);
        (assign31390_body16_e45915, ((locals.var_cnst1over__blk960_dn0 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn0 - locals.var_exp_bvbs__blk966_dn0))), ((locals.var_cnst1over__blk960_dn2 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn2 - locals.var_exp_bvbs__blk966_dn2))), ((locals.var_cnst1over__blk960_dn6 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn6 - locals.var_exp_bvbs__blk966_dn6))), ((locals.var_cnst1over__blk960_dn7 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn7 - locals.var_exp_bvbs__blk966_dn7))), ((locals.var_cnst1over__blk960_dn10 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn10 - locals.var_exp_bvbs__blk966_dn10))), ((locals.var_cnst1over__blk960_dn11 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn11 - locals.var_exp_bvbs__blk966_dn11))), ((locals.var_cnst1over__blk960_dn12 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn12 - locals.var_exp_bvbs__blk966_dn12))), ((locals.var_cnst1over__blk960_dn17 * assign31390_body16_e45914) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn17 - locals.var_exp_bvbs__blk966_dn17))),)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
            locals.var_fs01__blk969 = assign31390_body16_e45917;
            locals.var_fs01__blk969_dn0 = assign31390_body16_e45917_d_n0;
            locals.var_fs01__blk969_dn2 = assign31390_body16_e45917_d_n2;
            locals.var_fs01__blk969_dn6 = assign31390_body16_e45917_d_n6;
            locals.var_fs01__blk969_dn7 = assign31390_body16_e45917_d_n7;
            locals.var_fs01__blk969_dn10 = assign31390_body16_e45917_d_n10;
            locals.var_fs01__blk969_dn11 = assign31390_body16_e45917_d_n11;
            locals.var_fs01__blk969_dn12 = assign31390_body16_e45917_d_n12;
            locals.var_fs01__blk969_dn17 = assign31390_body16_e45917_d_n17;
            locals.var_fs01__blk969_rv = 0.0;
            let (assign31390_body17_e45941, assign31390_body17_e45941_d_n0, assign31390_body17_e45941_d_n2, assign31390_body17_e45941_d_n6, assign31390_body17_e45941_d_n7, assign31390_body17_e45941_d_n10, assign31390_body17_e45941_d_n11, assign31390_body17_e45941_d_n12, assign31390_body17_e45941_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 == 0.0)) {
        let assign31390_body17_e45937: f64 = (locals.var_cnst1over__blk960 * locals.var_beta);
        let assign31390_body17_e45939: f64 = (assign31390_body17_e45937 * locals.var_exp_bps0__blk976);
        (assign31390_body17_e45939, (((locals.var_cnst1over__blk960_dn0 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn0)), (((locals.var_cnst1over__blk960_dn2 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn2)), (((locals.var_cnst1over__blk960_dn6 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn6)), (((locals.var_cnst1over__blk960_dn7 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn7)), ((((locals.var_cnst1over__blk960_dn10 * locals.var_beta) + (locals.var_cnst1over__blk960 * locals.var_beta_dn10)) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn10)), (((locals.var_cnst1over__blk960_dn11 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn11)), (((locals.var_cnst1over__blk960_dn12 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn12)), (((locals.var_cnst1over__blk960_dn17 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign31390_body17_e45937 * locals.var_exp_bps0__blk976_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk970, locals.var_fs01_dps0__blk970_dn0, locals.var_fs01_dps0__blk970_dn2, locals.var_fs01_dps0__blk970_dn6, locals.var_fs01_dps0__blk970_dn7, locals.var_fs01_dps0__blk970_dn10, locals.var_fs01_dps0__blk970_dn11, locals.var_fs01_dps0__blk970_dn12, locals.var_fs01_dps0__blk970_dn17,)
    }
};
            locals.var_fs01_dps0__blk970 = assign31390_body17_e45941;
            locals.var_fs01_dps0__blk970_dn0 = assign31390_body17_e45941_d_n0;
            locals.var_fs01_dps0__blk970_dn2 = assign31390_body17_e45941_d_n2;
            locals.var_fs01_dps0__blk970_dn6 = assign31390_body17_e45941_d_n6;
            locals.var_fs01_dps0__blk970_dn7 = assign31390_body17_e45941_d_n7;
            locals.var_fs01_dps0__blk970_dn10 = assign31390_body17_e45941_d_n10;
            locals.var_fs01_dps0__blk970_dn11 = assign31390_body17_e45941_d_n11;
            locals.var_fs01_dps0__blk970_dn12 = assign31390_body17_e45941_d_n12;
            locals.var_fs01_dps0__blk970_dn17 = assign31390_body17_e45941_d_n17;
            locals.var_fs01_dps0__blk970_rv = 0.0;
            let (assign31390_body18_e45963, assign31390_body18_e45963_d_n0, assign31390_body18_e45963_d_n2, assign31390_body18_e45963_d_n6, assign31390_body18_e45963_d_n7, assign31390_body18_e45963_d_n10, assign31390_body18_e45963_d_n11, assign31390_body18_e45963_d_n12, assign31390_body18_e45963_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) {
        let assign31390_body18_e45958: f64 = (locals.var_chi__blk947 - 1.0);
        let assign31390_body18_e45960: f64 = (assign31390_body18_e45958 + locals.var_fs01__blk969);
        let assign31390_body18_e45961: f64 = (assign31390_body18_e45960).sqrt();
        (assign31390_body18_e45961, ((locals.var_chi__blk947_dn0 + locals.var_fs01__blk969_dn0) / (2.0 * assign31390_body18_e45961)), ((locals.var_chi__blk947_dn2 + locals.var_fs01__blk969_dn2) / (2.0 * assign31390_body18_e45961)), ((locals.var_chi__blk947_dn6 + locals.var_fs01__blk969_dn6) / (2.0 * assign31390_body18_e45961)), ((locals.var_chi__blk947_dn7 + locals.var_fs01__blk969_dn7) / (2.0 * assign31390_body18_e45961)), ((locals.var_chi__blk947_dn10 + locals.var_fs01__blk969_dn10) / (2.0 * assign31390_body18_e45961)), ((locals.var_chi__blk947_dn11 + locals.var_fs01__blk969_dn11) / (2.0 * assign31390_body18_e45961)), ((locals.var_chi__blk947_dn12 + locals.var_fs01__blk969_dn12) / (2.0 * assign31390_body18_e45961)), ((locals.var_chi__blk947_dn17 + locals.var_fs01__blk969_dn17) / (2.0 * assign31390_body18_e45961)),)
    } else {
        (locals.var_fs02__blk973, locals.var_fs02__blk973_dn0, locals.var_fs02__blk973_dn2, locals.var_fs02__blk973_dn6, locals.var_fs02__blk973_dn7, locals.var_fs02__blk973_dn10, locals.var_fs02__blk973_dn11, locals.var_fs02__blk973_dn12, locals.var_fs02__blk973_dn17,)
    }
};
            locals.var_fs02__blk973 = assign31390_body18_e45963;
            locals.var_fs02__blk973_dn0 = assign31390_body18_e45963_d_n0;
            locals.var_fs02__blk973_dn2 = assign31390_body18_e45963_d_n2;
            locals.var_fs02__blk973_dn6 = assign31390_body18_e45963_d_n6;
            locals.var_fs02__blk973_dn7 = assign31390_body18_e45963_d_n7;
            locals.var_fs02__blk973_dn10 = assign31390_body18_e45963_d_n10;
            locals.var_fs02__blk973_dn11 = assign31390_body18_e45963_d_n11;
            locals.var_fs02__blk973_dn12 = assign31390_body18_e45963_d_n12;
            locals.var_fs02__blk973_dn17 = assign31390_body18_e45963_d_n17;
            locals.var_fs02__blk973_rv = 0.0;
            let (assign31390_body19_e45986, assign31390_body19_e45986_d_n0, assign31390_body19_e45986_d_n2, assign31390_body19_e45986_d_n6, assign31390_body19_e45986_d_n7, assign31390_body19_e45986_d_n10, assign31390_body19_e45986_d_n11, assign31390_body19_e45986_d_n12, assign31390_body19_e45986_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1014 == 0.0)) {
        let assign31390_body19_e45980: f64 = (locals.var_beta + locals.var_fs01_dps0__blk970);
        let assign31390_body19_e45982: f64 = (assign31390_body19_e45980 / locals.var_fs02__blk973);
        let assign31390_body19_e45984: f64 = (assign31390_body19_e45982 * 0.5);
        (assign31390_body19_e45984, ((((locals.var_fs01_dps0__blk970_dn0 * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn0)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn2 * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn2)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn6 * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn6)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn7 * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn7)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk970_dn10) * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn10)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn11 * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn11)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn12 * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn12)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn17 * locals.var_fs02__blk973) - (assign31390_body19_e45980 * locals.var_fs02__blk973_dn17)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk974, locals.var_fs02_dps0__blk974_dn0, locals.var_fs02_dps0__blk974_dn2, locals.var_fs02_dps0__blk974_dn6, locals.var_fs02_dps0__blk974_dn7, locals.var_fs02_dps0__blk974_dn10, locals.var_fs02_dps0__blk974_dn11, locals.var_fs02_dps0__blk974_dn12, locals.var_fs02_dps0__blk974_dn17,)
    }
};
            locals.var_fs02_dps0__blk974 = assign31390_body19_e45986;
            locals.var_fs02_dps0__blk974_dn0 = assign31390_body19_e45986_d_n0;
            locals.var_fs02_dps0__blk974_dn2 = assign31390_body19_e45986_d_n2;
            locals.var_fs02_dps0__blk974_dn6 = assign31390_body19_e45986_d_n6;
            locals.var_fs02_dps0__blk974_dn7 = assign31390_body19_e45986_d_n7;
            locals.var_fs02_dps0__blk974_dn10 = assign31390_body19_e45986_d_n10;
            locals.var_fs02_dps0__blk974_dn11 = assign31390_body19_e45986_d_n11;
            locals.var_fs02_dps0__blk974_dn12 = assign31390_body19_e45986_d_n12;
            locals.var_fs02_dps0__blk974_dn17 = assign31390_body19_e45986_d_n17;
            locals.var_fs02_dps0__blk974_rv = 0.0;
            let (assign31390_body20_e46006, assign31390_body20_e46006_d_n0, assign31390_body20_e46006_d_n2, assign31390_body20_e46006_d_n6, assign31390_body20_e46006_d_n7, assign31390_body20_e46006_d_n10, assign31390_body20_e46006_d_n11, assign31390_body20_e46006_d_n12, assign31390_body20_e46006_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31390_body20_e46000: f64 = (locals.var_vgpld__blk935 - locals.var_ps0ld__blk949);
        let assign31390_body20_e46003: f64 = (locals.var_fac1__blk933 * locals.var_fs02__blk973);
        let assign31390_body20_e46004: f64 = (assign31390_body20_e46000 - assign31390_body20_e46003);
        (assign31390_body20_e46004, ((locals.var_vgpld__blk935_dn0 - locals.var_ps0ld__blk949_dn0) - ((locals.var_fac1__blk933_dn0 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn0))), ((locals.var_vgpld__blk935_dn2 - locals.var_ps0ld__blk949_dn2) - ((locals.var_fac1__blk933_dn2 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn2))), ((locals.var_vgpld__blk935_dn6 - locals.var_ps0ld__blk949_dn6) - ((locals.var_fac1__blk933_dn6 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn6))), ((locals.var_vgpld__blk935_dn7 - locals.var_ps0ld__blk949_dn7) - ((locals.var_fac1__blk933_dn7 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn7))), ((locals.var_vgpld__blk935_dn10 - locals.var_ps0ld__blk949_dn10) - ((locals.var_fac1__blk933_dn10 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn10))), ((locals.var_vgpld__blk935_dn11 - locals.var_ps0ld__blk949_dn11) - ((locals.var_fac1__blk933_dn11 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn11))), ((locals.var_vgpld__blk935_dn12 - locals.var_ps0ld__blk949_dn12) - ((locals.var_fac1__blk933_dn12 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn12))), ((locals.var_vgpld__blk935_dn17 - locals.var_ps0ld__blk949_dn17) - ((locals.var_fac1__blk933_dn17 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn17))),)
    } else {
        (locals.var_fs0__blk977, locals.var_fs0__blk977_dn0, locals.var_fs0__blk977_dn2, locals.var_fs0__blk977_dn6, locals.var_fs0__blk977_dn7, locals.var_fs0__blk977_dn10, locals.var_fs0__blk977_dn11, locals.var_fs0__blk977_dn12, locals.var_fs0__blk977_dn17,)
    }
};
            locals.var_fs0__blk977 = assign31390_body20_e46006;
            locals.var_fs0__blk977_dn0 = assign31390_body20_e46006_d_n0;
            locals.var_fs0__blk977_dn2 = assign31390_body20_e46006_d_n2;
            locals.var_fs0__blk977_dn6 = assign31390_body20_e46006_d_n6;
            locals.var_fs0__blk977_dn7 = assign31390_body20_e46006_d_n7;
            locals.var_fs0__blk977_dn10 = assign31390_body20_e46006_d_n10;
            locals.var_fs0__blk977_dn11 = assign31390_body20_e46006_d_n11;
            locals.var_fs0__blk977_dn12 = assign31390_body20_e46006_d_n12;
            locals.var_fs0__blk977_dn17 = assign31390_body20_e46006_d_n17;
            locals.var_fs0__blk977_rv = 0.0;
            let (assign31390_body21_e46025, assign31390_body21_e46025_d_n0, assign31390_body21_e46025_d_n2, assign31390_body21_e46025_d_n6, assign31390_body21_e46025_d_n7, assign31390_body21_e46025_d_n10, assign31390_body21_e46025_d_n11, assign31390_body21_e46025_d_n12, assign31390_body21_e46025_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31390_body21_e46019: f64 = (-1.0);
        let assign31390_body21_e46022: f64 = (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974);
        let assign31390_body21_e46023: f64 = (assign31390_body21_e46019 - assign31390_body21_e46022);
        (assign31390_body21_e46023, (-((locals.var_fac1__blk933_dn0 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn0))), (-((locals.var_fac1__blk933_dn2 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn2))), (-((locals.var_fac1__blk933_dn6 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn6))), (-((locals.var_fac1__blk933_dn7 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn7))), (-((locals.var_fac1__blk933_dn10 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn10))), (-((locals.var_fac1__blk933_dn11 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn11))), (-((locals.var_fac1__blk933_dn12 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn12))), (-((locals.var_fac1__blk933_dn17 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk978, locals.var_fs0_dps0__blk978_dn0, locals.var_fs0_dps0__blk978_dn2, locals.var_fs0_dps0__blk978_dn6, locals.var_fs0_dps0__blk978_dn7, locals.var_fs0_dps0__blk978_dn10, locals.var_fs0_dps0__blk978_dn11, locals.var_fs0_dps0__blk978_dn12, locals.var_fs0_dps0__blk978_dn17,)
    }
};
            locals.var_fs0_dps0__blk978 = assign31390_body21_e46025;
            locals.var_fs0_dps0__blk978_dn0 = assign31390_body21_e46025_d_n0;
            locals.var_fs0_dps0__blk978_dn2 = assign31390_body21_e46025_d_n2;
            locals.var_fs0_dps0__blk978_dn6 = assign31390_body21_e46025_d_n6;
            locals.var_fs0_dps0__blk978_dn7 = assign31390_body21_e46025_d_n7;
            locals.var_fs0_dps0__blk978_dn10 = assign31390_body21_e46025_d_n10;
            locals.var_fs0_dps0__blk978_dn11 = assign31390_body21_e46025_d_n11;
            locals.var_fs0_dps0__blk978_dn12 = assign31390_body21_e46025_d_n12;
            locals.var_fs0_dps0__blk978_dn17 = assign31390_body21_e46025_d_n17;
            locals.var_fs0_dps0__blk978_rv = 0.0;
            let assign31390_body22_e46028: f64 = if locals.var_flg_conv__blk922 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1016 = assign31390_body22_e46028;
            locals.var_guard1016_rv = 0.0;
            let (assign31390_body23_e46048,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1016 != 0.0)) {
        let assign31390_body23_e46044: f64 = (2.0 * 20.0);
        let assign31390_body23_e46046: f64 = (assign31390_body23_e46044 + 1.0);
        (assign31390_body23_e46046,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31390_body23_e46048;
            locals.var_lp_s0_rv = 0.0;
            let (assign31390_body24_e46068, assign31390_body24_e46068_d_n0, assign31390_body24_e46068_d_n2, assign31390_body24_e46068_d_n6, assign31390_body24_e46068_d_n7, assign31390_body24_e46068_d_n10, assign31390_body24_e46068_d_n11, assign31390_body24_e46068_d_n12, assign31390_body24_e46068_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let assign31390_body24_e46064: f64 = (-locals.var_fs0__blk977);
        let assign31390_body24_e46066: f64 = (assign31390_body24_e46064 / locals.var_fs0_dps0__blk978);
        (assign31390_body24_e46066, ((((-locals.var_fs0__blk977_dn0) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn0)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn2) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn2)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn6) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn6)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn7) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn7)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn10) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn10)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn11) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn11)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn12) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn12)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn17) * locals.var_fs0_dps0__blk978) - (assign31390_body24_e46064 * locals.var_fs0_dps0__blk978_dn17)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign31390_body24_e46068;
            locals.var_dps0_dn0 = assign31390_body24_e46068_d_n0;
            locals.var_dps0_dn2 = assign31390_body24_e46068_d_n2;
            locals.var_dps0_dn6 = assign31390_body24_e46068_d_n6;
            locals.var_dps0_dn7 = assign31390_body24_e46068_d_n7;
            locals.var_dps0_dn10 = assign31390_body24_e46068_d_n10;
            locals.var_dps0_dn11 = assign31390_body24_e46068_d_n11;
            locals.var_dps0_dn12 = assign31390_body24_e46068_d_n12;
            locals.var_dps0_dn17 = assign31390_body24_e46068_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign31390_body25_e46098, assign31390_body25_e46098_d_n0, assign31390_body25_e46098_d_n2, assign31390_body25_e46098_d_n6, assign31390_body25_e46098_d_n7, assign31390_body25_e46098_d_n10, assign31390_body25_e46098_d_n11, assign31390_body25_e46098_d_n12, assign31390_body25_e46098_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let assign31390_body25_e46085: f64 = (0.5 * 0.1);
        let assign31390_body25_e46089: f64 = (locals.var_ps0ld__blk949).abs();
        let (assign31390_body25_e46094, assign31390_body25_e46094_d_n0, assign31390_body25_e46094_d_n2, assign31390_body25_e46094_d_n6, assign31390_body25_e46094_d_n7, assign31390_body25_e46094_d_n10, assign31390_body25_e46094_d_n11, assign31390_body25_e46094_d_n12, assign31390_body25_e46094_d_n17,) = {
            if (1.0 >= assign31390_body25_e46089) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31390_body25_e46093: f64 = (locals.var_ps0ld__blk949).abs();
                (assign31390_body25_e46093, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn0 } else { (-locals.var_ps0ld__blk949_dn0) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn2 } else { (-locals.var_ps0ld__blk949_dn2) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn6 } else { (-locals.var_ps0ld__blk949_dn6) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn7 } else { (-locals.var_ps0ld__blk949_dn7) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn10 } else { (-locals.var_ps0ld__blk949_dn10) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn11 } else { (-locals.var_ps0ld__blk949_dn11) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn12 } else { (-locals.var_ps0ld__blk949_dn12) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn17 } else { (-locals.var_ps0ld__blk949_dn17) },)
            }
        };
        let assign31390_body25_e46095: f64 = (1.0 + assign31390_body25_e46094);
        let assign31390_body25_e46096: f64 = (assign31390_body25_e46085 * assign31390_body25_e46095);
        (assign31390_body25_e46096, (assign31390_body25_e46085 * assign31390_body25_e46094_d_n0), (assign31390_body25_e46085 * assign31390_body25_e46094_d_n2), (assign31390_body25_e46085 * assign31390_body25_e46094_d_n6), (assign31390_body25_e46085 * assign31390_body25_e46094_d_n7), (assign31390_body25_e46085 * assign31390_body25_e46094_d_n10), (assign31390_body25_e46085 * assign31390_body25_e46094_d_n11), (assign31390_body25_e46085 * assign31390_body25_e46094_d_n12), (assign31390_body25_e46085 * assign31390_body25_e46094_d_n17),)
    } else {
        (locals.var_dplim__blk979, locals.var_dplim__blk979_dn0, locals.var_dplim__blk979_dn2, locals.var_dplim__blk979_dn6, locals.var_dplim__blk979_dn7, locals.var_dplim__blk979_dn10, locals.var_dplim__blk979_dn11, locals.var_dplim__blk979_dn12, locals.var_dplim__blk979_dn17,)
    }
};
            locals.var_dplim__blk979 = assign31390_body25_e46098;
            locals.var_dplim__blk979_dn0 = assign31390_body25_e46098_d_n0;
            locals.var_dplim__blk979_dn2 = assign31390_body25_e46098_d_n2;
            locals.var_dplim__blk979_dn6 = assign31390_body25_e46098_d_n6;
            locals.var_dplim__blk979_dn7 = assign31390_body25_e46098_d_n7;
            locals.var_dplim__blk979_dn10 = assign31390_body25_e46098_d_n10;
            locals.var_dplim__blk979_dn11 = assign31390_body25_e46098_d_n11;
            locals.var_dplim__blk979_dn12 = assign31390_body25_e46098_d_n12;
            locals.var_dplim__blk979_dn17 = assign31390_body25_e46098_d_n17;
            locals.var_dplim__blk979_rv = 0.0;
            let assign31390_body26_e46100: f64 = (locals.var_dps0).abs();
            let assign31390_body26_e46102: f64 = if assign31390_body26_e46100 > locals.var_dplim__blk979 { 1.0 } else { 0.0 };
            locals.var_guard1017 = assign31390_body26_e46102;
            locals.var_guard1017_rv = 0.0;
            let (assign31390_body27_e46129, assign31390_body27_e46129_d_n0, assign31390_body27_e46129_d_n2, assign31390_body27_e46129_d_n6, assign31390_body27_e46129_d_n7, assign31390_body27_e46129_d_n10, assign31390_body27_e46129_d_n11, assign31390_body27_e46129_d_n12, assign31390_body27_e46129_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1016 == 0.0)) && (locals.var_guard1017 != 0.0)) {
        let (assign31390_body27_e46126,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign31390_body27_e46125: f64 = (-1.0);
                (assign31390_body27_e46125,)
            }
        };
        let assign31390_body27_e46127: f64 = (locals.var_dplim__blk979 * assign31390_body27_e46126);
        (assign31390_body27_e46127, (locals.var_dplim__blk979_dn0 * assign31390_body27_e46126), (locals.var_dplim__blk979_dn2 * assign31390_body27_e46126), (locals.var_dplim__blk979_dn6 * assign31390_body27_e46126), (locals.var_dplim__blk979_dn7 * assign31390_body27_e46126), (locals.var_dplim__blk979_dn10 * assign31390_body27_e46126), (locals.var_dplim__blk979_dn11 * assign31390_body27_e46126), (locals.var_dplim__blk979_dn12 * assign31390_body27_e46126), (locals.var_dplim__blk979_dn17 * assign31390_body27_e46126),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign31390_body27_e46129;
            locals.var_dps0_dn0 = assign31390_body27_e46129_d_n0;
            locals.var_dps0_dn2 = assign31390_body27_e46129_d_n2;
            locals.var_dps0_dn6 = assign31390_body27_e46129_d_n6;
            locals.var_dps0_dn7 = assign31390_body27_e46129_d_n7;
            locals.var_dps0_dn10 = assign31390_body27_e46129_d_n10;
            locals.var_dps0_dn11 = assign31390_body27_e46129_d_n11;
            locals.var_dps0_dn12 = assign31390_body27_e46129_d_n12;
            locals.var_dps0_dn17 = assign31390_body27_e46129_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign31390_body28_e46148, assign31390_body28_e46148_d_n0, assign31390_body28_e46148_d_n2, assign31390_body28_e46148_d_n6, assign31390_body28_e46148_d_n7, assign31390_body28_e46148_d_n10, assign31390_body28_e46148_d_n11, assign31390_body28_e46148_d_n12, assign31390_body28_e46148_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1016 == 0.0)) {
        let assign31390_body28_e46146: f64 = (locals.var_ps0ld__blk949 + locals.var_dps0);
        (assign31390_body28_e46146, (locals.var_ps0ld__blk949_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld__blk949_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld__blk949_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld__blk949_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld__blk949_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld__blk949_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld__blk949_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld__blk949_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld__blk949, locals.var_ps0ld__blk949_dn0, locals.var_ps0ld__blk949_dn2, locals.var_ps0ld__blk949_dn6, locals.var_ps0ld__blk949_dn7, locals.var_ps0ld__blk949_dn10, locals.var_ps0ld__blk949_dn11, locals.var_ps0ld__blk949_dn12, locals.var_ps0ld__blk949_dn17,)
    }
};
            locals.var_ps0ld__blk949 = assign31390_body28_e46148;
            locals.var_ps0ld__blk949_dn0 = assign31390_body28_e46148_d_n0;
            locals.var_ps0ld__blk949_dn2 = assign31390_body28_e46148_d_n2;
            locals.var_ps0ld__blk949_dn6 = assign31390_body28_e46148_d_n6;
            locals.var_ps0ld__blk949_dn7 = assign31390_body28_e46148_d_n7;
            locals.var_ps0ld__blk949_dn10 = assign31390_body28_e46148_d_n10;
            locals.var_ps0ld__blk949_dn11 = assign31390_body28_e46148_d_n11;
            locals.var_ps0ld__blk949_dn12 = assign31390_body28_e46148_d_n12;
            locals.var_ps0ld__blk949_dn17 = assign31390_body28_e46148_d_n17;
            locals.var_ps0ld__blk949_rv = 0.0;
            let assign31390_body29_e46150: f64 = (locals.var_dps0).abs();
            let assign31390_body29_e46154: f64 = (locals.var_fs0__blk977).abs();
            let assign31390_body29_e46157: f64 = if ((assign31390_body29_e46150 <= 5e-12) && (assign31390_body29_e46154 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1018 = assign31390_body29_e46157;
            locals.var_guard1018_rv = 0.0;
            let (assign31390_body30_e46176,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1016 == 0.0)) && (locals.var_guard1018 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk922,)
    }
};
            locals.var_flg_conv__blk922 = assign31390_body30_e46176;
            locals.var_flg_conv__blk922_rv = 0.0;
            let (assign31390_body31_e46192,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31390_body31_e46190: f64 = (locals.var_lp_s0 + 1.0);
        (assign31390_body31_e46190,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31390_body31_e46192;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign31410_e46198: f64 = if locals.var_chi__blk947 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign31410_e46198;
        locals.var_guard1020_rv = 0.0;

        let (assign31450_e46260, assign31450_e46260_d_n0, assign31450_e46260_d_n2, assign31450_e46260_d_n6, assign31450_e46260_d_n7, assign31450_e46260_d_n10, assign31450_e46260_d_n11, assign31450_e46260_d_n12, assign31450_e46260_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31450_e46254: f64 = (locals.var_fb__blk971 * locals.var_fb__blk971);
        let assign31450_e46257: f64 = (10.0 * 2.220446049250313e-16);
        let assign31450_e46258: f64 = (assign31450_e46254 + assign31450_e46257);
        (assign31450_e46258, ((locals.var_fb__blk971_dn0 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn0)), ((locals.var_fb__blk971_dn2 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn2)), ((locals.var_fb__blk971_dn6 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn6)), ((locals.var_fb__blk971_dn7 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn7)), ((locals.var_fb__blk971_dn10 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn10)), ((locals.var_fb__blk971_dn11 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn11)), ((locals.var_fb__blk971_dn12 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn12)), ((locals.var_fb__blk971_dn17 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn17)),)
    } else {
        (locals.var_xi0__blk980, locals.var_xi0__blk980_dn0, locals.var_xi0__blk980_dn2, locals.var_xi0__blk980_dn6, locals.var_xi0__blk980_dn7, locals.var_xi0__blk980_dn10, locals.var_xi0__blk980_dn11, locals.var_xi0__blk980_dn12, locals.var_xi0__blk980_dn17,)
    }
};
        locals.var_xi0__blk980 = assign31450_e46260;
        locals.var_xi0__blk980_dn0 = assign31450_e46260_d_n0;
        locals.var_xi0__blk980_dn2 = assign31450_e46260_d_n2;
        locals.var_xi0__blk980_dn6 = assign31450_e46260_d_n6;
        locals.var_xi0__blk980_dn7 = assign31450_e46260_d_n7;
        locals.var_xi0__blk980_dn10 = assign31450_e46260_d_n10;
        locals.var_xi0__blk980_dn11 = assign31450_e46260_d_n11;
        locals.var_xi0__blk980_dn12 = assign31450_e46260_d_n12;
        locals.var_xi0__blk980_dn17 = assign31450_e46260_d_n17;
        locals.var_xi0__blk980_rv = 0.0;

        let (assign31460_e46280, assign31460_e46280_d_n0, assign31460_e46280_d_n2, assign31460_e46280_d_n6, assign31460_e46280_d_n7, assign31460_e46280_d_n10, assign31460_e46280_d_n11, assign31460_e46280_d_n12, assign31460_e46280_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1020 != 0.0)) {
        let assign31460_e46277: f64 = (10.0 * 2.220446049250313e-16);
        let assign31460_e46278: f64 = (locals.var_fb__blk971 + assign31460_e46277);
        (assign31460_e46278, locals.var_fb__blk971_dn0, locals.var_fb__blk971_dn2, locals.var_fb__blk971_dn6, locals.var_fb__blk971_dn7, locals.var_fb__blk971_dn10, locals.var_fb__blk971_dn11, locals.var_fb__blk971_dn12, locals.var_fb__blk971_dn17,)
    } else {
        (locals.var_xi0p12__blk981, locals.var_xi0p12__blk981_dn0, locals.var_xi0p12__blk981_dn2, locals.var_xi0p12__blk981_dn6, locals.var_xi0p12__blk981_dn7, locals.var_xi0p12__blk981_dn10, locals.var_xi0p12__blk981_dn11, locals.var_xi0p12__blk981_dn12, locals.var_xi0p12__blk981_dn17,)
    }
};
        locals.var_xi0p12__blk981 = assign31460_e46280;
        locals.var_xi0p12__blk981_dn0 = assign31460_e46280_d_n0;
        locals.var_xi0p12__blk981_dn2 = assign31460_e46280_d_n2;
        locals.var_xi0p12__blk981_dn6 = assign31460_e46280_d_n6;
        locals.var_xi0p12__blk981_dn7 = assign31460_e46280_d_n7;
        locals.var_xi0p12__blk981_dn10 = assign31460_e46280_d_n10;
        locals.var_xi0p12__blk981_dn11 = assign31460_e46280_d_n11;
        locals.var_xi0p12__blk981_dn12 = assign31460_e46280_d_n12;
        locals.var_xi0p12__blk981_dn17 = assign31460_e46280_d_n17;
        locals.var_xi0p12__blk981_rv = 0.0;

        let (assign31480_e46316, assign31480_e46316_d_n0, assign31480_e46316_d_n2, assign31480_e46316_d_n6, assign31480_e46316_d_n7, assign31480_e46316_d_n10, assign31480_e46316_d_n11, assign31480_e46316_d_n12, assign31480_e46316_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1020 == 0.0)) {
        let assign31480_e46314: f64 = (locals.var_chi__blk947 - 1.0);
        (assign31480_e46314, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    } else {
        (locals.var_xi0__blk980, locals.var_xi0__blk980_dn0, locals.var_xi0__blk980_dn2, locals.var_xi0__blk980_dn6, locals.var_xi0__blk980_dn7, locals.var_xi0__blk980_dn10, locals.var_xi0__blk980_dn11, locals.var_xi0__blk980_dn12, locals.var_xi0__blk980_dn17,)
    }
};
        locals.var_xi0__blk980 = assign31480_e46316;
        locals.var_xi0__blk980_dn0 = assign31480_e46316_d_n0;
        locals.var_xi0__blk980_dn2 = assign31480_e46316_d_n2;
        locals.var_xi0__blk980_dn6 = assign31480_e46316_d_n6;
        locals.var_xi0__blk980_dn7 = assign31480_e46316_d_n7;
        locals.var_xi0__blk980_dn10 = assign31480_e46316_d_n10;
        locals.var_xi0__blk980_dn11 = assign31480_e46316_d_n11;
        locals.var_xi0__blk980_dn12 = assign31480_e46316_d_n12;
        locals.var_xi0__blk980_dn17 = assign31480_e46316_d_n17;
        locals.var_xi0__blk980_rv = 0.0;

        let (assign31490_e46334, assign31490_e46334_d_n0, assign31490_e46334_d_n2, assign31490_e46334_d_n6, assign31490_e46334_d_n7, assign31490_e46334_d_n10, assign31490_e46334_d_n11, assign31490_e46334_d_n12, assign31490_e46334_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) && (locals.var_guard1020 == 0.0)) {
        let assign31490_e46332: f64 = (locals.var_xi0__blk980).sqrt();
        (assign31490_e46332, (locals.var_xi0__blk980_dn0 / (2.0 * assign31490_e46332)), (locals.var_xi0__blk980_dn2 / (2.0 * assign31490_e46332)), (locals.var_xi0__blk980_dn6 / (2.0 * assign31490_e46332)), (locals.var_xi0__blk980_dn7 / (2.0 * assign31490_e46332)), (locals.var_xi0__blk980_dn10 / (2.0 * assign31490_e46332)), (locals.var_xi0__blk980_dn11 / (2.0 * assign31490_e46332)), (locals.var_xi0__blk980_dn12 / (2.0 * assign31490_e46332)), (locals.var_xi0__blk980_dn17 / (2.0 * assign31490_e46332)),)
    } else {
        (locals.var_xi0p12__blk981, locals.var_xi0p12__blk981_dn0, locals.var_xi0p12__blk981_dn2, locals.var_xi0p12__blk981_dn6, locals.var_xi0p12__blk981_dn7, locals.var_xi0p12__blk981_dn10, locals.var_xi0p12__blk981_dn11, locals.var_xi0p12__blk981_dn12, locals.var_xi0p12__blk981_dn17,)
    }
};
        locals.var_xi0p12__blk981 = assign31490_e46334;
        locals.var_xi0p12__blk981_dn0 = assign31490_e46334_d_n0;
        locals.var_xi0p12__blk981_dn2 = assign31490_e46334_d_n2;
        locals.var_xi0p12__blk981_dn6 = assign31490_e46334_d_n6;
        locals.var_xi0p12__blk981_dn7 = assign31490_e46334_d_n7;
        locals.var_xi0p12__blk981_dn10 = assign31490_e46334_d_n10;
        locals.var_xi0p12__blk981_dn11 = assign31490_e46334_d_n11;
        locals.var_xi0p12__blk981_dn12 = assign31490_e46334_d_n12;
        locals.var_xi0p12__blk981_dn17 = assign31490_e46334_d_n17;
        locals.var_xi0p12__blk981_rv = 0.0;

        let (assign31500_e46350, assign31500_e46350_d_n0, assign31500_e46350_d_n2, assign31500_e46350_d_n6, assign31500_e46350_d_n7, assign31500_e46350_d_n10, assign31500_e46350_d_n11, assign31500_e46350_d_n12, assign31500_e46350_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31500_e46348: f64 = (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981);
        (assign31500_e46348, ((locals.var_cnst0over__blk932_dn0 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn0)), ((locals.var_cnst0over__blk932_dn2 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn2)), ((locals.var_cnst0over__blk932_dn6 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn6)), ((locals.var_cnst0over__blk932_dn7 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn7)), ((locals.var_cnst0over__blk932_dn10 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn10)), ((locals.var_cnst0over__blk932_dn11 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn11)), ((locals.var_cnst0over__blk932_dn12 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn12)), ((locals.var_cnst0over__blk932_dn17 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign31500_e46350;
        locals.var_qbuld_dn0 = assign31500_e46350_d_n0;
        locals.var_qbuld_dn2 = assign31500_e46350_d_n2;
        locals.var_qbuld_dn6 = assign31500_e46350_d_n6;
        locals.var_qbuld_dn7 = assign31500_e46350_d_n7;
        locals.var_qbuld_dn10 = assign31500_e46350_d_n10;
        locals.var_qbuld_dn11 = assign31500_e46350_d_n11;
        locals.var_qbuld_dn12 = assign31500_e46350_d_n12;
        locals.var_qbuld_dn17 = assign31500_e46350_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign31510_e46368, assign31510_e46368_d_n0, assign31510_e46368_d_n2, assign31510_e46368_d_n6, assign31510_e46368_d_n7, assign31510_e46368_d_n10, assign31510_e46368_d_n11, assign31510_e46368_d_n12, assign31510_e46368_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31510_e46365: f64 = (locals.var_fs02__blk973 + locals.var_xi0p12__blk981);
        let assign31510_e46366: f64 = (1.0 / assign31510_e46365);
        (assign31510_e46366, (-((locals.var_fs02__blk973_dn0 + locals.var_xi0p12__blk981_dn0) / (assign31510_e46365 * assign31510_e46365))), (-((locals.var_fs02__blk973_dn2 + locals.var_xi0p12__blk981_dn2) / (assign31510_e46365 * assign31510_e46365))), (-((locals.var_fs02__blk973_dn6 + locals.var_xi0p12__blk981_dn6) / (assign31510_e46365 * assign31510_e46365))), (-((locals.var_fs02__blk973_dn7 + locals.var_xi0p12__blk981_dn7) / (assign31510_e46365 * assign31510_e46365))), (-((locals.var_fs02__blk973_dn10 + locals.var_xi0p12__blk981_dn10) / (assign31510_e46365 * assign31510_e46365))), (-((locals.var_fs02__blk973_dn11 + locals.var_xi0p12__blk981_dn11) / (assign31510_e46365 * assign31510_e46365))), (-((locals.var_fs02__blk973_dn12 + locals.var_xi0p12__blk981_dn12) / (assign31510_e46365 * assign31510_e46365))), (-((locals.var_fs02__blk973_dn17 + locals.var_xi0p12__blk981_dn17) / (assign31510_e46365 * assign31510_e46365))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31510_e46368;
        locals.var_t1__blk900_dn0 = assign31510_e46368_d_n0;
        locals.var_t1__blk900_dn2 = assign31510_e46368_d_n2;
        locals.var_t1__blk900_dn6 = assign31510_e46368_d_n6;
        locals.var_t1__blk900_dn7 = assign31510_e46368_d_n7;
        locals.var_t1__blk900_dn10 = assign31510_e46368_d_n10;
        locals.var_t1__blk900_dn11 = assign31510_e46368_d_n11;
        locals.var_t1__blk900_dn12 = assign31510_e46368_d_n12;
        locals.var_t1__blk900_dn17 = assign31510_e46368_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31520_e46386, assign31520_e46386_d_n0, assign31520_e46386_d_n2, assign31520_e46386_d_n6, assign31520_e46386_d_n7, assign31520_e46386_d_n10, assign31520_e46386_d_n11, assign31520_e46386_d_n12, assign31520_e46386_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31520_e46382: f64 = (locals.var_cnst0over__blk932 * locals.var_fs01__blk969);
        let assign31520_e46384: f64 = (assign31520_e46382 * locals.var_t1__blk900);
        (assign31520_e46384, ((((locals.var_cnst0over__blk932_dn0 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn0)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn0)), ((((locals.var_cnst0over__blk932_dn2 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn2)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn2)), ((((locals.var_cnst0over__blk932_dn6 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn6)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn6)), ((((locals.var_cnst0over__blk932_dn7 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn7)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn7)), ((((locals.var_cnst0over__blk932_dn10 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn10)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn10)), ((((locals.var_cnst0over__blk932_dn11 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn11)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn11)), ((((locals.var_cnst0over__blk932_dn12 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn12)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn12)), ((((locals.var_cnst0over__blk932_dn17 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn17)) * locals.var_t1__blk900) + (assign31520_e46382 * locals.var_t1__blk900_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31520_e46386;
        locals.var_qiuld_dn0 = assign31520_e46386_d_n0;
        locals.var_qiuld_dn2 = assign31520_e46386_d_n2;
        locals.var_qiuld_dn6 = assign31520_e46386_d_n6;
        locals.var_qiuld_dn7 = assign31520_e46386_d_n7;
        locals.var_qiuld_dn10 = assign31520_e46386_d_n10;
        locals.var_qiuld_dn11 = assign31520_e46386_d_n11;
        locals.var_qiuld_dn12 = assign31520_e46386_d_n12;
        locals.var_qiuld_dn17 = assign31520_e46386_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign31530_e46402, assign31530_e46402_d_n0, assign31530_e46402_d_n2, assign31530_e46402_d_n6, assign31530_e46402_d_n7, assign31530_e46402_d_n10, assign31530_e46402_d_n11, assign31530_e46402_d_n12, assign31530_e46402_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 == 0.0)) && (locals.var_guard1013 != 0.0)) {
        let assign31530_e46400: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign31530_e46400, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign31530_e46402;
        locals.var_qsuld_dn0 = assign31530_e46402_d_n0;
        locals.var_qsuld_dn2 = assign31530_e46402_d_n2;
        locals.var_qsuld_dn6 = assign31530_e46402_d_n6;
        locals.var_qsuld_dn7 = assign31530_e46402_d_n7;
        locals.var_qsuld_dn10 = assign31530_e46402_d_n10;
        locals.var_qsuld_dn11 = assign31530_e46402_d_n11;
        locals.var_qsuld_dn12 = assign31530_e46402_d_n12;
        locals.var_qsuld_dn17 = assign31530_e46402_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign31540_e46413, assign31540_e46413_d_n0, assign31540_e46413_d_n2, assign31540_e46413_d_n6, assign31540_e46413_d_n7, assign31540_e46413_d_n10, assign31540_e46413_d_n11, assign31540_e46413_d_n12, assign31540_e46413_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign31540_e46411: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign31540_e46411, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign31540_e46413;
        locals.var_qiuld_dn0 = assign31540_e46413_d_n0;
        locals.var_qiuld_dn2 = assign31540_e46413_d_n2;
        locals.var_qiuld_dn6 = assign31540_e46413_d_n6;
        locals.var_qiuld_dn7 = assign31540_e46413_d_n7;
        locals.var_qiuld_dn10 = assign31540_e46413_d_n10;
        locals.var_qiuld_dn11 = assign31540_e46413_d_n11;
        locals.var_qiuld_dn12 = assign31540_e46413_d_n12;
        locals.var_qiuld_dn17 = assign31540_e46413_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign31550_e46431, assign31550_e46431_d_n0, assign31550_e46431_d_n2, assign31550_e46431_d_n6, assign31550_e46431_d_n7, assign31550_e46431_d_n10, assign31550_e46431_d_n11, assign31550_e46431_d_n12, assign31550_e46431_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let (assign31550_e46429,) = {
            if (p.p43 == 1.0) {
                let assign31550_e46425: f64 = (locals.var_w_dioscv * locals.var_lov);
                (assign31550_e46425,)
            } else {
                let assign31550_e46428: f64 = (locals.var_weffcv_nf * locals.var_lov);
                (assign31550_e46428,)
            }
        };
        (assign31550_e46429, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk903, locals.var_t4__blk903_dn0, locals.var_t4__blk903_dn2, locals.var_t4__blk903_dn6, locals.var_t4__blk903_dn7, locals.var_t4__blk903_dn10, locals.var_t4__blk903_dn11, locals.var_t4__blk903_dn12, locals.var_t4__blk903_dn17,)
    }
};
        locals.var_t4__blk903 = assign31550_e46431;
        locals.var_t4__blk903_dn0 = assign31550_e46431_d_n0;
        locals.var_t4__blk903_dn2 = assign31550_e46431_d_n2;
        locals.var_t4__blk903_dn6 = assign31550_e46431_d_n6;
        locals.var_t4__blk903_dn7 = assign31550_e46431_d_n7;
        locals.var_t4__blk903_dn10 = assign31550_e46431_d_n10;
        locals.var_t4__blk903_dn11 = assign31550_e46431_d_n11;
        locals.var_t4__blk903_dn12 = assign31550_e46431_d_n12;
        locals.var_t4__blk903_dn17 = assign31550_e46431_d_n17;
        locals.var_t4__blk903_rv = 0.0;

        let assign31560_e46442: f64 = if (((locals.var_flg_overs__blk918 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk916 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign31560_e46442;
        locals.var_guard1022_rv = 0.0;

        let (assign31570_e46455, assign31570_e46455_d_n0, assign31570_e46455_d_n2, assign31570_e46455_d_n6, assign31570_e46455_d_n7, assign31570_e46455_d_n10, assign31570_e46455_d_n11, assign31570_e46455_d_n12, assign31570_e46455_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1022 != 0.0)) {
        let assign31570_e46453: f64 = (locals.var_t4__blk903 * locals.var_qsuld);
        (assign31570_e46453, ((locals.var_t4__blk903_dn0 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17,)
    }
};
        locals.var_qovs = assign31570_e46455;
        locals.var_qovs_dn0 = assign31570_e46455_d_n0;
        locals.var_qovs_dn2 = assign31570_e46455_d_n2;
        locals.var_qovs_dn6 = assign31570_e46455_d_n6;
        locals.var_qovs_dn7 = assign31570_e46455_d_n7;
        locals.var_qovs_dn10 = assign31570_e46455_d_n10;
        locals.var_qovs_dn11 = assign31570_e46455_d_n11;
        locals.var_qovs_dn12 = assign31570_e46455_d_n12;
        locals.var_qovs_dn17 = assign31570_e46455_d_n17;
        locals.var_qovs_rv = 0.0;

        let (assign31580_e46468, assign31580_e46468_d_n0, assign31580_e46468_d_n2, assign31580_e46468_d_n6, assign31580_e46468_d_n7, assign31580_e46468_d_n10, assign31580_e46468_d_n11, assign31580_e46468_d_n12, assign31580_e46468_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1022 != 0.0)) {
        let assign31580_e46466: f64 = (locals.var_t4__blk903 * locals.var_qbuld);
        (assign31580_e46466, ((locals.var_t4__blk903_dn0 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17,)
    }
};
        locals.var_qbsld = assign31580_e46468;
        locals.var_qbsld_dn0 = assign31580_e46468_d_n0;
        locals.var_qbsld_dn2 = assign31580_e46468_d_n2;
        locals.var_qbsld_dn6 = assign31580_e46468_d_n6;
        locals.var_qbsld_dn7 = assign31580_e46468_d_n7;
        locals.var_qbsld_dn10 = assign31580_e46468_d_n10;
        locals.var_qbsld_dn11 = assign31580_e46468_d_n11;
        locals.var_qbsld_dn12 = assign31580_e46468_d_n12;
        locals.var_qbsld_dn17 = assign31580_e46468_d_n17;
        locals.var_qbsld_rv = 0.0;

        let assign31590_e46479: f64 = if (((locals.var_flg_overd__blk919 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk917 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign31590_e46479;
        locals.var_guard1023_rv = 0.0;

        let (assign31600_e46492, assign31600_e46492_d_n0, assign31600_e46492_d_n2, assign31600_e46492_d_n6, assign31600_e46492_d_n7, assign31600_e46492_d_n10, assign31600_e46492_d_n11, assign31600_e46492_d_n12, assign31600_e46492_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1023 != 0.0)) {
        let assign31600_e46490: f64 = (locals.var_t4__blk903 * locals.var_qsuld);
        (assign31600_e46490, ((locals.var_t4__blk903_dn0 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17,)
    }
};
        locals.var_qovd = assign31600_e46492;
        locals.var_qovd_dn0 = assign31600_e46492_d_n0;
        locals.var_qovd_dn2 = assign31600_e46492_d_n2;
        locals.var_qovd_dn6 = assign31600_e46492_d_n6;
        locals.var_qovd_dn7 = assign31600_e46492_d_n7;
        locals.var_qovd_dn10 = assign31600_e46492_d_n10;
        locals.var_qovd_dn11 = assign31600_e46492_d_n11;
        locals.var_qovd_dn12 = assign31600_e46492_d_n12;
        locals.var_qovd_dn17 = assign31600_e46492_d_n17;
        locals.var_qovd_rv = 0.0;

        let (assign31610_e46505, assign31610_e46505_d_n0, assign31610_e46505_d_n2, assign31610_e46505_d_n6, assign31610_e46505_d_n7, assign31610_e46505_d_n10, assign31610_e46505_d_n11, assign31610_e46505_d_n12, assign31610_e46505_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1023 != 0.0)) {
        let assign31610_e46503: f64 = (locals.var_t4__blk903 * locals.var_qbuld);
        (assign31610_e46503, ((locals.var_t4__blk903_dn0 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17,)
    }
};
        locals.var_qbdld = assign31610_e46505;
        locals.var_qbdld_dn0 = assign31610_e46505_d_n0;
        locals.var_qbdld_dn2 = assign31610_e46505_d_n2;
        locals.var_qbdld_dn6 = assign31610_e46505_d_n6;
        locals.var_qbdld_dn7 = assign31610_e46505_d_n7;
        locals.var_qbdld_dn10 = assign31610_e46505_d_n10;
        locals.var_qbdld_dn11 = assign31610_e46505_d_n11;
        locals.var_qbdld_dn12 = assign31610_e46505_d_n12;
        locals.var_qbdld_dn17 = assign31610_e46505_d_n17;
        locals.var_qbdld_rv = 0.0;

        let (assign31620_e46517,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) {
        let assign31620_e46511: f64 = (locals.var_modervs * locals.var_cgso_given);
        let assign31620_e46514: f64 = (locals.var_modenml * locals.var_cgdo_given);
        let assign31620_e46515: f64 = (assign31620_e46511 + assign31620_e46514);
        (assign31620_e46515,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31620_e46517;
        locals.var_flg_overgiven_rv = 0.0;

        let (assign31630_e46531, assign31630_e46531_d_n0, assign31630_e46531_d_n2, assign31630_e46531_d_n6, assign31630_e46531_d_n7, assign31630_e46531_d_n10, assign31630_e46531_d_n11, assign31630_e46531_d_n12, assign31630_e46531_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31630_e46525: f64 = (locals.var_modervs * p.p170);
        let assign31630_e46528: f64 = (locals.var_modenml * p.p169);
        let assign31630_e46529: f64 = (assign31630_e46525 + assign31630_e46528);
        (assign31630_e46529, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31630_e46531;
        locals.var_cgdoe_dn0 = assign31630_e46531_d_n0;
        locals.var_cgdoe_dn2 = assign31630_e46531_d_n2;
        locals.var_cgdoe_dn6 = assign31630_e46531_d_n6;
        locals.var_cgdoe_dn7 = assign31630_e46531_d_n7;
        locals.var_cgdoe_dn10 = assign31630_e46531_d_n10;
        locals.var_cgdoe_dn11 = assign31630_e46531_d_n11;
        locals.var_cgdoe_dn12 = assign31630_e46531_d_n12;
        locals.var_cgdoe_dn17 = assign31630_e46531_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let assign31640_e46534: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign31640_e46534;
        locals.var_guard1024_rv = 0.0;

        let (assign31650_e46550, assign31650_e46550_d_n0, assign31650_e46550_d_n2, assign31650_e46550_d_n6, assign31650_e46550_d_n7, assign31650_e46550_d_n10, assign31650_e46550_d_n11, assign31650_e46550_d_n12, assign31650_e46550_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign31650_e46544: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31650_e46547: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31650_e46548: f64 = (assign31650_e46544 + assign31650_e46547);
        (assign31650_e46548, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31650_e46550;
        locals.var_t1__blk900_dn0 = assign31650_e46550_d_n0;
        locals.var_t1__blk900_dn2 = assign31650_e46550_d_n2;
        locals.var_t1__blk900_dn6 = assign31650_e46550_d_n6;
        locals.var_t1__blk900_dn7 = assign31650_e46550_d_n7;
        locals.var_t1__blk900_dn10 = assign31650_e46550_d_n10;
        locals.var_t1__blk900_dn11 = assign31650_e46550_d_n11;
        locals.var_t1__blk900_dn12 = assign31650_e46550_d_n12;
        locals.var_t1__blk900_dn17 = assign31650_e46550_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31660_e46563, assign31660_e46563_d_n0, assign31660_e46563_d_n2, assign31660_e46563_d_n6, assign31660_e46563_d_n7, assign31660_e46563_d_n10, assign31660_e46563_d_n11, assign31660_e46563_d_n12, assign31660_e46563_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign31660_e46560: f64 = (-locals.var_t1__blk900);
        let assign31660_e46561: f64 = (locals.var_cgdoe * assign31660_e46560);
        (assign31660_e46561, ((locals.var_cgdoe_dn0 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn0))), ((locals.var_cgdoe_dn2 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn2))), ((locals.var_cgdoe_dn6 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn6))), ((locals.var_cgdoe_dn7 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn7))), ((locals.var_cgdoe_dn10 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn10))), ((locals.var_cgdoe_dn11 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn11))), ((locals.var_cgdoe_dn12 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn12))), ((locals.var_cgdoe_dn17 * assign31660_e46560) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31660_e46563;
        locals.var_cgdoe_dn0 = assign31660_e46563_d_n0;
        locals.var_cgdoe_dn2 = assign31660_e46563_d_n2;
        locals.var_cgdoe_dn6 = assign31660_e46563_d_n6;
        locals.var_cgdoe_dn7 = assign31660_e46563_d_n7;
        locals.var_cgdoe_dn10 = assign31660_e46563_d_n10;
        locals.var_cgdoe_dn11 = assign31660_e46563_d_n11;
        locals.var_cgdoe_dn12 = assign31660_e46563_d_n12;
        locals.var_cgdoe_dn17 = assign31660_e46563_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31670_e46577, assign31670_e46577_d_n0, assign31670_e46577_d_n2, assign31670_e46577_d_n6, assign31670_e46577_d_n7, assign31670_e46577_d_n10, assign31670_e46577_d_n11, assign31670_e46577_d_n12, assign31670_e46577_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1024 == 0.0)) {
        let assign31670_e46574: f64 = (-locals.var_weffcv_nf);
        let assign31670_e46575: f64 = (locals.var_cgdoe * assign31670_e46574);
        (assign31670_e46575, (locals.var_cgdoe_dn0 * assign31670_e46574), (locals.var_cgdoe_dn2 * assign31670_e46574), (locals.var_cgdoe_dn6 * assign31670_e46574), (locals.var_cgdoe_dn7 * assign31670_e46574), (locals.var_cgdoe_dn10 * assign31670_e46574), (locals.var_cgdoe_dn11 * assign31670_e46574), (locals.var_cgdoe_dn12 * assign31670_e46574), (locals.var_cgdoe_dn17 * assign31670_e46574),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31670_e46577;
        locals.var_cgdoe_dn0 = assign31670_e46577_d_n0;
        locals.var_cgdoe_dn2 = assign31670_e46577_d_n2;
        locals.var_cgdoe_dn6 = assign31670_e46577_d_n6;
        locals.var_cgdoe_dn7 = assign31670_e46577_d_n7;
        locals.var_cgdoe_dn10 = assign31670_e46577_d_n10;
        locals.var_cgdoe_dn11 = assign31670_e46577_d_n11;
        locals.var_cgdoe_dn12 = assign31670_e46577_d_n12;
        locals.var_cgdoe_dn17 = assign31670_e46577_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31680_e46592, assign31680_e46592_d_n0, assign31680_e46592_d_n2, assign31680_e46592_d_n6, assign31680_e46592_d_n7, assign31680_e46592_d_n10, assign31680_e46592_d_n11, assign31680_e46592_d_n12, assign31680_e46592_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31680_e46585: f64 = (-locals.var_cgdoe);
        let assign31680_e46588: f64 = (locals.var_vgs - locals.var_vds);
        let assign31680_e46589: f64 = (assign31680_e46585 * assign31680_e46588);
        let assign31680_e46590: f64 = (locals.var_qgod + assign31680_e46589);
        (assign31680_e46590, (locals.var_qgod_dn0 + (((-locals.var_cgdoe_dn0) * assign31680_e46588) + (assign31680_e46585 * (-locals.var_vds_dn0)))), (locals.var_qgod_dn2 + (((-locals.var_cgdoe_dn2) * assign31680_e46588) + (assign31680_e46585 * (-locals.var_vds_dn2)))), (locals.var_qgod_dn6 + (((-locals.var_cgdoe_dn6) * assign31680_e46588) + (assign31680_e46585 * (locals.var_vgs_dn6 - locals.var_vds_dn6)))), (locals.var_qgod_dn7 + (((-locals.var_cgdoe_dn7) * assign31680_e46588) + (assign31680_e46585 * (locals.var_vgs_dn7 - locals.var_vds_dn7)))), (locals.var_qgod_dn10 + (((-locals.var_cgdoe_dn10) * assign31680_e46588) + (assign31680_e46585 * (-locals.var_vds_dn10)))), (locals.var_qgod_dn11 + (((-locals.var_cgdoe_dn11) * assign31680_e46588) + (assign31680_e46585 * (locals.var_vgs_dn11 - locals.var_vds_dn11)))), (locals.var_qgod_dn12 + (((-locals.var_cgdoe_dn12) * assign31680_e46588) + (assign31680_e46585 * (-locals.var_vds_dn12)))), (locals.var_qgod_dn17 + (((-locals.var_cgdoe_dn17) * assign31680_e46588) + (assign31680_e46585 * (-locals.var_vds_dn17)))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31680_e46592;
        locals.var_qgod_dn0 = assign31680_e46592_d_n0;
        locals.var_qgod_dn2 = assign31680_e46592_d_n2;
        locals.var_qgod_dn6 = assign31680_e46592_d_n6;
        locals.var_qgod_dn7 = assign31680_e46592_d_n7;
        locals.var_qgod_dn10 = assign31680_e46592_d_n10;
        locals.var_qgod_dn11 = assign31680_e46592_d_n11;
        locals.var_qgod_dn12 = assign31680_e46592_d_n12;
        locals.var_qgod_dn17 = assign31680_e46592_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign31690_e46604,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) {
        let assign31690_e46598: f64 = (locals.var_modenml * locals.var_cgso_given);
        let assign31690_e46601: f64 = (locals.var_modervs * locals.var_cgdo_given);
        let assign31690_e46602: f64 = (assign31690_e46598 + assign31690_e46601);
        (assign31690_e46602,)
    } else {
        (locals.var_flg_overgiven,)
    }
};
        locals.var_flg_overgiven = assign31690_e46604;
        locals.var_flg_overgiven_rv = 0.0;

        let (assign31700_e46618, assign31700_e46618_d_n0, assign31700_e46618_d_n2, assign31700_e46618_d_n6, assign31700_e46618_d_n7, assign31700_e46618_d_n10, assign31700_e46618_d_n11, assign31700_e46618_d_n12, assign31700_e46618_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31700_e46612: f64 = (locals.var_modenml * p.p170);
        let assign31700_e46615: f64 = (locals.var_modervs * p.p169);
        let assign31700_e46616: f64 = (assign31700_e46612 + assign31700_e46615);
        (assign31700_e46616, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31700_e46618;
        locals.var_cgsoe_dn0 = assign31700_e46618_d_n0;
        locals.var_cgsoe_dn2 = assign31700_e46618_d_n2;
        locals.var_cgsoe_dn6 = assign31700_e46618_d_n6;
        locals.var_cgsoe_dn7 = assign31700_e46618_d_n7;
        locals.var_cgsoe_dn10 = assign31700_e46618_d_n10;
        locals.var_cgsoe_dn11 = assign31700_e46618_d_n11;
        locals.var_cgsoe_dn12 = assign31700_e46618_d_n12;
        locals.var_cgsoe_dn17 = assign31700_e46618_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let assign31710_e46621: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign31710_e46621;
        locals.var_guard1025_rv = 0.0;

        let (assign31720_e46637, assign31720_e46637_d_n0, assign31720_e46637_d_n2, assign31720_e46637_d_n6, assign31720_e46637_d_n7, assign31720_e46637_d_n10, assign31720_e46637_d_n11, assign31720_e46637_d_n12, assign31720_e46637_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1025 != 0.0)) {
        let assign31720_e46631: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31720_e46634: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31720_e46635: f64 = (assign31720_e46631 + assign31720_e46634);
        (assign31720_e46635, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31720_e46637;
        locals.var_t1__blk900_dn0 = assign31720_e46637_d_n0;
        locals.var_t1__blk900_dn2 = assign31720_e46637_d_n2;
        locals.var_t1__blk900_dn6 = assign31720_e46637_d_n6;
        locals.var_t1__blk900_dn7 = assign31720_e46637_d_n7;
        locals.var_t1__blk900_dn10 = assign31720_e46637_d_n10;
        locals.var_t1__blk900_dn11 = assign31720_e46637_d_n11;
        locals.var_t1__blk900_dn12 = assign31720_e46637_d_n12;
        locals.var_t1__blk900_dn17 = assign31720_e46637_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31730_e46650, assign31730_e46650_d_n0, assign31730_e46650_d_n2, assign31730_e46650_d_n6, assign31730_e46650_d_n7, assign31730_e46650_d_n10, assign31730_e46650_d_n11, assign31730_e46650_d_n12, assign31730_e46650_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1025 != 0.0)) {
        let assign31730_e46647: f64 = (-locals.var_t1__blk900);
        let assign31730_e46648: f64 = (locals.var_cgsoe * assign31730_e46647);
        (assign31730_e46648, ((locals.var_cgsoe_dn0 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn0))), ((locals.var_cgsoe_dn2 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn2))), ((locals.var_cgsoe_dn6 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn6))), ((locals.var_cgsoe_dn7 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn7))), ((locals.var_cgsoe_dn10 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn10))), ((locals.var_cgsoe_dn11 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn11))), ((locals.var_cgsoe_dn12 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn12))), ((locals.var_cgsoe_dn17 * assign31730_e46647) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31730_e46650;
        locals.var_cgsoe_dn0 = assign31730_e46650_d_n0;
        locals.var_cgsoe_dn2 = assign31730_e46650_d_n2;
        locals.var_cgsoe_dn6 = assign31730_e46650_d_n6;
        locals.var_cgsoe_dn7 = assign31730_e46650_d_n7;
        locals.var_cgsoe_dn10 = assign31730_e46650_d_n10;
        locals.var_cgsoe_dn11 = assign31730_e46650_d_n11;
        locals.var_cgsoe_dn12 = assign31730_e46650_d_n12;
        locals.var_cgsoe_dn17 = assign31730_e46650_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31740_e46664, assign31740_e46664_d_n0, assign31740_e46664_d_n2, assign31740_e46664_d_n6, assign31740_e46664_d_n7, assign31740_e46664_d_n10, assign31740_e46664_d_n11, assign31740_e46664_d_n12, assign31740_e46664_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1025 == 0.0)) {
        let assign31740_e46661: f64 = (-locals.var_weffcv_nf);
        let assign31740_e46662: f64 = (locals.var_cgsoe * assign31740_e46661);
        (assign31740_e46662, (locals.var_cgsoe_dn0 * assign31740_e46661), (locals.var_cgsoe_dn2 * assign31740_e46661), (locals.var_cgsoe_dn6 * assign31740_e46661), (locals.var_cgsoe_dn7 * assign31740_e46661), (locals.var_cgsoe_dn10 * assign31740_e46661), (locals.var_cgsoe_dn11 * assign31740_e46661), (locals.var_cgsoe_dn12 * assign31740_e46661), (locals.var_cgsoe_dn17 * assign31740_e46661),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31740_e46664;
        locals.var_cgsoe_dn0 = assign31740_e46664_d_n0;
        locals.var_cgsoe_dn2 = assign31740_e46664_d_n2;
        locals.var_cgsoe_dn6 = assign31740_e46664_d_n6;
        locals.var_cgsoe_dn7 = assign31740_e46664_d_n7;
        locals.var_cgsoe_dn10 = assign31740_e46664_d_n10;
        locals.var_cgsoe_dn11 = assign31740_e46664_d_n11;
        locals.var_cgsoe_dn12 = assign31740_e46664_d_n12;
        locals.var_cgsoe_dn17 = assign31740_e46664_d_n17;
        locals.var_cgsoe_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31750_e46677, assign31750_e46677_d_n0, assign31750_e46677_d_n2, assign31750_e46677_d_n6, assign31750_e46677_d_n7, assign31750_e46677_d_n10, assign31750_e46677_d_n11, assign31750_e46677_d_n12, assign31750_e46677_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
        let assign31750_e46672: f64 = (-locals.var_cgsoe);
        let assign31750_e46674: f64 = (assign31750_e46672 * locals.var_vgs);
        let assign31750_e46675: f64 = (locals.var_qgos + assign31750_e46674);
        (assign31750_e46675, (locals.var_qgos_dn0 + ((-locals.var_cgsoe_dn0) * locals.var_vgs)), (locals.var_qgos_dn2 + ((-locals.var_cgsoe_dn2) * locals.var_vgs)), (locals.var_qgos_dn6 + (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31750_e46672 * locals.var_vgs_dn6))), (locals.var_qgos_dn7 + (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31750_e46672 * locals.var_vgs_dn7))), (locals.var_qgos_dn10 + ((-locals.var_cgsoe_dn10) * locals.var_vgs)), (locals.var_qgos_dn11 + (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31750_e46672 * locals.var_vgs_dn11))), (locals.var_qgos_dn12 + ((-locals.var_cgsoe_dn12) * locals.var_vgs)), (locals.var_qgos_dn17 + ((-locals.var_cgsoe_dn17) * locals.var_vgs)),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31750_e46677;
        locals.var_qgos_dn0 = assign31750_e46677_d_n0;
        locals.var_qgos_dn2 = assign31750_e46677_d_n2;
        locals.var_qgos_dn6 = assign31750_e46677_d_n6;
        locals.var_qgos_dn7 = assign31750_e46677_d_n7;
        locals.var_qgos_dn10 = assign31750_e46677_d_n10;
        locals.var_qgos_dn11 = assign31750_e46677_d_n11;
        locals.var_qgos_dn12 = assign31750_e46677_d_n12;
        locals.var_qgos_dn17 = assign31750_e46677_d_n17;
        locals.var_qgos_rv = 0.0;

        let assign31760_e46690: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgdo_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign31760_e46690;
        locals.var_guard1026_rv = 0.0;

        let assign31770_e46693: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign31770_e46693;
        locals.var_guard1027_rv = 0.0;

        let (assign31780_e46709, assign31780_e46709_d_n0, assign31780_e46709_d_n2, assign31780_e46709_d_n6, assign31780_e46709_d_n7, assign31780_e46709_d_n10, assign31780_e46709_d_n11, assign31780_e46709_d_n12, assign31780_e46709_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1026 != 0.0)) && (locals.var_guard1027 != 0.0)) {
        let assign31780_e46703: f64 = (-locals.var_cox0__blk910);
        let assign31780_e46705: f64 = (assign31780_e46703 * p.p188);
        let assign31780_e46707: f64 = (assign31780_e46705 * locals.var_w_diodcv);
        (assign31780_e46707, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31780_e46709;
        locals.var_cgdoe_dn0 = assign31780_e46709_d_n0;
        locals.var_cgdoe_dn2 = assign31780_e46709_d_n2;
        locals.var_cgdoe_dn6 = assign31780_e46709_d_n6;
        locals.var_cgdoe_dn7 = assign31780_e46709_d_n7;
        locals.var_cgdoe_dn10 = assign31780_e46709_d_n10;
        locals.var_cgdoe_dn11 = assign31780_e46709_d_n11;
        locals.var_cgdoe_dn12 = assign31780_e46709_d_n12;
        locals.var_cgdoe_dn17 = assign31780_e46709_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31790_e46726, assign31790_e46726_d_n0, assign31790_e46726_d_n2, assign31790_e46726_d_n6, assign31790_e46726_d_n7, assign31790_e46726_d_n10, assign31790_e46726_d_n11, assign31790_e46726_d_n12, assign31790_e46726_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1026 != 0.0)) && (locals.var_guard1027 == 0.0)) {
        let assign31790_e46720: f64 = (-locals.var_cox0__blk910);
        let assign31790_e46722: f64 = (assign31790_e46720 * p.p188);
        let assign31790_e46724: f64 = (assign31790_e46722 * locals.var_weffcv_nf);
        (assign31790_e46724, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31790_e46726;
        locals.var_cgdoe_dn0 = assign31790_e46726_d_n0;
        locals.var_cgdoe_dn2 = assign31790_e46726_d_n2;
        locals.var_cgdoe_dn6 = assign31790_e46726_d_n6;
        locals.var_cgdoe_dn7 = assign31790_e46726_d_n7;
        locals.var_cgdoe_dn10 = assign31790_e46726_d_n10;
        locals.var_cgdoe_dn11 = assign31790_e46726_d_n11;
        locals.var_cgdoe_dn12 = assign31790_e46726_d_n12;
        locals.var_cgdoe_dn17 = assign31790_e46726_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31800_e46742, assign31800_e46742_d_n0, assign31800_e46742_d_n2, assign31800_e46742_d_n6, assign31800_e46742_d_n7, assign31800_e46742_d_n10, assign31800_e46742_d_n11, assign31800_e46742_d_n12, assign31800_e46742_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1026 == 0.0)) {
        let assign31800_e46736: f64 = (locals.var_modervs * p.p170);
        let assign31800_e46739: f64 = (locals.var_modenml * p.p169);
        let assign31800_e46740: f64 = (assign31800_e46736 + assign31800_e46739);
        (assign31800_e46740, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31800_e46742;
        locals.var_cgdoe_dn0 = assign31800_e46742_d_n0;
        locals.var_cgdoe_dn2 = assign31800_e46742_d_n2;
        locals.var_cgdoe_dn6 = assign31800_e46742_d_n6;
        locals.var_cgdoe_dn7 = assign31800_e46742_d_n7;
        locals.var_cgdoe_dn10 = assign31800_e46742_d_n10;
        locals.var_cgdoe_dn11 = assign31800_e46742_d_n11;
        locals.var_cgdoe_dn12 = assign31800_e46742_d_n12;
        locals.var_cgdoe_dn17 = assign31800_e46742_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let assign31810_e46745: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign31810_e46745;
        locals.var_guard1028_rv = 0.0;

        let (assign31820_e46763, assign31820_e46763_d_n0, assign31820_e46763_d_n2, assign31820_e46763_d_n6, assign31820_e46763_d_n7, assign31820_e46763_d_n10, assign31820_e46763_d_n11, assign31820_e46763_d_n12, assign31820_e46763_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1026 == 0.0)) && (locals.var_guard1028 != 0.0)) {
        let assign31820_e46757: f64 = (locals.var_modervs * locals.var_w_dioscv);
        let assign31820_e46760: f64 = (locals.var_modenml * locals.var_w_diodcv);
        let assign31820_e46761: f64 = (assign31820_e46757 + assign31820_e46760);
        (assign31820_e46761, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31820_e46763;
        locals.var_t1__blk900_dn0 = assign31820_e46763_d_n0;
        locals.var_t1__blk900_dn2 = assign31820_e46763_d_n2;
        locals.var_t1__blk900_dn6 = assign31820_e46763_d_n6;
        locals.var_t1__blk900_dn7 = assign31820_e46763_d_n7;
        locals.var_t1__blk900_dn10 = assign31820_e46763_d_n10;
        locals.var_t1__blk900_dn11 = assign31820_e46763_d_n11;
        locals.var_t1__blk900_dn12 = assign31820_e46763_d_n12;
        locals.var_t1__blk900_dn17 = assign31820_e46763_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31830_e46778, assign31830_e46778_d_n0, assign31830_e46778_d_n2, assign31830_e46778_d_n6, assign31830_e46778_d_n7, assign31830_e46778_d_n10, assign31830_e46778_d_n11, assign31830_e46778_d_n12, assign31830_e46778_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1026 == 0.0)) && (locals.var_guard1028 != 0.0)) {
        let assign31830_e46775: f64 = (-locals.var_t1__blk900);
        let assign31830_e46776: f64 = (locals.var_cgdoe * assign31830_e46775);
        (assign31830_e46776, ((locals.var_cgdoe_dn0 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn0))), ((locals.var_cgdoe_dn2 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn2))), ((locals.var_cgdoe_dn6 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn6))), ((locals.var_cgdoe_dn7 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn7))), ((locals.var_cgdoe_dn10 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn10))), ((locals.var_cgdoe_dn11 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn11))), ((locals.var_cgdoe_dn12 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn12))), ((locals.var_cgdoe_dn17 * assign31830_e46775) + (locals.var_cgdoe * (-locals.var_t1__blk900_dn17))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31830_e46778;
        locals.var_cgdoe_dn0 = assign31830_e46778_d_n0;
        locals.var_cgdoe_dn2 = assign31830_e46778_d_n2;
        locals.var_cgdoe_dn6 = assign31830_e46778_d_n6;
        locals.var_cgdoe_dn7 = assign31830_e46778_d_n7;
        locals.var_cgdoe_dn10 = assign31830_e46778_d_n10;
        locals.var_cgdoe_dn11 = assign31830_e46778_d_n11;
        locals.var_cgdoe_dn12 = assign31830_e46778_d_n12;
        locals.var_cgdoe_dn17 = assign31830_e46778_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31840_e46794, assign31840_e46794_d_n0, assign31840_e46794_d_n2, assign31840_e46794_d_n6, assign31840_e46794_d_n7, assign31840_e46794_d_n10, assign31840_e46794_d_n11, assign31840_e46794_d_n12, assign31840_e46794_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1026 == 0.0)) && (locals.var_guard1028 == 0.0)) {
        let assign31840_e46791: f64 = (-locals.var_weffcv_nf);
        let assign31840_e46792: f64 = (locals.var_cgdoe * assign31840_e46791);
        (assign31840_e46792, (locals.var_cgdoe_dn0 * assign31840_e46791), (locals.var_cgdoe_dn2 * assign31840_e46791), (locals.var_cgdoe_dn6 * assign31840_e46791), (locals.var_cgdoe_dn7 * assign31840_e46791), (locals.var_cgdoe_dn10 * assign31840_e46791), (locals.var_cgdoe_dn11 * assign31840_e46791), (locals.var_cgdoe_dn12 * assign31840_e46791), (locals.var_cgdoe_dn17 * assign31840_e46791),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17,)
    }
};
        locals.var_cgdoe = assign31840_e46794;
        locals.var_cgdoe_dn0 = assign31840_e46794_d_n0;
        locals.var_cgdoe_dn2 = assign31840_e46794_d_n2;
        locals.var_cgdoe_dn6 = assign31840_e46794_d_n6;
        locals.var_cgdoe_dn7 = assign31840_e46794_d_n7;
        locals.var_cgdoe_dn10 = assign31840_e46794_d_n10;
        locals.var_cgdoe_dn11 = assign31840_e46794_d_n11;
        locals.var_cgdoe_dn12 = assign31840_e46794_d_n12;
        locals.var_cgdoe_dn17 = assign31840_e46794_d_n17;
        locals.var_cgdoe_rv = 0.0;

        let (assign31850_e46806, assign31850_e46806_d_n0, assign31850_e46806_d_n2, assign31850_e46806_d_n6, assign31850_e46806_d_n7, assign31850_e46806_d_n10, assign31850_e46806_d_n11, assign31850_e46806_d_n12, assign31850_e46806_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) {
        let assign31850_e46800: f64 = (-locals.var_cgdoe);
        let assign31850_e46803: f64 = (locals.var_vgs - locals.var_vds);
        let assign31850_e46804: f64 = (assign31850_e46800 * assign31850_e46803);
        (assign31850_e46804, (((-locals.var_cgdoe_dn0) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31850_e46803) + (assign31850_e46800 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31850_e46803) + (assign31850_e46800 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31850_e46803) + (assign31850_e46800 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign31850_e46806;
        locals.var_qgod_dn0 = assign31850_e46806_d_n0;
        locals.var_qgod_dn2 = assign31850_e46806_d_n2;
        locals.var_qgod_dn6 = assign31850_e46806_d_n6;
        locals.var_qgod_dn7 = assign31850_e46806_d_n7;
        locals.var_qgod_dn10 = assign31850_e46806_d_n10;
        locals.var_qgod_dn11 = assign31850_e46806_d_n11;
        locals.var_qgod_dn12 = assign31850_e46806_d_n12;
        locals.var_qgod_dn17 = assign31850_e46806_d_n17;
        locals.var_qgod_rv = 0.0;

        let assign31860_e46819: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1029 = assign31860_e46819;
        locals.var_guard1029_rv = 0.0;

        let assign31870_e46822: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1030 = assign31870_e46822;
        locals.var_guard1030_rv = 0.0;

        let (assign31880_e46838, assign31880_e46838_d_n0, assign31880_e46838_d_n2, assign31880_e46838_d_n6, assign31880_e46838_d_n7, assign31880_e46838_d_n10, assign31880_e46838_d_n11, assign31880_e46838_d_n12, assign31880_e46838_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 != 0.0)) && (locals.var_guard1030 != 0.0)) {
        let assign31880_e46832: f64 = (-locals.var_cox0__blk910);
        let assign31880_e46834: f64 = (assign31880_e46832 * p.p188);
        let assign31880_e46836: f64 = (assign31880_e46834 * locals.var_w_dioscv);
        (assign31880_e46836, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31880_e46838;
        locals.var_cgsoe_dn0 = assign31880_e46838_d_n0;
        locals.var_cgsoe_dn2 = assign31880_e46838_d_n2;
        locals.var_cgsoe_dn6 = assign31880_e46838_d_n6;
        locals.var_cgsoe_dn7 = assign31880_e46838_d_n7;
        locals.var_cgsoe_dn10 = assign31880_e46838_d_n10;
        locals.var_cgsoe_dn11 = assign31880_e46838_d_n11;
        locals.var_cgsoe_dn12 = assign31880_e46838_d_n12;
        locals.var_cgsoe_dn17 = assign31880_e46838_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31890_e46855, assign31890_e46855_d_n0, assign31890_e46855_d_n2, assign31890_e46855_d_n6, assign31890_e46855_d_n7, assign31890_e46855_d_n10, assign31890_e46855_d_n11, assign31890_e46855_d_n12, assign31890_e46855_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 != 0.0)) && (locals.var_guard1030 == 0.0)) {
        let assign31890_e46849: f64 = (-locals.var_cox0__blk910);
        let assign31890_e46851: f64 = (assign31890_e46849 * p.p188);
        let assign31890_e46853: f64 = (assign31890_e46851 * locals.var_weffcv_nf);
        (assign31890_e46853, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31890_e46855;
        locals.var_cgsoe_dn0 = assign31890_e46855_d_n0;
        locals.var_cgsoe_dn2 = assign31890_e46855_d_n2;
        locals.var_cgsoe_dn6 = assign31890_e46855_d_n6;
        locals.var_cgsoe_dn7 = assign31890_e46855_d_n7;
        locals.var_cgsoe_dn10 = assign31890_e46855_d_n10;
        locals.var_cgsoe_dn11 = assign31890_e46855_d_n11;
        locals.var_cgsoe_dn12 = assign31890_e46855_d_n12;
        locals.var_cgsoe_dn17 = assign31890_e46855_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31900_e46871, assign31900_e46871_d_n0, assign31900_e46871_d_n2, assign31900_e46871_d_n6, assign31900_e46871_d_n7, assign31900_e46871_d_n10, assign31900_e46871_d_n11, assign31900_e46871_d_n12, assign31900_e46871_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) {
        let assign31900_e46865: f64 = (locals.var_modenml * p.p170);
        let assign31900_e46868: f64 = (locals.var_modervs * p.p169);
        let assign31900_e46869: f64 = (assign31900_e46865 + assign31900_e46868);
        (assign31900_e46869, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31900_e46871;
        locals.var_cgsoe_dn0 = assign31900_e46871_d_n0;
        locals.var_cgsoe_dn2 = assign31900_e46871_d_n2;
        locals.var_cgsoe_dn6 = assign31900_e46871_d_n6;
        locals.var_cgsoe_dn7 = assign31900_e46871_d_n7;
        locals.var_cgsoe_dn10 = assign31900_e46871_d_n10;
        locals.var_cgsoe_dn11 = assign31900_e46871_d_n11;
        locals.var_cgsoe_dn12 = assign31900_e46871_d_n12;
        locals.var_cgsoe_dn17 = assign31900_e46871_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let assign31910_e46874: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1031 = assign31910_e46874;
        locals.var_guard1031_rv = 0.0;

        let (assign31920_e46892, assign31920_e46892_d_n0, assign31920_e46892_d_n2, assign31920_e46892_d_n6, assign31920_e46892_d_n7, assign31920_e46892_d_n10, assign31920_e46892_d_n11, assign31920_e46892_d_n12, assign31920_e46892_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) && (locals.var_guard1031 != 0.0)) {
        let assign31920_e46886: f64 = (locals.var_modenml * locals.var_w_dioscv);
        let assign31920_e46889: f64 = (locals.var_modervs * locals.var_w_diodcv);
        let assign31920_e46890: f64 = (assign31920_e46886 + assign31920_e46889);
        (assign31920_e46890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign31920_e46892;
        locals.var_t1__blk900_dn0 = assign31920_e46892_d_n0;
        locals.var_t1__blk900_dn2 = assign31920_e46892_d_n2;
        locals.var_t1__blk900_dn6 = assign31920_e46892_d_n6;
        locals.var_t1__blk900_dn7 = assign31920_e46892_d_n7;
        locals.var_t1__blk900_dn10 = assign31920_e46892_d_n10;
        locals.var_t1__blk900_dn11 = assign31920_e46892_d_n11;
        locals.var_t1__blk900_dn12 = assign31920_e46892_d_n12;
        locals.var_t1__blk900_dn17 = assign31920_e46892_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign31930_e46907, assign31930_e46907_d_n0, assign31930_e46907_d_n2, assign31930_e46907_d_n6, assign31930_e46907_d_n7, assign31930_e46907_d_n10, assign31930_e46907_d_n11, assign31930_e46907_d_n12, assign31930_e46907_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) && (locals.var_guard1031 != 0.0)) {
        let assign31930_e46904: f64 = (-locals.var_t1__blk900);
        let assign31930_e46905: f64 = (locals.var_cgsoe * assign31930_e46904);
        (assign31930_e46905, ((locals.var_cgsoe_dn0 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn0))), ((locals.var_cgsoe_dn2 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn2))), ((locals.var_cgsoe_dn6 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn6))), ((locals.var_cgsoe_dn7 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn7))), ((locals.var_cgsoe_dn10 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn10))), ((locals.var_cgsoe_dn11 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn11))), ((locals.var_cgsoe_dn12 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn12))), ((locals.var_cgsoe_dn17 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn17))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31930_e46907;
        locals.var_cgsoe_dn0 = assign31930_e46907_d_n0;
        locals.var_cgsoe_dn2 = assign31930_e46907_d_n2;
        locals.var_cgsoe_dn6 = assign31930_e46907_d_n6;
        locals.var_cgsoe_dn7 = assign31930_e46907_d_n7;
        locals.var_cgsoe_dn10 = assign31930_e46907_d_n10;
        locals.var_cgsoe_dn11 = assign31930_e46907_d_n11;
        locals.var_cgsoe_dn12 = assign31930_e46907_d_n12;
        locals.var_cgsoe_dn17 = assign31930_e46907_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31940_e46923, assign31940_e46923_d_n0, assign31940_e46923_d_n2, assign31940_e46923_d_n6, assign31940_e46923_d_n7, assign31940_e46923_d_n10, assign31940_e46923_d_n11, assign31940_e46923_d_n12, assign31940_e46923_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) && (locals.var_guard1031 == 0.0)) {
        let assign31940_e46920: f64 = (-locals.var_weffcv_nf);
        let assign31940_e46921: f64 = (locals.var_cgsoe * assign31940_e46920);
        (assign31940_e46921, (locals.var_cgsoe_dn0 * assign31940_e46920), (locals.var_cgsoe_dn2 * assign31940_e46920), (locals.var_cgsoe_dn6 * assign31940_e46920), (locals.var_cgsoe_dn7 * assign31940_e46920), (locals.var_cgsoe_dn10 * assign31940_e46920), (locals.var_cgsoe_dn11 * assign31940_e46920), (locals.var_cgsoe_dn12 * assign31940_e46920), (locals.var_cgsoe_dn17 * assign31940_e46920),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17,)
    }
};
        locals.var_cgsoe = assign31940_e46923;
        locals.var_cgsoe_dn0 = assign31940_e46923_d_n0;
        locals.var_cgsoe_dn2 = assign31940_e46923_d_n2;
        locals.var_cgsoe_dn6 = assign31940_e46923_d_n6;
        locals.var_cgsoe_dn7 = assign31940_e46923_d_n7;
        locals.var_cgsoe_dn10 = assign31940_e46923_d_n10;
        locals.var_cgsoe_dn11 = assign31940_e46923_d_n11;
        locals.var_cgsoe_dn12 = assign31940_e46923_d_n12;
        locals.var_cgsoe_dn17 = assign31940_e46923_d_n17;
        locals.var_cgsoe_rv = 0.0;

        let (assign31950_e46933, assign31950_e46933_d_n0, assign31950_e46933_d_n2, assign31950_e46933_d_n6, assign31950_e46933_d_n7, assign31950_e46933_d_n10, assign31950_e46933_d_n11, assign31950_e46933_d_n12, assign31950_e46933_d_n17,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) {
        let assign31950_e46929: f64 = (-locals.var_cgsoe);
        let assign31950_e46931: f64 = (assign31950_e46929 * locals.var_vgs);
        (assign31950_e46931, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31950_e46929 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31950_e46929 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31950_e46929 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign31950_e46933;
        locals.var_qgos_dn0 = assign31950_e46933_d_n0;
        locals.var_qgos_dn2 = assign31950_e46933_d_n2;
        locals.var_qgos_dn6 = assign31950_e46933_d_n6;
        locals.var_qgos_dn7 = assign31950_e46933_d_n7;
        locals.var_qgos_dn10 = assign31950_e46933_d_n10;
        locals.var_qgos_dn11 = assign31950_e46933_d_n11;
        locals.var_qgos_dn12 = assign31950_e46933_d_n12;
        locals.var_qgos_dn17 = assign31950_e46933_d_n17;
        locals.var_qgos_rv = 0.0;

        let assign31960_e46936: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1032 = assign31960_e46936;
        locals.var_guard1032_rv = 0.0;

        let (assign31970_e46940, assign31970_e46940_d_n6, assign31970_e46940_d_n12,) = {
    if (locals.var_guard1032 != 0.0) {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    } else {
        (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12,)
    }
};
        locals.var_vbdj = assign31970_e46940;
        locals.var_vbdj_dn6 = assign31970_e46940_d_n6;
        locals.var_vbdj_dn12 = assign31970_e46940_d_n12;
        locals.var_vbdj_rv = 0.0;

        let (assign31980_e46944, assign31980_e46944_d_n7, assign31980_e46944_d_n12,) = {
    if (locals.var_guard1032 != 0.0) {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    } else {
        (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12,)
    }
};
        locals.var_vbsj = assign31980_e46944;
        locals.var_vbsj_dn7 = assign31980_e46944_d_n7;
        locals.var_vbsj_dn12 = assign31980_e46944_d_n12;
        locals.var_vbsj_rv = 0.0;

        let (assign31990_e46966, assign31990_e46966_d_n0, assign31990_e46966_d_n2, assign31990_e46966_d_n6, assign31990_e46966_d_n7, assign31990_e46966_d_n10, assign31990_e46966_d_n11, assign31990_e46966_d_n12, assign31990_e46966_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign31990_e46949: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign31990_e46952: f64 = (locals.var_eg * locals.var_beta);
        let assign31990_e46953: f64 = (assign31990_e46949 - assign31990_e46952);
        let assign31990_e46957: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign31990_e46958: f64 = (assign31990_e46957).ln();
        let assign31990_e46959: f64 = (p.p175 * assign31990_e46958);
        let assign31990_e46960: f64 = (assign31990_e46953 + assign31990_e46959);
        let assign31990_e46962: f64 = (assign31990_e46960 / p.p174);
        let assign31990_e46963: f64 = (assign31990_e46962).exp();
        let assign31990_e46964: f64 = (p.p173 * assign31990_e46963);
        (assign31990_e46964, (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31990_e46957))) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17,)
    }
};
        locals.var_js = assign31990_e46966;
        locals.var_js_dn0 = assign31990_e46966_d_n0;
        locals.var_js_dn2 = assign31990_e46966_d_n2;
        locals.var_js_dn6 = assign31990_e46966_d_n6;
        locals.var_js_dn7 = assign31990_e46966_d_n7;
        locals.var_js_dn10 = assign31990_e46966_d_n10;
        locals.var_js_dn11 = assign31990_e46966_d_n11;
        locals.var_js_dn12 = assign31990_e46966_d_n12;
        locals.var_js_dn17 = assign31990_e46966_d_n17;
        locals.var_js_rv = 0.0;

        let (assign32000_e46988, assign32000_e46988_d_n0, assign32000_e46988_d_n2, assign32000_e46988_d_n6, assign32000_e46988_d_n7, assign32000_e46988_d_n10, assign32000_e46988_d_n11, assign32000_e46988_d_n12, assign32000_e46988_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32000_e46971: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign32000_e46974: f64 = (locals.var_eg * locals.var_beta);
        let assign32000_e46975: f64 = (assign32000_e46971 - assign32000_e46974);
        let assign32000_e46979: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign32000_e46980: f64 = (assign32000_e46979).ln();
        let assign32000_e46981: f64 = (p.p176 * assign32000_e46980);
        let assign32000_e46982: f64 = (assign32000_e46975 + assign32000_e46981);
        let assign32000_e46984: f64 = (assign32000_e46982 / p.p174);
        let assign32000_e46985: f64 = (assign32000_e46984).exp();
        let assign32000_e46986: f64 = (p.p173 * assign32000_e46985);
        (assign32000_e46986, (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign32000_e46979))) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17,)
    }
};
        locals.var_js2 = assign32000_e46988;
        locals.var_js2_dn0 = assign32000_e46988_d_n0;
        locals.var_js2_dn2 = assign32000_e46988_d_n2;
        locals.var_js2_dn6 = assign32000_e46988_d_n6;
        locals.var_js2_dn7 = assign32000_e46988_d_n7;
        locals.var_js2_dn10 = assign32000_e46988_d_n10;
        locals.var_js2_dn11 = assign32000_e46988_d_n11;
        locals.var_js2_dn12 = assign32000_e46988_d_n12;
        locals.var_js2_dn17 = assign32000_e46988_d_n17;
        locals.var_js2_rv = 0.0;

        let (assign32010_e46996, assign32010_e46996_d_n0, assign32010_e46996_d_n2, assign32010_e46996_d_n6, assign32010_e46996_d_n7, assign32010_e46996_d_n10, assign32010_e46996_d_n11, assign32010_e46996_d_n12, assign32010_e46996_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32010_e46992: f64 = (locals.var_w_diod * p.p237);
        let assign32010_e46994: f64 = (assign32010_e46992 * locals.var_js);
        (assign32010_e46994, (assign32010_e46992 * locals.var_js_dn0), (assign32010_e46992 * locals.var_js_dn2), (assign32010_e46992 * locals.var_js_dn6), (assign32010_e46992 * locals.var_js_dn7), (assign32010_e46992 * locals.var_js_dn10), (assign32010_e46992 * locals.var_js_dn11), (assign32010_e46992 * locals.var_js_dn12), (assign32010_e46992 * locals.var_js_dn17),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    }
};
        locals.var_isbd = assign32010_e46996;
        locals.var_isbd_dn0 = assign32010_e46996_d_n0;
        locals.var_isbd_dn2 = assign32010_e46996_d_n2;
        locals.var_isbd_dn6 = assign32010_e46996_d_n6;
        locals.var_isbd_dn7 = assign32010_e46996_d_n7;
        locals.var_isbd_dn10 = assign32010_e46996_d_n10;
        locals.var_isbd_dn11 = assign32010_e46996_d_n11;
        locals.var_isbd_dn12 = assign32010_e46996_d_n12;
        locals.var_isbd_dn17 = assign32010_e46996_d_n17;
        locals.var_isbd_rv = 0.0;

        let (assign32020_e47004, assign32020_e47004_d_n0, assign32020_e47004_d_n2, assign32020_e47004_d_n6, assign32020_e47004_d_n7, assign32020_e47004_d_n10, assign32020_e47004_d_n11, assign32020_e47004_d_n12, assign32020_e47004_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32020_e47000: f64 = (locals.var_w_diod * p.p237);
        let assign32020_e47002: f64 = (assign32020_e47000 * locals.var_js2);
        (assign32020_e47002, (assign32020_e47000 * locals.var_js2_dn0), (assign32020_e47000 * locals.var_js2_dn2), (assign32020_e47000 * locals.var_js2_dn6), (assign32020_e47000 * locals.var_js2_dn7), (assign32020_e47000 * locals.var_js2_dn10), (assign32020_e47000 * locals.var_js2_dn11), (assign32020_e47000 * locals.var_js2_dn12), (assign32020_e47000 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17,)
    }
};
        locals.var_isbd2 = assign32020_e47004;
        locals.var_isbd2_dn0 = assign32020_e47004_d_n0;
        locals.var_isbd2_dn2 = assign32020_e47004_d_n2;
        locals.var_isbd2_dn6 = assign32020_e47004_d_n6;
        locals.var_isbd2_dn7 = assign32020_e47004_d_n7;
        locals.var_isbd2_dn10 = assign32020_e47004_d_n10;
        locals.var_isbd2_dn11 = assign32020_e47004_d_n11;
        locals.var_isbd2_dn12 = assign32020_e47004_d_n12;
        locals.var_isbd2_dn17 = assign32020_e47004_d_n17;
        locals.var_isbd2_rv = 0.0;

        let (assign32030_e47012, assign32030_e47012_d_n0, assign32030_e47012_d_n2, assign32030_e47012_d_n6, assign32030_e47012_d_n7, assign32030_e47012_d_n10, assign32030_e47012_d_n11, assign32030_e47012_d_n12, assign32030_e47012_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32030_e47008: f64 = (locals.var_w_dios * p.p237);
        let assign32030_e47010: f64 = (assign32030_e47008 * locals.var_js);
        (assign32030_e47010, (assign32030_e47008 * locals.var_js_dn0), (assign32030_e47008 * locals.var_js_dn2), (assign32030_e47008 * locals.var_js_dn6), (assign32030_e47008 * locals.var_js_dn7), (assign32030_e47008 * locals.var_js_dn10), (assign32030_e47008 * locals.var_js_dn11), (assign32030_e47008 * locals.var_js_dn12), (assign32030_e47008 * locals.var_js_dn17),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17,)
    }
};
        locals.var_isbs = assign32030_e47012;
        locals.var_isbs_dn0 = assign32030_e47012_d_n0;
        locals.var_isbs_dn2 = assign32030_e47012_d_n2;
        locals.var_isbs_dn6 = assign32030_e47012_d_n6;
        locals.var_isbs_dn7 = assign32030_e47012_d_n7;
        locals.var_isbs_dn10 = assign32030_e47012_d_n10;
        locals.var_isbs_dn11 = assign32030_e47012_d_n11;
        locals.var_isbs_dn12 = assign32030_e47012_d_n12;
        locals.var_isbs_dn17 = assign32030_e47012_d_n17;
        locals.var_isbs_rv = 0.0;

        let (assign32040_e47020, assign32040_e47020_d_n0, assign32040_e47020_d_n2, assign32040_e47020_d_n6, assign32040_e47020_d_n7, assign32040_e47020_d_n10, assign32040_e47020_d_n11, assign32040_e47020_d_n12, assign32040_e47020_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32040_e47016: f64 = (locals.var_w_dios * p.p237);
        let assign32040_e47018: f64 = (assign32040_e47016 * locals.var_js2);
        (assign32040_e47018, (assign32040_e47016 * locals.var_js2_dn0), (assign32040_e47016 * locals.var_js2_dn2), (assign32040_e47016 * locals.var_js2_dn6), (assign32040_e47016 * locals.var_js2_dn7), (assign32040_e47016 * locals.var_js2_dn10), (assign32040_e47016 * locals.var_js2_dn11), (assign32040_e47016 * locals.var_js2_dn12), (assign32040_e47016 * locals.var_js2_dn17),)
    } else {
        (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17,)
    }
};
        locals.var_isbs2 = assign32040_e47020;
        locals.var_isbs2_dn0 = assign32040_e47020_d_n0;
        locals.var_isbs2_dn2 = assign32040_e47020_d_n2;
        locals.var_isbs2_dn6 = assign32040_e47020_d_n6;
        locals.var_isbs2_dn7 = assign32040_e47020_d_n7;
        locals.var_isbs2_dn10 = assign32040_e47020_d_n10;
        locals.var_isbs2_dn11 = assign32040_e47020_d_n11;
        locals.var_isbs2_dn12 = assign32040_e47020_d_n12;
        locals.var_isbs2_dn17 = assign32040_e47020_d_n17;
        locals.var_isbs2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32050_e47026, assign32050_e47026_d_n6, assign32050_e47026_d_n7, assign32050_e47026_d_n10, assign32050_e47026_d_n12,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32050_e47024: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign32050_e47024, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0,)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32050_e47026;
        locals.var_t1__blk1034_dn6 = assign32050_e47026_d_n6;
        locals.var_t1__blk1034_dn7 = assign32050_e47026_d_n7;
        locals.var_t1__blk1034_dn10 = assign32050_e47026_d_n10;
        locals.var_t1__blk1034_dn12 = assign32050_e47026_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign32070_e47038, assign32070_e47038_d_n0, assign32070_e47038_d_n2, assign32070_e47038_d_n6, assign32070_e47038_d_n7, assign32070_e47038_d_n10, assign32070_e47038_d_n11, assign32070_e47038_d_n12, assign32070_e47038_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32070_e47036: f64 = (locals.var_isbd + 1e-50);
        (assign32070_e47036, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17,)
    } else {
        (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17,)
    }
};
        locals.var_t2__blk1035 = assign32070_e47038;
        locals.var_t2__blk1035_dn0 = assign32070_e47038_d_n0;
        locals.var_t2__blk1035_dn2 = assign32070_e47038_d_n2;
        locals.var_t2__blk1035_dn6 = assign32070_e47038_d_n6;
        locals.var_t2__blk1035_dn7 = assign32070_e47038_d_n7;
        locals.var_t2__blk1035_dn10 = assign32070_e47038_d_n10;
        locals.var_t2__blk1035_dn11 = assign32070_e47038_d_n11;
        locals.var_t2__blk1035_dn12 = assign32070_e47038_d_n12;
        locals.var_t2__blk1035_dn17 = assign32070_e47038_d_n17;
        locals.var_t2__blk1035_rv = 0.0;

        let (assign32090_e47052, assign32090_e47052_d_n10,) = {
    if (locals.var_guard1032 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn10,)
    }
};
        locals.var_vbdt = assign32090_e47052;
        locals.var_vbdt_dn10 = assign32090_e47052_d_n10;
        locals.var_vbdt_rv = 0.0;

        let (assign32100_e47060, assign32100_e47060_d_n10,) = {
    if (locals.var_guard1032 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vbst, locals.var_vbst_dn10,)
    }
};
        locals.var_vbst = assign32100_e47060;
        locals.var_vbst_dn10 = assign32100_e47060_d_n10;
        locals.var_vbst_rv = 0.0;

        let (assign32110_e47066, assign32110_e47066_d_n10,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32110_e47064: f64 = (p.p174 * locals.var_beta_inv);
        (assign32110_e47064, (p.p174 * locals.var_beta_inv_dn10),)
    } else {
        (locals.var_nvtm, locals.var_nvtm_dn10,)
    }
};
        locals.var_nvtm = assign32110_e47066;
        locals.var_nvtm_dn10 = assign32110_e47066_d_n10;
        locals.var_nvtm_rv = 0.0;

        let assign32120_e47069: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32120_e47069;
        locals.var_guard1061_rv = 0.0;

        let (assign32130_e47078, assign32130_e47078_d_n6, assign32130_e47078_d_n7, assign32130_e47078_d_n10, assign32130_e47078_d_n12,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 != 0.0)) {
        let assign32130_e47075: f64 = (locals.var_vbdj / locals.var_nvtm);
        let assign32130_e47076: f64 = (assign32130_e47075).exp();
        (assign32130_e47076, (assign32130_e47076 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32130_e47076 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32130_e47076 * (locals.var_vbdj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32130_e47078;
        locals.var_t1__blk1034_dn6 = assign32130_e47078_d_n6;
        locals.var_t1__blk1034_dn7 = assign32130_e47078_d_n7;
        locals.var_t1__blk1034_dn10 = assign32130_e47078_d_n10;
        locals.var_t1__blk1034_dn12 = assign32130_e47078_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign32140_e47088, assign32140_e47088_d_n0, assign32140_e47088_d_n2, assign32140_e47088_d_n6, assign32140_e47088_d_n7, assign32140_e47088_d_n10, assign32140_e47088_d_n11, assign32140_e47088_d_n12, assign32140_e47088_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 != 0.0)) {
        let assign32140_e47085: f64 = (locals.var_t1__blk1034 - 1.0);
        let assign32140_e47086: f64 = (locals.var_isbd * assign32140_e47085);
        (assign32140_e47086, (locals.var_isbd_dn0 * assign32140_e47085), (locals.var_isbd_dn2 * assign32140_e47085), ((locals.var_isbd_dn6 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn6)), ((locals.var_isbd_dn7 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn7)), ((locals.var_isbd_dn10 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn10)), (locals.var_isbd_dn11 * assign32140_e47085), ((locals.var_isbd_dn12 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn12)), (locals.var_isbd_dn17 * assign32140_e47085),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32140_e47088;
        locals.var_ibd_dn0 = assign32140_e47088_d_n0;
        locals.var_ibd_dn2 = assign32140_e47088_d_n2;
        locals.var_ibd_dn6 = assign32140_e47088_d_n6;
        locals.var_ibd_dn7 = assign32140_e47088_d_n7;
        locals.var_ibd_dn10 = assign32140_e47088_d_n10;
        locals.var_ibd_dn11 = assign32140_e47088_d_n11;
        locals.var_ibd_dn12 = assign32140_e47088_d_n12;
        locals.var_ibd_dn17 = assign32140_e47088_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign32150_e47098, assign32150_e47098_d_n6, assign32150_e47098_d_n7, assign32150_e47098_d_n10, assign32150_e47098_d_n12,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 == 0.0)) {
        let assign32150_e47095: f64 = (locals.var_vbdt / locals.var_nvtm);
        let assign32150_e47096: f64 = (assign32150_e47095).exp();
        (assign32150_e47096, 0.0, 0.0, (assign32150_e47096 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32150_e47098;
        locals.var_t1__blk1034_dn6 = assign32150_e47098_d_n6;
        locals.var_t1__blk1034_dn7 = assign32150_e47098_d_n7;
        locals.var_t1__blk1034_dn10 = assign32150_e47098_d_n10;
        locals.var_t1__blk1034_dn12 = assign32150_e47098_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign32160_e47119, assign32160_e47119_d_n0, assign32160_e47119_d_n2, assign32160_e47119_d_n6, assign32160_e47119_d_n7, assign32160_e47119_d_n10, assign32160_e47119_d_n11, assign32160_e47119_d_n12, assign32160_e47119_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 == 0.0)) {
        let assign32160_e47106: f64 = (locals.var_t1__blk1034 - 1.0);
        let assign32160_e47107: f64 = (locals.var_isbd * assign32160_e47106);
        let assign32160_e47110: f64 = (locals.var_isbd / locals.var_nvtm);
        let assign32160_e47112: f64 = (assign32160_e47110 * locals.var_t1__blk1034);
        let assign32160_e47115: f64 = (locals.var_vbdj - locals.var_vbdt);
        let assign32160_e47116: f64 = (assign32160_e47112 * assign32160_e47115);
        let assign32160_e47117: f64 = (assign32160_e47107 + assign32160_e47116);
        (assign32160_e47117, ((locals.var_isbd_dn0 * assign32160_e47106) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)), ((locals.var_isbd_dn2 * assign32160_e47106) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)), (((locals.var_isbd_dn6 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn6)) * assign32160_e47115) + (assign32160_e47112 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn7)) * assign32160_e47115)), (((locals.var_isbd_dn10 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn10)) * assign32160_e47115) + (assign32160_e47112 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32160_e47106) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)), (((locals.var_isbd_dn12 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn12)) * assign32160_e47115) + (assign32160_e47112 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32160_e47106) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32160_e47119;
        locals.var_ibd_dn0 = assign32160_e47119_d_n0;
        locals.var_ibd_dn2 = assign32160_e47119_d_n2;
        locals.var_ibd_dn6 = assign32160_e47119_d_n6;
        locals.var_ibd_dn7 = assign32160_e47119_d_n7;
        locals.var_ibd_dn10 = assign32160_e47119_d_n10;
        locals.var_ibd_dn11 = assign32160_e47119_d_n11;
        locals.var_ibd_dn12 = assign32160_e47119_d_n12;
        locals.var_ibd_dn17 = assign32160_e47119_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign32170_e47129, assign32170_e47129_d_n0, assign32170_e47129_d_n2, assign32170_e47129_d_n6, assign32170_e47129_d_n7, assign32170_e47129_d_n10, assign32170_e47129_d_n11, assign32170_e47129_d_n12, assign32170_e47129_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32170_e47124: f64 = (p.p178 * locals.var_vbdj);
        let assign32170_e47126: f64 = (assign32170_e47124 * locals.var_isbd2);
        let assign32170_e47127: f64 = (locals.var_ibd + assign32170_e47126);
        (assign32170_e47127, (locals.var_ibd_dn0 + (assign32170_e47124 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32170_e47124 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32170_e47124 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32170_e47124 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32170_e47124 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32170_e47124 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32170_e47124 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32170_e47124 * locals.var_isbd2_dn17)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32170_e47129;
        locals.var_ibd_dn0 = assign32170_e47129_d_n0;
        locals.var_ibd_dn2 = assign32170_e47129_d_n2;
        locals.var_ibd_dn6 = assign32170_e47129_d_n6;
        locals.var_ibd_dn7 = assign32170_e47129_d_n7;
        locals.var_ibd_dn10 = assign32170_e47129_d_n10;
        locals.var_ibd_dn11 = assign32170_e47129_d_n11;
        locals.var_ibd_dn12 = assign32170_e47129_d_n12;
        locals.var_ibd_dn17 = assign32170_e47129_d_n17;
        locals.var_ibd_rv = 0.0;

        let assign32180_e47132: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32180_e47132;
        locals.var_guard1062_rv = 0.0;

        let (assign32190_e47141, assign32190_e47141_d_n6, assign32190_e47141_d_n7, assign32190_e47141_d_n10, assign32190_e47141_d_n12,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 != 0.0)) {
        let assign32190_e47138: f64 = (locals.var_vbsj / locals.var_nvtm);
        let assign32190_e47139: f64 = (assign32190_e47138).exp();
        (assign32190_e47139, 0.0, (assign32190_e47139 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32190_e47139 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32190_e47139 * (locals.var_vbsj_dn12 / locals.var_nvtm)),)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32190_e47141;
        locals.var_t1__blk1034_dn6 = assign32190_e47141_d_n6;
        locals.var_t1__blk1034_dn7 = assign32190_e47141_d_n7;
        locals.var_t1__blk1034_dn10 = assign32190_e47141_d_n10;
        locals.var_t1__blk1034_dn12 = assign32190_e47141_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign32200_e47151, assign32200_e47151_d_n0, assign32200_e47151_d_n2, assign32200_e47151_d_n6, assign32200_e47151_d_n7, assign32200_e47151_d_n10, assign32200_e47151_d_n11, assign32200_e47151_d_n12, assign32200_e47151_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 != 0.0)) {
        let assign32200_e47148: f64 = (locals.var_t1__blk1034 - 1.0);
        let assign32200_e47149: f64 = (locals.var_isbs * assign32200_e47148);
        (assign32200_e47149, (locals.var_isbs_dn0 * assign32200_e47148), (locals.var_isbs_dn2 * assign32200_e47148), ((locals.var_isbs_dn6 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn6)), ((locals.var_isbs_dn7 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn7)), ((locals.var_isbs_dn10 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn10)), (locals.var_isbs_dn11 * assign32200_e47148), ((locals.var_isbs_dn12 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn12)), (locals.var_isbs_dn17 * assign32200_e47148),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32200_e47151;
        locals.var_ibs_dn0 = assign32200_e47151_d_n0;
        locals.var_ibs_dn2 = assign32200_e47151_d_n2;
        locals.var_ibs_dn6 = assign32200_e47151_d_n6;
        locals.var_ibs_dn7 = assign32200_e47151_d_n7;
        locals.var_ibs_dn10 = assign32200_e47151_d_n10;
        locals.var_ibs_dn11 = assign32200_e47151_d_n11;
        locals.var_ibs_dn12 = assign32200_e47151_d_n12;
        locals.var_ibs_dn17 = assign32200_e47151_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32210_e47161, assign32210_e47161_d_n6, assign32210_e47161_d_n7, assign32210_e47161_d_n10, assign32210_e47161_d_n12,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 == 0.0)) {
        let assign32210_e47158: f64 = (locals.var_vbst / locals.var_nvtm);
        let assign32210_e47159: f64 = (assign32210_e47158).exp();
        (assign32210_e47159, 0.0, 0.0, (assign32210_e47159 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0,)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32210_e47161;
        locals.var_t1__blk1034_dn6 = assign32210_e47161_d_n6;
        locals.var_t1__blk1034_dn7 = assign32210_e47161_d_n7;
        locals.var_t1__blk1034_dn10 = assign32210_e47161_d_n10;
        locals.var_t1__blk1034_dn12 = assign32210_e47161_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign32220_e47182, assign32220_e47182_d_n0, assign32220_e47182_d_n2, assign32220_e47182_d_n6, assign32220_e47182_d_n7, assign32220_e47182_d_n10, assign32220_e47182_d_n11, assign32220_e47182_d_n12, assign32220_e47182_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 == 0.0)) {
        let assign32220_e47169: f64 = (locals.var_t1__blk1034 - 1.0);
        let assign32220_e47170: f64 = (locals.var_isbs * assign32220_e47169);
        let assign32220_e47173: f64 = (locals.var_isbs / locals.var_nvtm);
        let assign32220_e47175: f64 = (assign32220_e47173 * locals.var_t1__blk1034);
        let assign32220_e47178: f64 = (locals.var_vbsj - locals.var_vbst);
        let assign32220_e47179: f64 = (assign32220_e47175 * assign32220_e47178);
        let assign32220_e47180: f64 = (assign32220_e47170 + assign32220_e47179);
        (assign32220_e47180, ((locals.var_isbs_dn0 * assign32220_e47169) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)), ((locals.var_isbs_dn2 * assign32220_e47169) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)), (((locals.var_isbs_dn6 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn6)) * assign32220_e47178)), (((locals.var_isbs_dn7 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn7)) * assign32220_e47178) + (assign32220_e47175 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn10)) * assign32220_e47178) + (assign32220_e47175 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32220_e47169) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)), (((locals.var_isbs_dn12 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn12)) * assign32220_e47178) + (assign32220_e47175 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32220_e47169) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32220_e47182;
        locals.var_ibs_dn0 = assign32220_e47182_d_n0;
        locals.var_ibs_dn2 = assign32220_e47182_d_n2;
        locals.var_ibs_dn6 = assign32220_e47182_d_n6;
        locals.var_ibs_dn7 = assign32220_e47182_d_n7;
        locals.var_ibs_dn10 = assign32220_e47182_d_n10;
        locals.var_ibs_dn11 = assign32220_e47182_d_n11;
        locals.var_ibs_dn12 = assign32220_e47182_d_n12;
        locals.var_ibs_dn17 = assign32220_e47182_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32230_e47192, assign32230_e47192_d_n0, assign32230_e47192_d_n2, assign32230_e47192_d_n6, assign32230_e47192_d_n7, assign32230_e47192_d_n10, assign32230_e47192_d_n11, assign32230_e47192_d_n12, assign32230_e47192_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32230_e47187: f64 = (p.p178 * locals.var_vbsj);
        let assign32230_e47189: f64 = (assign32230_e47187 * locals.var_isbs2);
        let assign32230_e47190: f64 = (locals.var_ibs + assign32230_e47189);
        (assign32230_e47190, (locals.var_ibs_dn0 + (assign32230_e47187 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32230_e47187 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32230_e47187 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32230_e47187 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32230_e47187 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32230_e47187 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32230_e47187 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32230_e47187 * locals.var_isbs2_dn17)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32230_e47192;
        locals.var_ibs_dn0 = assign32230_e47192_d_n0;
        locals.var_ibs_dn2 = assign32230_e47192_d_n2;
        locals.var_ibs_dn6 = assign32230_e47192_d_n6;
        locals.var_ibs_dn7 = assign32230_e47192_d_n7;
        locals.var_ibs_dn10 = assign32230_e47192_d_n10;
        locals.var_ibs_dn11 = assign32230_e47192_d_n11;
        locals.var_ibs_dn12 = assign32230_e47192_d_n12;
        locals.var_ibs_dn17 = assign32230_e47192_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32240_e47200, assign32240_e47200_d_n0, assign32240_e47200_d_n2, assign32240_e47200_d_n6, assign32240_e47200_d_n7, assign32240_e47200_d_n10, assign32240_e47200_d_n11, assign32240_e47200_d_n12, assign32240_e47200_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32240_e47197: f64 = (locals.var_gjmin * locals.var_vbdj);
        let assign32240_e47198: f64 = (locals.var_ibd + assign32240_e47197);
        (assign32240_e47198, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign32240_e47200;
        locals.var_ibd_dn0 = assign32240_e47200_d_n0;
        locals.var_ibd_dn2 = assign32240_e47200_d_n2;
        locals.var_ibd_dn6 = assign32240_e47200_d_n6;
        locals.var_ibd_dn7 = assign32240_e47200_d_n7;
        locals.var_ibd_dn10 = assign32240_e47200_d_n10;
        locals.var_ibd_dn11 = assign32240_e47200_d_n11;
        locals.var_ibd_dn12 = assign32240_e47200_d_n12;
        locals.var_ibd_dn17 = assign32240_e47200_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign32250_e47208, assign32250_e47208_d_n0, assign32250_e47208_d_n2, assign32250_e47208_d_n6, assign32250_e47208_d_n7, assign32250_e47208_d_n10, assign32250_e47208_d_n11, assign32250_e47208_d_n12, assign32250_e47208_d_n17,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32250_e47205: f64 = (locals.var_gjmin * locals.var_vbsj);
        let assign32250_e47206: f64 = (locals.var_ibs + assign32250_e47205);
        (assign32250_e47206, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign32250_e47208;
        locals.var_ibs_dn0 = assign32250_e47208_d_n0;
        locals.var_ibs_dn2 = assign32250_e47208_d_n2;
        locals.var_ibs_dn6 = assign32250_e47208_d_n6;
        locals.var_ibs_dn7 = assign32250_e47208_d_n7;
        locals.var_ibs_dn10 = assign32250_e47208_d_n10;
        locals.var_ibs_dn11 = assign32250_e47208_d_n11;
        locals.var_ibs_dn12 = assign32250_e47208_d_n12;
        locals.var_ibs_dn17 = assign32250_e47208_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign32260_e47214,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32260_e47212: f64 = (p.p179 * p.p2);
        (assign32260_e47212,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32260_e47214;
        locals.var_czbd_rv = 0.0;

        let (assign32270_e47220,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32270_e47218: f64 = (p.p179 * p.p3);
        (assign32270_e47218,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32270_e47220;
        locals.var_czbs_rv = 0.0;

        let (assign32280_e47226,) = {
    if (locals.var_guard1032 != 0.0) {
        let assign32280_e47224: f64 = (p.p237 - p.p238);
        (assign32280_e47224,)
    } else {
        (locals.var_xp_max,)
    }
};
        locals.var_xp_max = assign32280_e47226;
        locals.var_xp_max_rv = 0.0;

        let assign32290_e47229: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32290_e47229;
        locals.var_guard1063_rv = 0.0;

        let (assign32300_e47235,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1063 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbd,)
    }
};
        locals.var_czbd = assign32300_e47235;
        locals.var_czbd_rv = 0.0;

        let (assign32310_e47241,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1063 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_czbs,)
    }
};
        locals.var_czbs = assign32310_e47241;
        locals.var_czbs_rv = 0.0;

        let assign32320_e47244: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32320_e47244;
        locals.var_guard1064_rv = 0.0;

        let (assign32330_e47254,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) {
        let assign32330_e47251: f64 = (p.p5 - locals.var_w_dioscv);
        let assign32330_e47252: f64 = (p.p180 * assign32330_e47251);
        (assign32330_e47252,)
    } else {
        (locals.var_czbssw,)
    }
};
        locals.var_czbssw = assign32330_e47254;
        locals.var_czbssw_rv = 0.0;

        let (assign32340_e47262,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) {
        let assign32340_e47260: f64 = (p.p181 * locals.var_w_dioscv);
        (assign32340_e47260,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32340_e47262;
        locals.var_czbsswg_rv = 0.0;

        let assign32350_e47265: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32350_e47265;
        locals.var_guard1065_rv = 0.0;

        let assign32360_e47268: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32360_e47268;
        locals.var_guard1066_rv = 0.0;

        let (assign32370_e47282, assign32370_e47282_d_n6, assign32370_e47282_d_n7, assign32370_e47282_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32370_e47279: f64 = (locals.var_vbsj / p.p185);
        let assign32370_e47280: f64 = (1.0 - assign32370_e47279);
        (assign32370_e47280, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32370_e47282;
        locals.var_arg__blk1059_dn6 = assign32370_e47282_d_n6;
        locals.var_arg__blk1059_dn7 = assign32370_e47282_d_n7;
        locals.var_arg__blk1059_dn12 = assign32370_e47282_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32380_e47285: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32380_e47285;
        locals.var_guard1067_rv = 0.0;

        let (assign32390_e47300, assign32390_e47300_d_n6, assign32390_e47300_d_n7, assign32390_e47300_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
        let assign32390_e47297: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32390_e47298: f64 = (1.0 / assign32390_e47297);
        (assign32390_e47298, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32390_e47300;
        locals.var_sarg_dn6 = assign32390_e47300_d_n6;
        locals.var_sarg_dn7 = assign32390_e47300_d_n7;
        locals.var_sarg_dn12 = assign32390_e47300_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32400_e47316, assign32400_e47316_d_n6, assign32400_e47316_d_n7, assign32400_e47316_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
        let assign32400_e47313: f64 = (-p.p182);
        let assign32400_e47314: f64 = (locals.var_arg__blk1059).powf(assign32400_e47313);
        (assign32400_e47314, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((locals.var_arg__blk1059).powf(assign32400_e47313 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32400_e47314 * (assign32400_e47313 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((locals.var_arg__blk1059).powf(assign32400_e47313 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32400_e47314 * (assign32400_e47313 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((locals.var_arg__blk1059).powf(assign32400_e47313 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32400_e47314 * (assign32400_e47313 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32400_e47316;
        locals.var_sarg_dn6 = assign32400_e47316_d_n6;
        locals.var_sarg_dn7 = assign32400_e47316_d_n7;
        locals.var_sarg_dn12 = assign32400_e47316_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32410_e47338, assign32410_e47338_d_n0, assign32410_e47338_d_n2, assign32410_e47338_d_n6, assign32410_e47338_d_n7, assign32410_e47338_d_n10, assign32410_e47338_d_n11, assign32410_e47338_d_n12, assign32410_e47338_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        let assign32410_e47326: f64 = (p.p185 * locals.var_czbs);
        let assign32410_e47330: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32410_e47331: f64 = (1.0 - assign32410_e47330);
        let assign32410_e47332: f64 = (assign32410_e47326 * assign32410_e47331);
        let assign32410_e47335: f64 = (1.0 - p.p182);
        let assign32410_e47336: f64 = (assign32410_e47332 / assign32410_e47335);
        (assign32410_e47336, 0.0, 0.0, ((assign32410_e47326 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32410_e47335), ((assign32410_e47326 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32410_e47335), 0.0, 0.0, ((assign32410_e47326 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32410_e47335), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32410_e47338;
        locals.var_qbs_dn0 = assign32410_e47338_d_n0;
        locals.var_qbs_dn2 = assign32410_e47338_d_n2;
        locals.var_qbs_dn6 = assign32410_e47338_d_n6;
        locals.var_qbs_dn7 = assign32410_e47338_d_n7;
        locals.var_qbs_dn10 = assign32410_e47338_d_n10;
        locals.var_qbs_dn11 = assign32410_e47338_d_n11;
        locals.var_qbs_dn12 = assign32410_e47338_d_n12;
        locals.var_qbs_dn17 = assign32410_e47338_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32420_e47349, assign32420_e47349_d_n0, assign32420_e47349_d_n2, assign32420_e47349_d_n6, assign32420_e47349_d_n7, assign32420_e47349_d_n10, assign32420_e47349_d_n11, assign32420_e47349_d_n12, assign32420_e47349_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32420_e47349;
        locals.var_qbs_dn0 = assign32420_e47349_d_n0;
        locals.var_qbs_dn2 = assign32420_e47349_d_n2;
        locals.var_qbs_dn6 = assign32420_e47349_d_n6;
        locals.var_qbs_dn7 = assign32420_e47349_d_n7;
        locals.var_qbs_dn10 = assign32420_e47349_d_n10;
        locals.var_qbs_dn11 = assign32420_e47349_d_n11;
        locals.var_qbs_dn12 = assign32420_e47349_d_n12;
        locals.var_qbs_dn17 = assign32420_e47349_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32430_e47352: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32430_e47352;
        locals.var_guard1068_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32440_e47366, assign32440_e47366_d_n6, assign32440_e47366_d_n7, assign32440_e47366_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign32440_e47363: f64 = (locals.var_vbsj / p.p186);
        let assign32440_e47364: f64 = (1.0 - assign32440_e47363);
        (assign32440_e47364, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32440_e47366;
        locals.var_arg__blk1059_dn6 = assign32440_e47366_d_n6;
        locals.var_arg__blk1059_dn7 = assign32440_e47366_d_n7;
        locals.var_arg__blk1059_dn12 = assign32440_e47366_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32450_e47369: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32450_e47369;
        locals.var_guard1069_rv = 0.0;

        let (assign32460_e47384, assign32460_e47384_d_n6, assign32460_e47384_d_n7, assign32460_e47384_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign32460_e47381: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32460_e47382: f64 = (1.0 / assign32460_e47381);
        (assign32460_e47382, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32460_e47384;
        locals.var_sarg_dn6 = assign32460_e47384_d_n6;
        locals.var_sarg_dn7 = assign32460_e47384_d_n7;
        locals.var_sarg_dn12 = assign32460_e47384_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32470_e47400, assign32470_e47400_d_n6, assign32470_e47400_d_n7, assign32470_e47400_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
        let assign32470_e47397: f64 = (-p.p183);
        let assign32470_e47398: f64 = (locals.var_arg__blk1059).powf(assign32470_e47397);
        (assign32470_e47398, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((locals.var_arg__blk1059).powf(assign32470_e47397 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32470_e47398 * (assign32470_e47397 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((locals.var_arg__blk1059).powf(assign32470_e47397 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32470_e47398 * (assign32470_e47397 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((locals.var_arg__blk1059).powf(assign32470_e47397 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32470_e47398 * (assign32470_e47397 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32470_e47400;
        locals.var_sarg_dn6 = assign32470_e47400_d_n6;
        locals.var_sarg_dn7 = assign32470_e47400_d_n7;
        locals.var_sarg_dn12 = assign32470_e47400_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32480_e47424, assign32480_e47424_d_n0, assign32480_e47424_d_n2, assign32480_e47424_d_n6, assign32480_e47424_d_n7, assign32480_e47424_d_n10, assign32480_e47424_d_n11, assign32480_e47424_d_n12, assign32480_e47424_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign32480_e47411: f64 = (p.p186 * locals.var_czbssw);
        let assign32480_e47415: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32480_e47416: f64 = (1.0 - assign32480_e47415);
        let assign32480_e47417: f64 = (assign32480_e47411 * assign32480_e47416);
        let assign32480_e47420: f64 = (1.0 - p.p183);
        let assign32480_e47421: f64 = (assign32480_e47417 / assign32480_e47420);
        let assign32480_e47422: f64 = (locals.var_qbs + assign32480_e47421);
        (assign32480_e47422, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32480_e47411 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32480_e47420)), (locals.var_qbs_dn7 + ((assign32480_e47411 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32480_e47420)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32480_e47411 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32480_e47420)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32480_e47424;
        locals.var_qbs_dn0 = assign32480_e47424_d_n0;
        locals.var_qbs_dn2 = assign32480_e47424_d_n2;
        locals.var_qbs_dn6 = assign32480_e47424_d_n6;
        locals.var_qbs_dn7 = assign32480_e47424_d_n7;
        locals.var_qbs_dn10 = assign32480_e47424_d_n10;
        locals.var_qbs_dn11 = assign32480_e47424_d_n11;
        locals.var_qbs_dn12 = assign32480_e47424_d_n12;
        locals.var_qbs_dn17 = assign32480_e47424_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32490_e47427: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32490_e47427;
        locals.var_guard1070_rv = 0.0;

        let (assign32500_e47441, assign32500_e47441_d_n6, assign32500_e47441_d_n7, assign32500_e47441_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) {
        let assign32500_e47438: f64 = (locals.var_vbsj / p.p187);
        let assign32500_e47439: f64 = (1.0 - assign32500_e47438);
        (assign32500_e47439, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32500_e47441;
        locals.var_arg__blk1059_dn6 = assign32500_e47441_d_n6;
        locals.var_arg__blk1059_dn7 = assign32500_e47441_d_n7;
        locals.var_arg__blk1059_dn12 = assign32500_e47441_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32510_e47444: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32510_e47444;
        locals.var_guard1071_rv = 0.0;

        let (assign32520_e47459, assign32520_e47459_d_n6, assign32520_e47459_d_n7, assign32520_e47459_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign32520_e47456: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32520_e47457: f64 = (1.0 / assign32520_e47456);
        (assign32520_e47457, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32520_e47459;
        locals.var_sarg_dn6 = assign32520_e47459_d_n6;
        locals.var_sarg_dn7 = assign32520_e47459_d_n7;
        locals.var_sarg_dn12 = assign32520_e47459_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32530_e47475, assign32530_e47475_d_n6, assign32530_e47475_d_n7, assign32530_e47475_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 == 0.0)) {
        let assign32530_e47472: f64 = (-p.p184);
        let assign32530_e47473: f64 = (locals.var_arg__blk1059).powf(assign32530_e47472);
        (assign32530_e47473, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((locals.var_arg__blk1059).powf(assign32530_e47472 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32530_e47473 * (assign32530_e47472 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((locals.var_arg__blk1059).powf(assign32530_e47472 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32530_e47473 * (assign32530_e47472 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((locals.var_arg__blk1059).powf(assign32530_e47472 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32530_e47473 * (assign32530_e47472 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32530_e47475;
        locals.var_sarg_dn6 = assign32530_e47475_d_n6;
        locals.var_sarg_dn7 = assign32530_e47475_d_n7;
        locals.var_sarg_dn12 = assign32530_e47475_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32540_e47499, assign32540_e47499_d_n0, assign32540_e47499_d_n2, assign32540_e47499_d_n6, assign32540_e47499_d_n7, assign32540_e47499_d_n10, assign32540_e47499_d_n11, assign32540_e47499_d_n12, assign32540_e47499_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) {
        let assign32540_e47486: f64 = (p.p187 * locals.var_czbsswg);
        let assign32540_e47490: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32540_e47491: f64 = (1.0 - assign32540_e47490);
        let assign32540_e47492: f64 = (assign32540_e47486 * assign32540_e47491);
        let assign32540_e47495: f64 = (1.0 - p.p184);
        let assign32540_e47496: f64 = (assign32540_e47492 / assign32540_e47495);
        let assign32540_e47497: f64 = (locals.var_qbs + assign32540_e47496);
        (assign32540_e47497, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32540_e47486 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32540_e47495)), (locals.var_qbs_dn7 + ((assign32540_e47486 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32540_e47495)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32540_e47486 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32540_e47495)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32540_e47499;
        locals.var_qbs_dn0 = assign32540_e47499_d_n0;
        locals.var_qbs_dn2 = assign32540_e47499_d_n2;
        locals.var_qbs_dn6 = assign32540_e47499_d_n6;
        locals.var_qbs_dn7 = assign32540_e47499_d_n7;
        locals.var_qbs_dn10 = assign32540_e47499_d_n10;
        locals.var_qbs_dn11 = assign32540_e47499_d_n11;
        locals.var_qbs_dn12 = assign32540_e47499_d_n12;
        locals.var_qbs_dn17 = assign32540_e47499_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32550_e47512, assign32550_e47512_d_n6, assign32550_e47512_d_n7, assign32550_e47512_d_n10, assign32550_e47512_d_n12,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
        let assign32550_e47508: f64 = (locals.var_czbs + locals.var_czbssw);
        let assign32550_e47510: f64 = (assign32550_e47508 + locals.var_czbsswg);
        (assign32550_e47510, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32550_e47512;
        locals.var_t1__blk1034_dn6 = assign32550_e47512_d_n6;
        locals.var_t1__blk1034_dn7 = assign32550_e47512_d_n7;
        locals.var_t1__blk1034_dn10 = assign32550_e47512_d_n10;
        locals.var_t1__blk1034_dn12 = assign32550_e47512_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign32560_e47537, assign32560_e47537_d_n0, assign32560_e47537_d_n2, assign32560_e47537_d_n6, assign32560_e47537_d_n7, assign32560_e47537_d_n10, assign32560_e47537_d_n11, assign32560_e47537_d_n12, assign32560_e47537_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
        let assign32560_e47521: f64 = (locals.var_czbs * p.p182);
        let assign32560_e47523: f64 = (assign32560_e47521 / p.p185);
        let assign32560_e47526: f64 = (locals.var_czbssw * p.p183);
        let assign32560_e47528: f64 = (assign32560_e47526 / p.p186);
        let assign32560_e47529: f64 = (assign32560_e47523 + assign32560_e47528);
        let assign32560_e47532: f64 = (locals.var_czbsswg * p.p184);
        let assign32560_e47534: f64 = (assign32560_e47532 / p.p187);
        let assign32560_e47535: f64 = (assign32560_e47529 + assign32560_e47534);
        (assign32560_e47535, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17,)
    }
};
        locals.var_t2__blk1035 = assign32560_e47537;
        locals.var_t2__blk1035_dn0 = assign32560_e47537_d_n0;
        locals.var_t2__blk1035_dn2 = assign32560_e47537_d_n2;
        locals.var_t2__blk1035_dn6 = assign32560_e47537_d_n6;
        locals.var_t2__blk1035_dn7 = assign32560_e47537_d_n7;
        locals.var_t2__blk1035_dn10 = assign32560_e47537_d_n10;
        locals.var_t2__blk1035_dn11 = assign32560_e47537_d_n11;
        locals.var_t2__blk1035_dn12 = assign32560_e47537_d_n12;
        locals.var_t2__blk1035_dn17 = assign32560_e47537_d_n17;
        locals.var_t2__blk1035_rv = 0.0;

        let (assign32570_e47554, assign32570_e47554_d_n0, assign32570_e47554_d_n2, assign32570_e47554_d_n6, assign32570_e47554_d_n7, assign32570_e47554_d_n10, assign32570_e47554_d_n11, assign32570_e47554_d_n12, assign32570_e47554_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
        let assign32570_e47548: f64 = (locals.var_vbsj * 0.5);
        let assign32570_e47550: f64 = (assign32570_e47548 * locals.var_t2__blk1035);
        let assign32570_e47551: f64 = (locals.var_t1__blk1034 + assign32570_e47550);
        let assign32570_e47552: f64 = (locals.var_vbsj * assign32570_e47551);
        (assign32570_e47552, (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn0)), (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn2)), (locals.var_vbsj * (locals.var_t1__blk1034_dn6 + (assign32570_e47548 * locals.var_t2__blk1035_dn6))), ((locals.var_vbsj_dn7 * assign32570_e47551) + (locals.var_vbsj * (locals.var_t1__blk1034_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1035) + (assign32570_e47548 * locals.var_t2__blk1035_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1034_dn10 + (assign32570_e47548 * locals.var_t2__blk1035_dn10))), (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn11)), ((locals.var_vbsj_dn12 * assign32570_e47551) + (locals.var_vbsj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign32570_e47548 * locals.var_t2__blk1035_dn12))))), (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32570_e47554;
        locals.var_qbs_dn0 = assign32570_e47554_d_n0;
        locals.var_qbs_dn2 = assign32570_e47554_d_n2;
        locals.var_qbs_dn6 = assign32570_e47554_d_n6;
        locals.var_qbs_dn7 = assign32570_e47554_d_n7;
        locals.var_qbs_dn10 = assign32570_e47554_d_n10;
        locals.var_qbs_dn11 = assign32570_e47554_d_n11;
        locals.var_qbs_dn12 = assign32570_e47554_d_n12;
        locals.var_qbs_dn17 = assign32570_e47554_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32580_e47563,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) {
        let assign32580_e47561: f64 = (p.p181 * p.p5);
        (assign32580_e47561,)
    } else {
        (locals.var_czbsswg,)
    }
};
        locals.var_czbsswg = assign32580_e47563;
        locals.var_czbsswg_rv = 0.0;

        let assign32590_e47566: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32590_e47566;
        locals.var_guard1072_rv = 0.0;

        let assign32600_e47569: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32600_e47569;
        locals.var_guard1073_rv = 0.0;

        let (assign32610_e47584, assign32610_e47584_d_n6, assign32610_e47584_d_n7, assign32610_e47584_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) {
        let assign32610_e47581: f64 = (locals.var_vbsj / p.p185);
        let assign32610_e47582: f64 = (1.0 - assign32610_e47581);
        (assign32610_e47582, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32610_e47584;
        locals.var_arg__blk1059_dn6 = assign32610_e47584_d_n6;
        locals.var_arg__blk1059_dn7 = assign32610_e47584_d_n7;
        locals.var_arg__blk1059_dn12 = assign32610_e47584_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32620_e47587: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32620_e47587;
        locals.var_guard1074_rv = 0.0;

        let (assign32630_e47603, assign32630_e47603_d_n6, assign32630_e47603_d_n7, assign32630_e47603_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) {
        let assign32630_e47600: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32630_e47601: f64 = (1.0 / assign32630_e47600);
        (assign32630_e47601, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32630_e47603;
        locals.var_sarg_dn6 = assign32630_e47603_d_n6;
        locals.var_sarg_dn7 = assign32630_e47603_d_n7;
        locals.var_sarg_dn12 = assign32630_e47603_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32640_e47620, assign32640_e47620_d_n6, assign32640_e47620_d_n7, assign32640_e47620_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
        let assign32640_e47617: f64 = (-p.p182);
        let assign32640_e47618: f64 = (locals.var_arg__blk1059).powf(assign32640_e47617);
        (assign32640_e47618, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((locals.var_arg__blk1059).powf(assign32640_e47617 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32640_e47618 * (assign32640_e47617 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((locals.var_arg__blk1059).powf(assign32640_e47617 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32640_e47618 * (assign32640_e47617 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((locals.var_arg__blk1059).powf(assign32640_e47617 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32640_e47618 * (assign32640_e47617 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32640_e47620;
        locals.var_sarg_dn6 = assign32640_e47620_d_n6;
        locals.var_sarg_dn7 = assign32640_e47620_d_n7;
        locals.var_sarg_dn12 = assign32640_e47620_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32650_e47643, assign32650_e47643_d_n0, assign32650_e47643_d_n2, assign32650_e47643_d_n6, assign32650_e47643_d_n7, assign32650_e47643_d_n10, assign32650_e47643_d_n11, assign32650_e47643_d_n12, assign32650_e47643_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) {
        let assign32650_e47631: f64 = (p.p185 * locals.var_czbs);
        let assign32650_e47635: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32650_e47636: f64 = (1.0 - assign32650_e47635);
        let assign32650_e47637: f64 = (assign32650_e47631 * assign32650_e47636);
        let assign32650_e47640: f64 = (1.0 - p.p182);
        let assign32650_e47641: f64 = (assign32650_e47637 / assign32650_e47640);
        (assign32650_e47641, 0.0, 0.0, ((assign32650_e47631 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32650_e47640), ((assign32650_e47631 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32650_e47640), 0.0, 0.0, ((assign32650_e47631 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32650_e47640), 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32650_e47643;
        locals.var_qbs_dn0 = assign32650_e47643_d_n0;
        locals.var_qbs_dn2 = assign32650_e47643_d_n2;
        locals.var_qbs_dn6 = assign32650_e47643_d_n6;
        locals.var_qbs_dn7 = assign32650_e47643_d_n7;
        locals.var_qbs_dn10 = assign32650_e47643_d_n10;
        locals.var_qbs_dn11 = assign32650_e47643_d_n11;
        locals.var_qbs_dn12 = assign32650_e47643_d_n12;
        locals.var_qbs_dn17 = assign32650_e47643_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32660_e47655, assign32660_e47655_d_n0, assign32660_e47655_d_n2, assign32660_e47655_d_n6, assign32660_e47655_d_n7, assign32660_e47655_d_n10, assign32660_e47655_d_n11, assign32660_e47655_d_n12, assign32660_e47655_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32660_e47655;
        locals.var_qbs_dn0 = assign32660_e47655_d_n0;
        locals.var_qbs_dn2 = assign32660_e47655_d_n2;
        locals.var_qbs_dn6 = assign32660_e47655_d_n6;
        locals.var_qbs_dn7 = assign32660_e47655_d_n7;
        locals.var_qbs_dn10 = assign32660_e47655_d_n10;
        locals.var_qbs_dn11 = assign32660_e47655_d_n11;
        locals.var_qbs_dn12 = assign32660_e47655_d_n12;
        locals.var_qbs_dn17 = assign32660_e47655_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32670_e47658: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32670_e47658;
        locals.var_guard1075_rv = 0.0;

        let (assign32680_e47673, assign32680_e47673_d_n6, assign32680_e47673_d_n7, assign32680_e47673_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign32680_e47670: f64 = (locals.var_vbsj / p.p187);
        let assign32680_e47671: f64 = (1.0 - assign32680_e47670);
        (assign32680_e47671, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32680_e47673;
        locals.var_arg__blk1059_dn6 = assign32680_e47673_d_n6;
        locals.var_arg__blk1059_dn7 = assign32680_e47673_d_n7;
        locals.var_arg__blk1059_dn12 = assign32680_e47673_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32690_e47676: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32690_e47676;
        locals.var_guard1076_rv = 0.0;

        let (assign32700_e47692, assign32700_e47692_d_n6, assign32700_e47692_d_n7, assign32700_e47692_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) {
        let assign32700_e47689: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32700_e47690: f64 = (1.0 / assign32700_e47689);
        (assign32700_e47690, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32700_e47692;
        locals.var_sarg_dn6 = assign32700_e47692_d_n6;
        locals.var_sarg_dn7 = assign32700_e47692_d_n7;
        locals.var_sarg_dn12 = assign32700_e47692_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32710_e47709, assign32710_e47709_d_n6, assign32710_e47709_d_n7, assign32710_e47709_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
        let assign32710_e47706: f64 = (-p.p184);
        let assign32710_e47707: f64 = (locals.var_arg__blk1059).powf(assign32710_e47706);
        (assign32710_e47707, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((locals.var_arg__blk1059).powf(assign32710_e47706 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32710_e47707 * (assign32710_e47706 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((locals.var_arg__blk1059).powf(assign32710_e47706 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32710_e47707 * (assign32710_e47706 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((locals.var_arg__blk1059).powf(assign32710_e47706 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32710_e47707 * (assign32710_e47706 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32710_e47709;
        locals.var_sarg_dn6 = assign32710_e47709_d_n6;
        locals.var_sarg_dn7 = assign32710_e47709_d_n7;
        locals.var_sarg_dn12 = assign32710_e47709_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32720_e47734, assign32720_e47734_d_n0, assign32720_e47734_d_n2, assign32720_e47734_d_n6, assign32720_e47734_d_n7, assign32720_e47734_d_n10, assign32720_e47734_d_n11, assign32720_e47734_d_n12, assign32720_e47734_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign32720_e47721: f64 = (p.p187 * locals.var_czbsswg);
        let assign32720_e47725: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32720_e47726: f64 = (1.0 - assign32720_e47725);
        let assign32720_e47727: f64 = (assign32720_e47721 * assign32720_e47726);
        let assign32720_e47730: f64 = (1.0 - p.p184);
        let assign32720_e47731: f64 = (assign32720_e47727 / assign32720_e47730);
        let assign32720_e47732: f64 = (locals.var_qbs + assign32720_e47731);
        (assign32720_e47732, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32720_e47721 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32720_e47730)), (locals.var_qbs_dn7 + ((assign32720_e47721 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32720_e47730)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32720_e47721 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32720_e47730)), locals.var_qbs_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32720_e47734;
        locals.var_qbs_dn0 = assign32720_e47734_d_n0;
        locals.var_qbs_dn2 = assign32720_e47734_d_n2;
        locals.var_qbs_dn6 = assign32720_e47734_d_n6;
        locals.var_qbs_dn7 = assign32720_e47734_d_n7;
        locals.var_qbs_dn10 = assign32720_e47734_d_n10;
        locals.var_qbs_dn11 = assign32720_e47734_d_n11;
        locals.var_qbs_dn12 = assign32720_e47734_d_n12;
        locals.var_qbs_dn17 = assign32720_e47734_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign32730_e47746, assign32730_e47746_d_n6, assign32730_e47746_d_n7, assign32730_e47746_d_n10, assign32730_e47746_d_n12,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 == 0.0)) {
        let assign32730_e47744: f64 = (locals.var_czbs + locals.var_czbsswg);
        (assign32730_e47744, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32730_e47746;
        locals.var_t1__blk1034_dn6 = assign32730_e47746_d_n6;
        locals.var_t1__blk1034_dn7 = assign32730_e47746_d_n7;
        locals.var_t1__blk1034_dn10 = assign32730_e47746_d_n10;
        locals.var_t1__blk1034_dn12 = assign32730_e47746_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign32740_e47766, assign32740_e47766_d_n0, assign32740_e47766_d_n2, assign32740_e47766_d_n6, assign32740_e47766_d_n7, assign32740_e47766_d_n10, assign32740_e47766_d_n11, assign32740_e47766_d_n12, assign32740_e47766_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 == 0.0)) {
        let assign32740_e47756: f64 = (locals.var_czbs * p.p182);
        let assign32740_e47758: f64 = (assign32740_e47756 / p.p185);
        let assign32740_e47761: f64 = (locals.var_czbsswg * p.p184);
        let assign32740_e47763: f64 = (assign32740_e47761 / p.p187);
        let assign32740_e47764: f64 = (assign32740_e47758 + assign32740_e47763);
        (assign32740_e47764, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17,)
    }
};
        locals.var_t2__blk1035 = assign32740_e47766;
        locals.var_t2__blk1035_dn0 = assign32740_e47766_d_n0;
        locals.var_t2__blk1035_dn2 = assign32740_e47766_d_n2;
        locals.var_t2__blk1035_dn6 = assign32740_e47766_d_n6;
        locals.var_t2__blk1035_dn7 = assign32740_e47766_d_n7;
        locals.var_t2__blk1035_dn10 = assign32740_e47766_d_n10;
        locals.var_t2__blk1035_dn11 = assign32740_e47766_d_n11;
        locals.var_t2__blk1035_dn12 = assign32740_e47766_d_n12;
        locals.var_t2__blk1035_dn17 = assign32740_e47766_d_n17;
        locals.var_t2__blk1035_rv = 0.0;

        let (assign32750_e47784, assign32750_e47784_d_n0, assign32750_e47784_d_n2, assign32750_e47784_d_n6, assign32750_e47784_d_n7, assign32750_e47784_d_n10, assign32750_e47784_d_n11, assign32750_e47784_d_n12, assign32750_e47784_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 == 0.0)) {
        let assign32750_e47778: f64 = (locals.var_vbsj * 0.5);
        let assign32750_e47780: f64 = (assign32750_e47778 * locals.var_t2__blk1035);
        let assign32750_e47781: f64 = (locals.var_t1__blk1034 + assign32750_e47780);
        let assign32750_e47782: f64 = (locals.var_vbsj * assign32750_e47781);
        (assign32750_e47782, (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn0)), (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn2)), (locals.var_vbsj * (locals.var_t1__blk1034_dn6 + (assign32750_e47778 * locals.var_t2__blk1035_dn6))), ((locals.var_vbsj_dn7 * assign32750_e47781) + (locals.var_vbsj * (locals.var_t1__blk1034_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1035) + (assign32750_e47778 * locals.var_t2__blk1035_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1034_dn10 + (assign32750_e47778 * locals.var_t2__blk1035_dn10))), (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn11)), ((locals.var_vbsj_dn12 * assign32750_e47781) + (locals.var_vbsj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign32750_e47778 * locals.var_t2__blk1035_dn12))))), (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn17)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign32750_e47784;
        locals.var_qbs_dn0 = assign32750_e47784_d_n0;
        locals.var_qbs_dn2 = assign32750_e47784_d_n2;
        locals.var_qbs_dn6 = assign32750_e47784_d_n6;
        locals.var_qbs_dn7 = assign32750_e47784_d_n7;
        locals.var_qbs_dn10 = assign32750_e47784_d_n10;
        locals.var_qbs_dn11 = assign32750_e47784_d_n11;
        locals.var_qbs_dn12 = assign32750_e47784_d_n12;
        locals.var_qbs_dn17 = assign32750_e47784_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign32760_e47787: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32760_e47787;
        locals.var_guard1077_rv = 0.0;

        let (assign32770_e47797,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) {
        let assign32770_e47794: f64 = (p.p4 - locals.var_w_diodcv);
        let assign32770_e47795: f64 = (p.p180 * assign32770_e47794);
        (assign32770_e47795,)
    } else {
        (locals.var_czbdsw,)
    }
};
        locals.var_czbdsw = assign32770_e47797;
        locals.var_czbdsw_rv = 0.0;

        let (assign32780_e47805,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) {
        let assign32780_e47803: f64 = (p.p181 * locals.var_w_diodcv);
        (assign32780_e47803,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign32780_e47805;
        locals.var_czbdswg_rv = 0.0;

        let assign32790_e47808: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32790_e47808;
        locals.var_guard1078_rv = 0.0;

        let assign32800_e47811: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32800_e47811;
        locals.var_guard1079_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32810_e47825, assign32810_e47825_d_n6, assign32810_e47825_d_n7, assign32810_e47825_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32810_e47822: f64 = (locals.var_vbdj / p.p185);
        let assign32810_e47823: f64 = (1.0 - assign32810_e47822);
        (assign32810_e47823, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32810_e47825;
        locals.var_arg__blk1059_dn6 = assign32810_e47825_d_n6;
        locals.var_arg__blk1059_dn7 = assign32810_e47825_d_n7;
        locals.var_arg__blk1059_dn12 = assign32810_e47825_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32820_e47828: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32820_e47828;
        locals.var_guard1080_rv = 0.0;

        let (assign32830_e47843, assign32830_e47843_d_n6, assign32830_e47843_d_n7, assign32830_e47843_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
        let assign32830_e47840: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32830_e47841: f64 = (1.0 / assign32830_e47840);
        (assign32830_e47841, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32830_e47843;
        locals.var_sarg_dn6 = assign32830_e47843_d_n6;
        locals.var_sarg_dn7 = assign32830_e47843_d_n7;
        locals.var_sarg_dn12 = assign32830_e47843_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32840_e47859, assign32840_e47859_d_n6, assign32840_e47859_d_n7, assign32840_e47859_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
        let assign32840_e47856: f64 = (-p.p182);
        let assign32840_e47857: f64 = (locals.var_arg__blk1059).powf(assign32840_e47856);
        (assign32840_e47857, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((locals.var_arg__blk1059).powf(assign32840_e47856 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32840_e47857 * (assign32840_e47856 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((locals.var_arg__blk1059).powf(assign32840_e47856 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32840_e47857 * (assign32840_e47856 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((locals.var_arg__blk1059).powf(assign32840_e47856 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32840_e47857 * (assign32840_e47856 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32840_e47859;
        locals.var_sarg_dn6 = assign32840_e47859_d_n6;
        locals.var_sarg_dn7 = assign32840_e47859_d_n7;
        locals.var_sarg_dn12 = assign32840_e47859_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32850_e47881, assign32850_e47881_d_n0, assign32850_e47881_d_n2, assign32850_e47881_d_n6, assign32850_e47881_d_n7, assign32850_e47881_d_n10, assign32850_e47881_d_n11, assign32850_e47881_d_n12, assign32850_e47881_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) {
        let assign32850_e47869: f64 = (p.p185 * locals.var_czbd);
        let assign32850_e47873: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32850_e47874: f64 = (1.0 - assign32850_e47873);
        let assign32850_e47875: f64 = (assign32850_e47869 * assign32850_e47874);
        let assign32850_e47878: f64 = (1.0 - p.p182);
        let assign32850_e47879: f64 = (assign32850_e47875 / assign32850_e47878);
        (assign32850_e47879, 0.0, 0.0, ((assign32850_e47869 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32850_e47878), ((assign32850_e47869 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32850_e47878), 0.0, 0.0, ((assign32850_e47869 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32850_e47878), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32850_e47881;
        locals.var_qbd_dn0 = assign32850_e47881_d_n0;
        locals.var_qbd_dn2 = assign32850_e47881_d_n2;
        locals.var_qbd_dn6 = assign32850_e47881_d_n6;
        locals.var_qbd_dn7 = assign32850_e47881_d_n7;
        locals.var_qbd_dn10 = assign32850_e47881_d_n10;
        locals.var_qbd_dn11 = assign32850_e47881_d_n11;
        locals.var_qbd_dn12 = assign32850_e47881_d_n12;
        locals.var_qbd_dn17 = assign32850_e47881_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign32860_e47892, assign32860_e47892_d_n0, assign32860_e47892_d_n2, assign32860_e47892_d_n6, assign32860_e47892_d_n7, assign32860_e47892_d_n10, assign32860_e47892_d_n11, assign32860_e47892_d_n12, assign32860_e47892_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32860_e47892;
        locals.var_qbd_dn0 = assign32860_e47892_d_n0;
        locals.var_qbd_dn2 = assign32860_e47892_d_n2;
        locals.var_qbd_dn6 = assign32860_e47892_d_n6;
        locals.var_qbd_dn7 = assign32860_e47892_d_n7;
        locals.var_qbd_dn10 = assign32860_e47892_d_n10;
        locals.var_qbd_dn11 = assign32860_e47892_d_n11;
        locals.var_qbd_dn12 = assign32860_e47892_d_n12;
        locals.var_qbd_dn17 = assign32860_e47892_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign32870_e47895: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32870_e47895;
        locals.var_guard1081_rv = 0.0;

        let (assign32880_e47909, assign32880_e47909_d_n6, assign32880_e47909_d_n7, assign32880_e47909_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign32880_e47906: f64 = (locals.var_vbdj / p.p186);
        let assign32880_e47907: f64 = (1.0 - assign32880_e47906);
        (assign32880_e47907, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32880_e47909;
        locals.var_arg__blk1059_dn6 = assign32880_e47909_d_n6;
        locals.var_arg__blk1059_dn7 = assign32880_e47909_d_n7;
        locals.var_arg__blk1059_dn12 = assign32880_e47909_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32890_e47912: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign32890_e47912;
        locals.var_guard1082_rv = 0.0;

        let (assign32900_e47927, assign32900_e47927_d_n6, assign32900_e47927_d_n7, assign32900_e47927_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign32900_e47924: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32900_e47925: f64 = (1.0 / assign32900_e47924);
        (assign32900_e47925, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32900_e47927;
        locals.var_sarg_dn6 = assign32900_e47927_d_n6;
        locals.var_sarg_dn7 = assign32900_e47927_d_n7;
        locals.var_sarg_dn12 = assign32900_e47927_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32910_e47943, assign32910_e47943_d_n6, assign32910_e47943_d_n7, assign32910_e47943_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
        let assign32910_e47940: f64 = (-p.p183);
        let assign32910_e47941: f64 = (locals.var_arg__blk1059).powf(assign32910_e47940);
        (assign32910_e47941, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((locals.var_arg__blk1059).powf(assign32910_e47940 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32910_e47941 * (assign32910_e47940 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((locals.var_arg__blk1059).powf(assign32910_e47940 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32910_e47941 * (assign32910_e47940 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((locals.var_arg__blk1059).powf(assign32910_e47940 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32910_e47941 * (assign32910_e47940 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32910_e47943;
        locals.var_sarg_dn6 = assign32910_e47943_d_n6;
        locals.var_sarg_dn7 = assign32910_e47943_d_n7;
        locals.var_sarg_dn12 = assign32910_e47943_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32920_e47967, assign32920_e47967_d_n0, assign32920_e47967_d_n2, assign32920_e47967_d_n6, assign32920_e47967_d_n7, assign32920_e47967_d_n10, assign32920_e47967_d_n11, assign32920_e47967_d_n12, assign32920_e47967_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign32920_e47954: f64 = (p.p186 * locals.var_czbdsw);
        let assign32920_e47958: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32920_e47959: f64 = (1.0 - assign32920_e47958);
        let assign32920_e47960: f64 = (assign32920_e47954 * assign32920_e47959);
        let assign32920_e47963: f64 = (1.0 - p.p183);
        let assign32920_e47964: f64 = (assign32920_e47960 / assign32920_e47963);
        let assign32920_e47965: f64 = (locals.var_qbd + assign32920_e47964);
        (assign32920_e47965, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32920_e47954 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32920_e47963)), (locals.var_qbd_dn7 + ((assign32920_e47954 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32920_e47963)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32920_e47954 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32920_e47963)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32920_e47967;
        locals.var_qbd_dn0 = assign32920_e47967_d_n0;
        locals.var_qbd_dn2 = assign32920_e47967_d_n2;
        locals.var_qbd_dn6 = assign32920_e47967_d_n6;
        locals.var_qbd_dn7 = assign32920_e47967_d_n7;
        locals.var_qbd_dn10 = assign32920_e47967_d_n10;
        locals.var_qbd_dn11 = assign32920_e47967_d_n11;
        locals.var_qbd_dn12 = assign32920_e47967_d_n12;
        locals.var_qbd_dn17 = assign32920_e47967_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign32930_e47970: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign32930_e47970;
        locals.var_guard1083_rv = 0.0;

        let (assign32940_e47984, assign32940_e47984_d_n6, assign32940_e47984_d_n7, assign32940_e47984_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign32940_e47981: f64 = (locals.var_vbdj / p.p187);
        let assign32940_e47982: f64 = (1.0 - assign32940_e47981);
        (assign32940_e47982, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign32940_e47984;
        locals.var_arg__blk1059_dn6 = assign32940_e47984_d_n6;
        locals.var_arg__blk1059_dn7 = assign32940_e47984_d_n7;
        locals.var_arg__blk1059_dn12 = assign32940_e47984_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign32950_e47987: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign32950_e47987;
        locals.var_guard1084_rv = 0.0;

        let (assign32960_e48002, assign32960_e48002_d_n6, assign32960_e48002_d_n7, assign32960_e48002_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign32960_e47999: f64 = (locals.var_arg__blk1059).sqrt();
        let assign32960_e48000: f64 = (1.0 / assign32960_e47999);
        (assign32960_e48000, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32960_e48002;
        locals.var_sarg_dn6 = assign32960_e48002_d_n6;
        locals.var_sarg_dn7 = assign32960_e48002_d_n7;
        locals.var_sarg_dn12 = assign32960_e48002_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32970_e48018, assign32970_e48018_d_n6, assign32970_e48018_d_n7, assign32970_e48018_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 == 0.0)) {
        let assign32970_e48015: f64 = (-p.p184);
        let assign32970_e48016: f64 = (locals.var_arg__blk1059).powf(assign32970_e48015);
        (assign32970_e48016, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((locals.var_arg__blk1059).powf(assign32970_e48015 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32970_e48016 * (assign32970_e48015 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((locals.var_arg__blk1059).powf(assign32970_e48015 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32970_e48016 * (assign32970_e48015 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((locals.var_arg__blk1059).powf(assign32970_e48015 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32970_e48016 * (assign32970_e48015 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign32970_e48018;
        locals.var_sarg_dn6 = assign32970_e48018_d_n6;
        locals.var_sarg_dn7 = assign32970_e48018_d_n7;
        locals.var_sarg_dn12 = assign32970_e48018_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign32980_e48042, assign32980_e48042_d_n0, assign32980_e48042_d_n2, assign32980_e48042_d_n6, assign32980_e48042_d_n7, assign32980_e48042_d_n10, assign32980_e48042_d_n11, assign32980_e48042_d_n12, assign32980_e48042_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign32980_e48029: f64 = (p.p187 * locals.var_czbdswg);
        let assign32980_e48033: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign32980_e48034: f64 = (1.0 - assign32980_e48033);
        let assign32980_e48035: f64 = (assign32980_e48029 * assign32980_e48034);
        let assign32980_e48038: f64 = (1.0 - p.p184);
        let assign32980_e48039: f64 = (assign32980_e48035 / assign32980_e48038);
        let assign32980_e48040: f64 = (locals.var_qbd + assign32980_e48039);
        (assign32980_e48040, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32980_e48029 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32980_e48038)), (locals.var_qbd_dn7 + ((assign32980_e48029 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32980_e48038)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32980_e48029 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32980_e48038)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign32980_e48042;
        locals.var_qbd_dn0 = assign32980_e48042_d_n0;
        locals.var_qbd_dn2 = assign32980_e48042_d_n2;
        locals.var_qbd_dn6 = assign32980_e48042_d_n6;
        locals.var_qbd_dn7 = assign32980_e48042_d_n7;
        locals.var_qbd_dn10 = assign32980_e48042_d_n10;
        locals.var_qbd_dn11 = assign32980_e48042_d_n11;
        locals.var_qbd_dn12 = assign32980_e48042_d_n12;
        locals.var_qbd_dn17 = assign32980_e48042_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign32990_e48055, assign32990_e48055_d_n6, assign32990_e48055_d_n7, assign32990_e48055_d_n10, assign32990_e48055_d_n12,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
        let assign32990_e48051: f64 = (locals.var_czbd + locals.var_czbdsw);
        let assign32990_e48053: f64 = (assign32990_e48051 + locals.var_czbdswg);
        (assign32990_e48053, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign32990_e48055;
        locals.var_t1__blk1034_dn6 = assign32990_e48055_d_n6;
        locals.var_t1__blk1034_dn7 = assign32990_e48055_d_n7;
        locals.var_t1__blk1034_dn10 = assign32990_e48055_d_n10;
        locals.var_t1__blk1034_dn12 = assign32990_e48055_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign33000_e48080, assign33000_e48080_d_n0, assign33000_e48080_d_n2, assign33000_e48080_d_n6, assign33000_e48080_d_n7, assign33000_e48080_d_n10, assign33000_e48080_d_n11, assign33000_e48080_d_n12, assign33000_e48080_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
        let assign33000_e48064: f64 = (locals.var_czbd * p.p182);
        let assign33000_e48066: f64 = (assign33000_e48064 / p.p185);
        let assign33000_e48069: f64 = (locals.var_czbdsw * p.p183);
        let assign33000_e48071: f64 = (assign33000_e48069 / p.p186);
        let assign33000_e48072: f64 = (assign33000_e48066 + assign33000_e48071);
        let assign33000_e48075: f64 = (locals.var_czbdswg * p.p184);
        let assign33000_e48077: f64 = (assign33000_e48075 / p.p187);
        let assign33000_e48078: f64 = (assign33000_e48072 + assign33000_e48077);
        (assign33000_e48078, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17,)
    }
};
        locals.var_t2__blk1035 = assign33000_e48080;
        locals.var_t2__blk1035_dn0 = assign33000_e48080_d_n0;
        locals.var_t2__blk1035_dn2 = assign33000_e48080_d_n2;
        locals.var_t2__blk1035_dn6 = assign33000_e48080_d_n6;
        locals.var_t2__blk1035_dn7 = assign33000_e48080_d_n7;
        locals.var_t2__blk1035_dn10 = assign33000_e48080_d_n10;
        locals.var_t2__blk1035_dn11 = assign33000_e48080_d_n11;
        locals.var_t2__blk1035_dn12 = assign33000_e48080_d_n12;
        locals.var_t2__blk1035_dn17 = assign33000_e48080_d_n17;
        locals.var_t2__blk1035_rv = 0.0;

        let (assign33010_e48097, assign33010_e48097_d_n0, assign33010_e48097_d_n2, assign33010_e48097_d_n6, assign33010_e48097_d_n7, assign33010_e48097_d_n10, assign33010_e48097_d_n11, assign33010_e48097_d_n12, assign33010_e48097_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
        let assign33010_e48091: f64 = (locals.var_vbdj * 0.5);
        let assign33010_e48093: f64 = (assign33010_e48091 * locals.var_t2__blk1035);
        let assign33010_e48094: f64 = (locals.var_t1__blk1034 + assign33010_e48093);
        let assign33010_e48095: f64 = (locals.var_vbdj * assign33010_e48094);
        (assign33010_e48095, (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn0)), (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn2)), ((locals.var_vbdj_dn6 * assign33010_e48094) + (locals.var_vbdj * (locals.var_t1__blk1034_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1035) + (assign33010_e48091 * locals.var_t2__blk1035_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1034_dn7 + (assign33010_e48091 * locals.var_t2__blk1035_dn7))), (locals.var_vbdj * (locals.var_t1__blk1034_dn10 + (assign33010_e48091 * locals.var_t2__blk1035_dn10))), (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn11)), ((locals.var_vbdj_dn12 * assign33010_e48094) + (locals.var_vbdj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign33010_e48091 * locals.var_t2__blk1035_dn12))))), (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33010_e48097;
        locals.var_qbd_dn0 = assign33010_e48097_d_n0;
        locals.var_qbd_dn2 = assign33010_e48097_d_n2;
        locals.var_qbd_dn6 = assign33010_e48097_d_n6;
        locals.var_qbd_dn7 = assign33010_e48097_d_n7;
        locals.var_qbd_dn10 = assign33010_e48097_d_n10;
        locals.var_qbd_dn11 = assign33010_e48097_d_n11;
        locals.var_qbd_dn12 = assign33010_e48097_d_n12;
        locals.var_qbd_dn17 = assign33010_e48097_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign33020_e48106,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) {
        let assign33020_e48104: f64 = (p.p181 * p.p4);
        (assign33020_e48104,)
    } else {
        (locals.var_czbdswg,)
    }
};
        locals.var_czbdswg = assign33020_e48106;
        locals.var_czbdswg_rv = 0.0;

        let assign33030_e48109: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33030_e48109;
        locals.var_guard1085_rv = 0.0;

        let assign33040_e48112: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33040_e48112;
        locals.var_guard1086_rv = 0.0;

        let (assign33050_e48127, assign33050_e48127_d_n6, assign33050_e48127_d_n7, assign33050_e48127_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) {
        let assign33050_e48124: f64 = (locals.var_vbdj / p.p185);
        let assign33050_e48125: f64 = (1.0 - assign33050_e48124);
        (assign33050_e48125, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign33050_e48127;
        locals.var_arg__blk1059_dn6 = assign33050_e48127_d_n6;
        locals.var_arg__blk1059_dn7 = assign33050_e48127_d_n7;
        locals.var_arg__blk1059_dn12 = assign33050_e48127_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign33060_e48130: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33060_e48130;
        locals.var_guard1087_rv = 0.0;

        let (assign33070_e48146, assign33070_e48146_d_n6, assign33070_e48146_d_n7, assign33070_e48146_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 != 0.0)) {
        let assign33070_e48143: f64 = (locals.var_arg__blk1059).sqrt();
        let assign33070_e48144: f64 = (1.0 / assign33070_e48143);
        (assign33070_e48144, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33070_e48146;
        locals.var_sarg_dn6 = assign33070_e48146_d_n6;
        locals.var_sarg_dn7 = assign33070_e48146_d_n7;
        locals.var_sarg_dn12 = assign33070_e48146_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33080_e48163, assign33080_e48163_d_n6, assign33080_e48163_d_n7, assign33080_e48163_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 == 0.0)) {
        let assign33080_e48160: f64 = (-p.p182);
        let assign33080_e48161: f64 = (locals.var_arg__blk1059).powf(assign33080_e48160);
        (assign33080_e48161, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((locals.var_arg__blk1059).powf(assign33080_e48160 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign33080_e48161 * (assign33080_e48160 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((locals.var_arg__blk1059).powf(assign33080_e48160 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign33080_e48161 * (assign33080_e48160 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((locals.var_arg__blk1059).powf(assign33080_e48160 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign33080_e48161 * (assign33080_e48160 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33080_e48163;
        locals.var_sarg_dn6 = assign33080_e48163_d_n6;
        locals.var_sarg_dn7 = assign33080_e48163_d_n7;
        locals.var_sarg_dn12 = assign33080_e48163_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33090_e48186, assign33090_e48186_d_n0, assign33090_e48186_d_n2, assign33090_e48186_d_n6, assign33090_e48186_d_n7, assign33090_e48186_d_n10, assign33090_e48186_d_n11, assign33090_e48186_d_n12, assign33090_e48186_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) {
        let assign33090_e48174: f64 = (p.p185 * locals.var_czbd);
        let assign33090_e48178: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign33090_e48179: f64 = (1.0 - assign33090_e48178);
        let assign33090_e48180: f64 = (assign33090_e48174 * assign33090_e48179);
        let assign33090_e48183: f64 = (1.0 - p.p182);
        let assign33090_e48184: f64 = (assign33090_e48180 / assign33090_e48183);
        (assign33090_e48184, 0.0, 0.0, ((assign33090_e48174 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign33090_e48183), ((assign33090_e48174 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign33090_e48183), 0.0, 0.0, ((assign33090_e48174 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign33090_e48183), 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33090_e48186;
        locals.var_qbd_dn0 = assign33090_e48186_d_n0;
        locals.var_qbd_dn2 = assign33090_e48186_d_n2;
        locals.var_qbd_dn6 = assign33090_e48186_d_n6;
        locals.var_qbd_dn7 = assign33090_e48186_d_n7;
        locals.var_qbd_dn10 = assign33090_e48186_d_n10;
        locals.var_qbd_dn11 = assign33090_e48186_d_n11;
        locals.var_qbd_dn12 = assign33090_e48186_d_n12;
        locals.var_qbd_dn17 = assign33090_e48186_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign33100_e48198, assign33100_e48198_d_n0, assign33100_e48198_d_n2, assign33100_e48198_d_n6, assign33100_e48198_d_n7, assign33100_e48198_d_n10, assign33100_e48198_d_n11, assign33100_e48198_d_n12, assign33100_e48198_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33100_e48198;
        locals.var_qbd_dn0 = assign33100_e48198_d_n0;
        locals.var_qbd_dn2 = assign33100_e48198_d_n2;
        locals.var_qbd_dn6 = assign33100_e48198_d_n6;
        locals.var_qbd_dn7 = assign33100_e48198_d_n7;
        locals.var_qbd_dn10 = assign33100_e48198_d_n10;
        locals.var_qbd_dn11 = assign33100_e48198_d_n11;
        locals.var_qbd_dn12 = assign33100_e48198_d_n12;
        locals.var_qbd_dn17 = assign33100_e48198_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign33110_e48201: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1088 = assign33110_e48201;
        locals.var_guard1088_rv = 0.0;

        let (assign33120_e48216, assign33120_e48216_d_n6, assign33120_e48216_d_n7, assign33120_e48216_d_n12,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign33120_e48213: f64 = (locals.var_vbdj / p.p187);
        let assign33120_e48214: f64 = (1.0 - assign33120_e48213);
        (assign33120_e48214, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)),)
    } else {
        (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12,)
    }
};
        locals.var_arg__blk1059 = assign33120_e48216;
        locals.var_arg__blk1059_dn6 = assign33120_e48216_d_n6;
        locals.var_arg__blk1059_dn7 = assign33120_e48216_d_n7;
        locals.var_arg__blk1059_dn12 = assign33120_e48216_d_n12;
        locals.var_arg__blk1059_rv = 0.0;

        let assign33130_e48219: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1089 = assign33130_e48219;
        locals.var_guard1089_rv = 0.0;

        let (assign33140_e48235, assign33140_e48235_d_n6, assign33140_e48235_d_n7, assign33140_e48235_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 != 0.0)) {
        let assign33140_e48232: f64 = (locals.var_arg__blk1059).sqrt();
        let assign33140_e48233: f64 = (1.0 / assign33140_e48232);
        (assign33140_e48233, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33140_e48235;
        locals.var_sarg_dn6 = assign33140_e48235_d_n6;
        locals.var_sarg_dn7 = assign33140_e48235_d_n7;
        locals.var_sarg_dn12 = assign33140_e48235_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33150_e48252, assign33150_e48252_d_n6, assign33150_e48252_d_n7, assign33150_e48252_d_n12,) = {
    if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 == 0.0)) {
        let assign33150_e48249: f64 = (-p.p184);
        let assign33150_e48250: f64 = (locals.var_arg__blk1059).powf(assign33150_e48249);
        (assign33150_e48250, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((locals.var_arg__blk1059).powf(assign33150_e48249 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign33150_e48250 * (assign33150_e48249 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((locals.var_arg__blk1059).powf(assign33150_e48249 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign33150_e48250 * (assign33150_e48249 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((locals.var_arg__blk1059).powf(assign33150_e48249 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign33150_e48250 * (assign33150_e48249 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12,)
    }
};
        locals.var_sarg = assign33150_e48252;
        locals.var_sarg_dn6 = assign33150_e48252_d_n6;
        locals.var_sarg_dn7 = assign33150_e48252_d_n7;
        locals.var_sarg_dn12 = assign33150_e48252_d_n12;
        locals.var_sarg_rv = 0.0;

        let (assign33160_e48277, assign33160_e48277_d_n0, assign33160_e48277_d_n2, assign33160_e48277_d_n6, assign33160_e48277_d_n7, assign33160_e48277_d_n10, assign33160_e48277_d_n11, assign33160_e48277_d_n12, assign33160_e48277_d_n17,) = {
    if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign33160_e48264: f64 = (p.p187 * locals.var_czbdswg);
        let assign33160_e48268: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
        let assign33160_e48269: f64 = (1.0 - assign33160_e48268);
        let assign33160_e48270: f64 = (assign33160_e48264 * assign33160_e48269);
        let assign33160_e48273: f64 = (1.0 - p.p184);
        let assign33160_e48274: f64 = (assign33160_e48270 / assign33160_e48273);
        let assign33160_e48275: f64 = (locals.var_qbd + assign33160_e48274);
        (assign33160_e48275, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33160_e48264 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign33160_e48273)), (locals.var_qbd_dn7 + ((assign33160_e48264 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign33160_e48273)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33160_e48264 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign33160_e48273)), locals.var_qbd_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33160_e48277;
        locals.var_qbd_dn0 = assign33160_e48277_d_n0;
        locals.var_qbd_dn2 = assign33160_e48277_d_n2;
        locals.var_qbd_dn6 = assign33160_e48277_d_n6;
        locals.var_qbd_dn7 = assign33160_e48277_d_n7;
        locals.var_qbd_dn10 = assign33160_e48277_d_n10;
        locals.var_qbd_dn11 = assign33160_e48277_d_n11;
        locals.var_qbd_dn12 = assign33160_e48277_d_n12;
        locals.var_qbd_dn17 = assign33160_e48277_d_n17;
        locals.var_qbd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_121(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33170_e48289, assign33170_e48289_d_n6, assign33170_e48289_d_n7, assign33170_e48289_d_n10, assign33170_e48289_d_n12,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 == 0.0)) {
        let assign33170_e48287: f64 = (locals.var_czbd + locals.var_czbdswg);
        (assign33170_e48287, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12,)
    }
};
        locals.var_t1__blk1034 = assign33170_e48289;
        locals.var_t1__blk1034_dn6 = assign33170_e48289_d_n6;
        locals.var_t1__blk1034_dn7 = assign33170_e48289_d_n7;
        locals.var_t1__blk1034_dn10 = assign33170_e48289_d_n10;
        locals.var_t1__blk1034_dn12 = assign33170_e48289_d_n12;
        locals.var_t1__blk1034_rv = 0.0;

        let (assign33180_e48309, assign33180_e48309_d_n0, assign33180_e48309_d_n2, assign33180_e48309_d_n6, assign33180_e48309_d_n7, assign33180_e48309_d_n10, assign33180_e48309_d_n11, assign33180_e48309_d_n12, assign33180_e48309_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 == 0.0)) {
        let assign33180_e48299: f64 = (locals.var_czbd * p.p182);
        let assign33180_e48301: f64 = (assign33180_e48299 / p.p185);
        let assign33180_e48304: f64 = (locals.var_czbdswg * p.p184);
        let assign33180_e48306: f64 = (assign33180_e48304 / p.p187);
        let assign33180_e48307: f64 = (assign33180_e48301 + assign33180_e48306);
        (assign33180_e48307, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17,)
    }
};
        locals.var_t2__blk1035 = assign33180_e48309;
        locals.var_t2__blk1035_dn0 = assign33180_e48309_d_n0;
        locals.var_t2__blk1035_dn2 = assign33180_e48309_d_n2;
        locals.var_t2__blk1035_dn6 = assign33180_e48309_d_n6;
        locals.var_t2__blk1035_dn7 = assign33180_e48309_d_n7;
        locals.var_t2__blk1035_dn10 = assign33180_e48309_d_n10;
        locals.var_t2__blk1035_dn11 = assign33180_e48309_d_n11;
        locals.var_t2__blk1035_dn12 = assign33180_e48309_d_n12;
        locals.var_t2__blk1035_dn17 = assign33180_e48309_d_n17;
        locals.var_t2__blk1035_rv = 0.0;

        let (assign33190_e48327, assign33190_e48327_d_n0, assign33190_e48327_d_n2, assign33190_e48327_d_n6, assign33190_e48327_d_n7, assign33190_e48327_d_n10, assign33190_e48327_d_n11, assign33190_e48327_d_n12, assign33190_e48327_d_n17,) = {
    if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 == 0.0)) {
        let assign33190_e48321: f64 = (locals.var_vbdj * 0.5);
        let assign33190_e48323: f64 = (assign33190_e48321 * locals.var_t2__blk1035);
        let assign33190_e48324: f64 = (locals.var_t1__blk1034 + assign33190_e48323);
        let assign33190_e48325: f64 = (locals.var_vbdj * assign33190_e48324);
        (assign33190_e48325, (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn0)), (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn2)), ((locals.var_vbdj_dn6 * assign33190_e48324) + (locals.var_vbdj * (locals.var_t1__blk1034_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1035) + (assign33190_e48321 * locals.var_t2__blk1035_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1034_dn7 + (assign33190_e48321 * locals.var_t2__blk1035_dn7))), (locals.var_vbdj * (locals.var_t1__blk1034_dn10 + (assign33190_e48321 * locals.var_t2__blk1035_dn10))), (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn11)), ((locals.var_vbdj_dn12 * assign33190_e48324) + (locals.var_vbdj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign33190_e48321 * locals.var_t2__blk1035_dn12))))), (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33190_e48327;
        locals.var_qbd_dn0 = assign33190_e48327_d_n0;
        locals.var_qbd_dn2 = assign33190_e48327_d_n2;
        locals.var_qbd_dn6 = assign33190_e48327_d_n6;
        locals.var_qbd_dn7 = assign33190_e48327_d_n7;
        locals.var_qbd_dn10 = assign33190_e48327_d_n10;
        locals.var_qbd_dn11 = assign33190_e48327_d_n11;
        locals.var_qbd_dn12 = assign33190_e48327_d_n12;
        locals.var_qbd_dn17 = assign33190_e48327_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign33200_e48330: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1090 = assign33200_e48330;
        locals.var_guard1090_rv = 0.0;

        let (assign33210_e48343, assign33210_e48343_d_n0, assign33210_e48343_d_n2, assign33210_e48343_d_n6, assign33210_e48343_d_n7, assign33210_e48343_d_n10, assign33210_e48343_d_n11, assign33210_e48343_d_n12, assign33210_e48343_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let assign33210_e48335: f64 = (-1.6021918e-19);
        let assign33210_e48337: f64 = (assign33210_e48335 * locals.var_uc_nsubs);
        let assign33210_e48339: f64 = (assign33210_e48337 * locals.var_xp_max);
        let assign33210_e48341: f64 = (assign33210_e48339 * p.p3);
        (assign33210_e48341, (((assign33210_e48335 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3),)
    } else {
        (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17,)
    }
};
        locals.var_qbs_max = assign33210_e48343;
        locals.var_qbs_max_dn0 = assign33210_e48343_d_n0;
        locals.var_qbs_max_dn2 = assign33210_e48343_d_n2;
        locals.var_qbs_max_dn6 = assign33210_e48343_d_n6;
        locals.var_qbs_max_dn7 = assign33210_e48343_d_n7;
        locals.var_qbs_max_dn10 = assign33210_e48343_d_n10;
        locals.var_qbs_max_dn11 = assign33210_e48343_d_n11;
        locals.var_qbs_max_dn12 = assign33210_e48343_d_n12;
        locals.var_qbs_max_dn17 = assign33210_e48343_d_n17;
        locals.var_qbs_max_rv = 0.0;

        let (assign33220_e48352, assign33220_e48352_d_n0, assign33220_e48352_d_n2, assign33220_e48352_d_n6, assign33220_e48352_d_n7, assign33220_e48352_d_n10, assign33220_e48352_d_n11, assign33220_e48352_d_n12, assign33220_e48352_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let assign33220_e48349: f64 = (-locals.var_qbs_max);
        let assign33220_e48350: f64 = (0.001 * assign33220_e48349);
        (assign33220_e48350, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)),)
    } else {
        (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17,)
    }
};
        locals.var_dlt_qbs = assign33220_e48352;
        locals.var_dlt_qbs_dn0 = assign33220_e48352_d_n0;
        locals.var_dlt_qbs_dn2 = assign33220_e48352_d_n2;
        locals.var_dlt_qbs_dn6 = assign33220_e48352_d_n6;
        locals.var_dlt_qbs_dn7 = assign33220_e48352_d_n7;
        locals.var_dlt_qbs_dn10 = assign33220_e48352_d_n10;
        locals.var_dlt_qbs_dn11 = assign33220_e48352_d_n11;
        locals.var_dlt_qbs_dn12 = assign33220_e48352_d_n12;
        locals.var_dlt_qbs_dn17 = assign33220_e48352_d_n17;
        locals.var_dlt_qbs_rv = 0.0;

        let (assign33230_e48364, assign33230_e48364_d_n0, assign33230_e48364_d_n2, assign33230_e48364_d_n6, assign33230_e48364_d_n7, assign33230_e48364_d_n10, assign33230_e48364_d_n11, assign33230_e48364_d_n12, assign33230_e48364_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let assign33230_e48357: f64 = (-locals.var_qbs_max);
        let assign33230_e48359: f64 = (-locals.var_qbs);
        let assign33230_e48360: f64 = (assign33230_e48357 - assign33230_e48359);
        let assign33230_e48362: f64 = (assign33230_e48360 - locals.var_dlt_qbs);
        (assign33230_e48362, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33230_e48364;
        locals.var_tmf1_dn0 = assign33230_e48364_d_n0;
        locals.var_tmf1_dn2 = assign33230_e48364_d_n2;
        locals.var_tmf1_dn6 = assign33230_e48364_d_n6;
        locals.var_tmf1_dn7 = assign33230_e48364_d_n7;
        locals.var_tmf1_dn10 = assign33230_e48364_d_n10;
        locals.var_tmf1_dn11 = assign33230_e48364_d_n11;
        locals.var_tmf1_dn12 = assign33230_e48364_d_n12;
        locals.var_tmf1_dn17 = assign33230_e48364_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign33240_e48375, assign33240_e48375_d_n0, assign33240_e48375_d_n2, assign33240_e48375_d_n6, assign33240_e48375_d_n7, assign33240_e48375_d_n10, assign33240_e48375_d_n11, assign33240_e48375_d_n12, assign33240_e48375_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let assign33240_e48370: f64 = (-locals.var_qbs_max);
        let assign33240_e48371: f64 = (4.0 * assign33240_e48370);
        let assign33240_e48373: f64 = (assign33240_e48371 * locals.var_dlt_qbs);
        (assign33240_e48373, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33240_e48375;
        locals.var_tmf2_dn0 = assign33240_e48375_d_n0;
        locals.var_tmf2_dn2 = assign33240_e48375_d_n2;
        locals.var_tmf2_dn6 = assign33240_e48375_d_n6;
        locals.var_tmf2_dn7 = assign33240_e48375_d_n7;
        locals.var_tmf2_dn10 = assign33240_e48375_d_n10;
        locals.var_tmf2_dn11 = assign33240_e48375_d_n11;
        locals.var_tmf2_dn12 = assign33240_e48375_d_n12;
        locals.var_tmf2_dn17 = assign33240_e48375_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33250_e48387, assign33250_e48387_d_n0, assign33250_e48387_d_n2, assign33250_e48387_d_n6, assign33250_e48387_d_n7, assign33250_e48387_d_n10, assign33250_e48387_d_n11, assign33250_e48387_d_n12, assign33250_e48387_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let (assign33250_e48385, assign33250_e48385_d_n0, assign33250_e48385_d_n2, assign33250_e48385_d_n6, assign33250_e48385_d_n7, assign33250_e48385_d_n10, assign33250_e48385_d_n11, assign33250_e48385_d_n12, assign33250_e48385_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33250_e48384: f64 = (-locals.var_tmf2);
                (assign33250_e48384, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33250_e48385, assign33250_e48385_d_n0, assign33250_e48385_d_n2, assign33250_e48385_d_n6, assign33250_e48385_d_n7, assign33250_e48385_d_n10, assign33250_e48385_d_n11, assign33250_e48385_d_n12, assign33250_e48385_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33250_e48387;
        locals.var_tmf2_dn0 = assign33250_e48387_d_n0;
        locals.var_tmf2_dn2 = assign33250_e48387_d_n2;
        locals.var_tmf2_dn6 = assign33250_e48387_d_n6;
        locals.var_tmf2_dn7 = assign33250_e48387_d_n7;
        locals.var_tmf2_dn10 = assign33250_e48387_d_n10;
        locals.var_tmf2_dn11 = assign33250_e48387_d_n11;
        locals.var_tmf2_dn12 = assign33250_e48387_d_n12;
        locals.var_tmf2_dn17 = assign33250_e48387_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33260_e48398, assign33260_e48398_d_n0, assign33260_e48398_d_n2, assign33260_e48398_d_n6, assign33260_e48398_d_n7, assign33260_e48398_d_n10, assign33260_e48398_d_n11, assign33260_e48398_d_n12, assign33260_e48398_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let assign33260_e48393: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33260_e48395: f64 = (assign33260_e48393 + locals.var_tmf2);
        let assign33260_e48396: f64 = (assign33260_e48395).sqrt();
        (assign33260_e48396, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33260_e48396)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33260_e48398;
        locals.var_tmf2_dn0 = assign33260_e48398_d_n0;
        locals.var_tmf2_dn2 = assign33260_e48398_d_n2;
        locals.var_tmf2_dn6 = assign33260_e48398_d_n6;
        locals.var_tmf2_dn7 = assign33260_e48398_d_n7;
        locals.var_tmf2_dn10 = assign33260_e48398_d_n10;
        locals.var_tmf2_dn11 = assign33260_e48398_d_n11;
        locals.var_tmf2_dn12 = assign33260_e48398_d_n12;
        locals.var_tmf2_dn17 = assign33260_e48398_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33270_e48411, assign33270_e48411_d_n0, assign33270_e48411_d_n2, assign33270_e48411_d_n6, assign33270_e48411_d_n7, assign33270_e48411_d_n10, assign33270_e48411_d_n11, assign33270_e48411_d_n12, assign33270_e48411_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let assign33270_e48403: f64 = (-locals.var_qbs_max);
        let assign33270_e48407: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33270_e48408: f64 = (0.5 * assign33270_e48407);
        let assign33270_e48409: f64 = (assign33270_e48403 - assign33270_e48408);
        (assign33270_e48409, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33270_e48411;
        locals.var_qbs_dn0 = assign33270_e48411_d_n0;
        locals.var_qbs_dn2 = assign33270_e48411_d_n2;
        locals.var_qbs_dn6 = assign33270_e48411_d_n6;
        locals.var_qbs_dn7 = assign33270_e48411_d_n7;
        locals.var_qbs_dn10 = assign33270_e48411_d_n10;
        locals.var_qbs_dn11 = assign33270_e48411_d_n11;
        locals.var_qbs_dn12 = assign33270_e48411_d_n12;
        locals.var_qbs_dn17 = assign33270_e48411_d_n17;
        locals.var_qbs_rv = 0.0;

        let (assign33280_e48420, assign33280_e48420_d_n0, assign33280_e48420_d_n2, assign33280_e48420_d_n6, assign33280_e48420_d_n7, assign33280_e48420_d_n10, assign33280_e48420_d_n11, assign33280_e48420_d_n12, assign33280_e48420_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
        let assign33280_e48417: f64 = (-1.0);
        let assign33280_e48418: f64 = (locals.var_qbs * assign33280_e48417);
        (assign33280_e48418, (locals.var_qbs_dn0 * assign33280_e48417), (locals.var_qbs_dn2 * assign33280_e48417), (locals.var_qbs_dn6 * assign33280_e48417), (locals.var_qbs_dn7 * assign33280_e48417), (locals.var_qbs_dn10 * assign33280_e48417), (locals.var_qbs_dn11 * assign33280_e48417), (locals.var_qbs_dn12 * assign33280_e48417), (locals.var_qbs_dn17 * assign33280_e48417),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign33280_e48420;
        locals.var_qbs_dn0 = assign33280_e48420_d_n0;
        locals.var_qbs_dn2 = assign33280_e48420_d_n2;
        locals.var_qbs_dn6 = assign33280_e48420_d_n6;
        locals.var_qbs_dn7 = assign33280_e48420_d_n7;
        locals.var_qbs_dn10 = assign33280_e48420_d_n10;
        locals.var_qbs_dn11 = assign33280_e48420_d_n11;
        locals.var_qbs_dn12 = assign33280_e48420_d_n12;
        locals.var_qbs_dn17 = assign33280_e48420_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign33290_e48423: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1091 = assign33290_e48423;
        locals.var_guard1091_rv = 0.0;

        let (assign33300_e48436, assign33300_e48436_d_n0, assign33300_e48436_d_n2, assign33300_e48436_d_n6, assign33300_e48436_d_n7, assign33300_e48436_d_n10, assign33300_e48436_d_n11, assign33300_e48436_d_n12, assign33300_e48436_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let assign33300_e48428: f64 = (-1.6021918e-19);
        let assign33300_e48430: f64 = (assign33300_e48428 * locals.var_uc_nsubs);
        let assign33300_e48432: f64 = (assign33300_e48430 * locals.var_xp_max);
        let assign33300_e48434: f64 = (assign33300_e48432 * p.p2);
        (assign33300_e48434, (((assign33300_e48428 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2),)
    } else {
        (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17,)
    }
};
        locals.var_qbd_max = assign33300_e48436;
        locals.var_qbd_max_dn0 = assign33300_e48436_d_n0;
        locals.var_qbd_max_dn2 = assign33300_e48436_d_n2;
        locals.var_qbd_max_dn6 = assign33300_e48436_d_n6;
        locals.var_qbd_max_dn7 = assign33300_e48436_d_n7;
        locals.var_qbd_max_dn10 = assign33300_e48436_d_n10;
        locals.var_qbd_max_dn11 = assign33300_e48436_d_n11;
        locals.var_qbd_max_dn12 = assign33300_e48436_d_n12;
        locals.var_qbd_max_dn17 = assign33300_e48436_d_n17;
        locals.var_qbd_max_rv = 0.0;

        let (assign33310_e48445, assign33310_e48445_d_n0, assign33310_e48445_d_n2, assign33310_e48445_d_n6, assign33310_e48445_d_n7, assign33310_e48445_d_n10, assign33310_e48445_d_n11, assign33310_e48445_d_n12, assign33310_e48445_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let assign33310_e48442: f64 = (-locals.var_qbd_max);
        let assign33310_e48443: f64 = (0.001 * assign33310_e48442);
        (assign33310_e48443, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)),)
    } else {
        (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17,)
    }
};
        locals.var_dlt_qbd = assign33310_e48445;
        locals.var_dlt_qbd_dn0 = assign33310_e48445_d_n0;
        locals.var_dlt_qbd_dn2 = assign33310_e48445_d_n2;
        locals.var_dlt_qbd_dn6 = assign33310_e48445_d_n6;
        locals.var_dlt_qbd_dn7 = assign33310_e48445_d_n7;
        locals.var_dlt_qbd_dn10 = assign33310_e48445_d_n10;
        locals.var_dlt_qbd_dn11 = assign33310_e48445_d_n11;
        locals.var_dlt_qbd_dn12 = assign33310_e48445_d_n12;
        locals.var_dlt_qbd_dn17 = assign33310_e48445_d_n17;
        locals.var_dlt_qbd_rv = 0.0;

        let (assign33320_e48457, assign33320_e48457_d_n0, assign33320_e48457_d_n2, assign33320_e48457_d_n6, assign33320_e48457_d_n7, assign33320_e48457_d_n10, assign33320_e48457_d_n11, assign33320_e48457_d_n12, assign33320_e48457_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let assign33320_e48450: f64 = (-locals.var_qbd_max);
        let assign33320_e48452: f64 = (-locals.var_qbd);
        let assign33320_e48453: f64 = (assign33320_e48450 - assign33320_e48452);
        let assign33320_e48455: f64 = (assign33320_e48453 - locals.var_dlt_qbd);
        (assign33320_e48455, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign33320_e48457;
        locals.var_tmf1_dn0 = assign33320_e48457_d_n0;
        locals.var_tmf1_dn2 = assign33320_e48457_d_n2;
        locals.var_tmf1_dn6 = assign33320_e48457_d_n6;
        locals.var_tmf1_dn7 = assign33320_e48457_d_n7;
        locals.var_tmf1_dn10 = assign33320_e48457_d_n10;
        locals.var_tmf1_dn11 = assign33320_e48457_d_n11;
        locals.var_tmf1_dn12 = assign33320_e48457_d_n12;
        locals.var_tmf1_dn17 = assign33320_e48457_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign33330_e48468, assign33330_e48468_d_n0, assign33330_e48468_d_n2, assign33330_e48468_d_n6, assign33330_e48468_d_n7, assign33330_e48468_d_n10, assign33330_e48468_d_n11, assign33330_e48468_d_n12, assign33330_e48468_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let assign33330_e48463: f64 = (-locals.var_qbd_max);
        let assign33330_e48464: f64 = (4.0 * assign33330_e48463);
        let assign33330_e48466: f64 = (assign33330_e48464 * locals.var_dlt_qbd);
        (assign33330_e48466, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33330_e48468;
        locals.var_tmf2_dn0 = assign33330_e48468_d_n0;
        locals.var_tmf2_dn2 = assign33330_e48468_d_n2;
        locals.var_tmf2_dn6 = assign33330_e48468_d_n6;
        locals.var_tmf2_dn7 = assign33330_e48468_d_n7;
        locals.var_tmf2_dn10 = assign33330_e48468_d_n10;
        locals.var_tmf2_dn11 = assign33330_e48468_d_n11;
        locals.var_tmf2_dn12 = assign33330_e48468_d_n12;
        locals.var_tmf2_dn17 = assign33330_e48468_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33340_e48480, assign33340_e48480_d_n0, assign33340_e48480_d_n2, assign33340_e48480_d_n6, assign33340_e48480_d_n7, assign33340_e48480_d_n10, assign33340_e48480_d_n11, assign33340_e48480_d_n12, assign33340_e48480_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let (assign33340_e48478, assign33340_e48478_d_n0, assign33340_e48478_d_n2, assign33340_e48478_d_n6, assign33340_e48478_d_n7, assign33340_e48478_d_n10, assign33340_e48478_d_n11, assign33340_e48478_d_n12, assign33340_e48478_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign33340_e48477: f64 = (-locals.var_tmf2);
                (assign33340_e48477, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign33340_e48478, assign33340_e48478_d_n0, assign33340_e48478_d_n2, assign33340_e48478_d_n6, assign33340_e48478_d_n7, assign33340_e48478_d_n10, assign33340_e48478_d_n11, assign33340_e48478_d_n12, assign33340_e48478_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33340_e48480;
        locals.var_tmf2_dn0 = assign33340_e48480_d_n0;
        locals.var_tmf2_dn2 = assign33340_e48480_d_n2;
        locals.var_tmf2_dn6 = assign33340_e48480_d_n6;
        locals.var_tmf2_dn7 = assign33340_e48480_d_n7;
        locals.var_tmf2_dn10 = assign33340_e48480_d_n10;
        locals.var_tmf2_dn11 = assign33340_e48480_d_n11;
        locals.var_tmf2_dn12 = assign33340_e48480_d_n12;
        locals.var_tmf2_dn17 = assign33340_e48480_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33350_e48491, assign33350_e48491_d_n0, assign33350_e48491_d_n2, assign33350_e48491_d_n6, assign33350_e48491_d_n7, assign33350_e48491_d_n10, assign33350_e48491_d_n11, assign33350_e48491_d_n12, assign33350_e48491_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let assign33350_e48486: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign33350_e48488: f64 = (assign33350_e48486 + locals.var_tmf2);
        let assign33350_e48489: f64 = (assign33350_e48488).sqrt();
        (assign33350_e48489, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33350_e48489)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign33350_e48491;
        locals.var_tmf2_dn0 = assign33350_e48491_d_n0;
        locals.var_tmf2_dn2 = assign33350_e48491_d_n2;
        locals.var_tmf2_dn6 = assign33350_e48491_d_n6;
        locals.var_tmf2_dn7 = assign33350_e48491_d_n7;
        locals.var_tmf2_dn10 = assign33350_e48491_d_n10;
        locals.var_tmf2_dn11 = assign33350_e48491_d_n11;
        locals.var_tmf2_dn12 = assign33350_e48491_d_n12;
        locals.var_tmf2_dn17 = assign33350_e48491_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign33360_e48504, assign33360_e48504_d_n0, assign33360_e48504_d_n2, assign33360_e48504_d_n6, assign33360_e48504_d_n7, assign33360_e48504_d_n10, assign33360_e48504_d_n11, assign33360_e48504_d_n12, assign33360_e48504_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let assign33360_e48496: f64 = (-locals.var_qbd_max);
        let assign33360_e48500: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign33360_e48501: f64 = (0.5 * assign33360_e48500);
        let assign33360_e48502: f64 = (assign33360_e48496 - assign33360_e48501);
        (assign33360_e48502, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33360_e48504;
        locals.var_qbd_dn0 = assign33360_e48504_d_n0;
        locals.var_qbd_dn2 = assign33360_e48504_d_n2;
        locals.var_qbd_dn6 = assign33360_e48504_d_n6;
        locals.var_qbd_dn7 = assign33360_e48504_d_n7;
        locals.var_qbd_dn10 = assign33360_e48504_d_n10;
        locals.var_qbd_dn11 = assign33360_e48504_d_n11;
        locals.var_qbd_dn12 = assign33360_e48504_d_n12;
        locals.var_qbd_dn17 = assign33360_e48504_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign33370_e48513, assign33370_e48513_d_n0, assign33370_e48513_d_n2, assign33370_e48513_d_n6, assign33370_e48513_d_n7, assign33370_e48513_d_n10, assign33370_e48513_d_n11, assign33370_e48513_d_n12, assign33370_e48513_d_n17,) = {
    if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
        let assign33370_e48510: f64 = (-1.0);
        let assign33370_e48511: f64 = (locals.var_qbd * assign33370_e48510);
        (assign33370_e48511, (locals.var_qbd_dn0 * assign33370_e48510), (locals.var_qbd_dn2 * assign33370_e48510), (locals.var_qbd_dn6 * assign33370_e48510), (locals.var_qbd_dn7 * assign33370_e48510), (locals.var_qbd_dn10 * assign33370_e48510), (locals.var_qbd_dn11 * assign33370_e48510), (locals.var_qbd_dn12 * assign33370_e48510), (locals.var_qbd_dn17 * assign33370_e48510),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign33370_e48513;
        locals.var_qbd_dn0 = assign33370_e48513_d_n0;
        locals.var_qbd_dn2 = assign33370_e48513_d_n2;
        locals.var_qbd_dn6 = assign33370_e48513_d_n6;
        locals.var_qbd_dn7 = assign33370_e48513_d_n7;
        locals.var_qbd_dn10 = assign33370_e48513_d_n10;
        locals.var_qbd_dn11 = assign33370_e48513_d_n11;
        locals.var_qbd_dn12 = assign33370_e48513_d_n12;
        locals.var_qbd_dn17 = assign33370_e48513_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign33600_e48767: f64 = if ((p.p32 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign33600_e48767;
        locals.var_guard1124_rv = 0.0;

        let (assign33610_e48775, assign33610_e48775_d_n0, assign33610_e48775_d_n2, assign33610_e48775_d_n6, assign33610_e48775_d_n7, assign33610_e48775_d_n10, assign33610_e48775_d_n11, assign33610_e48775_d_n12, assign33610_e48775_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33610_e48771: f64 = (locals.var_psdl - locals.var_ps0);
        let assign33610_e48773: f64 = (assign33610_e48771 / locals.var_lch);
        (assign33610_e48773, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn17 - locals.var_ps0_dn17) * locals.var_lch) - (assign33610_e48771 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12, locals.var_eyd_dn17,)
    }
};
        locals.var_eyd = assign33610_e48775;
        locals.var_eyd_dn0 = assign33610_e48775_d_n0;
        locals.var_eyd_dn2 = assign33610_e48775_d_n2;
        locals.var_eyd_dn6 = assign33610_e48775_d_n6;
        locals.var_eyd_dn7 = assign33610_e48775_d_n7;
        locals.var_eyd_dn10 = assign33610_e48775_d_n10;
        locals.var_eyd_dn11 = assign33610_e48775_d_n11;
        locals.var_eyd_dn12 = assign33610_e48775_d_n12;
        locals.var_eyd_dn17 = assign33610_e48775_d_n17;
        locals.var_eyd_rv = 0.0;

        let (assign33620_e48783, assign33620_e48783_d_n0, assign33620_e48783_d_n2, assign33620_e48783_d_n6, assign33620_e48783_d_n7, assign33620_e48783_d_n10, assign33620_e48783_d_n11, assign33620_e48783_d_n12, assign33620_e48783_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33620_e48779: f64 = (locals.var_muun * locals.var_eyd);
        let assign33620_e48781: f64 = (assign33620_e48779 / 100000.0);
        (assign33620_e48781, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 100000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 100000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 100000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 100000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 100000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 100000.0), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / 100000.0), (((locals.var_muun_dn17 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn17)) / 100000.0),)
    } else {
        (locals.var_t12__blk1108, locals.var_t12__blk1108_dn0, locals.var_t12__blk1108_dn2, locals.var_t12__blk1108_dn6, locals.var_t12__blk1108_dn7, locals.var_t12__blk1108_dn10, locals.var_t12__blk1108_dn11, locals.var_t12__blk1108_dn12, locals.var_t12__blk1108_dn17,)
    }
};
        locals.var_t12__blk1108 = assign33620_e48783;
        locals.var_t12__blk1108_dn0 = assign33620_e48783_d_n0;
        locals.var_t12__blk1108_dn2 = assign33620_e48783_d_n2;
        locals.var_t12__blk1108_dn6 = assign33620_e48783_d_n6;
        locals.var_t12__blk1108_dn7 = assign33620_e48783_d_n7;
        locals.var_t12__blk1108_dn10 = assign33620_e48783_d_n10;
        locals.var_t12__blk1108_dn11 = assign33620_e48783_d_n11;
        locals.var_t12__blk1108_dn12 = assign33620_e48783_d_n12;
        locals.var_t12__blk1108_dn17 = assign33620_e48783_d_n17;
        locals.var_t12__blk1108_rv = 0.0;

        let assign33630_e48787: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48788: f64 = (1.0 - assign33630_e48787);
        let assign33630_e48795: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48796: f64 = (1.0 + assign33630_e48795);
        let assign33630_e48798: f64 = if ((assign33630_e48788 <= p.p113) && (p.p113 <= assign33630_e48796)) { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign33630_e48798;
        locals.var_guard1125_rv = 0.0;

        let (assign33640_e48804, assign33640_e48804_d_n0, assign33640_e48804_d_n2, assign33640_e48804_d_n6, assign33640_e48804_d_n7, assign33640_e48804_d_n10, assign33640_e48804_d_n11, assign33640_e48804_d_n12, assign33640_e48804_d_n17,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk1109, locals.var_t7__blk1109_dn0, locals.var_t7__blk1109_dn2, locals.var_t7__blk1109_dn6, locals.var_t7__blk1109_dn7, locals.var_t7__blk1109_dn10, locals.var_t7__blk1109_dn11, locals.var_t7__blk1109_dn12, locals.var_t7__blk1109_dn17,)
    }
};
        locals.var_t7__blk1109 = assign33640_e48804;
        locals.var_t7__blk1109_dn0 = assign33640_e48804_d_n0;
        locals.var_t7__blk1109_dn2 = assign33640_e48804_d_n2;
        locals.var_t7__blk1109_dn6 = assign33640_e48804_d_n6;
        locals.var_t7__blk1109_dn7 = assign33640_e48804_d_n7;
        locals.var_t7__blk1109_dn10 = assign33640_e48804_d_n10;
        locals.var_t7__blk1109_dn11 = assign33640_e48804_d_n11;
        locals.var_t7__blk1109_dn12 = assign33640_e48804_d_n12;
        locals.var_t7__blk1109_dn17 = assign33640_e48804_d_n17;
        locals.var_t7__blk1109_rv = 0.0;

        let assign33650_e48808: f64 = (10.0 * 2.220446049250313e-16);
        let assign33650_e48809: f64 = (2.0 - assign33650_e48808);
        let assign33650_e48816: f64 = (10.0 * 2.220446049250313e-16);
        let assign33650_e48817: f64 = (2.0 + assign33650_e48816);
        let assign33650_e48819: f64 = if ((assign33650_e48809 <= p.p113) && (p.p113 <= assign33650_e48817)) { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign33650_e48819;
        locals.var_guard1126_rv = 0.0;

        let (assign33660_e48828, assign33660_e48828_d_n0, assign33660_e48828_d_n2, assign33660_e48828_d_n6, assign33660_e48828_d_n7, assign33660_e48828_d_n10, assign33660_e48828_d_n11, assign33660_e48828_d_n12, assign33660_e48828_d_n17,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) && (locals.var_guard1126 != 0.0)) {
        (locals.var_t12__blk1108, locals.var_t12__blk1108_dn0, locals.var_t12__blk1108_dn2, locals.var_t12__blk1108_dn6, locals.var_t12__blk1108_dn7, locals.var_t12__blk1108_dn10, locals.var_t12__blk1108_dn11, locals.var_t12__blk1108_dn12, locals.var_t12__blk1108_dn17,)
    } else {
        (locals.var_t7__blk1109, locals.var_t7__blk1109_dn0, locals.var_t7__blk1109_dn2, locals.var_t7__blk1109_dn6, locals.var_t7__blk1109_dn7, locals.var_t7__blk1109_dn10, locals.var_t7__blk1109_dn11, locals.var_t7__blk1109_dn12, locals.var_t7__blk1109_dn17,)
    }
};
        locals.var_t7__blk1109 = assign33660_e48828;
        locals.var_t7__blk1109_dn0 = assign33660_e48828_d_n0;
        locals.var_t7__blk1109_dn2 = assign33660_e48828_d_n2;
        locals.var_t7__blk1109_dn6 = assign33660_e48828_d_n6;
        locals.var_t7__blk1109_dn7 = assign33660_e48828_d_n7;
        locals.var_t7__blk1109_dn10 = assign33660_e48828_d_n10;
        locals.var_t7__blk1109_dn11 = assign33660_e48828_d_n11;
        locals.var_t7__blk1109_dn12 = assign33660_e48828_d_n12;
        locals.var_t7__blk1109_dn17 = assign33660_e48828_d_n17;
        locals.var_t7__blk1109_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_122(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33670_e48842, assign33670_e48842_d_n0, assign33670_e48842_d_n2, assign33670_e48842_d_n6, assign33670_e48842_d_n7, assign33670_e48842_d_n10, assign33670_e48842_d_n11, assign33670_e48842_d_n12, assign33670_e48842_d_n17,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) && (locals.var_guard1126 == 0.0)) {
        let assign33670_e48839: f64 = (p.p113 - 1.0);
        let assign33670_e48840: f64 = (locals.var_t12__blk1108).powf(assign33670_e48839);
        (assign33670_e48840, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn0)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn0 / locals.var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn2)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn2 / locals.var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn6)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn6 / locals.var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn7)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn7 / locals.var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn10)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn10 / locals.var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn11)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn11 / locals.var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn12)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn12 / locals.var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((locals.var_t12__blk1108).powf(assign33670_e48839 - 1.0) * locals.var_t12__blk1108_dn17)) } } else { (assign33670_e48840 * (assign33670_e48839 * (locals.var_t12__blk1108_dn17 / locals.var_t12__blk1108))) },)
    } else {
        (locals.var_t7__blk1109, locals.var_t7__blk1109_dn0, locals.var_t7__blk1109_dn2, locals.var_t7__blk1109_dn6, locals.var_t7__blk1109_dn7, locals.var_t7__blk1109_dn10, locals.var_t7__blk1109_dn11, locals.var_t7__blk1109_dn12, locals.var_t7__blk1109_dn17,)
    }
};
        locals.var_t7__blk1109 = assign33670_e48842;
        locals.var_t7__blk1109_dn0 = assign33670_e48842_d_n0;
        locals.var_t7__blk1109_dn2 = assign33670_e48842_d_n2;
        locals.var_t7__blk1109_dn6 = assign33670_e48842_d_n6;
        locals.var_t7__blk1109_dn7 = assign33670_e48842_d_n7;
        locals.var_t7__blk1109_dn10 = assign33670_e48842_d_n10;
        locals.var_t7__blk1109_dn11 = assign33670_e48842_d_n11;
        locals.var_t7__blk1109_dn12 = assign33670_e48842_d_n12;
        locals.var_t7__blk1109_dn17 = assign33670_e48842_d_n17;
        locals.var_t7__blk1109_rv = 0.0;

        let (assign33680_e48848, assign33680_e48848_d_n0, assign33680_e48848_d_n2, assign33680_e48848_d_n6, assign33680_e48848_d_n7, assign33680_e48848_d_n10, assign33680_e48848_d_n11, assign33680_e48848_d_n12, assign33680_e48848_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33680_e48846: f64 = (locals.var_t12__blk1108 * locals.var_t7__blk1109);
        (assign33680_e48846, ((locals.var_t12__blk1108_dn0 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn0)), ((locals.var_t12__blk1108_dn2 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn2)), ((locals.var_t12__blk1108_dn6 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn6)), ((locals.var_t12__blk1108_dn7 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn7)), ((locals.var_t12__blk1108_dn10 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn10)), ((locals.var_t12__blk1108_dn11 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn11)), ((locals.var_t12__blk1108_dn12 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn12)), ((locals.var_t12__blk1108_dn17 * locals.var_t7__blk1109) + (locals.var_t12__blk1108 * locals.var_t7__blk1109_dn17)),)
    } else {
        (locals.var_t8__blk1110, locals.var_t8__blk1110_dn0, locals.var_t8__blk1110_dn2, locals.var_t8__blk1110_dn6, locals.var_t8__blk1110_dn7, locals.var_t8__blk1110_dn10, locals.var_t8__blk1110_dn11, locals.var_t8__blk1110_dn12, locals.var_t8__blk1110_dn17,)
    }
};
        locals.var_t8__blk1110 = assign33680_e48848;
        locals.var_t8__blk1110_dn0 = assign33680_e48848_d_n0;
        locals.var_t8__blk1110_dn2 = assign33680_e48848_d_n2;
        locals.var_t8__blk1110_dn6 = assign33680_e48848_d_n6;
        locals.var_t8__blk1110_dn7 = assign33680_e48848_d_n7;
        locals.var_t8__blk1110_dn10 = assign33680_e48848_d_n10;
        locals.var_t8__blk1110_dn11 = assign33680_e48848_d_n11;
        locals.var_t8__blk1110_dn12 = assign33680_e48848_d_n12;
        locals.var_t8__blk1110_dn17 = assign33680_e48848_d_n17;
        locals.var_t8__blk1110_rv = 0.0;

        let (assign33690_e48854, assign33690_e48854_d_n0, assign33690_e48854_d_n2, assign33690_e48854_d_n6, assign33690_e48854_d_n7, assign33690_e48854_d_n10, assign33690_e48854_d_n11, assign33690_e48854_d_n12, assign33690_e48854_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33690_e48852: f64 = (1.0 + locals.var_t8__blk1110);
        (assign33690_e48852, locals.var_t8__blk1110_dn0, locals.var_t8__blk1110_dn2, locals.var_t8__blk1110_dn6, locals.var_t8__blk1110_dn7, locals.var_t8__blk1110_dn10, locals.var_t8__blk1110_dn11, locals.var_t8__blk1110_dn12, locals.var_t8__blk1110_dn17,)
    } else {
        (locals.var_t9__blk1111, locals.var_t9__blk1111_dn0, locals.var_t9__blk1111_dn2, locals.var_t9__blk1111_dn6, locals.var_t9__blk1111_dn7, locals.var_t9__blk1111_dn10, locals.var_t9__blk1111_dn11, locals.var_t9__blk1111_dn12, locals.var_t9__blk1111_dn17,)
    }
};
        locals.var_t9__blk1111 = assign33690_e48854;
        locals.var_t9__blk1111_dn0 = assign33690_e48854_d_n0;
        locals.var_t9__blk1111_dn2 = assign33690_e48854_d_n2;
        locals.var_t9__blk1111_dn6 = assign33690_e48854_d_n6;
        locals.var_t9__blk1111_dn7 = assign33690_e48854_d_n7;
        locals.var_t9__blk1111_dn10 = assign33690_e48854_d_n10;
        locals.var_t9__blk1111_dn11 = assign33690_e48854_d_n11;
        locals.var_t9__blk1111_dn12 = assign33690_e48854_d_n12;
        locals.var_t9__blk1111_dn17 = assign33690_e48854_d_n17;
        locals.var_t9__blk1111_rv = 0.0;

        let (assign33700_e48865, assign33700_e48865_d_n0, assign33700_e48865_d_n2, assign33700_e48865_d_n6, assign33700_e48865_d_n7, assign33700_e48865_d_n10, assign33700_e48865_d_n11, assign33700_e48865_d_n12, assign33700_e48865_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33700_e48858: f64 = (-1.0);
        let assign33700_e48860: f64 = (assign33700_e48858 / p.p113);
        let assign33700_e48862: f64 = (assign33700_e48860 - 1.0);
        let assign33700_e48863: f64 = (locals.var_t9__blk1111).powf(assign33700_e48862);
        (assign33700_e48863, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn0)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn0 / locals.var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn2)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn2 / locals.var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn6)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn6 / locals.var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn7)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn7 / locals.var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn10)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn10 / locals.var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn11)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn11 / locals.var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn12)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn12 / locals.var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((locals.var_t9__blk1111).powf(assign33700_e48862 - 1.0) * locals.var_t9__blk1111_dn17)) } } else { (assign33700_e48863 * (assign33700_e48862 * (locals.var_t9__blk1111_dn17 / locals.var_t9__blk1111))) },)
    } else {
        (locals.var_t10__blk1112, locals.var_t10__blk1112_dn0, locals.var_t10__blk1112_dn2, locals.var_t10__blk1112_dn6, locals.var_t10__blk1112_dn7, locals.var_t10__blk1112_dn10, locals.var_t10__blk1112_dn11, locals.var_t10__blk1112_dn12, locals.var_t10__blk1112_dn17,)
    }
};
        locals.var_t10__blk1112 = assign33700_e48865;
        locals.var_t10__blk1112_dn0 = assign33700_e48865_d_n0;
        locals.var_t10__blk1112_dn2 = assign33700_e48865_d_n2;
        locals.var_t10__blk1112_dn6 = assign33700_e48865_d_n6;
        locals.var_t10__blk1112_dn7 = assign33700_e48865_d_n7;
        locals.var_t10__blk1112_dn10 = assign33700_e48865_d_n10;
        locals.var_t10__blk1112_dn11 = assign33700_e48865_d_n11;
        locals.var_t10__blk1112_dn12 = assign33700_e48865_d_n12;
        locals.var_t10__blk1112_dn17 = assign33700_e48865_d_n17;
        locals.var_t10__blk1112_rv = 0.0;

        let (assign33710_e48871, assign33710_e48871_d_n0, assign33710_e48871_d_n2, assign33710_e48871_d_n6, assign33710_e48871_d_n7, assign33710_e48871_d_n10, assign33710_e48871_d_n11, assign33710_e48871_d_n12, assign33710_e48871_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33710_e48869: f64 = (locals.var_t9__blk1111 * locals.var_t10__blk1112);
        (assign33710_e48869, ((locals.var_t9__blk1111_dn0 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn0)), ((locals.var_t9__blk1111_dn2 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn2)), ((locals.var_t9__blk1111_dn6 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn6)), ((locals.var_t9__blk1111_dn7 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn7)), ((locals.var_t9__blk1111_dn10 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn10)), ((locals.var_t9__blk1111_dn11 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn11)), ((locals.var_t9__blk1111_dn12 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn12)), ((locals.var_t9__blk1111_dn17 * locals.var_t10__blk1112) + (locals.var_t9__blk1111 * locals.var_t10__blk1112_dn17)),)
    } else {
        (locals.var_t11__blk1113, locals.var_t11__blk1113_dn0, locals.var_t11__blk1113_dn2, locals.var_t11__blk1113_dn6, locals.var_t11__blk1113_dn7, locals.var_t11__blk1113_dn10, locals.var_t11__blk1113_dn11, locals.var_t11__blk1113_dn12, locals.var_t11__blk1113_dn17,)
    }
};
        locals.var_t11__blk1113 = assign33710_e48871;
        locals.var_t11__blk1113_dn0 = assign33710_e48871_d_n0;
        locals.var_t11__blk1113_dn2 = assign33710_e48871_d_n2;
        locals.var_t11__blk1113_dn6 = assign33710_e48871_d_n6;
        locals.var_t11__blk1113_dn7 = assign33710_e48871_d_n7;
        locals.var_t11__blk1113_dn10 = assign33710_e48871_d_n10;
        locals.var_t11__blk1113_dn11 = assign33710_e48871_d_n11;
        locals.var_t11__blk1113_dn12 = assign33710_e48871_d_n12;
        locals.var_t11__blk1113_dn17 = assign33710_e48871_d_n17;
        locals.var_t11__blk1113_rv = 0.0;

        let (assign33720_e48877, assign33720_e48877_d_n0, assign33720_e48877_d_n2, assign33720_e48877_d_n6, assign33720_e48877_d_n7, assign33720_e48877_d_n10, assign33720_e48877_d_n11, assign33720_e48877_d_n12, assign33720_e48877_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33720_e48875: f64 = (locals.var_muun * locals.var_t11__blk1113);
        (assign33720_e48875, ((locals.var_muun_dn0 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn0)), ((locals.var_muun_dn2 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn2)), ((locals.var_muun_dn6 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn6)), ((locals.var_muun_dn7 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn7)), ((locals.var_muun_dn10 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn10)), ((locals.var_muun_dn11 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn11)), ((locals.var_muun_dn12 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn12)), ((locals.var_muun_dn17 * locals.var_t11__blk1113) + (locals.var_muun * locals.var_t11__blk1113_dn17)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12, locals.var_mud_hoso_dn17,)
    }
};
        locals.var_mud_hoso = assign33720_e48877;
        locals.var_mud_hoso_dn0 = assign33720_e48877_d_n0;
        locals.var_mud_hoso_dn2 = assign33720_e48877_d_n2;
        locals.var_mud_hoso_dn6 = assign33720_e48877_d_n6;
        locals.var_mud_hoso_dn7 = assign33720_e48877_d_n7;
        locals.var_mud_hoso_dn10 = assign33720_e48877_d_n10;
        locals.var_mud_hoso_dn11 = assign33720_e48877_d_n11;
        locals.var_mud_hoso_dn12 = assign33720_e48877_d_n12;
        locals.var_mud_hoso_dn17 = assign33720_e48877_d_n17;
        locals.var_mud_hoso_rv = 0.0;

        let (assign33730_e48885, assign33730_e48885_d_n0, assign33730_e48885_d_n2, assign33730_e48885_d_n6, assign33730_e48885_d_n7, assign33730_e48885_d_n10, assign33730_e48885_d_n11, assign33730_e48885_d_n12, assign33730_e48885_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33730_e48881: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign33730_e48883: f64 = (assign33730_e48881 / 2.0);
        (assign33730_e48883, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0), ((locals.var_mu_dn17 + locals.var_mud_hoso_dn17) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12, locals.var_mu_ave_dn17,)
    }
};
        locals.var_mu_ave = assign33730_e48885;
        locals.var_mu_ave_dn0 = assign33730_e48885_d_n0;
        locals.var_mu_ave_dn2 = assign33730_e48885_d_n2;
        locals.var_mu_ave_dn6 = assign33730_e48885_d_n6;
        locals.var_mu_ave_dn7 = assign33730_e48885_d_n7;
        locals.var_mu_ave_dn10 = assign33730_e48885_d_n10;
        locals.var_mu_ave_dn11 = assign33730_e48885_d_n11;
        locals.var_mu_ave_dn12 = assign33730_e48885_d_n12;
        locals.var_mu_ave_dn17 = assign33730_e48885_d_n17;
        locals.var_mu_ave_rv = 0.0;

        let (assign33740_e48891, assign33740_e48891_d_n0, assign33740_e48891_d_n2, assign33740_e48891_d_n6, assign33740_e48891_d_n7, assign33740_e48891_d_n10, assign33740_e48891_d_n11, assign33740_e48891_d_n12, assign33740_e48891_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33740_e48889: f64 = (locals.var_alpha * locals.var_alpha);
        (assign33740_e48889, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_t0__blk1114, locals.var_t0__blk1114_dn0, locals.var_t0__blk1114_dn2, locals.var_t0__blk1114_dn6, locals.var_t0__blk1114_dn7, locals.var_t0__blk1114_dn10, locals.var_t0__blk1114_dn11, locals.var_t0__blk1114_dn12, locals.var_t0__blk1114_dn17,)
    }
};
        locals.var_t0__blk1114 = assign33740_e48891;
        locals.var_t0__blk1114_dn0 = assign33740_e48891_d_n0;
        locals.var_t0__blk1114_dn2 = assign33740_e48891_d_n2;
        locals.var_t0__blk1114_dn6 = assign33740_e48891_d_n6;
        locals.var_t0__blk1114_dn7 = assign33740_e48891_d_n7;
        locals.var_t0__blk1114_dn10 = assign33740_e48891_d_n10;
        locals.var_t0__blk1114_dn11 = assign33740_e48891_d_n11;
        locals.var_t0__blk1114_dn12 = assign33740_e48891_d_n12;
        locals.var_t0__blk1114_dn17 = assign33740_e48891_d_n17;
        locals.var_t0__blk1114_rv = 0.0;

        let (assign33750_e48953, assign33750_e48953_d_n0, assign33750_e48953_d_n2, assign33750_e48953_d_n6, assign33750_e48953_d_n7, assign33750_e48953_d_n10, assign33750_e48953_d_n11, assign33750_e48953_d_n12, assign33750_e48953_d_n17,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign33750_e48895: f64 = (locals.var_weff_nf * locals.var_c_fox);
        let assign33750_e48897: f64 = (assign33750_e48895 * locals.var_vgvt);
        let assign33750_e48899: f64 = (assign33750_e48897 * locals.var_mu);
        let assign33750_e48903: f64 = (3.0 * locals.var_alpha);
        let assign33750_e48904: f64 = (1.0 + assign33750_e48903);
        let assign33750_e48907: f64 = (6.0 * locals.var_t0__blk1114);
        let assign33750_e48908: f64 = (assign33750_e48904 + assign33750_e48907);
        let assign33750_e48910: f64 = (assign33750_e48908 * locals.var_mud_hoso);
        let assign33750_e48912: f64 = (assign33750_e48910 * locals.var_mud_hoso);
        let assign33750_e48916: f64 = (4.0 * locals.var_alpha);
        let assign33750_e48917: f64 = (3.0 + assign33750_e48916);
        let assign33750_e48920: f64 = (3.0 * locals.var_t0__blk1114);
        let assign33750_e48921: f64 = (assign33750_e48917 + assign33750_e48920);
        let assign33750_e48923: f64 = (assign33750_e48921 * locals.var_mud_hoso);
        let assign33750_e48925: f64 = (assign33750_e48923 * locals.var_mu);
        let assign33750_e48926: f64 = (assign33750_e48912 + assign33750_e48925);
        let assign33750_e48930: f64 = (3.0 * locals.var_alpha);
        let assign33750_e48931: f64 = (6.0 + assign33750_e48930);
        let assign33750_e48933: f64 = (assign33750_e48931 + locals.var_t0__blk1114);
        let assign33750_e48935: f64 = (assign33750_e48933 * locals.var_mu);
        let assign33750_e48937: f64 = (assign33750_e48935 * locals.var_mu);
        let assign33750_e48938: f64 = (assign33750_e48926 + assign33750_e48937);
        let assign33750_e48939: f64 = (assign33750_e48899 * assign33750_e48938);
        let assign33750_e48942: f64 = (15.0 * locals.var_lch);
        let assign33750_e48945: f64 = (1.0 + locals.var_alpha);
        let assign33750_e48946: f64 = (assign33750_e48942 * assign33750_e48945);
        let assign33750_e48948: f64 = (assign33750_e48946 * locals.var_mu_ave);
        let assign33750_e48950: f64 = (assign33750_e48948 * locals.var_mu_ave);
        let assign33750_e48951: f64 = (assign33750_e48939 / assign33750_e48950);
        (assign33750_e48951, ((((((((((locals.var_weff_nf * locals.var_c_fox_dn0) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn0)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0__blk1114_dn0)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0__blk1114_dn0)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0__blk1114_dn0) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn0)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn0))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn0) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn0)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn2) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn2)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0__blk1114_dn2)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0__blk1114_dn2)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0__blk1114_dn2) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn2)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn2))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn2) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn2)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn6) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn6)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0__blk1114_dn6)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0__blk1114_dn6)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0__blk1114_dn6) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn6)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn6))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn6) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn6)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn7) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn7)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0__blk1114_dn7)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0__blk1114_dn7)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0__blk1114_dn7) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn7)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn7))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn7) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn7)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn10) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn10)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0__blk1114_dn10)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0__blk1114_dn10)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0__blk1114_dn10) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn10)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn10))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn10) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn10)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn11) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn11)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0__blk1114_dn11)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0__blk1114_dn11)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0__blk1114_dn11) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn11)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn11))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn11) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn11)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn12) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn12)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0__blk1114_dn12)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0__blk1114_dn12)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0__blk1114_dn12) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn12)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn12))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn12) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn12)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn17) * locals.var_vgvt) + (assign33750_e48895 * locals.var_vgvt_dn17)) * locals.var_mu) + (assign33750_e48897 * locals.var_mu_dn17)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * locals.var_alpha_dn17) + (6.0 * locals.var_t0__blk1114_dn17)) * locals.var_mud_hoso) + (assign33750_e48908 * locals.var_mud_hoso_dn17)) * locals.var_mud_hoso) + (assign33750_e48910 * locals.var_mud_hoso_dn17)) + ((((((4.0 * locals.var_alpha_dn17) + (3.0 * locals.var_t0__blk1114_dn17)) * locals.var_mud_hoso) + (assign33750_e48921 * locals.var_mud_hoso_dn17)) * locals.var_mu) + (assign33750_e48923 * locals.var_mu_dn17))) + ((((((3.0 * locals.var_alpha_dn17) + locals.var_t0__blk1114_dn17) * locals.var_mu) + (assign33750_e48933 * locals.var_mu_dn17)) * locals.var_mu) + (assign33750_e48935 * locals.var_mu_dn17))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * locals.var_lch_dn17) * assign33750_e48945) + (assign33750_e48942 * locals.var_alpha_dn17)) * locals.var_mu_ave) + (assign33750_e48946 * locals.var_mu_ave_dn17)) * locals.var_mu_ave) + (assign33750_e48948 * locals.var_mu_ave_dn17)))) / (assign33750_e48950 * assign33750_e48950)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33750_e48953;
        locals.var_nthrml_dn0 = assign33750_e48953_d_n0;
        locals.var_nthrml_dn2 = assign33750_e48953_d_n2;
        locals.var_nthrml_dn6 = assign33750_e48953_d_n6;
        locals.var_nthrml_dn7 = assign33750_e48953_d_n7;
        locals.var_nthrml_dn10 = assign33750_e48953_d_n10;
        locals.var_nthrml_dn11 = assign33750_e48953_d_n11;
        locals.var_nthrml_dn12 = assign33750_e48953_d_n12;
        locals.var_nthrml_dn17 = assign33750_e48953_d_n17;
        locals.var_nthrml_rv = 0.0;

        let (assign33760_e48958, assign33760_e48958_d_n0, assign33760_e48958_d_n2, assign33760_e48958_d_n6, assign33760_e48958_d_n7, assign33760_e48958_d_n10, assign33760_e48958_d_n11, assign33760_e48958_d_n12, assign33760_e48958_d_n17,) = {
    if (locals.var_guard1124 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17,)
    }
};
        locals.var_nthrml = assign33760_e48958;
        locals.var_nthrml_dn0 = assign33760_e48958_d_n0;
        locals.var_nthrml_dn2 = assign33760_e48958_d_n2;
        locals.var_nthrml_dn6 = assign33760_e48958_d_n6;
        locals.var_nthrml_dn7 = assign33760_e48958_d_n7;
        locals.var_nthrml_dn10 = assign33760_e48958_d_n10;
        locals.var_nthrml_dn11 = assign33760_e48958_d_n11;
        locals.var_nthrml_dn12 = assign33760_e48958_d_n12;
        locals.var_nthrml_dn17 = assign33760_e48958_d_n17;
        locals.var_nthrml_rv = 0.0;

        let assign33770_e48972: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1127 = assign33770_e48972;
        locals.var_guard1127_rv = 0.0;

        let (assign33780_e48977, assign33780_e48977_d_n0, assign33780_e48977_d_n2, assign33780_e48977_d_n6, assign33780_e48977_d_n7, assign33780_e48977_d_n10, assign33780_e48977_d_n11, assign33780_e48977_d_n12, assign33780_e48977_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33780_e48975: f64 = (locals.var_kusail).sqrt();
        (assign33780_e48975, (locals.var_kusail_dn0 / (2.0 * assign33780_e48975)), (locals.var_kusail_dn2 / (2.0 * assign33780_e48975)), (locals.var_kusail_dn6 / (2.0 * assign33780_e48975)), (locals.var_kusail_dn7 / (2.0 * assign33780_e48975)), (locals.var_kusail_dn10 / (2.0 * assign33780_e48975)), (locals.var_kusail_dn11 / (2.0 * assign33780_e48975)), (locals.var_kusail_dn12 / (2.0 * assign33780_e48975)), (locals.var_kusail_dn17 / (2.0 * assign33780_e48975)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12, locals.var_sqrtkusail_dn17,)
    }
};
        locals.var_sqrtkusail = assign33780_e48977;
        locals.var_sqrtkusail_dn0 = assign33780_e48977_d_n0;
        locals.var_sqrtkusail_dn2 = assign33780_e48977_d_n2;
        locals.var_sqrtkusail_dn6 = assign33780_e48977_d_n6;
        locals.var_sqrtkusail_dn7 = assign33780_e48977_d_n7;
        locals.var_sqrtkusail_dn10 = assign33780_e48977_d_n10;
        locals.var_sqrtkusail_dn11 = assign33780_e48977_d_n11;
        locals.var_sqrtkusail_dn12 = assign33780_e48977_d_n12;
        locals.var_sqrtkusail_dn17 = assign33780_e48977_d_n17;
        locals.var_sqrtkusail_rv = 0.0;

        let (assign33790_e48983, assign33790_e48983_d_n0, assign33790_e48983_d_n2, assign33790_e48983_d_n6, assign33790_e48983_d_n7, assign33790_e48983_d_n10, assign33790_e48983_d_n11, assign33790_e48983_d_n12, assign33790_e48983_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33790_e48981: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign33790_e48981, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12), (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17),)
    } else {
        (locals.var_t2__blk1116, locals.var_t2__blk1116_dn0, locals.var_t2__blk1116_dn2, locals.var_t2__blk1116_dn6, locals.var_t2__blk1116_dn7, locals.var_t2__blk1116_dn10, locals.var_t2__blk1116_dn11, locals.var_t2__blk1116_dn12, locals.var_t2__blk1116_dn17,)
    }
};
        locals.var_t2__blk1116 = assign33790_e48983;
        locals.var_t2__blk1116_dn0 = assign33790_e48983_d_n0;
        locals.var_t2__blk1116_dn2 = assign33790_e48983_d_n2;
        locals.var_t2__blk1116_dn6 = assign33790_e48983_d_n6;
        locals.var_t2__blk1116_dn7 = assign33790_e48983_d_n7;
        locals.var_t2__blk1116_dn10 = assign33790_e48983_d_n10;
        locals.var_t2__blk1116_dn11 = assign33790_e48983_d_n11;
        locals.var_t2__blk1116_dn12 = assign33790_e48983_d_n12;
        locals.var_t2__blk1116_dn17 = assign33790_e48983_d_n17;
        locals.var_t2__blk1116_rv = 0.0;

        let (assign33800_e48989, assign33800_e48989_d_n0, assign33800_e48989_d_n2, assign33800_e48989_d_n6, assign33800_e48989_d_n7, assign33800_e48989_d_n10, assign33800_e48989_d_n11, assign33800_e48989_d_n12, assign33800_e48989_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33800_e48987: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign33800_e48987, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)), ((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)),)
    } else {
        (locals.var_t3__blk1117, locals.var_t3__blk1117_dn0, locals.var_t3__blk1117_dn2, locals.var_t3__blk1117_dn6, locals.var_t3__blk1117_dn7, locals.var_t3__blk1117_dn10, locals.var_t3__blk1117_dn11, locals.var_t3__blk1117_dn12, locals.var_t3__blk1117_dn17,)
    }
};
        locals.var_t3__blk1117 = assign33800_e48989;
        locals.var_t3__blk1117_dn0 = assign33800_e48989_d_n0;
        locals.var_t3__blk1117_dn2 = assign33800_e48989_d_n2;
        locals.var_t3__blk1117_dn6 = assign33800_e48989_d_n6;
        locals.var_t3__blk1117_dn7 = assign33800_e48989_d_n7;
        locals.var_t3__blk1117_dn10 = assign33800_e48989_d_n10;
        locals.var_t3__blk1117_dn11 = assign33800_e48989_d_n11;
        locals.var_t3__blk1117_dn12 = assign33800_e48989_d_n12;
        locals.var_t3__blk1117_dn17 = assign33800_e48989_d_n17;
        locals.var_t3__blk1117_rv = 0.0;

        let (assign33810_e48995, assign33810_e48995_d_n0, assign33810_e48995_d_n2, assign33810_e48995_d_n6, assign33810_e48995_d_n7, assign33810_e48995_d_n10, assign33810_e48995_d_n11, assign33810_e48995_d_n12, assign33810_e48995_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33810_e48993: f64 = (locals.var_kusail * locals.var_kusail);
        (assign33810_e48993, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)), ((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t4__blk1118, locals.var_t4__blk1118_dn0, locals.var_t4__blk1118_dn2, locals.var_t4__blk1118_dn6, locals.var_t4__blk1118_dn7, locals.var_t4__blk1118_dn10, locals.var_t4__blk1118_dn11, locals.var_t4__blk1118_dn12, locals.var_t4__blk1118_dn17,)
    }
};
        locals.var_t4__blk1118 = assign33810_e48995;
        locals.var_t4__blk1118_dn0 = assign33810_e48995_d_n0;
        locals.var_t4__blk1118_dn2 = assign33810_e48995_d_n2;
        locals.var_t4__blk1118_dn6 = assign33810_e48995_d_n6;
        locals.var_t4__blk1118_dn7 = assign33810_e48995_d_n7;
        locals.var_t4__blk1118_dn10 = assign33810_e48995_d_n10;
        locals.var_t4__blk1118_dn11 = assign33810_e48995_d_n11;
        locals.var_t4__blk1118_dn12 = assign33810_e48995_d_n12;
        locals.var_t4__blk1118_dn17 = assign33810_e48995_d_n17;
        locals.var_t4__blk1118_rv = 0.0;

        let (assign33820_e49003, assign33820_e49003_d_n0, assign33820_e49003_d_n2, assign33820_e49003_d_n6, assign33820_e49003_d_n7, assign33820_e49003_d_n10, assign33820_e49003_d_n11, assign33820_e49003_d_n12, assign33820_e49003_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33820_e48999: f64 = (42.0 * locals.var_kusai00);
        let assign33820_e49001: f64 = (assign33820_e48999 * locals.var_kusail);
        (assign33820_e49001, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn12)), (((42.0 * locals.var_kusai00_dn17) * locals.var_kusail) + (assign33820_e48999 * locals.var_kusail_dn17)),)
    } else {
        (locals.var_t5__blk1119, locals.var_t5__blk1119_dn0, locals.var_t5__blk1119_dn2, locals.var_t5__blk1119_dn6, locals.var_t5__blk1119_dn7, locals.var_t5__blk1119_dn10, locals.var_t5__blk1119_dn11, locals.var_t5__blk1119_dn12, locals.var_t5__blk1119_dn17,)
    }
};
        locals.var_t5__blk1119 = assign33820_e49003;
        locals.var_t5__blk1119_dn0 = assign33820_e49003_d_n0;
        locals.var_t5__blk1119_dn2 = assign33820_e49003_d_n2;
        locals.var_t5__blk1119_dn6 = assign33820_e49003_d_n6;
        locals.var_t5__blk1119_dn7 = assign33820_e49003_d_n7;
        locals.var_t5__blk1119_dn10 = assign33820_e49003_d_n10;
        locals.var_t5__blk1119_dn11 = assign33820_e49003_d_n11;
        locals.var_t5__blk1119_dn12 = assign33820_e49003_d_n12;
        locals.var_t5__blk1119_dn17 = assign33820_e49003_d_n17;
        locals.var_t5__blk1119_rv = 0.0;

        let (assign33830_e49013, assign33830_e49013_d_n0, assign33830_e49013_d_n2, assign33830_e49013_d_n6, assign33830_e49013_d_n7, assign33830_e49013_d_n10, assign33830_e49013_d_n11, assign33830_e49013_d_n12, assign33830_e49013_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33830_e49009: f64 = (locals.var_t3__blk1117 + locals.var_t4__blk1118);
        let assign33830_e49010: f64 = (4.0 * assign33830_e49009);
        let assign33830_e49011: f64 = (locals.var_t5__blk1119 + assign33830_e49010);
        (assign33830_e49011, (locals.var_t5__blk1119_dn0 + (4.0 * (locals.var_t3__blk1117_dn0 + locals.var_t4__blk1118_dn0))), (locals.var_t5__blk1119_dn2 + (4.0 * (locals.var_t3__blk1117_dn2 + locals.var_t4__blk1118_dn2))), (locals.var_t5__blk1119_dn6 + (4.0 * (locals.var_t3__blk1117_dn6 + locals.var_t4__blk1118_dn6))), (locals.var_t5__blk1119_dn7 + (4.0 * (locals.var_t3__blk1117_dn7 + locals.var_t4__blk1118_dn7))), (locals.var_t5__blk1119_dn10 + (4.0 * (locals.var_t3__blk1117_dn10 + locals.var_t4__blk1118_dn10))), (locals.var_t5__blk1119_dn11 + (4.0 * (locals.var_t3__blk1117_dn11 + locals.var_t4__blk1118_dn11))), (locals.var_t5__blk1119_dn12 + (4.0 * (locals.var_t3__blk1117_dn12 + locals.var_t4__blk1118_dn12))), (locals.var_t5__blk1119_dn17 + (4.0 * (locals.var_t3__blk1117_dn17 + locals.var_t4__blk1118_dn17))),)
    } else {
        (locals.var_t5__blk1119, locals.var_t5__blk1119_dn0, locals.var_t5__blk1119_dn2, locals.var_t5__blk1119_dn6, locals.var_t5__blk1119_dn7, locals.var_t5__blk1119_dn10, locals.var_t5__blk1119_dn11, locals.var_t5__blk1119_dn12, locals.var_t5__blk1119_dn17,)
    }
};
        locals.var_t5__blk1119 = assign33830_e49013;
        locals.var_t5__blk1119_dn0 = assign33830_e49013_d_n0;
        locals.var_t5__blk1119_dn2 = assign33830_e49013_d_n2;
        locals.var_t5__blk1119_dn6 = assign33830_e49013_d_n6;
        locals.var_t5__blk1119_dn7 = assign33830_e49013_d_n7;
        locals.var_t5__blk1119_dn10 = assign33830_e49013_d_n10;
        locals.var_t5__blk1119_dn11 = assign33830_e49013_d_n11;
        locals.var_t5__blk1119_dn12 = assign33830_e49013_d_n12;
        locals.var_t5__blk1119_dn17 = assign33830_e49013_d_n17;
        locals.var_t5__blk1119_rv = 0.0;

        let (assign33840_e49027, assign33840_e49027_d_n0, assign33840_e49027_d_n2, assign33840_e49027_d_n6, assign33840_e49027_d_n7, assign33840_e49027_d_n10, assign33840_e49027_d_n11, assign33840_e49027_d_n12, assign33840_e49027_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33840_e49018: f64 = (20.0 * locals.var_sqrtkusail);
        let assign33840_e49020: f64 = (assign33840_e49018 * locals.var_vgvt);
        let assign33840_e49023: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign33840_e49024: f64 = (assign33840_e49020 * assign33840_e49023);
        let assign33840_e49025: f64 = (locals.var_t5__blk1119 + assign33840_e49024);
        (assign33840_e49025, (locals.var_t5__blk1119_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn0)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5__blk1119_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn2)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5__blk1119_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn6)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5__blk1119_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn7)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5__blk1119_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn10)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5__blk1119_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn11)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5__blk1119_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn12)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))), (locals.var_t5__blk1119_dn17 + (((((20.0 * locals.var_sqrtkusail_dn17) * locals.var_vgvt) + (assign33840_e49018 * locals.var_vgvt_dn17)) * assign33840_e49023) + (assign33840_e49020 * (locals.var_kusai00_dn17 + locals.var_kusail_dn17)))),)
    } else {
        (locals.var_t5__blk1119, locals.var_t5__blk1119_dn0, locals.var_t5__blk1119_dn2, locals.var_t5__blk1119_dn6, locals.var_t5__blk1119_dn7, locals.var_t5__blk1119_dn10, locals.var_t5__blk1119_dn11, locals.var_t5__blk1119_dn12, locals.var_t5__blk1119_dn17,)
    }
};
        locals.var_t5__blk1119 = assign33840_e49027;
        locals.var_t5__blk1119_dn0 = assign33840_e49027_d_n0;
        locals.var_t5__blk1119_dn2 = assign33840_e49027_d_n2;
        locals.var_t5__blk1119_dn6 = assign33840_e49027_d_n6;
        locals.var_t5__blk1119_dn7 = assign33840_e49027_d_n7;
        locals.var_t5__blk1119_dn10 = assign33840_e49027_d_n10;
        locals.var_t5__blk1119_dn11 = assign33840_e49027_d_n11;
        locals.var_t5__blk1119_dn12 = assign33840_e49027_d_n12;
        locals.var_t5__blk1119_dn17 = assign33840_e49027_d_n17;
        locals.var_t5__blk1119_rv = 0.0;

        let (assign33850_e49033, assign33850_e49033_d_n0, assign33850_e49033_d_n2, assign33850_e49033_d_n6, assign33850_e49033_d_n7, assign33850_e49033_d_n10, assign33850_e49033_d_n11, assign33850_e49033_d_n12, assign33850_e49033_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33850_e49031: f64 = (locals.var_t2__blk1116 * locals.var_t2__blk1116);
        (assign33850_e49031, ((locals.var_t2__blk1116_dn0 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn0)), ((locals.var_t2__blk1116_dn2 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn2)), ((locals.var_t2__blk1116_dn6 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn6)), ((locals.var_t2__blk1116_dn7 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn7)), ((locals.var_t2__blk1116_dn10 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn10)), ((locals.var_t2__blk1116_dn11 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn11)), ((locals.var_t2__blk1116_dn12 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn12)), ((locals.var_t2__blk1116_dn17 * locals.var_t2__blk1116) + (locals.var_t2__blk1116 * locals.var_t2__blk1116_dn17)),)
    } else {
        (locals.var_t10w, locals.var_t10w_dn0, locals.var_t10w_dn2, locals.var_t10w_dn6, locals.var_t10w_dn7, locals.var_t10w_dn10, locals.var_t10w_dn11, locals.var_t10w_dn12, locals.var_t10w_dn17,)
    }
};
        locals.var_t10w = assign33850_e49033;
        locals.var_t10w_dn0 = assign33850_e49033_d_n0;
        locals.var_t10w_dn2 = assign33850_e49033_d_n2;
        locals.var_t10w_dn6 = assign33850_e49033_d_n6;
        locals.var_t10w_dn7 = assign33850_e49033_d_n7;
        locals.var_t10w_dn10 = assign33850_e49033_d_n10;
        locals.var_t10w_dn11 = assign33850_e49033_d_n11;
        locals.var_t10w_dn12 = assign33850_e49033_d_n12;
        locals.var_t10w_dn17 = assign33850_e49033_d_n17;
        locals.var_t10w_rv = 0.0;

        let (assign33860_e49039, assign33860_e49039_d_n0, assign33860_e49039_d_n2, assign33860_e49039_d_n6, assign33860_e49039_d_n7, assign33860_e49039_d_n10, assign33860_e49039_d_n11, assign33860_e49039_d_n12, assign33860_e49039_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33860_e49037: f64 = (locals.var_t10w * locals.var_t10w);
        (assign33860_e49037, ((locals.var_t10w_dn0 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn0)), ((locals.var_t10w_dn2 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn2)), ((locals.var_t10w_dn6 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn6)), ((locals.var_t10w_dn7 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn7)), ((locals.var_t10w_dn10 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn10)), ((locals.var_t10w_dn11 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn11)), ((locals.var_t10w_dn12 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn12)), ((locals.var_t10w_dn17 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn17)),)
    } else {
        (locals.var_t10__blk1112, locals.var_t10__blk1112_dn0, locals.var_t10__blk1112_dn2, locals.var_t10__blk1112_dn6, locals.var_t10__blk1112_dn7, locals.var_t10__blk1112_dn10, locals.var_t10__blk1112_dn11, locals.var_t10__blk1112_dn12, locals.var_t10__blk1112_dn17,)
    }
};
        locals.var_t10__blk1112 = assign33860_e49039;
        locals.var_t10__blk1112_dn0 = assign33860_e49039_d_n0;
        locals.var_t10__blk1112_dn2 = assign33860_e49039_d_n2;
        locals.var_t10__blk1112_dn6 = assign33860_e49039_d_n6;
        locals.var_t10__blk1112_dn7 = assign33860_e49039_d_n7;
        locals.var_t10__blk1112_dn10 = assign33860_e49039_d_n10;
        locals.var_t10__blk1112_dn11 = assign33860_e49039_d_n11;
        locals.var_t10__blk1112_dn12 = assign33860_e49039_d_n12;
        locals.var_t10__blk1112_dn17 = assign33860_e49039_d_n17;
        locals.var_t10__blk1112_rv = 0.0;

        let (assign33870_e49047, assign33870_e49047_d_n0, assign33870_e49047_d_n2, assign33870_e49047_d_n6, assign33870_e49047_d_n7, assign33870_e49047_d_n10, assign33870_e49047_d_n11, assign33870_e49047_d_n12, assign33870_e49047_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33870_e49044: f64 = (locals.var_t10__blk1112 * locals.var_t2__blk1116);
        let assign33870_e49045: f64 = (locals.var_t5__blk1119 / assign33870_e49044);
        (assign33870_e49045, (((locals.var_t5__blk1119_dn0 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn0 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn0)))) / (assign33870_e49044 * assign33870_e49044)), (((locals.var_t5__blk1119_dn2 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn2 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn2)))) / (assign33870_e49044 * assign33870_e49044)), (((locals.var_t5__blk1119_dn6 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn6 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn6)))) / (assign33870_e49044 * assign33870_e49044)), (((locals.var_t5__blk1119_dn7 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn7 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn7)))) / (assign33870_e49044 * assign33870_e49044)), (((locals.var_t5__blk1119_dn10 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn10 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn10)))) / (assign33870_e49044 * assign33870_e49044)), (((locals.var_t5__blk1119_dn11 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn11 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn11)))) / (assign33870_e49044 * assign33870_e49044)), (((locals.var_t5__blk1119_dn12 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn12 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn12)))) / (assign33870_e49044 * assign33870_e49044)), (((locals.var_t5__blk1119_dn17 * assign33870_e49044) - (locals.var_t5__blk1119 * ((locals.var_t10__blk1112_dn17 * locals.var_t2__blk1116) + (locals.var_t10__blk1112 * locals.var_t2__blk1116_dn17)))) / (assign33870_e49044 * assign33870_e49044)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12, locals.var_kusai_ig_dn17,)
    }
};
        locals.var_kusai_ig = assign33870_e49047;
        locals.var_kusai_ig_dn0 = assign33870_e49047_d_n0;
        locals.var_kusai_ig_dn2 = assign33870_e49047_d_n2;
        locals.var_kusai_ig_dn6 = assign33870_e49047_d_n6;
        locals.var_kusai_ig_dn7 = assign33870_e49047_d_n7;
        locals.var_kusai_ig_dn10 = assign33870_e49047_d_n10;
        locals.var_kusai_ig_dn11 = assign33870_e49047_d_n11;
        locals.var_kusai_ig_dn12 = assign33870_e49047_d_n12;
        locals.var_kusai_ig_dn17 = assign33870_e49047_d_n17;
        locals.var_kusai_ig_rv = 0.0;

        let (assign33880_e49057, assign33880_e49057_d_n0, assign33880_e49057_d_n2, assign33880_e49057_d_n6, assign33880_e49057_d_n7, assign33880_e49057_d_n10, assign33880_e49057_d_n11, assign33880_e49057_d_n12, assign33880_e49057_d_n17,) = {
    if (locals.var_guard1127 != 0.0) {
        let assign33880_e49051: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign33880_e49053: f64 = (assign33880_e49051 * locals.var_mu);
        let assign33880_e49055: f64 = (assign33880_e49053 * locals.var_c_fox);
        (assign33880_e49055, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn7)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn12) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn12)), (((((-((locals.var_weff_nf * locals.var_lch_dn17) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33880_e49051 * locals.var_mu_dn17)) * locals.var_c_fox) + (assign33880_e49053 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12, locals.var_gds0_ign_dn17,)
    }
};
        locals.var_gds0_ign = assign33880_e49057;
        locals.var_gds0_ign_dn0 = assign33880_e49057_d_n0;
        locals.var_gds0_ign_dn2 = assign33880_e49057_d_n2;
        locals.var_gds0_ign_dn6 = assign33880_e49057_d_n6;
        locals.var_gds0_ign_dn7 = assign33880_e49057_d_n7;
        locals.var_gds0_ign_dn10 = assign33880_e49057_d_n10;
        locals.var_gds0_ign_dn11 = assign33880_e49057_d_n11;
        locals.var_gds0_ign_dn12 = assign33880_e49057_d_n12;
        locals.var_gds0_ign_dn17 = assign33880_e49057_d_n17;
        locals.var_gds0_ign_rv = 0.0;

        let assign33930_e49105: f64 = (locals.var_ids + locals.var_idsibpc);
        locals.var_ids = assign33930_e49105;
        locals.var_ids_dn0 = (locals.var_ids_dn0 + locals.var_idsibpc_dn0);
        locals.var_ids_dn2 = (locals.var_ids_dn2 + locals.var_idsibpc_dn2);
        locals.var_ids_dn6 = (locals.var_ids_dn6 + locals.var_idsibpc_dn6);
        locals.var_ids_dn7 = (locals.var_ids_dn7 + locals.var_idsibpc_dn7);
        locals.var_ids_dn10 = (locals.var_ids_dn10 + locals.var_idsibpc_dn10);
        locals.var_ids_dn11 = (locals.var_ids_dn11 + locals.var_idsibpc_dn11);
        locals.var_ids_dn12 = (locals.var_ids_dn12 + locals.var_idsibpc_dn12);
        locals.var_ids_dn17 = (locals.var_ids_dn17 + locals.var_idsibpc_dn17);
        locals.var_ids_rv = 0.0;

        let assign33940_e49108: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1128 = assign33940_e49108;
        locals.var_guard1128_rv = 0.0;

        let (assign33950_e49114,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign33950_e49112: f64 = (locals.var_cbtp + locals.var_cbtn);
        (assign33950_e49112,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33950_e49114;
        locals.var_cgbe_rv = 0.0;

        let (assign33960_e49124,) = {
    if ((locals.var_guard1128 != 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign33960_e49121: f64 = (p.p168 * locals.var_lgleff);
        let assign33960_e49122: f64 = (locals.var_cgbe - assign33960_e49121);
        (assign33960_e49122,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign33960_e49124;
        locals.var_cgbe_rv = 0.0;

        let (assign33970_e49133, assign33970_e49133_d_n0, assign33970_e49133_d_n2, assign33970_e49133_d_n6, assign33970_e49133_d_n7, assign33970_e49133_d_n10, assign33970_e49133_d_n11, assign33970_e49133_d_n12, assign33970_e49133_d_n17,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign33970_e49127: f64 = (-locals.var_cgbe);
        let assign33970_e49130: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign33970_e49131: f64 = (assign33970_e49127 * assign33970_e49130);
        (assign33970_e49131, (assign33970_e49127 * (-locals.var_vbsp_dn0)), (assign33970_e49127 * (-locals.var_vbsp_dn2)), (assign33970_e49127 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33970_e49127 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33970_e49127 * (-locals.var_vbsp_dn10)), (assign33970_e49127 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33970_e49127 * (-locals.var_vbsp_dn12)), (assign33970_e49127 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign33970_e49133;
        locals.var_qgob_dn0 = assign33970_e49133_d_n0;
        locals.var_qgob_dn2 = assign33970_e49133_d_n2;
        locals.var_qgob_dn6 = assign33970_e49133_d_n6;
        locals.var_qgob_dn7 = assign33970_e49133_d_n7;
        locals.var_qgob_dn10 = assign33970_e49133_d_n10;
        locals.var_qgob_dn11 = assign33970_e49133_d_n11;
        locals.var_qgob_dn12 = assign33970_e49133_d_n12;
        locals.var_qgob_dn17 = assign33970_e49133_d_n17;
        locals.var_qgob_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_123(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33980_e49143,) = {
    if (locals.var_guard1128 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cfu,)
    }
};
        locals.var_cfu = assign33980_e49143;
        locals.var_cfu_rv = 0.0;

        let (assign33990_e49153,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign33990_e49147: f64 = (locals.var_cfu * p.p9);
        let assign33990_e49150: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
        let assign33990_e49151: f64 = (assign33990_e49147 * assign33990_e49150);
        (assign33990_e49151,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign33990_e49153;
        locals.var_cfd_rv = 0.0;

        let (assign34000_e49163,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign34000_e49157: f64 = (locals.var_cfu * p.p9);
        let assign34000_e49160: f64 = (locals.var_wgate + locals.var_uc_psbcp);
        let assign34000_e49161: f64 = (assign34000_e49157 * assign34000_e49160);
        (assign34000_e49161,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign34000_e49163;
        locals.var_cfs_rv = 0.0;

        let (assign34010_e49171, assign34010_e49171_d_n0, assign34010_e49171_d_n2, assign34010_e49171_d_n6, assign34010_e49171_d_n7, assign34010_e49171_d_n10, assign34010_e49171_d_n11, assign34010_e49171_d_n12, assign34010_e49171_d_n17,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign34010_e49168: f64 = (locals.var_vgs - locals.var_vds);
        let assign34010_e49169: f64 = (locals.var_cfd * assign34010_e49168);
        (assign34010_e49169, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign34010_e49171;
        locals.var_qfd_dn0 = assign34010_e49171_d_n0;
        locals.var_qfd_dn2 = assign34010_e49171_d_n2;
        locals.var_qfd_dn6 = assign34010_e49171_d_n6;
        locals.var_qfd_dn7 = assign34010_e49171_d_n7;
        locals.var_qfd_dn10 = assign34010_e49171_d_n10;
        locals.var_qfd_dn11 = assign34010_e49171_d_n11;
        locals.var_qfd_dn12 = assign34010_e49171_d_n12;
        locals.var_qfd_dn17 = assign34010_e49171_d_n17;
        locals.var_qfd_rv = 0.0;

        let (assign34020_e49177, assign34020_e49177_d_n6, assign34020_e49177_d_n7, assign34020_e49177_d_n11,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign34020_e49175: f64 = (locals.var_cfs * locals.var_vgs);
        (assign34020_e49175, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign34020_e49177;
        locals.var_qfs_dn6 = assign34020_e49177_d_n6;
        locals.var_qfs_dn7 = assign34020_e49177_d_n7;
        locals.var_qfs_dn11 = assign34020_e49177_d_n11;
        locals.var_qfs_rv = 0.0;

        let (assign34030_e49189, assign34030_e49189_d_n0, assign34030_e49189_d_n2, assign34030_e49189_d_n6, assign34030_e49189_d_n7, assign34030_e49189_d_n10, assign34030_e49189_d_n11, assign34030_e49189_d_n12, assign34030_e49189_d_n17,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign34030_e49181: f64 = (locals.var_cfu * p.p19);
        let assign34030_e49183: f64 = (assign34030_e49181 * p.p9);
        let assign34030_e49186: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign34030_e49187: f64 = (assign34030_e49183 * assign34030_e49186);
        (assign34030_e49187, (assign34030_e49183 * (-locals.var_vbsp_dn0)), (assign34030_e49183 * (-locals.var_vbsp_dn2)), (assign34030_e49183 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34030_e49183 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34030_e49183 * (-locals.var_vbsp_dn10)), (assign34030_e49183 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34030_e49183 * (-locals.var_vbsp_dn12)), (assign34030_e49183 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17,)
    }
};
        locals.var_qfbc = assign34030_e49189;
        locals.var_qfbc_dn0 = assign34030_e49189_d_n0;
        locals.var_qfbc_dn2 = assign34030_e49189_d_n2;
        locals.var_qfbc_dn6 = assign34030_e49189_d_n6;
        locals.var_qfbc_dn7 = assign34030_e49189_d_n7;
        locals.var_qfbc_dn10 = assign34030_e49189_d_n10;
        locals.var_qfbc_dn11 = assign34030_e49189_d_n11;
        locals.var_qfbc_dn12 = assign34030_e49189_d_n12;
        locals.var_qfbc_dn17 = assign34030_e49189_d_n17;
        locals.var_qfbc_rv = 0.0;

        let (assign34040_e49195, assign34040_e49195_d_n0, assign34040_e49195_d_n2, assign34040_e49195_d_n6, assign34040_e49195_d_n7, assign34040_e49195_d_n10, assign34040_e49195_d_n11, assign34040_e49195_d_n12, assign34040_e49195_d_n17,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign34040_e49193: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34040_e49193, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34040_e49195;
        locals.var_qgod_dn0 = assign34040_e49195_d_n0;
        locals.var_qgod_dn2 = assign34040_e49195_d_n2;
        locals.var_qgod_dn6 = assign34040_e49195_d_n6;
        locals.var_qgod_dn7 = assign34040_e49195_d_n7;
        locals.var_qgod_dn10 = assign34040_e49195_d_n10;
        locals.var_qgod_dn11 = assign34040_e49195_d_n11;
        locals.var_qgod_dn12 = assign34040_e49195_d_n12;
        locals.var_qgod_dn17 = assign34040_e49195_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign34050_e49201, assign34050_e49201_d_n0, assign34050_e49201_d_n2, assign34050_e49201_d_n6, assign34050_e49201_d_n7, assign34050_e49201_d_n10, assign34050_e49201_d_n11, assign34050_e49201_d_n12, assign34050_e49201_d_n17,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign34050_e49199: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34050_e49199, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34050_e49201;
        locals.var_qgos_dn0 = assign34050_e49201_d_n0;
        locals.var_qgos_dn2 = assign34050_e49201_d_n2;
        locals.var_qgos_dn6 = assign34050_e49201_d_n6;
        locals.var_qgos_dn7 = assign34050_e49201_d_n7;
        locals.var_qgos_dn10 = assign34050_e49201_d_n10;
        locals.var_qgos_dn11 = assign34050_e49201_d_n11;
        locals.var_qgos_dn12 = assign34050_e49201_d_n12;
        locals.var_qgos_dn17 = assign34050_e49201_d_n17;
        locals.var_qgos_rv = 0.0;

        let (assign34060_e49207, assign34060_e49207_d_n0, assign34060_e49207_d_n2, assign34060_e49207_d_n6, assign34060_e49207_d_n7, assign34060_e49207_d_n10, assign34060_e49207_d_n11, assign34060_e49207_d_n12, assign34060_e49207_d_n17,) = {
    if (locals.var_guard1128 != 0.0) {
        let assign34060_e49205: f64 = (locals.var_qgob + locals.var_qfbc);
        (assign34060_e49205, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34060_e49207;
        locals.var_qgob_dn0 = assign34060_e49207_d_n0;
        locals.var_qgob_dn2 = assign34060_e49207_d_n2;
        locals.var_qgob_dn6 = assign34060_e49207_d_n6;
        locals.var_qgob_dn7 = assign34060_e49207_d_n7;
        locals.var_qgob_dn10 = assign34060_e49207_d_n10;
        locals.var_qgob_dn11 = assign34060_e49207_d_n11;
        locals.var_qgob_dn12 = assign34060_e49207_d_n12;
        locals.var_qgob_dn17 = assign34060_e49207_d_n17;
        locals.var_qgob_rv = 0.0;

        let (assign34070_e49217,) = {
    if ((locals.var_guard1128 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34070_e49213: f64 = (-p.p168);
        let assign34070_e49215: f64 = (assign34070_e49213 * locals.var_lgleff);
        (assign34070_e49215,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34070_e49217;
        locals.var_cgbe_rv = 0.0;

        let (assign34080_e49229, assign34080_e49229_d_n0, assign34080_e49229_d_n2, assign34080_e49229_d_n6, assign34080_e49229_d_n7, assign34080_e49229_d_n10, assign34080_e49229_d_n11, assign34080_e49229_d_n12, assign34080_e49229_d_n17,) = {
    if ((locals.var_guard1128 == 0.0) && (locals.var_cgbo_given != 0.0)) {
        let assign34080_e49223: f64 = (-locals.var_cgbe);
        let assign34080_e49226: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign34080_e49227: f64 = (assign34080_e49223 * assign34080_e49226);
        (assign34080_e49227, (assign34080_e49223 * (-locals.var_vbsp_dn0)), (assign34080_e49223 * (-locals.var_vbsp_dn2)), (assign34080_e49223 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34080_e49223 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34080_e49223 * (-locals.var_vbsp_dn10)), (assign34080_e49223 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34080_e49223 * (-locals.var_vbsp_dn12)), (assign34080_e49223 * (-locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34080_e49229;
        locals.var_qgob_dn0 = assign34080_e49229_d_n0;
        locals.var_qgob_dn2 = assign34080_e49229_d_n2;
        locals.var_qgob_dn6 = assign34080_e49229_d_n6;
        locals.var_qgob_dn7 = assign34080_e49229_d_n7;
        locals.var_qgob_dn10 = assign34080_e49229_d_n10;
        locals.var_qgob_dn11 = assign34080_e49229_d_n11;
        locals.var_qgob_dn12 = assign34080_e49229_d_n12;
        locals.var_qgob_dn17 = assign34080_e49229_d_n17;
        locals.var_qgob_rv = 0.0;

        let (assign34090_e49237,) = {
    if ((locals.var_guard1128 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign34090_e49237;
        locals.var_cgbe_rv = 0.0;

        let (assign34100_e49245, assign34100_e49245_d_n0, assign34100_e49245_d_n2, assign34100_e49245_d_n6, assign34100_e49245_d_n7, assign34100_e49245_d_n10, assign34100_e49245_d_n11, assign34100_e49245_d_n12, assign34100_e49245_d_n17,) = {
    if ((locals.var_guard1128 == 0.0) && (locals.var_cgbo_given == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17,)
    }
};
        locals.var_qgob = assign34100_e49245;
        locals.var_qgob_dn0 = assign34100_e49245_d_n0;
        locals.var_qgob_dn2 = assign34100_e49245_d_n2;
        locals.var_qgob_dn6 = assign34100_e49245_d_n6;
        locals.var_qgob_dn7 = assign34100_e49245_d_n7;
        locals.var_qgob_dn10 = assign34100_e49245_d_n10;
        locals.var_qgob_dn11 = assign34100_e49245_d_n11;
        locals.var_qgob_dn12 = assign34100_e49245_d_n12;
        locals.var_qgob_dn17 = assign34100_e49245_d_n17;
        locals.var_qgob_rv = 0.0;

        let (assign34110_e49260,) = {
    if (locals.var_guard1128 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cf,)
    }
};
        locals.var_cf = assign34110_e49260;
        locals.var_cf_rv = 0.0;

        let (assign34120_e49265,) = {
    if (locals.var_guard1128 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfd,)
    }
};
        locals.var_cfd = assign34120_e49265;
        locals.var_cfd_rv = 0.0;

        let (assign34130_e49270,) = {
    if (locals.var_guard1128 == 0.0) {
        (locals.var_cf,)
    } else {
        (locals.var_cfs,)
    }
};
        locals.var_cfs = assign34130_e49270;
        locals.var_cfs_rv = 0.0;

        let (assign34140_e49279, assign34140_e49279_d_n0, assign34140_e49279_d_n2, assign34140_e49279_d_n6, assign34140_e49279_d_n7, assign34140_e49279_d_n10, assign34140_e49279_d_n11, assign34140_e49279_d_n12, assign34140_e49279_d_n17,) = {
    if (locals.var_guard1128 == 0.0) {
        let assign34140_e49276: f64 = (locals.var_vgs - locals.var_vds);
        let assign34140_e49277: f64 = (locals.var_cfd * assign34140_e49276);
        (assign34140_e49277, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17,)
    }
};
        locals.var_qfd = assign34140_e49279;
        locals.var_qfd_dn0 = assign34140_e49279_d_n0;
        locals.var_qfd_dn2 = assign34140_e49279_d_n2;
        locals.var_qfd_dn6 = assign34140_e49279_d_n6;
        locals.var_qfd_dn7 = assign34140_e49279_d_n7;
        locals.var_qfd_dn10 = assign34140_e49279_d_n10;
        locals.var_qfd_dn11 = assign34140_e49279_d_n11;
        locals.var_qfd_dn12 = assign34140_e49279_d_n12;
        locals.var_qfd_dn17 = assign34140_e49279_d_n17;
        locals.var_qfd_rv = 0.0;

        let (assign34150_e49286, assign34150_e49286_d_n6, assign34150_e49286_d_n7, assign34150_e49286_d_n11,) = {
    if (locals.var_guard1128 == 0.0) {
        let assign34150_e49284: f64 = (locals.var_cfs * locals.var_vgs);
        (assign34150_e49284, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11),)
    } else {
        (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11,)
    }
};
        locals.var_qfs = assign34150_e49286;
        locals.var_qfs_dn6 = assign34150_e49286_d_n6;
        locals.var_qfs_dn7 = assign34150_e49286_d_n7;
        locals.var_qfs_dn11 = assign34150_e49286_d_n11;
        locals.var_qfs_rv = 0.0;

        let (assign34160_e49293, assign34160_e49293_d_n0, assign34160_e49293_d_n2, assign34160_e49293_d_n6, assign34160_e49293_d_n7, assign34160_e49293_d_n10, assign34160_e49293_d_n11, assign34160_e49293_d_n12, assign34160_e49293_d_n17,) = {
    if (locals.var_guard1128 == 0.0) {
        let assign34160_e49291: f64 = (locals.var_qgod + locals.var_qfd);
        (assign34160_e49291, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign34160_e49293;
        locals.var_qgod_dn0 = assign34160_e49293_d_n0;
        locals.var_qgod_dn2 = assign34160_e49293_d_n2;
        locals.var_qgod_dn6 = assign34160_e49293_d_n6;
        locals.var_qgod_dn7 = assign34160_e49293_d_n7;
        locals.var_qgod_dn10 = assign34160_e49293_d_n10;
        locals.var_qgod_dn11 = assign34160_e49293_d_n11;
        locals.var_qgod_dn12 = assign34160_e49293_d_n12;
        locals.var_qgod_dn17 = assign34160_e49293_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign34170_e49300, assign34170_e49300_d_n0, assign34170_e49300_d_n2, assign34170_e49300_d_n6, assign34170_e49300_d_n7, assign34170_e49300_d_n10, assign34170_e49300_d_n11, assign34170_e49300_d_n12, assign34170_e49300_d_n17,) = {
    if (locals.var_guard1128 == 0.0) {
        let assign34170_e49298: f64 = (locals.var_qgos + locals.var_qfs);
        (assign34170_e49298, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17,)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign34170_e49300;
        locals.var_qgos_dn0 = assign34170_e49300_d_n0;
        locals.var_qgos_dn2 = assign34170_e49300_d_n2;
        locals.var_qgos_dn6 = assign34170_e49300_d_n6;
        locals.var_qgos_dn7 = assign34170_e49300_d_n7;
        locals.var_qgos_dn10 = assign34170_e49300_d_n10;
        locals.var_qgos_dn11 = assign34170_e49300_d_n11;
        locals.var_qgos_dn12 = assign34170_e49300_d_n12;
        locals.var_qgos_dn17 = assign34170_e49300_d_n17;
        locals.var_qgos_rv = 0.0;

        let assign34180_e49303: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign34180_e49303;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn12 = (locals.var_mfactor * locals.var_ids_dn12);
        locals.var_idse_dn17 = (locals.var_mfactor * locals.var_ids_dn17);
        locals.var_idse_rv = 0.0;

        let (assign34190_e49307, assign34190_e49307_d_n0, assign34190_e49307_d_n2, assign34190_e49307_d_n6, assign34190_e49307_d_n7, assign34190_e49307_d_n10, assign34190_e49307_d_n11, assign34190_e49307_d_n12, assign34190_e49307_d_n13, assign34190_e49307_d_n15, assign34190_e49307_d_n16, assign34190_e49307_d_n17, assign34190_e49307_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34190_e49307;
        locals.var_qde_dn0 = assign34190_e49307_d_n0;
        locals.var_qde_dn2 = assign34190_e49307_d_n2;
        locals.var_qde_dn6 = assign34190_e49307_d_n6;
        locals.var_qde_dn7 = assign34190_e49307_d_n7;
        locals.var_qde_dn10 = assign34190_e49307_d_n10;
        locals.var_qde_dn11 = assign34190_e49307_d_n11;
        locals.var_qde_dn12 = assign34190_e49307_d_n12;
        locals.var_qde_dn13 = assign34190_e49307_d_n13;
        locals.var_qde_dn15 = assign34190_e49307_d_n15;
        locals.var_qde_dn16 = assign34190_e49307_d_n16;
        locals.var_qde_dn17 = assign34190_e49307_d_n17;
        locals.var_qde_dn18 = assign34190_e49307_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34200_e49311, assign34200_e49311_d_n0, assign34200_e49311_d_n2, assign34200_e49311_d_n6, assign34200_e49311_d_n7, assign34200_e49311_d_n10, assign34200_e49311_d_n11, assign34200_e49311_d_n12, assign34200_e49311_d_n13, assign34200_e49311_d_n15, assign34200_e49311_d_n16, assign34200_e49311_d_n17, assign34200_e49311_d_n18,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34200_e49311;
        locals.var_qge_dn0 = assign34200_e49311_d_n0;
        locals.var_qge_dn2 = assign34200_e49311_d_n2;
        locals.var_qge_dn6 = assign34200_e49311_d_n6;
        locals.var_qge_dn7 = assign34200_e49311_d_n7;
        locals.var_qge_dn10 = assign34200_e49311_d_n10;
        locals.var_qge_dn11 = assign34200_e49311_d_n11;
        locals.var_qge_dn12 = assign34200_e49311_d_n12;
        locals.var_qge_dn13 = assign34200_e49311_d_n13;
        locals.var_qge_dn15 = assign34200_e49311_d_n15;
        locals.var_qge_dn16 = assign34200_e49311_d_n16;
        locals.var_qge_dn17 = assign34200_e49311_d_n17;
        locals.var_qge_dn18 = assign34200_e49311_d_n18;
        locals.var_qge_rv = 0.0;

        let assign34210_e49314: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1129 = assign34210_e49314;
        locals.var_guard1129_rv = 0.0;

        let (assign34220_e49320, assign34220_e49320_d_n0, assign34220_e49320_d_n2, assign34220_e49320_d_n6, assign34220_e49320_d_n7, assign34220_e49320_d_n10, assign34220_e49320_d_n11, assign34220_e49320_d_n12, assign34220_e49320_d_n13, assign34220_e49320_d_n15, assign34220_e49320_d_n16, assign34220_e49320_d_n17, assign34220_e49320_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1129 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34220_e49320;
        locals.var_qse_dn0 = assign34220_e49320_d_n0;
        locals.var_qse_dn2 = assign34220_e49320_d_n2;
        locals.var_qse_dn6 = assign34220_e49320_d_n6;
        locals.var_qse_dn7 = assign34220_e49320_d_n7;
        locals.var_qse_dn10 = assign34220_e49320_d_n10;
        locals.var_qse_dn11 = assign34220_e49320_d_n11;
        locals.var_qse_dn12 = assign34220_e49320_d_n12;
        locals.var_qse_dn13 = assign34220_e49320_d_n13;
        locals.var_qse_dn15 = assign34220_e49320_d_n15;
        locals.var_qse_dn16 = assign34220_e49320_d_n16;
        locals.var_qse_dn17 = assign34220_e49320_d_n17;
        locals.var_qse_dn18 = assign34220_e49320_d_n18;
        locals.var_qse_rv = 0.0;

        let (assign34230_e49326, assign34230_e49326_d_n0, assign34230_e49326_d_n2, assign34230_e49326_d_n6, assign34230_e49326_d_n7, assign34230_e49326_d_n10, assign34230_e49326_d_n11, assign34230_e49326_d_n12, assign34230_e49326_d_n17,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1129 != 0.0)) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    }
};
        locals.var_xd = assign34230_e49326;
        locals.var_xd_dn0 = assign34230_e49326_d_n0;
        locals.var_xd_dn2 = assign34230_e49326_d_n2;
        locals.var_xd_dn6 = assign34230_e49326_d_n6;
        locals.var_xd_dn7 = assign34230_e49326_d_n7;
        locals.var_xd_dn10 = assign34230_e49326_d_n10;
        locals.var_xd_dn11 = assign34230_e49326_d_n11;
        locals.var_xd_dn12 = assign34230_e49326_d_n12;
        locals.var_xd_dn17 = assign34230_e49326_d_n17;
        locals.var_xd_rv = 0.0;

        let (assign34260_e49349, assign34260_e49349_d_n0, assign34260_e49349_d_n2, assign34260_e49349_d_n6, assign34260_e49349_d_n7, assign34260_e49349_d_n10, assign34260_e49349_d_n11, assign34260_e49349_d_n12, assign34260_e49349_d_n13, assign34260_e49349_d_n15, assign34260_e49349_d_n16, assign34260_e49349_d_n17, assign34260_e49349_d_n18,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1129 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign34260_e49349;
        locals.var_qbe_dn0 = assign34260_e49349_d_n0;
        locals.var_qbe_dn2 = assign34260_e49349_d_n2;
        locals.var_qbe_dn6 = assign34260_e49349_d_n6;
        locals.var_qbe_dn7 = assign34260_e49349_d_n7;
        locals.var_qbe_dn10 = assign34260_e49349_d_n10;
        locals.var_qbe_dn11 = assign34260_e49349_d_n11;
        locals.var_qbe_dn12 = assign34260_e49349_d_n12;
        locals.var_qbe_dn13 = assign34260_e49349_d_n13;
        locals.var_qbe_dn15 = assign34260_e49349_d_n15;
        locals.var_qbe_dn16 = assign34260_e49349_d_n16;
        locals.var_qbe_dn17 = assign34260_e49349_d_n17;
        locals.var_qbe_dn18 = assign34260_e49349_d_n18;
        locals.var_qbe_rv = 0.0;

        let assign34300_e49385: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1130 = assign34300_e49385;
        locals.var_guard1130_rv = 0.0;

        let (assign34310_e49397, assign34310_e49397_d_n0, assign34310_e49397_d_n2, assign34310_e49397_d_n6, assign34310_e49397_d_n7, assign34310_e49397_d_n10, assign34310_e49397_d_n11, assign34310_e49397_d_n12, assign34310_e49397_d_n13, assign34310_e49397_d_n15, assign34310_e49397_d_n16, assign34310_e49397_d_n17, assign34310_e49397_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 != 0.0)) {
        let assign34310_e49392: f64 = (-locals.var_qb);
        let assign34310_e49394: f64 = (assign34310_e49392 - locals.var_qi);
        let assign34310_e49395: f64 = (locals.var_mfactor * assign34310_e49394);
        (assign34310_e49395, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34310_e49397;
        locals.var_qge_dn0 = assign34310_e49397_d_n0;
        locals.var_qge_dn2 = assign34310_e49397_d_n2;
        locals.var_qge_dn6 = assign34310_e49397_d_n6;
        locals.var_qge_dn7 = assign34310_e49397_d_n7;
        locals.var_qge_dn10 = assign34310_e49397_d_n10;
        locals.var_qge_dn11 = assign34310_e49397_d_n11;
        locals.var_qge_dn12 = assign34310_e49397_d_n12;
        locals.var_qge_dn13 = assign34310_e49397_d_n13;
        locals.var_qge_dn15 = assign34310_e49397_d_n15;
        locals.var_qge_dn16 = assign34310_e49397_d_n16;
        locals.var_qge_dn17 = assign34310_e49397_d_n17;
        locals.var_qge_dn18 = assign34310_e49397_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34320_e49406, assign34320_e49406_d_n0, assign34320_e49406_d_n2, assign34320_e49406_d_n6, assign34320_e49406_d_n7, assign34320_e49406_d_n10, assign34320_e49406_d_n11, assign34320_e49406_d_n12, assign34320_e49406_d_n13, assign34320_e49406_d_n15, assign34320_e49406_d_n16, assign34320_e49406_d_n17, assign34320_e49406_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 != 0.0)) {
        let assign34320_e49404: f64 = (locals.var_mfactor * locals.var_qd);
        (assign34320_e49404, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34320_e49406;
        locals.var_qde_dn0 = assign34320_e49406_d_n0;
        locals.var_qde_dn2 = assign34320_e49406_d_n2;
        locals.var_qde_dn6 = assign34320_e49406_d_n6;
        locals.var_qde_dn7 = assign34320_e49406_d_n7;
        locals.var_qde_dn10 = assign34320_e49406_d_n10;
        locals.var_qde_dn11 = assign34320_e49406_d_n11;
        locals.var_qde_dn12 = assign34320_e49406_d_n12;
        locals.var_qde_dn13 = assign34320_e49406_d_n13;
        locals.var_qde_dn15 = assign34320_e49406_d_n15;
        locals.var_qde_dn16 = assign34320_e49406_d_n16;
        locals.var_qde_dn17 = assign34320_e49406_d_n17;
        locals.var_qde_dn18 = assign34320_e49406_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34330_e49417, assign34330_e49417_d_n0, assign34330_e49417_d_n2, assign34330_e49417_d_n6, assign34330_e49417_d_n7, assign34330_e49417_d_n10, assign34330_e49417_d_n11, assign34330_e49417_d_n12, assign34330_e49417_d_n13, assign34330_e49417_d_n15, assign34330_e49417_d_n16, assign34330_e49417_d_n17, assign34330_e49417_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 != 0.0)) {
        let assign34330_e49414: f64 = (locals.var_qi - locals.var_qd);
        let assign34330_e49415: f64 = (locals.var_mfactor * assign34330_e49414);
        (assign34330_e49415, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34330_e49417;
        locals.var_qse_dn0 = assign34330_e49417_d_n0;
        locals.var_qse_dn2 = assign34330_e49417_d_n2;
        locals.var_qse_dn6 = assign34330_e49417_d_n6;
        locals.var_qse_dn7 = assign34330_e49417_d_n7;
        locals.var_qse_dn10 = assign34330_e49417_d_n10;
        locals.var_qse_dn11 = assign34330_e49417_d_n11;
        locals.var_qse_dn12 = assign34330_e49417_d_n12;
        locals.var_qse_dn13 = assign34330_e49417_d_n13;
        locals.var_qse_dn15 = assign34330_e49417_d_n15;
        locals.var_qse_dn16 = assign34330_e49417_d_n16;
        locals.var_qse_dn17 = assign34330_e49417_d_n17;
        locals.var_qse_dn18 = assign34330_e49417_d_n18;
        locals.var_qse_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_124(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34340_e49434, assign34340_e49434_d_n0, assign34340_e49434_d_n2, assign34340_e49434_d_n6, assign34340_e49434_d_n7, assign34340_e49434_d_n10, assign34340_e49434_d_n11, assign34340_e49434_d_n12, assign34340_e49434_d_n13, assign34340_e49434_d_n15, assign34340_e49434_d_n16, assign34340_e49434_d_n17, assign34340_e49434_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 == 0.0)) {
        let assign34340_e49425: f64 = (-locals.var_qsub);
        let assign34340_e49427: f64 = (assign34340_e49425 - locals.var_qi);
        let assign34340_e49429: f64 = (assign34340_e49427 - locals.var_qs_fb);
        let assign34340_e49431: f64 = (assign34340_e49429 - locals.var_qd_fb);
        let assign34340_e49432: f64 = (locals.var_mfactor * assign34340_e49431);
        (assign34340_e49432, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34340_e49434;
        locals.var_qge_dn0 = assign34340_e49434_d_n0;
        locals.var_qge_dn2 = assign34340_e49434_d_n2;
        locals.var_qge_dn6 = assign34340_e49434_d_n6;
        locals.var_qge_dn7 = assign34340_e49434_d_n7;
        locals.var_qge_dn10 = assign34340_e49434_d_n10;
        locals.var_qge_dn11 = assign34340_e49434_d_n11;
        locals.var_qge_dn12 = assign34340_e49434_d_n12;
        locals.var_qge_dn13 = assign34340_e49434_d_n13;
        locals.var_qge_dn15 = assign34340_e49434_d_n15;
        locals.var_qge_dn16 = assign34340_e49434_d_n16;
        locals.var_qge_dn17 = assign34340_e49434_d_n17;
        locals.var_qge_dn18 = assign34340_e49434_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34350_e49446, assign34350_e49446_d_n0, assign34350_e49446_d_n2, assign34350_e49446_d_n6, assign34350_e49446_d_n7, assign34350_e49446_d_n10, assign34350_e49446_d_n11, assign34350_e49446_d_n12, assign34350_e49446_d_n13, assign34350_e49446_d_n15, assign34350_e49446_d_n16, assign34350_e49446_d_n17, assign34350_e49446_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 == 0.0)) {
        let assign34350_e49443: f64 = (locals.var_qd + locals.var_qd_fb);
        let assign34350_e49444: f64 = (locals.var_mfactor * assign34350_e49443);
        (assign34350_e49444, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34350_e49446;
        locals.var_qde_dn0 = assign34350_e49446_d_n0;
        locals.var_qde_dn2 = assign34350_e49446_d_n2;
        locals.var_qde_dn6 = assign34350_e49446_d_n6;
        locals.var_qde_dn7 = assign34350_e49446_d_n7;
        locals.var_qde_dn10 = assign34350_e49446_d_n10;
        locals.var_qde_dn11 = assign34350_e49446_d_n11;
        locals.var_qde_dn12 = assign34350_e49446_d_n12;
        locals.var_qde_dn13 = assign34350_e49446_d_n13;
        locals.var_qde_dn15 = assign34350_e49446_d_n15;
        locals.var_qde_dn16 = assign34350_e49446_d_n16;
        locals.var_qde_dn17 = assign34350_e49446_d_n17;
        locals.var_qde_dn18 = assign34350_e49446_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34360_e49460, assign34360_e49460_d_n0, assign34360_e49460_d_n2, assign34360_e49460_d_n6, assign34360_e49460_d_n7, assign34360_e49460_d_n10, assign34360_e49460_d_n11, assign34360_e49460_d_n12, assign34360_e49460_d_n13, assign34360_e49460_d_n15, assign34360_e49460_d_n16, assign34360_e49460_d_n17, assign34360_e49460_d_n18,) = {
    if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 == 0.0)) {
        let assign34360_e49455: f64 = (locals.var_qi - locals.var_qd);
        let assign34360_e49457: f64 = (assign34360_e49455 + locals.var_qs_fb);
        let assign34360_e49458: f64 = (locals.var_mfactor * assign34360_e49457);
        (assign34360_e49458, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34360_e49460;
        locals.var_qse_dn0 = assign34360_e49460_d_n0;
        locals.var_qse_dn2 = assign34360_e49460_d_n2;
        locals.var_qse_dn6 = assign34360_e49460_d_n6;
        locals.var_qse_dn7 = assign34360_e49460_d_n7;
        locals.var_qse_dn10 = assign34360_e49460_d_n10;
        locals.var_qse_dn11 = assign34360_e49460_d_n11;
        locals.var_qse_dn12 = assign34360_e49460_d_n12;
        locals.var_qse_dn13 = assign34360_e49460_d_n13;
        locals.var_qse_dn15 = assign34360_e49460_d_n15;
        locals.var_qse_dn16 = assign34360_e49460_d_n16;
        locals.var_qse_dn17 = assign34360_e49460_d_n17;
        locals.var_qse_dn18 = assign34360_e49460_d_n18;
        locals.var_qse_rv = 0.0;

        let assign34370_e49463: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34370_e49463;
        locals.var_guard1136_rv = 0.0;

        let (assign34380_e49467, assign34380_e49467_d_n0, assign34380_e49467_d_n2, assign34380_e49467_d_n6, assign34380_e49467_d_n7, assign34380_e49467_d_n10, assign34380_e49467_d_n11, assign34380_e49467_d_n12, assign34380_e49467_d_n17,) = {
    if (locals.var_guard1136 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34380_e49467;
        locals.var_qy_dn0 = assign34380_e49467_d_n0;
        locals.var_qy_dn2 = assign34380_e49467_d_n2;
        locals.var_qy_dn6 = assign34380_e49467_d_n6;
        locals.var_qy_dn7 = assign34380_e49467_d_n7;
        locals.var_qy_dn10 = assign34380_e49467_d_n10;
        locals.var_qy_dn11 = assign34380_e49467_d_n11;
        locals.var_qy_dn12 = assign34380_e49467_d_n12;
        locals.var_qy_dn17 = assign34380_e49467_d_n17;
        locals.var_qy_rv = 0.0;

        let (assign34390_e49476, assign34390_e49476_d_n0, assign34390_e49476_d_n2, assign34390_e49476_d_n6, assign34390_e49476_d_n7, assign34390_e49476_d_n10, assign34390_e49476_d_n11, assign34390_e49476_d_n12, assign34390_e49476_d_n17,) = {
    if (locals.var_guard1136 == 0.0) {
        let assign34390_e49472: f64 = (locals.var_ec * locals.var_leff);
        let assign34390_e49474: f64 = (assign34390_e49472 + locals.var_ps0);
        (assign34390_e49474, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17,)
    }
};
        locals.var_pslk = assign34390_e49476;
        locals.var_pslk_dn0 = assign34390_e49476_d_n0;
        locals.var_pslk_dn2 = assign34390_e49476_d_n2;
        locals.var_pslk_dn6 = assign34390_e49476_d_n6;
        locals.var_pslk_dn7 = assign34390_e49476_d_n7;
        locals.var_pslk_dn10 = assign34390_e49476_d_n10;
        locals.var_pslk_dn11 = assign34390_e49476_d_n11;
        locals.var_pslk_dn12 = assign34390_e49476_d_n12;
        locals.var_pslk_dn17 = assign34390_e49476_d_n17;
        locals.var_pslk_rv = 0.0;

        let assign34400_e49479: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34400_e49479;
        locals.var_guard1137_rv = 0.0;

        let (assign34410_e49486, assign34410_e49486_d_n0, assign34410_e49486_d_n2, assign34410_e49486_d_n6, assign34410_e49486_d_n7, assign34410_e49486_d_n10, assign34410_e49486_d_n11, assign34410_e49486_d_n12, assign34410_e49486_d_n17,) = {
    if ((locals.var_guard1136 == 0.0) && (locals.var_guard1137 != 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17,)
    }
};
        locals.var_pslk = assign34410_e49486;
        locals.var_pslk_dn0 = assign34410_e49486_d_n0;
        locals.var_pslk_dn2 = assign34410_e49486_d_n2;
        locals.var_pslk_dn6 = assign34410_e49486_d_n6;
        locals.var_pslk_dn7 = assign34410_e49486_d_n7;
        locals.var_pslk_dn10 = assign34410_e49486_d_n10;
        locals.var_pslk_dn11 = assign34410_e49486_d_n11;
        locals.var_pslk_dn12 = assign34410_e49486_d_n12;
        locals.var_pslk_dn17 = assign34410_e49486_d_n17;
        locals.var_pslk_rv = 0.0;

        let (assign34420_e49501, assign34420_e49501_d_n0, assign34420_e49501_d_n2, assign34420_e49501_d_n6, assign34420_e49501_d_n7, assign34420_e49501_d_n10, assign34420_e49501_d_n11, assign34420_e49501_d_n12, assign34420_e49501_d_n17,) = {
    if (locals.var_guard1136 == 0.0) {
        let assign34420_e49492: f64 = (locals.var_vds + locals.var_ps0);
        let assign34420_e49493: f64 = (locals.var_aclm * assign34420_e49492);
        let assign34420_e49496: f64 = (1.0 - locals.var_aclm);
        let assign34420_e49498: f64 = (assign34420_e49496 * locals.var_pslk);
        let assign34420_e49499: f64 = (assign34420_e49493 + assign34420_e49498);
        (assign34420_e49499, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34420_e49496 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34420_e49496 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34420_e49496 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34420_e49496 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34420_e49496 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34420_e49496 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34420_e49496 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34420_e49496 * locals.var_pslk_dn17)),)
    } else {
        (locals.var_t1__blk1132, locals.var_t1__blk1132_dn0, locals.var_t1__blk1132_dn2, locals.var_t1__blk1132_dn6, locals.var_t1__blk1132_dn7, locals.var_t1__blk1132_dn10, locals.var_t1__blk1132_dn11, locals.var_t1__blk1132_dn12, locals.var_t1__blk1132_dn17,)
    }
};
        locals.var_t1__blk1132 = assign34420_e49501;
        locals.var_t1__blk1132_dn0 = assign34420_e49501_d_n0;
        locals.var_t1__blk1132_dn2 = assign34420_e49501_d_n2;
        locals.var_t1__blk1132_dn6 = assign34420_e49501_d_n6;
        locals.var_t1__blk1132_dn7 = assign34420_e49501_d_n7;
        locals.var_t1__blk1132_dn10 = assign34420_e49501_d_n10;
        locals.var_t1__blk1132_dn11 = assign34420_e49501_d_n11;
        locals.var_t1__blk1132_dn12 = assign34420_e49501_d_n12;
        locals.var_t1__blk1132_dn17 = assign34420_e49501_d_n17;
        locals.var_t1__blk1132_rv = 0.0;

        let (assign34430_e49511, assign34430_e49511_d_n0, assign34430_e49511_d_n2, assign34430_e49511_d_n6, assign34430_e49511_d_n7, assign34430_e49511_d_n10, assign34430_e49511_d_n11, assign34430_e49511_d_n12, assign34430_e49511_d_n17,) = {
    if (locals.var_guard1136 == 0.0) {
        let assign34430_e49506: f64 = (2.0 * 1.034943e-10);
        let assign34430_e49508: f64 = (assign34430_e49506 / locals.var_q_nsub);
        let assign34430_e49509: f64 = (assign34430_e49508).sqrt();
        (assign34430_e49509, ((-((assign34430_e49506 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)),)
    } else {
        (locals.var_t10__blk1133, locals.var_t10__blk1133_dn0, locals.var_t10__blk1133_dn2, locals.var_t10__blk1133_dn6, locals.var_t10__blk1133_dn7, locals.var_t10__blk1133_dn10, locals.var_t10__blk1133_dn11, locals.var_t10__blk1133_dn12, locals.var_t10__blk1133_dn17,)
    }
};
        locals.var_t10__blk1133 = assign34430_e49511;
        locals.var_t10__blk1133_dn0 = assign34430_e49511_d_n0;
        locals.var_t10__blk1133_dn2 = assign34430_e49511_d_n2;
        locals.var_t10__blk1133_dn6 = assign34430_e49511_d_n6;
        locals.var_t10__blk1133_dn7 = assign34430_e49511_d_n7;
        locals.var_t10__blk1133_dn10 = assign34430_e49511_d_n10;
        locals.var_t10__blk1133_dn11 = assign34430_e49511_d_n11;
        locals.var_t10__blk1133_dn12 = assign34430_e49511_d_n12;
        locals.var_t10__blk1133_dn17 = assign34430_e49511_d_n17;
        locals.var_t10__blk1133_rv = 0.0;

        let (assign34440_e49518, assign34440_e49518_d_n0, assign34440_e49518_d_n2, assign34440_e49518_d_n6, assign34440_e49518_d_n7, assign34440_e49518_d_n10, assign34440_e49518_d_n11, assign34440_e49518_d_n12, assign34440_e49518_d_n17,) = {
    if (locals.var_guard1136 == 0.0) {
        let assign34440_e49516: f64 = (locals.var_t10__blk1133 * 1.3);
        (assign34440_e49516, (locals.var_t10__blk1133_dn0 * 1.3), (locals.var_t10__blk1133_dn2 * 1.3), (locals.var_t10__blk1133_dn6 * 1.3), (locals.var_t10__blk1133_dn7 * 1.3), (locals.var_t10__blk1133_dn10 * 1.3), (locals.var_t10__blk1133_dn11 * 1.3), (locals.var_t10__blk1133_dn12 * 1.3), (locals.var_t10__blk1133_dn17 * 1.3),)
    } else {
        (locals.var_t3__blk1134, locals.var_t3__blk1134_dn0, locals.var_t3__blk1134_dn2, locals.var_t3__blk1134_dn6, locals.var_t3__blk1134_dn7, locals.var_t3__blk1134_dn10, locals.var_t3__blk1134_dn11, locals.var_t3__blk1134_dn12, locals.var_t3__blk1134_dn17,)
    }
};
        locals.var_t3__blk1134 = assign34440_e49518;
        locals.var_t3__blk1134_dn0 = assign34440_e49518_d_n0;
        locals.var_t3__blk1134_dn2 = assign34440_e49518_d_n2;
        locals.var_t3__blk1134_dn6 = assign34440_e49518_d_n6;
        locals.var_t3__blk1134_dn7 = assign34440_e49518_d_n7;
        locals.var_t3__blk1134_dn10 = assign34440_e49518_d_n10;
        locals.var_t3__blk1134_dn11 = assign34440_e49518_d_n11;
        locals.var_t3__blk1134_dn12 = assign34440_e49518_d_n12;
        locals.var_t3__blk1134_dn17 = assign34440_e49518_d_n17;
        locals.var_t3__blk1134_rv = 0.0;

        let (assign34450_e49527, assign34450_e49527_d_n0, assign34450_e49527_d_n2, assign34450_e49527_d_n6, assign34450_e49527_d_n7, assign34450_e49527_d_n10, assign34450_e49527_d_n11, assign34450_e49527_d_n12, assign34450_e49527_d_n17,) = {
    if (locals.var_guard1136 == 0.0) {
        let assign34450_e49523: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign34450_e49525: f64 = (assign34450_e49523 * locals.var_t3__blk1134);
        (assign34450_e49525, (assign34450_e49523 * locals.var_t3__blk1134_dn0), (assign34450_e49523 * locals.var_t3__blk1134_dn2), (assign34450_e49523 * locals.var_t3__blk1134_dn6), (assign34450_e49523 * locals.var_t3__blk1134_dn7), (assign34450_e49523 * locals.var_t3__blk1134_dn10), (assign34450_e49523 * locals.var_t3__blk1134_dn11), (assign34450_e49523 * locals.var_t3__blk1134_dn12), (assign34450_e49523 * locals.var_t3__blk1134_dn17),)
    } else {
        (locals.var_t2__blk1135, locals.var_t2__blk1135_dn0, locals.var_t2__blk1135_dn2, locals.var_t2__blk1135_dn6, locals.var_t2__blk1135_dn7, locals.var_t2__blk1135_dn10, locals.var_t2__blk1135_dn11, locals.var_t2__blk1135_dn12, locals.var_t2__blk1135_dn17,)
    }
};
        locals.var_t2__blk1135 = assign34450_e49527;
        locals.var_t2__blk1135_dn0 = assign34450_e49527_d_n0;
        locals.var_t2__blk1135_dn2 = assign34450_e49527_d_n2;
        locals.var_t2__blk1135_dn6 = assign34450_e49527_d_n6;
        locals.var_t2__blk1135_dn7 = assign34450_e49527_d_n7;
        locals.var_t2__blk1135_dn10 = assign34450_e49527_d_n10;
        locals.var_t2__blk1135_dn11 = assign34450_e49527_d_n11;
        locals.var_t2__blk1135_dn12 = assign34450_e49527_d_n12;
        locals.var_t2__blk1135_dn17 = assign34450_e49527_d_n17;
        locals.var_t2__blk1135_rv = 0.0;

        let (assign34460_e49542, assign34460_e49542_d_n0, assign34460_e49542_d_n2, assign34460_e49542_d_n6, assign34460_e49542_d_n7, assign34460_e49542_d_n10, assign34460_e49542_d_n11, assign34460_e49542_d_n12, assign34460_e49542_d_n17,) = {
    if (locals.var_guard1136 == 0.0) {
        let assign34460_e49532: f64 = (locals.var_ps0 + locals.var_vds);
        let assign34460_e49534: f64 = (assign34460_e49532 - locals.var_t1__blk1132);
        let assign34460_e49536: f64 = (assign34460_e49534 / p.p64);
        let assign34460_e49538: f64 = (assign34460_e49536 - locals.var_ec);
        let assign34460_e49540: f64 = (assign34460_e49538 * locals.var_t2__blk1135);
        (assign34460_e49540, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1132_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1132_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1132_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1132_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1132_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1132_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1132_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1132_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34460_e49542;
        locals.var_qy_dn0 = assign34460_e49542_d_n0;
        locals.var_qy_dn2 = assign34460_e49542_d_n2;
        locals.var_qy_dn6 = assign34460_e49542_d_n6;
        locals.var_qy_dn7 = assign34460_e49542_d_n7;
        locals.var_qy_dn10 = assign34460_e49542_d_n10;
        locals.var_qy_dn11 = assign34460_e49542_d_n11;
        locals.var_qy_dn12 = assign34460_e49542_d_n12;
        locals.var_qy_dn17 = assign34460_e49542_d_n17;
        locals.var_qy_rv = 0.0;

        let assign34470_e49545: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34470_e49545;
        locals.var_guard1138_rv = 0.0;

        let (assign34480_e49553, assign34480_e49553_d_n0, assign34480_e49553_d_n2, assign34480_e49553_d_n6, assign34480_e49553_d_n7, assign34480_e49553_d_n10, assign34480_e49553_d_n11, assign34480_e49553_d_n12, assign34480_e49553_d_n17,) = {
    if (locals.var_guard1138 != 0.0) {
        let assign34480_e49550: f64 = (locals.var_cqyb0 * locals.var_vbsp);
        let assign34480_e49551: f64 = (locals.var_qy + assign34480_e49550);
        (assign34480_e49551, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17,)
    }
};
        locals.var_qy = assign34480_e49553;
        locals.var_qy_dn0 = assign34480_e49553_d_n0;
        locals.var_qy_dn2 = assign34480_e49553_d_n2;
        locals.var_qy_dn6 = assign34480_e49553_d_n6;
        locals.var_qy_dn7 = assign34480_e49553_d_n7;
        locals.var_qy_dn10 = assign34480_e49553_d_n10;
        locals.var_qy_dn11 = assign34480_e49553_d_n11;
        locals.var_qy_dn12 = assign34480_e49553_d_n12;
        locals.var_qy_dn17 = assign34480_e49553_d_n17;
        locals.var_qy_rv = 0.0;

        let assign34490_e49556: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1139 = assign34490_e49556;
        locals.var_guard1139_rv = 0.0;

        let assign34500_e49559: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1140 = assign34500_e49559;
        locals.var_guard1140_rv = 0.0;

        let (assign34510_e49572, assign34510_e49572_d_n0, assign34510_e49572_d_n2, assign34510_e49572_d_n6, assign34510_e49572_d_n7, assign34510_e49572_d_n10, assign34510_e49572_d_n11, assign34510_e49572_d_n12, assign34510_e49572_d_n17,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
        let assign34510_e49564: f64 = (-locals.var_qbody_bt_p_sus);
        let assign34510_e49566: f64 = (assign34510_e49564 - locals.var_qbody_bt_p_sud);
        let assign34510_e49568: f64 = (assign34510_e49566 - locals.var_qbody_bt_n_sus);
        let assign34510_e49570: f64 = (assign34510_e49568 - locals.var_qbody_bt_n_sud);
        (assign34510_e49570, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17,)
    }
};
        locals.var_q_bt_ge = assign34510_e49572;
        locals.var_q_bt_ge_dn0 = assign34510_e49572_d_n0;
        locals.var_q_bt_ge_dn2 = assign34510_e49572_d_n2;
        locals.var_q_bt_ge_dn6 = assign34510_e49572_d_n6;
        locals.var_q_bt_ge_dn7 = assign34510_e49572_d_n7;
        locals.var_q_bt_ge_dn10 = assign34510_e49572_d_n10;
        locals.var_q_bt_ge_dn11 = assign34510_e49572_d_n11;
        locals.var_q_bt_ge_dn12 = assign34510_e49572_d_n12;
        locals.var_q_bt_ge_dn17 = assign34510_e49572_d_n17;
        locals.var_q_bt_ge_rv = 0.0;

        let (assign34520_e49580, assign34520_e49580_d_n0, assign34520_e49580_d_n2, assign34520_e49580_d_n6, assign34520_e49580_d_n7, assign34520_e49580_d_n10, assign34520_e49580_d_n11, assign34520_e49580_d_n12, assign34520_e49580_d_n17,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
        let assign34520_e49578: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
        (assign34520_e49578, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17),)
    } else {
        (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17,)
    }
};
        locals.var_q_bt_de = assign34520_e49580;
        locals.var_q_bt_de_dn0 = assign34520_e49580_d_n0;
        locals.var_q_bt_de_dn2 = assign34520_e49580_d_n2;
        locals.var_q_bt_de_dn6 = assign34520_e49580_d_n6;
        locals.var_q_bt_de_dn7 = assign34520_e49580_d_n7;
        locals.var_q_bt_de_dn10 = assign34520_e49580_d_n10;
        locals.var_q_bt_de_dn11 = assign34520_e49580_d_n11;
        locals.var_q_bt_de_dn12 = assign34520_e49580_d_n12;
        locals.var_q_bt_de_dn17 = assign34520_e49580_d_n17;
        locals.var_q_bt_de_rv = 0.0;

        let (assign34530_e49588, assign34530_e49588_d_n0, assign34530_e49588_d_n2, assign34530_e49588_d_n6, assign34530_e49588_d_n7, assign34530_e49588_d_n10, assign34530_e49588_d_n11, assign34530_e49588_d_n12, assign34530_e49588_d_n17,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
        let assign34530_e49586: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
        (assign34530_e49586, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17),)
    } else {
        (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17,)
    }
};
        locals.var_q_bt_se = assign34530_e49588;
        locals.var_q_bt_se_dn0 = assign34530_e49588_d_n0;
        locals.var_q_bt_se_dn2 = assign34530_e49588_d_n2;
        locals.var_q_bt_se_dn6 = assign34530_e49588_d_n6;
        locals.var_q_bt_se_dn7 = assign34530_e49588_d_n7;
        locals.var_q_bt_se_dn10 = assign34530_e49588_d_n10;
        locals.var_q_bt_se_dn11 = assign34530_e49588_d_n11;
        locals.var_q_bt_se_dn12 = assign34530_e49588_d_n12;
        locals.var_q_bt_se_dn17 = assign34530_e49588_d_n17;
        locals.var_q_bt_se_rv = 0.0;

        let (assign34540_e49610, assign34540_e49610_d_n0, assign34540_e49610_d_n2, assign34540_e49610_d_n6, assign34540_e49610_d_n7, assign34540_e49610_d_n10, assign34540_e49610_d_n11, assign34540_e49610_d_n12, assign34540_e49610_d_n13, assign34540_e49610_d_n15, assign34540_e49610_d_n16, assign34540_e49610_d_n17, assign34540_e49610_d_n18,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
        let assign34540_e49596: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34540_e49598: f64 = (assign34540_e49596 + locals.var_qgob);
        let assign34540_e49600: f64 = (assign34540_e49598 - locals.var_qy);
        let assign34540_e49602: f64 = (assign34540_e49600 - locals.var_qovs);
        let assign34540_e49604: f64 = (assign34540_e49602 - locals.var_qovd);
        let assign34540_e49606: f64 = (assign34540_e49604 + locals.var_q_bt_ge);
        let assign34540_e49607: f64 = (locals.var_mfactor * assign34540_e49606);
        let assign34540_e49608: f64 = (locals.var_qge + assign34540_e49607);
        (assign34540_e49608, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34540_e49610;
        locals.var_qge_dn0 = assign34540_e49610_d_n0;
        locals.var_qge_dn2 = assign34540_e49610_d_n2;
        locals.var_qge_dn6 = assign34540_e49610_d_n6;
        locals.var_qge_dn7 = assign34540_e49610_d_n7;
        locals.var_qge_dn10 = assign34540_e49610_d_n10;
        locals.var_qge_dn11 = assign34540_e49610_d_n11;
        locals.var_qge_dn12 = assign34540_e49610_d_n12;
        locals.var_qge_dn13 = assign34540_e49610_d_n13;
        locals.var_qge_dn15 = assign34540_e49610_d_n15;
        locals.var_qge_dn16 = assign34540_e49610_d_n16;
        locals.var_qge_dn17 = assign34540_e49610_d_n17;
        locals.var_qge_dn18 = assign34540_e49610_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34550_e49627, assign34550_e49627_d_n0, assign34550_e49627_d_n2, assign34550_e49627_d_n6, assign34550_e49627_d_n7, assign34550_e49627_d_n10, assign34550_e49627_d_n11, assign34550_e49627_d_n12, assign34550_e49627_d_n13, assign34550_e49627_d_n15, assign34550_e49627_d_n16, assign34550_e49627_d_n17, assign34550_e49627_d_n18,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
        let assign34550_e49617: f64 = (-locals.var_qgod);
        let assign34550_e49619: f64 = (assign34550_e49617 + locals.var_qy);
        let assign34550_e49621: f64 = (assign34550_e49619 + locals.var_qbdld);
        let assign34550_e49623: f64 = (assign34550_e49621 + locals.var_q_bt_de);
        let assign34550_e49624: f64 = (locals.var_mfactor * assign34550_e49623);
        let assign34550_e49625: f64 = (locals.var_qde + assign34550_e49624);
        (assign34550_e49625, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34550_e49627;
        locals.var_qde_dn0 = assign34550_e49627_d_n0;
        locals.var_qde_dn2 = assign34550_e49627_d_n2;
        locals.var_qde_dn6 = assign34550_e49627_d_n6;
        locals.var_qde_dn7 = assign34550_e49627_d_n7;
        locals.var_qde_dn10 = assign34550_e49627_d_n10;
        locals.var_qde_dn11 = assign34550_e49627_d_n11;
        locals.var_qde_dn12 = assign34550_e49627_d_n12;
        locals.var_qde_dn13 = assign34550_e49627_d_n13;
        locals.var_qde_dn15 = assign34550_e49627_d_n15;
        locals.var_qde_dn16 = assign34550_e49627_d_n16;
        locals.var_qde_dn17 = assign34550_e49627_d_n17;
        locals.var_qde_dn18 = assign34550_e49627_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34560_e49642, assign34560_e49642_d_n0, assign34560_e49642_d_n2, assign34560_e49642_d_n6, assign34560_e49642_d_n7, assign34560_e49642_d_n10, assign34560_e49642_d_n11, assign34560_e49642_d_n12, assign34560_e49642_d_n13, assign34560_e49642_d_n15, assign34560_e49642_d_n16, assign34560_e49642_d_n17, assign34560_e49642_d_n18,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
        let assign34560_e49634: f64 = (-locals.var_qgos);
        let assign34560_e49636: f64 = (assign34560_e49634 + locals.var_qbsld);
        let assign34560_e49638: f64 = (assign34560_e49636 + locals.var_q_bt_se);
        let assign34560_e49639: f64 = (locals.var_mfactor * assign34560_e49638);
        let assign34560_e49640: f64 = (locals.var_qse + assign34560_e49639);
        (assign34560_e49640, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34560_e49642;
        locals.var_qse_dn0 = assign34560_e49642_d_n0;
        locals.var_qse_dn2 = assign34560_e49642_d_n2;
        locals.var_qse_dn6 = assign34560_e49642_d_n6;
        locals.var_qse_dn7 = assign34560_e49642_d_n7;
        locals.var_qse_dn10 = assign34560_e49642_d_n10;
        locals.var_qse_dn11 = assign34560_e49642_d_n11;
        locals.var_qse_dn12 = assign34560_e49642_d_n12;
        locals.var_qse_dn13 = assign34560_e49642_d_n13;
        locals.var_qse_dn15 = assign34560_e49642_d_n15;
        locals.var_qse_dn16 = assign34560_e49642_d_n16;
        locals.var_qse_dn17 = assign34560_e49642_d_n17;
        locals.var_qse_dn18 = assign34560_e49642_d_n18;
        locals.var_qse_rv = 0.0;

        let (assign34570_e49663, assign34570_e49663_d_n0, assign34570_e49663_d_n2, assign34570_e49663_d_n6, assign34570_e49663_d_n7, assign34570_e49663_d_n10, assign34570_e49663_d_n11, assign34570_e49663_d_n12, assign34570_e49663_d_n13, assign34570_e49663_d_n15, assign34570_e49663_d_n16, assign34570_e49663_d_n17, assign34570_e49663_d_n18,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 == 0.0)) {
        let assign34570_e49651: f64 = (locals.var_qgod + locals.var_qgos);
        let assign34570_e49653: f64 = (assign34570_e49651 + locals.var_qgob);
        let assign34570_e49655: f64 = (assign34570_e49653 - locals.var_qy);
        let assign34570_e49657: f64 = (assign34570_e49655 - locals.var_qovs);
        let assign34570_e49659: f64 = (assign34570_e49657 - locals.var_qovd);
        let assign34570_e49660: f64 = (locals.var_mfactor * assign34570_e49659);
        let assign34570_e49661: f64 = (locals.var_qge + assign34570_e49660);
        (assign34570_e49661, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18,)
    }
};
        locals.var_qge = assign34570_e49663;
        locals.var_qge_dn0 = assign34570_e49663_d_n0;
        locals.var_qge_dn2 = assign34570_e49663_d_n2;
        locals.var_qge_dn6 = assign34570_e49663_d_n6;
        locals.var_qge_dn7 = assign34570_e49663_d_n7;
        locals.var_qge_dn10 = assign34570_e49663_d_n10;
        locals.var_qge_dn11 = assign34570_e49663_d_n11;
        locals.var_qge_dn12 = assign34570_e49663_d_n12;
        locals.var_qge_dn13 = assign34570_e49663_d_n13;
        locals.var_qge_dn15 = assign34570_e49663_d_n15;
        locals.var_qge_dn16 = assign34570_e49663_d_n16;
        locals.var_qge_dn17 = assign34570_e49663_d_n17;
        locals.var_qge_dn18 = assign34570_e49663_d_n18;
        locals.var_qge_rv = 0.0;

        let (assign34580_e49679, assign34580_e49679_d_n0, assign34580_e49679_d_n2, assign34580_e49679_d_n6, assign34580_e49679_d_n7, assign34580_e49679_d_n10, assign34580_e49679_d_n11, assign34580_e49679_d_n12, assign34580_e49679_d_n13, assign34580_e49679_d_n15, assign34580_e49679_d_n16, assign34580_e49679_d_n17, assign34580_e49679_d_n18,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 == 0.0)) {
        let assign34580_e49671: f64 = (-locals.var_qgod);
        let assign34580_e49673: f64 = (assign34580_e49671 + locals.var_qy);
        let assign34580_e49675: f64 = (assign34580_e49673 + locals.var_qbdld);
        let assign34580_e49676: f64 = (locals.var_mfactor * assign34580_e49675);
        let assign34580_e49677: f64 = (locals.var_qde + assign34580_e49676);
        (assign34580_e49677, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18,)
    }
};
        locals.var_qde = assign34580_e49679;
        locals.var_qde_dn0 = assign34580_e49679_d_n0;
        locals.var_qde_dn2 = assign34580_e49679_d_n2;
        locals.var_qde_dn6 = assign34580_e49679_d_n6;
        locals.var_qde_dn7 = assign34580_e49679_d_n7;
        locals.var_qde_dn10 = assign34580_e49679_d_n10;
        locals.var_qde_dn11 = assign34580_e49679_d_n11;
        locals.var_qde_dn12 = assign34580_e49679_d_n12;
        locals.var_qde_dn13 = assign34580_e49679_d_n13;
        locals.var_qde_dn15 = assign34580_e49679_d_n15;
        locals.var_qde_dn16 = assign34580_e49679_d_n16;
        locals.var_qde_dn17 = assign34580_e49679_d_n17;
        locals.var_qde_dn18 = assign34580_e49679_d_n18;
        locals.var_qde_rv = 0.0;

        let (assign34590_e49693, assign34590_e49693_d_n0, assign34590_e49693_d_n2, assign34590_e49693_d_n6, assign34590_e49693_d_n7, assign34590_e49693_d_n10, assign34590_e49693_d_n11, assign34590_e49693_d_n12, assign34590_e49693_d_n13, assign34590_e49693_d_n15, assign34590_e49693_d_n16, assign34590_e49693_d_n17, assign34590_e49693_d_n18,) = {
    if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 == 0.0)) {
        let assign34590_e49687: f64 = (-locals.var_qgos);
        let assign34590_e49689: f64 = (assign34590_e49687 + locals.var_qbsld);
        let assign34590_e49690: f64 = (locals.var_mfactor * assign34590_e49689);
        let assign34590_e49691: f64 = (locals.var_qse + assign34590_e49690);
        (assign34590_e49691, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18,)
    }
};
        locals.var_qse = assign34590_e49693;
        locals.var_qse_dn0 = assign34590_e49693_d_n0;
        locals.var_qse_dn2 = assign34590_e49693_d_n2;
        locals.var_qse_dn6 = assign34590_e49693_d_n6;
        locals.var_qse_dn7 = assign34590_e49693_d_n7;
        locals.var_qse_dn10 = assign34590_e49693_d_n10;
        locals.var_qse_dn11 = assign34590_e49693_d_n11;
        locals.var_qse_dn12 = assign34590_e49693_d_n12;
        locals.var_qse_dn13 = assign34590_e49693_d_n13;
        locals.var_qse_dn15 = assign34590_e49693_d_n15;
        locals.var_qse_dn16 = assign34590_e49693_d_n16;
        locals.var_qse_dn17 = assign34590_e49693_d_n17;
        locals.var_qse_dn18 = assign34590_e49693_d_n18;
        locals.var_qse_rv = 0.0;

        let assign34620_e49698: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1141 = assign34620_e49698;
        locals.var_guard1141_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_125(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34630_e49704, assign34630_e49704_d_n0, assign34630_e49704_d_n2, assign34630_e49704_d_n6, assign34630_e49704_d_n7, assign34630_e49704_d_n10, assign34630_e49704_d_n11, assign34630_e49704_d_n12, assign34630_e49704_d_n17,) = {
    if (locals.var_guard1141 != 0.0) {
        let assign34630_e49702: f64 = (locals.var_mfactor * locals.var_ibs);
        (assign34630_e49702, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34630_e49704;
        locals.var_ibsb_dn0 = assign34630_e49704_d_n0;
        locals.var_ibsb_dn2 = assign34630_e49704_d_n2;
        locals.var_ibsb_dn6 = assign34630_e49704_d_n6;
        locals.var_ibsb_dn7 = assign34630_e49704_d_n7;
        locals.var_ibsb_dn10 = assign34630_e49704_d_n10;
        locals.var_ibsb_dn11 = assign34630_e49704_d_n11;
        locals.var_ibsb_dn12 = assign34630_e49704_d_n12;
        locals.var_ibsb_dn17 = assign34630_e49704_d_n17;
        locals.var_ibsb_rv = 0.0;

        let (assign34640_e49710, assign34640_e49710_d_n0, assign34640_e49710_d_n2, assign34640_e49710_d_n6, assign34640_e49710_d_n7, assign34640_e49710_d_n10, assign34640_e49710_d_n11, assign34640_e49710_d_n12, assign34640_e49710_d_n17,) = {
    if (locals.var_guard1141 != 0.0) {
        let assign34640_e49708: f64 = (locals.var_mfactor * locals.var_ibd);
        (assign34640_e49708, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34640_e49710;
        locals.var_ibdb_dn0 = assign34640_e49710_d_n0;
        locals.var_ibdb_dn2 = assign34640_e49710_d_n2;
        locals.var_ibdb_dn6 = assign34640_e49710_d_n6;
        locals.var_ibdb_dn7 = assign34640_e49710_d_n7;
        locals.var_ibdb_dn10 = assign34640_e49710_d_n10;
        locals.var_ibdb_dn11 = assign34640_e49710_d_n11;
        locals.var_ibdb_dn12 = assign34640_e49710_d_n12;
        locals.var_ibdb_dn17 = assign34640_e49710_d_n17;
        locals.var_ibdb_rv = 0.0;

        let (assign34650_e49716, assign34650_e49716_d_n0, assign34650_e49716_d_n2, assign34650_e49716_d_n6, assign34650_e49716_d_n7, assign34650_e49716_d_n10, assign34650_e49716_d_n11, assign34650_e49716_d_n12, assign34650_e49716_d_n17,) = {
    if (locals.var_guard1141 != 0.0) {
        let assign34650_e49714: f64 = (locals.var_mfactor * locals.var_qbd);
        (assign34650_e49714, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17),)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34650_e49716;
        locals.var_qbd_s0_dn0 = assign34650_e49716_d_n0;
        locals.var_qbd_s0_dn2 = assign34650_e49716_d_n2;
        locals.var_qbd_s0_dn6 = assign34650_e49716_d_n6;
        locals.var_qbd_s0_dn7 = assign34650_e49716_d_n7;
        locals.var_qbd_s0_dn10 = assign34650_e49716_d_n10;
        locals.var_qbd_s0_dn11 = assign34650_e49716_d_n11;
        locals.var_qbd_s0_dn12 = assign34650_e49716_d_n12;
        locals.var_qbd_s0_dn17 = assign34650_e49716_d_n17;
        locals.var_qbd_s0_rv = 0.0;

        let (assign34660_e49722, assign34660_e49722_d_n0, assign34660_e49722_d_n2, assign34660_e49722_d_n6, assign34660_e49722_d_n7, assign34660_e49722_d_n10, assign34660_e49722_d_n11, assign34660_e49722_d_n12, assign34660_e49722_d_n17,) = {
    if (locals.var_guard1141 != 0.0) {
        let assign34660_e49720: f64 = (locals.var_mfactor * locals.var_qbs);
        (assign34660_e49720, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17),)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34660_e49722;
        locals.var_qbs_s0_dn0 = assign34660_e49722_d_n0;
        locals.var_qbs_s0_dn2 = assign34660_e49722_d_n2;
        locals.var_qbs_s0_dn6 = assign34660_e49722_d_n6;
        locals.var_qbs_s0_dn7 = assign34660_e49722_d_n7;
        locals.var_qbs_s0_dn10 = assign34660_e49722_d_n10;
        locals.var_qbs_s0_dn11 = assign34660_e49722_d_n11;
        locals.var_qbs_s0_dn12 = assign34660_e49722_d_n12;
        locals.var_qbs_s0_dn17 = assign34660_e49722_d_n17;
        locals.var_qbs_s0_rv = 0.0;

        let (assign34670_e49727, assign34670_e49727_d_n0, assign34670_e49727_d_n2, assign34670_e49727_d_n6, assign34670_e49727_d_n7, assign34670_e49727_d_n10, assign34670_e49727_d_n11, assign34670_e49727_d_n12, assign34670_e49727_d_n17,) = {
    if (locals.var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign34670_e49727;
        locals.var_ibsb_dn0 = assign34670_e49727_d_n0;
        locals.var_ibsb_dn2 = assign34670_e49727_d_n2;
        locals.var_ibsb_dn6 = assign34670_e49727_d_n6;
        locals.var_ibsb_dn7 = assign34670_e49727_d_n7;
        locals.var_ibsb_dn10 = assign34670_e49727_d_n10;
        locals.var_ibsb_dn11 = assign34670_e49727_d_n11;
        locals.var_ibsb_dn12 = assign34670_e49727_d_n12;
        locals.var_ibsb_dn17 = assign34670_e49727_d_n17;
        locals.var_ibsb_rv = 0.0;

        let (assign34680_e49732, assign34680_e49732_d_n0, assign34680_e49732_d_n2, assign34680_e49732_d_n6, assign34680_e49732_d_n7, assign34680_e49732_d_n10, assign34680_e49732_d_n11, assign34680_e49732_d_n12, assign34680_e49732_d_n17,) = {
    if (locals.var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign34680_e49732;
        locals.var_ibdb_dn0 = assign34680_e49732_d_n0;
        locals.var_ibdb_dn2 = assign34680_e49732_d_n2;
        locals.var_ibdb_dn6 = assign34680_e49732_d_n6;
        locals.var_ibdb_dn7 = assign34680_e49732_d_n7;
        locals.var_ibdb_dn10 = assign34680_e49732_d_n10;
        locals.var_ibdb_dn11 = assign34680_e49732_d_n11;
        locals.var_ibdb_dn12 = assign34680_e49732_d_n12;
        locals.var_ibdb_dn17 = assign34680_e49732_d_n17;
        locals.var_ibdb_rv = 0.0;

        let (assign34690_e49737, assign34690_e49737_d_n0, assign34690_e49737_d_n2, assign34690_e49737_d_n6, assign34690_e49737_d_n7, assign34690_e49737_d_n10, assign34690_e49737_d_n11, assign34690_e49737_d_n12, assign34690_e49737_d_n17,) = {
    if (locals.var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    }
};
        locals.var_qbd_s0 = assign34690_e49737;
        locals.var_qbd_s0_dn0 = assign34690_e49737_d_n0;
        locals.var_qbd_s0_dn2 = assign34690_e49737_d_n2;
        locals.var_qbd_s0_dn6 = assign34690_e49737_d_n6;
        locals.var_qbd_s0_dn7 = assign34690_e49737_d_n7;
        locals.var_qbd_s0_dn10 = assign34690_e49737_d_n10;
        locals.var_qbd_s0_dn11 = assign34690_e49737_d_n11;
        locals.var_qbd_s0_dn12 = assign34690_e49737_d_n12;
        locals.var_qbd_s0_dn17 = assign34690_e49737_d_n17;
        locals.var_qbd_s0_rv = 0.0;

        let (assign34700_e49742, assign34700_e49742_d_n0, assign34700_e49742_d_n2, assign34700_e49742_d_n6, assign34700_e49742_d_n7, assign34700_e49742_d_n10, assign34700_e49742_d_n11, assign34700_e49742_d_n12, assign34700_e49742_d_n17,) = {
    if (locals.var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    }
};
        locals.var_qbs_s0 = assign34700_e49742;
        locals.var_qbs_s0_dn0 = assign34700_e49742_d_n0;
        locals.var_qbs_s0_dn2 = assign34700_e49742_d_n2;
        locals.var_qbs_s0_dn6 = assign34700_e49742_d_n6;
        locals.var_qbs_s0_dn7 = assign34700_e49742_d_n7;
        locals.var_qbs_s0_dn10 = assign34700_e49742_d_n10;
        locals.var_qbs_s0_dn11 = assign34700_e49742_d_n11;
        locals.var_qbs_s0_dn12 = assign34700_e49742_d_n12;
        locals.var_qbs_s0_dn17 = assign34700_e49742_d_n17;
        locals.var_qbs_s0_rv = 0.0;

        let assign34710_e49745: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1142 = assign34710_e49745;
        locals.var_guard1142_rv = 0.0;

        let (assign34720_e49749, assign34720_e49749_d_n0, assign34720_e49749_d_n2, assign34720_e49749_d_n6, assign34720_e49749_d_n7, assign34720_e49749_d_n10, assign34720_e49749_d_n11, assign34720_e49749_d_n12, assign34720_e49749_d_n17,) = {
    if (locals.var_guard1142 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34720_e49749;
        locals.var_isube_dn0 = assign34720_e49749_d_n0;
        locals.var_isube_dn2 = assign34720_e49749_d_n2;
        locals.var_isube_dn6 = assign34720_e49749_d_n6;
        locals.var_isube_dn7 = assign34720_e49749_d_n7;
        locals.var_isube_dn10 = assign34720_e49749_d_n10;
        locals.var_isube_dn11 = assign34720_e49749_d_n11;
        locals.var_isube_dn12 = assign34720_e49749_d_n12;
        locals.var_isube_dn17 = assign34720_e49749_d_n17;
        locals.var_isube_rv = 0.0;

        let (assign34730_e49756, assign34730_e49756_d_n0, assign34730_e49756_d_n2, assign34730_e49756_d_n6, assign34730_e49756_d_n7, assign34730_e49756_d_n10, assign34730_e49756_d_n11, assign34730_e49756_d_n12, assign34730_e49756_d_n17,) = {
    if (locals.var_guard1142 == 0.0) {
        let assign34730_e49754: f64 = (locals.var_mfactor * locals.var_isub);
        (assign34730_e49754, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17),)
    } else {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    }
};
        locals.var_isube = assign34730_e49756;
        locals.var_isube_dn0 = assign34730_e49756_d_n0;
        locals.var_isube_dn2 = assign34730_e49756_d_n2;
        locals.var_isube_dn6 = assign34730_e49756_d_n6;
        locals.var_isube_dn7 = assign34730_e49756_d_n7;
        locals.var_isube_dn10 = assign34730_e49756_d_n10;
        locals.var_isube_dn11 = assign34730_e49756_d_n11;
        locals.var_isube_dn12 = assign34730_e49756_d_n12;
        locals.var_isube_dn17 = assign34730_e49756_d_n17;
        locals.var_isube_rv = 0.0;

        let assign34840_e49838: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign34840_e49838;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn12 = (locals.var_mfactor * locals.var_nthrml_dn12);
        locals.var_noithrml_dn17 = (locals.var_mfactor * locals.var_nthrml_dn17);
        locals.var_noithrml_rv = 0.0;

        let assign34850_e49841: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign34850_e49841;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;
        locals.var_cgdbd_dn15 = 0.0;
        locals.var_cgdbd_dn16 = 0.0;
        locals.var_cgdbd_dn17 = 0.0;
        locals.var_cgdbd_dn18 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign34860_e49844: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign34860_e49844;
        locals.var_cgdbd_dn0 = (p.p50 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p50 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn6 = (p.p50 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p50 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn10 = (p.p50 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p50 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p50 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_dn13 = (p.p50 * locals.var_cgdbd_dn13);
        locals.var_cgdbd_dn15 = (p.p50 * locals.var_cgdbd_dn15);
        locals.var_cgdbd_dn16 = (p.p50 * locals.var_cgdbd_dn16);
        locals.var_cgdbd_dn17 = (p.p50 * locals.var_cgdbd_dn17);
        locals.var_cgdbd_dn18 = (p.p50 * locals.var_cgdbd_dn18);
        locals.var_cgdbd_rv = 0.0;

        let assign34870_e49847: f64 = locals.var_qge_dn7;
        locals.var_cgsbd = assign34870_e49847;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;
        locals.var_cgsbd_dn15 = 0.0;
        locals.var_cgsbd_dn16 = 0.0;
        locals.var_cgsbd_dn17 = 0.0;
        locals.var_cgsbd_dn18 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign34880_e49850: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign34880_e49850;
        locals.var_cgsbd_dn0 = (p.p50 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p50 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn6 = (p.p50 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p50 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn10 = (p.p50 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p50 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p50 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_dn13 = (p.p50 * locals.var_cgsbd_dn13);
        locals.var_cgsbd_dn15 = (p.p50 * locals.var_cgsbd_dn15);
        locals.var_cgsbd_dn16 = (p.p50 * locals.var_cgsbd_dn16);
        locals.var_cgsbd_dn17 = (p.p50 * locals.var_cgsbd_dn17);
        locals.var_cgsbd_dn18 = (p.p50 * locals.var_cgsbd_dn18);
        locals.var_cgsbd_rv = 0.0;

        let (assign34890_e49856, assign34890_e49856_d_n0, assign34890_e49856_d_n2, assign34890_e49856_d_n6, assign34890_e49856_d_n7, assign34890_e49856_d_n10, assign34890_e49856_d_n11, assign34890_e49856_d_n12, assign34890_e49856_d_n13, assign34890_e49856_d_n15, assign34890_e49856_d_n16, assign34890_e49856_d_n17, assign34890_e49856_d_n18,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18,)
    }
};
        locals.var_cgsb = assign34890_e49856;
        locals.var_cgsb_dn0 = assign34890_e49856_d_n0;
        locals.var_cgsb_dn2 = assign34890_e49856_d_n2;
        locals.var_cgsb_dn6 = assign34890_e49856_d_n6;
        locals.var_cgsb_dn7 = assign34890_e49856_d_n7;
        locals.var_cgsb_dn10 = assign34890_e49856_d_n10;
        locals.var_cgsb_dn11 = assign34890_e49856_d_n11;
        locals.var_cgsb_dn12 = assign34890_e49856_d_n12;
        locals.var_cgsb_dn13 = assign34890_e49856_d_n13;
        locals.var_cgsb_dn15 = assign34890_e49856_d_n15;
        locals.var_cgsb_dn16 = assign34890_e49856_d_n16;
        locals.var_cgsb_dn17 = assign34890_e49856_d_n17;
        locals.var_cgsb_dn18 = assign34890_e49856_d_n18;
        locals.var_cgsb_rv = 0.0;

        let assign34900_e49870: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1151 = assign34900_e49870;
        locals.var_guard1151_rv = 0.0;

        let (assign34910_e49880, assign34910_e49880_d_n0, assign34910_e49880_d_n2, assign34910_e49880_d_n6, assign34910_e49880_d_n7, assign34910_e49880_d_n10, assign34910_e49880_d_n11, assign34910_e49880_d_n12, assign34910_e49880_d_n17,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign34910_e49874: f64 = (1e-6 * locals.var_c_fox);
        let assign34910_e49876: f64 = (assign34910_e49874 * locals.var_weffcv_nf);
        let assign34910_e49878: f64 = (assign34910_e49876 * locals.var_leff_cv);
        (assign34910_e49878, (((1e-6 * locals.var_c_fox_dn0) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn2) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn6) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn7) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn10) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn11) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn12) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn17) * locals.var_weffcv_nf) * locals.var_leff_cv),)
    } else {
        (locals.var_t0__blk1145, locals.var_t0__blk1145_dn0, locals.var_t0__blk1145_dn2, locals.var_t0__blk1145_dn6, locals.var_t0__blk1145_dn7, locals.var_t0__blk1145_dn10, locals.var_t0__blk1145_dn11, locals.var_t0__blk1145_dn12, locals.var_t0__blk1145_dn17,)
    }
};
        locals.var_t0__blk1145 = assign34910_e49880;
        locals.var_t0__blk1145_dn0 = assign34910_e49880_d_n0;
        locals.var_t0__blk1145_dn2 = assign34910_e49880_d_n2;
        locals.var_t0__blk1145_dn6 = assign34910_e49880_d_n6;
        locals.var_t0__blk1145_dn7 = assign34910_e49880_d_n7;
        locals.var_t0__blk1145_dn10 = assign34910_e49880_d_n10;
        locals.var_t0__blk1145_dn11 = assign34910_e49880_d_n11;
        locals.var_t0__blk1145_dn12 = assign34910_e49880_d_n12;
        locals.var_t0__blk1145_dn17 = assign34910_e49880_d_n17;
        locals.var_t0__blk1145_rv = 0.0;

        let (assign34920_e49886, assign34920_e49886_d_n0, assign34920_e49886_d_n2, assign34920_e49886_d_n6, assign34920_e49886_d_n7, assign34920_e49886_d_n10, assign34920_e49886_d_n11, assign34920_e49886_d_n12, assign34920_e49886_d_n13, assign34920_e49886_d_n15, assign34920_e49886_d_n16, assign34920_e49886_d_n17, assign34920_e49886_d_n18,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign34920_e49884: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign34920_e49884, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor), (locals.var_cgsb_dn15 / locals.var_mfactor), (locals.var_cgsb_dn16 / locals.var_mfactor), (locals.var_cgsb_dn17 / locals.var_mfactor), (locals.var_cgsb_dn18 / locals.var_mfactor),)
    } else {
        (locals.var_t1__blk1146, locals.var_t1__blk1146_dn0, locals.var_t1__blk1146_dn2, locals.var_t1__blk1146_dn6, locals.var_t1__blk1146_dn7, locals.var_t1__blk1146_dn10, locals.var_t1__blk1146_dn11, locals.var_t1__blk1146_dn12, locals.var_t1__blk1146_dn13, locals.var_t1__blk1146_dn15, locals.var_t1__blk1146_dn16, locals.var_t1__blk1146_dn17, locals.var_t1__blk1146_dn18,)
    }
};
        locals.var_t1__blk1146 = assign34920_e49886;
        locals.var_t1__blk1146_dn0 = assign34920_e49886_d_n0;
        locals.var_t1__blk1146_dn2 = assign34920_e49886_d_n2;
        locals.var_t1__blk1146_dn6 = assign34920_e49886_d_n6;
        locals.var_t1__blk1146_dn7 = assign34920_e49886_d_n7;
        locals.var_t1__blk1146_dn10 = assign34920_e49886_d_n10;
        locals.var_t1__blk1146_dn11 = assign34920_e49886_d_n11;
        locals.var_t1__blk1146_dn12 = assign34920_e49886_d_n12;
        locals.var_t1__blk1146_dn13 = assign34920_e49886_d_n13;
        locals.var_t1__blk1146_dn15 = assign34920_e49886_d_n15;
        locals.var_t1__blk1146_dn16 = assign34920_e49886_d_n16;
        locals.var_t1__blk1146_dn17 = assign34920_e49886_d_n17;
        locals.var_t1__blk1146_dn18 = assign34920_e49886_d_n18;
        locals.var_t1__blk1146_rv = 0.0;

        let (assign34930_e49900, assign34930_e49900_d_n0, assign34930_e49900_d_n2, assign34930_e49900_d_n6, assign34930_e49900_d_n7, assign34930_e49900_d_n10, assign34930_e49900_d_n11, assign34930_e49900_d_n12, assign34930_e49900_d_n13, assign34930_e49900_d_n15, assign34930_e49900_d_n16, assign34930_e49900_d_n17, assign34930_e49900_d_n18,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign34930_e49890: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign34930_e49892: f64 = (assign34930_e49890 * locals.var_beta_inv);
        let assign34930_e49894: f64 = (assign34930_e49892 * locals.var_t1__blk1146);
        let assign34930_e49896: f64 = (assign34930_e49894 * locals.var_t1__blk1146);
        let assign34930_e49898: f64 = (assign34930_e49896 / locals.var_gds0_ign);
        (assign34930_e49898, ((((((assign34930_e49892 * locals.var_t1__blk1146_dn0) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn0)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34930_e49892 * locals.var_t1__blk1146_dn2) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn2)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34930_e49892 * locals.var_t1__blk1146_dn6) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn6)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34930_e49892 * locals.var_t1__blk1146_dn7) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn7)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign34930_e49890 * locals.var_beta_inv_dn10) * locals.var_t1__blk1146) + (assign34930_e49892 * locals.var_t1__blk1146_dn10)) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn10)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34930_e49892 * locals.var_t1__blk1146_dn11) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn11)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34930_e49892 * locals.var_t1__blk1146_dn12) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn12)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34930_e49892 * locals.var_t1__blk1146_dn13) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn13)) / locals.var_gds0_ign), ((((assign34930_e49892 * locals.var_t1__blk1146_dn15) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn15)) / locals.var_gds0_ign), ((((assign34930_e49892 * locals.var_t1__blk1146_dn16) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn16)) / locals.var_gds0_ign), ((((((assign34930_e49892 * locals.var_t1__blk1146_dn17) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn17)) * locals.var_gds0_ign) - (assign34930_e49896 * locals.var_gds0_ign_dn17)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34930_e49892 * locals.var_t1__blk1146_dn18) * locals.var_t1__blk1146) + (assign34930_e49894 * locals.var_t1__blk1146_dn18)) / locals.var_gds0_ign),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12, locals.var_nign0_dn13, locals.var_nign0_dn15, locals.var_nign0_dn16, locals.var_nign0_dn17, locals.var_nign0_dn18,)
    }
};
        locals.var_nign0 = assign34930_e49900;
        locals.var_nign0_dn0 = assign34930_e49900_d_n0;
        locals.var_nign0_dn2 = assign34930_e49900_d_n2;
        locals.var_nign0_dn6 = assign34930_e49900_d_n6;
        locals.var_nign0_dn7 = assign34930_e49900_d_n7;
        locals.var_nign0_dn10 = assign34930_e49900_d_n10;
        locals.var_nign0_dn11 = assign34930_e49900_d_n11;
        locals.var_nign0_dn12 = assign34930_e49900_d_n12;
        locals.var_nign0_dn13 = assign34930_e49900_d_n13;
        locals.var_nign0_dn15 = assign34930_e49900_d_n15;
        locals.var_nign0_dn16 = assign34930_e49900_d_n16;
        locals.var_nign0_dn17 = assign34930_e49900_d_n17;
        locals.var_nign0_dn18 = assign34930_e49900_d_n18;
        locals.var_nign0_rv = 0.0;

        let assign34940_e49904: f64 = (10.0 * 2.220446049250313e-16);
        let assign34940_e49909: f64 = (10.0 * 2.220446049250313e-16);
        let assign34940_e49911: f64 = if ((locals.var_kusai00l > assign34940_e49904) && (locals.var_vds > assign34940_e49909)) { 1.0 } else { 0.0 };
        locals.var_guard1152 = assign34940_e49911;
        locals.var_guard1152_rv = 0.0;

        let (assign34950_e49919, assign34950_e49919_d_n0, assign34950_e49919_d_n2, assign34950_e49919_d_n6, assign34950_e49919_d_n7, assign34950_e49919_d_n10, assign34950_e49919_d_n11, assign34950_e49919_d_n12, assign34950_e49919_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1152 != 0.0)) {
        let assign34950_e49917: f64 = (locals.var_muun / locals.var_mu);
        (assign34950_e49917, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn17 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn17)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12, locals.var_mumoda_dn17,)
    }
};
        locals.var_mumoda = assign34950_e49919;
        locals.var_mumoda_dn0 = assign34950_e49919_d_n0;
        locals.var_mumoda_dn2 = assign34950_e49919_d_n2;
        locals.var_mumoda_dn6 = assign34950_e49919_d_n6;
        locals.var_mumoda_dn7 = assign34950_e49919_d_n7;
        locals.var_mumoda_dn10 = assign34950_e49919_d_n10;
        locals.var_mumoda_dn11 = assign34950_e49919_d_n11;
        locals.var_mumoda_dn12 = assign34950_e49919_d_n12;
        locals.var_mumoda_dn17 = assign34950_e49919_d_n17;
        locals.var_mumoda_rv = 0.0;

        let (assign34960_e49931, assign34960_e49931_d_n0, assign34960_e49931_d_n2, assign34960_e49931_d_n6, assign34960_e49931_d_n7, assign34960_e49931_d_n10, assign34960_e49931_d_n11, assign34960_e49931_d_n12, assign34960_e49931_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1152 != 0.0)) {
        let assign34960_e49925: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign34960_e49927: f64 = (assign34960_e49925 - locals.var_mumoda);
        let assign34960_e49929: f64 = (assign34960_e49927 / locals.var_vds);
        (assign34960_e49929, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn17) * locals.var_vds) - (assign34960_e49927 * locals.var_vds_dn17)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12, locals.var_mumodb_dn17,)
    }
};
        locals.var_mumodb = assign34960_e49931;
        locals.var_mumodb_dn0 = assign34960_e49931_d_n0;
        locals.var_mumodb_dn2 = assign34960_e49931_d_n2;
        locals.var_mumodb_dn6 = assign34960_e49931_d_n6;
        locals.var_mumodb_dn7 = assign34960_e49931_d_n7;
        locals.var_mumodb_dn10 = assign34960_e49931_d_n10;
        locals.var_mumodb_dn11 = assign34960_e49931_d_n11;
        locals.var_mumodb_dn12 = assign34960_e49931_d_n12;
        locals.var_mumodb_dn17 = assign34960_e49931_d_n17;
        locals.var_mumodb_rv = 0.0;

        let (assign34970_e49953, assign34970_e49953_d_n0, assign34970_e49953_d_n2, assign34970_e49953_d_n6, assign34970_e49953_d_n7, assign34970_e49953_d_n10, assign34970_e49953_d_n11, assign34970_e49953_d_n12, assign34970_e49953_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1152 != 0.0)) {
        let assign34970_e49938: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign34970_e49942: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign34970_e49943: f64 = (locals.var_kusai00 + assign34970_e49942);
        let assign34970_e49945: f64 = (assign34970_e49943 + locals.var_kusail);
        let assign34970_e49946: f64 = (assign34970_e49938 * assign34970_e49945);
        let assign34970_e49949: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign34970_e49950: f64 = (assign34970_e49946 / assign34970_e49949);
        let assign34970_e49951: f64 = (locals.var_mumoda + assign34970_e49950);
        (assign34970_e49951, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign34970_e49949 * assign34970_e49949))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign34970_e49949 * assign34970_e49949))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign34970_e49949 * assign34970_e49949))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign34970_e49949 * assign34970_e49949))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign34970_e49949 * assign34970_e49949))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign34970_e49949 * assign34970_e49949))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign34970_e49949 * assign34970_e49949))), (locals.var_mumoda_dn17 + ((((((0.6666666666666667 * locals.var_mumodb_dn17) * assign34970_e49945) + (assign34970_e49938 * ((locals.var_kusai00_dn17 + ((locals.var_vgvt_dn17 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17))) * assign34970_e49949) - (assign34970_e49946 * (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17))) / (assign34970_e49949 * assign34970_e49949))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34970_e49953;
        locals.var_correct_w1_dn0 = assign34970_e49953_d_n0;
        locals.var_correct_w1_dn2 = assign34970_e49953_d_n2;
        locals.var_correct_w1_dn6 = assign34970_e49953_d_n6;
        locals.var_correct_w1_dn7 = assign34970_e49953_d_n7;
        locals.var_correct_w1_dn10 = assign34970_e49953_d_n10;
        locals.var_correct_w1_dn11 = assign34970_e49953_d_n11;
        locals.var_correct_w1_dn12 = assign34970_e49953_d_n12;
        locals.var_correct_w1_dn17 = assign34970_e49953_d_n17;
        locals.var_correct_w1_rv = 0.0;

        let (assign34980_e49962, assign34980_e49962_d_n0, assign34980_e49962_d_n2, assign34980_e49962_d_n6, assign34980_e49962_d_n7, assign34980_e49962_d_n10, assign34980_e49962_d_n11, assign34980_e49962_d_n12, assign34980_e49962_d_n17,) = {
    if ((locals.var_guard1151 != 0.0) && (locals.var_guard1152 == 0.0)) {
        let assign34980_e49960: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign34980_e49960, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17,)
    }
};
        locals.var_correct_w1 = assign34980_e49962;
        locals.var_correct_w1_dn0 = assign34980_e49962_d_n0;
        locals.var_correct_w1_dn2 = assign34980_e49962_d_n2;
        locals.var_correct_w1_dn6 = assign34980_e49962_d_n6;
        locals.var_correct_w1_dn7 = assign34980_e49962_d_n7;
        locals.var_correct_w1_dn10 = assign34980_e49962_d_n10;
        locals.var_correct_w1_dn11 = assign34980_e49962_d_n11;
        locals.var_correct_w1_dn12 = assign34980_e49962_d_n12;
        locals.var_correct_w1_dn17 = assign34980_e49962_d_n17;
        locals.var_correct_w1_rv = 0.0;

        let (assign34990_e49972, assign34990_e49972_d_n0, assign34990_e49972_d_n2, assign34990_e49972_d_n6, assign34990_e49972_d_n7, assign34990_e49972_d_n10, assign34990_e49972_d_n11, assign34990_e49972_d_n12, assign34990_e49972_d_n13, assign34990_e49972_d_n15, assign34990_e49972_d_n16, assign34990_e49972_d_n17, assign34990_e49972_d_n18,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign34990_e49966: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign34990_e49968: f64 = (assign34990_e49966 * locals.var_kusai_ig);
        let assign34990_e49970: f64 = (assign34990_e49968 * locals.var_correct_w1);
        (assign34990_e49970, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn12)), (((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn15) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn16) * locals.var_kusai_ig) * locals.var_correct_w1), (((((locals.var_mfactor * locals.var_nign0_dn17) * locals.var_kusai_ig) + (assign34990_e49966 * locals.var_kusai_ig_dn17)) * locals.var_correct_w1) + (assign34990_e49968 * locals.var_correct_w1_dn17)), (((locals.var_mfactor * locals.var_nign0_dn18) * locals.var_kusai_ig) * locals.var_correct_w1),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign34990_e49972;
        locals.var_noiigate_dn0 = assign34990_e49972_d_n0;
        locals.var_noiigate_dn2 = assign34990_e49972_d_n2;
        locals.var_noiigate_dn6 = assign34990_e49972_d_n6;
        locals.var_noiigate_dn7 = assign34990_e49972_d_n7;
        locals.var_noiigate_dn10 = assign34990_e49972_d_n10;
        locals.var_noiigate_dn11 = assign34990_e49972_d_n11;
        locals.var_noiigate_dn12 = assign34990_e49972_d_n12;
        locals.var_noiigate_dn13 = assign34990_e49972_d_n13;
        locals.var_noiigate_dn15 = assign34990_e49972_d_n15;
        locals.var_noiigate_dn16 = assign34990_e49972_d_n16;
        locals.var_noiigate_dn17 = assign34990_e49972_d_n17;
        locals.var_noiigate_dn18 = assign34990_e49972_d_n18;
        locals.var_noiigate_rv = 0.0;

        let (assign35010_e49990, assign35010_e49990_d_n0, assign35010_e49990_d_n2, assign35010_e49990_d_n6, assign35010_e49990_d_n7, assign35010_e49990_d_n10, assign35010_e49990_d_n11, assign35010_e49990_d_n12, assign35010_e49990_d_n13, assign35010_e49990_d_n15, assign35010_e49990_d_n16, assign35010_e49990_d_n17, assign35010_e49990_d_n18,) = {
    if (locals.var_guard1151 != 0.0) {
        let assign35010_e49979: f64 = (-locals.var_t1__blk1146);
        let (assign35010_e49988, assign35010_e49988_d_n0, assign35010_e49988_d_n2, assign35010_e49988_d_n6, assign35010_e49988_d_n7, assign35010_e49988_d_n10, assign35010_e49988_d_n11, assign35010_e49988_d_n12, assign35010_e49988_d_n13, assign35010_e49988_d_n15, assign35010_e49988_d_n16, assign35010_e49988_d_n17, assign35010_e49988_d_n18,) = {
            if ((assign35010_e49979 > locals.var_t0__blk1145) && (locals.var_noiigate > 0.0)) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign35010_e49988, assign35010_e49988_d_n0, assign35010_e49988_d_n2, assign35010_e49988_d_n6, assign35010_e49988_d_n7, assign35010_e49988_d_n10, assign35010_e49988_d_n11, assign35010_e49988_d_n12, assign35010_e49988_d_n13, assign35010_e49988_d_n15, assign35010_e49988_d_n16, assign35010_e49988_d_n17, assign35010_e49988_d_n18,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign35010_e49990;
        locals.var_noiigate_dn0 = assign35010_e49990_d_n0;
        locals.var_noiigate_dn2 = assign35010_e49990_d_n2;
        locals.var_noiigate_dn6 = assign35010_e49990_d_n6;
        locals.var_noiigate_dn7 = assign35010_e49990_d_n7;
        locals.var_noiigate_dn10 = assign35010_e49990_d_n10;
        locals.var_noiigate_dn11 = assign35010_e49990_d_n11;
        locals.var_noiigate_dn12 = assign35010_e49990_d_n12;
        locals.var_noiigate_dn13 = assign35010_e49990_d_n13;
        locals.var_noiigate_dn15 = assign35010_e49990_d_n15;
        locals.var_noiigate_dn16 = assign35010_e49990_d_n16;
        locals.var_noiigate_dn17 = assign35010_e49990_d_n17;
        locals.var_noiigate_dn18 = assign35010_e49990_d_n18;
        locals.var_noiigate_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_126(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign35030_e50005, assign35030_e50005_d_n0, assign35030_e50005_d_n2, assign35030_e50005_d_n6, assign35030_e50005_d_n7, assign35030_e50005_d_n10, assign35030_e50005_d_n11, assign35030_e50005_d_n12, assign35030_e50005_d_n13, assign35030_e50005_d_n15, assign35030_e50005_d_n16, assign35030_e50005_d_n17, assign35030_e50005_d_n18,) = {
    if (locals.var_guard1151 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    }
};
        locals.var_noiigate = assign35030_e50005;
        locals.var_noiigate_dn0 = assign35030_e50005_d_n0;
        locals.var_noiigate_dn2 = assign35030_e50005_d_n2;
        locals.var_noiigate_dn6 = assign35030_e50005_d_n6;
        locals.var_noiigate_dn7 = assign35030_e50005_d_n7;
        locals.var_noiigate_dn10 = assign35030_e50005_d_n10;
        locals.var_noiigate_dn11 = assign35030_e50005_d_n11;
        locals.var_noiigate_dn12 = assign35030_e50005_d_n12;
        locals.var_noiigate_dn13 = assign35030_e50005_d_n13;
        locals.var_noiigate_dn15 = assign35030_e50005_d_n15;
        locals.var_noiigate_dn16 = assign35030_e50005_d_n16;
        locals.var_noiigate_dn17 = assign35030_e50005_d_n17;
        locals.var_noiigate_dn18 = assign35030_e50005_d_n18;
        locals.var_noiigate_rv = 0.0;

        let assign35090_e50017: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1153 = assign35090_e50017;
        locals.var_guard1153_rv = 0.0;

        let (assign35100_e50021,) = {
    if (locals.var_guard1153 != 0.0) {
        (1.0,)
    } else {
        (locals.var_rdmod,)
    }
};
        locals.var_rdmod = assign35100_e50021;
        locals.var_rdmod_rv = 0.0;

        let assign35110_e50024: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign35110_e50024;
        locals.var_guard1173_rv = 0.0;

        let (assign35130_e50038,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 != 0.0)) {
        (p.p266,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35130_e50038;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign35140_e50044,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 != 0.0)) {
        (p.p268,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35140_e50044;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign35150_e50050, assign35150_e50050_d_n10,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35150_e50050;
        locals.var_rrdrbb_dn10 = assign35150_e50050_d_n10;
        locals.var_rrdrbb_rv = 0.0;

        let (assign35170_e50069,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 != 0.0)) {
        (p.p258,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35170_e50069;
        locals.var_ldrifte_rv = 0.0;

        let (assign35180_e50077, assign35180_e50077_d_n0, assign35180_e50077_d_n2, assign35180_e50077_d_n6, assign35180_e50077_d_n7,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 != 0.0)) {
        let assign35180_e50075: f64 = (p.p50 * (nv7 - nv2));
        (assign35180_e50075, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35180_e50077;
        locals.var_vrdr_dn0 = assign35180_e50077_d_n0;
        locals.var_vrdr_dn2 = assign35180_e50077_d_n2;
        locals.var_vrdr_dn6 = assign35180_e50077_d_n6;
        locals.var_vrdr_dn7 = assign35180_e50077_d_n7;
        locals.var_vrdr_rv = 0.0;

        let (assign35200_e50093,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 == 0.0)) {
        (p.p265,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35200_e50093;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign35210_e50100,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 == 0.0)) {
        (p.p267,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35210_e50100;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign35220_e50107, assign35220_e50107_d_n10,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35220_e50107;
        locals.var_rrdrbb_dn10 = assign35220_e50107_d_n10;
        locals.var_rrdrbb_rv = 0.0;

        let (assign35240_e50128,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 == 0.0)) {
        (p.p257,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign35240_e50128;
        locals.var_ldrifte_rv = 0.0;

        let (assign35250_e50137, assign35250_e50137_d_n0, assign35250_e50137_d_n2, assign35250_e50137_d_n6, assign35250_e50137_d_n7,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 == 0.0)) {
        let assign35250_e50135: f64 = (p.p50 * (nv0 - nv6));
        (assign35250_e50135, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7,)
    }
};
        locals.var_vrdr = assign35250_e50137;
        locals.var_vrdr_dn0 = assign35250_e50137_d_n0;
        locals.var_vrdr_dn2 = assign35250_e50137_d_n2;
        locals.var_vrdr_dn6 = assign35250_e50137_d_n6;
        locals.var_vrdr_dn7 = assign35250_e50137_d_n7;
        locals.var_vrdr_rv = 0.0;

        let (assign35280_e50160,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35280_e50158: f64 = (locals.var_mks_rdrmue / 10000.0);
        (assign35280_e50158,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign35280_e50160;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign35290_e50166,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35290_e50164: f64 = (locals.var_mks_rdrvmax / 100.0);
        (assign35290_e50164,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign35290_e50166;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign35300_e50172, assign35300_e50172_d_n10,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35300_e50170: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign35300_e50170, (locals.var_ttemp_dn10 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn10,)
    }
};
        locals.var_tratio = assign35300_e50172;
        locals.var_tratio_dn10 = assign35300_e50172_d_n10;
        locals.var_tratio_rv = 0.0;

        let (assign35310_e50178, assign35310_e50178_d_n0, assign35310_e50178_d_n2, assign35310_e50178_d_n6, assign35310_e50178_d_n7, assign35310_e50178_d_n10, assign35310_e50178_d_n11, assign35310_e50178_d_n12, assign35310_e50178_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35310_e50176: f64 = (locals.var_tratio).powf(p.p269);
        (assign35310_e50176, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35310_e50176 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35310_e50178;
        locals.var_t1_dn0 = assign35310_e50178_d_n0;
        locals.var_t1_dn2 = assign35310_e50178_d_n2;
        locals.var_t1_dn6 = assign35310_e50178_d_n6;
        locals.var_t1_dn7 = assign35310_e50178_d_n7;
        locals.var_t1_dn10 = assign35310_e50178_d_n10;
        locals.var_t1_dn11 = assign35310_e50178_d_n11;
        locals.var_t1_dn12 = assign35310_e50178_d_n12;
        locals.var_t1_dn17 = assign35310_e50178_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign35320_e50184, assign35320_e50184_d_n0, assign35320_e50184_d_n2, assign35320_e50184_d_n6, assign35320_e50184_d_n7, assign35320_e50184_d_n10, assign35320_e50184_d_n11, assign35320_e50184_d_n12, assign35320_e50184_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35320_e50182: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign35320_e50182, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35320_e50184;
        locals.var_mu0_dn0 = assign35320_e50184_d_n0;
        locals.var_mu0_dn2 = assign35320_e50184_d_n2;
        locals.var_mu0_dn6 = assign35320_e50184_d_n6;
        locals.var_mu0_dn7 = assign35320_e50184_d_n7;
        locals.var_mu0_dn10 = assign35320_e50184_d_n10;
        locals.var_mu0_dn11 = assign35320_e50184_d_n11;
        locals.var_mu0_dn12 = assign35320_e50184_d_n12;
        locals.var_mu0_dn17 = assign35320_e50184_d_n17;
        locals.var_mu0_rv = 0.0;

        let (assign35330_e50204, assign35330_e50204_d_n0, assign35330_e50204_d_n2, assign35330_e50204_d_n6, assign35330_e50204_d_n7, assign35330_e50204_d_n10, assign35330_e50204_d_n11, assign35330_e50204_d_n12, assign35330_e50204_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35330_e50189: f64 = (0.4 * locals.var_tratio);
        let assign35330_e50190: f64 = (1.8 + assign35330_e50189);
        let assign35330_e50193: f64 = (0.1 * locals.var_tratio);
        let assign35330_e50195: f64 = (assign35330_e50193 * locals.var_tratio);
        let assign35330_e50196: f64 = (assign35330_e50190 + assign35330_e50195);
        let assign35330_e50200: f64 = (1.0 - locals.var_tratio);
        let assign35330_e50201: f64 = (p.p270 * assign35330_e50200);
        let assign35330_e50202: f64 = (assign35330_e50196 - assign35330_e50201);
        (assign35330_e50202, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35330_e50193 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign35330_e50204;
        locals.var_t0_dn0 = assign35330_e50204_d_n0;
        locals.var_t0_dn2 = assign35330_e50204_d_n2;
        locals.var_t0_dn6 = assign35330_e50204_d_n6;
        locals.var_t0_dn7 = assign35330_e50204_d_n7;
        locals.var_t0_dn10 = assign35330_e50204_d_n10;
        locals.var_t0_dn11 = assign35330_e50204_d_n11;
        locals.var_t0_dn12 = assign35330_e50204_d_n12;
        locals.var_t0_dn17 = assign35330_e50204_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign35340_e50210, assign35340_e50210_d_n0, assign35340_e50210_d_n2, assign35340_e50210_d_n6, assign35340_e50210_d_n7, assign35340_e50210_d_n10, assign35340_e50210_d_n11, assign35340_e50210_d_n12, assign35340_e50210_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35340_e50208: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign35340_e50208, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk1166, locals.var_vmaxe__blk1166_dn0, locals.var_vmaxe__blk1166_dn2, locals.var_vmaxe__blk1166_dn6, locals.var_vmaxe__blk1166_dn7, locals.var_vmaxe__blk1166_dn10, locals.var_vmaxe__blk1166_dn11, locals.var_vmaxe__blk1166_dn12, locals.var_vmaxe__blk1166_dn17,)
    }
};
        locals.var_vmaxe__blk1166 = assign35340_e50210;
        locals.var_vmaxe__blk1166_dn0 = assign35340_e50210_d_n0;
        locals.var_vmaxe__blk1166_dn2 = assign35340_e50210_d_n2;
        locals.var_vmaxe__blk1166_dn6 = assign35340_e50210_d_n6;
        locals.var_vmaxe__blk1166_dn7 = assign35340_e50210_d_n7;
        locals.var_vmaxe__blk1166_dn10 = assign35340_e50210_d_n10;
        locals.var_vmaxe__blk1166_dn11 = assign35340_e50210_d_n11;
        locals.var_vmaxe__blk1166_dn12 = assign35340_e50210_d_n12;
        locals.var_vmaxe__blk1166_dn17 = assign35340_e50210_d_n17;
        locals.var_vmaxe__blk1166_rv = 0.0;

        let (assign35350_e50220, assign35350_e50220_d_n10,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35350_e50216: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign35350_e50217: f64 = (p.p274 * assign35350_e50216);
        let assign35350_e50218: f64 = (locals.var_rrdrbb + assign35350_e50217);
        (assign35350_e50218, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)),)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn10,)
    }
};
        locals.var_rrdrbb = assign35350_e50220;
        locals.var_rrdrbb_dn10 = assign35350_e50220_d_n10;
        locals.var_rrdrbb_rv = 0.0;

        let (assign35360_e50230,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35360_e50226: f64 = (locals.var_lgle).powf(p.p280);
        let assign35360_e50227: f64 = (p.p279 / assign35360_e50226);
        let assign35360_e50228: f64 = (1.0 + assign35360_e50227);
        (assign35360_e50228,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign35360_e50230;
        locals.var_rdrmuele_rv = 0.0;

        let (assign35370_e50240,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35370_e50236: f64 = (locals.var_lgle).powf(p.p278);
        let assign35370_e50237: f64 = (p.p277 / assign35370_e50236);
        let assign35370_e50238: f64 = (1.0 + assign35370_e50237);
        (assign35370_e50238,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign35370_e50240;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign35380_e50250,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35380_e50246: f64 = (locals.var_wg).powf(p.p276);
        let assign35380_e50247: f64 = (p.p275 / assign35380_e50246);
        let assign35380_e50248: f64 = (1.0 + assign35380_e50247);
        (assign35380_e50248,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign35380_e50250;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign35390_e50256, assign35390_e50256_d_n0, assign35390_e50256_d_n2, assign35390_e50256_d_n6, assign35390_e50256_d_n7, assign35390_e50256_d_n10, assign35390_e50256_d_n11, assign35390_e50256_d_n12, assign35390_e50256_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35390_e50254: f64 = (locals.var_mu0 * locals.var_rdrmuele);
        (assign35390_e50254, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17,)
    }
};
        locals.var_mu0 = assign35390_e50256;
        locals.var_mu0_dn0 = assign35390_e50256_d_n0;
        locals.var_mu0_dn2 = assign35390_e50256_d_n2;
        locals.var_mu0_dn6 = assign35390_e50256_d_n6;
        locals.var_mu0_dn7 = assign35390_e50256_d_n7;
        locals.var_mu0_dn10 = assign35390_e50256_d_n10;
        locals.var_mu0_dn11 = assign35390_e50256_d_n11;
        locals.var_mu0_dn12 = assign35390_e50256_d_n12;
        locals.var_mu0_dn17 = assign35390_e50256_d_n17;
        locals.var_mu0_rv = 0.0;

        let (assign35400_e50266, assign35400_e50266_d_n0, assign35400_e50266_d_n2, assign35400_e50266_d_n6, assign35400_e50266_d_n7, assign35400_e50266_d_n10, assign35400_e50266_d_n11, assign35400_e50266_d_n12, assign35400_e50266_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35400_e50260: f64 = (locals.var_vmaxe__blk1166 * locals.var_rdrvmaxwe);
        let assign35400_e50262: f64 = (assign35400_e50260 * locals.var_rdrvmaxle);
        let assign35400_e50264: f64 = (assign35400_e50262 + 1e-50);
        (assign35400_e50264, ((locals.var_vmaxe__blk1166_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk1166, locals.var_vmaxe__blk1166_dn0, locals.var_vmaxe__blk1166_dn2, locals.var_vmaxe__blk1166_dn6, locals.var_vmaxe__blk1166_dn7, locals.var_vmaxe__blk1166_dn10, locals.var_vmaxe__blk1166_dn11, locals.var_vmaxe__blk1166_dn12, locals.var_vmaxe__blk1166_dn17,)
    }
};
        locals.var_vmaxe__blk1166 = assign35400_e50266;
        locals.var_vmaxe__blk1166_dn0 = assign35400_e50266_d_n0;
        locals.var_vmaxe__blk1166_dn2 = assign35400_e50266_d_n2;
        locals.var_vmaxe__blk1166_dn6 = assign35400_e50266_d_n6;
        locals.var_vmaxe__blk1166_dn7 = assign35400_e50266_d_n7;
        locals.var_vmaxe__blk1166_dn10 = assign35400_e50266_d_n10;
        locals.var_vmaxe__blk1166_dn11 = assign35400_e50266_d_n11;
        locals.var_vmaxe__blk1166_dn12 = assign35400_e50266_d_n12;
        locals.var_vmaxe__blk1166_dn17 = assign35400_e50266_d_n17;
        locals.var_vmaxe__blk1166_rv = 0.0;

        let (assign35410_e50272, assign35410_e50272_d_n0, assign35410_e50272_d_n2, assign35410_e50272_d_n6, assign35410_e50272_d_n7,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35410_e50270: f64 = (locals.var_vrdr / locals.var_ldrifte);
        (assign35410_e50270, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7,)
    }
};
        locals.var_edri = assign35410_e50272;
        locals.var_edri_dn0 = assign35410_e50272_d_n0;
        locals.var_edri_dn2 = assign35410_e50272_d_n2;
        locals.var_edri_dn6 = assign35410_e50272_d_n6;
        locals.var_edri_dn7 = assign35410_e50272_d_n7;
        locals.var_edri_rv = 0.0;

        let (assign35420_e50278, assign35420_e50278_d_n0, assign35420_e50278_d_n2, assign35420_e50278_d_n6, assign35420_e50278_d_n7, assign35420_e50278_d_n10, assign35420_e50278_d_n11, assign35420_e50278_d_n12, assign35420_e50278_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35420_e50276: f64 = (locals.var_mu0 * locals.var_edri);
        (assign35420_e50276, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17,)
    }
};
        locals.var_vdri = assign35420_e50278;
        locals.var_vdri_dn0 = assign35420_e50278_d_n0;
        locals.var_vdri_dn2 = assign35420_e50278_d_n2;
        locals.var_vdri_dn6 = assign35420_e50278_d_n6;
        locals.var_vdri_dn7 = assign35420_e50278_d_n7;
        locals.var_vdri_dn10 = assign35420_e50278_d_n10;
        locals.var_vdri_dn11 = assign35420_e50278_d_n11;
        locals.var_vdri_dn12 = assign35420_e50278_d_n12;
        locals.var_vdri_dn17 = assign35420_e50278_d_n17;
        locals.var_vdri_rv = 0.0;

        let assign35430_e50281: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign35430_e50281;
        locals.var_guard1174_rv = 0.0;

        let (assign35440_e50289, assign35440_e50289_d_n0, assign35440_e50289_d_n2, assign35440_e50289_d_n6, assign35440_e50289_d_n7, assign35440_e50289_d_n10, assign35440_e50289_d_n11, assign35440_e50289_d_n12, assign35440_e50289_d_n17,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1174 != 0.0)) {
        let assign35440_e50287: f64 = (locals.var_vdri / locals.var_vmaxe__blk1166);
        (assign35440_e50287, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn0)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn2)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn6)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn7)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn10)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn11)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn12)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn17)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35440_e50289;
        locals.var_t1_dn0 = assign35440_e50289_d_n0;
        locals.var_t1_dn2 = assign35440_e50289_d_n2;
        locals.var_t1_dn6 = assign35440_e50289_d_n6;
        locals.var_t1_dn7 = assign35440_e50289_d_n7;
        locals.var_t1_dn10 = assign35440_e50289_d_n10;
        locals.var_t1_dn11 = assign35440_e50289_d_n11;
        locals.var_t1_dn12 = assign35440_e50289_d_n12;
        locals.var_t1_dn17 = assign35440_e50289_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign35450_e50299, assign35450_e50299_d_n0, assign35450_e50299_d_n2, assign35450_e50299_d_n6, assign35450_e50299_d_n7, assign35450_e50299_d_n10, assign35450_e50299_d_n11, assign35450_e50299_d_n12, assign35450_e50299_d_n17,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign35450_e50295: f64 = (-locals.var_vdri);
        let assign35450_e50297: f64 = (assign35450_e50295 / locals.var_vmaxe__blk1166);
        (assign35450_e50297, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn0)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn2)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn6)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn7)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn10)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn11)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn12)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn17)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35450_e50299;
        locals.var_t1_dn0 = assign35450_e50299_d_n0;
        locals.var_t1_dn2 = assign35450_e50299_d_n2;
        locals.var_t1_dn6 = assign35450_e50299_d_n6;
        locals.var_t1_dn7 = assign35450_e50299_d_n7;
        locals.var_t1_dn10 = assign35450_e50299_d_n10;
        locals.var_t1_dn11 = assign35450_e50299_d_n11;
        locals.var_t1_dn12 = assign35450_e50299_d_n12;
        locals.var_t1_dn17 = assign35450_e50299_d_n17;
        locals.var_t1_rv = 0.0;

        let assign35460_e50303: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50304: f64 = (1.0 - assign35460_e50303);
        let assign35460_e50311: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50312: f64 = (1.0 + assign35460_e50311);
        let assign35460_e50314: f64 = if ((assign35460_e50304 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35460_e50312)) { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign35460_e50314;
        locals.var_guard1175_rv = 0.0;

        let (assign35470_e50320, assign35470_e50320_d_n0, assign35470_e50320_d_n2, assign35470_e50320_d_n6, assign35470_e50320_d_n7, assign35470_e50320_d_n10, assign35470_e50320_d_n11, assign35470_e50320_d_n12, assign35470_e50320_d_n17,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1175 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35470_e50320;
        locals.var_t3_dn0 = assign35470_e50320_d_n0;
        locals.var_t3_dn2 = assign35470_e50320_d_n2;
        locals.var_t3_dn6 = assign35470_e50320_d_n6;
        locals.var_t3_dn7 = assign35470_e50320_d_n7;
        locals.var_t3_dn10 = assign35470_e50320_d_n10;
        locals.var_t3_dn11 = assign35470_e50320_d_n11;
        locals.var_t3_dn12 = assign35470_e50320_d_n12;
        locals.var_t3_dn17 = assign35470_e50320_d_n17;
        locals.var_t3_rv = 0.0;

        let assign35480_e50324: f64 = (10.0 * 2.220446049250313e-16);
        let assign35480_e50325: f64 = (2.0 - assign35480_e50324);
        let assign35480_e50332: f64 = (10.0 * 2.220446049250313e-16);
        let assign35480_e50333: f64 = (2.0 + assign35480_e50332);
        let assign35480_e50335: f64 = if ((assign35480_e50325 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35480_e50333)) { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign35480_e50335;
        locals.var_guard1176_rv = 0.0;

        let (assign35490_e50344, assign35490_e50344_d_n0, assign35490_e50344_d_n2, assign35490_e50344_d_n6, assign35490_e50344_d_n7, assign35490_e50344_d_n10, assign35490_e50344_d_n11, assign35490_e50344_d_n12, assign35490_e50344_d_n17,) = {
    if (((locals.var_guard1153 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35490_e50344;
        locals.var_t3_dn0 = assign35490_e50344_d_n0;
        locals.var_t3_dn2 = assign35490_e50344_d_n2;
        locals.var_t3_dn6 = assign35490_e50344_d_n6;
        locals.var_t3_dn7 = assign35490_e50344_d_n7;
        locals.var_t3_dn10 = assign35490_e50344_d_n10;
        locals.var_t3_dn11 = assign35490_e50344_d_n11;
        locals.var_t3_dn12 = assign35490_e50344_d_n12;
        locals.var_t3_dn17 = assign35490_e50344_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign35500_e50358, assign35500_e50358_d_n0, assign35500_e50358_d_n2, assign35500_e50358_d_n6, assign35500_e50358_d_n7, assign35500_e50358_d_n10, assign35500_e50358_d_n11, assign35500_e50358_d_n12, assign35500_e50358_d_n17,) = {
    if (((locals.var_guard1153 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 == 0.0)) {
        let assign35500_e50355: f64 = (locals.var_rrdrbb - 1.0);
        let assign35500_e50356: f64 = (locals.var_t1).powf(assign35500_e50355);
        (assign35500_e50356, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn0)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn2)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn6)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn7)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn10)) } } else { (assign35500_e50356 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35500_e50355 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn11)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn12)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn17)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn17 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign35500_e50358;
        locals.var_t3_dn0 = assign35500_e50358_d_n0;
        locals.var_t3_dn2 = assign35500_e50358_d_n2;
        locals.var_t3_dn6 = assign35500_e50358_d_n6;
        locals.var_t3_dn7 = assign35500_e50358_d_n7;
        locals.var_t3_dn10 = assign35500_e50358_d_n10;
        locals.var_t3_dn11 = assign35500_e50358_d_n11;
        locals.var_t3_dn12 = assign35500_e50358_d_n12;
        locals.var_t3_dn17 = assign35500_e50358_d_n17;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_127(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign35510_e50364, assign35510_e50364_d_n0, assign35510_e50364_d_n2, assign35510_e50364_d_n6, assign35510_e50364_d_n7, assign35510_e50364_d_n10, assign35510_e50364_d_n11, assign35510_e50364_d_n12, assign35510_e50364_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35510_e50362: f64 = (locals.var_t1 * locals.var_t3);
        (assign35510_e50362, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign35510_e50364;
        locals.var_t2_dn0 = assign35510_e50364_d_n0;
        locals.var_t2_dn2 = assign35510_e50364_d_n2;
        locals.var_t2_dn6 = assign35510_e50364_d_n6;
        locals.var_t2_dn7 = assign35510_e50364_d_n7;
        locals.var_t2_dn10 = assign35510_e50364_d_n10;
        locals.var_t2_dn11 = assign35510_e50364_d_n11;
        locals.var_t2_dn12 = assign35510_e50364_d_n12;
        locals.var_t2_dn17 = assign35510_e50364_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign35520_e50370, assign35520_e50370_d_n0, assign35520_e50370_d_n2, assign35520_e50370_d_n6, assign35520_e50370_d_n7, assign35520_e50370_d_n10, assign35520_e50370_d_n11, assign35520_e50370_d_n12, assign35520_e50370_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35520_e50368: f64 = (1.0 + locals.var_t2);
        (assign35520_e50368, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign35520_e50370;
        locals.var_t4_dn0 = assign35520_e50370_d_n0;
        locals.var_t4_dn2 = assign35520_e50370_d_n2;
        locals.var_t4_dn6 = assign35520_e50370_d_n6;
        locals.var_t4_dn7 = assign35520_e50370_d_n7;
        locals.var_t4_dn10 = assign35520_e50370_d_n10;
        locals.var_t4_dn11 = assign35520_e50370_d_n11;
        locals.var_t4_dn12 = assign35520_e50370_d_n12;
        locals.var_t4_dn17 = assign35520_e50370_d_n17;
        locals.var_t4_rv = 0.0;

        let assign35530_e50374: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50375: f64 = (1.0 - assign35530_e50374);
        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (1.0 + assign35530_e50382);
        let assign35530_e50385: f64 = if ((assign35530_e50375 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35530_e50383)) { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign35530_e50385;
        locals.var_guard1177_rv = 0.0;

        let (assign35540_e50393, assign35540_e50393_d_n0, assign35540_e50393_d_n2, assign35540_e50393_d_n6, assign35540_e50393_d_n7, assign35540_e50393_d_n10, assign35540_e50393_d_n11, assign35540_e50393_d_n12, assign35540_e50393_d_n17,) = {
    if ((locals.var_guard1153 != 0.0) && (locals.var_guard1177 != 0.0)) {
        let assign35540_e50391: f64 = (1.0 / locals.var_t4);
        (assign35540_e50391, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35540_e50393;
        locals.var_t5_dn0 = assign35540_e50393_d_n0;
        locals.var_t5_dn2 = assign35540_e50393_d_n2;
        locals.var_t5_dn6 = assign35540_e50393_d_n6;
        locals.var_t5_dn7 = assign35540_e50393_d_n7;
        locals.var_t5_dn10 = assign35540_e50393_d_n10;
        locals.var_t5_dn11 = assign35540_e50393_d_n11;
        locals.var_t5_dn12 = assign35540_e50393_d_n12;
        locals.var_t5_dn17 = assign35540_e50393_d_n17;
        locals.var_t5_rv = 0.0;

        let assign35550_e50397: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50398: f64 = (2.0 - assign35550_e50397);
        let assign35550_e50405: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50406: f64 = (2.0 + assign35550_e50405);
        let assign35550_e50408: f64 = if ((assign35550_e50398 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35550_e50406)) { 1.0 } else { 0.0 };
        locals.var_guard1178 = assign35550_e50408;
        locals.var_guard1178_rv = 0.0;

        let (assign35560_e50420, assign35560_e50420_d_n0, assign35560_e50420_d_n2, assign35560_e50420_d_n6, assign35560_e50420_d_n7, assign35560_e50420_d_n10, assign35560_e50420_d_n11, assign35560_e50420_d_n12, assign35560_e50420_d_n17,) = {
    if (((locals.var_guard1153 != 0.0) && (locals.var_guard1177 == 0.0)) && (locals.var_guard1178 != 0.0)) {
        let assign35560_e50417: f64 = (locals.var_t4).sqrt();
        let assign35560_e50418: f64 = (1.0 / assign35560_e50417);
        (assign35560_e50418, (-((locals.var_t4_dn0 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn2 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn6 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn7 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn10 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn11 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn12 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn17 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35560_e50420;
        locals.var_t5_dn0 = assign35560_e50420_d_n0;
        locals.var_t5_dn2 = assign35560_e50420_d_n2;
        locals.var_t5_dn6 = assign35560_e50420_d_n6;
        locals.var_t5_dn7 = assign35560_e50420_d_n7;
        locals.var_t5_dn10 = assign35560_e50420_d_n10;
        locals.var_t5_dn11 = assign35560_e50420_d_n11;
        locals.var_t5_dn12 = assign35560_e50420_d_n12;
        locals.var_t5_dn17 = assign35560_e50420_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign35570_e50437, assign35570_e50437_d_n0, assign35570_e50437_d_n2, assign35570_e50437_d_n6, assign35570_e50437_d_n7, assign35570_e50437_d_n10, assign35570_e50437_d_n11, assign35570_e50437_d_n12, assign35570_e50437_d_n17,) = {
    if (((locals.var_guard1153 != 0.0) && (locals.var_guard1177 == 0.0)) && (locals.var_guard1178 == 0.0)) {
        let assign35570_e50430: f64 = (-1.0);
        let assign35570_e50432: f64 = (assign35570_e50430 / locals.var_rrdrbb);
        let assign35570_e50434: f64 = (assign35570_e50432 - 1.0);
        let assign35570_e50435: f64 = (locals.var_t4).powf(assign35570_e50434);
        (assign35570_e50435, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn0)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn2)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn6)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn7)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign35570_e50430 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn10)) } } else { (assign35570_e50435 * (((-((assign35570_e50430 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign35570_e50434 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn11)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn12)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn17)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn17 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign35570_e50437;
        locals.var_t6_dn0 = assign35570_e50437_d_n0;
        locals.var_t6_dn2 = assign35570_e50437_d_n2;
        locals.var_t6_dn6 = assign35570_e50437_d_n6;
        locals.var_t6_dn7 = assign35570_e50437_d_n7;
        locals.var_t6_dn10 = assign35570_e50437_d_n10;
        locals.var_t6_dn11 = assign35570_e50437_d_n11;
        locals.var_t6_dn12 = assign35570_e50437_d_n12;
        locals.var_t6_dn17 = assign35570_e50437_d_n17;
        locals.var_t6_rv = 0.0;

        let (assign35580_e50449, assign35580_e50449_d_n0, assign35580_e50449_d_n2, assign35580_e50449_d_n6, assign35580_e50449_d_n7, assign35580_e50449_d_n10, assign35580_e50449_d_n11, assign35580_e50449_d_n12, assign35580_e50449_d_n17,) = {
    if (((locals.var_guard1153 != 0.0) && (locals.var_guard1177 == 0.0)) && (locals.var_guard1178 == 0.0)) {
        let assign35580_e50447: f64 = (locals.var_t4 * locals.var_t6);
        (assign35580_e50447, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign35580_e50449;
        locals.var_t5_dn0 = assign35580_e50449_d_n0;
        locals.var_t5_dn2 = assign35580_e50449_d_n2;
        locals.var_t5_dn6 = assign35580_e50449_d_n6;
        locals.var_t5_dn7 = assign35580_e50449_d_n7;
        locals.var_t5_dn10 = assign35580_e50449_d_n10;
        locals.var_t5_dn11 = assign35580_e50449_d_n11;
        locals.var_t5_dn12 = assign35580_e50449_d_n12;
        locals.var_t5_dn17 = assign35580_e50449_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign35600_e50461, assign35600_e50461_d_n0, assign35600_e50461_d_n2, assign35600_e50461_d_n6, assign35600_e50461_d_n7, assign35600_e50461_d_n10, assign35600_e50461_d_n11, assign35600_e50461_d_n12, assign35600_e50461_d_n17,) = {
    if (locals.var_guard1153 != 0.0) {
        let assign35600_e50459: f64 = (1.6021918e-19 / locals.var_ldrifte);
        (assign35600_e50459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35600_e50461;
        locals.var_t1_dn0 = assign35600_e50461_d_n0;
        locals.var_t1_dn2 = assign35600_e50461_d_n2;
        locals.var_t1_dn6 = assign35600_e50461_d_n6;
        locals.var_t1_dn7 = assign35600_e50461_d_n7;
        locals.var_t1_dn10 = assign35600_e50461_d_n10;
        locals.var_t1_dn11 = assign35600_e50461_d_n11;
        locals.var_t1_dn12 = assign35600_e50461_d_n12;
        locals.var_t1_dn17 = assign35600_e50461_d_n17;
        locals.var_t1_rv = 0.0;

        let assign35720_e50535: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1181 = assign35720_e50535;
        locals.var_guard1181_rv = 0.0;

        let (assign35730_e50539,) = {
    if (locals.var_guard1181 != 0.0) {
        (2.0,)
    } else {
        (locals.var_rdmod,)
    }
};
        locals.var_rdmod = assign35730_e50539;
        locals.var_rdmod_rv = 0.0;

        let assign35740_e50542: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign35740_e50542;
        locals.var_guard1201_rv = 0.0;

        let (assign35760_e50556,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 != 0.0)) {
        (p.p266,)
    } else {
        (locals.var_mks_rdrmue__blk1185,)
    }
};
        locals.var_mks_rdrmue__blk1185 = assign35760_e50556;
        locals.var_mks_rdrmue__blk1185_rv = 0.0;

        let (assign35770_e50562,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 != 0.0)) {
        (p.p268,)
    } else {
        (locals.var_mks_rdrvmax__blk1186,)
    }
};
        locals.var_mks_rdrvmax__blk1186 = assign35770_e50562;
        locals.var_mks_rdrvmax__blk1186_rv = 0.0;

        let (assign35780_e50568, assign35780_e50568_d_n10,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (locals.var_rrdrbb__blk1187, locals.var_rrdrbb__blk1187_dn10,)
    }
};
        locals.var_rrdrbb__blk1187 = assign35780_e50568;
        locals.var_rrdrbb__blk1187_dn10 = assign35780_e50568_d_n10;
        locals.var_rrdrbb__blk1187_rv = 0.0;

        let (assign35800_e50587,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 != 0.0)) {
        (p.p258,)
    } else {
        (locals.var_ldrifte__blk1191,)
    }
};
        locals.var_ldrifte__blk1191 = assign35800_e50587;
        locals.var_ldrifte__blk1191_rv = 0.0;

        let (assign35810_e50595, assign35810_e50595_d_n0, assign35810_e50595_d_n2, assign35810_e50595_d_n6, assign35810_e50595_d_n7,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 != 0.0)) {
        let assign35810_e50593: f64 = (p.p50 * (nv7 - nv2));
        (assign35810_e50593, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (locals.var_vrdr__blk1189, locals.var_vrdr__blk1189_dn0, locals.var_vrdr__blk1189_dn2, locals.var_vrdr__blk1189_dn6, locals.var_vrdr__blk1189_dn7,)
    }
};
        locals.var_vrdr__blk1189 = assign35810_e50595;
        locals.var_vrdr__blk1189_dn0 = assign35810_e50595_d_n0;
        locals.var_vrdr__blk1189_dn2 = assign35810_e50595_d_n2;
        locals.var_vrdr__blk1189_dn6 = assign35810_e50595_d_n6;
        locals.var_vrdr__blk1189_dn7 = assign35810_e50595_d_n7;
        locals.var_vrdr__blk1189_rv = 0.0;

        let (assign35830_e50611,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 == 0.0)) {
        (p.p265,)
    } else {
        (locals.var_mks_rdrmue__blk1185,)
    }
};
        locals.var_mks_rdrmue__blk1185 = assign35830_e50611;
        locals.var_mks_rdrmue__blk1185_rv = 0.0;

        let (assign35840_e50618,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 == 0.0)) {
        (p.p267,)
    } else {
        (locals.var_mks_rdrvmax__blk1186,)
    }
};
        locals.var_mks_rdrvmax__blk1186 = assign35840_e50618;
        locals.var_mks_rdrvmax__blk1186_rv = 0.0;

        let (assign35850_e50625, assign35850_e50625_d_n10,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (locals.var_rrdrbb__blk1187, locals.var_rrdrbb__blk1187_dn10,)
    }
};
        locals.var_rrdrbb__blk1187 = assign35850_e50625;
        locals.var_rrdrbb__blk1187_dn10 = assign35850_e50625_d_n10;
        locals.var_rrdrbb__blk1187_rv = 0.0;

        let (assign35870_e50646,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 == 0.0)) {
        (p.p257,)
    } else {
        (locals.var_ldrifte__blk1191,)
    }
};
        locals.var_ldrifte__blk1191 = assign35870_e50646;
        locals.var_ldrifte__blk1191_rv = 0.0;

        let (assign35880_e50655, assign35880_e50655_d_n0, assign35880_e50655_d_n2, assign35880_e50655_d_n6, assign35880_e50655_d_n7,) = {
    if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 == 0.0)) {
        let assign35880_e50653: f64 = (p.p50 * (nv0 - nv6));
        (assign35880_e50653, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (locals.var_vrdr__blk1189, locals.var_vrdr__blk1189_dn0, locals.var_vrdr__blk1189_dn2, locals.var_vrdr__blk1189_dn6, locals.var_vrdr__blk1189_dn7,)
    }
};
        locals.var_vrdr__blk1189 = assign35880_e50655;
        locals.var_vrdr__blk1189_dn0 = assign35880_e50655_d_n0;
        locals.var_vrdr__blk1189_dn2 = assign35880_e50655_d_n2;
        locals.var_vrdr__blk1189_dn6 = assign35880_e50655_d_n6;
        locals.var_vrdr__blk1189_dn7 = assign35880_e50655_d_n7;
        locals.var_vrdr__blk1189_rv = 0.0;

        let (assign35910_e50678,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35910_e50676: f64 = (locals.var_mks_rdrmue__blk1185 / 10000.0);
        (assign35910_e50676,)
    } else {
        (locals.var_mks_rdrmue__blk1185,)
    }
};
        locals.var_mks_rdrmue__blk1185 = assign35910_e50678;
        locals.var_mks_rdrmue__blk1185_rv = 0.0;

        let (assign35920_e50684,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35920_e50682: f64 = (locals.var_mks_rdrvmax__blk1186 / 100.0);
        (assign35920_e50682,)
    } else {
        (locals.var_mks_rdrvmax__blk1186,)
    }
};
        locals.var_mks_rdrvmax__blk1186 = assign35920_e50684;
        locals.var_mks_rdrvmax__blk1186_rv = 0.0;

        let (assign35930_e50690, assign35930_e50690_d_n10,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35930_e50688: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign35930_e50688, (locals.var_ttemp_dn10 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio__blk1190, locals.var_tratio__blk1190_dn10,)
    }
};
        locals.var_tratio__blk1190 = assign35930_e50690;
        locals.var_tratio__blk1190_dn10 = assign35930_e50690_d_n10;
        locals.var_tratio__blk1190_rv = 0.0;

        let (assign35940_e50696, assign35940_e50696_d_n0, assign35940_e50696_d_n2, assign35940_e50696_d_n6, assign35940_e50696_d_n7, assign35940_e50696_d_n10, assign35940_e50696_d_n11, assign35940_e50696_d_n12, assign35940_e50696_d_n17,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35940_e50694: f64 = (locals.var_tratio__blk1190).powf(p.p269);
        (assign35940_e50694, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio__blk1190).powf(p.p269 - 1.0) * locals.var_tratio__blk1190_dn10)) } } else { (assign35940_e50694 * (p.p269 * (locals.var_tratio__blk1190_dn10 / locals.var_tratio__blk1190))) }, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign35940_e50696;
        locals.var_t1_dn0 = assign35940_e50696_d_n0;
        locals.var_t1_dn2 = assign35940_e50696_d_n2;
        locals.var_t1_dn6 = assign35940_e50696_d_n6;
        locals.var_t1_dn7 = assign35940_e50696_d_n7;
        locals.var_t1_dn10 = assign35940_e50696_d_n10;
        locals.var_t1_dn11 = assign35940_e50696_d_n11;
        locals.var_t1_dn12 = assign35940_e50696_d_n12;
        locals.var_t1_dn17 = assign35940_e50696_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign35950_e50702, assign35950_e50702_d_n0, assign35950_e50702_d_n2, assign35950_e50702_d_n6, assign35950_e50702_d_n7, assign35950_e50702_d_n10, assign35950_e50702_d_n11, assign35950_e50702_d_n12, assign35950_e50702_d_n17,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35950_e50700: f64 = (locals.var_mks_rdrmue__blk1185 / locals.var_t1);
        (assign35950_e50700, (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0__blk1193, locals.var_mu0__blk1193_dn0, locals.var_mu0__blk1193_dn2, locals.var_mu0__blk1193_dn6, locals.var_mu0__blk1193_dn7, locals.var_mu0__blk1193_dn10, locals.var_mu0__blk1193_dn11, locals.var_mu0__blk1193_dn12, locals.var_mu0__blk1193_dn17,)
    }
};
        locals.var_mu0__blk1193 = assign35950_e50702;
        locals.var_mu0__blk1193_dn0 = assign35950_e50702_d_n0;
        locals.var_mu0__blk1193_dn2 = assign35950_e50702_d_n2;
        locals.var_mu0__blk1193_dn6 = assign35950_e50702_d_n6;
        locals.var_mu0__blk1193_dn7 = assign35950_e50702_d_n7;
        locals.var_mu0__blk1193_dn10 = assign35950_e50702_d_n10;
        locals.var_mu0__blk1193_dn11 = assign35950_e50702_d_n11;
        locals.var_mu0__blk1193_dn12 = assign35950_e50702_d_n12;
        locals.var_mu0__blk1193_dn17 = assign35950_e50702_d_n17;
        locals.var_mu0__blk1193_rv = 0.0;

        let (assign35960_e50722, assign35960_e50722_d_n0, assign35960_e50722_d_n2, assign35960_e50722_d_n6, assign35960_e50722_d_n7, assign35960_e50722_d_n10, assign35960_e50722_d_n11, assign35960_e50722_d_n12, assign35960_e50722_d_n17,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35960_e50707: f64 = (0.4 * locals.var_tratio__blk1190);
        let assign35960_e50708: f64 = (1.8 + assign35960_e50707);
        let assign35960_e50711: f64 = (0.1 * locals.var_tratio__blk1190);
        let assign35960_e50713: f64 = (assign35960_e50711 * locals.var_tratio__blk1190);
        let assign35960_e50714: f64 = (assign35960_e50708 + assign35960_e50713);
        let assign35960_e50718: f64 = (1.0 - locals.var_tratio__blk1190);
        let assign35960_e50719: f64 = (p.p270 * assign35960_e50718);
        let assign35960_e50720: f64 = (assign35960_e50714 - assign35960_e50719);
        (assign35960_e50720, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio__blk1190_dn10) + (((0.1 * locals.var_tratio__blk1190_dn10) * locals.var_tratio__blk1190) + (assign35960_e50711 * locals.var_tratio__blk1190_dn10))) - (p.p270 * (-locals.var_tratio__blk1190_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign35960_e50722;
        locals.var_t0_dn0 = assign35960_e50722_d_n0;
        locals.var_t0_dn2 = assign35960_e50722_d_n2;
        locals.var_t0_dn6 = assign35960_e50722_d_n6;
        locals.var_t0_dn7 = assign35960_e50722_d_n7;
        locals.var_t0_dn10 = assign35960_e50722_d_n10;
        locals.var_t0_dn11 = assign35960_e50722_d_n11;
        locals.var_t0_dn12 = assign35960_e50722_d_n12;
        locals.var_t0_dn17 = assign35960_e50722_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign35970_e50728, assign35970_e50728_d_n0, assign35970_e50728_d_n2, assign35970_e50728_d_n6, assign35970_e50728_d_n7, assign35970_e50728_d_n10, assign35970_e50728_d_n11, assign35970_e50728_d_n12, assign35970_e50728_d_n17,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35970_e50726: f64 = (locals.var_mks_rdrvmax__blk1186 / locals.var_t0);
        (assign35970_e50726, (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk1194, locals.var_vmaxe__blk1194_dn0, locals.var_vmaxe__blk1194_dn2, locals.var_vmaxe__blk1194_dn6, locals.var_vmaxe__blk1194_dn7, locals.var_vmaxe__blk1194_dn10, locals.var_vmaxe__blk1194_dn11, locals.var_vmaxe__blk1194_dn12, locals.var_vmaxe__blk1194_dn17,)
    }
};
        locals.var_vmaxe__blk1194 = assign35970_e50728;
        locals.var_vmaxe__blk1194_dn0 = assign35970_e50728_d_n0;
        locals.var_vmaxe__blk1194_dn2 = assign35970_e50728_d_n2;
        locals.var_vmaxe__blk1194_dn6 = assign35970_e50728_d_n6;
        locals.var_vmaxe__blk1194_dn7 = assign35970_e50728_d_n7;
        locals.var_vmaxe__blk1194_dn10 = assign35970_e50728_d_n10;
        locals.var_vmaxe__blk1194_dn11 = assign35970_e50728_d_n11;
        locals.var_vmaxe__blk1194_dn12 = assign35970_e50728_d_n12;
        locals.var_vmaxe__blk1194_dn17 = assign35970_e50728_d_n17;
        locals.var_vmaxe__blk1194_rv = 0.0;

        let (assign35980_e50738, assign35980_e50738_d_n10,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35980_e50734: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign35980_e50735: f64 = (p.p274 * assign35980_e50734);
        let assign35980_e50736: f64 = (locals.var_rrdrbb__blk1187 + assign35980_e50735);
        (assign35980_e50736, (locals.var_rrdrbb__blk1187_dn10 + (p.p274 * locals.var_ttemp_dn10)),)
    } else {
        (locals.var_rrdrbb__blk1187, locals.var_rrdrbb__blk1187_dn10,)
    }
};
        locals.var_rrdrbb__blk1187 = assign35980_e50738;
        locals.var_rrdrbb__blk1187_dn10 = assign35980_e50738_d_n10;
        locals.var_rrdrbb__blk1187_rv = 0.0;

        let (assign35990_e50748,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign35990_e50744: f64 = (locals.var_lgle).powf(p.p280);
        let assign35990_e50745: f64 = (p.p279 / assign35990_e50744);
        let assign35990_e50746: f64 = (1.0 + assign35990_e50745);
        (assign35990_e50746,)
    } else {
        (locals.var_rdrmuele__blk1182,)
    }
};
        locals.var_rdrmuele__blk1182 = assign35990_e50748;
        locals.var_rdrmuele__blk1182_rv = 0.0;

        let (assign36000_e50758,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign36000_e50754: f64 = (locals.var_lgle).powf(p.p278);
        let assign36000_e50755: f64 = (p.p277 / assign36000_e50754);
        let assign36000_e50756: f64 = (1.0 + assign36000_e50755);
        (assign36000_e50756,)
    } else {
        (locals.var_rdrvmaxle__blk1184,)
    }
};
        locals.var_rdrvmaxle__blk1184 = assign36000_e50758;
        locals.var_rdrvmaxle__blk1184_rv = 0.0;

        let (assign36010_e50768,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign36010_e50764: f64 = (locals.var_wg).powf(p.p276);
        let assign36010_e50765: f64 = (p.p275 / assign36010_e50764);
        let assign36010_e50766: f64 = (1.0 + assign36010_e50765);
        (assign36010_e50766,)
    } else {
        (locals.var_rdrvmaxwe__blk1183,)
    }
};
        locals.var_rdrvmaxwe__blk1183 = assign36010_e50768;
        locals.var_rdrvmaxwe__blk1183_rv = 0.0;

        let (assign36020_e50774, assign36020_e50774_d_n0, assign36020_e50774_d_n2, assign36020_e50774_d_n6, assign36020_e50774_d_n7, assign36020_e50774_d_n10, assign36020_e50774_d_n11, assign36020_e50774_d_n12, assign36020_e50774_d_n17,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign36020_e50772: f64 = (locals.var_mu0__blk1193 * locals.var_rdrmuele__blk1182);
        (assign36020_e50772, (locals.var_mu0__blk1193_dn0 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn2 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn6 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn7 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn10 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn11 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn12 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn17 * locals.var_rdrmuele__blk1182),)
    } else {
        (locals.var_mu0__blk1193, locals.var_mu0__blk1193_dn0, locals.var_mu0__blk1193_dn2, locals.var_mu0__blk1193_dn6, locals.var_mu0__blk1193_dn7, locals.var_mu0__blk1193_dn10, locals.var_mu0__blk1193_dn11, locals.var_mu0__blk1193_dn12, locals.var_mu0__blk1193_dn17,)
    }
};
        locals.var_mu0__blk1193 = assign36020_e50774;
        locals.var_mu0__blk1193_dn0 = assign36020_e50774_d_n0;
        locals.var_mu0__blk1193_dn2 = assign36020_e50774_d_n2;
        locals.var_mu0__blk1193_dn6 = assign36020_e50774_d_n6;
        locals.var_mu0__blk1193_dn7 = assign36020_e50774_d_n7;
        locals.var_mu0__blk1193_dn10 = assign36020_e50774_d_n10;
        locals.var_mu0__blk1193_dn11 = assign36020_e50774_d_n11;
        locals.var_mu0__blk1193_dn12 = assign36020_e50774_d_n12;
        locals.var_mu0__blk1193_dn17 = assign36020_e50774_d_n17;
        locals.var_mu0__blk1193_rv = 0.0;

        let (assign36030_e50784, assign36030_e50784_d_n0, assign36030_e50784_d_n2, assign36030_e50784_d_n6, assign36030_e50784_d_n7, assign36030_e50784_d_n10, assign36030_e50784_d_n11, assign36030_e50784_d_n12, assign36030_e50784_d_n17,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign36030_e50778: f64 = (locals.var_vmaxe__blk1194 * locals.var_rdrvmaxwe__blk1183);
        let assign36030_e50780: f64 = (assign36030_e50778 * locals.var_rdrvmaxle__blk1184);
        let assign36030_e50782: f64 = (assign36030_e50780 + 1e-50);
        (assign36030_e50782, ((locals.var_vmaxe__blk1194_dn0 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn2 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn6 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn7 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn10 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn11 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn12 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn17 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184),)
    } else {
        (locals.var_vmaxe__blk1194, locals.var_vmaxe__blk1194_dn0, locals.var_vmaxe__blk1194_dn2, locals.var_vmaxe__blk1194_dn6, locals.var_vmaxe__blk1194_dn7, locals.var_vmaxe__blk1194_dn10, locals.var_vmaxe__blk1194_dn11, locals.var_vmaxe__blk1194_dn12, locals.var_vmaxe__blk1194_dn17,)
    }
};
        locals.var_vmaxe__blk1194 = assign36030_e50784;
        locals.var_vmaxe__blk1194_dn0 = assign36030_e50784_d_n0;
        locals.var_vmaxe__blk1194_dn2 = assign36030_e50784_d_n2;
        locals.var_vmaxe__blk1194_dn6 = assign36030_e50784_d_n6;
        locals.var_vmaxe__blk1194_dn7 = assign36030_e50784_d_n7;
        locals.var_vmaxe__blk1194_dn10 = assign36030_e50784_d_n10;
        locals.var_vmaxe__blk1194_dn11 = assign36030_e50784_d_n11;
        locals.var_vmaxe__blk1194_dn12 = assign36030_e50784_d_n12;
        locals.var_vmaxe__blk1194_dn17 = assign36030_e50784_d_n17;
        locals.var_vmaxe__blk1194_rv = 0.0;

        let (assign36040_e50790, assign36040_e50790_d_n0, assign36040_e50790_d_n2, assign36040_e50790_d_n6, assign36040_e50790_d_n7,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign36040_e50788: f64 = (locals.var_vrdr__blk1189 / locals.var_ldrifte__blk1191);
        (assign36040_e50788, (locals.var_vrdr__blk1189_dn0 / locals.var_ldrifte__blk1191), (locals.var_vrdr__blk1189_dn2 / locals.var_ldrifte__blk1191), (locals.var_vrdr__blk1189_dn6 / locals.var_ldrifte__blk1191), (locals.var_vrdr__blk1189_dn7 / locals.var_ldrifte__blk1191),)
    } else {
        (locals.var_edri__blk1195, locals.var_edri__blk1195_dn0, locals.var_edri__blk1195_dn2, locals.var_edri__blk1195_dn6, locals.var_edri__blk1195_dn7,)
    }
};
        locals.var_edri__blk1195 = assign36040_e50790;
        locals.var_edri__blk1195_dn0 = assign36040_e50790_d_n0;
        locals.var_edri__blk1195_dn2 = assign36040_e50790_d_n2;
        locals.var_edri__blk1195_dn6 = assign36040_e50790_d_n6;
        locals.var_edri__blk1195_dn7 = assign36040_e50790_d_n7;
        locals.var_edri__blk1195_rv = 0.0;

    }
}
