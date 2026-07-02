#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_279(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign76770_e116373, assign76770_e116373_d_n0, assign76770_e116373_d_n2, assign76770_e116373_d_n4, assign76770_e116373_d_n5, assign76770_e116373_d_n6, assign76770_e116373_d_n7, assign76770_e116373_d_n8, assign76770_e116373_d_n9, assign76770_e116373_d_n10, assign76770_e116373_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 != 0.0)) {
        let assign76770_e116370: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76770_e116371: f64 = (locals.var_beta * assign76770_e116370);
        (assign76770_e116371, ((locals.var_beta_dn0 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign76770_e116370) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign76770_e116373;
        locals.var_chi_dn0 = assign76770_e116373_d_n0;
        locals.var_chi_dn2 = assign76770_e116373_d_n2;
        locals.var_chi_dn4 = assign76770_e116373_d_n4;
        locals.var_chi_dn5 = assign76770_e116373_d_n5;
        locals.var_chi_dn6 = assign76770_e116373_d_n6;
        locals.var_chi_dn7 = assign76770_e116373_d_n7;
        locals.var_chi_dn8 = assign76770_e116373_d_n8;
        locals.var_chi_dn9 = assign76770_e116373_d_n9;
        locals.var_chi_dn10 = assign76770_e116373_d_n10;
        locals.var_chi_dn13 = assign76770_e116373_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign76780_e116384, assign76780_e116384_d_n0, assign76780_e116384_d_n2, assign76780_e116384_d_n4, assign76780_e116384_d_n5, assign76780_e116384_d_n6, assign76780_e116384_d_n7, assign76780_e116384_d_n8, assign76780_e116384_d_n9, assign76780_e116384_d_n10, assign76780_e116384_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 != 0.0)) {
        let assign76780_e116381: f64 = (-locals.var_chi);
        let assign76780_e116382: f64 = (assign76780_e116381).exp();
        (assign76780_e116382, (assign76780_e116382 * (-locals.var_chi_dn0)), (assign76780_e116382 * (-locals.var_chi_dn2)), (assign76780_e116382 * (-locals.var_chi_dn4)), (assign76780_e116382 * (-locals.var_chi_dn5)), (assign76780_e116382 * (-locals.var_chi_dn6)), (assign76780_e116382 * (-locals.var_chi_dn7)), (assign76780_e116382 * (-locals.var_chi_dn8)), (assign76780_e116382 * (-locals.var_chi_dn9)), (assign76780_e116382 * (-locals.var_chi_dn10)), (assign76780_e116382 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign76780_e116384;
        locals.var_ty_dn0 = assign76780_e116384_d_n0;
        locals.var_ty_dn2 = assign76780_e116384_d_n2;
        locals.var_ty_dn4 = assign76780_e116384_d_n4;
        locals.var_ty_dn5 = assign76780_e116384_d_n5;
        locals.var_ty_dn6 = assign76780_e116384_d_n6;
        locals.var_ty_dn7 = assign76780_e116384_d_n7;
        locals.var_ty_dn8 = assign76780_e116384_d_n8;
        locals.var_ty_dn9 = assign76780_e116384_d_n9;
        locals.var_ty_dn10 = assign76780_e116384_d_n10;
        locals.var_ty_dn13 = assign76780_e116384_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign76790_e116409, assign76790_e116409_d_n0, assign76790_e116409_d_n2, assign76790_e116409_d_n4, assign76790_e116409_d_n5, assign76790_e116409_d_n6, assign76790_e116409_d_n7, assign76790_e116409_d_n8, assign76790_e116409_d_n9, assign76790_e116409_d_n10, assign76790_e116409_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 != 0.0)) {
        let assign76790_e116396: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76790_e116397: f64 = (locals.var_beta * assign76790_e116396);
        let assign76790_e116399: f64 = (assign76790_e116397 - 1.0);
        let assign76790_e116401: f64 = (assign76790_e116399 + locals.var_ty);
        let assign76790_e116402: f64 = (4.0 * assign76790_e116401);
        let assign76790_e116405: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76790_e116406: f64 = (assign76790_e116402 / assign76790_e116405);
        let assign76790_e116407: f64 = (1.0 + assign76790_e116406);
        (assign76790_e116407, ((((4.0 * (((locals.var_beta_dn0 * assign76790_e116396) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn2 * assign76790_e116396) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn4 * assign76790_e116396) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn5 * assign76790_e116396) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn6 * assign76790_e116396) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn7 * assign76790_e116396) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn8 * assign76790_e116396) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn9 * assign76790_e116396) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn10 * assign76790_e116396) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76790_e116405 * assign76790_e116405)), ((((4.0 * (((locals.var_beta_dn13 * assign76790_e116396) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign76790_e116405) - (assign76790_e116402 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign76790_e116405 * assign76790_e116405)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign76790_e116409;
        locals.var_tx_dn0 = assign76790_e116409_d_n0;
        locals.var_tx_dn2 = assign76790_e116409_d_n2;
        locals.var_tx_dn4 = assign76790_e116409_d_n4;
        locals.var_tx_dn5 = assign76790_e116409_d_n5;
        locals.var_tx_dn6 = assign76790_e116409_d_n6;
        locals.var_tx_dn7 = assign76790_e116409_d_n7;
        locals.var_tx_dn8 = assign76790_e116409_d_n8;
        locals.var_tx_dn9 = assign76790_e116409_d_n9;
        locals.var_tx_dn10 = assign76790_e116409_d_n10;
        locals.var_tx_dn13 = assign76790_e116409_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign76800_e116429, assign76800_e116429_d_n0, assign76800_e116429_d_n2, assign76800_e116429_d_n4, assign76800_e116429_d_n5, assign76800_e116429_d_n6, assign76800_e116429_d_n7, assign76800_e116429_d_n8, assign76800_e116429_d_n9, assign76800_e116429_d_n10, assign76800_e116429_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 != 0.0)) {
        let assign76800_e116419: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76800_e116421: f64 = (assign76800_e116419 / 2.0);
        let assign76800_e116424: f64 = (locals.var_tx).sqrt();
        let assign76800_e116425: f64 = (1.0 - assign76800_e116424);
        let assign76800_e116426: f64 = (assign76800_e116421 * assign76800_e116425);
        let assign76800_e116427: f64 = (locals.var_vgpld + assign76800_e116426);
        (assign76800_e116427, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn0 / (2.0 * assign76800_e116424))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn2 / (2.0 * assign76800_e116424)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn4 / (2.0 * assign76800_e116424))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn5 / (2.0 * assign76800_e116424))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn6 / (2.0 * assign76800_e116424)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn7 / (2.0 * assign76800_e116424)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn8 / (2.0 * assign76800_e116424)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn9 / (2.0 * assign76800_e116424))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn10 / (2.0 * assign76800_e116424))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign76800_e116425) + (assign76800_e116421 * (-(locals.var_tx_dn13 / (2.0 * assign76800_e116424))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign76800_e116429;
        locals.var_ps0_inia_dn0 = assign76800_e116429_d_n0;
        locals.var_ps0_inia_dn2 = assign76800_e116429_d_n2;
        locals.var_ps0_inia_dn4 = assign76800_e116429_d_n4;
        locals.var_ps0_inia_dn5 = assign76800_e116429_d_n5;
        locals.var_ps0_inia_dn6 = assign76800_e116429_d_n6;
        locals.var_ps0_inia_dn7 = assign76800_e116429_d_n7;
        locals.var_ps0_inia_dn8 = assign76800_e116429_d_n8;
        locals.var_ps0_inia_dn9 = assign76800_e116429_d_n9;
        locals.var_ps0_inia_dn10 = assign76800_e116429_d_n10;
        locals.var_ps0_inia_dn13 = assign76800_e116429_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign76810_e116442, assign76810_e116442_d_n0, assign76810_e116442_d_n2, assign76810_e116442_d_n4, assign76810_e116442_d_n5, assign76810_e116442_d_n6, assign76810_e116442_d_n7, assign76810_e116442_d_n8, assign76810_e116442_d_n9, assign76810_e116442_d_n10, assign76810_e116442_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 != 0.0)) {
        let assign76810_e116439: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76810_e116440: f64 = (locals.var_beta * assign76810_e116439);
        (assign76810_e116440, ((locals.var_beta_dn0 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign76810_e116439) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign76810_e116442;
        locals.var_chi_dn0 = assign76810_e116442_d_n0;
        locals.var_chi_dn2 = assign76810_e116442_d_n2;
        locals.var_chi_dn4 = assign76810_e116442_d_n4;
        locals.var_chi_dn5 = assign76810_e116442_d_n5;
        locals.var_chi_dn6 = assign76810_e116442_d_n6;
        locals.var_chi_dn7 = assign76810_e116442_d_n7;
        locals.var_chi_dn8 = assign76810_e116442_d_n8;
        locals.var_chi_dn9 = assign76810_e116442_d_n9;
        locals.var_chi_dn10 = assign76810_e116442_d_n10;
        locals.var_chi_dn13 = assign76810_e116442_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign76830_e116484,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76830_e116463: f64 = (2.0_f64).sqrt();
        let assign76830_e116464: f64 = (9.0 * assign76830_e116463);
        let assign76830_e116465: f64 = (1.0 / assign76830_e116464);
        let assign76830_e116469: f64 = (-3.0);
        let assign76830_e116470: f64 = (assign76830_e116469).exp();
        let assign76830_e116471: f64 = (7.0 * assign76830_e116470);
        let assign76830_e116472: f64 = (5.0 + assign76830_e116471);
        let assign76830_e116476: f64 = (-3.0);
        let assign76830_e116477: f64 = (assign76830_e116476).exp();
        let assign76830_e116478: f64 = (2.0 + assign76830_e116477);
        let assign76830_e116479: f64 = (assign76830_e116478).sqrt();
        let assign76830_e116480: f64 = (54.0 * assign76830_e116479);
        let assign76830_e116481: f64 = (assign76830_e116472 / assign76830_e116480);
        let assign76830_e116482: f64 = (assign76830_e116465 - assign76830_e116481);
        (assign76830_e116482,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign76830_e116484;
        locals.var_ta_rv = 0.0;

        let (assign76840_e116512,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76840_e116494: f64 = (-3.0);
        let assign76840_e116495: f64 = (assign76840_e116494).exp();
        let assign76840_e116496: f64 = (1.0 + assign76840_e116495);
        let assign76840_e116500: f64 = (-3.0);
        let assign76840_e116501: f64 = (assign76840_e116500).exp();
        let assign76840_e116502: f64 = (2.0 + assign76840_e116501);
        let assign76840_e116503: f64 = (assign76840_e116502).sqrt();
        let assign76840_e116504: f64 = (2.0 * assign76840_e116503);
        let assign76840_e116505: f64 = (assign76840_e116496 / assign76840_e116504);
        let assign76840_e116507: f64 = (2.0_f64).sqrt();
        let assign76840_e116509: f64 = (assign76840_e116507 / 3.0);
        let assign76840_e116510: f64 = (assign76840_e116505 - assign76840_e116509);
        (assign76840_e116510,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign76840_e116512;
        locals.var_tb_rv = 0.0;

        let (assign76850_e116531, assign76850_e116531_d_n0, assign76850_e116531_d_n2, assign76850_e116531_d_n4, assign76850_e116531_d_n5, assign76850_e116531_d_n6, assign76850_e116531_d_n7, assign76850_e116531_d_n8, assign76850_e116531_d_n9, assign76850_e116531_d_n10, assign76850_e116531_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76850_e116522: f64 = (2.0_f64).sqrt();
        let assign76850_e116523: f64 = (1.0 / assign76850_e116522);
        let assign76850_e116527: f64 = (locals.var_beta * locals.var_fac1);
        let assign76850_e116528: f64 = (1.0 / assign76850_e116527);
        let assign76850_e116529: f64 = (assign76850_e116523 + assign76850_e116528);
        (assign76850_e116529, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign76850_e116527 * assign76850_e116527))), (-(((locals.var_beta_dn13 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn13)) / (assign76850_e116527 * assign76850_e116527))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn13,)
    }
};
        locals.var_tc = assign76850_e116531;
        locals.var_tc_dn0 = assign76850_e116531_d_n0;
        locals.var_tc_dn2 = assign76850_e116531_d_n2;
        locals.var_tc_dn4 = assign76850_e116531_d_n4;
        locals.var_tc_dn5 = assign76850_e116531_d_n5;
        locals.var_tc_dn6 = assign76850_e116531_d_n6;
        locals.var_tc_dn7 = assign76850_e116531_d_n7;
        locals.var_tc_dn8 = assign76850_e116531_d_n8;
        locals.var_tc_dn9 = assign76850_e116531_d_n9;
        locals.var_tc_dn10 = assign76850_e116531_d_n10;
        locals.var_tc_dn13 = assign76850_e116531_d_n13;
        locals.var_tc_rv = 0.0;

        let (assign76860_e116546, assign76860_e116546_d_n0, assign76860_e116546_d_n2, assign76860_e116546_d_n4, assign76860_e116546_d_n5, assign76860_e116546_d_n6, assign76860_e116546_d_n7, assign76860_e116546_d_n8, assign76860_e116546_d_n9, assign76860_e116546_d_n10, assign76860_e116546_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76860_e116541: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76860_e116542: f64 = (-assign76860_e116541);
        let assign76860_e116544: f64 = (assign76860_e116542 / locals.var_fac1);
        (assign76860_e116544, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn9) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn13) * locals.var_fac1) - (assign76860_e116542 * locals.var_fac1_dn13)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn13,)
    }
};
        locals.var_td = assign76860_e116546;
        locals.var_td_dn0 = assign76860_e116546_d_n0;
        locals.var_td_dn2 = assign76860_e116546_d_n2;
        locals.var_td_dn4 = assign76860_e116546_d_n4;
        locals.var_td_dn5 = assign76860_e116546_d_n5;
        locals.var_td_dn6 = assign76860_e116546_d_n6;
        locals.var_td_dn7 = assign76860_e116546_d_n7;
        locals.var_td_dn8 = assign76860_e116546_d_n8;
        locals.var_td_dn9 = assign76860_e116546_d_n9;
        locals.var_td_dn10 = assign76860_e116546_d_n10;
        locals.var_td_dn13 = assign76860_e116546_d_n13;
        locals.var_td_rv = 0.0;

        let (assign76870_e116584, assign76870_e116584_d_n0, assign76870_e116584_d_n2, assign76870_e116584_d_n4, assign76870_e116584_d_n5, assign76870_e116584_d_n6, assign76870_e116584_d_n7, assign76870_e116584_d_n8, assign76870_e116584_d_n9, assign76870_e116584_d_n10, assign76870_e116584_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76870_e116556: f64 = (locals.var_tb * locals.var_tb);
        let assign76870_e116558: f64 = (assign76870_e116556 * locals.var_tb);
        let assign76870_e116561: f64 = (27.0 * locals.var_ta);
        let assign76870_e116563: f64 = (assign76870_e116561 * locals.var_ta);
        let assign76870_e116565: f64 = (assign76870_e116563 * locals.var_ta);
        let assign76870_e116566: f64 = (assign76870_e116558 / assign76870_e116565);
        let assign76870_e116569: f64 = (locals.var_tb * locals.var_tc);
        let assign76870_e116572: f64 = (6.0 * locals.var_ta);
        let assign76870_e116574: f64 = (assign76870_e116572 * locals.var_ta);
        let assign76870_e116575: f64 = (assign76870_e116569 / assign76870_e116574);
        let assign76870_e116576: f64 = (assign76870_e116566 - assign76870_e116575);
        let assign76870_e116580: f64 = (2.0 * locals.var_ta);
        let assign76870_e116581: f64 = (locals.var_td / assign76870_e116580);
        let assign76870_e116582: f64 = (assign76870_e116576 + assign76870_e116581);
        (assign76870_e116582, ((-((locals.var_tb * locals.var_tc_dn0) / assign76870_e116574)) + (locals.var_td_dn0 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn2) / assign76870_e116574)) + (locals.var_td_dn2 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn4) / assign76870_e116574)) + (locals.var_td_dn4 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn5) / assign76870_e116574)) + (locals.var_td_dn5 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn6) / assign76870_e116574)) + (locals.var_td_dn6 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn7) / assign76870_e116574)) + (locals.var_td_dn7 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn8) / assign76870_e116574)) + (locals.var_td_dn8 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn9) / assign76870_e116574)) + (locals.var_td_dn9 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn10) / assign76870_e116574)) + (locals.var_td_dn10 / assign76870_e116580)), ((-((locals.var_tb * locals.var_tc_dn13) / assign76870_e116574)) + (locals.var_td_dn13 / assign76870_e116580)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn13,)
    }
};
        locals.var_tq = assign76870_e116584;
        locals.var_tq_dn0 = assign76870_e116584_d_n0;
        locals.var_tq_dn2 = assign76870_e116584_d_n2;
        locals.var_tq_dn4 = assign76870_e116584_d_n4;
        locals.var_tq_dn5 = assign76870_e116584_d_n5;
        locals.var_tq_dn6 = assign76870_e116584_d_n6;
        locals.var_tq_dn7 = assign76870_e116584_d_n7;
        locals.var_tq_dn8 = assign76870_e116584_d_n8;
        locals.var_tq_dn9 = assign76870_e116584_d_n9;
        locals.var_tq_dn10 = assign76870_e116584_d_n10;
        locals.var_tq_dn13 = assign76870_e116584_d_n13;
        locals.var_tq_rv = 0.0;

        let (assign76880_e116608, assign76880_e116608_d_n0, assign76880_e116608_d_n2, assign76880_e116608_d_n4, assign76880_e116608_d_n5, assign76880_e116608_d_n6, assign76880_e116608_d_n7, assign76880_e116608_d_n8, assign76880_e116608_d_n9, assign76880_e116608_d_n10, assign76880_e116608_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76880_e116594: f64 = (3.0 * locals.var_ta);
        let assign76880_e116596: f64 = (assign76880_e116594 * locals.var_tc);
        let assign76880_e116599: f64 = (locals.var_tb * locals.var_tb);
        let assign76880_e116600: f64 = (assign76880_e116596 - assign76880_e116599);
        let assign76880_e116603: f64 = (9.0 * locals.var_ta);
        let assign76880_e116605: f64 = (assign76880_e116603 * locals.var_ta);
        let assign76880_e116606: f64 = (assign76880_e116600 / assign76880_e116605);
        (assign76880_e116606, ((assign76880_e116594 * locals.var_tc_dn0) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn2) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn4) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn5) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn6) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn7) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn8) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn9) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn10) / assign76880_e116605), ((assign76880_e116594 * locals.var_tc_dn13) / assign76880_e116605),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn13,)
    }
};
        locals.var_tp = assign76880_e116608;
        locals.var_tp_dn0 = assign76880_e116608_d_n0;
        locals.var_tp_dn2 = assign76880_e116608_d_n2;
        locals.var_tp_dn4 = assign76880_e116608_d_n4;
        locals.var_tp_dn5 = assign76880_e116608_d_n5;
        locals.var_tp_dn6 = assign76880_e116608_d_n6;
        locals.var_tp_dn7 = assign76880_e116608_d_n7;
        locals.var_tp_dn8 = assign76880_e116608_d_n8;
        locals.var_tp_dn9 = assign76880_e116608_d_n9;
        locals.var_tp_dn10 = assign76880_e116608_d_n10;
        locals.var_tp_dn13 = assign76880_e116608_d_n13;
        locals.var_tp_rv = 0.0;

        let (assign76890_e116627, assign76890_e116627_d_n0, assign76890_e116627_d_n2, assign76890_e116627_d_n4, assign76890_e116627_d_n5, assign76890_e116627_d_n6, assign76890_e116627_d_n7, assign76890_e116627_d_n8, assign76890_e116627_d_n9, assign76890_e116627_d_n10, assign76890_e116627_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76890_e116618: f64 = (locals.var_tq * locals.var_tq);
        let assign76890_e116621: f64 = (locals.var_tp * locals.var_tp);
        let assign76890_e116623: f64 = (assign76890_e116621 * locals.var_tp);
        let assign76890_e116624: f64 = (assign76890_e116618 + assign76890_e116623);
        let assign76890_e116625: f64 = (assign76890_e116624).sqrt();
        (assign76890_e116625, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn0))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn2))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn4))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn5))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn6))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn7))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn8))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn9))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn10))) / (2.0 * assign76890_e116625)), ((((locals.var_tq_dn13 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn13)) + ((((locals.var_tp_dn13 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn13)) * locals.var_tp) + (assign76890_e116621 * locals.var_tp_dn13))) / (2.0 * assign76890_e116625)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign76890_e116627;
        locals.var_t5_dn0 = assign76890_e116627_d_n0;
        locals.var_t5_dn2 = assign76890_e116627_d_n2;
        locals.var_t5_dn4 = assign76890_e116627_d_n4;
        locals.var_t5_dn5 = assign76890_e116627_d_n5;
        locals.var_t5_dn6 = assign76890_e116627_d_n6;
        locals.var_t5_dn7 = assign76890_e116627_d_n7;
        locals.var_t5_dn8 = assign76890_e116627_d_n8;
        locals.var_t5_dn9 = assign76890_e116627_d_n9;
        locals.var_t5_dn10 = assign76890_e116627_d_n10;
        locals.var_t5_dn13 = assign76890_e116627_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign76900_e116642, assign76900_e116642_d_n0, assign76900_e116642_d_n2, assign76900_e116642_d_n4, assign76900_e116642_d_n5, assign76900_e116642_d_n6, assign76900_e116642_d_n7, assign76900_e116642_d_n8, assign76900_e116642_d_n9, assign76900_e116642_d_n10, assign76900_e116642_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76900_e116636: f64 = (-locals.var_tq);
        let assign76900_e116638: f64 = (assign76900_e116636 + locals.var_t5);
        let assign76900_e116640: f64 = (assign76900_e116638).powf(0.3333333333333333);
        (assign76900_e116640, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign76900_e116638))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76900_e116638).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn13) + locals.var_t5_dn13))) } } else { (assign76900_e116640 * (0.3333333333333333 * (((-locals.var_tq_dn13) + locals.var_t5_dn13) / assign76900_e116638))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn13,)
    }
};
        locals.var_tu = assign76900_e116642;
        locals.var_tu_dn0 = assign76900_e116642_d_n0;
        locals.var_tu_dn2 = assign76900_e116642_d_n2;
        locals.var_tu_dn4 = assign76900_e116642_d_n4;
        locals.var_tu_dn5 = assign76900_e116642_d_n5;
        locals.var_tu_dn6 = assign76900_e116642_d_n6;
        locals.var_tu_dn7 = assign76900_e116642_d_n7;
        locals.var_tu_dn8 = assign76900_e116642_d_n8;
        locals.var_tu_dn9 = assign76900_e116642_d_n9;
        locals.var_tu_dn10 = assign76900_e116642_d_n10;
        locals.var_tu_dn13 = assign76900_e116642_d_n13;
        locals.var_tu_rv = 0.0;

        let (assign76910_e116657, assign76910_e116657_d_n0, assign76910_e116657_d_n2, assign76910_e116657_d_n4, assign76910_e116657_d_n5, assign76910_e116657_d_n6, assign76910_e116657_d_n7, assign76910_e116657_d_n8, assign76910_e116657_d_n9, assign76910_e116657_d_n10, assign76910_e116657_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76910_e116652: f64 = (locals.var_tq + locals.var_t5);
        let assign76910_e116654: f64 = (assign76910_e116652).powf(0.3333333333333333);
        let assign76910_e116655: f64 = (-assign76910_e116654);
        (assign76910_e116655, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign76910_e116652))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76910_e116652).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn13 + locals.var_t5_dn13))) } } else { (assign76910_e116654 * (0.3333333333333333 * ((locals.var_tq_dn13 + locals.var_t5_dn13) / assign76910_e116652))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn13,)
    }
};
        locals.var_tv = assign76910_e116657;
        locals.var_tv_dn0 = assign76910_e116657_d_n0;
        locals.var_tv_dn2 = assign76910_e116657_d_n2;
        locals.var_tv_dn4 = assign76910_e116657_d_n4;
        locals.var_tv_dn5 = assign76910_e116657_d_n5;
        locals.var_tv_dn6 = assign76910_e116657_d_n6;
        locals.var_tv_dn7 = assign76910_e116657_d_n7;
        locals.var_tv_dn8 = assign76910_e116657_d_n8;
        locals.var_tv_dn9 = assign76910_e116657_d_n9;
        locals.var_tv_dn10 = assign76910_e116657_d_n10;
        locals.var_tv_dn13 = assign76910_e116657_d_n13;
        locals.var_tv_rv = 0.0;

        let (assign76920_e116675, assign76920_e116675_d_n0, assign76920_e116675_d_n2, assign76920_e116675_d_n4, assign76920_e116675_d_n5, assign76920_e116675_d_n6, assign76920_e116675_d_n7, assign76920_e116675_d_n8, assign76920_e116675_d_n9, assign76920_e116675_d_n10, assign76920_e116675_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76920_e116667: f64 = (locals.var_tu + locals.var_tv);
        let assign76920_e116671: f64 = (3.0 * locals.var_ta);
        let assign76920_e116672: f64 = (locals.var_tb / assign76920_e116671);
        let assign76920_e116673: f64 = (assign76920_e116667 - assign76920_e116672);
        (assign76920_e116673, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn13 + locals.var_tv_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign76920_e116675;
        locals.var_chi_dn0 = assign76920_e116675_d_n0;
        locals.var_chi_dn2 = assign76920_e116675_d_n2;
        locals.var_chi_dn4 = assign76920_e116675_d_n4;
        locals.var_chi_dn5 = assign76920_e116675_d_n5;
        locals.var_chi_dn6 = assign76920_e116675_d_n6;
        locals.var_chi_dn7 = assign76920_e116675_d_n7;
        locals.var_chi_dn8 = assign76920_e116675_d_n8;
        locals.var_chi_dn9 = assign76920_e116675_d_n9;
        locals.var_chi_dn10 = assign76920_e116675_d_n10;
        locals.var_chi_dn13 = assign76920_e116675_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign76930_e116689, assign76930_e116689_d_n0, assign76930_e116689_d_n2, assign76930_e116689_d_n4, assign76930_e116689_d_n5, assign76930_e116689_d_n6, assign76930_e116689_d_n7, assign76930_e116689_d_n8, assign76930_e116689_d_n9, assign76930_e116689_d_n10, assign76930_e116689_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76930_e116685: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign76930_e116687: f64 = (assign76930_e116685 - locals.var_vxbgmtcl);
        (assign76930_e116687, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign76930_e116689;
        locals.var_ps0_inia_dn0 = assign76930_e116689_d_n0;
        locals.var_ps0_inia_dn2 = assign76930_e116689_d_n2;
        locals.var_ps0_inia_dn4 = assign76930_e116689_d_n4;
        locals.var_ps0_inia_dn5 = assign76930_e116689_d_n5;
        locals.var_ps0_inia_dn6 = assign76930_e116689_d_n6;
        locals.var_ps0_inia_dn7 = assign76930_e116689_d_n7;
        locals.var_ps0_inia_dn8 = assign76930_e116689_d_n8;
        locals.var_ps0_inia_dn9 = assign76930_e116689_d_n9;
        locals.var_ps0_inia_dn10 = assign76930_e116689_d_n10;
        locals.var_ps0_inia_dn13 = assign76930_e116689_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let assign76940_e116692: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1790 = assign76940_e116692;
        locals.var_guard1790_rv = 0.0;

        let (assign76950_e116705, assign76950_e116705_d_n0, assign76950_e116705_d_n2, assign76950_e116705_d_n4, assign76950_e116705_d_n5, assign76950_e116705_d_n6, assign76950_e116705_d_n7, assign76950_e116705_d_n8, assign76950_e116705_d_n9, assign76950_e116705_d_n10, assign76950_e116705_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign76950_e116701: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76950_e116703: f64 = (assign76950_e116701 + 0.1);
        (assign76950_e116703, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn13,)
    }
};
        locals.var_vgpld_shift = assign76950_e116705;
        locals.var_vgpld_shift_dn0 = assign76950_e116705_d_n0;
        locals.var_vgpld_shift_dn2 = assign76950_e116705_d_n2;
        locals.var_vgpld_shift_dn4 = assign76950_e116705_d_n4;
        locals.var_vgpld_shift_dn5 = assign76950_e116705_d_n5;
        locals.var_vgpld_shift_dn6 = assign76950_e116705_d_n6;
        locals.var_vgpld_shift_dn7 = assign76950_e116705_d_n7;
        locals.var_vgpld_shift_dn8 = assign76950_e116705_d_n8;
        locals.var_vgpld_shift_dn9 = assign76950_e116705_d_n9;
        locals.var_vgpld_shift_dn10 = assign76950_e116705_d_n10;
        locals.var_vgpld_shift_dn13 = assign76950_e116705_d_n13;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign76960_e116716, assign76960_e116716_d_n0, assign76960_e116716_d_n2, assign76960_e116716_d_n4, assign76960_e116716_d_n5, assign76960_e116716_d_n6, assign76960_e116716_d_n7, assign76960_e116716_d_n8, assign76960_e116716_d_n9, assign76960_e116716_d_n10, assign76960_e116716_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign76960_e116714: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign76960_e116714, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign76960_e116716;
        locals.var_cfs1_dn0 = assign76960_e116716_d_n0;
        locals.var_cfs1_dn2 = assign76960_e116716_d_n2;
        locals.var_cfs1_dn4 = assign76960_e116716_d_n4;
        locals.var_cfs1_dn5 = assign76960_e116716_d_n5;
        locals.var_cfs1_dn6 = assign76960_e116716_d_n6;
        locals.var_cfs1_dn7 = assign76960_e116716_d_n7;
        locals.var_cfs1_dn8 = assign76960_e116716_d_n8;
        locals.var_cfs1_dn9 = assign76960_e116716_d_n9;
        locals.var_cfs1_dn10 = assign76960_e116716_d_n10;
        locals.var_cfs1_dn13 = assign76960_e116716_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign76970_e116727, assign76970_e116727_d_n0, assign76970_e116727_d_n2, assign76970_e116727_d_n4, assign76970_e116727_d_n5, assign76970_e116727_d_n6, assign76970_e116727_d_n7, assign76970_e116727_d_n8, assign76970_e116727_d_n9, assign76970_e116727_d_n10, assign76970_e116727_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign76970_e116725: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign76970_e116725, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn13,)
    }
};
        locals.var_gammachi = assign76970_e116727;
        locals.var_gammachi_dn0 = assign76970_e116727_d_n0;
        locals.var_gammachi_dn2 = assign76970_e116727_d_n2;
        locals.var_gammachi_dn4 = assign76970_e116727_d_n4;
        locals.var_gammachi_dn5 = assign76970_e116727_d_n5;
        locals.var_gammachi_dn6 = assign76970_e116727_d_n6;
        locals.var_gammachi_dn7 = assign76970_e116727_d_n7;
        locals.var_gammachi_dn8 = assign76970_e116727_d_n8;
        locals.var_gammachi_dn9 = assign76970_e116727_d_n9;
        locals.var_gammachi_dn10 = assign76970_e116727_d_n10;
        locals.var_gammachi_dn13 = assign76970_e116727_d_n13;
        locals.var_gammachi_rv = 0.0;

        let (assign76980_e116738, assign76980_e116738_d_n0, assign76980_e116738_d_n2, assign76980_e116738_d_n4, assign76980_e116738_d_n5, assign76980_e116738_d_n6, assign76980_e116738_d_n7, assign76980_e116738_d_n8, assign76980_e116738_d_n9, assign76980_e116738_d_n10, assign76980_e116738_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign76980_e116736: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign76980_e116736, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn13 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign76980_e116738;
        locals.var_t0_dn0 = assign76980_e116738_d_n0;
        locals.var_t0_dn2 = assign76980_e116738_d_n2;
        locals.var_t0_dn4 = assign76980_e116738_d_n4;
        locals.var_t0_dn5 = assign76980_e116738_d_n5;
        locals.var_t0_dn6 = assign76980_e116738_d_n6;
        locals.var_t0_dn7 = assign76980_e116738_d_n7;
        locals.var_t0_dn8 = assign76980_e116738_d_n8;
        locals.var_t0_dn9 = assign76980_e116738_d_n9;
        locals.var_t0_dn10 = assign76980_e116738_d_n10;
        locals.var_t0_dn13 = assign76980_e116738_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign76990_e116749, assign76990_e116749_d_n0, assign76990_e116749_d_n2, assign76990_e116749_d_n4, assign76990_e116749_d_n5, assign76990_e116749_d_n6, assign76990_e116749_d_n7, assign76990_e116749_d_n8, assign76990_e116749_d_n9, assign76990_e116749_d_n10, assign76990_e116749_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign76990_e116747: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign76990_e116747, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn13 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn13)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign76990_e116749;
        locals.var_psi_dn0 = assign76990_e116749_d_n0;
        locals.var_psi_dn2 = assign76990_e116749_d_n2;
        locals.var_psi_dn4 = assign76990_e116749_d_n4;
        locals.var_psi_dn5 = assign76990_e116749_d_n5;
        locals.var_psi_dn6 = assign76990_e116749_d_n6;
        locals.var_psi_dn7 = assign76990_e116749_d_n7;
        locals.var_psi_dn8 = assign76990_e116749_d_n8;
        locals.var_psi_dn9 = assign76990_e116749_d_n9;
        locals.var_psi_dn10 = assign76990_e116749_d_n10;
        locals.var_psi_dn13 = assign76990_e116749_d_n13;
        locals.var_psi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_280(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77000_e116774, assign77000_e116774_d_n0, assign77000_e116774_d_n2, assign77000_e116774_d_n4, assign77000_e116774_d_n5, assign77000_e116774_d_n6, assign77000_e116774_d_n7, assign77000_e116774_d_n8, assign77000_e116774_d_n9, assign77000_e116774_d_n10, assign77000_e116774_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign77000_e116758: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77000_e116761: f64 = (locals.var_psi * locals.var_psi);
        let assign77000_e116762: f64 = (assign77000_e116758 + assign77000_e116761);
        let assign77000_e116763: f64 = (assign77000_e116762).ln();
        let assign77000_e116766: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77000_e116767: f64 = (assign77000_e116766).ln();
        let assign77000_e116768: f64 = (assign77000_e116763 - assign77000_e116767);
        let assign77000_e116771: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77000_e116772: f64 = (assign77000_e116768 + assign77000_e116771);
        (assign77000_e116772, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77000_e116762) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77000_e116766)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77000_e116762) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77000_e116766)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77000_e116762) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77000_e116766)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77000_e116762) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77000_e116766)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77000_e116762) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77000_e116766)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77000_e116762) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77000_e116766)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77000_e116762) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77000_e116766)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77000_e116762) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77000_e116766)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77000_e116762) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77000_e116766)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign77000_e116762) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign77000_e116766)) + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign77000_e116774;
        locals.var_chi_1_dn0 = assign77000_e116774_d_n0;
        locals.var_chi_1_dn2 = assign77000_e116774_d_n2;
        locals.var_chi_1_dn4 = assign77000_e116774_d_n4;
        locals.var_chi_1_dn5 = assign77000_e116774_d_n5;
        locals.var_chi_1_dn6 = assign77000_e116774_d_n6;
        locals.var_chi_1_dn7 = assign77000_e116774_d_n7;
        locals.var_chi_1_dn8 = assign77000_e116774_d_n8;
        locals.var_chi_1_dn9 = assign77000_e116774_d_n9;
        locals.var_chi_1_dn10 = assign77000_e116774_d_n10;
        locals.var_chi_1_dn13 = assign77000_e116774_d_n13;
        locals.var_chi_1_rv = 0.0;

        let assign77010_e116777: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1791 = assign77010_e116777;
        locals.var_guard1791_rv = 0.0;

        let (assign77020_e116792, assign77020_e116792_d_n0, assign77020_e116792_d_n2, assign77020_e116792_d_n4, assign77020_e116792_d_n5, assign77020_e116792_d_n6, assign77020_e116792_d_n7, assign77020_e116792_d_n8, assign77020_e116792_d_n9, assign77020_e116792_d_n10, assign77020_e116792_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign77020_e116788: f64 = (locals.var_psi - locals.var_chi_1);
        let assign77020_e116790: f64 = (assign77020_e116788 - 1.0);
        (assign77020_e116790, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign77020_e116792;
        locals.var_tmf1_dn0 = assign77020_e116792_d_n0;
        locals.var_tmf1_dn2 = assign77020_e116792_d_n2;
        locals.var_tmf1_dn4 = assign77020_e116792_d_n4;
        locals.var_tmf1_dn5 = assign77020_e116792_d_n5;
        locals.var_tmf1_dn6 = assign77020_e116792_d_n6;
        locals.var_tmf1_dn7 = assign77020_e116792_d_n7;
        locals.var_tmf1_dn8 = assign77020_e116792_d_n8;
        locals.var_tmf1_dn9 = assign77020_e116792_d_n9;
        locals.var_tmf1_dn10 = assign77020_e116792_d_n10;
        locals.var_tmf1_dn13 = assign77020_e116792_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign77030_e116807, assign77030_e116807_d_n0, assign77030_e116807_d_n2, assign77030_e116807_d_n4, assign77030_e116807_d_n5, assign77030_e116807_d_n6, assign77030_e116807_d_n7, assign77030_e116807_d_n8, assign77030_e116807_d_n9, assign77030_e116807_d_n10, assign77030_e116807_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign77030_e116803: f64 = (4.0 * locals.var_psi);
        let assign77030_e116805: f64 = assign77030_e116803;
        (assign77030_e116805, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn13),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign77030_e116807;
        locals.var_tmf2_dn0 = assign77030_e116807_d_n0;
        locals.var_tmf2_dn2 = assign77030_e116807_d_n2;
        locals.var_tmf2_dn4 = assign77030_e116807_d_n4;
        locals.var_tmf2_dn5 = assign77030_e116807_d_n5;
        locals.var_tmf2_dn6 = assign77030_e116807_d_n6;
        locals.var_tmf2_dn7 = assign77030_e116807_d_n7;
        locals.var_tmf2_dn8 = assign77030_e116807_d_n8;
        locals.var_tmf2_dn9 = assign77030_e116807_d_n9;
        locals.var_tmf2_dn10 = assign77030_e116807_d_n10;
        locals.var_tmf2_dn13 = assign77030_e116807_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign77040_e116824, assign77040_e116824_d_n0, assign77040_e116824_d_n2, assign77040_e116824_d_n4, assign77040_e116824_d_n5, assign77040_e116824_d_n6, assign77040_e116824_d_n7, assign77040_e116824_d_n8, assign77040_e116824_d_n9, assign77040_e116824_d_n10, assign77040_e116824_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let (assign77040_e116822, assign77040_e116822_d_n0, assign77040_e116822_d_n2, assign77040_e116822_d_n4, assign77040_e116822_d_n5, assign77040_e116822_d_n6, assign77040_e116822_d_n7, assign77040_e116822_d_n8, assign77040_e116822_d_n9, assign77040_e116822_d_n10, assign77040_e116822_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign77040_e116821: f64 = (-locals.var_tmf2);
                (assign77040_e116821, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign77040_e116822, assign77040_e116822_d_n0, assign77040_e116822_d_n2, assign77040_e116822_d_n4, assign77040_e116822_d_n5, assign77040_e116822_d_n6, assign77040_e116822_d_n7, assign77040_e116822_d_n8, assign77040_e116822_d_n9, assign77040_e116822_d_n10, assign77040_e116822_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign77040_e116824;
        locals.var_tmf2_dn0 = assign77040_e116824_d_n0;
        locals.var_tmf2_dn2 = assign77040_e116824_d_n2;
        locals.var_tmf2_dn4 = assign77040_e116824_d_n4;
        locals.var_tmf2_dn5 = assign77040_e116824_d_n5;
        locals.var_tmf2_dn6 = assign77040_e116824_d_n6;
        locals.var_tmf2_dn7 = assign77040_e116824_d_n7;
        locals.var_tmf2_dn8 = assign77040_e116824_d_n8;
        locals.var_tmf2_dn9 = assign77040_e116824_d_n9;
        locals.var_tmf2_dn10 = assign77040_e116824_d_n10;
        locals.var_tmf2_dn13 = assign77040_e116824_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign77050_e116840, assign77050_e116840_d_n0, assign77050_e116840_d_n2, assign77050_e116840_d_n4, assign77050_e116840_d_n5, assign77050_e116840_d_n6, assign77050_e116840_d_n7, assign77050_e116840_d_n8, assign77050_e116840_d_n9, assign77050_e116840_d_n10, assign77050_e116840_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign77050_e116835: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign77050_e116837: f64 = (assign77050_e116835 + locals.var_tmf2);
        let assign77050_e116838: f64 = (assign77050_e116837).sqrt();
        (assign77050_e116838, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign77050_e116838)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign77050_e116838)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign77050_e116840;
        locals.var_tmf2_dn0 = assign77050_e116840_d_n0;
        locals.var_tmf2_dn2 = assign77050_e116840_d_n2;
        locals.var_tmf2_dn4 = assign77050_e116840_d_n4;
        locals.var_tmf2_dn5 = assign77050_e116840_d_n5;
        locals.var_tmf2_dn6 = assign77050_e116840_d_n6;
        locals.var_tmf2_dn7 = assign77050_e116840_d_n7;
        locals.var_tmf2_dn8 = assign77050_e116840_d_n8;
        locals.var_tmf2_dn9 = assign77050_e116840_d_n9;
        locals.var_tmf2_dn10 = assign77050_e116840_d_n10;
        locals.var_tmf2_dn13 = assign77050_e116840_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign77060_e116857, assign77060_e116857_d_n0, assign77060_e116857_d_n2, assign77060_e116857_d_n4, assign77060_e116857_d_n5, assign77060_e116857_d_n6, assign77060_e116857_d_n7, assign77060_e116857_d_n8, assign77060_e116857_d_n9, assign77060_e116857_d_n10, assign77060_e116857_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign77060_e116853: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign77060_e116854: f64 = (1.0 + assign77060_e116853);
        let assign77060_e116855: f64 = (0.5 * assign77060_e116854);
        (assign77060_e116855, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77060_e116857;
        locals.var_t1_dn0 = assign77060_e116857_d_n0;
        locals.var_t1_dn2 = assign77060_e116857_d_n2;
        locals.var_t1_dn4 = assign77060_e116857_d_n4;
        locals.var_t1_dn5 = assign77060_e116857_d_n5;
        locals.var_t1_dn6 = assign77060_e116857_d_n6;
        locals.var_t1_dn7 = assign77060_e116857_d_n7;
        locals.var_t1_dn8 = assign77060_e116857_d_n8;
        locals.var_t1_dn9 = assign77060_e116857_d_n9;
        locals.var_t1_dn10 = assign77060_e116857_d_n10;
        locals.var_t1_dn13 = assign77060_e116857_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77070_e116874, assign77070_e116874_d_n0, assign77070_e116874_d_n2, assign77070_e116874_d_n4, assign77070_e116874_d_n5, assign77070_e116874_d_n6, assign77070_e116874_d_n7, assign77070_e116874_d_n8, assign77070_e116874_d_n9, assign77070_e116874_d_n10, assign77070_e116874_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign77070_e116870: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign77070_e116871: f64 = (0.5 * assign77070_e116870);
        let assign77070_e116872: f64 = (locals.var_psi - assign77070_e116871);
        (assign77070_e116872, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign77070_e116874;
        locals.var_chi_1_dn0 = assign77070_e116874_d_n0;
        locals.var_chi_1_dn2 = assign77070_e116874_d_n2;
        locals.var_chi_1_dn4 = assign77070_e116874_d_n4;
        locals.var_chi_1_dn5 = assign77070_e116874_d_n5;
        locals.var_chi_1_dn6 = assign77070_e116874_d_n6;
        locals.var_chi_1_dn7 = assign77070_e116874_d_n7;
        locals.var_chi_1_dn8 = assign77070_e116874_d_n8;
        locals.var_chi_1_dn9 = assign77070_e116874_d_n9;
        locals.var_chi_1_dn10 = assign77070_e116874_d_n10;
        locals.var_chi_1_dn13 = assign77070_e116874_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign77080_e116891, assign77080_e116891_d_n0, assign77080_e116891_d_n2, assign77080_e116891_d_n4, assign77080_e116891_d_n5, assign77080_e116891_d_n6, assign77080_e116891_d_n7, assign77080_e116891_d_n8, assign77080_e116891_d_n9, assign77080_e116891_d_n10, assign77080_e116891_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 == 0.0)) {
        let (assign77080_e116889, assign77080_e116889_d_n0, assign77080_e116889_d_n2, assign77080_e116889_d_n4, assign77080_e116889_d_n5, assign77080_e116889_d_n6, assign77080_e116889_d_n7, assign77080_e116889_d_n8, assign77080_e116889_d_n9, assign77080_e116889_d_n10, assign77080_e116889_d_n13,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
            }
        };
        (assign77080_e116889, assign77080_e116889_d_n0, assign77080_e116889_d_n2, assign77080_e116889_d_n4, assign77080_e116889_d_n5, assign77080_e116889_d_n6, assign77080_e116889_d_n7, assign77080_e116889_d_n8, assign77080_e116889_d_n9, assign77080_e116889_d_n10, assign77080_e116889_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign77080_e116891;
        locals.var_chi_1_dn0 = assign77080_e116891_d_n0;
        locals.var_chi_1_dn2 = assign77080_e116891_d_n2;
        locals.var_chi_1_dn4 = assign77080_e116891_d_n4;
        locals.var_chi_1_dn5 = assign77080_e116891_d_n5;
        locals.var_chi_1_dn6 = assign77080_e116891_d_n6;
        locals.var_chi_1_dn7 = assign77080_e116891_d_n7;
        locals.var_chi_1_dn8 = assign77080_e116891_d_n8;
        locals.var_chi_1_dn9 = assign77080_e116891_d_n9;
        locals.var_chi_1_dn10 = assign77080_e116891_d_n10;
        locals.var_chi_1_dn13 = assign77080_e116891_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign77090_e116905, assign77090_e116905_d_n0, assign77090_e116905_d_n2, assign77090_e116905_d_n4, assign77090_e116905_d_n5, assign77090_e116905_d_n6, assign77090_e116905_d_n7, assign77090_e116905_d_n8, assign77090_e116905_d_n9, assign77090_e116905_d_n10, assign77090_e116905_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let (assign77090_e116903, assign77090_e116903_d_n0, assign77090_e116903_d_n2, assign77090_e116903_d_n4, assign77090_e116903_d_n5, assign77090_e116903_d_n6, assign77090_e116903_d_n7, assign77090_e116903_d_n8, assign77090_e116903_d_n9, assign77090_e116903_d_n10, assign77090_e116903_d_n13,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77090_e116903, assign77090_e116903_d_n0, assign77090_e116903_d_n2, assign77090_e116903_d_n4, assign77090_e116903_d_n5, assign77090_e116903_d_n6, assign77090_e116903_d_n7, assign77090_e116903_d_n8, assign77090_e116903_d_n9, assign77090_e116903_d_n10, assign77090_e116903_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign77090_e116905;
        locals.var_chi_1_dn0 = assign77090_e116905_d_n0;
        locals.var_chi_1_dn2 = assign77090_e116905_d_n2;
        locals.var_chi_1_dn4 = assign77090_e116905_d_n4;
        locals.var_chi_1_dn5 = assign77090_e116905_d_n5;
        locals.var_chi_1_dn6 = assign77090_e116905_d_n6;
        locals.var_chi_1_dn7 = assign77090_e116905_d_n7;
        locals.var_chi_1_dn8 = assign77090_e116905_d_n8;
        locals.var_chi_1_dn9 = assign77090_e116905_d_n9;
        locals.var_chi_1_dn10 = assign77090_e116905_d_n10;
        locals.var_chi_1_dn13 = assign77090_e116905_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign77100_e116916, assign77100_e116916_d_n0, assign77100_e116916_d_n2, assign77100_e116916_d_n4, assign77100_e116916_d_n5, assign77100_e116916_d_n6, assign77100_e116916_d_n7, assign77100_e116916_d_n8, assign77100_e116916_d_n9, assign77100_e116916_d_n10, assign77100_e116916_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign77100_e116914: f64 = (locals.var_psi - locals.var_chi_1);
        (assign77100_e116914, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign77100_e116916;
        locals.var_psi_dn0 = assign77100_e116916_d_n0;
        locals.var_psi_dn2 = assign77100_e116916_d_n2;
        locals.var_psi_dn4 = assign77100_e116916_d_n4;
        locals.var_psi_dn5 = assign77100_e116916_d_n5;
        locals.var_psi_dn6 = assign77100_e116916_d_n6;
        locals.var_psi_dn7 = assign77100_e116916_d_n7;
        locals.var_psi_dn8 = assign77100_e116916_d_n8;
        locals.var_psi_dn9 = assign77100_e116916_d_n9;
        locals.var_psi_dn10 = assign77100_e116916_d_n10;
        locals.var_psi_dn13 = assign77100_e116916_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign77110_e116929, assign77110_e116929_d_n0, assign77110_e116929_d_n2, assign77110_e116929_d_n4, assign77110_e116929_d_n5, assign77110_e116929_d_n6, assign77110_e116929_d_n7, assign77110_e116929_d_n8, assign77110_e116929_d_n9, assign77110_e116929_d_n10, assign77110_e116929_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign77110_e116926: f64 = (locals.var_beta * 0.1);
        let assign77110_e116927: f64 = (locals.var_psi + assign77110_e116926);
        (assign77110_e116927, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn13 + (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign77110_e116929;
        locals.var_psi_dn0 = assign77110_e116929_d_n0;
        locals.var_psi_dn2 = assign77110_e116929_d_n2;
        locals.var_psi_dn4 = assign77110_e116929_d_n4;
        locals.var_psi_dn5 = assign77110_e116929_d_n5;
        locals.var_psi_dn6 = assign77110_e116929_d_n6;
        locals.var_psi_dn7 = assign77110_e116929_d_n7;
        locals.var_psi_dn8 = assign77110_e116929_d_n8;
        locals.var_psi_dn9 = assign77110_e116929_d_n9;
        locals.var_psi_dn10 = assign77110_e116929_d_n10;
        locals.var_psi_dn13 = assign77110_e116929_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign77120_e116950, assign77120_e116950_d_n0, assign77120_e116950_d_n2, assign77120_e116950_d_n4, assign77120_e116950_d_n5, assign77120_e116950_d_n6, assign77120_e116950_d_n7, assign77120_e116950_d_n8, assign77120_e116950_d_n9, assign77120_e116950_d_n10, assign77120_e116950_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign77120_e116938: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77120_e116941: f64 = (locals.var_psi * locals.var_psi);
        let assign77120_e116942: f64 = (assign77120_e116938 + assign77120_e116941);
        let assign77120_e116943: f64 = (assign77120_e116942).ln();
        let assign77120_e116946: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77120_e116947: f64 = (assign77120_e116946).ln();
        let assign77120_e116948: f64 = (assign77120_e116943 - assign77120_e116947);
        (assign77120_e116948, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77120_e116942) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77120_e116946)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77120_e116942) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77120_e116946)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77120_e116942) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77120_e116946)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77120_e116942) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77120_e116946)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77120_e116942) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77120_e116946)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77120_e116942) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77120_e116946)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77120_e116942) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77120_e116946)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77120_e116942) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77120_e116946)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77120_e116942) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77120_e116946)), (((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign77120_e116942) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign77120_e116946)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77120_e116950;
        locals.var_t1_dn0 = assign77120_e116950_d_n0;
        locals.var_t1_dn2 = assign77120_e116950_d_n2;
        locals.var_t1_dn4 = assign77120_e116950_d_n4;
        locals.var_t1_dn5 = assign77120_e116950_d_n5;
        locals.var_t1_dn6 = assign77120_e116950_d_n6;
        locals.var_t1_dn7 = assign77120_e116950_d_n7;
        locals.var_t1_dn8 = assign77120_e116950_d_n8;
        locals.var_t1_dn9 = assign77120_e116950_d_n9;
        locals.var_t1_dn10 = assign77120_e116950_d_n10;
        locals.var_t1_dn13 = assign77120_e116950_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77130_e116963, assign77130_e116963_d_n0, assign77130_e116963_d_n2, assign77130_e116963_d_n4, assign77130_e116963_d_n5, assign77130_e116963_d_n6, assign77130_e116963_d_n7, assign77130_e116963_d_n8, assign77130_e116963_d_n9, assign77130_e116963_d_n10, assign77130_e116963_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign77130_e116960: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77130_e116961: f64 = (locals.var_t1 + assign77130_e116960);
        (assign77130_e116961, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn13 + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign77130_e116963;
        locals.var_chi_b_dn0 = assign77130_e116963_d_n0;
        locals.var_chi_b_dn2 = assign77130_e116963_d_n2;
        locals.var_chi_b_dn4 = assign77130_e116963_d_n4;
        locals.var_chi_b_dn5 = assign77130_e116963_d_n5;
        locals.var_chi_b_dn6 = assign77130_e116963_d_n6;
        locals.var_chi_b_dn7 = assign77130_e116963_d_n7;
        locals.var_chi_b_dn8 = assign77130_e116963_d_n8;
        locals.var_chi_b_dn9 = assign77130_e116963_d_n9;
        locals.var_chi_b_dn10 = assign77130_e116963_d_n10;
        locals.var_chi_b_dn13 = assign77130_e116963_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign77140_e116977, assign77140_e116977_d_n0, assign77140_e116977_d_n2, assign77140_e116977_d_n4, assign77140_e116977_d_n5, assign77140_e116977_d_n6, assign77140_e116977_d_n7, assign77140_e116977_d_n8, assign77140_e116977_d_n9, assign77140_e116977_d_n10, assign77140_e116977_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let (assign77140_e116975, assign77140_e116975_d_n0, assign77140_e116975_d_n2, assign77140_e116975_d_n4, assign77140_e116975_d_n5, assign77140_e116975_d_n6, assign77140_e116975_d_n7, assign77140_e116975_d_n8, assign77140_e116975_d_n9, assign77140_e116975_d_n10, assign77140_e116975_d_n13,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77140_e116975, assign77140_e116975_d_n0, assign77140_e116975_d_n2, assign77140_e116975_d_n4, assign77140_e116975_d_n5, assign77140_e116975_d_n6, assign77140_e116975_d_n7, assign77140_e116975_d_n8, assign77140_e116975_d_n9, assign77140_e116975_d_n10, assign77140_e116975_d_n13,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign77140_e116977;
        locals.var_chi_b_dn0 = assign77140_e116977_d_n0;
        locals.var_chi_b_dn2 = assign77140_e116977_d_n2;
        locals.var_chi_b_dn4 = assign77140_e116977_d_n4;
        locals.var_chi_b_dn5 = assign77140_e116977_d_n5;
        locals.var_chi_b_dn6 = assign77140_e116977_d_n6;
        locals.var_chi_b_dn7 = assign77140_e116977_d_n7;
        locals.var_chi_b_dn8 = assign77140_e116977_d_n8;
        locals.var_chi_b_dn9 = assign77140_e116977_d_n9;
        locals.var_chi_b_dn10 = assign77140_e116977_d_n10;
        locals.var_chi_b_dn13 = assign77140_e116977_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign77150_e116986, assign77150_e116986_d_n0, assign77150_e116986_d_n2, assign77150_e116986_d_n4, assign77150_e116986_d_n5, assign77150_e116986_d_n6, assign77150_e116986_d_n7, assign77150_e116986_d_n8, assign77150_e116986_d_n9, assign77150_e116986_d_n10, assign77150_e116986_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign77150_e116986;
        locals.var_chi_a_dn0 = assign77150_e116986_d_n0;
        locals.var_chi_a_dn2 = assign77150_e116986_d_n2;
        locals.var_chi_a_dn4 = assign77150_e116986_d_n4;
        locals.var_chi_a_dn5 = assign77150_e116986_d_n5;
        locals.var_chi_a_dn6 = assign77150_e116986_d_n6;
        locals.var_chi_a_dn7 = assign77150_e116986_d_n7;
        locals.var_chi_a_dn8 = assign77150_e116986_d_n8;
        locals.var_chi_a_dn9 = assign77150_e116986_d_n9;
        locals.var_chi_a_dn10 = assign77150_e116986_d_n10;
        locals.var_chi_a_dn13 = assign77150_e116986_d_n13;
        locals.var_chi_a_rv = 0.0;

        let assign77160_e116989: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1792 = assign77160_e116989;
        locals.var_guard1792_rv = 0.0;

        let assign77170_e116994: f64 = (0.2 * locals.var_chi_b);
        let assign77170_e116995: f64 = (locals.var_chi_b - assign77170_e116994);
        let assign77170_e116999: f64 = (0.2 * locals.var_chi_b);
        let assign77170_e117002: f64 = if ((locals.var_chi_a > assign77170_e116995) && (assign77170_e116999 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1793 = assign77170_e117002;
        locals.var_guard1793_rv = 0.0;

        let (assign77180_e117021, assign77180_e117021_d_n0, assign77180_e117021_d_n2, assign77180_e117021_d_n4, assign77180_e117021_d_n5, assign77180_e117021_d_n6, assign77180_e117021_d_n7, assign77180_e117021_d_n8, assign77180_e117021_d_n9, assign77180_e117021_d_n10, assign77180_e117021_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77180_e117015: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign77180_e117018: f64 = (0.2 * locals.var_chi_b);
        let assign77180_e117019: f64 = (assign77180_e117015 + assign77180_e117018);
        (assign77180_e117019, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn13 - locals.var_chi_b_dn13) + (0.2 * locals.var_chi_b_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign77180_e117021;
        locals.var_tmf1_dn0 = assign77180_e117021_d_n0;
        locals.var_tmf1_dn2 = assign77180_e117021_d_n2;
        locals.var_tmf1_dn4 = assign77180_e117021_d_n4;
        locals.var_tmf1_dn5 = assign77180_e117021_d_n5;
        locals.var_tmf1_dn6 = assign77180_e117021_d_n6;
        locals.var_tmf1_dn7 = assign77180_e117021_d_n7;
        locals.var_tmf1_dn8 = assign77180_e117021_d_n8;
        locals.var_tmf1_dn9 = assign77180_e117021_d_n9;
        locals.var_tmf1_dn10 = assign77180_e117021_d_n10;
        locals.var_tmf1_dn13 = assign77180_e117021_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign77190_e117036, assign77190_e117036_d_n0, assign77190_e117036_d_n2, assign77190_e117036_d_n4, assign77190_e117036_d_n5, assign77190_e117036_d_n6, assign77190_e117036_d_n7, assign77190_e117036_d_n8, assign77190_e117036_d_n9, assign77190_e117036_d_n10, assign77190_e117036_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77190_e117034: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign77190_e117034, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign77190_e117036;
        locals.var_x2_dn0 = assign77190_e117036_d_n0;
        locals.var_x2_dn2 = assign77190_e117036_d_n2;
        locals.var_x2_dn4 = assign77190_e117036_d_n4;
        locals.var_x2_dn5 = assign77190_e117036_d_n5;
        locals.var_x2_dn6 = assign77190_e117036_d_n6;
        locals.var_x2_dn7 = assign77190_e117036_d_n7;
        locals.var_x2_dn8 = assign77190_e117036_d_n8;
        locals.var_x2_dn9 = assign77190_e117036_d_n9;
        locals.var_x2_dn10 = assign77190_e117036_d_n10;
        locals.var_x2_dn13 = assign77190_e117036_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign77200_e117055, assign77200_e117055_d_n0, assign77200_e117055_d_n2, assign77200_e117055_d_n4, assign77200_e117055_d_n5, assign77200_e117055_d_n6, assign77200_e117055_d_n7, assign77200_e117055_d_n8, assign77200_e117055_d_n9, assign77200_e117055_d_n10, assign77200_e117055_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77200_e117049: f64 = (0.2 * locals.var_chi_b);
        let assign77200_e117052: f64 = (0.2 * locals.var_chi_b);
        let assign77200_e117053: f64 = (assign77200_e117049 * assign77200_e117052);
        (assign77200_e117053, (((0.2 * locals.var_chi_b_dn0) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn13) * assign77200_e117052) + (assign77200_e117049 * (0.2 * locals.var_chi_b_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign77200_e117055;
        locals.var_xmax2_dn0 = assign77200_e117055_d_n0;
        locals.var_xmax2_dn2 = assign77200_e117055_d_n2;
        locals.var_xmax2_dn4 = assign77200_e117055_d_n4;
        locals.var_xmax2_dn5 = assign77200_e117055_d_n5;
        locals.var_xmax2_dn6 = assign77200_e117055_d_n6;
        locals.var_xmax2_dn7 = assign77200_e117055_d_n7;
        locals.var_xmax2_dn8 = assign77200_e117055_d_n8;
        locals.var_xmax2_dn9 = assign77200_e117055_d_n9;
        locals.var_xmax2_dn10 = assign77200_e117055_d_n10;
        locals.var_xmax2_dn13 = assign77200_e117055_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign77210_e117068, assign77210_e117068_d_n0, assign77210_e117068_d_n2, assign77210_e117068_d_n4, assign77210_e117068_d_n5, assign77210_e117068_d_n6, assign77210_e117068_d_n7, assign77210_e117068_d_n8, assign77210_e117068_d_n9, assign77210_e117068_d_n10, assign77210_e117068_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign77210_e117068;
        locals.var_xp_dn0 = assign77210_e117068_d_n0;
        locals.var_xp_dn2 = assign77210_e117068_d_n2;
        locals.var_xp_dn4 = assign77210_e117068_d_n4;
        locals.var_xp_dn5 = assign77210_e117068_d_n5;
        locals.var_xp_dn6 = assign77210_e117068_d_n6;
        locals.var_xp_dn7 = assign77210_e117068_d_n7;
        locals.var_xp_dn8 = assign77210_e117068_d_n8;
        locals.var_xp_dn9 = assign77210_e117068_d_n9;
        locals.var_xp_dn10 = assign77210_e117068_d_n10;
        locals.var_xp_dn13 = assign77210_e117068_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign77220_e117081, assign77220_e117081_d_n0, assign77220_e117081_d_n2, assign77220_e117081_d_n4, assign77220_e117081_d_n5, assign77220_e117081_d_n6, assign77220_e117081_d_n7, assign77220_e117081_d_n8, assign77220_e117081_d_n9, assign77220_e117081_d_n10, assign77220_e117081_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign77220_e117081;
        locals.var_xmp_dn0 = assign77220_e117081_d_n0;
        locals.var_xmp_dn2 = assign77220_e117081_d_n2;
        locals.var_xmp_dn4 = assign77220_e117081_d_n4;
        locals.var_xmp_dn5 = assign77220_e117081_d_n5;
        locals.var_xmp_dn6 = assign77220_e117081_d_n6;
        locals.var_xmp_dn7 = assign77220_e117081_d_n7;
        locals.var_xmp_dn8 = assign77220_e117081_d_n8;
        locals.var_xmp_dn9 = assign77220_e117081_d_n9;
        locals.var_xmp_dn10 = assign77220_e117081_d_n10;
        locals.var_xmp_dn13 = assign77220_e117081_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign77230_e117094,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77230_e117094;
        locals.var_m0_rv = 0.0;

        let (assign77240_e117107,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77240_e117107;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_281(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77250_e117120, assign77250_e117120_d_n0, assign77250_e117120_d_n2, assign77250_e117120_d_n4, assign77250_e117120_d_n5, assign77250_e117120_d_n6, assign77250_e117120_d_n7, assign77250_e117120_d_n8, assign77250_e117120_d_n9, assign77250_e117120_d_n10, assign77250_e117120_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign77250_e117120;
        locals.var_arg_dn0 = assign77250_e117120_d_n0;
        locals.var_arg_dn2 = assign77250_e117120_d_n2;
        locals.var_arg_dn4 = assign77250_e117120_d_n4;
        locals.var_arg_dn5 = assign77250_e117120_d_n5;
        locals.var_arg_dn6 = assign77250_e117120_d_n6;
        locals.var_arg_dn7 = assign77250_e117120_d_n7;
        locals.var_arg_dn8 = assign77250_e117120_d_n8;
        locals.var_arg_dn9 = assign77250_e117120_d_n9;
        locals.var_arg_dn10 = assign77250_e117120_d_n10;
        locals.var_arg_dn13 = assign77250_e117120_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign77260_e117133, assign77260_e117133_d_n0, assign77260_e117133_d_n2, assign77260_e117133_d_n4, assign77260_e117133_d_n5, assign77260_e117133_d_n6, assign77260_e117133_d_n7, assign77260_e117133_d_n8, assign77260_e117133_d_n9, assign77260_e117133_d_n10, assign77260_e117133_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign77260_e117133;
        locals.var_dnm_dn0 = assign77260_e117133_d_n0;
        locals.var_dnm_dn2 = assign77260_e117133_d_n2;
        locals.var_dnm_dn4 = assign77260_e117133_d_n4;
        locals.var_dnm_dn5 = assign77260_e117133_d_n5;
        locals.var_dnm_dn6 = assign77260_e117133_d_n6;
        locals.var_dnm_dn7 = assign77260_e117133_d_n7;
        locals.var_dnm_dn8 = assign77260_e117133_d_n8;
        locals.var_dnm_dn9 = assign77260_e117133_d_n9;
        locals.var_dnm_dn10 = assign77260_e117133_d_n10;
        locals.var_dnm_dn13 = assign77260_e117133_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign77270_e117148, assign77270_e117148_d_n0, assign77270_e117148_d_n2, assign77270_e117148_d_n4, assign77270_e117148_d_n5, assign77270_e117148_d_n6, assign77270_e117148_d_n7, assign77270_e117148_d_n8, assign77270_e117148_d_n9, assign77270_e117148_d_n10, assign77270_e117148_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77270_e117146: f64 = (locals.var_xp * locals.var_x2);
        (assign77270_e117146, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign77270_e117148;
        locals.var_xp_dn0 = assign77270_e117148_d_n0;
        locals.var_xp_dn2 = assign77270_e117148_d_n2;
        locals.var_xp_dn4 = assign77270_e117148_d_n4;
        locals.var_xp_dn5 = assign77270_e117148_d_n5;
        locals.var_xp_dn6 = assign77270_e117148_d_n6;
        locals.var_xp_dn7 = assign77270_e117148_d_n7;
        locals.var_xp_dn8 = assign77270_e117148_d_n8;
        locals.var_xp_dn9 = assign77270_e117148_d_n9;
        locals.var_xp_dn10 = assign77270_e117148_d_n10;
        locals.var_xp_dn13 = assign77270_e117148_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign77280_e117163, assign77280_e117163_d_n0, assign77280_e117163_d_n2, assign77280_e117163_d_n4, assign77280_e117163_d_n5, assign77280_e117163_d_n6, assign77280_e117163_d_n7, assign77280_e117163_d_n8, assign77280_e117163_d_n9, assign77280_e117163_d_n10, assign77280_e117163_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77280_e117161: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77280_e117161, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign77280_e117163;
        locals.var_xmp_dn0 = assign77280_e117163_d_n0;
        locals.var_xmp_dn2 = assign77280_e117163_d_n2;
        locals.var_xmp_dn4 = assign77280_e117163_d_n4;
        locals.var_xmp_dn5 = assign77280_e117163_d_n5;
        locals.var_xmp_dn6 = assign77280_e117163_d_n6;
        locals.var_xmp_dn7 = assign77280_e117163_d_n7;
        locals.var_xmp_dn8 = assign77280_e117163_d_n8;
        locals.var_xmp_dn9 = assign77280_e117163_d_n9;
        locals.var_xmp_dn10 = assign77280_e117163_d_n10;
        locals.var_xmp_dn13 = assign77280_e117163_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign77290_e117178, assign77290_e117178_d_n0, assign77290_e117178_d_n2, assign77290_e117178_d_n4, assign77290_e117178_d_n5, assign77290_e117178_d_n6, assign77290_e117178_d_n7, assign77290_e117178_d_n8, assign77290_e117178_d_n9, assign77290_e117178_d_n10, assign77290_e117178_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77290_e117176: f64 = (locals.var_xp * locals.var_x2);
        (assign77290_e117176, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign77290_e117178;
        locals.var_xp_dn0 = assign77290_e117178_d_n0;
        locals.var_xp_dn2 = assign77290_e117178_d_n2;
        locals.var_xp_dn4 = assign77290_e117178_d_n4;
        locals.var_xp_dn5 = assign77290_e117178_d_n5;
        locals.var_xp_dn6 = assign77290_e117178_d_n6;
        locals.var_xp_dn7 = assign77290_e117178_d_n7;
        locals.var_xp_dn8 = assign77290_e117178_d_n8;
        locals.var_xp_dn9 = assign77290_e117178_d_n9;
        locals.var_xp_dn10 = assign77290_e117178_d_n10;
        locals.var_xp_dn13 = assign77290_e117178_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign77300_e117193, assign77300_e117193_d_n0, assign77300_e117193_d_n2, assign77300_e117193_d_n4, assign77300_e117193_d_n5, assign77300_e117193_d_n6, assign77300_e117193_d_n7, assign77300_e117193_d_n8, assign77300_e117193_d_n9, assign77300_e117193_d_n10, assign77300_e117193_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77300_e117191: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77300_e117191, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign77300_e117193;
        locals.var_xmp_dn0 = assign77300_e117193_d_n0;
        locals.var_xmp_dn2 = assign77300_e117193_d_n2;
        locals.var_xmp_dn4 = assign77300_e117193_d_n4;
        locals.var_xmp_dn5 = assign77300_e117193_d_n5;
        locals.var_xmp_dn6 = assign77300_e117193_d_n6;
        locals.var_xmp_dn7 = assign77300_e117193_d_n7;
        locals.var_xmp_dn8 = assign77300_e117193_d_n8;
        locals.var_xmp_dn9 = assign77300_e117193_d_n9;
        locals.var_xmp_dn10 = assign77300_e117193_d_n10;
        locals.var_xmp_dn13 = assign77300_e117193_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign77310_e117208, assign77310_e117208_d_n0, assign77310_e117208_d_n2, assign77310_e117208_d_n4, assign77310_e117208_d_n5, assign77310_e117208_d_n6, assign77310_e117208_d_n7, assign77310_e117208_d_n8, assign77310_e117208_d_n9, assign77310_e117208_d_n10, assign77310_e117208_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77310_e117206: f64 = (locals.var_xp + locals.var_xmp);
        (assign77310_e117206, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign77310_e117208;
        locals.var_arg_dn0 = assign77310_e117208_d_n0;
        locals.var_arg_dn2 = assign77310_e117208_d_n2;
        locals.var_arg_dn4 = assign77310_e117208_d_n4;
        locals.var_arg_dn5 = assign77310_e117208_d_n5;
        locals.var_arg_dn6 = assign77310_e117208_d_n6;
        locals.var_arg_dn7 = assign77310_e117208_d_n7;
        locals.var_arg_dn8 = assign77310_e117208_d_n8;
        locals.var_arg_dn9 = assign77310_e117208_d_n9;
        locals.var_arg_dn10 = assign77310_e117208_d_n10;
        locals.var_arg_dn13 = assign77310_e117208_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign77320_e117221, assign77320_e117221_d_n0, assign77320_e117221_d_n2, assign77320_e117221_d_n4, assign77320_e117221_d_n5, assign77320_e117221_d_n6, assign77320_e117221_d_n7, assign77320_e117221_d_n8, assign77320_e117221_d_n9, assign77320_e117221_d_n10, assign77320_e117221_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign77320_e117221;
        locals.var_dnm_dn0 = assign77320_e117221_d_n0;
        locals.var_dnm_dn2 = assign77320_e117221_d_n2;
        locals.var_dnm_dn4 = assign77320_e117221_d_n4;
        locals.var_dnm_dn5 = assign77320_e117221_d_n5;
        locals.var_dnm_dn6 = assign77320_e117221_d_n6;
        locals.var_dnm_dn7 = assign77320_e117221_d_n7;
        locals.var_dnm_dn8 = assign77320_e117221_d_n8;
        locals.var_dnm_dn9 = assign77320_e117221_d_n9;
        locals.var_dnm_dn10 = assign77320_e117221_d_n10;
        locals.var_dnm_dn13 = assign77320_e117221_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign77330_e117236: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1794 = assign77330_e117236;
        locals.var_guard1794_rv = 0.0;

        let assign77340_e117239: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1795 = assign77340_e117239;
        locals.var_guard1795_rv = 0.0;

        let (assign77350_e117256,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77350_e117256;
        locals.var_mm_rv = 0.0;

        let assign77360_e117259: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1796 = assign77360_e117259;
        locals.var_guard1796_rv = 0.0;

        let (assign77370_e117279,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 == 0.0)) && (locals.var_guard1796 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77370_e117279;
        locals.var_mm_rv = 0.0;

        let assign77380_e117282: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1797 = assign77380_e117282;
        locals.var_guard1797_rv = 0.0;

        let (assign77390_e117305,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 == 0.0)) && (locals.var_guard1796 == 0.0)) && (locals.var_guard1797 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77390_e117305;
        locals.var_mm_rv = 0.0;

        let assign77400_e117308: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1798 = assign77400_e117308;
        locals.var_guard1798_rv = 0.0;

        let (assign77410_e117334,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 == 0.0)) && (locals.var_guard1796 == 0.0)) && (locals.var_guard1797 == 0.0)) && (locals.var_guard1798 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77410_e117334;
        locals.var_mm_rv = 0.0;

        let (assign77420_e117349,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77420_e117349;
        locals.var_m0_rv = 0.0;

        let mut assign77430_loop_guard: usize = 0;
        while {
            let assign77430_cond_e117365: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign77430_cond_e117365 != 0.0
        } {
            assign77430_loop_guard += 1;
            assert!(assign77430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign77430_body0_e117381, assign77430_body0_e117381_d_n0, assign77430_body0_e117381_d_n2, assign77430_body0_e117381_d_n4, assign77430_body0_e117381_d_n5, assign77430_body0_e117381_d_n6, assign77430_body0_e117381_d_n7, assign77430_body0_e117381_d_n8, assign77430_body0_e117381_d_n9, assign77430_body0_e117381_d_n10, assign77430_body0_e117381_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77430_body0_e117379: f64 = (locals.var_dnm).sqrt();
        (assign77430_body0_e117379, (locals.var_dnm_dn0 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn2 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn4 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn5 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn6 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn7 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn8 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn9 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn10 / (2.0 * assign77430_body0_e117379)), (locals.var_dnm_dn13 / (2.0 * assign77430_body0_e117379)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign77430_body0_e117381;
            locals.var_dnm_dn0 = assign77430_body0_e117381_d_n0;
            locals.var_dnm_dn2 = assign77430_body0_e117381_d_n2;
            locals.var_dnm_dn4 = assign77430_body0_e117381_d_n4;
            locals.var_dnm_dn5 = assign77430_body0_e117381_d_n5;
            locals.var_dnm_dn6 = assign77430_body0_e117381_d_n6;
            locals.var_dnm_dn7 = assign77430_body0_e117381_d_n7;
            locals.var_dnm_dn8 = assign77430_body0_e117381_d_n8;
            locals.var_dnm_dn9 = assign77430_body0_e117381_d_n9;
            locals.var_dnm_dn10 = assign77430_body0_e117381_d_n10;
            locals.var_dnm_dn13 = assign77430_body0_e117381_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign77430_body1_e117398,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77430_body1_e117396: f64 = (locals.var_m0 + 1.0);
        (assign77430_body1_e117396,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign77430_body1_e117398;
            locals.var_m0_rv = 0.0;
        }

        let (assign77440_e117425, assign77440_e117425_d_n0, assign77440_e117425_d_n2, assign77440_e117425_d_n4, assign77440_e117425_d_n5, assign77440_e117425_d_n6, assign77440_e117425_d_n7, assign77440_e117425_d_n8, assign77440_e117425_d_n9, assign77440_e117425_d_n10, assign77440_e117425_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) && (locals.var_guard1794 == 0.0)) {
        let (assign77440_e117423, assign77440_e117423_d_n0, assign77440_e117423_d_n2, assign77440_e117423_d_n4, assign77440_e117423_d_n5, assign77440_e117423_d_n6, assign77440_e117423_d_n7, assign77440_e117423_d_n8, assign77440_e117423_d_n9, assign77440_e117423_d_n10, assign77440_e117423_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign77440_e117420: f64 = (2.0 * 2.0);
                let assign77440_e117421: f64 = (1.0 / assign77440_e117420);
                let assign77440_e117422: f64 = (locals.var_dnm).powf(assign77440_e117421);
                (assign77440_e117422, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn0)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn2)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn4)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn5)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn6)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn7)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn8)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn9)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn10)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77440_e117421) as f64).is_finite() && ((assign77440_e117421) as f64).fract() == 0.0 { if assign77440_e117421 == 0.0 { 0.0 } else { (assign77440_e117421 * ((locals.var_dnm).powf(assign77440_e117421 - 1.0) * locals.var_dnm_dn13)) } } else { (assign77440_e117422 * (assign77440_e117421 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign77440_e117423, assign77440_e117423_d_n0, assign77440_e117423_d_n2, assign77440_e117423_d_n4, assign77440_e117423_d_n5, assign77440_e117423_d_n6, assign77440_e117423_d_n7, assign77440_e117423_d_n8, assign77440_e117423_d_n9, assign77440_e117423_d_n10, assign77440_e117423_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign77440_e117425;
        locals.var_dnm_dn0 = assign77440_e117425_d_n0;
        locals.var_dnm_dn2 = assign77440_e117425_d_n2;
        locals.var_dnm_dn4 = assign77440_e117425_d_n4;
        locals.var_dnm_dn5 = assign77440_e117425_d_n5;
        locals.var_dnm_dn6 = assign77440_e117425_d_n6;
        locals.var_dnm_dn7 = assign77440_e117425_d_n7;
        locals.var_dnm_dn8 = assign77440_e117425_d_n8;
        locals.var_dnm_dn9 = assign77440_e117425_d_n9;
        locals.var_dnm_dn10 = assign77440_e117425_d_n10;
        locals.var_dnm_dn13 = assign77440_e117425_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign77450_e117440, assign77450_e117440_d_n0, assign77450_e117440_d_n2, assign77450_e117440_d_n4, assign77450_e117440_d_n5, assign77450_e117440_d_n6, assign77450_e117440_d_n7, assign77450_e117440_d_n8, assign77450_e117440_d_n9, assign77450_e117440_d_n10, assign77450_e117440_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77450_e117438: f64 = (1.0 / locals.var_dnm);
        (assign77450_e117438, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign77450_e117440;
        locals.var_dnm_dn0 = assign77450_e117440_d_n0;
        locals.var_dnm_dn2 = assign77450_e117440_d_n2;
        locals.var_dnm_dn4 = assign77450_e117440_d_n4;
        locals.var_dnm_dn5 = assign77450_e117440_d_n5;
        locals.var_dnm_dn6 = assign77450_e117440_d_n6;
        locals.var_dnm_dn7 = assign77450_e117440_d_n7;
        locals.var_dnm_dn8 = assign77450_e117440_d_n8;
        locals.var_dnm_dn9 = assign77450_e117440_d_n9;
        locals.var_dnm_dn10 = assign77450_e117440_d_n10;
        locals.var_dnm_dn13 = assign77450_e117440_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign77460_e117459, assign77460_e117459_d_n0, assign77460_e117459_d_n2, assign77460_e117459_d_n4, assign77460_e117459_d_n5, assign77460_e117459_d_n6, assign77460_e117459_d_n7, assign77460_e117459_d_n8, assign77460_e117459_d_n9, assign77460_e117459_d_n10, assign77460_e117459_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77460_e117454: f64 = (0.2 * locals.var_chi_b);
        let assign77460_e117455: f64 = (locals.var_tmf1 * assign77460_e117454);
        let assign77460_e117457: f64 = (assign77460_e117455 * locals.var_dnm);
        (assign77460_e117457, ((((locals.var_tmf1_dn0 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign77460_e117454) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn13))) * locals.var_dnm) + (assign77460_e117455 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign77460_e117459;
        locals.var_tmf0_dn0 = assign77460_e117459_d_n0;
        locals.var_tmf0_dn2 = assign77460_e117459_d_n2;
        locals.var_tmf0_dn4 = assign77460_e117459_d_n4;
        locals.var_tmf0_dn5 = assign77460_e117459_d_n5;
        locals.var_tmf0_dn6 = assign77460_e117459_d_n6;
        locals.var_tmf0_dn7 = assign77460_e117459_d_n7;
        locals.var_tmf0_dn8 = assign77460_e117459_d_n8;
        locals.var_tmf0_dn9 = assign77460_e117459_d_n9;
        locals.var_tmf0_dn10 = assign77460_e117459_d_n10;
        locals.var_tmf0_dn13 = assign77460_e117459_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign77470_e117480, assign77470_e117480_d_n0, assign77470_e117480_d_n2, assign77470_e117480_d_n4, assign77470_e117480_d_n5, assign77470_e117480_d_n6, assign77470_e117480_d_n7, assign77470_e117480_d_n8, assign77470_e117480_d_n9, assign77470_e117480_d_n10, assign77470_e117480_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77470_e117472: f64 = (0.2 * locals.var_chi_b);
        let assign77470_e117474: f64 = (assign77470_e117472 * locals.var_xmp);
        let assign77470_e117476: f64 = (assign77470_e117474 * locals.var_dnm);
        let assign77470_e117478: f64 = (assign77470_e117476 / locals.var_arg);
        (assign77470_e117478, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn0)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn2)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn4)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn5)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn6)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn7)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn8)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn9)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn10)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn13) * locals.var_xmp) + (assign77470_e117472 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign77470_e117474 * locals.var_dnm_dn13)) * locals.var_arg) - (assign77470_e117476 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77470_e117480;
        locals.var_t1_dn0 = assign77470_e117480_d_n0;
        locals.var_t1_dn2 = assign77470_e117480_d_n2;
        locals.var_t1_dn4 = assign77470_e117480_d_n4;
        locals.var_t1_dn5 = assign77470_e117480_d_n5;
        locals.var_t1_dn6 = assign77470_e117480_d_n6;
        locals.var_t1_dn7 = assign77470_e117480_d_n7;
        locals.var_t1_dn8 = assign77470_e117480_d_n8;
        locals.var_t1_dn9 = assign77470_e117480_d_n9;
        locals.var_t1_dn10 = assign77470_e117480_d_n10;
        locals.var_t1_dn13 = assign77470_e117480_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77480_e117499, assign77480_e117499_d_n0, assign77480_e117499_d_n2, assign77480_e117499_d_n4, assign77480_e117499_d_n5, assign77480_e117499_d_n6, assign77480_e117499_d_n7, assign77480_e117499_d_n8, assign77480_e117499_d_n9, assign77480_e117499_d_n10, assign77480_e117499_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77480_e117494: f64 = (0.2 * locals.var_chi_b);
        let assign77480_e117495: f64 = (locals.var_chi_b - assign77480_e117494);
        let assign77480_e117497: f64 = (assign77480_e117495 + locals.var_tmf0);
        (assign77480_e117497, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn13 - (0.2 * locals.var_chi_b_dn13)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign77480_e117499;
        locals.var_chi_dn0 = assign77480_e117499_d_n0;
        locals.var_chi_dn2 = assign77480_e117499_d_n2;
        locals.var_chi_dn4 = assign77480_e117499_d_n4;
        locals.var_chi_dn5 = assign77480_e117499_d_n5;
        locals.var_chi_dn6 = assign77480_e117499_d_n6;
        locals.var_chi_dn7 = assign77480_e117499_d_n7;
        locals.var_chi_dn8 = assign77480_e117499_d_n8;
        locals.var_chi_dn9 = assign77480_e117499_d_n9;
        locals.var_chi_dn10 = assign77480_e117499_d_n10;
        locals.var_chi_dn13 = assign77480_e117499_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign77490_e117512, assign77490_e117512_d_n0, assign77490_e117512_d_n2, assign77490_e117512_d_n4, assign77490_e117512_d_n5, assign77490_e117512_d_n6, assign77490_e117512_d_n7, assign77490_e117512_d_n8, assign77490_e117512_d_n9, assign77490_e117512_d_n10, assign77490_e117512_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77490_e117512;
        locals.var_t1_dn0 = assign77490_e117512_d_n0;
        locals.var_t1_dn2 = assign77490_e117512_d_n2;
        locals.var_t1_dn4 = assign77490_e117512_d_n4;
        locals.var_t1_dn5 = assign77490_e117512_d_n5;
        locals.var_t1_dn6 = assign77490_e117512_d_n6;
        locals.var_t1_dn7 = assign77490_e117512_d_n7;
        locals.var_t1_dn8 = assign77490_e117512_d_n8;
        locals.var_t1_dn9 = assign77490_e117512_d_n9;
        locals.var_t1_dn10 = assign77490_e117512_d_n10;
        locals.var_t1_dn13 = assign77490_e117512_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77500_e117526, assign77500_e117526_d_n0, assign77500_e117526_d_n2, assign77500_e117526_d_n4, assign77500_e117526_d_n5, assign77500_e117526_d_n6, assign77500_e117526_d_n7, assign77500_e117526_d_n8, assign77500_e117526_d_n9, assign77500_e117526_d_n10, assign77500_e117526_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign77500_e117526;
        locals.var_chi_dn0 = assign77500_e117526_d_n0;
        locals.var_chi_dn2 = assign77500_e117526_d_n2;
        locals.var_chi_dn4 = assign77500_e117526_d_n4;
        locals.var_chi_dn5 = assign77500_e117526_d_n5;
        locals.var_chi_dn6 = assign77500_e117526_d_n6;
        locals.var_chi_dn7 = assign77500_e117526_d_n7;
        locals.var_chi_dn8 = assign77500_e117526_d_n8;
        locals.var_chi_dn9 = assign77500_e117526_d_n9;
        locals.var_chi_dn10 = assign77500_e117526_d_n10;
        locals.var_chi_dn13 = assign77500_e117526_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign77510_e117540, assign77510_e117540_d_n0, assign77510_e117540_d_n2, assign77510_e117540_d_n4, assign77510_e117540_d_n5, assign77510_e117540_d_n6, assign77510_e117540_d_n7, assign77510_e117540_d_n8, assign77510_e117540_d_n9, assign77510_e117540_d_n10, assign77510_e117540_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77510_e117540;
        locals.var_t1_dn0 = assign77510_e117540_d_n0;
        locals.var_t1_dn2 = assign77510_e117540_d_n2;
        locals.var_t1_dn4 = assign77510_e117540_d_n4;
        locals.var_t1_dn5 = assign77510_e117540_d_n5;
        locals.var_t1_dn6 = assign77510_e117540_d_n6;
        locals.var_t1_dn7 = assign77510_e117540_d_n7;
        locals.var_t1_dn8 = assign77510_e117540_d_n8;
        locals.var_t1_dn9 = assign77510_e117540_d_n9;
        locals.var_t1_dn10 = assign77510_e117540_d_n10;
        locals.var_t1_dn13 = assign77510_e117540_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77520_e117557, assign77520_e117557_d_n0, assign77520_e117557_d_n2, assign77520_e117557_d_n4, assign77520_e117557_d_n5, assign77520_e117557_d_n6, assign77520_e117557_d_n7, assign77520_e117557_d_n8, assign77520_e117557_d_n9, assign77520_e117557_d_n10, assign77520_e117557_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1792 == 0.0)) {
        let (assign77520_e117555, assign77520_e117555_d_n0, assign77520_e117555_d_n2, assign77520_e117555_d_n4, assign77520_e117555_d_n5, assign77520_e117555_d_n6, assign77520_e117555_d_n7, assign77520_e117555_d_n8, assign77520_e117555_d_n9, assign77520_e117555_d_n10, assign77520_e117555_d_n13,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            }
        };
        (assign77520_e117555, assign77520_e117555_d_n0, assign77520_e117555_d_n2, assign77520_e117555_d_n4, assign77520_e117555_d_n5, assign77520_e117555_d_n6, assign77520_e117555_d_n7, assign77520_e117555_d_n8, assign77520_e117555_d_n9, assign77520_e117555_d_n10, assign77520_e117555_d_n13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign77520_e117557;
        locals.var_chi_dn0 = assign77520_e117557_d_n0;
        locals.var_chi_dn2 = assign77520_e117557_d_n2;
        locals.var_chi_dn4 = assign77520_e117557_d_n4;
        locals.var_chi_dn5 = assign77520_e117557_d_n5;
        locals.var_chi_dn6 = assign77520_e117557_d_n6;
        locals.var_chi_dn7 = assign77520_e117557_d_n7;
        locals.var_chi_dn8 = assign77520_e117557_d_n8;
        locals.var_chi_dn9 = assign77520_e117557_d_n9;
        locals.var_chi_dn10 = assign77520_e117557_d_n10;
        locals.var_chi_dn13 = assign77520_e117557_d_n13;
        locals.var_chi_rv = 0.0;

        let assign77530_e117560: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1799 = assign77530_e117560;
        locals.var_guard1799_rv = 0.0;

        let (assign77540_e117573, assign77540_e117573_d_n0, assign77540_e117573_d_n2, assign77540_e117573_d_n4, assign77540_e117573_d_n5, assign77540_e117573_d_n6, assign77540_e117573_d_n7, assign77540_e117573_d_n8, assign77540_e117573_d_n9, assign77540_e117573_d_n10, assign77540_e117573_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77540_e117569: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77540_e117571: f64 = (assign77540_e117569 - locals.var_vxbgmtcl);
        (assign77540_e117571, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign77540_e117573;
        locals.var_ps0ld_dn0 = assign77540_e117573_d_n0;
        locals.var_ps0ld_dn2 = assign77540_e117573_d_n2;
        locals.var_ps0ld_dn4 = assign77540_e117573_d_n4;
        locals.var_ps0ld_dn5 = assign77540_e117573_d_n5;
        locals.var_ps0ld_dn6 = assign77540_e117573_d_n6;
        locals.var_ps0ld_dn7 = assign77540_e117573_d_n7;
        locals.var_ps0ld_dn8 = assign77540_e117573_d_n8;
        locals.var_ps0ld_dn9 = assign77540_e117573_d_n9;
        locals.var_ps0ld_dn10 = assign77540_e117573_d_n10;
        locals.var_ps0ld_dn13 = assign77540_e117573_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign77550_e117576: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1800 = assign77550_e117576;
        locals.var_guard1800_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_282(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77560_e117589, assign77560_e117589_d_n0, assign77560_e117589_d_n2, assign77560_e117589_d_n4, assign77560_e117589_d_n5, assign77560_e117589_d_n6, assign77560_e117589_d_n7, assign77560_e117589_d_n8, assign77560_e117589_d_n9, assign77560_e117589_d_n10, assign77560_e117589_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 != 0.0)) {
        let assign77560_e117587: f64 = (p.p334 - locals.var_wdep_func);
        (assign77560_e117587, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77560_e117589;
        locals.var_t2_dn0 = assign77560_e117589_d_n0;
        locals.var_t2_dn2 = assign77560_e117589_d_n2;
        locals.var_t2_dn4 = assign77560_e117589_d_n4;
        locals.var_t2_dn5 = assign77560_e117589_d_n5;
        locals.var_t2_dn6 = assign77560_e117589_d_n6;
        locals.var_t2_dn7 = assign77560_e117589_d_n7;
        locals.var_t2_dn8 = assign77560_e117589_d_n8;
        locals.var_t2_dn9 = assign77560_e117589_d_n9;
        locals.var_t2_dn10 = assign77560_e117589_d_n10;
        locals.var_t2_dn13 = assign77560_e117589_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign77570_e117614, assign77570_e117614_d_n0, assign77570_e117614_d_n2, assign77570_e117614_d_n4, assign77570_e117614_d_n5, assign77570_e117614_d_n6, assign77570_e117614_d_n7, assign77570_e117614_d_n8, assign77570_e117614_d_n9, assign77570_e117614_d_n10, assign77570_e117614_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 == 0.0)) {
        let assign77570_e117601: f64 = (locals.var_vdsi + p.p137);
        let assign77570_e117604: f64 = (locals.var_vdsi + p.p137);
        let assign77570_e117605: f64 = (assign77570_e117601 * assign77570_e117604);
        let assign77570_e117608: f64 = (4.0 * 0.1);
        let assign77570_e117610: f64 = (assign77570_e117608 * 0.1);
        let assign77570_e117611: f64 = (assign77570_e117605 + assign77570_e117610);
        let assign77570_e117612: f64 = (assign77570_e117611).sqrt();
        (assign77570_e117612, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign77570_e117604) + (assign77570_e117601 * locals.var_vdsi_dn5)) / (2.0 * assign77570_e117612)), 0.0, (((locals.var_vdsi_dn7 * assign77570_e117604) + (assign77570_e117601 * locals.var_vdsi_dn7)) / (2.0 * assign77570_e117612)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign77570_e117614;
        locals.var_tmf2_dn0 = assign77570_e117614_d_n0;
        locals.var_tmf2_dn2 = assign77570_e117614_d_n2;
        locals.var_tmf2_dn4 = assign77570_e117614_d_n4;
        locals.var_tmf2_dn5 = assign77570_e117614_d_n5;
        locals.var_tmf2_dn6 = assign77570_e117614_d_n6;
        locals.var_tmf2_dn7 = assign77570_e117614_d_n7;
        locals.var_tmf2_dn8 = assign77570_e117614_d_n8;
        locals.var_tmf2_dn9 = assign77570_e117614_d_n9;
        locals.var_tmf2_dn10 = assign77570_e117614_d_n10;
        locals.var_tmf2_dn13 = assign77570_e117614_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign77580_e117634, assign77580_e117634_d_n0, assign77580_e117634_d_n2, assign77580_e117634_d_n4, assign77580_e117634_d_n5, assign77580_e117634_d_n6, assign77580_e117634_d_n7, assign77580_e117634_d_n8, assign77580_e117634_d_n9, assign77580_e117634_d_n10, assign77580_e117634_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 == 0.0)) {
        let assign77580_e117628: f64 = (locals.var_vdsi + p.p137);
        let assign77580_e117630: f64 = (assign77580_e117628 / locals.var_tmf2);
        let assign77580_e117631: f64 = (1.0 + assign77580_e117630);
        let assign77580_e117632: f64 = (0.5 * assign77580_e117631);
        (assign77580_e117632, (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign77580_e117628 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign77580_e117628 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77580_e117628 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign77580_e117634;
        locals.var_t9_dn0 = assign77580_e117634_d_n0;
        locals.var_t9_dn2 = assign77580_e117634_d_n2;
        locals.var_t9_dn4 = assign77580_e117634_d_n4;
        locals.var_t9_dn5 = assign77580_e117634_d_n5;
        locals.var_t9_dn6 = assign77580_e117634_d_n6;
        locals.var_t9_dn7 = assign77580_e117634_d_n7;
        locals.var_t9_dn8 = assign77580_e117634_d_n8;
        locals.var_t9_dn9 = assign77580_e117634_d_n9;
        locals.var_t9_dn10 = assign77580_e117634_d_n10;
        locals.var_t9_dn13 = assign77580_e117634_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign77590_e117652, assign77590_e117652_d_n0, assign77590_e117652_d_n2, assign77590_e117652_d_n4, assign77590_e117652_d_n5, assign77590_e117652_d_n6, assign77590_e117652_d_n7, assign77590_e117652_d_n8, assign77590_e117652_d_n9, assign77590_e117652_d_n10, assign77590_e117652_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 == 0.0)) {
        let assign77590_e117647: f64 = (locals.var_vdsi + p.p137);
        let assign77590_e117649: f64 = (assign77590_e117647 + locals.var_tmf2);
        let assign77590_e117650: f64 = (0.5 * assign77590_e117649);
        (assign77590_e117650, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77590_e117652;
        locals.var_t2_dn0 = assign77590_e117652_d_n0;
        locals.var_t2_dn2 = assign77590_e117652_d_n2;
        locals.var_t2_dn4 = assign77590_e117652_d_n4;
        locals.var_t2_dn5 = assign77590_e117652_d_n5;
        locals.var_t2_dn6 = assign77590_e117652_d_n6;
        locals.var_t2_dn7 = assign77590_e117652_d_n7;
        locals.var_t2_dn8 = assign77590_e117652_d_n8;
        locals.var_t2_dn9 = assign77590_e117652_d_n9;
        locals.var_t2_dn10 = assign77590_e117652_d_n10;
        locals.var_t2_dn13 = assign77590_e117652_d_n13;
        locals.var_t2_rv = 0.0;

        let assign77600_e117655: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1801 = assign77600_e117655;
        locals.var_guard1801_rv = 0.0;

        let (assign77610_e117669, assign77610_e117669_d_n0, assign77610_e117669_d_n2, assign77610_e117669_d_n4, assign77610_e117669_d_n5, assign77610_e117669_d_n6, assign77610_e117669_d_n7, assign77610_e117669_d_n8, assign77610_e117669_d_n9, assign77610_e117669_d_n10, assign77610_e117669_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77610_e117669;
        locals.var_t2_dn0 = assign77610_e117669_d_n0;
        locals.var_t2_dn2 = assign77610_e117669_d_n2;
        locals.var_t2_dn4 = assign77610_e117669_d_n4;
        locals.var_t2_dn5 = assign77610_e117669_d_n5;
        locals.var_t2_dn6 = assign77610_e117669_d_n6;
        locals.var_t2_dn7 = assign77610_e117669_d_n7;
        locals.var_t2_dn8 = assign77610_e117669_d_n8;
        locals.var_t2_dn9 = assign77610_e117669_d_n9;
        locals.var_t2_dn10 = assign77610_e117669_d_n10;
        locals.var_t2_dn13 = assign77610_e117669_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign77620_e117683, assign77620_e117683_d_n0, assign77620_e117683_d_n2, assign77620_e117683_d_n4, assign77620_e117683_d_n5, assign77620_e117683_d_n6, assign77620_e117683_d_n7, assign77620_e117683_d_n8, assign77620_e117683_d_n9, assign77620_e117683_d_n10, assign77620_e117683_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign77620_e117683;
        locals.var_t9_dn0 = assign77620_e117683_d_n0;
        locals.var_t9_dn2 = assign77620_e117683_d_n2;
        locals.var_t9_dn4 = assign77620_e117683_d_n4;
        locals.var_t9_dn5 = assign77620_e117683_d_n5;
        locals.var_t9_dn6 = assign77620_e117683_d_n6;
        locals.var_t9_dn7 = assign77620_e117683_d_n7;
        locals.var_t9_dn8 = assign77620_e117683_d_n8;
        locals.var_t9_dn9 = assign77620_e117683_d_n9;
        locals.var_t9_dn10 = assign77620_e117683_d_n10;
        locals.var_t9_dn13 = assign77620_e117683_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign77630_e117700, assign77630_e117700_d_n0, assign77630_e117700_d_n2, assign77630_e117700_d_n4, assign77630_e117700_d_n5, assign77630_e117700_d_n6, assign77630_e117700_d_n7, assign77630_e117700_d_n8, assign77630_e117700_d_n9, assign77630_e117700_d_n10, assign77630_e117700_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 == 0.0)) {
        let assign77630_e117695: f64 = (locals.var_kjunc * locals.var_t2);
        let assign77630_e117696: f64 = (assign77630_e117695).sqrt();
        let assign77630_e117698: f64 = (assign77630_e117696 * p.p432);
        (assign77630_e117698, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign77630_e117696)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign77630_e117696)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign77630_e117700;
        locals.var_wjunc0_dn0 = assign77630_e117700_d_n0;
        locals.var_wjunc0_dn2 = assign77630_e117700_d_n2;
        locals.var_wjunc0_dn4 = assign77630_e117700_d_n4;
        locals.var_wjunc0_dn5 = assign77630_e117700_d_n5;
        locals.var_wjunc0_dn6 = assign77630_e117700_d_n6;
        locals.var_wjunc0_dn7 = assign77630_e117700_d_n7;
        locals.var_wjunc0_dn8 = assign77630_e117700_d_n8;
        locals.var_wjunc0_dn9 = assign77630_e117700_d_n9;
        locals.var_wjunc0_dn10 = assign77630_e117700_d_n10;
        locals.var_wjunc0_dn13 = assign77630_e117700_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign77640_e117714, assign77640_e117714_d_n0, assign77640_e117714_d_n2, assign77640_e117714_d_n4, assign77640_e117714_d_n5, assign77640_e117714_d_n6, assign77640_e117714_d_n7, assign77640_e117714_d_n8, assign77640_e117714_d_n9, assign77640_e117714_d_n10, assign77640_e117714_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1800 == 0.0)) {
        let assign77640_e117712: f64 = (p.p334 - locals.var_wjunc0);
        (assign77640_e117712, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77640_e117714;
        locals.var_t2_dn0 = assign77640_e117714_d_n0;
        locals.var_t2_dn2 = assign77640_e117714_d_n2;
        locals.var_t2_dn4 = assign77640_e117714_d_n4;
        locals.var_t2_dn5 = assign77640_e117714_d_n5;
        locals.var_t2_dn6 = assign77640_e117714_d_n6;
        locals.var_t2_dn7 = assign77640_e117714_d_n7;
        locals.var_t2_dn8 = assign77640_e117714_d_n8;
        locals.var_t2_dn9 = assign77640_e117714_d_n9;
        locals.var_t2_dn10 = assign77640_e117714_d_n10;
        locals.var_t2_dn13 = assign77640_e117714_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign77650_e117736, assign77650_e117736_d_n0, assign77650_e117736_d_n2, assign77650_e117736_d_n4, assign77650_e117736_d_n5, assign77650_e117736_d_n6, assign77650_e117736_d_n7, assign77650_e117736_d_n8, assign77650_e117736_d_n9, assign77650_e117736_d_n10, assign77650_e117736_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77650_e117723: f64 = (locals.var_t2 * locals.var_t2);
        let assign77650_e117727: f64 = (p.p334 * 0.01);
        let assign77650_e117728: f64 = (4.0 * assign77650_e117727);
        let assign77650_e117731: f64 = (p.p334 * 0.01);
        let assign77650_e117732: f64 = (assign77650_e117728 * assign77650_e117731);
        let assign77650_e117733: f64 = (assign77650_e117723 + assign77650_e117732);
        let assign77650_e117734: f64 = (assign77650_e117733).sqrt();
        (assign77650_e117734, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign77650_e117734)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign77650_e117734)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign77650_e117736;
        locals.var_tmf2_dn0 = assign77650_e117736_d_n0;
        locals.var_tmf2_dn2 = assign77650_e117736_d_n2;
        locals.var_tmf2_dn4 = assign77650_e117736_d_n4;
        locals.var_tmf2_dn5 = assign77650_e117736_d_n5;
        locals.var_tmf2_dn6 = assign77650_e117736_d_n6;
        locals.var_tmf2_dn7 = assign77650_e117736_d_n7;
        locals.var_tmf2_dn8 = assign77650_e117736_d_n8;
        locals.var_tmf2_dn9 = assign77650_e117736_d_n9;
        locals.var_tmf2_dn10 = assign77650_e117736_d_n10;
        locals.var_tmf2_dn13 = assign77650_e117736_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign77660_e117751, assign77660_e117751_d_n0, assign77660_e117751_d_n2, assign77660_e117751_d_n4, assign77660_e117751_d_n5, assign77660_e117751_d_n6, assign77660_e117751_d_n7, assign77660_e117751_d_n8, assign77660_e117751_d_n9, assign77660_e117751_d_n10, assign77660_e117751_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77660_e117747: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign77660_e117748: f64 = (1.0 + assign77660_e117747);
        let assign77660_e117749: f64 = (0.5 * assign77660_e117748);
        (assign77660_e117749, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign77660_e117751;
        locals.var_t9_dn0 = assign77660_e117751_d_n0;
        locals.var_t9_dn2 = assign77660_e117751_d_n2;
        locals.var_t9_dn4 = assign77660_e117751_d_n4;
        locals.var_t9_dn5 = assign77660_e117751_d_n5;
        locals.var_t9_dn6 = assign77660_e117751_d_n6;
        locals.var_t9_dn7 = assign77660_e117751_d_n7;
        locals.var_t9_dn8 = assign77660_e117751_d_n8;
        locals.var_t9_dn9 = assign77660_e117751_d_n9;
        locals.var_t9_dn10 = assign77660_e117751_d_n10;
        locals.var_t9_dn13 = assign77660_e117751_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign77670_e117764, assign77670_e117764_d_n0, assign77670_e117764_d_n2, assign77670_e117764_d_n4, assign77670_e117764_d_n5, assign77670_e117764_d_n6, assign77670_e117764_d_n7, assign77670_e117764_d_n8, assign77670_e117764_d_n9, assign77670_e117764_d_n10, assign77670_e117764_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77670_e117761: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign77670_e117762: f64 = (0.5 * assign77670_e117761);
        (assign77670_e117762, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77670_e117764;
        locals.var_t2_dn0 = assign77670_e117764_d_n0;
        locals.var_t2_dn2 = assign77670_e117764_d_n2;
        locals.var_t2_dn4 = assign77670_e117764_d_n4;
        locals.var_t2_dn5 = assign77670_e117764_d_n5;
        locals.var_t2_dn6 = assign77670_e117764_d_n6;
        locals.var_t2_dn7 = assign77670_e117764_d_n7;
        locals.var_t2_dn8 = assign77670_e117764_d_n8;
        locals.var_t2_dn9 = assign77670_e117764_d_n9;
        locals.var_t2_dn10 = assign77670_e117764_d_n10;
        locals.var_t2_dn13 = assign77670_e117764_d_n13;
        locals.var_t2_rv = 0.0;

        let assign77680_e117767: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1802 = assign77680_e117767;
        locals.var_guard1802_rv = 0.0;

        let (assign77690_e117778, assign77690_e117778_d_n0, assign77690_e117778_d_n2, assign77690_e117778_d_n4, assign77690_e117778_d_n5, assign77690_e117778_d_n6, assign77690_e117778_d_n7, assign77690_e117778_d_n8, assign77690_e117778_d_n9, assign77690_e117778_d_n10, assign77690_e117778_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1802 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77690_e117778;
        locals.var_t2_dn0 = assign77690_e117778_d_n0;
        locals.var_t2_dn2 = assign77690_e117778_d_n2;
        locals.var_t2_dn4 = assign77690_e117778_d_n4;
        locals.var_t2_dn5 = assign77690_e117778_d_n5;
        locals.var_t2_dn6 = assign77690_e117778_d_n6;
        locals.var_t2_dn7 = assign77690_e117778_d_n7;
        locals.var_t2_dn8 = assign77690_e117778_d_n8;
        locals.var_t2_dn9 = assign77690_e117778_d_n9;
        locals.var_t2_dn10 = assign77690_e117778_d_n10;
        locals.var_t2_dn13 = assign77690_e117778_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign77700_e117789, assign77700_e117789_d_n0, assign77700_e117789_d_n2, assign77700_e117789_d_n4, assign77700_e117789_d_n5, assign77700_e117789_d_n6, assign77700_e117789_d_n7, assign77700_e117789_d_n8, assign77700_e117789_d_n9, assign77700_e117789_d_n10, assign77700_e117789_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1802 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign77700_e117789;
        locals.var_t9_dn0 = assign77700_e117789_d_n0;
        locals.var_t9_dn2 = assign77700_e117789_d_n2;
        locals.var_t9_dn4 = assign77700_e117789_d_n4;
        locals.var_t9_dn5 = assign77700_e117789_d_n5;
        locals.var_t9_dn6 = assign77700_e117789_d_n6;
        locals.var_t9_dn7 = assign77700_e117789_d_n7;
        locals.var_t9_dn8 = assign77700_e117789_d_n8;
        locals.var_t9_dn9 = assign77700_e117789_d_n9;
        locals.var_t9_dn10 = assign77700_e117789_d_n10;
        locals.var_t9_dn13 = assign77700_e117789_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign77710_e117798, assign77710_e117798_d_n0, assign77710_e117798_d_n2, assign77710_e117798_d_n4, assign77710_e117798_d_n5, assign77710_e117798_d_n6, assign77710_e117798_d_n7, assign77710_e117798_d_n8, assign77710_e117798_d_n9, assign77710_e117798_d_n10, assign77710_e117798_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign77710_e117798;
        locals.var_ddriftldc_dn0 = assign77710_e117798_d_n0;
        locals.var_ddriftldc_dn2 = assign77710_e117798_d_n2;
        locals.var_ddriftldc_dn4 = assign77710_e117798_d_n4;
        locals.var_ddriftldc_dn5 = assign77710_e117798_d_n5;
        locals.var_ddriftldc_dn6 = assign77710_e117798_d_n6;
        locals.var_ddriftldc_dn7 = assign77710_e117798_d_n7;
        locals.var_ddriftldc_dn8 = assign77710_e117798_d_n8;
        locals.var_ddriftldc_dn9 = assign77710_e117798_d_n9;
        locals.var_ddriftldc_dn10 = assign77710_e117798_d_n10;
        locals.var_ddriftldc_dn13 = assign77710_e117798_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign77720_e117815, assign77720_e117815_d_n0, assign77720_e117815_d_n2, assign77720_e117815_d_n4, assign77720_e117815_d_n5, assign77720_e117815_d_n6, assign77720_e117815_d_n7, assign77720_e117815_d_n8, assign77720_e117815_d_n9, assign77720_e117815_d_n10, assign77720_e117815_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77720_e117807: f64 = (locals.var_q_nsubld__blk1762 * locals.var_ddriftldc);
        let assign77720_e117809: f64 = (assign77720_e117807 * locals.var_ddriftldc);
        let assign77720_e117811: f64 = (assign77720_e117809 / 2.0);
        let assign77720_e117813: f64 = (assign77720_e117811 / 1.034943e-10);
        (assign77720_e117813, (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign77720_e117807 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign77720_e117815;
        locals.var_dphi_sb_dn0 = assign77720_e117815_d_n0;
        locals.var_dphi_sb_dn2 = assign77720_e117815_d_n2;
        locals.var_dphi_sb_dn4 = assign77720_e117815_d_n4;
        locals.var_dphi_sb_dn5 = assign77720_e117815_d_n5;
        locals.var_dphi_sb_dn6 = assign77720_e117815_d_n6;
        locals.var_dphi_sb_dn7 = assign77720_e117815_d_n7;
        locals.var_dphi_sb_dn8 = assign77720_e117815_d_n8;
        locals.var_dphi_sb_dn9 = assign77720_e117815_d_n9;
        locals.var_dphi_sb_dn10 = assign77720_e117815_d_n10;
        locals.var_dphi_sb_dn13 = assign77720_e117815_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign77730_e117829, assign77730_e117829_d_n0, assign77730_e117829_d_n2, assign77730_e117829_d_n4, assign77730_e117829_d_n5, assign77730_e117829_d_n6, assign77730_e117829_d_n7, assign77730_e117829_d_n8, assign77730_e117829_d_n9, assign77730_e117829_d_n10, assign77730_e117829_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77730_e117824: f64 = (2.0 * locals.var_beta);
        let assign77730_e117826: f64 = (assign77730_e117824 * locals.var_dphi_sb);
        let assign77730_e117827: f64 = (assign77730_e117826).sqrt();
        (assign77730_e117827, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn0)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn2)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn4)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn5)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn6)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn7)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn8)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn9)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn10)) / (2.0 * assign77730_e117827)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign77730_e117824 * locals.var_dphi_sb_dn13)) / (2.0 * assign77730_e117827)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign77730_e117829;
        locals.var_t0_dn0 = assign77730_e117829_d_n0;
        locals.var_t0_dn2 = assign77730_e117829_d_n2;
        locals.var_t0_dn4 = assign77730_e117829_d_n4;
        locals.var_t0_dn5 = assign77730_e117829_d_n5;
        locals.var_t0_dn6 = assign77730_e117829_d_n6;
        locals.var_t0_dn7 = assign77730_e117829_d_n7;
        locals.var_t0_dn8 = assign77730_e117829_d_n8;
        locals.var_t0_dn9 = assign77730_e117829_d_n9;
        locals.var_t0_dn10 = assign77730_e117829_d_n10;
        locals.var_t0_dn13 = assign77730_e117829_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign77740_e117845, assign77740_e117845_d_n0, assign77740_e117845_d_n2, assign77740_e117845_d_n4, assign77740_e117845_d_n5, assign77740_e117845_d_n6, assign77740_e117845_d_n7, assign77740_e117845_d_n8, assign77740_e117845_d_n9, assign77740_e117845_d_n10, assign77740_e117845_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77740_e117837: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77740_e117839: f64 = (-locals.var_t0);
        let assign77740_e117840: f64 = { let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77740_e117841: f64 = (assign77740_e117837 + assign77740_e117840);
        let assign77740_e117843: f64 = (assign77740_e117841 / 2.0);
        (assign77740_e117843, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign77740_e117839; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77740_e117845;
        locals.var_t1_dn0 = assign77740_e117845_d_n0;
        locals.var_t1_dn2 = assign77740_e117845_d_n2;
        locals.var_t1_dn4 = assign77740_e117845_d_n4;
        locals.var_t1_dn5 = assign77740_e117845_d_n5;
        locals.var_t1_dn6 = assign77740_e117845_d_n6;
        locals.var_t1_dn7 = assign77740_e117845_d_n7;
        locals.var_t1_dn8 = assign77740_e117845_d_n8;
        locals.var_t1_dn9 = assign77740_e117845_d_n9;
        locals.var_t1_dn10 = assign77740_e117845_d_n10;
        locals.var_t1_dn13 = assign77740_e117845_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77750_e117857, assign77750_e117857_d_n0, assign77750_e117857_d_n2, assign77750_e117857_d_n4, assign77750_e117857_d_n5, assign77750_e117857_d_n6, assign77750_e117857_d_n7, assign77750_e117857_d_n8, assign77750_e117857_d_n9, assign77750_e117857_d_n10, assign77750_e117857_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77750_e117853: f64 = (locals.var_t1).ln();
        let assign77750_e117855: f64 = (assign77750_e117853 / locals.var_dphi_sb);
        (assign77750_e117855, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign77750_e117853 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign77750_e117857;
        locals.var_c_sb_dn0 = assign77750_e117857_d_n0;
        locals.var_c_sb_dn2 = assign77750_e117857_d_n2;
        locals.var_c_sb_dn4 = assign77750_e117857_d_n4;
        locals.var_c_sb_dn5 = assign77750_e117857_d_n5;
        locals.var_c_sb_dn6 = assign77750_e117857_d_n6;
        locals.var_c_sb_dn7 = assign77750_e117857_d_n7;
        locals.var_c_sb_dn8 = assign77750_e117857_d_n8;
        locals.var_c_sb_dn9 = assign77750_e117857_d_n9;
        locals.var_c_sb_dn10 = assign77750_e117857_d_n10;
        locals.var_c_sb_dn13 = assign77750_e117857_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign77760_e117868, assign77760_e117868_d_n0, assign77760_e117868_d_n2, assign77760_e117868_d_n4, assign77760_e117868_d_n5, assign77760_e117868_d_n6, assign77760_e117868_d_n7, assign77760_e117868_d_n8, assign77760_e117868_d_n9, assign77760_e117868_d_n10, assign77760_e117868_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77760_e117866: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign77760_e117866, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign77760_e117868;
        locals.var_ps0ld_vxb_dn0 = assign77760_e117868_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign77760_e117868_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign77760_e117868_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign77760_e117868_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign77760_e117868_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign77760_e117868_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign77760_e117868_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign77760_e117868_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign77760_e117868_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign77760_e117868_d_n13;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign77770_e117881, assign77770_e117881_d_n0, assign77770_e117881_d_n2, assign77770_e117881_d_n4, assign77770_e117881_d_n5, assign77770_e117881_d_n6, assign77770_e117881_d_n7, assign77770_e117881_d_n8, assign77770_e117881_d_n9, assign77770_e117881_d_n10, assign77770_e117881_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77770_e117878: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign77770_e117879: f64 = (locals.var_c_sb * assign77770_e117878);
        (assign77770_e117879, ((locals.var_c_sb_dn0 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign77770_e117878) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign77770_e117881;
        locals.var_ty_dn0 = assign77770_e117881_d_n0;
        locals.var_ty_dn2 = assign77770_e117881_d_n2;
        locals.var_ty_dn4 = assign77770_e117881_d_n4;
        locals.var_ty_dn5 = assign77770_e117881_d_n5;
        locals.var_ty_dn6 = assign77770_e117881_d_n6;
        locals.var_ty_dn7 = assign77770_e117881_d_n7;
        locals.var_ty_dn8 = assign77770_e117881_d_n8;
        locals.var_ty_dn9 = assign77770_e117881_d_n9;
        locals.var_ty_dn10 = assign77770_e117881_d_n10;
        locals.var_ty_dn13 = assign77770_e117881_d_n13;
        locals.var_ty_rv = 0.0;

        let assign77780_e117884: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1803 = assign77780_e117884;
        locals.var_guard1803_rv = 0.0;

        let (assign77790_e117896, assign77790_e117896_d_n0, assign77790_e117896_d_n2, assign77790_e117896_d_n4, assign77790_e117896_d_n5, assign77790_e117896_d_n6, assign77790_e117896_d_n7, assign77790_e117896_d_n8, assign77790_e117896_d_n9, assign77790_e117896_d_n10, assign77790_e117896_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77790_e117894: f64 = (locals.var_ty).exp();
        (assign77790_e117894, (assign77790_e117894 * locals.var_ty_dn0), (assign77790_e117894 * locals.var_ty_dn2), (assign77790_e117894 * locals.var_ty_dn4), (assign77790_e117894 * locals.var_ty_dn5), (assign77790_e117894 * locals.var_ty_dn6), (assign77790_e117894 * locals.var_ty_dn7), (assign77790_e117894 * locals.var_ty_dn8), (assign77790_e117894 * locals.var_ty_dn9), (assign77790_e117894 * locals.var_ty_dn10), (assign77790_e117894 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77790_e117896;
        locals.var_t1_dn0 = assign77790_e117896_d_n0;
        locals.var_t1_dn2 = assign77790_e117896_d_n2;
        locals.var_t1_dn4 = assign77790_e117896_d_n4;
        locals.var_t1_dn5 = assign77790_e117896_d_n5;
        locals.var_t1_dn6 = assign77790_e117896_d_n6;
        locals.var_t1_dn7 = assign77790_e117896_d_n7;
        locals.var_t1_dn8 = assign77790_e117896_d_n8;
        locals.var_t1_dn9 = assign77790_e117896_d_n9;
        locals.var_t1_dn10 = assign77790_e117896_d_n10;
        locals.var_t1_dn13 = assign77790_e117896_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77800_e117911, assign77800_e117911_d_n0, assign77800_e117911_d_n2, assign77800_e117911_d_n4, assign77800_e117911_d_n5, assign77800_e117911_d_n6, assign77800_e117911_d_n7, assign77800_e117911_d_n8, assign77800_e117911_d_n9, assign77800_e117911_d_n10, assign77800_e117911_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77800_e117906: f64 = (-locals.var_c_sb);
        let assign77800_e117908: f64 = (assign77800_e117906 * locals.var_dphi_sb);
        let assign77800_e117909: f64 = (assign77800_e117908).exp();
        (assign77800_e117909, (assign77800_e117909 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn0))), (assign77800_e117909 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn2))), (assign77800_e117909 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn4))), (assign77800_e117909 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn5))), (assign77800_e117909 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn6))), (assign77800_e117909 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn7))), (assign77800_e117909 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn8))), (assign77800_e117909 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn9))), (assign77800_e117909 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn10))), (assign77800_e117909 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign77800_e117906 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign77800_e117911;
        locals.var_t0_dn0 = assign77800_e117911_d_n0;
        locals.var_t0_dn2 = assign77800_e117911_d_n2;
        locals.var_t0_dn4 = assign77800_e117911_d_n4;
        locals.var_t0_dn5 = assign77800_e117911_d_n5;
        locals.var_t0_dn6 = assign77800_e117911_d_n6;
        locals.var_t0_dn7 = assign77800_e117911_d_n7;
        locals.var_t0_dn8 = assign77800_e117911_d_n8;
        locals.var_t0_dn9 = assign77800_e117911_d_n9;
        locals.var_t0_dn10 = assign77800_e117911_d_n10;
        locals.var_t0_dn13 = assign77800_e117911_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_283(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77810_e117924, assign77810_e117924_d_n0, assign77810_e117924_d_n2, assign77810_e117924_d_n4, assign77810_e117924_d_n5, assign77810_e117924_d_n6, assign77810_e117924_d_n7, assign77810_e117924_d_n8, assign77810_e117924_d_n9, assign77810_e117924_d_n10, assign77810_e117924_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77810_e117922: f64 = (locals.var_t1 - locals.var_t0);
        (assign77810_e117922, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77810_e117924;
        locals.var_t2_dn0 = assign77810_e117924_d_n0;
        locals.var_t2_dn2 = assign77810_e117924_d_n2;
        locals.var_t2_dn4 = assign77810_e117924_d_n4;
        locals.var_t2_dn5 = assign77810_e117924_d_n5;
        locals.var_t2_dn6 = assign77810_e117924_d_n6;
        locals.var_t2_dn7 = assign77810_e117924_d_n7;
        locals.var_t2_dn8 = assign77810_e117924_d_n8;
        locals.var_t2_dn9 = assign77810_e117924_d_n9;
        locals.var_t2_dn10 = assign77810_e117924_d_n10;
        locals.var_t2_dn13 = assign77810_e117924_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign77820_e117940, assign77820_e117940_d_n0, assign77820_e117940_d_n2, assign77820_e117940_d_n4, assign77820_e117940_d_n5, assign77820_e117940_d_n6, assign77820_e117940_d_n7, assign77820_e117940_d_n8, assign77820_e117940_d_n9, assign77820_e117940_d_n10, assign77820_e117940_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77820_e117935: f64 = (1.0 + locals.var_t2);
        let assign77820_e117936: f64 = (assign77820_e117935).ln();
        let assign77820_e117938: f64 = (assign77820_e117936 / locals.var_c_sb);
        (assign77820_e117938, ((((locals.var_t2_dn0 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign77820_e117935) * locals.var_c_sb) - (assign77820_e117936 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign77820_e117940;
        locals.var_phi_b_dn0 = assign77820_e117940_d_n0;
        locals.var_phi_b_dn2 = assign77820_e117940_d_n2;
        locals.var_phi_b_dn4 = assign77820_e117940_d_n4;
        locals.var_phi_b_dn5 = assign77820_e117940_d_n5;
        locals.var_phi_b_dn6 = assign77820_e117940_d_n6;
        locals.var_phi_b_dn7 = assign77820_e117940_d_n7;
        locals.var_phi_b_dn8 = assign77820_e117940_d_n8;
        locals.var_phi_b_dn9 = assign77820_e117940_d_n9;
        locals.var_phi_b_dn10 = assign77820_e117940_d_n10;
        locals.var_phi_b_dn13 = assign77820_e117940_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign77830_e117954, assign77830_e117954_d_n0, assign77830_e117954_d_n2, assign77830_e117954_d_n4, assign77830_e117954_d_n5, assign77830_e117954_d_n6, assign77830_e117954_d_n7, assign77830_e117954_d_n8, assign77830_e117954_d_n9, assign77830_e117954_d_n10, assign77830_e117954_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1803 == 0.0)) {
        let assign77830_e117952: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign77830_e117952, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign77830_e117954;
        locals.var_phi_b_dn0 = assign77830_e117954_d_n0;
        locals.var_phi_b_dn2 = assign77830_e117954_d_n2;
        locals.var_phi_b_dn4 = assign77830_e117954_d_n4;
        locals.var_phi_b_dn5 = assign77830_e117954_d_n5;
        locals.var_phi_b_dn6 = assign77830_e117954_d_n6;
        locals.var_phi_b_dn7 = assign77830_e117954_d_n7;
        locals.var_phi_b_dn8 = assign77830_e117954_d_n8;
        locals.var_phi_b_dn9 = assign77830_e117954_d_n9;
        locals.var_phi_b_dn10 = assign77830_e117954_d_n10;
        locals.var_phi_b_dn13 = assign77830_e117954_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign77840_e117965, assign77840_e117965_d_n0, assign77840_e117965_d_n2, assign77840_e117965_d_n4, assign77840_e117965_d_n5, assign77840_e117965_d_n6, assign77840_e117965_d_n7, assign77840_e117965_d_n8, assign77840_e117965_d_n9, assign77840_e117965_d_n10, assign77840_e117965_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        let assign77840_e117963: f64 = (locals.var_beta * locals.var_phi_b);
        (assign77840_e117963, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
        locals.var_chib = assign77840_e117965;
        locals.var_chib_dn0 = assign77840_e117965_d_n0;
        locals.var_chib_dn2 = assign77840_e117965_d_n2;
        locals.var_chib_dn4 = assign77840_e117965_d_n4;
        locals.var_chib_dn5 = assign77840_e117965_d_n5;
        locals.var_chib_dn6 = assign77840_e117965_d_n6;
        locals.var_chib_dn7 = assign77840_e117965_d_n7;
        locals.var_chib_dn8 = assign77840_e117965_d_n8;
        locals.var_chib_dn9 = assign77840_e117965_d_n9;
        locals.var_chib_dn10 = assign77840_e117965_d_n10;
        locals.var_chib_dn13 = assign77840_e117965_d_n13;
        locals.var_chib_rv = 0.0;

        let assign77850_e117969: f64 = (locals.var_chi / 100.0);
        let assign77850_e117974: f64 = if ((locals.var_chib > assign77850_e117969) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1804 = assign77850_e117974;
        locals.var_guard1804_rv = 0.0;

        let (assign77860_e117987,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1804 != 0.0)) {
        let assign77860_e117985: f64 = (locals.var_flg_fd_mode__blk1768 + 1.0);
        (assign77860_e117985,)
    } else {
        (locals.var_flg_fd_mode__blk1768,)
    }
};
        locals.var_flg_fd_mode__blk1768 = assign77860_e117987;
        locals.var_flg_fd_mode__blk1768_rv = 0.0;

        let (assign77870_e117998, assign77870_e117998_d_n0, assign77870_e117998_d_n2, assign77870_e117998_d_n4, assign77870_e117998_d_n5, assign77870_e117998_d_n6, assign77870_e117998_d_n7, assign77870_e117998_d_n8, assign77870_e117998_d_n9, assign77870_e117998_d_n10, assign77870_e117998_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1799 != 0.0)) && (locals.var_guard1804 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign77870_e117998;
        locals.var_chi_dn0 = assign77870_e117998_d_n0;
        locals.var_chi_dn2 = assign77870_e117998_d_n2;
        locals.var_chi_dn4 = assign77870_e117998_d_n4;
        locals.var_chi_dn5 = assign77870_e117998_d_n5;
        locals.var_chi_dn6 = assign77870_e117998_d_n6;
        locals.var_chi_dn7 = assign77870_e117998_d_n7;
        locals.var_chi_dn8 = assign77870_e117998_d_n8;
        locals.var_chi_dn9 = assign77870_e117998_d_n9;
        locals.var_chi_dn10 = assign77870_e117998_d_n10;
        locals.var_chi_dn13 = assign77870_e117998_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign77880_e118009, assign77880_e118009_d_n0, assign77880_e118009_d_n2, assign77880_e118009_d_n4, assign77880_e118009_d_n5, assign77880_e118009_d_n6, assign77880_e118009_d_n7, assign77880_e118009_d_n8, assign77880_e118009_d_n9, assign77880_e118009_d_n10, assign77880_e118009_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) {
        let assign77880_e118005: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77880_e118007: f64 = (assign77880_e118005 - locals.var_vxbgmtcl);
        (assign77880_e118007, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign77880_e118009;
        locals.var_ps0ld_dn0 = assign77880_e118009_d_n0;
        locals.var_ps0ld_dn2 = assign77880_e118009_d_n2;
        locals.var_ps0ld_dn4 = assign77880_e118009_d_n4;
        locals.var_ps0ld_dn5 = assign77880_e118009_d_n5;
        locals.var_ps0ld_dn6 = assign77880_e118009_d_n6;
        locals.var_ps0ld_dn7 = assign77880_e118009_d_n7;
        locals.var_ps0ld_dn8 = assign77880_e118009_d_n8;
        locals.var_ps0ld_dn9 = assign77880_e118009_d_n9;
        locals.var_ps0ld_dn10 = assign77880_e118009_d_n10;
        locals.var_ps0ld_dn13 = assign77880_e118009_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign77890_e118011: f64 = (locals.var_chi).abs();
        let assign77890_e118013: f64 = if assign77890_e118011 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1805 = assign77890_e118013;
        locals.var_guard1805_rv = 0.0;

        let (assign77900_e118028, assign77900_e118028_d_n0, assign77900_e118028_d_n2, assign77900_e118028_d_n4, assign77900_e118028_d_n5, assign77900_e118028_d_n6, assign77900_e118028_d_n7, assign77900_e118028_d_n8, assign77900_e118028_d_n9, assign77900_e118028_d_n10, assign77900_e118028_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1805 != 0.0)) {
        let assign77900_e118022: f64 = (locals.var_chi - 1.0);
        let assign77900_e118024: f64 = (-locals.var_chi);
        let assign77900_e118025: f64 = (assign77900_e118024).exp();
        let assign77900_e118026: f64 = (assign77900_e118022 + assign77900_e118025);
        (assign77900_e118026, (locals.var_chi_dn0 + (assign77900_e118025 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign77900_e118025 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign77900_e118025 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign77900_e118025 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign77900_e118025 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign77900_e118025 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign77900_e118025 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign77900_e118025 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign77900_e118025 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign77900_e118025 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign77900_e118028;
        locals.var_t1_dn0 = assign77900_e118028_d_n0;
        locals.var_t1_dn2 = assign77900_e118028_d_n2;
        locals.var_t1_dn4 = assign77900_e118028_d_n4;
        locals.var_t1_dn5 = assign77900_e118028_d_n5;
        locals.var_t1_dn6 = assign77900_e118028_d_n6;
        locals.var_t1_dn7 = assign77900_e118028_d_n7;
        locals.var_t1_dn8 = assign77900_e118028_d_n8;
        locals.var_t1_dn9 = assign77900_e118028_d_n9;
        locals.var_t1_dn10 = assign77900_e118028_d_n10;
        locals.var_t1_dn13 = assign77900_e118028_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign77910_e118038, assign77910_e118038_d_n0, assign77910_e118038_d_n2, assign77910_e118038_d_n4, assign77910_e118038_d_n5, assign77910_e118038_d_n6, assign77910_e118038_d_n7, assign77910_e118038_d_n8, assign77910_e118038_d_n9, assign77910_e118038_d_n10, assign77910_e118038_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1805 != 0.0)) {
        let assign77910_e118036: f64 = (locals.var_t1).sqrt();
        (assign77910_e118036, (locals.var_t1_dn0 / (2.0 * assign77910_e118036)), (locals.var_t1_dn2 / (2.0 * assign77910_e118036)), (locals.var_t1_dn4 / (2.0 * assign77910_e118036)), (locals.var_t1_dn5 / (2.0 * assign77910_e118036)), (locals.var_t1_dn6 / (2.0 * assign77910_e118036)), (locals.var_t1_dn7 / (2.0 * assign77910_e118036)), (locals.var_t1_dn8 / (2.0 * assign77910_e118036)), (locals.var_t1_dn9 / (2.0 * assign77910_e118036)), (locals.var_t1_dn10 / (2.0 * assign77910_e118036)), (locals.var_t1_dn13 / (2.0 * assign77910_e118036)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77910_e118038;
        locals.var_t2_dn0 = assign77910_e118038_d_n0;
        locals.var_t2_dn2 = assign77910_e118038_d_n2;
        locals.var_t2_dn4 = assign77910_e118038_d_n4;
        locals.var_t2_dn5 = assign77910_e118038_d_n5;
        locals.var_t2_dn6 = assign77910_e118038_d_n6;
        locals.var_t2_dn7 = assign77910_e118038_d_n7;
        locals.var_t2_dn8 = assign77910_e118038_d_n8;
        locals.var_t2_dn9 = assign77910_e118038_d_n9;
        locals.var_t2_dn10 = assign77910_e118038_d_n10;
        locals.var_t2_dn13 = assign77910_e118038_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign77930_e118069, assign77930_e118069_d_n0, assign77930_e118069_d_n2, assign77930_e118069_d_n4, assign77930_e118069_d_n5, assign77930_e118069_d_n6, assign77930_e118069_d_n7, assign77930_e118069_d_n8, assign77930_e118069_d_n9, assign77930_e118069_d_n10, assign77930_e118069_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1805 == 0.0)) {
        let assign77930_e118060: f64 = (0.7071067811865475 * locals.var_chi);
        let assign77930_e118064: f64 = (locals.var_chi * 0.3333333333333333);
        let assign77930_e118065: f64 = (1.0 - assign77930_e118064);
        let assign77930_e118066: f64 = (assign77930_e118065).sqrt();
        let assign77930_e118067: f64 = (assign77930_e118060 * assign77930_e118066);
        (assign77930_e118067, (((0.7071067811865475 * locals.var_chi_dn0) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign77930_e118066) + (assign77930_e118060 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign77930_e118066)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign77930_e118069;
        locals.var_t2_dn0 = assign77930_e118069_d_n0;
        locals.var_t2_dn2 = assign77930_e118069_d_n2;
        locals.var_t2_dn4 = assign77930_e118069_d_n4;
        locals.var_t2_dn5 = assign77930_e118069_d_n5;
        locals.var_t2_dn6 = assign77930_e118069_d_n6;
        locals.var_t2_dn7 = assign77930_e118069_d_n7;
        locals.var_t2_dn8 = assign77930_e118069_d_n8;
        locals.var_t2_dn9 = assign77930_e118069_d_n9;
        locals.var_t2_dn10 = assign77930_e118069_d_n10;
        locals.var_t2_dn13 = assign77930_e118069_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign77940_e118078, assign77940_e118078_d_n0, assign77940_e118078_d_n2, assign77940_e118078_d_n4, assign77940_e118078_d_n5, assign77940_e118078_d_n6, assign77940_e118078_d_n7, assign77940_e118078_d_n8, assign77940_e118078_d_n9, assign77940_e118078_d_n10, assign77940_e118078_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) {
        let assign77940_e118076: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign77940_e118076, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign77940_e118078;
        locals.var_qbuld_dn0 = assign77940_e118078_d_n0;
        locals.var_qbuld_dn2 = assign77940_e118078_d_n2;
        locals.var_qbuld_dn4 = assign77940_e118078_d_n4;
        locals.var_qbuld_dn5 = assign77940_e118078_d_n5;
        locals.var_qbuld_dn6 = assign77940_e118078_d_n6;
        locals.var_qbuld_dn7 = assign77940_e118078_d_n7;
        locals.var_qbuld_dn8 = assign77940_e118078_d_n8;
        locals.var_qbuld_dn9 = assign77940_e118078_d_n9;
        locals.var_qbuld_dn10 = assign77940_e118078_d_n10;
        locals.var_qbuld_dn13 = assign77940_e118078_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign77950_e118089, assign77950_e118089_d_n0, assign77950_e118089_d_n2, assign77950_e118089_d_n4, assign77950_e118089_d_n5, assign77950_e118089_d_n6, assign77950_e118089_d_n7, assign77950_e118089_d_n8, assign77950_e118089_d_n9, assign77950_e118089_d_n10, assign77950_e118089_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) {
        let assign77950_e118086: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign77950_e118087: f64 = (locals.var_cox0_func * assign77950_e118086);
        (assign77950_e118087, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign77950_e118089;
        locals.var_qsuld_dn0 = assign77950_e118089_d_n0;
        locals.var_qsuld_dn2 = assign77950_e118089_d_n2;
        locals.var_qsuld_dn4 = assign77950_e118089_d_n4;
        locals.var_qsuld_dn5 = assign77950_e118089_d_n5;
        locals.var_qsuld_dn6 = assign77950_e118089_d_n6;
        locals.var_qsuld_dn7 = assign77950_e118089_d_n7;
        locals.var_qsuld_dn8 = assign77950_e118089_d_n8;
        locals.var_qsuld_dn9 = assign77950_e118089_d_n9;
        locals.var_qsuld_dn10 = assign77950_e118089_d_n10;
        locals.var_qsuld_dn13 = assign77950_e118089_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign77960_e118098, assign77960_e118098_d_n0, assign77960_e118098_d_n2, assign77960_e118098_d_n4, assign77960_e118098_d_n5, assign77960_e118098_d_n6, assign77960_e118098_d_n7, assign77960_e118098_d_n8, assign77960_e118098_d_n9, assign77960_e118098_d_n10, assign77960_e118098_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) {
        let assign77960_e118096: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk1762);
        (assign77960_e118096, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk1762), (locals.var_qbuld_dn13 / locals.var_q_nsubld__blk1762),)
    } else {
        (locals.var_wdld0__blk1806, locals.var_wdld0__blk1806_dn0, locals.var_wdld0__blk1806_dn2, locals.var_wdld0__blk1806_dn4, locals.var_wdld0__blk1806_dn5, locals.var_wdld0__blk1806_dn6, locals.var_wdld0__blk1806_dn7, locals.var_wdld0__blk1806_dn8, locals.var_wdld0__blk1806_dn9, locals.var_wdld0__blk1806_dn10, locals.var_wdld0__blk1806_dn13,)
    }
};
        locals.var_wdld0__blk1806 = assign77960_e118098;
        locals.var_wdld0__blk1806_dn0 = assign77960_e118098_d_n0;
        locals.var_wdld0__blk1806_dn2 = assign77960_e118098_d_n2;
        locals.var_wdld0__blk1806_dn4 = assign77960_e118098_d_n4;
        locals.var_wdld0__blk1806_dn5 = assign77960_e118098_d_n5;
        locals.var_wdld0__blk1806_dn6 = assign77960_e118098_d_n6;
        locals.var_wdld0__blk1806_dn7 = assign77960_e118098_d_n7;
        locals.var_wdld0__blk1806_dn8 = assign77960_e118098_d_n8;
        locals.var_wdld0__blk1806_dn9 = assign77960_e118098_d_n9;
        locals.var_wdld0__blk1806_dn10 = assign77960_e118098_d_n10;
        locals.var_wdld0__blk1806_dn13 = assign77960_e118098_d_n13;
        locals.var_wdld0__blk1806_rv = 0.0;

        let assign77970_e118101: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1808 = assign77970_e118101;
        locals.var_guard1808_rv = 0.0;

        let assign77980_e118106: f64 = (locals.var_ddriftldc * 0.1);
        let assign77980_e118107: f64 = (locals.var_ddriftldc - assign77980_e118106);
        let assign77980_e118111: f64 = (locals.var_ddriftldc * 0.1);
        let assign77980_e118114: f64 = if ((locals.var_wdld0__blk1806 > assign77980_e118107) && (assign77980_e118111 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1809 = assign77980_e118114;
        locals.var_guard1809_rv = 0.0;

        let (assign77990_e118131, assign77990_e118131_d_n0, assign77990_e118131_d_n2, assign77990_e118131_d_n4, assign77990_e118131_d_n5, assign77990_e118131_d_n6, assign77990_e118131_d_n7, assign77990_e118131_d_n8, assign77990_e118131_d_n9, assign77990_e118131_d_n10, assign77990_e118131_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign77990_e118125: f64 = (locals.var_wdld0__blk1806 - locals.var_ddriftldc);
        let assign77990_e118128: f64 = (locals.var_ddriftldc * 0.1);
        let assign77990_e118129: f64 = (assign77990_e118125 + assign77990_e118128);
        (assign77990_e118129, ((locals.var_wdld0__blk1806_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk1806_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk1806_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk1806_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk1806_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk1806_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk1806_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk1806_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk1806_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk1806_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign77990_e118131;
        locals.var_tmf1_dn0 = assign77990_e118131_d_n0;
        locals.var_tmf1_dn2 = assign77990_e118131_d_n2;
        locals.var_tmf1_dn4 = assign77990_e118131_d_n4;
        locals.var_tmf1_dn5 = assign77990_e118131_d_n5;
        locals.var_tmf1_dn6 = assign77990_e118131_d_n6;
        locals.var_tmf1_dn7 = assign77990_e118131_d_n7;
        locals.var_tmf1_dn8 = assign77990_e118131_d_n8;
        locals.var_tmf1_dn9 = assign77990_e118131_d_n9;
        locals.var_tmf1_dn10 = assign77990_e118131_d_n10;
        locals.var_tmf1_dn13 = assign77990_e118131_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign78000_e118144, assign78000_e118144_d_n0, assign78000_e118144_d_n2, assign78000_e118144_d_n4, assign78000_e118144_d_n5, assign78000_e118144_d_n6, assign78000_e118144_d_n7, assign78000_e118144_d_n8, assign78000_e118144_d_n9, assign78000_e118144_d_n10, assign78000_e118144_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78000_e118142: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign78000_e118142, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign78000_e118144;
        locals.var_x2_dn0 = assign78000_e118144_d_n0;
        locals.var_x2_dn2 = assign78000_e118144_d_n2;
        locals.var_x2_dn4 = assign78000_e118144_d_n4;
        locals.var_x2_dn5 = assign78000_e118144_d_n5;
        locals.var_x2_dn6 = assign78000_e118144_d_n6;
        locals.var_x2_dn7 = assign78000_e118144_d_n7;
        locals.var_x2_dn8 = assign78000_e118144_d_n8;
        locals.var_x2_dn9 = assign78000_e118144_d_n9;
        locals.var_x2_dn10 = assign78000_e118144_d_n10;
        locals.var_x2_dn13 = assign78000_e118144_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign78010_e118161, assign78010_e118161_d_n0, assign78010_e118161_d_n2, assign78010_e118161_d_n4, assign78010_e118161_d_n5, assign78010_e118161_d_n6, assign78010_e118161_d_n7, assign78010_e118161_d_n8, assign78010_e118161_d_n9, assign78010_e118161_d_n10, assign78010_e118161_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78010_e118155: f64 = (locals.var_ddriftldc * 0.1);
        let assign78010_e118158: f64 = (locals.var_ddriftldc * 0.1);
        let assign78010_e118159: f64 = (assign78010_e118155 * assign78010_e118158);
        (assign78010_e118159, (((locals.var_ddriftldc_dn0 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign78010_e118158) + (assign78010_e118155 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign78010_e118161;
        locals.var_xmax2_dn0 = assign78010_e118161_d_n0;
        locals.var_xmax2_dn2 = assign78010_e118161_d_n2;
        locals.var_xmax2_dn4 = assign78010_e118161_d_n4;
        locals.var_xmax2_dn5 = assign78010_e118161_d_n5;
        locals.var_xmax2_dn6 = assign78010_e118161_d_n6;
        locals.var_xmax2_dn7 = assign78010_e118161_d_n7;
        locals.var_xmax2_dn8 = assign78010_e118161_d_n8;
        locals.var_xmax2_dn9 = assign78010_e118161_d_n9;
        locals.var_xmax2_dn10 = assign78010_e118161_d_n10;
        locals.var_xmax2_dn13 = assign78010_e118161_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign78020_e118172, assign78020_e118172_d_n0, assign78020_e118172_d_n2, assign78020_e118172_d_n4, assign78020_e118172_d_n5, assign78020_e118172_d_n6, assign78020_e118172_d_n7, assign78020_e118172_d_n8, assign78020_e118172_d_n9, assign78020_e118172_d_n10, assign78020_e118172_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign78020_e118172;
        locals.var_xp_dn0 = assign78020_e118172_d_n0;
        locals.var_xp_dn2 = assign78020_e118172_d_n2;
        locals.var_xp_dn4 = assign78020_e118172_d_n4;
        locals.var_xp_dn5 = assign78020_e118172_d_n5;
        locals.var_xp_dn6 = assign78020_e118172_d_n6;
        locals.var_xp_dn7 = assign78020_e118172_d_n7;
        locals.var_xp_dn8 = assign78020_e118172_d_n8;
        locals.var_xp_dn9 = assign78020_e118172_d_n9;
        locals.var_xp_dn10 = assign78020_e118172_d_n10;
        locals.var_xp_dn13 = assign78020_e118172_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign78030_e118183, assign78030_e118183_d_n0, assign78030_e118183_d_n2, assign78030_e118183_d_n4, assign78030_e118183_d_n5, assign78030_e118183_d_n6, assign78030_e118183_d_n7, assign78030_e118183_d_n8, assign78030_e118183_d_n9, assign78030_e118183_d_n10, assign78030_e118183_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign78030_e118183;
        locals.var_xmp_dn0 = assign78030_e118183_d_n0;
        locals.var_xmp_dn2 = assign78030_e118183_d_n2;
        locals.var_xmp_dn4 = assign78030_e118183_d_n4;
        locals.var_xmp_dn5 = assign78030_e118183_d_n5;
        locals.var_xmp_dn6 = assign78030_e118183_d_n6;
        locals.var_xmp_dn7 = assign78030_e118183_d_n7;
        locals.var_xmp_dn8 = assign78030_e118183_d_n8;
        locals.var_xmp_dn9 = assign78030_e118183_d_n9;
        locals.var_xmp_dn10 = assign78030_e118183_d_n10;
        locals.var_xmp_dn13 = assign78030_e118183_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign78040_e118194,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78040_e118194;
        locals.var_m0_rv = 0.0;

        let (assign78050_e118205,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78050_e118205;
        locals.var_mm_rv = 0.0;

        let (assign78060_e118216, assign78060_e118216_d_n0, assign78060_e118216_d_n2, assign78060_e118216_d_n4, assign78060_e118216_d_n5, assign78060_e118216_d_n6, assign78060_e118216_d_n7, assign78060_e118216_d_n8, assign78060_e118216_d_n9, assign78060_e118216_d_n10, assign78060_e118216_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign78060_e118216;
        locals.var_arg_dn0 = assign78060_e118216_d_n0;
        locals.var_arg_dn2 = assign78060_e118216_d_n2;
        locals.var_arg_dn4 = assign78060_e118216_d_n4;
        locals.var_arg_dn5 = assign78060_e118216_d_n5;
        locals.var_arg_dn6 = assign78060_e118216_d_n6;
        locals.var_arg_dn7 = assign78060_e118216_d_n7;
        locals.var_arg_dn8 = assign78060_e118216_d_n8;
        locals.var_arg_dn9 = assign78060_e118216_d_n9;
        locals.var_arg_dn10 = assign78060_e118216_d_n10;
        locals.var_arg_dn13 = assign78060_e118216_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign78070_e118227, assign78070_e118227_d_n0, assign78070_e118227_d_n2, assign78070_e118227_d_n4, assign78070_e118227_d_n5, assign78070_e118227_d_n6, assign78070_e118227_d_n7, assign78070_e118227_d_n8, assign78070_e118227_d_n9, assign78070_e118227_d_n10, assign78070_e118227_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78070_e118227;
        locals.var_dnm_dn0 = assign78070_e118227_d_n0;
        locals.var_dnm_dn2 = assign78070_e118227_d_n2;
        locals.var_dnm_dn4 = assign78070_e118227_d_n4;
        locals.var_dnm_dn5 = assign78070_e118227_d_n5;
        locals.var_dnm_dn6 = assign78070_e118227_d_n6;
        locals.var_dnm_dn7 = assign78070_e118227_d_n7;
        locals.var_dnm_dn8 = assign78070_e118227_d_n8;
        locals.var_dnm_dn9 = assign78070_e118227_d_n9;
        locals.var_dnm_dn10 = assign78070_e118227_d_n10;
        locals.var_dnm_dn13 = assign78070_e118227_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign78080_e118240, assign78080_e118240_d_n0, assign78080_e118240_d_n2, assign78080_e118240_d_n4, assign78080_e118240_d_n5, assign78080_e118240_d_n6, assign78080_e118240_d_n7, assign78080_e118240_d_n8, assign78080_e118240_d_n9, assign78080_e118240_d_n10, assign78080_e118240_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78080_e118238: f64 = (locals.var_xp * locals.var_x2);
        (assign78080_e118238, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign78080_e118240;
        locals.var_xp_dn0 = assign78080_e118240_d_n0;
        locals.var_xp_dn2 = assign78080_e118240_d_n2;
        locals.var_xp_dn4 = assign78080_e118240_d_n4;
        locals.var_xp_dn5 = assign78080_e118240_d_n5;
        locals.var_xp_dn6 = assign78080_e118240_d_n6;
        locals.var_xp_dn7 = assign78080_e118240_d_n7;
        locals.var_xp_dn8 = assign78080_e118240_d_n8;
        locals.var_xp_dn9 = assign78080_e118240_d_n9;
        locals.var_xp_dn10 = assign78080_e118240_d_n10;
        locals.var_xp_dn13 = assign78080_e118240_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign78090_e118253, assign78090_e118253_d_n0, assign78090_e118253_d_n2, assign78090_e118253_d_n4, assign78090_e118253_d_n5, assign78090_e118253_d_n6, assign78090_e118253_d_n7, assign78090_e118253_d_n8, assign78090_e118253_d_n9, assign78090_e118253_d_n10, assign78090_e118253_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78090_e118251: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78090_e118251, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign78090_e118253;
        locals.var_xmp_dn0 = assign78090_e118253_d_n0;
        locals.var_xmp_dn2 = assign78090_e118253_d_n2;
        locals.var_xmp_dn4 = assign78090_e118253_d_n4;
        locals.var_xmp_dn5 = assign78090_e118253_d_n5;
        locals.var_xmp_dn6 = assign78090_e118253_d_n6;
        locals.var_xmp_dn7 = assign78090_e118253_d_n7;
        locals.var_xmp_dn8 = assign78090_e118253_d_n8;
        locals.var_xmp_dn9 = assign78090_e118253_d_n9;
        locals.var_xmp_dn10 = assign78090_e118253_d_n10;
        locals.var_xmp_dn13 = assign78090_e118253_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_284(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78100_e118266, assign78100_e118266_d_n0, assign78100_e118266_d_n2, assign78100_e118266_d_n4, assign78100_e118266_d_n5, assign78100_e118266_d_n6, assign78100_e118266_d_n7, assign78100_e118266_d_n8, assign78100_e118266_d_n9, assign78100_e118266_d_n10, assign78100_e118266_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78100_e118264: f64 = (locals.var_xp * locals.var_x2);
        (assign78100_e118264, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign78100_e118266;
        locals.var_xp_dn0 = assign78100_e118266_d_n0;
        locals.var_xp_dn2 = assign78100_e118266_d_n2;
        locals.var_xp_dn4 = assign78100_e118266_d_n4;
        locals.var_xp_dn5 = assign78100_e118266_d_n5;
        locals.var_xp_dn6 = assign78100_e118266_d_n6;
        locals.var_xp_dn7 = assign78100_e118266_d_n7;
        locals.var_xp_dn8 = assign78100_e118266_d_n8;
        locals.var_xp_dn9 = assign78100_e118266_d_n9;
        locals.var_xp_dn10 = assign78100_e118266_d_n10;
        locals.var_xp_dn13 = assign78100_e118266_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign78110_e118279, assign78110_e118279_d_n0, assign78110_e118279_d_n2, assign78110_e118279_d_n4, assign78110_e118279_d_n5, assign78110_e118279_d_n6, assign78110_e118279_d_n7, assign78110_e118279_d_n8, assign78110_e118279_d_n9, assign78110_e118279_d_n10, assign78110_e118279_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78110_e118277: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78110_e118277, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign78110_e118279;
        locals.var_xmp_dn0 = assign78110_e118279_d_n0;
        locals.var_xmp_dn2 = assign78110_e118279_d_n2;
        locals.var_xmp_dn4 = assign78110_e118279_d_n4;
        locals.var_xmp_dn5 = assign78110_e118279_d_n5;
        locals.var_xmp_dn6 = assign78110_e118279_d_n6;
        locals.var_xmp_dn7 = assign78110_e118279_d_n7;
        locals.var_xmp_dn8 = assign78110_e118279_d_n8;
        locals.var_xmp_dn9 = assign78110_e118279_d_n9;
        locals.var_xmp_dn10 = assign78110_e118279_d_n10;
        locals.var_xmp_dn13 = assign78110_e118279_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign78120_e118292, assign78120_e118292_d_n0, assign78120_e118292_d_n2, assign78120_e118292_d_n4, assign78120_e118292_d_n5, assign78120_e118292_d_n6, assign78120_e118292_d_n7, assign78120_e118292_d_n8, assign78120_e118292_d_n9, assign78120_e118292_d_n10, assign78120_e118292_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78120_e118290: f64 = (locals.var_xp + locals.var_xmp);
        (assign78120_e118290, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign78120_e118292;
        locals.var_arg_dn0 = assign78120_e118292_d_n0;
        locals.var_arg_dn2 = assign78120_e118292_d_n2;
        locals.var_arg_dn4 = assign78120_e118292_d_n4;
        locals.var_arg_dn5 = assign78120_e118292_d_n5;
        locals.var_arg_dn6 = assign78120_e118292_d_n6;
        locals.var_arg_dn7 = assign78120_e118292_d_n7;
        locals.var_arg_dn8 = assign78120_e118292_d_n8;
        locals.var_arg_dn9 = assign78120_e118292_d_n9;
        locals.var_arg_dn10 = assign78120_e118292_d_n10;
        locals.var_arg_dn13 = assign78120_e118292_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign78130_e118303, assign78130_e118303_d_n0, assign78130_e118303_d_n2, assign78130_e118303_d_n4, assign78130_e118303_d_n5, assign78130_e118303_d_n6, assign78130_e118303_d_n7, assign78130_e118303_d_n8, assign78130_e118303_d_n9, assign78130_e118303_d_n10, assign78130_e118303_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78130_e118303;
        locals.var_dnm_dn0 = assign78130_e118303_d_n0;
        locals.var_dnm_dn2 = assign78130_e118303_d_n2;
        locals.var_dnm_dn4 = assign78130_e118303_d_n4;
        locals.var_dnm_dn5 = assign78130_e118303_d_n5;
        locals.var_dnm_dn6 = assign78130_e118303_d_n6;
        locals.var_dnm_dn7 = assign78130_e118303_d_n7;
        locals.var_dnm_dn8 = assign78130_e118303_d_n8;
        locals.var_dnm_dn9 = assign78130_e118303_d_n9;
        locals.var_dnm_dn10 = assign78130_e118303_d_n10;
        locals.var_dnm_dn13 = assign78130_e118303_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign78140_e118318: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1810 = assign78140_e118318;
        locals.var_guard1810_rv = 0.0;

        let assign78150_e118321: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1811 = assign78150_e118321;
        locals.var_guard1811_rv = 0.0;

        let (assign78160_e118336,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78160_e118336;
        locals.var_mm_rv = 0.0;

        let assign78170_e118339: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1812 = assign78170_e118339;
        locals.var_guard1812_rv = 0.0;

        let (assign78180_e118357,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 == 0.0)) && (locals.var_guard1812 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78180_e118357;
        locals.var_mm_rv = 0.0;

        let assign78190_e118360: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1813 = assign78190_e118360;
        locals.var_guard1813_rv = 0.0;

        let (assign78200_e118381,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 == 0.0)) && (locals.var_guard1812 == 0.0)) && (locals.var_guard1813 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78200_e118381;
        locals.var_mm_rv = 0.0;

        let assign78210_e118384: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1814 = assign78210_e118384;
        locals.var_guard1814_rv = 0.0;

        let (assign78220_e118408,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 == 0.0)) && (locals.var_guard1812 == 0.0)) && (locals.var_guard1813 == 0.0)) && (locals.var_guard1814 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78220_e118408;
        locals.var_mm_rv = 0.0;

        let (assign78230_e118421,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78230_e118421;
        locals.var_m0_rv = 0.0;

        let mut assign78240_loop_guard: usize = 0;
        while {
            let assign78240_cond_e118435: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign78240_cond_e118435 != 0.0
        } {
            assign78240_loop_guard += 1;
            assert!(assign78240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign78240_body0_e118449, assign78240_body0_e118449_d_n0, assign78240_body0_e118449_d_n2, assign78240_body0_e118449_d_n4, assign78240_body0_e118449_d_n5, assign78240_body0_e118449_d_n6, assign78240_body0_e118449_d_n7, assign78240_body0_e118449_d_n8, assign78240_body0_e118449_d_n9, assign78240_body0_e118449_d_n10, assign78240_body0_e118449_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) {
        let assign78240_body0_e118447: f64 = (locals.var_dnm).sqrt();
        (assign78240_body0_e118447, (locals.var_dnm_dn0 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn2 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn4 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn5 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn6 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn7 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn8 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn9 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn10 / (2.0 * assign78240_body0_e118447)), (locals.var_dnm_dn13 / (2.0 * assign78240_body0_e118447)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign78240_body0_e118449;
            locals.var_dnm_dn0 = assign78240_body0_e118449_d_n0;
            locals.var_dnm_dn2 = assign78240_body0_e118449_d_n2;
            locals.var_dnm_dn4 = assign78240_body0_e118449_d_n4;
            locals.var_dnm_dn5 = assign78240_body0_e118449_d_n5;
            locals.var_dnm_dn6 = assign78240_body0_e118449_d_n6;
            locals.var_dnm_dn7 = assign78240_body0_e118449_d_n7;
            locals.var_dnm_dn8 = assign78240_body0_e118449_d_n8;
            locals.var_dnm_dn9 = assign78240_body0_e118449_d_n9;
            locals.var_dnm_dn10 = assign78240_body0_e118449_d_n10;
            locals.var_dnm_dn13 = assign78240_body0_e118449_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign78240_body1_e118464,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 != 0.0)) {
        let assign78240_body1_e118462: f64 = (locals.var_m0 + 1.0);
        (assign78240_body1_e118462,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign78240_body1_e118464;
            locals.var_m0_rv = 0.0;
        }

        let (assign78250_e118489, assign78250_e118489_d_n0, assign78250_e118489_d_n2, assign78250_e118489_d_n4, assign78250_e118489_d_n5, assign78250_e118489_d_n6, assign78250_e118489_d_n7, assign78250_e118489_d_n8, assign78250_e118489_d_n9, assign78250_e118489_d_n10, assign78250_e118489_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) && (locals.var_guard1810 == 0.0)) {
        let (assign78250_e118487, assign78250_e118487_d_n0, assign78250_e118487_d_n2, assign78250_e118487_d_n4, assign78250_e118487_d_n5, assign78250_e118487_d_n6, assign78250_e118487_d_n7, assign78250_e118487_d_n8, assign78250_e118487_d_n9, assign78250_e118487_d_n10, assign78250_e118487_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign78250_e118484: f64 = (2.0 * 2.0);
                let assign78250_e118485: f64 = (1.0 / assign78250_e118484);
                let assign78250_e118486: f64 = (locals.var_dnm).powf(assign78250_e118485);
                (assign78250_e118486, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn0)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn2)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn4)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn5)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn6)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn7)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn8)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn9)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn10)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78250_e118485) as f64).is_finite() && ((assign78250_e118485) as f64).fract() == 0.0 { if assign78250_e118485 == 0.0 { 0.0 } else { (assign78250_e118485 * ((locals.var_dnm).powf(assign78250_e118485 - 1.0) * locals.var_dnm_dn13)) } } else { (assign78250_e118486 * (assign78250_e118485 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign78250_e118487, assign78250_e118487_d_n0, assign78250_e118487_d_n2, assign78250_e118487_d_n4, assign78250_e118487_d_n5, assign78250_e118487_d_n6, assign78250_e118487_d_n7, assign78250_e118487_d_n8, assign78250_e118487_d_n9, assign78250_e118487_d_n10, assign78250_e118487_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78250_e118489;
        locals.var_dnm_dn0 = assign78250_e118489_d_n0;
        locals.var_dnm_dn2 = assign78250_e118489_d_n2;
        locals.var_dnm_dn4 = assign78250_e118489_d_n4;
        locals.var_dnm_dn5 = assign78250_e118489_d_n5;
        locals.var_dnm_dn6 = assign78250_e118489_d_n6;
        locals.var_dnm_dn7 = assign78250_e118489_d_n7;
        locals.var_dnm_dn8 = assign78250_e118489_d_n8;
        locals.var_dnm_dn9 = assign78250_e118489_d_n9;
        locals.var_dnm_dn10 = assign78250_e118489_d_n10;
        locals.var_dnm_dn13 = assign78250_e118489_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign78260_e118502, assign78260_e118502_d_n0, assign78260_e118502_d_n2, assign78260_e118502_d_n4, assign78260_e118502_d_n5, assign78260_e118502_d_n6, assign78260_e118502_d_n7, assign78260_e118502_d_n8, assign78260_e118502_d_n9, assign78260_e118502_d_n10, assign78260_e118502_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78260_e118500: f64 = (1.0 / locals.var_dnm);
        (assign78260_e118500, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78260_e118502;
        locals.var_dnm_dn0 = assign78260_e118502_d_n0;
        locals.var_dnm_dn2 = assign78260_e118502_d_n2;
        locals.var_dnm_dn4 = assign78260_e118502_d_n4;
        locals.var_dnm_dn5 = assign78260_e118502_d_n5;
        locals.var_dnm_dn6 = assign78260_e118502_d_n6;
        locals.var_dnm_dn7 = assign78260_e118502_d_n7;
        locals.var_dnm_dn8 = assign78260_e118502_d_n8;
        locals.var_dnm_dn9 = assign78260_e118502_d_n9;
        locals.var_dnm_dn10 = assign78260_e118502_d_n10;
        locals.var_dnm_dn13 = assign78260_e118502_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign78270_e118519, assign78270_e118519_d_n0, assign78270_e118519_d_n2, assign78270_e118519_d_n4, assign78270_e118519_d_n5, assign78270_e118519_d_n6, assign78270_e118519_d_n7, assign78270_e118519_d_n8, assign78270_e118519_d_n9, assign78270_e118519_d_n10, assign78270_e118519_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78270_e118514: f64 = (locals.var_ddriftldc * 0.1);
        let assign78270_e118515: f64 = (locals.var_tmf1 * assign78270_e118514);
        let assign78270_e118517: f64 = (assign78270_e118515 * locals.var_dnm);
        (assign78270_e118517, ((((locals.var_tmf1_dn0 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign78270_e118514) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign78270_e118515 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign78270_e118519;
        locals.var_tmf0_dn0 = assign78270_e118519_d_n0;
        locals.var_tmf0_dn2 = assign78270_e118519_d_n2;
        locals.var_tmf0_dn4 = assign78270_e118519_d_n4;
        locals.var_tmf0_dn5 = assign78270_e118519_d_n5;
        locals.var_tmf0_dn6 = assign78270_e118519_d_n6;
        locals.var_tmf0_dn7 = assign78270_e118519_d_n7;
        locals.var_tmf0_dn8 = assign78270_e118519_d_n8;
        locals.var_tmf0_dn9 = assign78270_e118519_d_n9;
        locals.var_tmf0_dn10 = assign78270_e118519_d_n10;
        locals.var_tmf0_dn13 = assign78270_e118519_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign78280_e118538, assign78280_e118538_d_n0, assign78280_e118538_d_n2, assign78280_e118538_d_n4, assign78280_e118538_d_n5, assign78280_e118538_d_n6, assign78280_e118538_d_n7, assign78280_e118538_d_n8, assign78280_e118538_d_n9, assign78280_e118538_d_n10, assign78280_e118538_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78280_e118530: f64 = (locals.var_ddriftldc * 0.1);
        let assign78280_e118532: f64 = (assign78280_e118530 * locals.var_xmp);
        let assign78280_e118534: f64 = (assign78280_e118532 * locals.var_dnm);
        let assign78280_e118536: f64 = (assign78280_e118534 / locals.var_arg);
        (assign78280_e118536, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn0)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn2)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn4)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn5)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn6)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn7)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn8)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn9)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn10)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign78280_e118530 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign78280_e118532 * locals.var_dnm_dn13)) * locals.var_arg) - (assign78280_e118534 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign78280_e118538;
        locals.var_t0_dn0 = assign78280_e118538_d_n0;
        locals.var_t0_dn2 = assign78280_e118538_d_n2;
        locals.var_t0_dn4 = assign78280_e118538_d_n4;
        locals.var_t0_dn5 = assign78280_e118538_d_n5;
        locals.var_t0_dn6 = assign78280_e118538_d_n6;
        locals.var_t0_dn7 = assign78280_e118538_d_n7;
        locals.var_t0_dn8 = assign78280_e118538_d_n8;
        locals.var_t0_dn9 = assign78280_e118538_d_n9;
        locals.var_t0_dn10 = assign78280_e118538_d_n10;
        locals.var_t0_dn13 = assign78280_e118538_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign78290_e118555, assign78290_e118555_d_n0, assign78290_e118555_d_n2, assign78290_e118555_d_n4, assign78290_e118555_d_n5, assign78290_e118555_d_n6, assign78290_e118555_d_n7, assign78290_e118555_d_n8, assign78290_e118555_d_n9, assign78290_e118555_d_n10, assign78290_e118555_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign78290_e118550: f64 = (locals.var_ddriftldc * 0.1);
        let assign78290_e118551: f64 = (locals.var_ddriftldc - assign78290_e118550);
        let assign78290_e118553: f64 = (assign78290_e118551 + locals.var_tmf0);
        (assign78290_e118553, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign78290_e118555;
        locals.var_t1_dn0 = assign78290_e118555_d_n0;
        locals.var_t1_dn2 = assign78290_e118555_d_n2;
        locals.var_t1_dn4 = assign78290_e118555_d_n4;
        locals.var_t1_dn5 = assign78290_e118555_d_n5;
        locals.var_t1_dn6 = assign78290_e118555_d_n6;
        locals.var_t1_dn7 = assign78290_e118555_d_n7;
        locals.var_t1_dn8 = assign78290_e118555_d_n8;
        locals.var_t1_dn9 = assign78290_e118555_d_n9;
        locals.var_t1_dn10 = assign78290_e118555_d_n10;
        locals.var_t1_dn13 = assign78290_e118555_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign78300_e118566, assign78300_e118566_d_n0, assign78300_e118566_d_n2, assign78300_e118566_d_n4, assign78300_e118566_d_n5, assign78300_e118566_d_n6, assign78300_e118566_d_n7, assign78300_e118566_d_n8, assign78300_e118566_d_n9, assign78300_e118566_d_n10, assign78300_e118566_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign78300_e118566;
        locals.var_t0_dn0 = assign78300_e118566_d_n0;
        locals.var_t0_dn2 = assign78300_e118566_d_n2;
        locals.var_t0_dn4 = assign78300_e118566_d_n4;
        locals.var_t0_dn5 = assign78300_e118566_d_n5;
        locals.var_t0_dn6 = assign78300_e118566_d_n6;
        locals.var_t0_dn7 = assign78300_e118566_d_n7;
        locals.var_t0_dn8 = assign78300_e118566_d_n8;
        locals.var_t0_dn9 = assign78300_e118566_d_n9;
        locals.var_t0_dn10 = assign78300_e118566_d_n10;
        locals.var_t0_dn13 = assign78300_e118566_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign78310_e118578, assign78310_e118578_d_n0, assign78310_e118578_d_n2, assign78310_e118578_d_n4, assign78310_e118578_d_n5, assign78310_e118578_d_n6, assign78310_e118578_d_n7, assign78310_e118578_d_n8, assign78310_e118578_d_n9, assign78310_e118578_d_n10, assign78310_e118578_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 == 0.0)) {
        (locals.var_wdld0__blk1806, locals.var_wdld0__blk1806_dn0, locals.var_wdld0__blk1806_dn2, locals.var_wdld0__blk1806_dn4, locals.var_wdld0__blk1806_dn5, locals.var_wdld0__blk1806_dn6, locals.var_wdld0__blk1806_dn7, locals.var_wdld0__blk1806_dn8, locals.var_wdld0__blk1806_dn9, locals.var_wdld0__blk1806_dn10, locals.var_wdld0__blk1806_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign78310_e118578;
        locals.var_t1_dn0 = assign78310_e118578_d_n0;
        locals.var_t1_dn2 = assign78310_e118578_d_n2;
        locals.var_t1_dn4 = assign78310_e118578_d_n4;
        locals.var_t1_dn5 = assign78310_e118578_d_n5;
        locals.var_t1_dn6 = assign78310_e118578_d_n6;
        locals.var_t1_dn7 = assign78310_e118578_d_n7;
        locals.var_t1_dn8 = assign78310_e118578_d_n8;
        locals.var_t1_dn9 = assign78310_e118578_d_n9;
        locals.var_t1_dn10 = assign78310_e118578_d_n10;
        locals.var_t1_dn13 = assign78310_e118578_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign78320_e118590, assign78320_e118590_d_n0, assign78320_e118590_d_n2, assign78320_e118590_d_n4, assign78320_e118590_d_n5, assign78320_e118590_d_n6, assign78320_e118590_d_n7, assign78320_e118590_d_n8, assign78320_e118590_d_n9, assign78320_e118590_d_n10, assign78320_e118590_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1809 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign78320_e118590;
        locals.var_t0_dn0 = assign78320_e118590_d_n0;
        locals.var_t0_dn2 = assign78320_e118590_d_n2;
        locals.var_t0_dn4 = assign78320_e118590_d_n4;
        locals.var_t0_dn5 = assign78320_e118590_d_n5;
        locals.var_t0_dn6 = assign78320_e118590_d_n6;
        locals.var_t0_dn7 = assign78320_e118590_d_n7;
        locals.var_t0_dn8 = assign78320_e118590_d_n8;
        locals.var_t0_dn9 = assign78320_e118590_d_n9;
        locals.var_t0_dn10 = assign78320_e118590_d_n10;
        locals.var_t0_dn13 = assign78320_e118590_d_n13;
        locals.var_t0_rv = 0.0;

        let assign78330_e118593: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1815 = assign78330_e118593;
        locals.var_guard1815_rv = 0.0;

        let (assign78340_e118606,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 != 0.0)) && (locals.var_guard1815 != 0.0)) {
        let assign78340_e118604: f64 = (locals.var_flg_fd_mode__blk1768 + 2.0);
        (assign78340_e118604,)
    } else {
        (locals.var_flg_fd_mode__blk1768,)
    }
};
        locals.var_flg_fd_mode__blk1768 = assign78340_e118606;
        locals.var_flg_fd_mode__blk1768_rv = 0.0;

        let (assign78350_e118621, assign78350_e118621_d_n0, assign78350_e118621_d_n2, assign78350_e118621_d_n4, assign78350_e118621_d_n5, assign78350_e118621_d_n6, assign78350_e118621_d_n7, assign78350_e118621_d_n8, assign78350_e118621_d_n9, assign78350_e118621_d_n10, assign78350_e118621_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 == 0.0)) {
        let (assign78350_e118619, assign78350_e118619_d_n0, assign78350_e118619_d_n2, assign78350_e118619_d_n4, assign78350_e118619_d_n5, assign78350_e118619_d_n6, assign78350_e118619_d_n7, assign78350_e118619_d_n8, assign78350_e118619_d_n9, assign78350_e118619_d_n10, assign78350_e118619_d_n13,) = {
            if (locals.var_wdld0__blk1806 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk1806, locals.var_wdld0__blk1806_dn0, locals.var_wdld0__blk1806_dn2, locals.var_wdld0__blk1806_dn4, locals.var_wdld0__blk1806_dn5, locals.var_wdld0__blk1806_dn6, locals.var_wdld0__blk1806_dn7, locals.var_wdld0__blk1806_dn8, locals.var_wdld0__blk1806_dn9, locals.var_wdld0__blk1806_dn10, locals.var_wdld0__blk1806_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign78350_e118619, assign78350_e118619_d_n0, assign78350_e118619_d_n2, assign78350_e118619_d_n4, assign78350_e118619_d_n5, assign78350_e118619_d_n6, assign78350_e118619_d_n7, assign78350_e118619_d_n8, assign78350_e118619_d_n9, assign78350_e118619_d_n10, assign78350_e118619_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign78350_e118621;
        locals.var_t1_dn0 = assign78350_e118621_d_n0;
        locals.var_t1_dn2 = assign78350_e118621_d_n2;
        locals.var_t1_dn4 = assign78350_e118621_d_n4;
        locals.var_t1_dn5 = assign78350_e118621_d_n5;
        locals.var_t1_dn6 = assign78350_e118621_d_n6;
        locals.var_t1_dn7 = assign78350_e118621_d_n7;
        locals.var_t1_dn8 = assign78350_e118621_d_n8;
        locals.var_t1_dn9 = assign78350_e118621_d_n9;
        locals.var_t1_dn10 = assign78350_e118621_d_n10;
        locals.var_t1_dn13 = assign78350_e118621_d_n13;
        locals.var_t1_rv = 0.0;

        let assign78360_e118624: f64 = if locals.var_wdld0__blk1806 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1816 = assign78360_e118624;
        locals.var_guard1816_rv = 0.0;

        let (assign78370_e118638,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1808 == 0.0)) && (locals.var_guard1816 != 0.0)) {
        let assign78370_e118636: f64 = (locals.var_flg_fd_mode__blk1768 + 2.0);
        (assign78370_e118636,)
    } else {
        (locals.var_flg_fd_mode__blk1768,)
    }
};
        locals.var_flg_fd_mode__blk1768 = assign78370_e118638;
        locals.var_flg_fd_mode__blk1768_rv = 0.0;

        let assign78380_e118641: f64 = if locals.var_flg_fd_mode__blk1768 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1817 = assign78380_e118641;
        locals.var_guard1817_rv = 0.0;

        let (assign78390_e118650, assign78390_e118650_d_n0, assign78390_e118650_d_n2, assign78390_e118650_d_n4, assign78390_e118650_d_n5, assign78390_e118650_d_n6, assign78390_e118650_d_n7, assign78390_e118650_d_n8, assign78390_e118650_d_n9, assign78390_e118650_d_n10, assign78390_e118650_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1__blk1807, locals.var_ps0ld_bef1__blk1807_dn0, locals.var_ps0ld_bef1__blk1807_dn2, locals.var_ps0ld_bef1__blk1807_dn4, locals.var_ps0ld_bef1__blk1807_dn5, locals.var_ps0ld_bef1__blk1807_dn6, locals.var_ps0ld_bef1__blk1807_dn7, locals.var_ps0ld_bef1__blk1807_dn8, locals.var_ps0ld_bef1__blk1807_dn9, locals.var_ps0ld_bef1__blk1807_dn10, locals.var_ps0ld_bef1__blk1807_dn13,)
    }
};
        locals.var_ps0ld_bef1__blk1807 = assign78390_e118650;
        locals.var_ps0ld_bef1__blk1807_dn0 = assign78390_e118650_d_n0;
        locals.var_ps0ld_bef1__blk1807_dn2 = assign78390_e118650_d_n2;
        locals.var_ps0ld_bef1__blk1807_dn4 = assign78390_e118650_d_n4;
        locals.var_ps0ld_bef1__blk1807_dn5 = assign78390_e118650_d_n5;
        locals.var_ps0ld_bef1__blk1807_dn6 = assign78390_e118650_d_n6;
        locals.var_ps0ld_bef1__blk1807_dn7 = assign78390_e118650_d_n7;
        locals.var_ps0ld_bef1__blk1807_dn8 = assign78390_e118650_d_n8;
        locals.var_ps0ld_bef1__blk1807_dn9 = assign78390_e118650_d_n9;
        locals.var_ps0ld_bef1__blk1807_dn10 = assign78390_e118650_d_n10;
        locals.var_ps0ld_bef1__blk1807_dn13 = assign78390_e118650_d_n13;
        locals.var_ps0ld_bef1__blk1807_rv = 0.0;

        let (assign78400_e118661, assign78400_e118661_d_n0, assign78400_e118661_d_n2, assign78400_e118661_d_n4, assign78400_e118661_d_n5, assign78400_e118661_d_n6, assign78400_e118661_d_n7, assign78400_e118661_d_n8, assign78400_e118661_d_n9, assign78400_e118661_d_n10, assign78400_e118661_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) {
        let assign78400_e118659: f64 = (locals.var_t1 * locals.var_q_nsubld__blk1762);
        (assign78400_e118659, (locals.var_t1_dn0 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn2 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn4 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn5 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn6 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn7 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn8 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn9 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn10 * locals.var_q_nsubld__blk1762), (locals.var_t1_dn13 * locals.var_q_nsubld__blk1762),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign78400_e118661;
        locals.var_qbuld_dn0 = assign78400_e118661_d_n0;
        locals.var_qbuld_dn2 = assign78400_e118661_d_n2;
        locals.var_qbuld_dn4 = assign78400_e118661_d_n4;
        locals.var_qbuld_dn5 = assign78400_e118661_d_n5;
        locals.var_qbuld_dn6 = assign78400_e118661_d_n6;
        locals.var_qbuld_dn7 = assign78400_e118661_d_n7;
        locals.var_qbuld_dn8 = assign78400_e118661_d_n8;
        locals.var_qbuld_dn9 = assign78400_e118661_d_n9;
        locals.var_qbuld_dn10 = assign78400_e118661_d_n10;
        locals.var_qbuld_dn13 = assign78400_e118661_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign78410_e118674, assign78410_e118674_d_n0, assign78410_e118674_d_n2, assign78410_e118674_d_n4, assign78410_e118674_d_n5, assign78410_e118674_d_n6, assign78410_e118674_d_n7, assign78410_e118674_d_n8, assign78410_e118674_d_n9, assign78410_e118674_d_n10, assign78410_e118674_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) {
        let assign78410_e118671: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign78410_e118672: f64 = (locals.var_vgpld - assign78410_e118671);
        (assign78410_e118672, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign78410_e118674;
        locals.var_ps0ld_dn0 = assign78410_e118674_d_n0;
        locals.var_ps0ld_dn2 = assign78410_e118674_d_n2;
        locals.var_ps0ld_dn4 = assign78410_e118674_d_n4;
        locals.var_ps0ld_dn5 = assign78410_e118674_d_n5;
        locals.var_ps0ld_dn6 = assign78410_e118674_d_n6;
        locals.var_ps0ld_dn7 = assign78410_e118674_d_n7;
        locals.var_ps0ld_dn8 = assign78410_e118674_d_n8;
        locals.var_ps0ld_dn9 = assign78410_e118674_d_n9;
        locals.var_ps0ld_dn10 = assign78410_e118674_d_n10;
        locals.var_ps0ld_dn13 = assign78410_e118674_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign78420_e118677: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1818 = assign78420_e118677;
        locals.var_guard1818_rv = 0.0;

        let assign78430_e118681: f64 = (locals.var_ps0ld_bef1__blk1807 - 0.1);
        let assign78430_e118686: f64 = if ((locals.var_ps0ld > assign78430_e118681) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1819 = assign78430_e118686;
        locals.var_guard1819_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_285(
        locals: &mut StampLocals,
    ) {
        let (assign78440_e118703, assign78440_e118703_d_n0, assign78440_e118703_d_n2, assign78440_e118703_d_n4, assign78440_e118703_d_n5, assign78440_e118703_d_n6, assign78440_e118703_d_n7, assign78440_e118703_d_n8, assign78440_e118703_d_n9, assign78440_e118703_d_n10, assign78440_e118703_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78440_e118699: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk1807);
        let assign78440_e118701: f64 = (assign78440_e118699 + 0.1);
        (assign78440_e118701, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk1807_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk1807_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk1807_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk1807_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk1807_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk1807_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk1807_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk1807_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk1807_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1__blk1807_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign78440_e118703;
        locals.var_tmf1_dn0 = assign78440_e118703_d_n0;
        locals.var_tmf1_dn2 = assign78440_e118703_d_n2;
        locals.var_tmf1_dn4 = assign78440_e118703_d_n4;
        locals.var_tmf1_dn5 = assign78440_e118703_d_n5;
        locals.var_tmf1_dn6 = assign78440_e118703_d_n6;
        locals.var_tmf1_dn7 = assign78440_e118703_d_n7;
        locals.var_tmf1_dn8 = assign78440_e118703_d_n8;
        locals.var_tmf1_dn9 = assign78440_e118703_d_n9;
        locals.var_tmf1_dn10 = assign78440_e118703_d_n10;
        locals.var_tmf1_dn13 = assign78440_e118703_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign78450_e118718, assign78450_e118718_d_n0, assign78450_e118718_d_n2, assign78450_e118718_d_n4, assign78450_e118718_d_n5, assign78450_e118718_d_n6, assign78450_e118718_d_n7, assign78450_e118718_d_n8, assign78450_e118718_d_n9, assign78450_e118718_d_n10, assign78450_e118718_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78450_e118716: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign78450_e118716, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign78450_e118718;
        locals.var_x2_dn0 = assign78450_e118718_d_n0;
        locals.var_x2_dn2 = assign78450_e118718_d_n2;
        locals.var_x2_dn4 = assign78450_e118718_d_n4;
        locals.var_x2_dn5 = assign78450_e118718_d_n5;
        locals.var_x2_dn6 = assign78450_e118718_d_n6;
        locals.var_x2_dn7 = assign78450_e118718_d_n7;
        locals.var_x2_dn8 = assign78450_e118718_d_n8;
        locals.var_x2_dn9 = assign78450_e118718_d_n9;
        locals.var_x2_dn10 = assign78450_e118718_d_n10;
        locals.var_x2_dn13 = assign78450_e118718_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign78460_e118733, assign78460_e118733_d_n0, assign78460_e118733_d_n2, assign78460_e118733_d_n4, assign78460_e118733_d_n5, assign78460_e118733_d_n6, assign78460_e118733_d_n7, assign78460_e118733_d_n8, assign78460_e118733_d_n9, assign78460_e118733_d_n10, assign78460_e118733_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78460_e118731: f64 = (0.1 * 0.1);
        (assign78460_e118731, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign78460_e118733;
        locals.var_xmax2_dn0 = assign78460_e118733_d_n0;
        locals.var_xmax2_dn2 = assign78460_e118733_d_n2;
        locals.var_xmax2_dn4 = assign78460_e118733_d_n4;
        locals.var_xmax2_dn5 = assign78460_e118733_d_n5;
        locals.var_xmax2_dn6 = assign78460_e118733_d_n6;
        locals.var_xmax2_dn7 = assign78460_e118733_d_n7;
        locals.var_xmax2_dn8 = assign78460_e118733_d_n8;
        locals.var_xmax2_dn9 = assign78460_e118733_d_n9;
        locals.var_xmax2_dn10 = assign78460_e118733_d_n10;
        locals.var_xmax2_dn13 = assign78460_e118733_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign78470_e118746, assign78470_e118746_d_n0, assign78470_e118746_d_n2, assign78470_e118746_d_n4, assign78470_e118746_d_n5, assign78470_e118746_d_n6, assign78470_e118746_d_n7, assign78470_e118746_d_n8, assign78470_e118746_d_n9, assign78470_e118746_d_n10, assign78470_e118746_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign78470_e118746;
        locals.var_xp_dn0 = assign78470_e118746_d_n0;
        locals.var_xp_dn2 = assign78470_e118746_d_n2;
        locals.var_xp_dn4 = assign78470_e118746_d_n4;
        locals.var_xp_dn5 = assign78470_e118746_d_n5;
        locals.var_xp_dn6 = assign78470_e118746_d_n6;
        locals.var_xp_dn7 = assign78470_e118746_d_n7;
        locals.var_xp_dn8 = assign78470_e118746_d_n8;
        locals.var_xp_dn9 = assign78470_e118746_d_n9;
        locals.var_xp_dn10 = assign78470_e118746_d_n10;
        locals.var_xp_dn13 = assign78470_e118746_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign78480_e118759, assign78480_e118759_d_n0, assign78480_e118759_d_n2, assign78480_e118759_d_n4, assign78480_e118759_d_n5, assign78480_e118759_d_n6, assign78480_e118759_d_n7, assign78480_e118759_d_n8, assign78480_e118759_d_n9, assign78480_e118759_d_n10, assign78480_e118759_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign78480_e118759;
        locals.var_xmp_dn0 = assign78480_e118759_d_n0;
        locals.var_xmp_dn2 = assign78480_e118759_d_n2;
        locals.var_xmp_dn4 = assign78480_e118759_d_n4;
        locals.var_xmp_dn5 = assign78480_e118759_d_n5;
        locals.var_xmp_dn6 = assign78480_e118759_d_n6;
        locals.var_xmp_dn7 = assign78480_e118759_d_n7;
        locals.var_xmp_dn8 = assign78480_e118759_d_n8;
        locals.var_xmp_dn9 = assign78480_e118759_d_n9;
        locals.var_xmp_dn10 = assign78480_e118759_d_n10;
        locals.var_xmp_dn13 = assign78480_e118759_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign78490_e118772,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78490_e118772;
        locals.var_m0_rv = 0.0;

        let (assign78500_e118785,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78500_e118785;
        locals.var_mm_rv = 0.0;

        let (assign78510_e118798, assign78510_e118798_d_n0, assign78510_e118798_d_n2, assign78510_e118798_d_n4, assign78510_e118798_d_n5, assign78510_e118798_d_n6, assign78510_e118798_d_n7, assign78510_e118798_d_n8, assign78510_e118798_d_n9, assign78510_e118798_d_n10, assign78510_e118798_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign78510_e118798;
        locals.var_arg_dn0 = assign78510_e118798_d_n0;
        locals.var_arg_dn2 = assign78510_e118798_d_n2;
        locals.var_arg_dn4 = assign78510_e118798_d_n4;
        locals.var_arg_dn5 = assign78510_e118798_d_n5;
        locals.var_arg_dn6 = assign78510_e118798_d_n6;
        locals.var_arg_dn7 = assign78510_e118798_d_n7;
        locals.var_arg_dn8 = assign78510_e118798_d_n8;
        locals.var_arg_dn9 = assign78510_e118798_d_n9;
        locals.var_arg_dn10 = assign78510_e118798_d_n10;
        locals.var_arg_dn13 = assign78510_e118798_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign78520_e118811, assign78520_e118811_d_n0, assign78520_e118811_d_n2, assign78520_e118811_d_n4, assign78520_e118811_d_n5, assign78520_e118811_d_n6, assign78520_e118811_d_n7, assign78520_e118811_d_n8, assign78520_e118811_d_n9, assign78520_e118811_d_n10, assign78520_e118811_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78520_e118811;
        locals.var_dnm_dn0 = assign78520_e118811_d_n0;
        locals.var_dnm_dn2 = assign78520_e118811_d_n2;
        locals.var_dnm_dn4 = assign78520_e118811_d_n4;
        locals.var_dnm_dn5 = assign78520_e118811_d_n5;
        locals.var_dnm_dn6 = assign78520_e118811_d_n6;
        locals.var_dnm_dn7 = assign78520_e118811_d_n7;
        locals.var_dnm_dn8 = assign78520_e118811_d_n8;
        locals.var_dnm_dn9 = assign78520_e118811_d_n9;
        locals.var_dnm_dn10 = assign78520_e118811_d_n10;
        locals.var_dnm_dn13 = assign78520_e118811_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign78530_e118826, assign78530_e118826_d_n0, assign78530_e118826_d_n2, assign78530_e118826_d_n4, assign78530_e118826_d_n5, assign78530_e118826_d_n6, assign78530_e118826_d_n7, assign78530_e118826_d_n8, assign78530_e118826_d_n9, assign78530_e118826_d_n10, assign78530_e118826_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78530_e118824: f64 = (locals.var_xp * locals.var_x2);
        (assign78530_e118824, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign78530_e118826;
        locals.var_xp_dn0 = assign78530_e118826_d_n0;
        locals.var_xp_dn2 = assign78530_e118826_d_n2;
        locals.var_xp_dn4 = assign78530_e118826_d_n4;
        locals.var_xp_dn5 = assign78530_e118826_d_n5;
        locals.var_xp_dn6 = assign78530_e118826_d_n6;
        locals.var_xp_dn7 = assign78530_e118826_d_n7;
        locals.var_xp_dn8 = assign78530_e118826_d_n8;
        locals.var_xp_dn9 = assign78530_e118826_d_n9;
        locals.var_xp_dn10 = assign78530_e118826_d_n10;
        locals.var_xp_dn13 = assign78530_e118826_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign78540_e118841, assign78540_e118841_d_n0, assign78540_e118841_d_n2, assign78540_e118841_d_n4, assign78540_e118841_d_n5, assign78540_e118841_d_n6, assign78540_e118841_d_n7, assign78540_e118841_d_n8, assign78540_e118841_d_n9, assign78540_e118841_d_n10, assign78540_e118841_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78540_e118839: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78540_e118839, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign78540_e118841;
        locals.var_xmp_dn0 = assign78540_e118841_d_n0;
        locals.var_xmp_dn2 = assign78540_e118841_d_n2;
        locals.var_xmp_dn4 = assign78540_e118841_d_n4;
        locals.var_xmp_dn5 = assign78540_e118841_d_n5;
        locals.var_xmp_dn6 = assign78540_e118841_d_n6;
        locals.var_xmp_dn7 = assign78540_e118841_d_n7;
        locals.var_xmp_dn8 = assign78540_e118841_d_n8;
        locals.var_xmp_dn9 = assign78540_e118841_d_n9;
        locals.var_xmp_dn10 = assign78540_e118841_d_n10;
        locals.var_xmp_dn13 = assign78540_e118841_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign78550_e118856, assign78550_e118856_d_n0, assign78550_e118856_d_n2, assign78550_e118856_d_n4, assign78550_e118856_d_n5, assign78550_e118856_d_n6, assign78550_e118856_d_n7, assign78550_e118856_d_n8, assign78550_e118856_d_n9, assign78550_e118856_d_n10, assign78550_e118856_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78550_e118854: f64 = (locals.var_xp * locals.var_x2);
        (assign78550_e118854, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign78550_e118856;
        locals.var_xp_dn0 = assign78550_e118856_d_n0;
        locals.var_xp_dn2 = assign78550_e118856_d_n2;
        locals.var_xp_dn4 = assign78550_e118856_d_n4;
        locals.var_xp_dn5 = assign78550_e118856_d_n5;
        locals.var_xp_dn6 = assign78550_e118856_d_n6;
        locals.var_xp_dn7 = assign78550_e118856_d_n7;
        locals.var_xp_dn8 = assign78550_e118856_d_n8;
        locals.var_xp_dn9 = assign78550_e118856_d_n9;
        locals.var_xp_dn10 = assign78550_e118856_d_n10;
        locals.var_xp_dn13 = assign78550_e118856_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign78560_e118871, assign78560_e118871_d_n0, assign78560_e118871_d_n2, assign78560_e118871_d_n4, assign78560_e118871_d_n5, assign78560_e118871_d_n6, assign78560_e118871_d_n7, assign78560_e118871_d_n8, assign78560_e118871_d_n9, assign78560_e118871_d_n10, assign78560_e118871_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78560_e118869: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78560_e118869, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign78560_e118871;
        locals.var_xmp_dn0 = assign78560_e118871_d_n0;
        locals.var_xmp_dn2 = assign78560_e118871_d_n2;
        locals.var_xmp_dn4 = assign78560_e118871_d_n4;
        locals.var_xmp_dn5 = assign78560_e118871_d_n5;
        locals.var_xmp_dn6 = assign78560_e118871_d_n6;
        locals.var_xmp_dn7 = assign78560_e118871_d_n7;
        locals.var_xmp_dn8 = assign78560_e118871_d_n8;
        locals.var_xmp_dn9 = assign78560_e118871_d_n9;
        locals.var_xmp_dn10 = assign78560_e118871_d_n10;
        locals.var_xmp_dn13 = assign78560_e118871_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign78570_e118886, assign78570_e118886_d_n0, assign78570_e118886_d_n2, assign78570_e118886_d_n4, assign78570_e118886_d_n5, assign78570_e118886_d_n6, assign78570_e118886_d_n7, assign78570_e118886_d_n8, assign78570_e118886_d_n9, assign78570_e118886_d_n10, assign78570_e118886_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78570_e118884: f64 = (locals.var_xp + locals.var_xmp);
        (assign78570_e118884, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign78570_e118886;
        locals.var_arg_dn0 = assign78570_e118886_d_n0;
        locals.var_arg_dn2 = assign78570_e118886_d_n2;
        locals.var_arg_dn4 = assign78570_e118886_d_n4;
        locals.var_arg_dn5 = assign78570_e118886_d_n5;
        locals.var_arg_dn6 = assign78570_e118886_d_n6;
        locals.var_arg_dn7 = assign78570_e118886_d_n7;
        locals.var_arg_dn8 = assign78570_e118886_d_n8;
        locals.var_arg_dn9 = assign78570_e118886_d_n9;
        locals.var_arg_dn10 = assign78570_e118886_d_n10;
        locals.var_arg_dn13 = assign78570_e118886_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign78580_e118899, assign78580_e118899_d_n0, assign78580_e118899_d_n2, assign78580_e118899_d_n4, assign78580_e118899_d_n5, assign78580_e118899_d_n6, assign78580_e118899_d_n7, assign78580_e118899_d_n8, assign78580_e118899_d_n9, assign78580_e118899_d_n10, assign78580_e118899_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78580_e118899;
        locals.var_dnm_dn0 = assign78580_e118899_d_n0;
        locals.var_dnm_dn2 = assign78580_e118899_d_n2;
        locals.var_dnm_dn4 = assign78580_e118899_d_n4;
        locals.var_dnm_dn5 = assign78580_e118899_d_n5;
        locals.var_dnm_dn6 = assign78580_e118899_d_n6;
        locals.var_dnm_dn7 = assign78580_e118899_d_n7;
        locals.var_dnm_dn8 = assign78580_e118899_d_n8;
        locals.var_dnm_dn9 = assign78580_e118899_d_n9;
        locals.var_dnm_dn10 = assign78580_e118899_d_n10;
        locals.var_dnm_dn13 = assign78580_e118899_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign78590_e118914: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1820 = assign78590_e118914;
        locals.var_guard1820_rv = 0.0;

        let assign78600_e118917: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1821 = assign78600_e118917;
        locals.var_guard1821_rv = 0.0;

        let (assign78610_e118934,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78610_e118934;
        locals.var_mm_rv = 0.0;

        let assign78620_e118937: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1822 = assign78620_e118937;
        locals.var_guard1822_rv = 0.0;

        let (assign78630_e118957,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 == 0.0)) && (locals.var_guard1822 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78630_e118957;
        locals.var_mm_rv = 0.0;

        let assign78640_e118960: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1823 = assign78640_e118960;
        locals.var_guard1823_rv = 0.0;

        let (assign78650_e118983,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 == 0.0)) && (locals.var_guard1822 == 0.0)) && (locals.var_guard1823 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78650_e118983;
        locals.var_mm_rv = 0.0;

        let assign78660_e118986: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1824 = assign78660_e118986;
        locals.var_guard1824_rv = 0.0;

        let (assign78670_e119012,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 == 0.0)) && (locals.var_guard1822 == 0.0)) && (locals.var_guard1823 == 0.0)) && (locals.var_guard1824 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78670_e119012;
        locals.var_mm_rv = 0.0;

        let (assign78680_e119027,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78680_e119027;
        locals.var_m0_rv = 0.0;

        let mut assign78690_loop_guard: usize = 0;
        while {
            let assign78690_cond_e119043: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign78690_cond_e119043 != 0.0
        } {
            assign78690_loop_guard += 1;
            assert!(assign78690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign78690_body0_e119059, assign78690_body0_e119059_d_n0, assign78690_body0_e119059_d_n2, assign78690_body0_e119059_d_n4, assign78690_body0_e119059_d_n5, assign78690_body0_e119059_d_n6, assign78690_body0_e119059_d_n7, assign78690_body0_e119059_d_n8, assign78690_body0_e119059_d_n9, assign78690_body0_e119059_d_n10, assign78690_body0_e119059_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) {
        let assign78690_body0_e119057: f64 = (locals.var_dnm).sqrt();
        (assign78690_body0_e119057, (locals.var_dnm_dn0 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn2 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn4 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn5 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn6 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn7 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn8 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn9 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn10 / (2.0 * assign78690_body0_e119057)), (locals.var_dnm_dn13 / (2.0 * assign78690_body0_e119057)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign78690_body0_e119059;
            locals.var_dnm_dn0 = assign78690_body0_e119059_d_n0;
            locals.var_dnm_dn2 = assign78690_body0_e119059_d_n2;
            locals.var_dnm_dn4 = assign78690_body0_e119059_d_n4;
            locals.var_dnm_dn5 = assign78690_body0_e119059_d_n5;
            locals.var_dnm_dn6 = assign78690_body0_e119059_d_n6;
            locals.var_dnm_dn7 = assign78690_body0_e119059_d_n7;
            locals.var_dnm_dn8 = assign78690_body0_e119059_d_n8;
            locals.var_dnm_dn9 = assign78690_body0_e119059_d_n9;
            locals.var_dnm_dn10 = assign78690_body0_e119059_d_n10;
            locals.var_dnm_dn13 = assign78690_body0_e119059_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign78690_body1_e119076,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) {
        let assign78690_body1_e119074: f64 = (locals.var_m0 + 1.0);
        (assign78690_body1_e119074,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign78690_body1_e119076;
            locals.var_m0_rv = 0.0;
        }

        let (assign78700_e119103, assign78700_e119103_d_n0, assign78700_e119103_d_n2, assign78700_e119103_d_n4, assign78700_e119103_d_n5, assign78700_e119103_d_n6, assign78700_e119103_d_n7, assign78700_e119103_d_n8, assign78700_e119103_d_n9, assign78700_e119103_d_n10, assign78700_e119103_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 == 0.0)) {
        let (assign78700_e119101, assign78700_e119101_d_n0, assign78700_e119101_d_n2, assign78700_e119101_d_n4, assign78700_e119101_d_n5, assign78700_e119101_d_n6, assign78700_e119101_d_n7, assign78700_e119101_d_n8, assign78700_e119101_d_n9, assign78700_e119101_d_n10, assign78700_e119101_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign78700_e119098: f64 = (2.0 * 2.0);
                let assign78700_e119099: f64 = (1.0 / assign78700_e119098);
                let assign78700_e119100: f64 = (locals.var_dnm).powf(assign78700_e119099);
                (assign78700_e119100, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn0)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn2)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn4)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn5)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn6)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn7)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn8)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn9)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn10)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78700_e119099) as f64).is_finite() && ((assign78700_e119099) as f64).fract() == 0.0 { if assign78700_e119099 == 0.0 { 0.0 } else { (assign78700_e119099 * ((locals.var_dnm).powf(assign78700_e119099 - 1.0) * locals.var_dnm_dn13)) } } else { (assign78700_e119100 * (assign78700_e119099 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign78700_e119101, assign78700_e119101_d_n0, assign78700_e119101_d_n2, assign78700_e119101_d_n4, assign78700_e119101_d_n5, assign78700_e119101_d_n6, assign78700_e119101_d_n7, assign78700_e119101_d_n8, assign78700_e119101_d_n9, assign78700_e119101_d_n10, assign78700_e119101_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78700_e119103;
        locals.var_dnm_dn0 = assign78700_e119103_d_n0;
        locals.var_dnm_dn2 = assign78700_e119103_d_n2;
        locals.var_dnm_dn4 = assign78700_e119103_d_n4;
        locals.var_dnm_dn5 = assign78700_e119103_d_n5;
        locals.var_dnm_dn6 = assign78700_e119103_d_n6;
        locals.var_dnm_dn7 = assign78700_e119103_d_n7;
        locals.var_dnm_dn8 = assign78700_e119103_d_n8;
        locals.var_dnm_dn9 = assign78700_e119103_d_n9;
        locals.var_dnm_dn10 = assign78700_e119103_d_n10;
        locals.var_dnm_dn13 = assign78700_e119103_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign78710_e119118, assign78710_e119118_d_n0, assign78710_e119118_d_n2, assign78710_e119118_d_n4, assign78710_e119118_d_n5, assign78710_e119118_d_n6, assign78710_e119118_d_n7, assign78710_e119118_d_n8, assign78710_e119118_d_n9, assign78710_e119118_d_n10, assign78710_e119118_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78710_e119116: f64 = (1.0 / locals.var_dnm);
        (assign78710_e119116, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign78710_e119118;
        locals.var_dnm_dn0 = assign78710_e119118_d_n0;
        locals.var_dnm_dn2 = assign78710_e119118_d_n2;
        locals.var_dnm_dn4 = assign78710_e119118_d_n4;
        locals.var_dnm_dn5 = assign78710_e119118_d_n5;
        locals.var_dnm_dn6 = assign78710_e119118_d_n6;
        locals.var_dnm_dn7 = assign78710_e119118_d_n7;
        locals.var_dnm_dn8 = assign78710_e119118_d_n8;
        locals.var_dnm_dn9 = assign78710_e119118_d_n9;
        locals.var_dnm_dn10 = assign78710_e119118_d_n10;
        locals.var_dnm_dn13 = assign78710_e119118_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign78720_e119135, assign78720_e119135_d_n0, assign78720_e119135_d_n2, assign78720_e119135_d_n4, assign78720_e119135_d_n5, assign78720_e119135_d_n6, assign78720_e119135_d_n7, assign78720_e119135_d_n8, assign78720_e119135_d_n9, assign78720_e119135_d_n10, assign78720_e119135_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78720_e119131: f64 = (locals.var_tmf1 * 0.1);
        let assign78720_e119133: f64 = (assign78720_e119131 * locals.var_dnm);
        (assign78720_e119133, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign78720_e119131 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign78720_e119135;
        locals.var_tmf0_dn0 = assign78720_e119135_d_n0;
        locals.var_tmf0_dn2 = assign78720_e119135_d_n2;
        locals.var_tmf0_dn4 = assign78720_e119135_d_n4;
        locals.var_tmf0_dn5 = assign78720_e119135_d_n5;
        locals.var_tmf0_dn6 = assign78720_e119135_d_n6;
        locals.var_tmf0_dn7 = assign78720_e119135_d_n7;
        locals.var_tmf0_dn8 = assign78720_e119135_d_n8;
        locals.var_tmf0_dn9 = assign78720_e119135_d_n9;
        locals.var_tmf0_dn10 = assign78720_e119135_d_n10;
        locals.var_tmf0_dn13 = assign78720_e119135_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign78730_e119154, assign78730_e119154_d_n0, assign78730_e119154_d_n2, assign78730_e119154_d_n4, assign78730_e119154_d_n5, assign78730_e119154_d_n6, assign78730_e119154_d_n7, assign78730_e119154_d_n8, assign78730_e119154_d_n9, assign78730_e119154_d_n10, assign78730_e119154_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78730_e119148: f64 = (0.1 * locals.var_xmp);
        let assign78730_e119150: f64 = (assign78730_e119148 * locals.var_dnm);
        let assign78730_e119152: f64 = (assign78730_e119150 / locals.var_arg);
        (assign78730_e119152, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn0)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn2)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn4)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn5)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn6)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn7)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn8)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn9)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn10)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign78730_e119148 * locals.var_dnm_dn13)) * locals.var_arg) - (assign78730_e119150 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign78730_e119154;
        locals.var_t0_dn0 = assign78730_e119154_d_n0;
        locals.var_t0_dn2 = assign78730_e119154_d_n2;
        locals.var_t0_dn4 = assign78730_e119154_d_n4;
        locals.var_t0_dn5 = assign78730_e119154_d_n5;
        locals.var_t0_dn6 = assign78730_e119154_d_n6;
        locals.var_t0_dn7 = assign78730_e119154_d_n7;
        locals.var_t0_dn8 = assign78730_e119154_d_n8;
        locals.var_t0_dn9 = assign78730_e119154_d_n9;
        locals.var_t0_dn10 = assign78730_e119154_d_n10;
        locals.var_t0_dn13 = assign78730_e119154_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_286(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78740_e119171, assign78740_e119171_d_n0, assign78740_e119171_d_n2, assign78740_e119171_d_n4, assign78740_e119171_d_n5, assign78740_e119171_d_n6, assign78740_e119171_d_n7, assign78740_e119171_d_n8, assign78740_e119171_d_n9, assign78740_e119171_d_n10, assign78740_e119171_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78740_e119167: f64 = (locals.var_ps0ld_bef1__blk1807 - 0.1);
        let assign78740_e119169: f64 = (assign78740_e119167 + locals.var_tmf0);
        (assign78740_e119169, (locals.var_ps0ld_bef1__blk1807_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk1807_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk1807_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk1807_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk1807_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk1807_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk1807_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk1807_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk1807_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk1807_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign78740_e119171;
        locals.var_ps0ld_dn0 = assign78740_e119171_d_n0;
        locals.var_ps0ld_dn2 = assign78740_e119171_d_n2;
        locals.var_ps0ld_dn4 = assign78740_e119171_d_n4;
        locals.var_ps0ld_dn5 = assign78740_e119171_d_n5;
        locals.var_ps0ld_dn6 = assign78740_e119171_d_n6;
        locals.var_ps0ld_dn7 = assign78740_e119171_d_n7;
        locals.var_ps0ld_dn8 = assign78740_e119171_d_n8;
        locals.var_ps0ld_dn9 = assign78740_e119171_d_n9;
        locals.var_ps0ld_dn10 = assign78740_e119171_d_n10;
        locals.var_ps0ld_dn13 = assign78740_e119171_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign78750_e119184, assign78750_e119184_d_n0, assign78750_e119184_d_n2, assign78750_e119184_d_n4, assign78750_e119184_d_n5, assign78750_e119184_d_n6, assign78750_e119184_d_n7, assign78750_e119184_d_n8, assign78750_e119184_d_n9, assign78750_e119184_d_n10, assign78750_e119184_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign78750_e119184;
        locals.var_t0_dn0 = assign78750_e119184_d_n0;
        locals.var_t0_dn2 = assign78750_e119184_d_n2;
        locals.var_t0_dn4 = assign78750_e119184_d_n4;
        locals.var_t0_dn5 = assign78750_e119184_d_n5;
        locals.var_t0_dn6 = assign78750_e119184_d_n6;
        locals.var_t0_dn7 = assign78750_e119184_d_n7;
        locals.var_t0_dn8 = assign78750_e119184_d_n8;
        locals.var_t0_dn9 = assign78750_e119184_d_n9;
        locals.var_t0_dn10 = assign78750_e119184_d_n10;
        locals.var_t0_dn13 = assign78750_e119184_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign78760_e119198, assign78760_e119198_d_n0, assign78760_e119198_d_n2, assign78760_e119198_d_n4, assign78760_e119198_d_n5, assign78760_e119198_d_n6, assign78760_e119198_d_n7, assign78760_e119198_d_n8, assign78760_e119198_d_n9, assign78760_e119198_d_n10, assign78760_e119198_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign78760_e119198;
        locals.var_ps0ld_dn0 = assign78760_e119198_d_n0;
        locals.var_ps0ld_dn2 = assign78760_e119198_d_n2;
        locals.var_ps0ld_dn4 = assign78760_e119198_d_n4;
        locals.var_ps0ld_dn5 = assign78760_e119198_d_n5;
        locals.var_ps0ld_dn6 = assign78760_e119198_d_n6;
        locals.var_ps0ld_dn7 = assign78760_e119198_d_n7;
        locals.var_ps0ld_dn8 = assign78760_e119198_d_n8;
        locals.var_ps0ld_dn9 = assign78760_e119198_d_n9;
        locals.var_ps0ld_dn10 = assign78760_e119198_d_n10;
        locals.var_ps0ld_dn13 = assign78760_e119198_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign78770_e119212, assign78770_e119212_d_n0, assign78770_e119212_d_n2, assign78770_e119212_d_n4, assign78770_e119212_d_n5, assign78770_e119212_d_n6, assign78770_e119212_d_n7, assign78770_e119212_d_n8, assign78770_e119212_d_n9, assign78770_e119212_d_n10, assign78770_e119212_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 != 0.0)) && (locals.var_guard1819 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign78770_e119212;
        locals.var_t0_dn0 = assign78770_e119212_d_n0;
        locals.var_t0_dn2 = assign78770_e119212_d_n2;
        locals.var_t0_dn4 = assign78770_e119212_d_n4;
        locals.var_t0_dn5 = assign78770_e119212_d_n5;
        locals.var_t0_dn6 = assign78770_e119212_d_n6;
        locals.var_t0_dn7 = assign78770_e119212_d_n7;
        locals.var_t0_dn8 = assign78770_e119212_d_n8;
        locals.var_t0_dn9 = assign78770_e119212_d_n9;
        locals.var_t0_dn10 = assign78770_e119212_d_n10;
        locals.var_t0_dn13 = assign78770_e119212_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign78780_e119229, assign78780_e119229_d_n0, assign78780_e119229_d_n2, assign78780_e119229_d_n4, assign78780_e119229_d_n5, assign78780_e119229_d_n6, assign78780_e119229_d_n7, assign78780_e119229_d_n8, assign78780_e119229_d_n9, assign78780_e119229_d_n10, assign78780_e119229_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1817 != 0.0)) && (locals.var_guard1818 == 0.0)) {
        let (assign78780_e119227, assign78780_e119227_d_n0, assign78780_e119227_d_n2, assign78780_e119227_d_n4, assign78780_e119227_d_n5, assign78780_e119227_d_n6, assign78780_e119227_d_n7, assign78780_e119227_d_n8, assign78780_e119227_d_n9, assign78780_e119227_d_n10, assign78780_e119227_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk1807) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1__blk1807, locals.var_ps0ld_bef1__blk1807_dn0, locals.var_ps0ld_bef1__blk1807_dn2, locals.var_ps0ld_bef1__blk1807_dn4, locals.var_ps0ld_bef1__blk1807_dn5, locals.var_ps0ld_bef1__blk1807_dn6, locals.var_ps0ld_bef1__blk1807_dn7, locals.var_ps0ld_bef1__blk1807_dn8, locals.var_ps0ld_bef1__blk1807_dn9, locals.var_ps0ld_bef1__blk1807_dn10, locals.var_ps0ld_bef1__blk1807_dn13,)
            }
        };
        (assign78780_e119227, assign78780_e119227_d_n0, assign78780_e119227_d_n2, assign78780_e119227_d_n4, assign78780_e119227_d_n5, assign78780_e119227_d_n6, assign78780_e119227_d_n7, assign78780_e119227_d_n8, assign78780_e119227_d_n9, assign78780_e119227_d_n10, assign78780_e119227_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign78780_e119229;
        locals.var_ps0ld_dn0 = assign78780_e119229_d_n0;
        locals.var_ps0ld_dn2 = assign78780_e119229_d_n2;
        locals.var_ps0ld_dn4 = assign78780_e119229_d_n4;
        locals.var_ps0ld_dn5 = assign78780_e119229_d_n5;
        locals.var_ps0ld_dn6 = assign78780_e119229_d_n6;
        locals.var_ps0ld_dn7 = assign78780_e119229_d_n7;
        locals.var_ps0ld_dn8 = assign78780_e119229_d_n8;
        locals.var_ps0ld_dn9 = assign78780_e119229_d_n9;
        locals.var_ps0ld_dn10 = assign78780_e119229_d_n10;
        locals.var_ps0ld_dn13 = assign78780_e119229_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign78790_e119236, assign78790_e119236_d_n0, assign78790_e119236_d_n2, assign78790_e119236_d_n4, assign78790_e119236_d_n5, assign78790_e119236_d_n6, assign78790_e119236_d_n7, assign78790_e119236_d_n8, assign78790_e119236_d_n9, assign78790_e119236_d_n10, assign78790_e119236_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk1769, locals.var_ps0ld_ini__blk1769_dn0, locals.var_ps0ld_ini__blk1769_dn2, locals.var_ps0ld_ini__blk1769_dn4, locals.var_ps0ld_ini__blk1769_dn5, locals.var_ps0ld_ini__blk1769_dn6, locals.var_ps0ld_ini__blk1769_dn7, locals.var_ps0ld_ini__blk1769_dn8, locals.var_ps0ld_ini__blk1769_dn9, locals.var_ps0ld_ini__blk1769_dn10, locals.var_ps0ld_ini__blk1769_dn13,)
    }
};
        locals.var_ps0ld_ini__blk1769 = assign78790_e119236;
        locals.var_ps0ld_ini__blk1769_dn0 = assign78790_e119236_d_n0;
        locals.var_ps0ld_ini__blk1769_dn2 = assign78790_e119236_d_n2;
        locals.var_ps0ld_ini__blk1769_dn4 = assign78790_e119236_d_n4;
        locals.var_ps0ld_ini__blk1769_dn5 = assign78790_e119236_d_n5;
        locals.var_ps0ld_ini__blk1769_dn6 = assign78790_e119236_d_n6;
        locals.var_ps0ld_ini__blk1769_dn7 = assign78790_e119236_d_n7;
        locals.var_ps0ld_ini__blk1769_dn8 = assign78790_e119236_d_n8;
        locals.var_ps0ld_ini__blk1769_dn9 = assign78790_e119236_d_n9;
        locals.var_ps0ld_ini__blk1769_dn10 = assign78790_e119236_d_n10;
        locals.var_ps0ld_ini__blk1769_dn13 = assign78790_e119236_d_n13;
        locals.var_ps0ld_ini__blk1769_rv = 0.0;

        let assign78800_e119239: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1825 = assign78800_e119239;
        locals.var_guard1825_rv = 0.0;

        let (assign78810_e119248,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign78810_e119248;
        locals.var_flg_conv_rv = 0.0;

        let (assign78820_e119264, assign78820_e119264_d_n0, assign78820_e119264_d_n2, assign78820_e119264_d_n4, assign78820_e119264_d_n5, assign78820_e119264_d_n6, assign78820_e119264_d_n7, assign78820_e119264_d_n8, assign78820_e119264_d_n9, assign78820_e119264_d_n10, assign78820_e119264_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign78820_e119258: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1762);
        let assign78820_e119260: f64 = (assign78820_e119258 * locals.var_beta_inv);
        let assign78820_e119261: f64 = (2.0 * assign78820_e119260);
        let assign78820_e119262: f64 = (assign78820_e119261).sqrt();
        (assign78820_e119262, ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn0)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn2)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn4)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn5)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn6)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn7)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn8)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn9)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn10)) / (2.0 * assign78820_e119262)), ((2.0 * (assign78820_e119258 * locals.var_beta_inv_dn13)) / (2.0 * assign78820_e119262)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign78820_e119264;
        locals.var_c_w_ld_dn0 = assign78820_e119264_d_n0;
        locals.var_c_w_ld_dn2 = assign78820_e119264_d_n2;
        locals.var_c_w_ld_dn4 = assign78820_e119264_d_n4;
        locals.var_c_w_ld_dn5 = assign78820_e119264_d_n5;
        locals.var_c_w_ld_dn6 = assign78820_e119264_d_n6;
        locals.var_c_w_ld_dn7 = assign78820_e119264_d_n7;
        locals.var_c_w_ld_dn8 = assign78820_e119264_d_n8;
        locals.var_c_w_ld_dn9 = assign78820_e119264_d_n9;
        locals.var_c_w_ld_dn10 = assign78820_e119264_d_n10;
        locals.var_c_w_ld_dn13 = assign78820_e119264_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign78830_e119267: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1826 = assign78830_e119267;
        locals.var_guard1826_rv = 0.0;

        let (assign78840_e119280, assign78840_e119280_d_n0, assign78840_e119280_d_n2, assign78840_e119280_d_n4, assign78840_e119280_d_n5, assign78840_e119280_d_n6, assign78840_e119280_d_n7, assign78840_e119280_d_n8, assign78840_e119280_d_n9, assign78840_e119280_d_n10, assign78840_e119280_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 != 0.0)) {
        let assign78840_e119278: f64 = (p.p334 - locals.var_wdep_func);
        (assign78840_e119278, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign78840_e119280;
        locals.var_t2_dn0 = assign78840_e119280_d_n0;
        locals.var_t2_dn2 = assign78840_e119280_d_n2;
        locals.var_t2_dn4 = assign78840_e119280_d_n4;
        locals.var_t2_dn5 = assign78840_e119280_d_n5;
        locals.var_t2_dn6 = assign78840_e119280_d_n6;
        locals.var_t2_dn7 = assign78840_e119280_d_n7;
        locals.var_t2_dn8 = assign78840_e119280_d_n8;
        locals.var_t2_dn9 = assign78840_e119280_d_n9;
        locals.var_t2_dn10 = assign78840_e119280_d_n10;
        locals.var_t2_dn13 = assign78840_e119280_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign78850_e119305, assign78850_e119305_d_n0, assign78850_e119305_d_n2, assign78850_e119305_d_n4, assign78850_e119305_d_n5, assign78850_e119305_d_n6, assign78850_e119305_d_n7, assign78850_e119305_d_n8, assign78850_e119305_d_n9, assign78850_e119305_d_n10, assign78850_e119305_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 == 0.0)) {
        let assign78850_e119292: f64 = (locals.var_vdsi + p.p137);
        let assign78850_e119295: f64 = (locals.var_vdsi + p.p137);
        let assign78850_e119296: f64 = (assign78850_e119292 * assign78850_e119295);
        let assign78850_e119299: f64 = (4.0 * 0.1);
        let assign78850_e119301: f64 = (assign78850_e119299 * 0.1);
        let assign78850_e119302: f64 = (assign78850_e119296 + assign78850_e119301);
        let assign78850_e119303: f64 = (assign78850_e119302).sqrt();
        (assign78850_e119303, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign78850_e119295) + (assign78850_e119292 * locals.var_vdsi_dn5)) / (2.0 * assign78850_e119303)), 0.0, (((locals.var_vdsi_dn7 * assign78850_e119295) + (assign78850_e119292 * locals.var_vdsi_dn7)) / (2.0 * assign78850_e119303)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign78850_e119305;
        locals.var_tmf2_dn0 = assign78850_e119305_d_n0;
        locals.var_tmf2_dn2 = assign78850_e119305_d_n2;
        locals.var_tmf2_dn4 = assign78850_e119305_d_n4;
        locals.var_tmf2_dn5 = assign78850_e119305_d_n5;
        locals.var_tmf2_dn6 = assign78850_e119305_d_n6;
        locals.var_tmf2_dn7 = assign78850_e119305_d_n7;
        locals.var_tmf2_dn8 = assign78850_e119305_d_n8;
        locals.var_tmf2_dn9 = assign78850_e119305_d_n9;
        locals.var_tmf2_dn10 = assign78850_e119305_d_n10;
        locals.var_tmf2_dn13 = assign78850_e119305_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign78860_e119325, assign78860_e119325_d_n0, assign78860_e119325_d_n2, assign78860_e119325_d_n4, assign78860_e119325_d_n5, assign78860_e119325_d_n6, assign78860_e119325_d_n7, assign78860_e119325_d_n8, assign78860_e119325_d_n9, assign78860_e119325_d_n10, assign78860_e119325_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 == 0.0)) {
        let assign78860_e119319: f64 = (locals.var_vdsi + p.p137);
        let assign78860_e119321: f64 = (assign78860_e119319 / locals.var_tmf2);
        let assign78860_e119322: f64 = (1.0 + assign78860_e119321);
        let assign78860_e119323: f64 = (0.5 * assign78860_e119322);
        (assign78860_e119323, (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign78860_e119319 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign78860_e119319 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78860_e119319 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign78860_e119325;
        locals.var_t9_dn0 = assign78860_e119325_d_n0;
        locals.var_t9_dn2 = assign78860_e119325_d_n2;
        locals.var_t9_dn4 = assign78860_e119325_d_n4;
        locals.var_t9_dn5 = assign78860_e119325_d_n5;
        locals.var_t9_dn6 = assign78860_e119325_d_n6;
        locals.var_t9_dn7 = assign78860_e119325_d_n7;
        locals.var_t9_dn8 = assign78860_e119325_d_n8;
        locals.var_t9_dn9 = assign78860_e119325_d_n9;
        locals.var_t9_dn10 = assign78860_e119325_d_n10;
        locals.var_t9_dn13 = assign78860_e119325_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign78870_e119343, assign78870_e119343_d_n0, assign78870_e119343_d_n2, assign78870_e119343_d_n4, assign78870_e119343_d_n5, assign78870_e119343_d_n6, assign78870_e119343_d_n7, assign78870_e119343_d_n8, assign78870_e119343_d_n9, assign78870_e119343_d_n10, assign78870_e119343_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 == 0.0)) {
        let assign78870_e119338: f64 = (locals.var_vdsi + p.p137);
        let assign78870_e119340: f64 = (assign78870_e119338 + locals.var_tmf2);
        let assign78870_e119341: f64 = (0.5 * assign78870_e119340);
        (assign78870_e119341, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign78870_e119343;
        locals.var_t2_dn0 = assign78870_e119343_d_n0;
        locals.var_t2_dn2 = assign78870_e119343_d_n2;
        locals.var_t2_dn4 = assign78870_e119343_d_n4;
        locals.var_t2_dn5 = assign78870_e119343_d_n5;
        locals.var_t2_dn6 = assign78870_e119343_d_n6;
        locals.var_t2_dn7 = assign78870_e119343_d_n7;
        locals.var_t2_dn8 = assign78870_e119343_d_n8;
        locals.var_t2_dn9 = assign78870_e119343_d_n9;
        locals.var_t2_dn10 = assign78870_e119343_d_n10;
        locals.var_t2_dn13 = assign78870_e119343_d_n13;
        locals.var_t2_rv = 0.0;

        let assign78880_e119346: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1827 = assign78880_e119346;
        locals.var_guard1827_rv = 0.0;

        let (assign78890_e119360, assign78890_e119360_d_n0, assign78890_e119360_d_n2, assign78890_e119360_d_n4, assign78890_e119360_d_n5, assign78890_e119360_d_n6, assign78890_e119360_d_n7, assign78890_e119360_d_n8, assign78890_e119360_d_n9, assign78890_e119360_d_n10, assign78890_e119360_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign78890_e119360;
        locals.var_t2_dn0 = assign78890_e119360_d_n0;
        locals.var_t2_dn2 = assign78890_e119360_d_n2;
        locals.var_t2_dn4 = assign78890_e119360_d_n4;
        locals.var_t2_dn5 = assign78890_e119360_d_n5;
        locals.var_t2_dn6 = assign78890_e119360_d_n6;
        locals.var_t2_dn7 = assign78890_e119360_d_n7;
        locals.var_t2_dn8 = assign78890_e119360_d_n8;
        locals.var_t2_dn9 = assign78890_e119360_d_n9;
        locals.var_t2_dn10 = assign78890_e119360_d_n10;
        locals.var_t2_dn13 = assign78890_e119360_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign78900_e119374, assign78900_e119374_d_n0, assign78900_e119374_d_n2, assign78900_e119374_d_n4, assign78900_e119374_d_n5, assign78900_e119374_d_n6, assign78900_e119374_d_n7, assign78900_e119374_d_n8, assign78900_e119374_d_n9, assign78900_e119374_d_n10, assign78900_e119374_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign78900_e119374;
        locals.var_t9_dn0 = assign78900_e119374_d_n0;
        locals.var_t9_dn2 = assign78900_e119374_d_n2;
        locals.var_t9_dn4 = assign78900_e119374_d_n4;
        locals.var_t9_dn5 = assign78900_e119374_d_n5;
        locals.var_t9_dn6 = assign78900_e119374_d_n6;
        locals.var_t9_dn7 = assign78900_e119374_d_n7;
        locals.var_t9_dn8 = assign78900_e119374_d_n8;
        locals.var_t9_dn9 = assign78900_e119374_d_n9;
        locals.var_t9_dn10 = assign78900_e119374_d_n10;
        locals.var_t9_dn13 = assign78900_e119374_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign78910_e119391, assign78910_e119391_d_n0, assign78910_e119391_d_n2, assign78910_e119391_d_n4, assign78910_e119391_d_n5, assign78910_e119391_d_n6, assign78910_e119391_d_n7, assign78910_e119391_d_n8, assign78910_e119391_d_n9, assign78910_e119391_d_n10, assign78910_e119391_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 == 0.0)) {
        let assign78910_e119386: f64 = (locals.var_kjunc * locals.var_t2);
        let assign78910_e119387: f64 = (assign78910_e119386).sqrt();
        let assign78910_e119389: f64 = (assign78910_e119387 * p.p432);
        (assign78910_e119389, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign78910_e119387)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign78910_e119387)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign78910_e119391;
        locals.var_wjunc0_dn0 = assign78910_e119391_d_n0;
        locals.var_wjunc0_dn2 = assign78910_e119391_d_n2;
        locals.var_wjunc0_dn4 = assign78910_e119391_d_n4;
        locals.var_wjunc0_dn5 = assign78910_e119391_d_n5;
        locals.var_wjunc0_dn6 = assign78910_e119391_d_n6;
        locals.var_wjunc0_dn7 = assign78910_e119391_d_n7;
        locals.var_wjunc0_dn8 = assign78910_e119391_d_n8;
        locals.var_wjunc0_dn9 = assign78910_e119391_d_n9;
        locals.var_wjunc0_dn10 = assign78910_e119391_d_n10;
        locals.var_wjunc0_dn13 = assign78910_e119391_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign78920_e119405, assign78920_e119405_d_n0, assign78920_e119405_d_n2, assign78920_e119405_d_n4, assign78920_e119405_d_n5, assign78920_e119405_d_n6, assign78920_e119405_d_n7, assign78920_e119405_d_n8, assign78920_e119405_d_n9, assign78920_e119405_d_n10, assign78920_e119405_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1826 == 0.0)) {
        let assign78920_e119403: f64 = (p.p334 - locals.var_wjunc0);
        (assign78920_e119403, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign78920_e119405;
        locals.var_t2_dn0 = assign78920_e119405_d_n0;
        locals.var_t2_dn2 = assign78920_e119405_d_n2;
        locals.var_t2_dn4 = assign78920_e119405_d_n4;
        locals.var_t2_dn5 = assign78920_e119405_d_n5;
        locals.var_t2_dn6 = assign78920_e119405_d_n6;
        locals.var_t2_dn7 = assign78920_e119405_d_n7;
        locals.var_t2_dn8 = assign78920_e119405_d_n8;
        locals.var_t2_dn9 = assign78920_e119405_d_n9;
        locals.var_t2_dn10 = assign78920_e119405_d_n10;
        locals.var_t2_dn13 = assign78920_e119405_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign78930_e119427, assign78930_e119427_d_n0, assign78930_e119427_d_n2, assign78930_e119427_d_n4, assign78930_e119427_d_n5, assign78930_e119427_d_n6, assign78930_e119427_d_n7, assign78930_e119427_d_n8, assign78930_e119427_d_n9, assign78930_e119427_d_n10, assign78930_e119427_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign78930_e119414: f64 = (locals.var_t2 * locals.var_t2);
        let assign78930_e119418: f64 = (p.p334 * 0.01);
        let assign78930_e119419: f64 = (4.0 * assign78930_e119418);
        let assign78930_e119422: f64 = (p.p334 * 0.01);
        let assign78930_e119423: f64 = (assign78930_e119419 * assign78930_e119422);
        let assign78930_e119424: f64 = (assign78930_e119414 + assign78930_e119423);
        let assign78930_e119425: f64 = (assign78930_e119424).sqrt();
        (assign78930_e119425, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign78930_e119425)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign78930_e119425)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign78930_e119427;
        locals.var_tmf2_dn0 = assign78930_e119427_d_n0;
        locals.var_tmf2_dn2 = assign78930_e119427_d_n2;
        locals.var_tmf2_dn4 = assign78930_e119427_d_n4;
        locals.var_tmf2_dn5 = assign78930_e119427_d_n5;
        locals.var_tmf2_dn6 = assign78930_e119427_d_n6;
        locals.var_tmf2_dn7 = assign78930_e119427_d_n7;
        locals.var_tmf2_dn8 = assign78930_e119427_d_n8;
        locals.var_tmf2_dn9 = assign78930_e119427_d_n9;
        locals.var_tmf2_dn10 = assign78930_e119427_d_n10;
        locals.var_tmf2_dn13 = assign78930_e119427_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign78940_e119442, assign78940_e119442_d_n0, assign78940_e119442_d_n2, assign78940_e119442_d_n4, assign78940_e119442_d_n5, assign78940_e119442_d_n6, assign78940_e119442_d_n7, assign78940_e119442_d_n8, assign78940_e119442_d_n9, assign78940_e119442_d_n10, assign78940_e119442_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign78940_e119438: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign78940_e119439: f64 = (1.0 + assign78940_e119438);
        let assign78940_e119440: f64 = (0.5 * assign78940_e119439);
        (assign78940_e119440, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign78940_e119442;
        locals.var_t9_dn0 = assign78940_e119442_d_n0;
        locals.var_t9_dn2 = assign78940_e119442_d_n2;
        locals.var_t9_dn4 = assign78940_e119442_d_n4;
        locals.var_t9_dn5 = assign78940_e119442_d_n5;
        locals.var_t9_dn6 = assign78940_e119442_d_n6;
        locals.var_t9_dn7 = assign78940_e119442_d_n7;
        locals.var_t9_dn8 = assign78940_e119442_d_n8;
        locals.var_t9_dn9 = assign78940_e119442_d_n9;
        locals.var_t9_dn10 = assign78940_e119442_d_n10;
        locals.var_t9_dn13 = assign78940_e119442_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign78950_e119455, assign78950_e119455_d_n0, assign78950_e119455_d_n2, assign78950_e119455_d_n4, assign78950_e119455_d_n5, assign78950_e119455_d_n6, assign78950_e119455_d_n7, assign78950_e119455_d_n8, assign78950_e119455_d_n9, assign78950_e119455_d_n10, assign78950_e119455_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign78950_e119452: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign78950_e119453: f64 = (0.5 * assign78950_e119452);
        (assign78950_e119453, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign78950_e119455;
        locals.var_t2_dn0 = assign78950_e119455_d_n0;
        locals.var_t2_dn2 = assign78950_e119455_d_n2;
        locals.var_t2_dn4 = assign78950_e119455_d_n4;
        locals.var_t2_dn5 = assign78950_e119455_d_n5;
        locals.var_t2_dn6 = assign78950_e119455_d_n6;
        locals.var_t2_dn7 = assign78950_e119455_d_n7;
        locals.var_t2_dn8 = assign78950_e119455_d_n8;
        locals.var_t2_dn9 = assign78950_e119455_d_n9;
        locals.var_t2_dn10 = assign78950_e119455_d_n10;
        locals.var_t2_dn13 = assign78950_e119455_d_n13;
        locals.var_t2_rv = 0.0;

        let assign78960_e119458: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1828 = assign78960_e119458;
        locals.var_guard1828_rv = 0.0;

        let (assign78970_e119469, assign78970_e119469_d_n0, assign78970_e119469_d_n2, assign78970_e119469_d_n4, assign78970_e119469_d_n5, assign78970_e119469_d_n6, assign78970_e119469_d_n7, assign78970_e119469_d_n8, assign78970_e119469_d_n9, assign78970_e119469_d_n10, assign78970_e119469_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1828 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign78970_e119469;
        locals.var_t2_dn0 = assign78970_e119469_d_n0;
        locals.var_t2_dn2 = assign78970_e119469_d_n2;
        locals.var_t2_dn4 = assign78970_e119469_d_n4;
        locals.var_t2_dn5 = assign78970_e119469_d_n5;
        locals.var_t2_dn6 = assign78970_e119469_d_n6;
        locals.var_t2_dn7 = assign78970_e119469_d_n7;
        locals.var_t2_dn8 = assign78970_e119469_d_n8;
        locals.var_t2_dn9 = assign78970_e119469_d_n9;
        locals.var_t2_dn10 = assign78970_e119469_d_n10;
        locals.var_t2_dn13 = assign78970_e119469_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign78980_e119480, assign78980_e119480_d_n0, assign78980_e119480_d_n2, assign78980_e119480_d_n4, assign78980_e119480_d_n5, assign78980_e119480_d_n6, assign78980_e119480_d_n7, assign78980_e119480_d_n8, assign78980_e119480_d_n9, assign78980_e119480_d_n10, assign78980_e119480_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1828 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign78980_e119480;
        locals.var_t9_dn0 = assign78980_e119480_d_n0;
        locals.var_t9_dn2 = assign78980_e119480_d_n2;
        locals.var_t9_dn4 = assign78980_e119480_d_n4;
        locals.var_t9_dn5 = assign78980_e119480_d_n5;
        locals.var_t9_dn6 = assign78980_e119480_d_n6;
        locals.var_t9_dn7 = assign78980_e119480_d_n7;
        locals.var_t9_dn8 = assign78980_e119480_d_n8;
        locals.var_t9_dn9 = assign78980_e119480_d_n9;
        locals.var_t9_dn10 = assign78980_e119480_d_n10;
        locals.var_t9_dn13 = assign78980_e119480_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign78990_e119489, assign78990_e119489_d_n0, assign78990_e119489_d_n2, assign78990_e119489_d_n4, assign78990_e119489_d_n5, assign78990_e119489_d_n6, assign78990_e119489_d_n7, assign78990_e119489_d_n8, assign78990_e119489_d_n9, assign78990_e119489_d_n10, assign78990_e119489_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign78990_e119489;
        locals.var_ddriftldc_dn0 = assign78990_e119489_d_n0;
        locals.var_ddriftldc_dn2 = assign78990_e119489_d_n2;
        locals.var_ddriftldc_dn4 = assign78990_e119489_d_n4;
        locals.var_ddriftldc_dn5 = assign78990_e119489_d_n5;
        locals.var_ddriftldc_dn6 = assign78990_e119489_d_n6;
        locals.var_ddriftldc_dn7 = assign78990_e119489_d_n7;
        locals.var_ddriftldc_dn8 = assign78990_e119489_d_n8;
        locals.var_ddriftldc_dn9 = assign78990_e119489_d_n9;
        locals.var_ddriftldc_dn10 = assign78990_e119489_d_n10;
        locals.var_ddriftldc_dn13 = assign78990_e119489_d_n13;
        locals.var_ddriftldc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_287(
        locals: &mut StampLocals,
    ) {
        let (assign79000_e119506, assign79000_e119506_d_n0, assign79000_e119506_d_n2, assign79000_e119506_d_n4, assign79000_e119506_d_n5, assign79000_e119506_d_n6, assign79000_e119506_d_n7, assign79000_e119506_d_n8, assign79000_e119506_d_n9, assign79000_e119506_d_n10, assign79000_e119506_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79000_e119498: f64 = (locals.var_q_nsubld__blk1762 * locals.var_ddriftldc);
        let assign79000_e119500: f64 = (assign79000_e119498 * locals.var_ddriftldc);
        let assign79000_e119502: f64 = (assign79000_e119500 / 2.0);
        let assign79000_e119504: f64 = (assign79000_e119502 / 1.034943e-10);
        (assign79000_e119504, (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign79000_e119498 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign79000_e119506;
        locals.var_dphi_sb_dn0 = assign79000_e119506_d_n0;
        locals.var_dphi_sb_dn2 = assign79000_e119506_d_n2;
        locals.var_dphi_sb_dn4 = assign79000_e119506_d_n4;
        locals.var_dphi_sb_dn5 = assign79000_e119506_d_n5;
        locals.var_dphi_sb_dn6 = assign79000_e119506_d_n6;
        locals.var_dphi_sb_dn7 = assign79000_e119506_d_n7;
        locals.var_dphi_sb_dn8 = assign79000_e119506_d_n8;
        locals.var_dphi_sb_dn9 = assign79000_e119506_d_n9;
        locals.var_dphi_sb_dn10 = assign79000_e119506_d_n10;
        locals.var_dphi_sb_dn13 = assign79000_e119506_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign79010_e119520, assign79010_e119520_d_n0, assign79010_e119520_d_n2, assign79010_e119520_d_n4, assign79010_e119520_d_n5, assign79010_e119520_d_n6, assign79010_e119520_d_n7, assign79010_e119520_d_n8, assign79010_e119520_d_n9, assign79010_e119520_d_n10, assign79010_e119520_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79010_e119515: f64 = (2.0 * locals.var_beta);
        let assign79010_e119517: f64 = (assign79010_e119515 * locals.var_dphi_sb);
        let assign79010_e119518: f64 = (assign79010_e119517).sqrt();
        (assign79010_e119518, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn0)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn2)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn4)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn5)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn6)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn7)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn8)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn9)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn10)) / (2.0 * assign79010_e119518)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign79010_e119515 * locals.var_dphi_sb_dn13)) / (2.0 * assign79010_e119518)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign79010_e119520;
        locals.var_t0_dn0 = assign79010_e119520_d_n0;
        locals.var_t0_dn2 = assign79010_e119520_d_n2;
        locals.var_t0_dn4 = assign79010_e119520_d_n4;
        locals.var_t0_dn5 = assign79010_e119520_d_n5;
        locals.var_t0_dn6 = assign79010_e119520_d_n6;
        locals.var_t0_dn7 = assign79010_e119520_d_n7;
        locals.var_t0_dn8 = assign79010_e119520_d_n8;
        locals.var_t0_dn9 = assign79010_e119520_d_n9;
        locals.var_t0_dn10 = assign79010_e119520_d_n10;
        locals.var_t0_dn13 = assign79010_e119520_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign79020_e119536, assign79020_e119536_d_n0, assign79020_e119536_d_n2, assign79020_e119536_d_n4, assign79020_e119536_d_n5, assign79020_e119536_d_n6, assign79020_e119536_d_n7, assign79020_e119536_d_n8, assign79020_e119536_d_n9, assign79020_e119536_d_n10, assign79020_e119536_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79020_e119528: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79020_e119530: f64 = (-locals.var_t0);
        let assign79020_e119531: f64 = { let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79020_e119532: f64 = (assign79020_e119528 + assign79020_e119531);
        let assign79020_e119534: f64 = (assign79020_e119532 / 2.0);
        (assign79020_e119534, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79020_e119536;
        locals.var_t1_dn0 = assign79020_e119536_d_n0;
        locals.var_t1_dn2 = assign79020_e119536_d_n2;
        locals.var_t1_dn4 = assign79020_e119536_d_n4;
        locals.var_t1_dn5 = assign79020_e119536_d_n5;
        locals.var_t1_dn6 = assign79020_e119536_d_n6;
        locals.var_t1_dn7 = assign79020_e119536_d_n7;
        locals.var_t1_dn8 = assign79020_e119536_d_n8;
        locals.var_t1_dn9 = assign79020_e119536_d_n9;
        locals.var_t1_dn10 = assign79020_e119536_d_n10;
        locals.var_t1_dn13 = assign79020_e119536_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign79030_e119548, assign79030_e119548_d_n0, assign79030_e119548_d_n2, assign79030_e119548_d_n4, assign79030_e119548_d_n5, assign79030_e119548_d_n6, assign79030_e119548_d_n7, assign79030_e119548_d_n8, assign79030_e119548_d_n9, assign79030_e119548_d_n10, assign79030_e119548_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79030_e119544: f64 = (locals.var_t1).ln();
        let assign79030_e119546: f64 = (assign79030_e119544 / locals.var_dphi_sb);
        (assign79030_e119546, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign79030_e119548;
        locals.var_c_sb_dn0 = assign79030_e119548_d_n0;
        locals.var_c_sb_dn2 = assign79030_e119548_d_n2;
        locals.var_c_sb_dn4 = assign79030_e119548_d_n4;
        locals.var_c_sb_dn5 = assign79030_e119548_d_n5;
        locals.var_c_sb_dn6 = assign79030_e119548_d_n6;
        locals.var_c_sb_dn7 = assign79030_e119548_d_n7;
        locals.var_c_sb_dn8 = assign79030_e119548_d_n8;
        locals.var_c_sb_dn9 = assign79030_e119548_d_n9;
        locals.var_c_sb_dn10 = assign79030_e119548_d_n10;
        locals.var_c_sb_dn13 = assign79030_e119548_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign79040_e119557,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign79040_e119557;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_288(
        locals: &mut StampLocals,
    ) {
        let mut assign79050_loop_guard: usize = 0;
        while {
            let assign79050_cond_e119567: f64 = (locals.var_lp_s0_max + 1.0);
            let assign79050_cond_e119569: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_lp_s0 <= assign79050_cond_e119567)) { 1.0 } else { 0.0 };
            assign79050_cond_e119569 != 0.0
        } {
            assign79050_loop_guard += 1;
            assert!(assign79050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign79050_body3_e119605, assign79050_body3_e119605_d_n0, assign79050_body3_e119605_d_n2, assign79050_body3_e119605_d_n4, assign79050_body3_e119605_d_n5, assign79050_body3_e119605_d_n6, assign79050_body3_e119605_d_n7, assign79050_body3_e119605_d_n8, assign79050_body3_e119605_d_n9, assign79050_body3_e119605_d_n10, assign79050_body3_e119605_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body3_e119603: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign79050_body3_e119603, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign79050_body3_e119605;
            locals.var_ps0ld_vxb_dn0 = assign79050_body3_e119605_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign79050_body3_e119605_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign79050_body3_e119605_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign79050_body3_e119605_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign79050_body3_e119605_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign79050_body3_e119605_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign79050_body3_e119605_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign79050_body3_e119605_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign79050_body3_e119605_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign79050_body3_e119605_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign79050_body4_e119616, assign79050_body4_e119616_d_n0, assign79050_body4_e119616_d_n2, assign79050_body4_e119616_d_n4, assign79050_body4_e119616_d_n5, assign79050_body4_e119616_d_n6, assign79050_body4_e119616_d_n7, assign79050_body4_e119616_d_n8, assign79050_body4_e119616_d_n9, assign79050_body4_e119616_d_n10, assign79050_body4_e119616_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body4_e119614: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign79050_body4_e119614, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign79050_body4_e119616;
            locals.var_chi_dn0 = assign79050_body4_e119616_d_n0;
            locals.var_chi_dn2 = assign79050_body4_e119616_d_n2;
            locals.var_chi_dn4 = assign79050_body4_e119616_d_n4;
            locals.var_chi_dn5 = assign79050_body4_e119616_d_n5;
            locals.var_chi_dn6 = assign79050_body4_e119616_d_n6;
            locals.var_chi_dn7 = assign79050_body4_e119616_d_n7;
            locals.var_chi_dn8 = assign79050_body4_e119616_d_n8;
            locals.var_chi_dn9 = assign79050_body4_e119616_d_n9;
            locals.var_chi_dn10 = assign79050_body4_e119616_d_n10;
            locals.var_chi_dn13 = assign79050_body4_e119616_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign79050_body5_e119629, assign79050_body5_e119629_d_n0, assign79050_body5_e119629_d_n2, assign79050_body5_e119629_d_n4, assign79050_body5_e119629_d_n5, assign79050_body5_e119629_d_n6, assign79050_body5_e119629_d_n7, assign79050_body5_e119629_d_n8, assign79050_body5_e119629_d_n9, assign79050_body5_e119629_d_n10, assign79050_body5_e119629_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body5_e119626: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign79050_body5_e119627: f64 = (locals.var_c_sb * assign79050_body5_e119626);
        (assign79050_body5_e119627, ((locals.var_c_sb_dn0 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign79050_body5_e119629;
            locals.var_ty_dn0 = assign79050_body5_e119629_d_n0;
            locals.var_ty_dn2 = assign79050_body5_e119629_d_n2;
            locals.var_ty_dn4 = assign79050_body5_e119629_d_n4;
            locals.var_ty_dn5 = assign79050_body5_e119629_d_n5;
            locals.var_ty_dn6 = assign79050_body5_e119629_d_n6;
            locals.var_ty_dn7 = assign79050_body5_e119629_d_n7;
            locals.var_ty_dn8 = assign79050_body5_e119629_d_n8;
            locals.var_ty_dn9 = assign79050_body5_e119629_d_n9;
            locals.var_ty_dn10 = assign79050_body5_e119629_d_n10;
            locals.var_ty_dn13 = assign79050_body5_e119629_d_n13;
            locals.var_ty_rv = 0.0;
            let assign79050_body6_e119632: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1830 = assign79050_body6_e119632;
            locals.var_guard1830_rv = 0.0;
            let (assign79050_body7_e119644, assign79050_body7_e119644_d_n0, assign79050_body7_e119644_d_n2, assign79050_body7_e119644_d_n4, assign79050_body7_e119644_d_n5, assign79050_body7_e119644_d_n6, assign79050_body7_e119644_d_n7, assign79050_body7_e119644_d_n8, assign79050_body7_e119644_d_n9, assign79050_body7_e119644_d_n10, assign79050_body7_e119644_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body7_e119642: f64 = (locals.var_ty).exp();
        (assign79050_body7_e119642, (assign79050_body7_e119642 * locals.var_ty_dn0), (assign79050_body7_e119642 * locals.var_ty_dn2), (assign79050_body7_e119642 * locals.var_ty_dn4), (assign79050_body7_e119642 * locals.var_ty_dn5), (assign79050_body7_e119642 * locals.var_ty_dn6), (assign79050_body7_e119642 * locals.var_ty_dn7), (assign79050_body7_e119642 * locals.var_ty_dn8), (assign79050_body7_e119642 * locals.var_ty_dn9), (assign79050_body7_e119642 * locals.var_ty_dn10), (assign79050_body7_e119642 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body7_e119644;
            locals.var_t1_dn0 = assign79050_body7_e119644_d_n0;
            locals.var_t1_dn2 = assign79050_body7_e119644_d_n2;
            locals.var_t1_dn4 = assign79050_body7_e119644_d_n4;
            locals.var_t1_dn5 = assign79050_body7_e119644_d_n5;
            locals.var_t1_dn6 = assign79050_body7_e119644_d_n6;
            locals.var_t1_dn7 = assign79050_body7_e119644_d_n7;
            locals.var_t1_dn8 = assign79050_body7_e119644_d_n8;
            locals.var_t1_dn9 = assign79050_body7_e119644_d_n9;
            locals.var_t1_dn10 = assign79050_body7_e119644_d_n10;
            locals.var_t1_dn13 = assign79050_body7_e119644_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79050_body8_e119659, assign79050_body8_e119659_d_n0, assign79050_body8_e119659_d_n2, assign79050_body8_e119659_d_n4, assign79050_body8_e119659_d_n5, assign79050_body8_e119659_d_n6, assign79050_body8_e119659_d_n7, assign79050_body8_e119659_d_n8, assign79050_body8_e119659_d_n9, assign79050_body8_e119659_d_n10, assign79050_body8_e119659_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body8_e119654: f64 = (-locals.var_c_sb);
        let assign79050_body8_e119656: f64 = (assign79050_body8_e119654 * locals.var_dphi_sb);
        let assign79050_body8_e119657: f64 = (assign79050_body8_e119656).exp();
        (assign79050_body8_e119657, (assign79050_body8_e119657 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn0))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn2))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn4))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn5))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn6))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn7))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn8))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn9))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn10))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body8_e119659;
            locals.var_t0_dn0 = assign79050_body8_e119659_d_n0;
            locals.var_t0_dn2 = assign79050_body8_e119659_d_n2;
            locals.var_t0_dn4 = assign79050_body8_e119659_d_n4;
            locals.var_t0_dn5 = assign79050_body8_e119659_d_n5;
            locals.var_t0_dn6 = assign79050_body8_e119659_d_n6;
            locals.var_t0_dn7 = assign79050_body8_e119659_d_n7;
            locals.var_t0_dn8 = assign79050_body8_e119659_d_n8;
            locals.var_t0_dn9 = assign79050_body8_e119659_d_n9;
            locals.var_t0_dn10 = assign79050_body8_e119659_d_n10;
            locals.var_t0_dn13 = assign79050_body8_e119659_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79050_body9_e119672, assign79050_body9_e119672_d_n0, assign79050_body9_e119672_d_n2, assign79050_body9_e119672_d_n4, assign79050_body9_e119672_d_n5, assign79050_body9_e119672_d_n6, assign79050_body9_e119672_d_n7, assign79050_body9_e119672_d_n8, assign79050_body9_e119672_d_n9, assign79050_body9_e119672_d_n10, assign79050_body9_e119672_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body9_e119670: f64 = (locals.var_t1 - locals.var_t0);
        (assign79050_body9_e119670, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79050_body9_e119672;
            locals.var_t2_dn0 = assign79050_body9_e119672_d_n0;
            locals.var_t2_dn2 = assign79050_body9_e119672_d_n2;
            locals.var_t2_dn4 = assign79050_body9_e119672_d_n4;
            locals.var_t2_dn5 = assign79050_body9_e119672_d_n5;
            locals.var_t2_dn6 = assign79050_body9_e119672_d_n6;
            locals.var_t2_dn7 = assign79050_body9_e119672_d_n7;
            locals.var_t2_dn8 = assign79050_body9_e119672_d_n8;
            locals.var_t2_dn9 = assign79050_body9_e119672_d_n9;
            locals.var_t2_dn10 = assign79050_body9_e119672_d_n10;
            locals.var_t2_dn13 = assign79050_body9_e119672_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign79050_body10_e119688, assign79050_body10_e119688_d_n0, assign79050_body10_e119688_d_n2, assign79050_body10_e119688_d_n4, assign79050_body10_e119688_d_n5, assign79050_body10_e119688_d_n6, assign79050_body10_e119688_d_n7, assign79050_body10_e119688_d_n8, assign79050_body10_e119688_d_n9, assign79050_body10_e119688_d_n10, assign79050_body10_e119688_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body10_e119683: f64 = (1.0 + locals.var_t2);
        let assign79050_body10_e119684: f64 = (assign79050_body10_e119683).ln();
        let assign79050_body10_e119686: f64 = (assign79050_body10_e119684 / locals.var_c_sb);
        (assign79050_body10_e119686, ((((locals.var_t2_dn0 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79050_body10_e119688;
            locals.var_phi_b_dn0 = assign79050_body10_e119688_d_n0;
            locals.var_phi_b_dn2 = assign79050_body10_e119688_d_n2;
            locals.var_phi_b_dn4 = assign79050_body10_e119688_d_n4;
            locals.var_phi_b_dn5 = assign79050_body10_e119688_d_n5;
            locals.var_phi_b_dn6 = assign79050_body10_e119688_d_n6;
            locals.var_phi_b_dn7 = assign79050_body10_e119688_d_n7;
            locals.var_phi_b_dn8 = assign79050_body10_e119688_d_n8;
            locals.var_phi_b_dn9 = assign79050_body10_e119688_d_n9;
            locals.var_phi_b_dn10 = assign79050_body10_e119688_d_n10;
            locals.var_phi_b_dn13 = assign79050_body10_e119688_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign79050_body11_e119703, assign79050_body11_e119703_d_n0, assign79050_body11_e119703_d_n2, assign79050_body11_e119703_d_n4, assign79050_body11_e119703_d_n5, assign79050_body11_e119703_d_n6, assign79050_body11_e119703_d_n7, assign79050_body11_e119703_d_n8, assign79050_body11_e119703_d_n9, assign79050_body11_e119703_d_n10, assign79050_body11_e119703_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body11_e119700: f64 = (1.0 + locals.var_t2);
        let assign79050_body11_e119701: f64 = (locals.var_t1 / assign79050_body11_e119700);
        (assign79050_body11_e119701, (((locals.var_t1_dn0 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn0)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn2 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn2)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn4 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn4)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn5 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn5)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn6 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn6)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn7 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn7)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn8 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn8)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn9 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn9)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn10 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn10)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn13 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn13)) / (assign79050_body11_e119700 * assign79050_body11_e119700)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79050_body11_e119703;
            locals.var_phi_b_dpss_dn0 = assign79050_body11_e119703_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79050_body11_e119703_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79050_body11_e119703_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79050_body11_e119703_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79050_body11_e119703_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79050_body11_e119703_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79050_body11_e119703_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79050_body11_e119703_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79050_body11_e119703_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79050_body11_e119703_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign79050_body13_e119731, assign79050_body13_e119731_d_n0, assign79050_body13_e119731_d_n2, assign79050_body13_e119731_d_n4, assign79050_body13_e119731_d_n5, assign79050_body13_e119731_d_n6, assign79050_body13_e119731_d_n7, assign79050_body13_e119731_d_n8, assign79050_body13_e119731_d_n9, assign79050_body13_e119731_d_n10, assign79050_body13_e119731_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        let assign79050_body13_e119729: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign79050_body13_e119729, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79050_body13_e119731;
            locals.var_phi_b_dn0 = assign79050_body13_e119731_d_n0;
            locals.var_phi_b_dn2 = assign79050_body13_e119731_d_n2;
            locals.var_phi_b_dn4 = assign79050_body13_e119731_d_n4;
            locals.var_phi_b_dn5 = assign79050_body13_e119731_d_n5;
            locals.var_phi_b_dn6 = assign79050_body13_e119731_d_n6;
            locals.var_phi_b_dn7 = assign79050_body13_e119731_d_n7;
            locals.var_phi_b_dn8 = assign79050_body13_e119731_d_n8;
            locals.var_phi_b_dn9 = assign79050_body13_e119731_d_n9;
            locals.var_phi_b_dn10 = assign79050_body13_e119731_d_n10;
            locals.var_phi_b_dn13 = assign79050_body13_e119731_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign79050_body14_e119743, assign79050_body14_e119743_d_n0, assign79050_body14_e119743_d_n2, assign79050_body14_e119743_d_n4, assign79050_body14_e119743_d_n5, assign79050_body14_e119743_d_n6, assign79050_body14_e119743_d_n7, assign79050_body14_e119743_d_n8, assign79050_body14_e119743_d_n9, assign79050_body14_e119743_d_n10, assign79050_body14_e119743_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79050_body14_e119743;
            locals.var_phi_b_dpss_dn0 = assign79050_body14_e119743_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79050_body14_e119743_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79050_body14_e119743_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79050_body14_e119743_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79050_body14_e119743_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79050_body14_e119743_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79050_body14_e119743_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79050_body14_e119743_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79050_body14_e119743_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79050_body14_e119743_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign79050_body15_e119754, assign79050_body15_e119754_d_n0, assign79050_body15_e119754_d_n2, assign79050_body15_e119754_d_n4, assign79050_body15_e119754_d_n5, assign79050_body15_e119754_d_n6, assign79050_body15_e119754_d_n7, assign79050_body15_e119754_d_n8, assign79050_body15_e119754_d_n9, assign79050_body15_e119754_d_n10, assign79050_body15_e119754_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body15_e119752: f64 = (locals.var_beta * locals.var_phi_b);
        (assign79050_body15_e119752, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign79050_body15_e119754;
            locals.var_chib_dn0 = assign79050_body15_e119754_d_n0;
            locals.var_chib_dn2 = assign79050_body15_e119754_d_n2;
            locals.var_chib_dn4 = assign79050_body15_e119754_d_n4;
            locals.var_chib_dn5 = assign79050_body15_e119754_d_n5;
            locals.var_chib_dn6 = assign79050_body15_e119754_d_n6;
            locals.var_chib_dn7 = assign79050_body15_e119754_d_n7;
            locals.var_chib_dn8 = assign79050_body15_e119754_d_n8;
            locals.var_chib_dn9 = assign79050_body15_e119754_d_n9;
            locals.var_chib_dn10 = assign79050_body15_e119754_d_n10;
            locals.var_chib_dn13 = assign79050_body15_e119754_d_n13;
            locals.var_chib_rv = 0.0;
            let assign79050_body16_e119757: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1831 = assign79050_body16_e119757;
            locals.var_guard1831_rv = 0.0;
            let (assign79050_body18_e119782, assign79050_body18_e119782_d_n0, assign79050_body18_e119782_d_n2, assign79050_body18_e119782_d_n4, assign79050_body18_e119782_d_n5, assign79050_body18_e119782_d_n6, assign79050_body18_e119782_d_n7, assign79050_body18_e119782_d_n8, assign79050_body18_e119782_d_n9, assign79050_body18_e119782_d_n10, assign79050_body18_e119782_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 != 0.0)) {
        let assign79050_body18_e119780: f64 = (-0.7071067811865475);
        (assign79050_body18_e119780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body18_e119782;
            locals.var_t0_dn0 = assign79050_body18_e119782_d_n0;
            locals.var_t0_dn2 = assign79050_body18_e119782_d_n2;
            locals.var_t0_dn4 = assign79050_body18_e119782_d_n4;
            locals.var_t0_dn5 = assign79050_body18_e119782_d_n5;
            locals.var_t0_dn6 = assign79050_body18_e119782_d_n6;
            locals.var_t0_dn7 = assign79050_body18_e119782_d_n7;
            locals.var_t0_dn8 = assign79050_body18_e119782_d_n8;
            locals.var_t0_dn9 = assign79050_body18_e119782_d_n9;
            locals.var_t0_dn10 = assign79050_body18_e119782_d_n10;
            locals.var_t0_dn13 = assign79050_body18_e119782_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79050_body19_e119795, assign79050_body19_e119795_d_n0, assign79050_body19_e119795_d_n2, assign79050_body19_e119795_d_n4, assign79050_body19_e119795_d_n5, assign79050_body19_e119795_d_n6, assign79050_body19_e119795_d_n7, assign79050_body19_e119795_d_n8, assign79050_body19_e119795_d_n9, assign79050_body19_e119795_d_n10, assign79050_body19_e119795_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 != 0.0)) {
        let assign79050_body19_e119793: f64 = (locals.var_chi * locals.var_t0);
        (assign79050_body19_e119793, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body19_e119795;
            locals.var_fb_dn0 = assign79050_body19_e119795_d_n0;
            locals.var_fb_dn2 = assign79050_body19_e119795_d_n2;
            locals.var_fb_dn4 = assign79050_body19_e119795_d_n4;
            locals.var_fb_dn5 = assign79050_body19_e119795_d_n5;
            locals.var_fb_dn6 = assign79050_body19_e119795_d_n6;
            locals.var_fb_dn7 = assign79050_body19_e119795_d_n7;
            locals.var_fb_dn8 = assign79050_body19_e119795_d_n8;
            locals.var_fb_dn9 = assign79050_body19_e119795_d_n9;
            locals.var_fb_dn10 = assign79050_body19_e119795_d_n10;
            locals.var_fb_dn13 = assign79050_body19_e119795_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign79050_body20_e119808, assign79050_body20_e119808_d_n0, assign79050_body20_e119808_d_n2, assign79050_body20_e119808_d_n4, assign79050_body20_e119808_d_n5, assign79050_body20_e119808_d_n6, assign79050_body20_e119808_d_n7, assign79050_body20_e119808_d_n8, assign79050_body20_e119808_d_n9, assign79050_body20_e119808_d_n10, assign79050_body20_e119808_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 != 0.0)) {
        let assign79050_body20_e119806: f64 = (locals.var_beta * locals.var_t0);
        (assign79050_body20_e119806, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body20_e119808;
            locals.var_fb_dpss_dn0 = assign79050_body20_e119808_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body20_e119808_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body20_e119808_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body20_e119808_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body20_e119808_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body20_e119808_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body20_e119808_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body20_e119808_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body20_e119808_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body20_e119808_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign79050_body21_e119811: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1832 = assign79050_body21_e119811;
            locals.var_guard1832_rv = 0.0;
            let (assign79050_body23_e119863, assign79050_body23_e119863_d_n0, assign79050_body23_e119863_d_n2, assign79050_body23_e119863_d_n4, assign79050_body23_e119863_d_n5, assign79050_body23_e119863_d_n6, assign79050_body23_e119863_d_n7, assign79050_body23_e119863_d_n8, assign79050_body23_e119863_d_n9, assign79050_body23_e119863_d_n10, assign79050_body23_e119863_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body23_e119841: f64 = (locals.var_chi * locals.var_chi);
        let assign79050_body23_e119843: f64 = (assign79050_body23_e119841 / 2.0);
        let assign79050_body23_e119847: f64 = (locals.var_chi / 3.0);
        let assign79050_body23_e119851: f64 = (locals.var_chi / 4.0);
        let assign79050_body23_e119855: f64 = (locals.var_chi / 5.0);
        let assign79050_body23_e119856: f64 = (1.0 - assign79050_body23_e119855);
        let assign79050_body23_e119857: f64 = (assign79050_body23_e119851 * assign79050_body23_e119856);
        let assign79050_body23_e119858: f64 = (1.0 - assign79050_body23_e119857);
        let assign79050_body23_e119859: f64 = (assign79050_body23_e119847 * assign79050_body23_e119858);
        let assign79050_body23_e119860: f64 = (1.0 - assign79050_body23_e119859);
        let assign79050_body23_e119861: f64 = (assign79050_body23_e119843 * assign79050_body23_e119860);
        (assign79050_body23_e119861, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn0 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn0 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn2 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn2 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn4 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn4 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn5 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn5 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn6 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn6 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn7 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn7 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn8 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn8 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn9 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn9 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn10 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn10 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn13 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn13 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body23_e119863;
            locals.var_t0_dn0 = assign79050_body23_e119863_d_n0;
            locals.var_t0_dn2 = assign79050_body23_e119863_d_n2;
            locals.var_t0_dn4 = assign79050_body23_e119863_d_n4;
            locals.var_t0_dn5 = assign79050_body23_e119863_d_n5;
            locals.var_t0_dn6 = assign79050_body23_e119863_d_n6;
            locals.var_t0_dn7 = assign79050_body23_e119863_d_n7;
            locals.var_t0_dn8 = assign79050_body23_e119863_d_n8;
            locals.var_t0_dn9 = assign79050_body23_e119863_d_n9;
            locals.var_t0_dn10 = assign79050_body23_e119863_d_n10;
            locals.var_t0_dn13 = assign79050_body23_e119863_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79050_body24_e119895, assign79050_body24_e119895_d_n0, assign79050_body24_e119895_d_n2, assign79050_body24_e119895_d_n4, assign79050_body24_e119895_d_n5, assign79050_body24_e119895_d_n6, assign79050_body24_e119895_d_n7, assign79050_body24_e119895_d_n8, assign79050_body24_e119895_d_n9, assign79050_body24_e119895_d_n10, assign79050_body24_e119895_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body24_e119879: f64 = (locals.var_chi / 2.0);
        let assign79050_body24_e119883: f64 = (locals.var_chi / 3.0);
        let assign79050_body24_e119887: f64 = (locals.var_chi / 4.0);
        let assign79050_body24_e119888: f64 = (1.0 - assign79050_body24_e119887);
        let assign79050_body24_e119889: f64 = (assign79050_body24_e119883 * assign79050_body24_e119888);
        let assign79050_body24_e119890: f64 = (1.0 - assign79050_body24_e119889);
        let assign79050_body24_e119891: f64 = (assign79050_body24_e119879 * assign79050_body24_e119890);
        let assign79050_body24_e119892: f64 = (1.0 - assign79050_body24_e119891);
        let assign79050_body24_e119893: f64 = (locals.var_chi * assign79050_body24_e119892);
        (assign79050_body24_e119893, ((locals.var_chi_dn0 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn0 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn2 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn4 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn5 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn6 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn7 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn8 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn9 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn10 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn13 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body24_e119895;
            locals.var_t1_dn0 = assign79050_body24_e119895_d_n0;
            locals.var_t1_dn2 = assign79050_body24_e119895_d_n2;
            locals.var_t1_dn4 = assign79050_body24_e119895_d_n4;
            locals.var_t1_dn5 = assign79050_body24_e119895_d_n5;
            locals.var_t1_dn6 = assign79050_body24_e119895_d_n6;
            locals.var_t1_dn7 = assign79050_body24_e119895_d_n7;
            locals.var_t1_dn8 = assign79050_body24_e119895_d_n8;
            locals.var_t1_dn9 = assign79050_body24_e119895_d_n9;
            locals.var_t1_dn10 = assign79050_body24_e119895_d_n10;
            locals.var_t1_dn13 = assign79050_body24_e119895_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79050_body25_e119931, assign79050_body25_e119931_d_n0, assign79050_body25_e119931_d_n2, assign79050_body25_e119931_d_n4, assign79050_body25_e119931_d_n5, assign79050_body25_e119931_d_n6, assign79050_body25_e119931_d_n7, assign79050_body25_e119931_d_n8, assign79050_body25_e119931_d_n9, assign79050_body25_e119931_d_n10, assign79050_body25_e119931_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body25_e119909: f64 = (locals.var_chib * locals.var_chib);
        let assign79050_body25_e119911: f64 = (assign79050_body25_e119909 / 2.0);
        let assign79050_body25_e119915: f64 = (locals.var_chib / 3.0);
        let assign79050_body25_e119919: f64 = (locals.var_chib / 4.0);
        let assign79050_body25_e119923: f64 = (locals.var_chib / 5.0);
        let assign79050_body25_e119924: f64 = (1.0 - assign79050_body25_e119923);
        let assign79050_body25_e119925: f64 = (assign79050_body25_e119919 * assign79050_body25_e119924);
        let assign79050_body25_e119926: f64 = (1.0 - assign79050_body25_e119925);
        let assign79050_body25_e119927: f64 = (assign79050_body25_e119915 * assign79050_body25_e119926);
        let assign79050_body25_e119928: f64 = (1.0 - assign79050_body25_e119927);
        let assign79050_body25_e119929: f64 = (assign79050_body25_e119911 * assign79050_body25_e119928);
        (assign79050_body25_e119929, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn0 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn0 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn2 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn2 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn4 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn4 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn5 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn5 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn6 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn6 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn7 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn7 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn8 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn8 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn9 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn9 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn10 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn10 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn13 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn13 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79050_body25_e119931;
            locals.var_t2_dn0 = assign79050_body25_e119931_d_n0;
            locals.var_t2_dn2 = assign79050_body25_e119931_d_n2;
            locals.var_t2_dn4 = assign79050_body25_e119931_d_n4;
            locals.var_t2_dn5 = assign79050_body25_e119931_d_n5;
            locals.var_t2_dn6 = assign79050_body25_e119931_d_n6;
            locals.var_t2_dn7 = assign79050_body25_e119931_d_n7;
            locals.var_t2_dn8 = assign79050_body25_e119931_d_n8;
            locals.var_t2_dn9 = assign79050_body25_e119931_d_n9;
            locals.var_t2_dn10 = assign79050_body25_e119931_d_n10;
            locals.var_t2_dn13 = assign79050_body25_e119931_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign79050_body26_e119963, assign79050_body26_e119963_d_n0, assign79050_body26_e119963_d_n2, assign79050_body26_e119963_d_n4, assign79050_body26_e119963_d_n5, assign79050_body26_e119963_d_n6, assign79050_body26_e119963_d_n7, assign79050_body26_e119963_d_n8, assign79050_body26_e119963_d_n9, assign79050_body26_e119963_d_n10, assign79050_body26_e119963_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body26_e119947: f64 = (locals.var_chib / 2.0);
        let assign79050_body26_e119951: f64 = (locals.var_chib / 3.0);
        let assign79050_body26_e119955: f64 = (locals.var_chib / 4.0);
        let assign79050_body26_e119956: f64 = (1.0 - assign79050_body26_e119955);
        let assign79050_body26_e119957: f64 = (assign79050_body26_e119951 * assign79050_body26_e119956);
        let assign79050_body26_e119958: f64 = (1.0 - assign79050_body26_e119957);
        let assign79050_body26_e119959: f64 = (assign79050_body26_e119947 * assign79050_body26_e119958);
        let assign79050_body26_e119960: f64 = (1.0 - assign79050_body26_e119959);
        let assign79050_body26_e119961: f64 = (locals.var_chib * assign79050_body26_e119960);
        (assign79050_body26_e119961, ((locals.var_chib_dn0 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn0 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn2 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn4 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn5 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn6 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn7 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn8 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn9 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn10 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn13 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign79050_body26_e119963;
            locals.var_t3_dn0 = assign79050_body26_e119963_d_n0;
            locals.var_t3_dn2 = assign79050_body26_e119963_d_n2;
            locals.var_t3_dn4 = assign79050_body26_e119963_d_n4;
            locals.var_t3_dn5 = assign79050_body26_e119963_d_n5;
            locals.var_t3_dn6 = assign79050_body26_e119963_d_n6;
            locals.var_t3_dn7 = assign79050_body26_e119963_d_n7;
            locals.var_t3_dn8 = assign79050_body26_e119963_d_n8;
            locals.var_t3_dn9 = assign79050_body26_e119963_d_n9;
            locals.var_t3_dn10 = assign79050_body26_e119963_d_n10;
            locals.var_t3_dn13 = assign79050_body26_e119963_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign79050_body27_e119979, assign79050_body27_e119979_d_n0, assign79050_body27_e119979_d_n2, assign79050_body27_e119979_d_n4, assign79050_body27_e119979_d_n5, assign79050_body27_e119979_d_n6, assign79050_body27_e119979_d_n7, assign79050_body27_e119979_d_n8, assign79050_body27_e119979_d_n9, assign79050_body27_e119979_d_n10, assign79050_body27_e119979_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body27_e119977: f64 = (locals.var_t0 - locals.var_t2);
        (assign79050_body27_e119977, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign79050_body27_e119979;
            locals.var_t4_dn0 = assign79050_body27_e119979_d_n0;
            locals.var_t4_dn2 = assign79050_body27_e119979_d_n2;
            locals.var_t4_dn4 = assign79050_body27_e119979_d_n4;
            locals.var_t4_dn5 = assign79050_body27_e119979_d_n5;
            locals.var_t4_dn6 = assign79050_body27_e119979_d_n6;
            locals.var_t4_dn7 = assign79050_body27_e119979_d_n7;
            locals.var_t4_dn8 = assign79050_body27_e119979_d_n8;
            locals.var_t4_dn9 = assign79050_body27_e119979_d_n9;
            locals.var_t4_dn10 = assign79050_body27_e119979_d_n10;
            locals.var_t4_dn13 = assign79050_body27_e119979_d_n13;
            locals.var_t4_rv = 0.0;
            let assign79050_body28_e119982: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1833 = assign79050_body28_e119982;
            locals.var_guard1833_rv = 0.0;
            let (assign79050_body29_e119999, assign79050_body29_e119999_d_n0, assign79050_body29_e119999_d_n2, assign79050_body29_e119999_d_n4, assign79050_body29_e119999_d_n5, assign79050_body29_e119999_d_n6, assign79050_body29_e119999_d_n7, assign79050_body29_e119999_d_n8, assign79050_body29_e119999_d_n9, assign79050_body29_e119999_d_n10, assign79050_body29_e119999_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 != 0.0)) {
        let assign79050_body29_e119997: f64 = (locals.var_t4).sqrt();
        (assign79050_body29_e119997, (locals.var_t4_dn0 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn2 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn4 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn5 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn6 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn7 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn8 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn9 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn10 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn13 / (2.0 * assign79050_body29_e119997)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body29_e119999;
            locals.var_fb_dn0 = assign79050_body29_e119999_d_n0;
            locals.var_fb_dn2 = assign79050_body29_e119999_d_n2;
            locals.var_fb_dn4 = assign79050_body29_e119999_d_n4;
            locals.var_fb_dn5 = assign79050_body29_e119999_d_n5;
            locals.var_fb_dn6 = assign79050_body29_e119999_d_n6;
            locals.var_fb_dn7 = assign79050_body29_e119999_d_n7;
            locals.var_fb_dn8 = assign79050_body29_e119999_d_n8;
            locals.var_fb_dn9 = assign79050_body29_e119999_d_n9;
            locals.var_fb_dn10 = assign79050_body29_e119999_d_n10;
            locals.var_fb_dn13 = assign79050_body29_e119999_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign79050_body30_e120025, assign79050_body30_e120025_d_n0, assign79050_body30_e120025_d_n2, assign79050_body30_e120025_d_n4, assign79050_body30_e120025_d_n5, assign79050_body30_e120025_d_n6, assign79050_body30_e120025_d_n7, assign79050_body30_e120025_d_n8, assign79050_body30_e120025_d_n9, assign79050_body30_e120025_d_n10, assign79050_body30_e120025_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 != 0.0)) {
        let assign79050_body30_e120015: f64 = (locals.var_beta * 0.5);
        let assign79050_body30_e120019: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign79050_body30_e120020: f64 = (locals.var_t1 - assign79050_body30_e120019);
        let assign79050_body30_e120021: f64 = (assign79050_body30_e120015 * assign79050_body30_e120020);
        let assign79050_body30_e120023: f64 = (assign79050_body30_e120021 / locals.var_fb);
        (assign79050_body30_e120023, ((((((locals.var_beta_dn0 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body30_e120025;
            locals.var_fb_dpss_dn0 = assign79050_body30_e120025_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body30_e120025_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body30_e120025_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body30_e120025_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body30_e120025_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body30_e120025_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body30_e120025_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body30_e120025_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body30_e120025_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body30_e120025_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign79050_body32_e120061, assign79050_body32_e120061_d_n0, assign79050_body32_e120061_d_n2, assign79050_body32_e120061_d_n4, assign79050_body32_e120061_d_n5, assign79050_body32_e120061_d_n6, assign79050_body32_e120061_d_n7, assign79050_body32_e120061_d_n8, assign79050_body32_e120061_d_n9, assign79050_body32_e120061_d_n10, assign79050_body32_e120061_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body32_e120061;
            locals.var_fb_dn0 = assign79050_body32_e120061_d_n0;
            locals.var_fb_dn2 = assign79050_body32_e120061_d_n2;
            locals.var_fb_dn4 = assign79050_body32_e120061_d_n4;
            locals.var_fb_dn5 = assign79050_body32_e120061_d_n5;
            locals.var_fb_dn6 = assign79050_body32_e120061_d_n6;
            locals.var_fb_dn7 = assign79050_body32_e120061_d_n7;
            locals.var_fb_dn8 = assign79050_body32_e120061_d_n8;
            locals.var_fb_dn9 = assign79050_body32_e120061_d_n9;
            locals.var_fb_dn10 = assign79050_body32_e120061_d_n10;
            locals.var_fb_dn13 = assign79050_body32_e120061_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign79050_body33_e120078, assign79050_body33_e120078_d_n0, assign79050_body33_e120078_d_n2, assign79050_body33_e120078_d_n4, assign79050_body33_e120078_d_n5, assign79050_body33_e120078_d_n6, assign79050_body33_e120078_d_n7, assign79050_body33_e120078_d_n8, assign79050_body33_e120078_d_n9, assign79050_body33_e120078_d_n10, assign79050_body33_e120078_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body33_e120078;
            locals.var_fb_dpss_dn0 = assign79050_body33_e120078_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body33_e120078_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body33_e120078_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body33_e120078_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body33_e120078_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body33_e120078_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body33_e120078_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body33_e120078_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body33_e120078_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body33_e120078_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign79050_body34_e120095, assign79050_body34_e120095_d_n0, assign79050_body34_e120095_d_n2, assign79050_body34_e120095_d_n4, assign79050_body34_e120095_d_n5, assign79050_body34_e120095_d_n6, assign79050_body34_e120095_d_n7, assign79050_body34_e120095_d_n8, assign79050_body34_e120095_d_n9, assign79050_body34_e120095_d_n10, assign79050_body34_e120095_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) {
        let assign79050_body34_e120092: f64 = (-locals.var_chi);
        let assign79050_body34_e120093: f64 = (assign79050_body34_e120092).exp();
        (assign79050_body34_e120093, (assign79050_body34_e120093 * (-locals.var_chi_dn0)), (assign79050_body34_e120093 * (-locals.var_chi_dn2)), (assign79050_body34_e120093 * (-locals.var_chi_dn4)), (assign79050_body34_e120093 * (-locals.var_chi_dn5)), (assign79050_body34_e120093 * (-locals.var_chi_dn6)), (assign79050_body34_e120093 * (-locals.var_chi_dn7)), (assign79050_body34_e120093 * (-locals.var_chi_dn8)), (assign79050_body34_e120093 * (-locals.var_chi_dn9)), (assign79050_body34_e120093 * (-locals.var_chi_dn10)), (assign79050_body34_e120093 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body34_e120095;
            locals.var_t0_dn0 = assign79050_body34_e120095_d_n0;
            locals.var_t0_dn2 = assign79050_body34_e120095_d_n2;
            locals.var_t0_dn4 = assign79050_body34_e120095_d_n4;
            locals.var_t0_dn5 = assign79050_body34_e120095_d_n5;
            locals.var_t0_dn6 = assign79050_body34_e120095_d_n6;
            locals.var_t0_dn7 = assign79050_body34_e120095_d_n7;
            locals.var_t0_dn8 = assign79050_body34_e120095_d_n8;
            locals.var_t0_dn9 = assign79050_body34_e120095_d_n9;
            locals.var_t0_dn10 = assign79050_body34_e120095_d_n10;
            locals.var_t0_dn13 = assign79050_body34_e120095_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79050_body35_e120112, assign79050_body35_e120112_d_n0, assign79050_body35_e120112_d_n2, assign79050_body35_e120112_d_n4, assign79050_body35_e120112_d_n5, assign79050_body35_e120112_d_n6, assign79050_body35_e120112_d_n7, assign79050_body35_e120112_d_n8, assign79050_body35_e120112_d_n9, assign79050_body35_e120112_d_n10, assign79050_body35_e120112_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) {
        let assign79050_body35_e120109: f64 = (-locals.var_chib);
        let assign79050_body35_e120110: f64 = (assign79050_body35_e120109).exp();
        (assign79050_body35_e120110, (assign79050_body35_e120110 * (-locals.var_chib_dn0)), (assign79050_body35_e120110 * (-locals.var_chib_dn2)), (assign79050_body35_e120110 * (-locals.var_chib_dn4)), (assign79050_body35_e120110 * (-locals.var_chib_dn5)), (assign79050_body35_e120110 * (-locals.var_chib_dn6)), (assign79050_body35_e120110 * (-locals.var_chib_dn7)), (assign79050_body35_e120110 * (-locals.var_chib_dn8)), (assign79050_body35_e120110 * (-locals.var_chib_dn9)), (assign79050_body35_e120110 * (-locals.var_chib_dn10)), (assign79050_body35_e120110 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body35_e120112;
            locals.var_t1_dn0 = assign79050_body35_e120112_d_n0;
            locals.var_t1_dn2 = assign79050_body35_e120112_d_n2;
            locals.var_t1_dn4 = assign79050_body35_e120112_d_n4;
            locals.var_t1_dn5 = assign79050_body35_e120112_d_n5;
            locals.var_t1_dn6 = assign79050_body35_e120112_d_n6;
            locals.var_t1_dn7 = assign79050_body35_e120112_d_n7;
            locals.var_t1_dn8 = assign79050_body35_e120112_d_n8;
            locals.var_t1_dn9 = assign79050_body35_e120112_d_n9;
            locals.var_t1_dn10 = assign79050_body35_e120112_d_n10;
            locals.var_t1_dn13 = assign79050_body35_e120112_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79050_body36_e120133, assign79050_body36_e120133_d_n0, assign79050_body36_e120133_d_n2, assign79050_body36_e120133_d_n4, assign79050_body36_e120133_d_n5, assign79050_body36_e120133_d_n6, assign79050_body36_e120133_d_n7, assign79050_body36_e120133_d_n8, assign79050_body36_e120133_d_n9, assign79050_body36_e120133_d_n10, assign79050_body36_e120133_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) {
        let assign79050_body36_e120127: f64 = (locals.var_chi - locals.var_chib);
        let assign79050_body36_e120130: f64 = (locals.var_t0 - locals.var_t1);
        let assign79050_body36_e120131: f64 = (assign79050_body36_e120127 + assign79050_body36_e120130);
        (assign79050_body36_e120131, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign79050_body36_e120133;
            locals.var_t4_dn0 = assign79050_body36_e120133_d_n0;
            locals.var_t4_dn2 = assign79050_body36_e120133_d_n2;
            locals.var_t4_dn4 = assign79050_body36_e120133_d_n4;
            locals.var_t4_dn5 = assign79050_body36_e120133_d_n5;
            locals.var_t4_dn6 = assign79050_body36_e120133_d_n6;
            locals.var_t4_dn7 = assign79050_body36_e120133_d_n7;
            locals.var_t4_dn8 = assign79050_body36_e120133_d_n8;
            locals.var_t4_dn9 = assign79050_body36_e120133_d_n9;
            locals.var_t4_dn10 = assign79050_body36_e120133_d_n10;
            locals.var_t4_dn13 = assign79050_body36_e120133_d_n13;
            locals.var_t4_rv = 0.0;
            let assign79050_body37_e120136: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1834 = assign79050_body37_e120136;
            locals.var_guard1834_rv = 0.0;
            let (assign79050_body38_e120154, assign79050_body38_e120154_d_n0, assign79050_body38_e120154_d_n2, assign79050_body38_e120154_d_n4, assign79050_body38_e120154_d_n5, assign79050_body38_e120154_d_n6, assign79050_body38_e120154_d_n7, assign79050_body38_e120154_d_n8, assign79050_body38_e120154_d_n9, assign79050_body38_e120154_d_n10, assign79050_body38_e120154_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79050_body38_e120152: f64 = (locals.var_t4).sqrt();
        (assign79050_body38_e120152, (locals.var_t4_dn0 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn2 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn4 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn5 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn6 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn7 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn8 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn9 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn10 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn13 / (2.0 * assign79050_body38_e120152)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body38_e120154;
            locals.var_fb_dn0 = assign79050_body38_e120154_d_n0;
            locals.var_fb_dn2 = assign79050_body38_e120154_d_n2;
            locals.var_fb_dn4 = assign79050_body38_e120154_d_n4;
            locals.var_fb_dn5 = assign79050_body38_e120154_d_n5;
            locals.var_fb_dn6 = assign79050_body38_e120154_d_n6;
            locals.var_fb_dn7 = assign79050_body38_e120154_d_n7;
            locals.var_fb_dn8 = assign79050_body38_e120154_d_n8;
            locals.var_fb_dn9 = assign79050_body38_e120154_d_n9;
            locals.var_fb_dn10 = assign79050_body38_e120154_d_n10;
            locals.var_fb_dn13 = assign79050_body38_e120154_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign79050_body39_e120185, assign79050_body39_e120185_d_n0, assign79050_body39_e120185_d_n2, assign79050_body39_e120185_d_n4, assign79050_body39_e120185_d_n5, assign79050_body39_e120185_d_n6, assign79050_body39_e120185_d_n7, assign79050_body39_e120185_d_n8, assign79050_body39_e120185_d_n9, assign79050_body39_e120185_d_n10, assign79050_body39_e120185_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79050_body39_e120171: f64 = (locals.var_beta * 0.5);
        let assign79050_body39_e120174: f64 = (1.0 - locals.var_t0);
        let assign79050_body39_e120178: f64 = (1.0 - locals.var_t1);
        let assign79050_body39_e120179: f64 = (locals.var_phi_b_dpss * assign79050_body39_e120178);
        let assign79050_body39_e120180: f64 = (assign79050_body39_e120174 - assign79050_body39_e120179);
        let assign79050_body39_e120181: f64 = (assign79050_body39_e120171 * assign79050_body39_e120180);
        let assign79050_body39_e120183: f64 = (assign79050_body39_e120181 / locals.var_fb);
        (assign79050_body39_e120183, ((((((locals.var_beta_dn0 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body39_e120185;
            locals.var_fb_dpss_dn0 = assign79050_body39_e120185_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body39_e120185_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body39_e120185_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body39_e120185_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body39_e120185_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body39_e120185_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body39_e120185_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body39_e120185_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body39_e120185_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body39_e120185_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign79050_body41_e120223, assign79050_body41_e120223_d_n0, assign79050_body41_e120223_d_n2, assign79050_body41_e120223_d_n4, assign79050_body41_e120223_d_n5, assign79050_body41_e120223_d_n6, assign79050_body41_e120223_d_n7, assign79050_body41_e120223_d_n8, assign79050_body41_e120223_d_n9, assign79050_body41_e120223_d_n10, assign79050_body41_e120223_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body41_e120223;
            locals.var_fb_dn0 = assign79050_body41_e120223_d_n0;
            locals.var_fb_dn2 = assign79050_body41_e120223_d_n2;
            locals.var_fb_dn4 = assign79050_body41_e120223_d_n4;
            locals.var_fb_dn5 = assign79050_body41_e120223_d_n5;
            locals.var_fb_dn6 = assign79050_body41_e120223_d_n6;
            locals.var_fb_dn7 = assign79050_body41_e120223_d_n7;
            locals.var_fb_dn8 = assign79050_body41_e120223_d_n8;
            locals.var_fb_dn9 = assign79050_body41_e120223_d_n9;
            locals.var_fb_dn10 = assign79050_body41_e120223_d_n10;
            locals.var_fb_dn13 = assign79050_body41_e120223_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign79050_body42_e120241, assign79050_body42_e120241_d_n0, assign79050_body42_e120241_d_n2, assign79050_body42_e120241_d_n4, assign79050_body42_e120241_d_n5, assign79050_body42_e120241_d_n6, assign79050_body42_e120241_d_n7, assign79050_body42_e120241_d_n8, assign79050_body42_e120241_d_n9, assign79050_body42_e120241_d_n10, assign79050_body42_e120241_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body42_e120241;
            locals.var_fb_dpss_dn0 = assign79050_body42_e120241_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body42_e120241_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body42_e120241_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body42_e120241_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body42_e120241_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body42_e120241_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body42_e120241_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body42_e120241_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body42_e120241_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body42_e120241_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign79050_body43_e120244: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1835 = assign79050_body43_e120244;
            locals.var_guard1835_rv = 0.0;
            let (assign79050_body45_e120268, assign79050_body45_e120268_d_n0, assign79050_body45_e120268_d_n2, assign79050_body45_e120268_d_n4, assign79050_body45_e120268_d_n5, assign79050_body45_e120268_d_n6, assign79050_body45_e120268_d_n7, assign79050_body45_e120268_d_n8, assign79050_body45_e120268_d_n9, assign79050_body45_e120268_d_n10, assign79050_body45_e120268_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body45_e120268;
            locals.var_fs01_dn0 = assign79050_body45_e120268_d_n0;
            locals.var_fs01_dn2 = assign79050_body45_e120268_d_n2;
            locals.var_fs01_dn4 = assign79050_body45_e120268_d_n4;
            locals.var_fs01_dn5 = assign79050_body45_e120268_d_n5;
            locals.var_fs01_dn6 = assign79050_body45_e120268_d_n6;
            locals.var_fs01_dn7 = assign79050_body45_e120268_d_n7;
            locals.var_fs01_dn8 = assign79050_body45_e120268_d_n8;
            locals.var_fs01_dn9 = assign79050_body45_e120268_d_n9;
            locals.var_fs01_dn10 = assign79050_body45_e120268_d_n10;
            locals.var_fs01_dn13 = assign79050_body45_e120268_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign79050_body46_e120279, assign79050_body46_e120279_d_n0, assign79050_body46_e120279_d_n2, assign79050_body46_e120279_d_n4, assign79050_body46_e120279_d_n5, assign79050_body46_e120279_d_n6, assign79050_body46_e120279_d_n7, assign79050_body46_e120279_d_n8, assign79050_body46_e120279_d_n9, assign79050_body46_e120279_d_n10, assign79050_body46_e120279_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body46_e120279;
            locals.var_fs01_dps0_dn0 = assign79050_body46_e120279_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body46_e120279_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body46_e120279_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body46_e120279_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body46_e120279_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body46_e120279_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body46_e120279_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body46_e120279_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body46_e120279_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body46_e120279_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign79050_body47_e120291, assign79050_body47_e120291_d_n0, assign79050_body47_e120291_d_n2, assign79050_body47_e120291_d_n4, assign79050_body47_e120291_d_n5, assign79050_body47_e120291_d_n6, assign79050_body47_e120291_d_n7, assign79050_body47_e120291_d_n8, assign79050_body47_e120291_d_n9, assign79050_body47_e120291_d_n10, assign79050_body47_e120291_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        let assign79050_body47_e120289: f64 = (-locals.var_fb);
        (assign79050_body47_e120289, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79050_body47_e120291;
            locals.var_fs02_dn0 = assign79050_body47_e120291_d_n0;
            locals.var_fs02_dn2 = assign79050_body47_e120291_d_n2;
            locals.var_fs02_dn4 = assign79050_body47_e120291_d_n4;
            locals.var_fs02_dn5 = assign79050_body47_e120291_d_n5;
            locals.var_fs02_dn6 = assign79050_body47_e120291_d_n6;
            locals.var_fs02_dn7 = assign79050_body47_e120291_d_n7;
            locals.var_fs02_dn8 = assign79050_body47_e120291_d_n8;
            locals.var_fs02_dn9 = assign79050_body47_e120291_d_n9;
            locals.var_fs02_dn10 = assign79050_body47_e120291_d_n10;
            locals.var_fs02_dn13 = assign79050_body47_e120291_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign79050_body48_e120303, assign79050_body48_e120303_d_n0, assign79050_body48_e120303_d_n2, assign79050_body48_e120303_d_n4, assign79050_body48_e120303_d_n5, assign79050_body48_e120303_d_n6, assign79050_body48_e120303_d_n7, assign79050_body48_e120303_d_n8, assign79050_body48_e120303_d_n9, assign79050_body48_e120303_d_n10, assign79050_body48_e120303_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        let assign79050_body48_e120301: f64 = (-locals.var_fb_dpss);
        (assign79050_body48_e120301, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79050_body48_e120303;
            locals.var_fs02_dps0_dn0 = assign79050_body48_e120303_d_n0;
            locals.var_fs02_dps0_dn2 = assign79050_body48_e120303_d_n2;
            locals.var_fs02_dps0_dn4 = assign79050_body48_e120303_d_n4;
            locals.var_fs02_dps0_dn5 = assign79050_body48_e120303_d_n5;
            locals.var_fs02_dps0_dn6 = assign79050_body48_e120303_d_n6;
            locals.var_fs02_dps0_dn7 = assign79050_body48_e120303_d_n7;
            locals.var_fs02_dps0_dn8 = assign79050_body48_e120303_d_n8;
            locals.var_fs02_dps0_dn9 = assign79050_body48_e120303_d_n9;
            locals.var_fs02_dps0_dn10 = assign79050_body48_e120303_d_n10;
            locals.var_fs02_dps0_dn13 = assign79050_body48_e120303_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign79050_body49_e120306: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1836 = assign79050_body49_e120306;
            locals.var_guard1836_rv = 0.0;
            let assign79050_body50_e120309: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1837 = assign79050_body50_e120309;
            locals.var_guard1837_rv = 0.0;
            let (assign79050_body51_e120347, assign79050_body51_e120347_d_n0, assign79050_body51_e120347_d_n2, assign79050_body51_e120347_d_n4, assign79050_body51_e120347_d_n5, assign79050_body51_e120347_d_n6, assign79050_body51_e120347_d_n7, assign79050_body51_e120347_d_n8, assign79050_body51_e120347_d_n9, assign79050_body51_e120347_d_n10, assign79050_body51_e120347_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body51_e120325: f64 = (locals.var_chi * locals.var_chi);
        let assign79050_body51_e120327: f64 = (assign79050_body51_e120325 / 2.0);
        let assign79050_body51_e120331: f64 = (locals.var_chi / 3.0);
        let assign79050_body51_e120335: f64 = (locals.var_chi / 4.0);
        let assign79050_body51_e120339: f64 = (locals.var_chi / 5.0);
        let assign79050_body51_e120340: f64 = (1.0 + assign79050_body51_e120339);
        let assign79050_body51_e120341: f64 = (assign79050_body51_e120335 * assign79050_body51_e120340);
        let assign79050_body51_e120342: f64 = (1.0 + assign79050_body51_e120341);
        let assign79050_body51_e120343: f64 = (assign79050_body51_e120331 * assign79050_body51_e120342);
        let assign79050_body51_e120344: f64 = (1.0 + assign79050_body51_e120343);
        let assign79050_body51_e120345: f64 = (assign79050_body51_e120327 * assign79050_body51_e120344);
        (assign79050_body51_e120345, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn0 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn0 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn2 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn2 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn4 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn4 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn5 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn5 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn6 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn6 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn7 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn7 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn8 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn8 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn9 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn9 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn10 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn10 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn13 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn13 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body51_e120347;
            locals.var_t0_dn0 = assign79050_body51_e120347_d_n0;
            locals.var_t0_dn2 = assign79050_body51_e120347_d_n2;
            locals.var_t0_dn4 = assign79050_body51_e120347_d_n4;
            locals.var_t0_dn5 = assign79050_body51_e120347_d_n5;
            locals.var_t0_dn6 = assign79050_body51_e120347_d_n6;
            locals.var_t0_dn7 = assign79050_body51_e120347_d_n7;
            locals.var_t0_dn8 = assign79050_body51_e120347_d_n8;
            locals.var_t0_dn9 = assign79050_body51_e120347_d_n9;
            locals.var_t0_dn10 = assign79050_body51_e120347_d_n10;
            locals.var_t0_dn13 = assign79050_body51_e120347_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79050_body52_e120381, assign79050_body52_e120381_d_n0, assign79050_body52_e120381_d_n2, assign79050_body52_e120381_d_n4, assign79050_body52_e120381_d_n5, assign79050_body52_e120381_d_n6, assign79050_body52_e120381_d_n7, assign79050_body52_e120381_d_n8, assign79050_body52_e120381_d_n9, assign79050_body52_e120381_d_n10, assign79050_body52_e120381_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body52_e120365: f64 = (locals.var_chi / 2.0);
        let assign79050_body52_e120369: f64 = (locals.var_chi / 3.0);
        let assign79050_body52_e120373: f64 = (locals.var_chi / 4.0);
        let assign79050_body52_e120374: f64 = (1.0 + assign79050_body52_e120373);
        let assign79050_body52_e120375: f64 = (assign79050_body52_e120369 * assign79050_body52_e120374);
        let assign79050_body52_e120376: f64 = (1.0 + assign79050_body52_e120375);
        let assign79050_body52_e120377: f64 = (assign79050_body52_e120365 * assign79050_body52_e120376);
        let assign79050_body52_e120378: f64 = (1.0 + assign79050_body52_e120377);
        let assign79050_body52_e120379: f64 = (locals.var_chi * assign79050_body52_e120378);
        (assign79050_body52_e120379, ((locals.var_chi_dn0 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn0 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn2 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn4 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn5 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn6 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn7 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn8 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn9 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn10 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn13 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body52_e120381;
            locals.var_t1_dn0 = assign79050_body52_e120381_d_n0;
            locals.var_t1_dn2 = assign79050_body52_e120381_d_n2;
            locals.var_t1_dn4 = assign79050_body52_e120381_d_n4;
            locals.var_t1_dn5 = assign79050_body52_e120381_d_n5;
            locals.var_t1_dn6 = assign79050_body52_e120381_d_n6;
            locals.var_t1_dn7 = assign79050_body52_e120381_d_n7;
            locals.var_t1_dn8 = assign79050_body52_e120381_d_n8;
            locals.var_t1_dn9 = assign79050_body52_e120381_d_n9;
            locals.var_t1_dn10 = assign79050_body52_e120381_d_n10;
            locals.var_t1_dn13 = assign79050_body52_e120381_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79050_body53_e120399, assign79050_body53_e120399_d_n0, assign79050_body53_e120399_d_n2, assign79050_body53_e120399_d_n4, assign79050_body53_e120399_d_n5, assign79050_body53_e120399_d_n6, assign79050_body53_e120399_d_n7, assign79050_body53_e120399_d_n8, assign79050_body53_e120399_d_n9, assign79050_body53_e120399_d_n10, assign79050_body53_e120399_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body53_e120397: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign79050_body53_e120397, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body53_e120399;
            locals.var_fs01_dn0 = assign79050_body53_e120399_d_n0;
            locals.var_fs01_dn2 = assign79050_body53_e120399_d_n2;
            locals.var_fs01_dn4 = assign79050_body53_e120399_d_n4;
            locals.var_fs01_dn5 = assign79050_body53_e120399_d_n5;
            locals.var_fs01_dn6 = assign79050_body53_e120399_d_n6;
            locals.var_fs01_dn7 = assign79050_body53_e120399_d_n7;
            locals.var_fs01_dn8 = assign79050_body53_e120399_d_n8;
            locals.var_fs01_dn9 = assign79050_body53_e120399_d_n9;
            locals.var_fs01_dn10 = assign79050_body53_e120399_d_n10;
            locals.var_fs01_dn13 = assign79050_body53_e120399_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign79050_body54_e120419, assign79050_body54_e120419_d_n0, assign79050_body54_e120419_d_n2, assign79050_body54_e120419_d_n4, assign79050_body54_e120419_d_n5, assign79050_body54_e120419_d_n6, assign79050_body54_e120419_d_n7, assign79050_body54_e120419_d_n8, assign79050_body54_e120419_d_n9, assign79050_body54_e120419_d_n10, assign79050_body54_e120419_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body54_e120415: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign79050_body54_e120417: f64 = (assign79050_body54_e120415 * locals.var_beta);
        (assign79050_body54_e120417, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body54_e120419;
            locals.var_fs01_dps0_dn0 = assign79050_body54_e120419_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body54_e120419_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body54_e120419_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body54_e120419_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body54_e120419_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body54_e120419_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body54_e120419_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body54_e120419_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body54_e120419_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body54_e120419_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign79050_body55_e120437, assign79050_body55_e120437_d_n0, assign79050_body55_e120437_d_n2, assign79050_body55_e120437_d_n4, assign79050_body55_e120437_d_n5, assign79050_body55_e120437_d_n6, assign79050_body55_e120437_d_n7, assign79050_body55_e120437_d_n8, assign79050_body55_e120437_d_n9, assign79050_body55_e120437_d_n10, assign79050_body55_e120437_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body55_e120435: f64 = (locals.var_chi).exp();
        (assign79050_body55_e120435, (assign79050_body55_e120435 * locals.var_chi_dn0), (assign79050_body55_e120435 * locals.var_chi_dn2), (assign79050_body55_e120435 * locals.var_chi_dn4), (assign79050_body55_e120435 * locals.var_chi_dn5), (assign79050_body55_e120435 * locals.var_chi_dn6), (assign79050_body55_e120435 * locals.var_chi_dn7), (assign79050_body55_e120435 * locals.var_chi_dn8), (assign79050_body55_e120435 * locals.var_chi_dn9), (assign79050_body55_e120435 * locals.var_chi_dn10), (assign79050_body55_e120435 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign79050_body55_e120437;
            locals.var_exp_chi_dn0 = assign79050_body55_e120437_d_n0;
            locals.var_exp_chi_dn2 = assign79050_body55_e120437_d_n2;
            locals.var_exp_chi_dn4 = assign79050_body55_e120437_d_n4;
            locals.var_exp_chi_dn5 = assign79050_body55_e120437_d_n5;
            locals.var_exp_chi_dn6 = assign79050_body55_e120437_d_n6;
            locals.var_exp_chi_dn7 = assign79050_body55_e120437_d_n7;
            locals.var_exp_chi_dn8 = assign79050_body55_e120437_d_n8;
            locals.var_exp_chi_dn9 = assign79050_body55_e120437_d_n9;
            locals.var_exp_chi_dn10 = assign79050_body55_e120437_d_n10;
            locals.var_exp_chi_dn13 = assign79050_body55_e120437_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign79050_body56_e120456, assign79050_body56_e120456_d_n0, assign79050_body56_e120456_d_n2, assign79050_body56_e120456_d_n4, assign79050_body56_e120456_d_n5, assign79050_body56_e120456_d_n6, assign79050_body56_e120456_d_n7, assign79050_body56_e120456_d_n8, assign79050_body56_e120456_d_n9, assign79050_body56_e120456_d_n10, assign79050_body56_e120456_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body56_e120454: f64 = (locals.var_exp_chi - 1.0);
        (assign79050_body56_e120454, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body56_e120456;
            locals.var_t1_dn0 = assign79050_body56_e120456_d_n0;
            locals.var_t1_dn2 = assign79050_body56_e120456_d_n2;
            locals.var_t1_dn4 = assign79050_body56_e120456_d_n4;
            locals.var_t1_dn5 = assign79050_body56_e120456_d_n5;
            locals.var_t1_dn6 = assign79050_body56_e120456_d_n6;
            locals.var_t1_dn7 = assign79050_body56_e120456_d_n7;
            locals.var_t1_dn8 = assign79050_body56_e120456_d_n8;
            locals.var_t1_dn9 = assign79050_body56_e120456_d_n9;
            locals.var_t1_dn10 = assign79050_body56_e120456_d_n10;
            locals.var_t1_dn13 = assign79050_body56_e120456_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79050_body57_e120477, assign79050_body57_e120477_d_n0, assign79050_body57_e120477_d_n2, assign79050_body57_e120477_d_n4, assign79050_body57_e120477_d_n5, assign79050_body57_e120477_d_n6, assign79050_body57_e120477_d_n7, assign79050_body57_e120477_d_n8, assign79050_body57_e120477_d_n9, assign79050_body57_e120477_d_n10, assign79050_body57_e120477_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body57_e120474: f64 = (locals.var_t1 - locals.var_chi);
        let assign79050_body57_e120475: f64 = (locals.var_cfs1 * assign79050_body57_e120474);
        (assign79050_body57_e120475, ((locals.var_cfs1_dn0 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body57_e120477;
            locals.var_fs01_dn0 = assign79050_body57_e120477_d_n0;
            locals.var_fs01_dn2 = assign79050_body57_e120477_d_n2;
            locals.var_fs01_dn4 = assign79050_body57_e120477_d_n4;
            locals.var_fs01_dn5 = assign79050_body57_e120477_d_n5;
            locals.var_fs01_dn6 = assign79050_body57_e120477_d_n6;
            locals.var_fs01_dn7 = assign79050_body57_e120477_d_n7;
            locals.var_fs01_dn8 = assign79050_body57_e120477_d_n8;
            locals.var_fs01_dn9 = assign79050_body57_e120477_d_n9;
            locals.var_fs01_dn10 = assign79050_body57_e120477_d_n10;
            locals.var_fs01_dn13 = assign79050_body57_e120477_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign79050_body58_e120498, assign79050_body58_e120498_d_n0, assign79050_body58_e120498_d_n2, assign79050_body58_e120498_d_n4, assign79050_body58_e120498_d_n5, assign79050_body58_e120498_d_n6, assign79050_body58_e120498_d_n7, assign79050_body58_e120498_d_n8, assign79050_body58_e120498_d_n9, assign79050_body58_e120498_d_n10, assign79050_body58_e120498_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body58_e120494: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign79050_body58_e120496: f64 = (assign79050_body58_e120494 * locals.var_t1);
        (assign79050_body58_e120496, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body58_e120498;
            locals.var_fs01_dps0_dn0 = assign79050_body58_e120498_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body58_e120498_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body58_e120498_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body58_e120498_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body58_e120498_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body58_e120498_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body58_e120498_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body58_e120498_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body58_e120498_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body58_e120498_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign79050_body60_e120533, assign79050_body60_e120533_d_n0, assign79050_body60_e120533_d_n2, assign79050_body60_e120533_d_n4, assign79050_body60_e120533_d_n5, assign79050_body60_e120533_d_n6, assign79050_body60_e120533_d_n7, assign79050_body60_e120533_d_n8, assign79050_body60_e120533_d_n9, assign79050_body60_e120533_d_n10, assign79050_body60_e120533_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79050_body60_e120530: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign79050_body60_e120531: f64 = (assign79050_body60_e120530).exp();
        (assign79050_body60_e120531, (assign79050_body60_e120531 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign79050_body60_e120531 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign79050_body60_e120531 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign79050_body60_e120531 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign79050_body60_e120531 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign79050_body60_e120531 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign79050_body60_e120531 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign79050_body60_e120531 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign79050_body60_e120531 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign79050_body60_e120531 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign79050_body60_e120533;
            locals.var_exp_bps0_dn0 = assign79050_body60_e120533_d_n0;
            locals.var_exp_bps0_dn2 = assign79050_body60_e120533_d_n2;
            locals.var_exp_bps0_dn4 = assign79050_body60_e120533_d_n4;
            locals.var_exp_bps0_dn5 = assign79050_body60_e120533_d_n5;
            locals.var_exp_bps0_dn6 = assign79050_body60_e120533_d_n6;
            locals.var_exp_bps0_dn7 = assign79050_body60_e120533_d_n7;
            locals.var_exp_bps0_dn8 = assign79050_body60_e120533_d_n8;
            locals.var_exp_bps0_dn9 = assign79050_body60_e120533_d_n9;
            locals.var_exp_bps0_dn10 = assign79050_body60_e120533_d_n10;
            locals.var_exp_bps0_dn13 = assign79050_body60_e120533_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign79050_body61_e120556, assign79050_body61_e120556_d_n0, assign79050_body61_e120556_d_n2, assign79050_body61_e120556_d_n4, assign79050_body61_e120556_d_n5, assign79050_body61_e120556_d_n6, assign79050_body61_e120556_d_n7, assign79050_body61_e120556_d_n8, assign79050_body61_e120556_d_n9, assign79050_body61_e120556_d_n10, assign79050_body61_e120556_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79050_body61_e120551: f64 = (locals.var_chi + 1.0);
        let assign79050_body61_e120552: f64 = (locals.var_exp_bvbs * assign79050_body61_e120551);
        let assign79050_body61_e120553: f64 = (locals.var_exp_bps0 - assign79050_body61_e120552);
        let assign79050_body61_e120554: f64 = (locals.var_cnst1over * assign79050_body61_e120553);
        (assign79050_body61_e120554, ((locals.var_cnst1over_dn0 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body61_e120556;
            locals.var_fs01_dn0 = assign79050_body61_e120556_d_n0;
            locals.var_fs01_dn2 = assign79050_body61_e120556_d_n2;
            locals.var_fs01_dn4 = assign79050_body61_e120556_d_n4;
            locals.var_fs01_dn5 = assign79050_body61_e120556_d_n5;
            locals.var_fs01_dn6 = assign79050_body61_e120556_d_n6;
            locals.var_fs01_dn7 = assign79050_body61_e120556_d_n7;
            locals.var_fs01_dn8 = assign79050_body61_e120556_d_n8;
            locals.var_fs01_dn9 = assign79050_body61_e120556_d_n9;
            locals.var_fs01_dn10 = assign79050_body61_e120556_d_n10;
            locals.var_fs01_dn13 = assign79050_body61_e120556_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign79050_body62_e120577, assign79050_body62_e120577_d_n0, assign79050_body62_e120577_d_n2, assign79050_body62_e120577_d_n4, assign79050_body62_e120577_d_n5, assign79050_body62_e120577_d_n6, assign79050_body62_e120577_d_n7, assign79050_body62_e120577_d_n8, assign79050_body62_e120577_d_n9, assign79050_body62_e120577_d_n10, assign79050_body62_e120577_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79050_body62_e120571: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign79050_body62_e120574: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign79050_body62_e120575: f64 = (assign79050_body62_e120571 * assign79050_body62_e120574);
        (assign79050_body62_e120575, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body62_e120577;
            locals.var_fs01_dps0_dn0 = assign79050_body62_e120577_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body62_e120577_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body62_e120577_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body62_e120577_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body62_e120577_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body62_e120577_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body62_e120577_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body62_e120577_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body62_e120577_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body62_e120577_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign79050_body63_e120580: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1838 = assign79050_body63_e120580;
            locals.var_guard1838_rv = 0.0;
            let (assign79050_body64_e120599, assign79050_body64_e120599_d_n0, assign79050_body64_e120599_d_n2, assign79050_body64_e120599_d_n4, assign79050_body64_e120599_d_n5, assign79050_body64_e120599_d_n6, assign79050_body64_e120599_d_n7, assign79050_body64_e120599_d_n8, assign79050_body64_e120599_d_n9, assign79050_body64_e120599_d_n10, assign79050_body64_e120599_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 != 0.0)) {
        let assign79050_body64_e120594: f64 = (locals.var_fb * locals.var_fb);
        let assign79050_body64_e120596: f64 = (assign79050_body64_e120594 + locals.var_fs01);
        let assign79050_body64_e120597: f64 = (assign79050_body64_e120596).sqrt();
        (assign79050_body64_e120597, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign79050_body64_e120597)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79050_body64_e120599;
            locals.var_fs02_dn0 = assign79050_body64_e120599_d_n0;
            locals.var_fs02_dn2 = assign79050_body64_e120599_d_n2;
            locals.var_fs02_dn4 = assign79050_body64_e120599_d_n4;
            locals.var_fs02_dn5 = assign79050_body64_e120599_d_n5;
            locals.var_fs02_dn6 = assign79050_body64_e120599_d_n6;
            locals.var_fs02_dn7 = assign79050_body64_e120599_d_n7;
            locals.var_fs02_dn8 = assign79050_body64_e120599_d_n8;
            locals.var_fs02_dn9 = assign79050_body64_e120599_d_n9;
            locals.var_fs02_dn10 = assign79050_body64_e120599_d_n10;
            locals.var_fs02_dn13 = assign79050_body64_e120599_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign79050_body65_e120623, assign79050_body65_e120623_d_n0, assign79050_body65_e120623_d_n2, assign79050_body65_e120623_d_n4, assign79050_body65_e120623_d_n5, assign79050_body65_e120623_d_n6, assign79050_body65_e120623_d_n7, assign79050_body65_e120623_d_n8, assign79050_body65_e120623_d_n9, assign79050_body65_e120623_d_n10, assign79050_body65_e120623_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 != 0.0)) {
        let assign79050_body65_e120614: f64 = (2.0 * locals.var_fb_dpss);
        let assign79050_body65_e120616: f64 = (assign79050_body65_e120614 * locals.var_fb);
        let assign79050_body65_e120618: f64 = (assign79050_body65_e120616 + locals.var_fs01_dps0);
        let assign79050_body65_e120619: f64 = (0.5 * assign79050_body65_e120618);
        let assign79050_body65_e120621: f64 = (assign79050_body65_e120619 / locals.var_fs02);
        (assign79050_body65_e120621, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79050_body65_e120623;
            locals.var_fs02_dps0_dn0 = assign79050_body65_e120623_d_n0;
            locals.var_fs02_dps0_dn2 = assign79050_body65_e120623_d_n2;
            locals.var_fs02_dps0_dn4 = assign79050_body65_e120623_d_n4;
            locals.var_fs02_dps0_dn5 = assign79050_body65_e120623_d_n5;
            locals.var_fs02_dps0_dn6 = assign79050_body65_e120623_d_n6;
            locals.var_fs02_dps0_dn7 = assign79050_body65_e120623_d_n7;
            locals.var_fs02_dps0_dn8 = assign79050_body65_e120623_d_n8;
            locals.var_fs02_dps0_dn9 = assign79050_body65_e120623_d_n9;
            locals.var_fs02_dps0_dn10 = assign79050_body65_e120623_d_n10;
            locals.var_fs02_dps0_dn13 = assign79050_body65_e120623_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign79050_body67_e120655, assign79050_body67_e120655_d_n0, assign79050_body67_e120655_d_n2, assign79050_body67_e120655_d_n4, assign79050_body67_e120655_d_n5, assign79050_body67_e120655_d_n6, assign79050_body67_e120655_d_n7, assign79050_body67_e120655_d_n8, assign79050_body67_e120655_d_n9, assign79050_body67_e120655_d_n10, assign79050_body67_e120655_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79050_body67_e120655;
            locals.var_fs02_dn0 = assign79050_body67_e120655_d_n0;
            locals.var_fs02_dn2 = assign79050_body67_e120655_d_n2;
            locals.var_fs02_dn4 = assign79050_body67_e120655_d_n4;
            locals.var_fs02_dn5 = assign79050_body67_e120655_d_n5;
            locals.var_fs02_dn6 = assign79050_body67_e120655_d_n6;
            locals.var_fs02_dn7 = assign79050_body67_e120655_d_n7;
            locals.var_fs02_dn8 = assign79050_body67_e120655_d_n8;
            locals.var_fs02_dn9 = assign79050_body67_e120655_d_n9;
            locals.var_fs02_dn10 = assign79050_body67_e120655_d_n10;
            locals.var_fs02_dn13 = assign79050_body67_e120655_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign79050_body68_e120670, assign79050_body68_e120670_d_n0, assign79050_body68_e120670_d_n2, assign79050_body68_e120670_d_n4, assign79050_body68_e120670_d_n5, assign79050_body68_e120670_d_n6, assign79050_body68_e120670_d_n7, assign79050_body68_e120670_d_n8, assign79050_body68_e120670_d_n9, assign79050_body68_e120670_d_n10, assign79050_body68_e120670_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79050_body68_e120670;
            locals.var_fs02_dps0_dn0 = assign79050_body68_e120670_d_n0;
            locals.var_fs02_dps0_dn2 = assign79050_body68_e120670_d_n2;
            locals.var_fs02_dps0_dn4 = assign79050_body68_e120670_d_n4;
            locals.var_fs02_dps0_dn5 = assign79050_body68_e120670_d_n5;
            locals.var_fs02_dps0_dn6 = assign79050_body68_e120670_d_n6;
            locals.var_fs02_dps0_dn7 = assign79050_body68_e120670_d_n7;
            locals.var_fs02_dps0_dn8 = assign79050_body68_e120670_d_n8;
            locals.var_fs02_dps0_dn9 = assign79050_body68_e120670_d_n9;
            locals.var_fs02_dps0_dn10 = assign79050_body68_e120670_d_n10;
            locals.var_fs02_dps0_dn13 = assign79050_body68_e120670_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign79050_body69_e120686, assign79050_body69_e120686_d_n0, assign79050_body69_e120686_d_n2, assign79050_body69_e120686_d_n4, assign79050_body69_e120686_d_n5, assign79050_body69_e120686_d_n6, assign79050_body69_e120686_d_n7, assign79050_body69_e120686_d_n8, assign79050_body69_e120686_d_n9, assign79050_body69_e120686_d_n10, assign79050_body69_e120686_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body69_e120678: f64 = (-locals.var_vgpld);
        let assign79050_body69_e120680: f64 = (assign79050_body69_e120678 + locals.var_ps0ld);
        let assign79050_body69_e120683: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign79050_body69_e120684: f64 = (assign79050_body69_e120680 + assign79050_body69_e120683);
        (assign79050_body69_e120684, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign79050_body69_e120686;
            locals.var_fs0_dn0 = assign79050_body69_e120686_d_n0;
            locals.var_fs0_dn2 = assign79050_body69_e120686_d_n2;
            locals.var_fs0_dn4 = assign79050_body69_e120686_d_n4;
            locals.var_fs0_dn5 = assign79050_body69_e120686_d_n5;
            locals.var_fs0_dn6 = assign79050_body69_e120686_d_n6;
            locals.var_fs0_dn7 = assign79050_body69_e120686_d_n7;
            locals.var_fs0_dn8 = assign79050_body69_e120686_d_n8;
            locals.var_fs0_dn9 = assign79050_body69_e120686_d_n9;
            locals.var_fs0_dn10 = assign79050_body69_e120686_d_n10;
            locals.var_fs0_dn13 = assign79050_body69_e120686_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign79050_body70_e120699, assign79050_body70_e120699_d_n0, assign79050_body70_e120699_d_n2, assign79050_body70_e120699_d_n4, assign79050_body70_e120699_d_n5, assign79050_body70_e120699_d_n6, assign79050_body70_e120699_d_n7, assign79050_body70_e120699_d_n8, assign79050_body70_e120699_d_n9, assign79050_body70_e120699_d_n10, assign79050_body70_e120699_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body70_e120696: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign79050_body70_e120697: f64 = (1.0 + assign79050_body70_e120696);
        (assign79050_body70_e120697, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign79050_body70_e120699;
            locals.var_fs0_dps0_dn0 = assign79050_body70_e120699_d_n0;
            locals.var_fs0_dps0_dn2 = assign79050_body70_e120699_d_n2;
            locals.var_fs0_dps0_dn4 = assign79050_body70_e120699_d_n4;
            locals.var_fs0_dps0_dn5 = assign79050_body70_e120699_d_n5;
            locals.var_fs0_dps0_dn6 = assign79050_body70_e120699_d_n6;
            locals.var_fs0_dps0_dn7 = assign79050_body70_e120699_d_n7;
            locals.var_fs0_dps0_dn8 = assign79050_body70_e120699_d_n8;
            locals.var_fs0_dps0_dn9 = assign79050_body70_e120699_d_n9;
            locals.var_fs0_dps0_dn10 = assign79050_body70_e120699_d_n10;
            locals.var_fs0_dps0_dn13 = assign79050_body70_e120699_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign79050_body71_e120702: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1839 = assign79050_body71_e120702;
            locals.var_guard1839_rv = 0.0;
            let (assign79050_body72_e120715,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 != 0.0)) {
        let assign79050_body72_e120713: f64 = (locals.var_lp_s0_max + 1.0);
        (assign79050_body72_e120713,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79050_body72_e120715;
            locals.var_lp_s0_rv = 0.0;
            let (assign79050_body73_e120730, assign79050_body73_e120730_d_n0, assign79050_body73_e120730_d_n2, assign79050_body73_e120730_d_n4, assign79050_body73_e120730_d_n5, assign79050_body73_e120730_d_n6, assign79050_body73_e120730_d_n7, assign79050_body73_e120730_d_n8, assign79050_body73_e120730_d_n9, assign79050_body73_e120730_d_n10, assign79050_body73_e120730_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) {
        let assign79050_body73_e120726: f64 = (-locals.var_fs0);
        let assign79050_body73_e120728: f64 = (assign79050_body73_e120726 / locals.var_fs0_dps0);
        (assign79050_body73_e120728, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79050_body73_e120730;
            locals.var_dps0_dn0 = assign79050_body73_e120730_d_n0;
            locals.var_dps0_dn2 = assign79050_body73_e120730_d_n2;
            locals.var_dps0_dn4 = assign79050_body73_e120730_d_n4;
            locals.var_dps0_dn5 = assign79050_body73_e120730_d_n5;
            locals.var_dps0_dn6 = assign79050_body73_e120730_d_n6;
            locals.var_dps0_dn7 = assign79050_body73_e120730_d_n7;
            locals.var_dps0_dn8 = assign79050_body73_e120730_d_n8;
            locals.var_dps0_dn9 = assign79050_body73_e120730_d_n9;
            locals.var_dps0_dn10 = assign79050_body73_e120730_d_n10;
            locals.var_dps0_dn13 = assign79050_body73_e120730_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign79050_body74_e120755, assign79050_body74_e120755_d_n0, assign79050_body74_e120755_d_n2, assign79050_body74_e120755_d_n4, assign79050_body74_e120755_d_n5, assign79050_body74_e120755_d_n6, assign79050_body74_e120755_d_n7, assign79050_body74_e120755_d_n8, assign79050_body74_e120755_d_n9, assign79050_body74_e120755_d_n10, assign79050_body74_e120755_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) {
        let assign79050_body74_e120742: f64 = (0.5 * 0.1);
        let assign79050_body74_e120746: f64 = (locals.var_ps0ld).abs();
        let (assign79050_body74_e120751, assign79050_body74_e120751_d_n0, assign79050_body74_e120751_d_n2, assign79050_body74_e120751_d_n4, assign79050_body74_e120751_d_n5, assign79050_body74_e120751_d_n6, assign79050_body74_e120751_d_n7, assign79050_body74_e120751_d_n8, assign79050_body74_e120751_d_n9, assign79050_body74_e120751_d_n10, assign79050_body74_e120751_d_n13,) = {
            if (1.0 >= assign79050_body74_e120746) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign79050_body74_e120750: f64 = (locals.var_ps0ld).abs();
                (assign79050_body74_e120750, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign79050_body74_e120752: f64 = (1.0 + assign79050_body74_e120751);
        let assign79050_body74_e120753: f64 = (assign79050_body74_e120742 * assign79050_body74_e120752);
        (assign79050_body74_e120753, (assign79050_body74_e120742 * assign79050_body74_e120751_d_n0), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n2), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n4), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n5), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n6), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n7), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n8), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n9), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n10), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign79050_body74_e120755;
            locals.var_dplim_dn0 = assign79050_body74_e120755_d_n0;
            locals.var_dplim_dn2 = assign79050_body74_e120755_d_n2;
            locals.var_dplim_dn4 = assign79050_body74_e120755_d_n4;
            locals.var_dplim_dn5 = assign79050_body74_e120755_d_n5;
            locals.var_dplim_dn6 = assign79050_body74_e120755_d_n6;
            locals.var_dplim_dn7 = assign79050_body74_e120755_d_n7;
            locals.var_dplim_dn8 = assign79050_body74_e120755_d_n8;
            locals.var_dplim_dn9 = assign79050_body74_e120755_d_n9;
            locals.var_dplim_dn10 = assign79050_body74_e120755_d_n10;
            locals.var_dplim_dn13 = assign79050_body74_e120755_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign79050_body75_e120757: f64 = (locals.var_dps0).abs();
            let assign79050_body75_e120759: f64 = if assign79050_body75_e120757 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1840 = assign79050_body75_e120759;
            locals.var_guard1840_rv = 0.0;
            let (assign79050_body76_e120781, assign79050_body76_e120781_d_n0, assign79050_body76_e120781_d_n2, assign79050_body76_e120781_d_n4, assign79050_body76_e120781_d_n5, assign79050_body76_e120781_d_n6, assign79050_body76_e120781_d_n7, assign79050_body76_e120781_d_n8, assign79050_body76_e120781_d_n9, assign79050_body76_e120781_d_n10, assign79050_body76_e120781_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) {
        let (assign79050_body76_e120778,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign79050_body76_e120777: f64 = (-1.0);
                (assign79050_body76_e120777,)
            }
        };
        let assign79050_body76_e120779: f64 = (locals.var_dplim * assign79050_body76_e120778);
        (assign79050_body76_e120779, (locals.var_dplim_dn0 * assign79050_body76_e120778), (locals.var_dplim_dn2 * assign79050_body76_e120778), (locals.var_dplim_dn4 * assign79050_body76_e120778), (locals.var_dplim_dn5 * assign79050_body76_e120778), (locals.var_dplim_dn6 * assign79050_body76_e120778), (locals.var_dplim_dn7 * assign79050_body76_e120778), (locals.var_dplim_dn8 * assign79050_body76_e120778), (locals.var_dplim_dn9 * assign79050_body76_e120778), (locals.var_dplim_dn10 * assign79050_body76_e120778), (locals.var_dplim_dn13 * assign79050_body76_e120778),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79050_body76_e120781;
            locals.var_dps0_dn0 = assign79050_body76_e120781_d_n0;
            locals.var_dps0_dn2 = assign79050_body76_e120781_d_n2;
            locals.var_dps0_dn4 = assign79050_body76_e120781_d_n4;
            locals.var_dps0_dn5 = assign79050_body76_e120781_d_n5;
            locals.var_dps0_dn6 = assign79050_body76_e120781_d_n6;
            locals.var_dps0_dn7 = assign79050_body76_e120781_d_n7;
            locals.var_dps0_dn8 = assign79050_body76_e120781_d_n8;
            locals.var_dps0_dn9 = assign79050_body76_e120781_d_n9;
            locals.var_dps0_dn10 = assign79050_body76_e120781_d_n10;
            locals.var_dps0_dn13 = assign79050_body76_e120781_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign79050_body77_e120795, assign79050_body77_e120795_d_n0, assign79050_body77_e120795_d_n2, assign79050_body77_e120795_d_n4, assign79050_body77_e120795_d_n5, assign79050_body77_e120795_d_n6, assign79050_body77_e120795_d_n7, assign79050_body77_e120795_d_n8, assign79050_body77_e120795_d_n9, assign79050_body77_e120795_d_n10, assign79050_body77_e120795_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) {
        let assign79050_body77_e120793: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign79050_body77_e120793, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign79050_body77_e120795;
            locals.var_ps0ld_dn0 = assign79050_body77_e120795_d_n0;
            locals.var_ps0ld_dn2 = assign79050_body77_e120795_d_n2;
            locals.var_ps0ld_dn4 = assign79050_body77_e120795_d_n4;
            locals.var_ps0ld_dn5 = assign79050_body77_e120795_d_n5;
            locals.var_ps0ld_dn6 = assign79050_body77_e120795_d_n6;
            locals.var_ps0ld_dn7 = assign79050_body77_e120795_d_n7;
            locals.var_ps0ld_dn8 = assign79050_body77_e120795_d_n8;
            locals.var_ps0ld_dn9 = assign79050_body77_e120795_d_n9;
            locals.var_ps0ld_dn10 = assign79050_body77_e120795_d_n10;
            locals.var_ps0ld_dn13 = assign79050_body77_e120795_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign79050_body78_e120797: f64 = (locals.var_dps0).abs();
            let assign79050_body78_e120801: f64 = (locals.var_fs0).abs();
            let assign79050_body78_e120804: f64 = if ((assign79050_body78_e120797 <= 1e-12) && (assign79050_body78_e120801 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1841 = assign79050_body78_e120804;
            locals.var_guard1841_rv = 0.0;
            let (assign79050_body79_e120818,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1841 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign79050_body79_e120818;
            locals.var_flg_conv_rv = 0.0;
            let (assign79050_body80_e120829,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body80_e120827: f64 = (locals.var_lp_s0 + 1.0);
        (assign79050_body80_e120827,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79050_body80_e120829;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_289(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79070_e120843, assign79070_e120843_d_n0, assign79070_e120843_d_n2, assign79070_e120843_d_n4, assign79070_e120843_d_n5, assign79070_e120843_d_n6, assign79070_e120843_d_n7, assign79070_e120843_d_n8, assign79070_e120843_d_n9, assign79070_e120843_d_n10, assign79070_e120843_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79070_e120841: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign79070_e120841, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1760, locals.var_wdld__blk1760_dn0, locals.var_wdld__blk1760_dn2, locals.var_wdld__blk1760_dn4, locals.var_wdld__blk1760_dn5, locals.var_wdld__blk1760_dn6, locals.var_wdld__blk1760_dn7, locals.var_wdld__blk1760_dn8, locals.var_wdld__blk1760_dn9, locals.var_wdld__blk1760_dn10, locals.var_wdld__blk1760_dn13,)
    }
};
        locals.var_wdld__blk1760 = assign79070_e120843;
        locals.var_wdld__blk1760_dn0 = assign79070_e120843_d_n0;
        locals.var_wdld__blk1760_dn2 = assign79070_e120843_d_n2;
        locals.var_wdld__blk1760_dn4 = assign79070_e120843_d_n4;
        locals.var_wdld__blk1760_dn5 = assign79070_e120843_d_n5;
        locals.var_wdld__blk1760_dn6 = assign79070_e120843_d_n6;
        locals.var_wdld__blk1760_dn7 = assign79070_e120843_d_n7;
        locals.var_wdld__blk1760_dn8 = assign79070_e120843_d_n8;
        locals.var_wdld__blk1760_dn9 = assign79070_e120843_d_n9;
        locals.var_wdld__blk1760_dn10 = assign79070_e120843_d_n10;
        locals.var_wdld__blk1760_dn13 = assign79070_e120843_d_n13;
        locals.var_wdld__blk1760_rv = 0.0;

        let (assign79080_e120854, assign79080_e120854_d_n0, assign79080_e120854_d_n2, assign79080_e120854_d_n4, assign79080_e120854_d_n5, assign79080_e120854_d_n6, assign79080_e120854_d_n7, assign79080_e120854_d_n8, assign79080_e120854_d_n9, assign79080_e120854_d_n10, assign79080_e120854_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79080_e120852: f64 = (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760);
        (assign79080_e120852, (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn0), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn2), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn4), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn5), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn6), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn7), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn8), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn9), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn10), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1761, locals.var_q_dep_ld__blk1761_dn0, locals.var_q_dep_ld__blk1761_dn2, locals.var_q_dep_ld__blk1761_dn4, locals.var_q_dep_ld__blk1761_dn5, locals.var_q_dep_ld__blk1761_dn6, locals.var_q_dep_ld__blk1761_dn7, locals.var_q_dep_ld__blk1761_dn8, locals.var_q_dep_ld__blk1761_dn9, locals.var_q_dep_ld__blk1761_dn10, locals.var_q_dep_ld__blk1761_dn13,)
    }
};
        locals.var_q_dep_ld__blk1761 = assign79080_e120854;
        locals.var_q_dep_ld__blk1761_dn0 = assign79080_e120854_d_n0;
        locals.var_q_dep_ld__blk1761_dn2 = assign79080_e120854_d_n2;
        locals.var_q_dep_ld__blk1761_dn4 = assign79080_e120854_d_n4;
        locals.var_q_dep_ld__blk1761_dn5 = assign79080_e120854_d_n5;
        locals.var_q_dep_ld__blk1761_dn6 = assign79080_e120854_d_n6;
        locals.var_q_dep_ld__blk1761_dn7 = assign79080_e120854_d_n7;
        locals.var_q_dep_ld__blk1761_dn8 = assign79080_e120854_d_n8;
        locals.var_q_dep_ld__blk1761_dn9 = assign79080_e120854_d_n9;
        locals.var_q_dep_ld__blk1761_dn10 = assign79080_e120854_d_n10;
        locals.var_q_dep_ld__blk1761_dn13 = assign79080_e120854_d_n13;
        locals.var_q_dep_ld__blk1761_rv = 0.0;

        let (assign79090_e120869, assign79090_e120869_d_n0, assign79090_e120869_d_n2, assign79090_e120869_d_n4, assign79090_e120869_d_n5, assign79090_e120869_d_n6, assign79090_e120869_d_n7, assign79090_e120869_d_n8, assign79090_e120869_d_n9, assign79090_e120869_d_n10, assign79090_e120869_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79090_e120863: f64 = (locals.var_q_dep_ld__blk1761 / locals.var_cnst0over_func);
        let assign79090_e120866: f64 = (10.0 * 2.220446049250313e-16);
        let assign79090_e120867: f64 = (assign79090_e120863 + assign79090_e120866);
        (assign79090_e120867, (((locals.var_q_dep_ld__blk1761_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign79090_e120869;
        locals.var_xi0p12_dn0 = assign79090_e120869_d_n0;
        locals.var_xi0p12_dn2 = assign79090_e120869_d_n2;
        locals.var_xi0p12_dn4 = assign79090_e120869_d_n4;
        locals.var_xi0p12_dn5 = assign79090_e120869_d_n5;
        locals.var_xi0p12_dn6 = assign79090_e120869_d_n6;
        locals.var_xi0p12_dn7 = assign79090_e120869_d_n7;
        locals.var_xi0p12_dn8 = assign79090_e120869_d_n8;
        locals.var_xi0p12_dn9 = assign79090_e120869_d_n9;
        locals.var_xi0p12_dn10 = assign79090_e120869_d_n10;
        locals.var_xi0p12_dn13 = assign79090_e120869_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign79100_e120880, assign79100_e120880_d_n0, assign79100_e120880_d_n2, assign79100_e120880_d_n4, assign79100_e120880_d_n5, assign79100_e120880_d_n6, assign79100_e120880_d_n7, assign79100_e120880_d_n8, assign79100_e120880_d_n9, assign79100_e120880_d_n10, assign79100_e120880_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79100_e120878: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign79100_e120878, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign79100_e120880;
        locals.var_qbuld_dn0 = assign79100_e120880_d_n0;
        locals.var_qbuld_dn2 = assign79100_e120880_d_n2;
        locals.var_qbuld_dn4 = assign79100_e120880_d_n4;
        locals.var_qbuld_dn5 = assign79100_e120880_d_n5;
        locals.var_qbuld_dn6 = assign79100_e120880_d_n6;
        locals.var_qbuld_dn7 = assign79100_e120880_d_n7;
        locals.var_qbuld_dn8 = assign79100_e120880_d_n8;
        locals.var_qbuld_dn9 = assign79100_e120880_d_n9;
        locals.var_qbuld_dn10 = assign79100_e120880_d_n10;
        locals.var_qbuld_dn13 = assign79100_e120880_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign79110_e120893, assign79110_e120893_d_n0, assign79110_e120893_d_n2, assign79110_e120893_d_n4, assign79110_e120893_d_n5, assign79110_e120893_d_n6, assign79110_e120893_d_n7, assign79110_e120893_d_n8, assign79110_e120893_d_n9, assign79110_e120893_d_n10, assign79110_e120893_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79110_e120890: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign79110_e120891: f64 = (1.0 / assign79110_e120890);
        (assign79110_e120891, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign79110_e120890 * assign79110_e120890))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79110_e120893;
        locals.var_t1_dn0 = assign79110_e120893_d_n0;
        locals.var_t1_dn2 = assign79110_e120893_d_n2;
        locals.var_t1_dn4 = assign79110_e120893_d_n4;
        locals.var_t1_dn5 = assign79110_e120893_d_n5;
        locals.var_t1_dn6 = assign79110_e120893_d_n6;
        locals.var_t1_dn7 = assign79110_e120893_d_n7;
        locals.var_t1_dn8 = assign79110_e120893_d_n8;
        locals.var_t1_dn9 = assign79110_e120893_d_n9;
        locals.var_t1_dn10 = assign79110_e120893_d_n10;
        locals.var_t1_dn13 = assign79110_e120893_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign79120_e120906, assign79120_e120906_d_n0, assign79120_e120906_d_n2, assign79120_e120906_d_n4, assign79120_e120906_d_n5, assign79120_e120906_d_n6, assign79120_e120906_d_n7, assign79120_e120906_d_n8, assign79120_e120906_d_n9, assign79120_e120906_d_n10, assign79120_e120906_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79120_e120902: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign79120_e120904: f64 = (assign79120_e120902 * locals.var_t1);
        (assign79120_e120904, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign79120_e120906;
        locals.var_qiuld_dn0 = assign79120_e120906_d_n0;
        locals.var_qiuld_dn2 = assign79120_e120906_d_n2;
        locals.var_qiuld_dn4 = assign79120_e120906_d_n4;
        locals.var_qiuld_dn5 = assign79120_e120906_d_n5;
        locals.var_qiuld_dn6 = assign79120_e120906_d_n6;
        locals.var_qiuld_dn7 = assign79120_e120906_d_n7;
        locals.var_qiuld_dn8 = assign79120_e120906_d_n8;
        locals.var_qiuld_dn9 = assign79120_e120906_d_n9;
        locals.var_qiuld_dn10 = assign79120_e120906_d_n10;
        locals.var_qiuld_dn13 = assign79120_e120906_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign79130_e120917, assign79130_e120917_d_n0, assign79130_e120917_d_n2, assign79130_e120917_d_n4, assign79130_e120917_d_n5, assign79130_e120917_d_n6, assign79130_e120917_d_n7, assign79130_e120917_d_n8, assign79130_e120917_d_n9, assign79130_e120917_d_n10, assign79130_e120917_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79130_e120915: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign79130_e120915, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign79130_e120917;
        locals.var_qsuld_dn0 = assign79130_e120917_d_n0;
        locals.var_qsuld_dn2 = assign79130_e120917_d_n2;
        locals.var_qsuld_dn4 = assign79130_e120917_d_n4;
        locals.var_qsuld_dn5 = assign79130_e120917_d_n5;
        locals.var_qsuld_dn6 = assign79130_e120917_d_n6;
        locals.var_qsuld_dn7 = assign79130_e120917_d_n7;
        locals.var_qsuld_dn8 = assign79130_e120917_d_n8;
        locals.var_qsuld_dn9 = assign79130_e120917_d_n9;
        locals.var_qsuld_dn10 = assign79130_e120917_d_n10;
        locals.var_qsuld_dn13 = assign79130_e120917_d_n13;
        locals.var_qsuld_rv = 0.0;

        let assign79140_e120920: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1843 = assign79140_e120920;
        locals.var_guard1843_rv = 0.0;

        let (assign79150_e120930, assign79150_e120930_d_n0, assign79150_e120930_d_n2, assign79150_e120930_d_n4, assign79150_e120930_d_n5, assign79150_e120930_d_n6, assign79150_e120930_d_n7, assign79150_e120930_d_n8, assign79150_e120930_d_n9, assign79150_e120930_d_n10, assign79150_e120930_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79150_e120926: f64 = (-locals.var_vxbgmtcl);
        let assign79150_e120927: f64 = (locals.var_beta * assign79150_e120926);
        let assign79150_e120928: f64 = (assign79150_e120927).exp();
        (assign79150_e120928, (assign79150_e120928 * ((locals.var_beta_dn0 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign79150_e120928 * ((locals.var_beta_dn2 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign79150_e120928 * ((locals.var_beta_dn4 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign79150_e120928 * ((locals.var_beta_dn5 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign79150_e120928 * ((locals.var_beta_dn6 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign79150_e120928 * ((locals.var_beta_dn7 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign79150_e120928 * ((locals.var_beta_dn8 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign79150_e120928 * ((locals.var_beta_dn9 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign79150_e120928 * ((locals.var_beta_dn10 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign79150_e120928 * ((locals.var_beta_dn13 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign79150_e120930;
        locals.var_exp_bvbs_dn0 = assign79150_e120930_d_n0;
        locals.var_exp_bvbs_dn2 = assign79150_e120930_d_n2;
        locals.var_exp_bvbs_dn4 = assign79150_e120930_d_n4;
        locals.var_exp_bvbs_dn5 = assign79150_e120930_d_n5;
        locals.var_exp_bvbs_dn6 = assign79150_e120930_d_n6;
        locals.var_exp_bvbs_dn7 = assign79150_e120930_d_n7;
        locals.var_exp_bvbs_dn8 = assign79150_e120930_d_n8;
        locals.var_exp_bvbs_dn9 = assign79150_e120930_d_n9;
        locals.var_exp_bvbs_dn10 = assign79150_e120930_d_n10;
        locals.var_exp_bvbs_dn13 = assign79150_e120930_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign79160_e120938, assign79160_e120938_d_n0, assign79160_e120938_d_n2, assign79160_e120938_d_n4, assign79160_e120938_d_n5, assign79160_e120938_d_n6, assign79160_e120938_d_n7, assign79160_e120938_d_n8, assign79160_e120938_d_n9, assign79160_e120938_d_n10, assign79160_e120938_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79160_e120936: f64 = (locals.var_nin / locals.var_nover_func);
        (assign79160_e120936, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign79160_e120938;
        locals.var_t0_dn0 = assign79160_e120938_d_n0;
        locals.var_t0_dn2 = assign79160_e120938_d_n2;
        locals.var_t0_dn4 = assign79160_e120938_d_n4;
        locals.var_t0_dn5 = assign79160_e120938_d_n5;
        locals.var_t0_dn6 = assign79160_e120938_d_n6;
        locals.var_t0_dn7 = assign79160_e120938_d_n7;
        locals.var_t0_dn8 = assign79160_e120938_d_n8;
        locals.var_t0_dn9 = assign79160_e120938_d_n9;
        locals.var_t0_dn10 = assign79160_e120938_d_n10;
        locals.var_t0_dn13 = assign79160_e120938_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign79170_e120946, assign79170_e120946_d_n0, assign79170_e120946_d_n2, assign79170_e120946_d_n4, assign79170_e120946_d_n5, assign79170_e120946_d_n6, assign79170_e120946_d_n7, assign79170_e120946_d_n8, assign79170_e120946_d_n9, assign79170_e120946_d_n10, assign79170_e120946_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79170_e120944: f64 = (locals.var_t0 * locals.var_t0);
        (assign79170_e120944, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign79170_e120946;
        locals.var_cnst1over_dn0 = assign79170_e120946_d_n0;
        locals.var_cnst1over_dn2 = assign79170_e120946_d_n2;
        locals.var_cnst1over_dn4 = assign79170_e120946_d_n4;
        locals.var_cnst1over_dn5 = assign79170_e120946_d_n5;
        locals.var_cnst1over_dn6 = assign79170_e120946_d_n6;
        locals.var_cnst1over_dn7 = assign79170_e120946_d_n7;
        locals.var_cnst1over_dn8 = assign79170_e120946_d_n8;
        locals.var_cnst1over_dn9 = assign79170_e120946_d_n9;
        locals.var_cnst1over_dn10 = assign79170_e120946_d_n10;
        locals.var_cnst1over_dn13 = assign79170_e120946_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let (assign79180_e120954, assign79180_e120954_d_n0, assign79180_e120954_d_n2, assign79180_e120954_d_n4, assign79180_e120954_d_n5, assign79180_e120954_d_n6, assign79180_e120954_d_n7, assign79180_e120954_d_n8, assign79180_e120954_d_n9, assign79180_e120954_d_n10, assign79180_e120954_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79180_e120952: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign79180_e120952, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign79180_e120954;
        locals.var_cfs1_dn0 = assign79180_e120954_d_n0;
        locals.var_cfs1_dn2 = assign79180_e120954_d_n2;
        locals.var_cfs1_dn4 = assign79180_e120954_d_n4;
        locals.var_cfs1_dn5 = assign79180_e120954_d_n5;
        locals.var_cfs1_dn6 = assign79180_e120954_d_n6;
        locals.var_cfs1_dn7 = assign79180_e120954_d_n7;
        locals.var_cfs1_dn8 = assign79180_e120954_d_n8;
        locals.var_cfs1_dn9 = assign79180_e120954_d_n9;
        locals.var_cfs1_dn10 = assign79180_e120954_d_n10;
        locals.var_cfs1_dn13 = assign79180_e120954_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign79190_e120960, assign79190_e120960_d_n0, assign79190_e120960_d_n2, assign79190_e120960_d_n4, assign79190_e120960_d_n5, assign79190_e120960_d_n6, assign79190_e120960_d_n7, assign79190_e120960_d_n8, assign79190_e120960_d_n9, assign79190_e120960_d_n10, assign79190_e120960_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (locals.var_ps0ld_ini__blk1769, locals.var_ps0ld_ini__blk1769_dn0, locals.var_ps0ld_ini__blk1769_dn2, locals.var_ps0ld_ini__blk1769_dn4, locals.var_ps0ld_ini__blk1769_dn5, locals.var_ps0ld_ini__blk1769_dn6, locals.var_ps0ld_ini__blk1769_dn7, locals.var_ps0ld_ini__blk1769_dn8, locals.var_ps0ld_ini__blk1769_dn9, locals.var_ps0ld_ini__blk1769_dn10, locals.var_ps0ld_ini__blk1769_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign79190_e120960;
        locals.var_ps0ld_dn0 = assign79190_e120960_d_n0;
        locals.var_ps0ld_dn2 = assign79190_e120960_d_n2;
        locals.var_ps0ld_dn4 = assign79190_e120960_d_n4;
        locals.var_ps0ld_dn5 = assign79190_e120960_d_n5;
        locals.var_ps0ld_dn6 = assign79190_e120960_d_n6;
        locals.var_ps0ld_dn7 = assign79190_e120960_d_n7;
        locals.var_ps0ld_dn8 = assign79190_e120960_d_n8;
        locals.var_ps0ld_dn9 = assign79190_e120960_d_n9;
        locals.var_ps0ld_dn10 = assign79190_e120960_d_n10;
        locals.var_ps0ld_dn13 = assign79190_e120960_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign79200_e120966,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign79200_e120966;
        locals.var_flg_conv_rv = 0.0;

        let (assign79210_e120979, assign79210_e120979_d_n0, assign79210_e120979_d_n2, assign79210_e120979_d_n4, assign79210_e120979_d_n5, assign79210_e120979_d_n6, assign79210_e120979_d_n7, assign79210_e120979_d_n8, assign79210_e120979_d_n9, assign79210_e120979_d_n10, assign79210_e120979_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79210_e120973: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1762);
        let assign79210_e120975: f64 = (assign79210_e120973 * locals.var_beta_inv);
        let assign79210_e120976: f64 = (2.0 * assign79210_e120975);
        let assign79210_e120977: f64 = (assign79210_e120976).sqrt();
        (assign79210_e120977, ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn0)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn2)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn4)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn5)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn6)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn7)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn8)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn9)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn10)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn13)) / (2.0 * assign79210_e120977)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign79210_e120979;
        locals.var_c_w_ld_dn0 = assign79210_e120979_d_n0;
        locals.var_c_w_ld_dn2 = assign79210_e120979_d_n2;
        locals.var_c_w_ld_dn4 = assign79210_e120979_d_n4;
        locals.var_c_w_ld_dn5 = assign79210_e120979_d_n5;
        locals.var_c_w_ld_dn6 = assign79210_e120979_d_n6;
        locals.var_c_w_ld_dn7 = assign79210_e120979_d_n7;
        locals.var_c_w_ld_dn8 = assign79210_e120979_d_n8;
        locals.var_c_w_ld_dn9 = assign79210_e120979_d_n9;
        locals.var_c_w_ld_dn10 = assign79210_e120979_d_n10;
        locals.var_c_w_ld_dn13 = assign79210_e120979_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign79220_e120982: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1844 = assign79220_e120982;
        locals.var_guard1844_rv = 0.0;

        let (assign79230_e120992, assign79230_e120992_d_n0, assign79230_e120992_d_n2, assign79230_e120992_d_n4, assign79230_e120992_d_n5, assign79230_e120992_d_n6, assign79230_e120992_d_n7, assign79230_e120992_d_n8, assign79230_e120992_d_n9, assign79230_e120992_d_n10, assign79230_e120992_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 != 0.0)) {
        let assign79230_e120990: f64 = (p.p334 - locals.var_wdep_func);
        (assign79230_e120990, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79230_e120992;
        locals.var_t2_dn0 = assign79230_e120992_d_n0;
        locals.var_t2_dn2 = assign79230_e120992_d_n2;
        locals.var_t2_dn4 = assign79230_e120992_d_n4;
        locals.var_t2_dn5 = assign79230_e120992_d_n5;
        locals.var_t2_dn6 = assign79230_e120992_d_n6;
        locals.var_t2_dn7 = assign79230_e120992_d_n7;
        locals.var_t2_dn8 = assign79230_e120992_d_n8;
        locals.var_t2_dn9 = assign79230_e120992_d_n9;
        locals.var_t2_dn10 = assign79230_e120992_d_n10;
        locals.var_t2_dn13 = assign79230_e120992_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign79240_e121014, assign79240_e121014_d_n0, assign79240_e121014_d_n2, assign79240_e121014_d_n4, assign79240_e121014_d_n5, assign79240_e121014_d_n6, assign79240_e121014_d_n7, assign79240_e121014_d_n8, assign79240_e121014_d_n9, assign79240_e121014_d_n10, assign79240_e121014_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79240_e121001: f64 = (locals.var_vdsi + p.p137);
        let assign79240_e121004: f64 = (locals.var_vdsi + p.p137);
        let assign79240_e121005: f64 = (assign79240_e121001 * assign79240_e121004);
        let assign79240_e121008: f64 = (4.0 * 0.1);
        let assign79240_e121010: f64 = (assign79240_e121008 * 0.1);
        let assign79240_e121011: f64 = (assign79240_e121005 + assign79240_e121010);
        let assign79240_e121012: f64 = (assign79240_e121011).sqrt();
        (assign79240_e121012, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign79240_e121004) + (assign79240_e121001 * locals.var_vdsi_dn5)) / (2.0 * assign79240_e121012)), 0.0, (((locals.var_vdsi_dn7 * assign79240_e121004) + (assign79240_e121001 * locals.var_vdsi_dn7)) / (2.0 * assign79240_e121012)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79240_e121014;
        locals.var_tmf2_dn0 = assign79240_e121014_d_n0;
        locals.var_tmf2_dn2 = assign79240_e121014_d_n2;
        locals.var_tmf2_dn4 = assign79240_e121014_d_n4;
        locals.var_tmf2_dn5 = assign79240_e121014_d_n5;
        locals.var_tmf2_dn6 = assign79240_e121014_d_n6;
        locals.var_tmf2_dn7 = assign79240_e121014_d_n7;
        locals.var_tmf2_dn8 = assign79240_e121014_d_n8;
        locals.var_tmf2_dn9 = assign79240_e121014_d_n9;
        locals.var_tmf2_dn10 = assign79240_e121014_d_n10;
        locals.var_tmf2_dn13 = assign79240_e121014_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign79250_e121031, assign79250_e121031_d_n0, assign79250_e121031_d_n2, assign79250_e121031_d_n4, assign79250_e121031_d_n5, assign79250_e121031_d_n6, assign79250_e121031_d_n7, assign79250_e121031_d_n8, assign79250_e121031_d_n9, assign79250_e121031_d_n10, assign79250_e121031_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79250_e121025: f64 = (locals.var_vdsi + p.p137);
        let assign79250_e121027: f64 = (assign79250_e121025 / locals.var_tmf2);
        let assign79250_e121028: f64 = (1.0 + assign79250_e121027);
        let assign79250_e121029: f64 = (0.5 * assign79250_e121028);
        (assign79250_e121029, (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign79250_e121025 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign79250_e121025 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79250_e121031;
        locals.var_t9_dn0 = assign79250_e121031_d_n0;
        locals.var_t9_dn2 = assign79250_e121031_d_n2;
        locals.var_t9_dn4 = assign79250_e121031_d_n4;
        locals.var_t9_dn5 = assign79250_e121031_d_n5;
        locals.var_t9_dn6 = assign79250_e121031_d_n6;
        locals.var_t9_dn7 = assign79250_e121031_d_n7;
        locals.var_t9_dn8 = assign79250_e121031_d_n8;
        locals.var_t9_dn9 = assign79250_e121031_d_n9;
        locals.var_t9_dn10 = assign79250_e121031_d_n10;
        locals.var_t9_dn13 = assign79250_e121031_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign79260_e121046, assign79260_e121046_d_n0, assign79260_e121046_d_n2, assign79260_e121046_d_n4, assign79260_e121046_d_n5, assign79260_e121046_d_n6, assign79260_e121046_d_n7, assign79260_e121046_d_n8, assign79260_e121046_d_n9, assign79260_e121046_d_n10, assign79260_e121046_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79260_e121041: f64 = (locals.var_vdsi + p.p137);
        let assign79260_e121043: f64 = (assign79260_e121041 + locals.var_tmf2);
        let assign79260_e121044: f64 = (0.5 * assign79260_e121043);
        (assign79260_e121044, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79260_e121046;
        locals.var_t2_dn0 = assign79260_e121046_d_n0;
        locals.var_t2_dn2 = assign79260_e121046_d_n2;
        locals.var_t2_dn4 = assign79260_e121046_d_n4;
        locals.var_t2_dn5 = assign79260_e121046_d_n5;
        locals.var_t2_dn6 = assign79260_e121046_d_n6;
        locals.var_t2_dn7 = assign79260_e121046_d_n7;
        locals.var_t2_dn8 = assign79260_e121046_d_n8;
        locals.var_t2_dn9 = assign79260_e121046_d_n9;
        locals.var_t2_dn10 = assign79260_e121046_d_n10;
        locals.var_t2_dn13 = assign79260_e121046_d_n13;
        locals.var_t2_rv = 0.0;

        let assign79270_e121049: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1845 = assign79270_e121049;
        locals.var_guard1845_rv = 0.0;

        let (assign79280_e121060, assign79280_e121060_d_n0, assign79280_e121060_d_n2, assign79280_e121060_d_n4, assign79280_e121060_d_n5, assign79280_e121060_d_n6, assign79280_e121060_d_n7, assign79280_e121060_d_n8, assign79280_e121060_d_n9, assign79280_e121060_d_n10, assign79280_e121060_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) && (locals.var_guard1845 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79280_e121060;
        locals.var_t2_dn0 = assign79280_e121060_d_n0;
        locals.var_t2_dn2 = assign79280_e121060_d_n2;
        locals.var_t2_dn4 = assign79280_e121060_d_n4;
        locals.var_t2_dn5 = assign79280_e121060_d_n5;
        locals.var_t2_dn6 = assign79280_e121060_d_n6;
        locals.var_t2_dn7 = assign79280_e121060_d_n7;
        locals.var_t2_dn8 = assign79280_e121060_d_n8;
        locals.var_t2_dn9 = assign79280_e121060_d_n9;
        locals.var_t2_dn10 = assign79280_e121060_d_n10;
        locals.var_t2_dn13 = assign79280_e121060_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign79290_e121071, assign79290_e121071_d_n0, assign79290_e121071_d_n2, assign79290_e121071_d_n4, assign79290_e121071_d_n5, assign79290_e121071_d_n6, assign79290_e121071_d_n7, assign79290_e121071_d_n8, assign79290_e121071_d_n9, assign79290_e121071_d_n10, assign79290_e121071_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) && (locals.var_guard1845 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79290_e121071;
        locals.var_t9_dn0 = assign79290_e121071_d_n0;
        locals.var_t9_dn2 = assign79290_e121071_d_n2;
        locals.var_t9_dn4 = assign79290_e121071_d_n4;
        locals.var_t9_dn5 = assign79290_e121071_d_n5;
        locals.var_t9_dn6 = assign79290_e121071_d_n6;
        locals.var_t9_dn7 = assign79290_e121071_d_n7;
        locals.var_t9_dn8 = assign79290_e121071_d_n8;
        locals.var_t9_dn9 = assign79290_e121071_d_n9;
        locals.var_t9_dn10 = assign79290_e121071_d_n10;
        locals.var_t9_dn13 = assign79290_e121071_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign79300_e121085, assign79300_e121085_d_n0, assign79300_e121085_d_n2, assign79300_e121085_d_n4, assign79300_e121085_d_n5, assign79300_e121085_d_n6, assign79300_e121085_d_n7, assign79300_e121085_d_n8, assign79300_e121085_d_n9, assign79300_e121085_d_n10, assign79300_e121085_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79300_e121080: f64 = (locals.var_kjunc * locals.var_t2);
        let assign79300_e121081: f64 = (assign79300_e121080).sqrt();
        let assign79300_e121083: f64 = (assign79300_e121081 * p.p432);
        (assign79300_e121083, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign79300_e121081)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign79300_e121085;
        locals.var_wjunc0_dn0 = assign79300_e121085_d_n0;
        locals.var_wjunc0_dn2 = assign79300_e121085_d_n2;
        locals.var_wjunc0_dn4 = assign79300_e121085_d_n4;
        locals.var_wjunc0_dn5 = assign79300_e121085_d_n5;
        locals.var_wjunc0_dn6 = assign79300_e121085_d_n6;
        locals.var_wjunc0_dn7 = assign79300_e121085_d_n7;
        locals.var_wjunc0_dn8 = assign79300_e121085_d_n8;
        locals.var_wjunc0_dn9 = assign79300_e121085_d_n9;
        locals.var_wjunc0_dn10 = assign79300_e121085_d_n10;
        locals.var_wjunc0_dn13 = assign79300_e121085_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign79310_e121096, assign79310_e121096_d_n0, assign79310_e121096_d_n2, assign79310_e121096_d_n4, assign79310_e121096_d_n5, assign79310_e121096_d_n6, assign79310_e121096_d_n7, assign79310_e121096_d_n8, assign79310_e121096_d_n9, assign79310_e121096_d_n10, assign79310_e121096_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79310_e121094: f64 = (p.p334 - locals.var_wjunc0);
        (assign79310_e121094, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79310_e121096;
        locals.var_t2_dn0 = assign79310_e121096_d_n0;
        locals.var_t2_dn2 = assign79310_e121096_d_n2;
        locals.var_t2_dn4 = assign79310_e121096_d_n4;
        locals.var_t2_dn5 = assign79310_e121096_d_n5;
        locals.var_t2_dn6 = assign79310_e121096_d_n6;
        locals.var_t2_dn7 = assign79310_e121096_d_n7;
        locals.var_t2_dn8 = assign79310_e121096_d_n8;
        locals.var_t2_dn9 = assign79310_e121096_d_n9;
        locals.var_t2_dn10 = assign79310_e121096_d_n10;
        locals.var_t2_dn13 = assign79310_e121096_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign79320_e121115, assign79320_e121115_d_n0, assign79320_e121115_d_n2, assign79320_e121115_d_n4, assign79320_e121115_d_n5, assign79320_e121115_d_n6, assign79320_e121115_d_n7, assign79320_e121115_d_n8, assign79320_e121115_d_n9, assign79320_e121115_d_n10, assign79320_e121115_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79320_e121102: f64 = (locals.var_t2 * locals.var_t2);
        let assign79320_e121106: f64 = (p.p334 * 0.01);
        let assign79320_e121107: f64 = (4.0 * assign79320_e121106);
        let assign79320_e121110: f64 = (p.p334 * 0.01);
        let assign79320_e121111: f64 = (assign79320_e121107 * assign79320_e121110);
        let assign79320_e121112: f64 = (assign79320_e121102 + assign79320_e121111);
        let assign79320_e121113: f64 = (assign79320_e121112).sqrt();
        (assign79320_e121113, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign79320_e121113)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79320_e121115;
        locals.var_tmf2_dn0 = assign79320_e121115_d_n0;
        locals.var_tmf2_dn2 = assign79320_e121115_d_n2;
        locals.var_tmf2_dn4 = assign79320_e121115_d_n4;
        locals.var_tmf2_dn5 = assign79320_e121115_d_n5;
        locals.var_tmf2_dn6 = assign79320_e121115_d_n6;
        locals.var_tmf2_dn7 = assign79320_e121115_d_n7;
        locals.var_tmf2_dn8 = assign79320_e121115_d_n8;
        locals.var_tmf2_dn9 = assign79320_e121115_d_n9;
        locals.var_tmf2_dn10 = assign79320_e121115_d_n10;
        locals.var_tmf2_dn13 = assign79320_e121115_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_290(
        locals: &mut StampLocals,
    ) {
        let (assign79330_e121127, assign79330_e121127_d_n0, assign79330_e121127_d_n2, assign79330_e121127_d_n4, assign79330_e121127_d_n5, assign79330_e121127_d_n6, assign79330_e121127_d_n7, assign79330_e121127_d_n8, assign79330_e121127_d_n9, assign79330_e121127_d_n10, assign79330_e121127_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79330_e121123: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign79330_e121124: f64 = (1.0 + assign79330_e121123);
        let assign79330_e121125: f64 = (0.5 * assign79330_e121124);
        (assign79330_e121125, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79330_e121127;
        locals.var_t9_dn0 = assign79330_e121127_d_n0;
        locals.var_t9_dn2 = assign79330_e121127_d_n2;
        locals.var_t9_dn4 = assign79330_e121127_d_n4;
        locals.var_t9_dn5 = assign79330_e121127_d_n5;
        locals.var_t9_dn6 = assign79330_e121127_d_n6;
        locals.var_t9_dn7 = assign79330_e121127_d_n7;
        locals.var_t9_dn8 = assign79330_e121127_d_n8;
        locals.var_t9_dn9 = assign79330_e121127_d_n9;
        locals.var_t9_dn10 = assign79330_e121127_d_n10;
        locals.var_t9_dn13 = assign79330_e121127_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign79340_e121137, assign79340_e121137_d_n0, assign79340_e121137_d_n2, assign79340_e121137_d_n4, assign79340_e121137_d_n5, assign79340_e121137_d_n6, assign79340_e121137_d_n7, assign79340_e121137_d_n8, assign79340_e121137_d_n9, assign79340_e121137_d_n10, assign79340_e121137_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79340_e121134: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign79340_e121135: f64 = (0.5 * assign79340_e121134);
        (assign79340_e121135, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79340_e121137;
        locals.var_t2_dn0 = assign79340_e121137_d_n0;
        locals.var_t2_dn2 = assign79340_e121137_d_n2;
        locals.var_t2_dn4 = assign79340_e121137_d_n4;
        locals.var_t2_dn5 = assign79340_e121137_d_n5;
        locals.var_t2_dn6 = assign79340_e121137_d_n6;
        locals.var_t2_dn7 = assign79340_e121137_d_n7;
        locals.var_t2_dn8 = assign79340_e121137_d_n8;
        locals.var_t2_dn9 = assign79340_e121137_d_n9;
        locals.var_t2_dn10 = assign79340_e121137_d_n10;
        locals.var_t2_dn13 = assign79340_e121137_d_n13;
        locals.var_t2_rv = 0.0;

        let assign79350_e121140: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1846 = assign79350_e121140;
        locals.var_guard1846_rv = 0.0;

        let (assign79360_e121148, assign79360_e121148_d_n0, assign79360_e121148_d_n2, assign79360_e121148_d_n4, assign79360_e121148_d_n5, assign79360_e121148_d_n6, assign79360_e121148_d_n7, assign79360_e121148_d_n8, assign79360_e121148_d_n9, assign79360_e121148_d_n10, assign79360_e121148_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1846 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79360_e121148;
        locals.var_t2_dn0 = assign79360_e121148_d_n0;
        locals.var_t2_dn2 = assign79360_e121148_d_n2;
        locals.var_t2_dn4 = assign79360_e121148_d_n4;
        locals.var_t2_dn5 = assign79360_e121148_d_n5;
        locals.var_t2_dn6 = assign79360_e121148_d_n6;
        locals.var_t2_dn7 = assign79360_e121148_d_n7;
        locals.var_t2_dn8 = assign79360_e121148_d_n8;
        locals.var_t2_dn9 = assign79360_e121148_d_n9;
        locals.var_t2_dn10 = assign79360_e121148_d_n10;
        locals.var_t2_dn13 = assign79360_e121148_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign79370_e121156, assign79370_e121156_d_n0, assign79370_e121156_d_n2, assign79370_e121156_d_n4, assign79370_e121156_d_n5, assign79370_e121156_d_n6, assign79370_e121156_d_n7, assign79370_e121156_d_n8, assign79370_e121156_d_n9, assign79370_e121156_d_n10, assign79370_e121156_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1846 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79370_e121156;
        locals.var_t9_dn0 = assign79370_e121156_d_n0;
        locals.var_t9_dn2 = assign79370_e121156_d_n2;
        locals.var_t9_dn4 = assign79370_e121156_d_n4;
        locals.var_t9_dn5 = assign79370_e121156_d_n5;
        locals.var_t9_dn6 = assign79370_e121156_d_n6;
        locals.var_t9_dn7 = assign79370_e121156_d_n7;
        locals.var_t9_dn8 = assign79370_e121156_d_n8;
        locals.var_t9_dn9 = assign79370_e121156_d_n9;
        locals.var_t9_dn10 = assign79370_e121156_d_n10;
        locals.var_t9_dn13 = assign79370_e121156_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign79380_e121162, assign79380_e121162_d_n0, assign79380_e121162_d_n2, assign79380_e121162_d_n4, assign79380_e121162_d_n5, assign79380_e121162_d_n6, assign79380_e121162_d_n7, assign79380_e121162_d_n8, assign79380_e121162_d_n9, assign79380_e121162_d_n10, assign79380_e121162_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign79380_e121162;
        locals.var_ddriftldc_dn0 = assign79380_e121162_d_n0;
        locals.var_ddriftldc_dn2 = assign79380_e121162_d_n2;
        locals.var_ddriftldc_dn4 = assign79380_e121162_d_n4;
        locals.var_ddriftldc_dn5 = assign79380_e121162_d_n5;
        locals.var_ddriftldc_dn6 = assign79380_e121162_d_n6;
        locals.var_ddriftldc_dn7 = assign79380_e121162_d_n7;
        locals.var_ddriftldc_dn8 = assign79380_e121162_d_n8;
        locals.var_ddriftldc_dn9 = assign79380_e121162_d_n9;
        locals.var_ddriftldc_dn10 = assign79380_e121162_d_n10;
        locals.var_ddriftldc_dn13 = assign79380_e121162_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign79390_e121176, assign79390_e121176_d_n0, assign79390_e121176_d_n2, assign79390_e121176_d_n4, assign79390_e121176_d_n5, assign79390_e121176_d_n6, assign79390_e121176_d_n7, assign79390_e121176_d_n8, assign79390_e121176_d_n9, assign79390_e121176_d_n10, assign79390_e121176_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79390_e121168: f64 = (locals.var_q_nsubld__blk1762 * locals.var_ddriftldc);
        let assign79390_e121170: f64 = (assign79390_e121168 * locals.var_ddriftldc);
        let assign79390_e121172: f64 = (assign79390_e121170 / 2.0);
        let assign79390_e121174: f64 = (assign79390_e121172 / 1.034943e-10);
        (assign79390_e121174, (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign79390_e121176;
        locals.var_dphi_sb_dn0 = assign79390_e121176_d_n0;
        locals.var_dphi_sb_dn2 = assign79390_e121176_d_n2;
        locals.var_dphi_sb_dn4 = assign79390_e121176_d_n4;
        locals.var_dphi_sb_dn5 = assign79390_e121176_d_n5;
        locals.var_dphi_sb_dn6 = assign79390_e121176_d_n6;
        locals.var_dphi_sb_dn7 = assign79390_e121176_d_n7;
        locals.var_dphi_sb_dn8 = assign79390_e121176_d_n8;
        locals.var_dphi_sb_dn9 = assign79390_e121176_d_n9;
        locals.var_dphi_sb_dn10 = assign79390_e121176_d_n10;
        locals.var_dphi_sb_dn13 = assign79390_e121176_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign79400_e121187, assign79400_e121187_d_n0, assign79400_e121187_d_n2, assign79400_e121187_d_n4, assign79400_e121187_d_n5, assign79400_e121187_d_n6, assign79400_e121187_d_n7, assign79400_e121187_d_n8, assign79400_e121187_d_n9, assign79400_e121187_d_n10, assign79400_e121187_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79400_e121182: f64 = (2.0 * locals.var_beta);
        let assign79400_e121184: f64 = (assign79400_e121182 * locals.var_dphi_sb);
        let assign79400_e121185: f64 = (assign79400_e121184).sqrt();
        (assign79400_e121185, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn0)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn2)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn4)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn5)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn6)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn7)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn8)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn9)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn10)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn13)) / (2.0 * assign79400_e121185)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign79400_e121187;
        locals.var_t0_dn0 = assign79400_e121187_d_n0;
        locals.var_t0_dn2 = assign79400_e121187_d_n2;
        locals.var_t0_dn4 = assign79400_e121187_d_n4;
        locals.var_t0_dn5 = assign79400_e121187_d_n5;
        locals.var_t0_dn6 = assign79400_e121187_d_n6;
        locals.var_t0_dn7 = assign79400_e121187_d_n7;
        locals.var_t0_dn8 = assign79400_e121187_d_n8;
        locals.var_t0_dn9 = assign79400_e121187_d_n9;
        locals.var_t0_dn10 = assign79400_e121187_d_n10;
        locals.var_t0_dn13 = assign79400_e121187_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign79410_e121200, assign79410_e121200_d_n0, assign79410_e121200_d_n2, assign79410_e121200_d_n4, assign79410_e121200_d_n5, assign79410_e121200_d_n6, assign79410_e121200_d_n7, assign79410_e121200_d_n8, assign79410_e121200_d_n9, assign79410_e121200_d_n10, assign79410_e121200_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79410_e121192: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79410_e121194: f64 = (-locals.var_t0);
        let assign79410_e121195: f64 = { let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79410_e121196: f64 = (assign79410_e121192 + assign79410_e121195);
        let assign79410_e121198: f64 = (assign79410_e121196 / 2.0);
        (assign79410_e121198, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79410_e121200;
        locals.var_t1_dn0 = assign79410_e121200_d_n0;
        locals.var_t1_dn2 = assign79410_e121200_d_n2;
        locals.var_t1_dn4 = assign79410_e121200_d_n4;
        locals.var_t1_dn5 = assign79410_e121200_d_n5;
        locals.var_t1_dn6 = assign79410_e121200_d_n6;
        locals.var_t1_dn7 = assign79410_e121200_d_n7;
        locals.var_t1_dn8 = assign79410_e121200_d_n8;
        locals.var_t1_dn9 = assign79410_e121200_d_n9;
        locals.var_t1_dn10 = assign79410_e121200_d_n10;
        locals.var_t1_dn13 = assign79410_e121200_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign79420_e121209, assign79420_e121209_d_n0, assign79420_e121209_d_n2, assign79420_e121209_d_n4, assign79420_e121209_d_n5, assign79420_e121209_d_n6, assign79420_e121209_d_n7, assign79420_e121209_d_n8, assign79420_e121209_d_n9, assign79420_e121209_d_n10, assign79420_e121209_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79420_e121205: f64 = (locals.var_t1).ln();
        let assign79420_e121207: f64 = (assign79420_e121205 / locals.var_dphi_sb);
        (assign79420_e121207, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign79420_e121209;
        locals.var_c_sb_dn0 = assign79420_e121209_d_n0;
        locals.var_c_sb_dn2 = assign79420_e121209_d_n2;
        locals.var_c_sb_dn4 = assign79420_e121209_d_n4;
        locals.var_c_sb_dn5 = assign79420_e121209_d_n5;
        locals.var_c_sb_dn6 = assign79420_e121209_d_n6;
        locals.var_c_sb_dn7 = assign79420_e121209_d_n7;
        locals.var_c_sb_dn8 = assign79420_e121209_d_n8;
        locals.var_c_sb_dn9 = assign79420_e121209_d_n9;
        locals.var_c_sb_dn10 = assign79420_e121209_d_n10;
        locals.var_c_sb_dn13 = assign79420_e121209_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign79430_e121215,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign79430_e121215;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_291(
        locals: &mut StampLocals,
    ) {
        let mut assign79440_loop_guard: usize = 0;
        while {
            let assign79440_cond_e121222: f64 = (locals.var_lp_s0_max + 1.0);
            let assign79440_cond_e121224: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_lp_s0 <= assign79440_cond_e121222)) { 1.0 } else { 0.0 };
            assign79440_cond_e121224 != 0.0
        } {
            assign79440_loop_guard += 1;
            assert!(assign79440_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign79440_body3_e121251, assign79440_body3_e121251_d_n0, assign79440_body3_e121251_d_n2, assign79440_body3_e121251_d_n4, assign79440_body3_e121251_d_n5, assign79440_body3_e121251_d_n6, assign79440_body3_e121251_d_n7, assign79440_body3_e121251_d_n8, assign79440_body3_e121251_d_n9, assign79440_body3_e121251_d_n10, assign79440_body3_e121251_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body3_e121249: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign79440_body3_e121249, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign79440_body3_e121251;
            locals.var_ps0ld_vxb_dn0 = assign79440_body3_e121251_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign79440_body3_e121251_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign79440_body3_e121251_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign79440_body3_e121251_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign79440_body3_e121251_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign79440_body3_e121251_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign79440_body3_e121251_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign79440_body3_e121251_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign79440_body3_e121251_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign79440_body3_e121251_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign79440_body4_e121259, assign79440_body4_e121259_d_n0, assign79440_body4_e121259_d_n2, assign79440_body4_e121259_d_n4, assign79440_body4_e121259_d_n5, assign79440_body4_e121259_d_n6, assign79440_body4_e121259_d_n7, assign79440_body4_e121259_d_n8, assign79440_body4_e121259_d_n9, assign79440_body4_e121259_d_n10, assign79440_body4_e121259_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body4_e121257: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign79440_body4_e121257, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign79440_body4_e121259;
            locals.var_chi_dn0 = assign79440_body4_e121259_d_n0;
            locals.var_chi_dn2 = assign79440_body4_e121259_d_n2;
            locals.var_chi_dn4 = assign79440_body4_e121259_d_n4;
            locals.var_chi_dn5 = assign79440_body4_e121259_d_n5;
            locals.var_chi_dn6 = assign79440_body4_e121259_d_n6;
            locals.var_chi_dn7 = assign79440_body4_e121259_d_n7;
            locals.var_chi_dn8 = assign79440_body4_e121259_d_n8;
            locals.var_chi_dn9 = assign79440_body4_e121259_d_n9;
            locals.var_chi_dn10 = assign79440_body4_e121259_d_n10;
            locals.var_chi_dn13 = assign79440_body4_e121259_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign79440_body5_e121269, assign79440_body5_e121269_d_n0, assign79440_body5_e121269_d_n2, assign79440_body5_e121269_d_n4, assign79440_body5_e121269_d_n5, assign79440_body5_e121269_d_n6, assign79440_body5_e121269_d_n7, assign79440_body5_e121269_d_n8, assign79440_body5_e121269_d_n9, assign79440_body5_e121269_d_n10, assign79440_body5_e121269_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body5_e121266: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign79440_body5_e121267: f64 = (locals.var_c_sb * assign79440_body5_e121266);
        (assign79440_body5_e121267, ((locals.var_c_sb_dn0 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign79440_body5_e121269;
            locals.var_ty_dn0 = assign79440_body5_e121269_d_n0;
            locals.var_ty_dn2 = assign79440_body5_e121269_d_n2;
            locals.var_ty_dn4 = assign79440_body5_e121269_d_n4;
            locals.var_ty_dn5 = assign79440_body5_e121269_d_n5;
            locals.var_ty_dn6 = assign79440_body5_e121269_d_n6;
            locals.var_ty_dn7 = assign79440_body5_e121269_d_n7;
            locals.var_ty_dn8 = assign79440_body5_e121269_d_n8;
            locals.var_ty_dn9 = assign79440_body5_e121269_d_n9;
            locals.var_ty_dn10 = assign79440_body5_e121269_d_n10;
            locals.var_ty_dn13 = assign79440_body5_e121269_d_n13;
            locals.var_ty_rv = 0.0;
            let assign79440_body6_e121272: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1848 = assign79440_body6_e121272;
            locals.var_guard1848_rv = 0.0;
            let (assign79440_body7_e121281, assign79440_body7_e121281_d_n0, assign79440_body7_e121281_d_n2, assign79440_body7_e121281_d_n4, assign79440_body7_e121281_d_n5, assign79440_body7_e121281_d_n6, assign79440_body7_e121281_d_n7, assign79440_body7_e121281_d_n8, assign79440_body7_e121281_d_n9, assign79440_body7_e121281_d_n10, assign79440_body7_e121281_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body7_e121279: f64 = (locals.var_ty).exp();
        (assign79440_body7_e121279, (assign79440_body7_e121279 * locals.var_ty_dn0), (assign79440_body7_e121279 * locals.var_ty_dn2), (assign79440_body7_e121279 * locals.var_ty_dn4), (assign79440_body7_e121279 * locals.var_ty_dn5), (assign79440_body7_e121279 * locals.var_ty_dn6), (assign79440_body7_e121279 * locals.var_ty_dn7), (assign79440_body7_e121279 * locals.var_ty_dn8), (assign79440_body7_e121279 * locals.var_ty_dn9), (assign79440_body7_e121279 * locals.var_ty_dn10), (assign79440_body7_e121279 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body7_e121281;
            locals.var_t1_dn0 = assign79440_body7_e121281_d_n0;
            locals.var_t1_dn2 = assign79440_body7_e121281_d_n2;
            locals.var_t1_dn4 = assign79440_body7_e121281_d_n4;
            locals.var_t1_dn5 = assign79440_body7_e121281_d_n5;
            locals.var_t1_dn6 = assign79440_body7_e121281_d_n6;
            locals.var_t1_dn7 = assign79440_body7_e121281_d_n7;
            locals.var_t1_dn8 = assign79440_body7_e121281_d_n8;
            locals.var_t1_dn9 = assign79440_body7_e121281_d_n9;
            locals.var_t1_dn10 = assign79440_body7_e121281_d_n10;
            locals.var_t1_dn13 = assign79440_body7_e121281_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79440_body8_e121293, assign79440_body8_e121293_d_n0, assign79440_body8_e121293_d_n2, assign79440_body8_e121293_d_n4, assign79440_body8_e121293_d_n5, assign79440_body8_e121293_d_n6, assign79440_body8_e121293_d_n7, assign79440_body8_e121293_d_n8, assign79440_body8_e121293_d_n9, assign79440_body8_e121293_d_n10, assign79440_body8_e121293_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body8_e121288: f64 = (-locals.var_c_sb);
        let assign79440_body8_e121290: f64 = (assign79440_body8_e121288 * locals.var_dphi_sb);
        let assign79440_body8_e121291: f64 = (assign79440_body8_e121290).exp();
        (assign79440_body8_e121291, (assign79440_body8_e121291 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn0))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn2))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn4))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn5))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn6))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn7))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn8))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn9))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn10))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body8_e121293;
            locals.var_t0_dn0 = assign79440_body8_e121293_d_n0;
            locals.var_t0_dn2 = assign79440_body8_e121293_d_n2;
            locals.var_t0_dn4 = assign79440_body8_e121293_d_n4;
            locals.var_t0_dn5 = assign79440_body8_e121293_d_n5;
            locals.var_t0_dn6 = assign79440_body8_e121293_d_n6;
            locals.var_t0_dn7 = assign79440_body8_e121293_d_n7;
            locals.var_t0_dn8 = assign79440_body8_e121293_d_n8;
            locals.var_t0_dn9 = assign79440_body8_e121293_d_n9;
            locals.var_t0_dn10 = assign79440_body8_e121293_d_n10;
            locals.var_t0_dn13 = assign79440_body8_e121293_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79440_body9_e121303, assign79440_body9_e121303_d_n0, assign79440_body9_e121303_d_n2, assign79440_body9_e121303_d_n4, assign79440_body9_e121303_d_n5, assign79440_body9_e121303_d_n6, assign79440_body9_e121303_d_n7, assign79440_body9_e121303_d_n8, assign79440_body9_e121303_d_n9, assign79440_body9_e121303_d_n10, assign79440_body9_e121303_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body9_e121301: f64 = (locals.var_t1 - locals.var_t0);
        (assign79440_body9_e121301, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79440_body9_e121303;
            locals.var_t2_dn0 = assign79440_body9_e121303_d_n0;
            locals.var_t2_dn2 = assign79440_body9_e121303_d_n2;
            locals.var_t2_dn4 = assign79440_body9_e121303_d_n4;
            locals.var_t2_dn5 = assign79440_body9_e121303_d_n5;
            locals.var_t2_dn6 = assign79440_body9_e121303_d_n6;
            locals.var_t2_dn7 = assign79440_body9_e121303_d_n7;
            locals.var_t2_dn8 = assign79440_body9_e121303_d_n8;
            locals.var_t2_dn9 = assign79440_body9_e121303_d_n9;
            locals.var_t2_dn10 = assign79440_body9_e121303_d_n10;
            locals.var_t2_dn13 = assign79440_body9_e121303_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign79440_body10_e121316, assign79440_body10_e121316_d_n0, assign79440_body10_e121316_d_n2, assign79440_body10_e121316_d_n4, assign79440_body10_e121316_d_n5, assign79440_body10_e121316_d_n6, assign79440_body10_e121316_d_n7, assign79440_body10_e121316_d_n8, assign79440_body10_e121316_d_n9, assign79440_body10_e121316_d_n10, assign79440_body10_e121316_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body10_e121311: f64 = (1.0 + locals.var_t2);
        let assign79440_body10_e121312: f64 = (assign79440_body10_e121311).ln();
        let assign79440_body10_e121314: f64 = (assign79440_body10_e121312 / locals.var_c_sb);
        (assign79440_body10_e121314, ((((locals.var_t2_dn0 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79440_body10_e121316;
            locals.var_phi_b_dn0 = assign79440_body10_e121316_d_n0;
            locals.var_phi_b_dn2 = assign79440_body10_e121316_d_n2;
            locals.var_phi_b_dn4 = assign79440_body10_e121316_d_n4;
            locals.var_phi_b_dn5 = assign79440_body10_e121316_d_n5;
            locals.var_phi_b_dn6 = assign79440_body10_e121316_d_n6;
            locals.var_phi_b_dn7 = assign79440_body10_e121316_d_n7;
            locals.var_phi_b_dn8 = assign79440_body10_e121316_d_n8;
            locals.var_phi_b_dn9 = assign79440_body10_e121316_d_n9;
            locals.var_phi_b_dn10 = assign79440_body10_e121316_d_n10;
            locals.var_phi_b_dn13 = assign79440_body10_e121316_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign79440_body11_e121328, assign79440_body11_e121328_d_n0, assign79440_body11_e121328_d_n2, assign79440_body11_e121328_d_n4, assign79440_body11_e121328_d_n5, assign79440_body11_e121328_d_n6, assign79440_body11_e121328_d_n7, assign79440_body11_e121328_d_n8, assign79440_body11_e121328_d_n9, assign79440_body11_e121328_d_n10, assign79440_body11_e121328_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body11_e121325: f64 = (1.0 + locals.var_t2);
        let assign79440_body11_e121326: f64 = (locals.var_t1 / assign79440_body11_e121325);
        (assign79440_body11_e121326, (((locals.var_t1_dn0 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn0)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn2 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn2)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn4 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn4)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn5 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn5)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn6 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn6)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn7 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn7)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn8 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn8)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn9 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn9)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn10 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn10)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn13 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn13)) / (assign79440_body11_e121325 * assign79440_body11_e121325)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79440_body11_e121328;
            locals.var_phi_b_dpss_dn0 = assign79440_body11_e121328_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79440_body11_e121328_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79440_body11_e121328_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79440_body11_e121328_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79440_body11_e121328_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79440_body11_e121328_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79440_body11_e121328_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79440_body11_e121328_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79440_body11_e121328_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79440_body11_e121328_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign79440_body12_e121339, assign79440_body12_e121339_d_n0, assign79440_body12_e121339_d_n2, assign79440_body12_e121339_d_n4, assign79440_body12_e121339_d_n5, assign79440_body12_e121339_d_n6, assign79440_body12_e121339_d_n7, assign79440_body12_e121339_d_n8, assign79440_body12_e121339_d_n9, assign79440_body12_e121339_d_n10, assign79440_body12_e121339_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        let assign79440_body12_e121337: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign79440_body12_e121337, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79440_body12_e121339;
            locals.var_phi_b_dn0 = assign79440_body12_e121339_d_n0;
            locals.var_phi_b_dn2 = assign79440_body12_e121339_d_n2;
            locals.var_phi_b_dn4 = assign79440_body12_e121339_d_n4;
            locals.var_phi_b_dn5 = assign79440_body12_e121339_d_n5;
            locals.var_phi_b_dn6 = assign79440_body12_e121339_d_n6;
            locals.var_phi_b_dn7 = assign79440_body12_e121339_d_n7;
            locals.var_phi_b_dn8 = assign79440_body12_e121339_d_n8;
            locals.var_phi_b_dn9 = assign79440_body12_e121339_d_n9;
            locals.var_phi_b_dn10 = assign79440_body12_e121339_d_n10;
            locals.var_phi_b_dn13 = assign79440_body12_e121339_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign79440_body13_e121348, assign79440_body13_e121348_d_n0, assign79440_body13_e121348_d_n2, assign79440_body13_e121348_d_n4, assign79440_body13_e121348_d_n5, assign79440_body13_e121348_d_n6, assign79440_body13_e121348_d_n7, assign79440_body13_e121348_d_n8, assign79440_body13_e121348_d_n9, assign79440_body13_e121348_d_n10, assign79440_body13_e121348_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79440_body13_e121348;
            locals.var_phi_b_dpss_dn0 = assign79440_body13_e121348_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79440_body13_e121348_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79440_body13_e121348_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79440_body13_e121348_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79440_body13_e121348_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79440_body13_e121348_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79440_body13_e121348_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79440_body13_e121348_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79440_body13_e121348_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79440_body13_e121348_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign79440_body14_e121356, assign79440_body14_e121356_d_n0, assign79440_body14_e121356_d_n2, assign79440_body14_e121356_d_n4, assign79440_body14_e121356_d_n5, assign79440_body14_e121356_d_n6, assign79440_body14_e121356_d_n7, assign79440_body14_e121356_d_n8, assign79440_body14_e121356_d_n9, assign79440_body14_e121356_d_n10, assign79440_body14_e121356_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body14_e121354: f64 = (locals.var_beta * locals.var_phi_b);
        (assign79440_body14_e121354, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign79440_body14_e121356;
            locals.var_chib_dn0 = assign79440_body14_e121356_d_n0;
            locals.var_chib_dn2 = assign79440_body14_e121356_d_n2;
            locals.var_chib_dn4 = assign79440_body14_e121356_d_n4;
            locals.var_chib_dn5 = assign79440_body14_e121356_d_n5;
            locals.var_chib_dn6 = assign79440_body14_e121356_d_n6;
            locals.var_chib_dn7 = assign79440_body14_e121356_d_n7;
            locals.var_chib_dn8 = assign79440_body14_e121356_d_n8;
            locals.var_chib_dn9 = assign79440_body14_e121356_d_n9;
            locals.var_chib_dn10 = assign79440_body14_e121356_d_n10;
            locals.var_chib_dn13 = assign79440_body14_e121356_d_n13;
            locals.var_chib_rv = 0.0;
            let assign79440_body15_e121358: f64 = (locals.var_chi).abs();
            let assign79440_body15_e121360: f64 = if assign79440_body15_e121358 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1849 = assign79440_body15_e121360;
            locals.var_guard1849_rv = 0.0;
            let (assign79440_body17_e121406, assign79440_body17_e121406_d_n0, assign79440_body17_e121406_d_n2, assign79440_body17_e121406_d_n4, assign79440_body17_e121406_d_n5, assign79440_body17_e121406_d_n6, assign79440_body17_e121406_d_n7, assign79440_body17_e121406_d_n8, assign79440_body17_e121406_d_n9, assign79440_body17_e121406_d_n10, assign79440_body17_e121406_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body17_e121384: f64 = (locals.var_chi * locals.var_chi);
        let assign79440_body17_e121386: f64 = (assign79440_body17_e121384 / 2.0);
        let assign79440_body17_e121390: f64 = (locals.var_chi / 3.0);
        let assign79440_body17_e121394: f64 = (locals.var_chi / 4.0);
        let assign79440_body17_e121398: f64 = (locals.var_chi / 5.0);
        let assign79440_body17_e121399: f64 = (1.0 - assign79440_body17_e121398);
        let assign79440_body17_e121400: f64 = (assign79440_body17_e121394 * assign79440_body17_e121399);
        let assign79440_body17_e121401: f64 = (1.0 - assign79440_body17_e121400);
        let assign79440_body17_e121402: f64 = (assign79440_body17_e121390 * assign79440_body17_e121401);
        let assign79440_body17_e121403: f64 = (1.0 - assign79440_body17_e121402);
        let assign79440_body17_e121404: f64 = (assign79440_body17_e121386 * assign79440_body17_e121403);
        (assign79440_body17_e121404, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn0 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn0 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn2 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn2 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn4 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn4 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn5 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn5 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn6 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn6 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn7 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn7 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn8 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn8 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn9 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn9 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn10 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn10 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn13 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn13 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body17_e121406;
            locals.var_t0_dn0 = assign79440_body17_e121406_d_n0;
            locals.var_t0_dn2 = assign79440_body17_e121406_d_n2;
            locals.var_t0_dn4 = assign79440_body17_e121406_d_n4;
            locals.var_t0_dn5 = assign79440_body17_e121406_d_n5;
            locals.var_t0_dn6 = assign79440_body17_e121406_d_n6;
            locals.var_t0_dn7 = assign79440_body17_e121406_d_n7;
            locals.var_t0_dn8 = assign79440_body17_e121406_d_n8;
            locals.var_t0_dn9 = assign79440_body17_e121406_d_n9;
            locals.var_t0_dn10 = assign79440_body17_e121406_d_n10;
            locals.var_t0_dn13 = assign79440_body17_e121406_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79440_body18_e121432, assign79440_body18_e121432_d_n0, assign79440_body18_e121432_d_n2, assign79440_body18_e121432_d_n4, assign79440_body18_e121432_d_n5, assign79440_body18_e121432_d_n6, assign79440_body18_e121432_d_n7, assign79440_body18_e121432_d_n8, assign79440_body18_e121432_d_n9, assign79440_body18_e121432_d_n10, assign79440_body18_e121432_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body18_e121416: f64 = (locals.var_chi / 2.0);
        let assign79440_body18_e121420: f64 = (locals.var_chi / 3.0);
        let assign79440_body18_e121424: f64 = (locals.var_chi / 4.0);
        let assign79440_body18_e121425: f64 = (1.0 - assign79440_body18_e121424);
        let assign79440_body18_e121426: f64 = (assign79440_body18_e121420 * assign79440_body18_e121425);
        let assign79440_body18_e121427: f64 = (1.0 - assign79440_body18_e121426);
        let assign79440_body18_e121428: f64 = (assign79440_body18_e121416 * assign79440_body18_e121427);
        let assign79440_body18_e121429: f64 = (1.0 - assign79440_body18_e121428);
        let assign79440_body18_e121430: f64 = (locals.var_chi * assign79440_body18_e121429);
        (assign79440_body18_e121430, ((locals.var_chi_dn0 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn0 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn2 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn4 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn5 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn6 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn7 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn8 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn9 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn10 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn13 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body18_e121432;
            locals.var_t1_dn0 = assign79440_body18_e121432_d_n0;
            locals.var_t1_dn2 = assign79440_body18_e121432_d_n2;
            locals.var_t1_dn4 = assign79440_body18_e121432_d_n4;
            locals.var_t1_dn5 = assign79440_body18_e121432_d_n5;
            locals.var_t1_dn6 = assign79440_body18_e121432_d_n6;
            locals.var_t1_dn7 = assign79440_body18_e121432_d_n7;
            locals.var_t1_dn8 = assign79440_body18_e121432_d_n8;
            locals.var_t1_dn9 = assign79440_body18_e121432_d_n9;
            locals.var_t1_dn10 = assign79440_body18_e121432_d_n10;
            locals.var_t1_dn13 = assign79440_body18_e121432_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79440_body19_e121462, assign79440_body19_e121462_d_n0, assign79440_body19_e121462_d_n2, assign79440_body19_e121462_d_n4, assign79440_body19_e121462_d_n5, assign79440_body19_e121462_d_n6, assign79440_body19_e121462_d_n7, assign79440_body19_e121462_d_n8, assign79440_body19_e121462_d_n9, assign79440_body19_e121462_d_n10, assign79440_body19_e121462_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body19_e121440: f64 = (locals.var_chib * locals.var_chib);
        let assign79440_body19_e121442: f64 = (assign79440_body19_e121440 / 2.0);
        let assign79440_body19_e121446: f64 = (locals.var_chib / 3.0);
        let assign79440_body19_e121450: f64 = (locals.var_chib / 4.0);
        let assign79440_body19_e121454: f64 = (locals.var_chib / 5.0);
        let assign79440_body19_e121455: f64 = (1.0 - assign79440_body19_e121454);
        let assign79440_body19_e121456: f64 = (assign79440_body19_e121450 * assign79440_body19_e121455);
        let assign79440_body19_e121457: f64 = (1.0 - assign79440_body19_e121456);
        let assign79440_body19_e121458: f64 = (assign79440_body19_e121446 * assign79440_body19_e121457);
        let assign79440_body19_e121459: f64 = (1.0 - assign79440_body19_e121458);
        let assign79440_body19_e121460: f64 = (assign79440_body19_e121442 * assign79440_body19_e121459);
        (assign79440_body19_e121460, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn0 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn0 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn2 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn2 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn4 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn4 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn5 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn5 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn6 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn6 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn7 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn7 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn8 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn8 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn9 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn9 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn10 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn10 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn13 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn13 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79440_body19_e121462;
            locals.var_t2_dn0 = assign79440_body19_e121462_d_n0;
            locals.var_t2_dn2 = assign79440_body19_e121462_d_n2;
            locals.var_t2_dn4 = assign79440_body19_e121462_d_n4;
            locals.var_t2_dn5 = assign79440_body19_e121462_d_n5;
            locals.var_t2_dn6 = assign79440_body19_e121462_d_n6;
            locals.var_t2_dn7 = assign79440_body19_e121462_d_n7;
            locals.var_t2_dn8 = assign79440_body19_e121462_d_n8;
            locals.var_t2_dn9 = assign79440_body19_e121462_d_n9;
            locals.var_t2_dn10 = assign79440_body19_e121462_d_n10;
            locals.var_t2_dn13 = assign79440_body19_e121462_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign79440_body20_e121488, assign79440_body20_e121488_d_n0, assign79440_body20_e121488_d_n2, assign79440_body20_e121488_d_n4, assign79440_body20_e121488_d_n5, assign79440_body20_e121488_d_n6, assign79440_body20_e121488_d_n7, assign79440_body20_e121488_d_n8, assign79440_body20_e121488_d_n9, assign79440_body20_e121488_d_n10, assign79440_body20_e121488_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body20_e121472: f64 = (locals.var_chib / 2.0);
        let assign79440_body20_e121476: f64 = (locals.var_chib / 3.0);
        let assign79440_body20_e121480: f64 = (locals.var_chib / 4.0);
        let assign79440_body20_e121481: f64 = (1.0 - assign79440_body20_e121480);
        let assign79440_body20_e121482: f64 = (assign79440_body20_e121476 * assign79440_body20_e121481);
        let assign79440_body20_e121483: f64 = (1.0 - assign79440_body20_e121482);
        let assign79440_body20_e121484: f64 = (assign79440_body20_e121472 * assign79440_body20_e121483);
        let assign79440_body20_e121485: f64 = (1.0 - assign79440_body20_e121484);
        let assign79440_body20_e121486: f64 = (locals.var_chib * assign79440_body20_e121485);
        (assign79440_body20_e121486, ((locals.var_chib_dn0 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn0 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn2 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn4 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn5 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn6 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn7 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn8 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn9 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn10 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn13 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign79440_body20_e121488;
            locals.var_t3_dn0 = assign79440_body20_e121488_d_n0;
            locals.var_t3_dn2 = assign79440_body20_e121488_d_n2;
            locals.var_t3_dn4 = assign79440_body20_e121488_d_n4;
            locals.var_t3_dn5 = assign79440_body20_e121488_d_n5;
            locals.var_t3_dn6 = assign79440_body20_e121488_d_n6;
            locals.var_t3_dn7 = assign79440_body20_e121488_d_n7;
            locals.var_t3_dn8 = assign79440_body20_e121488_d_n8;
            locals.var_t3_dn9 = assign79440_body20_e121488_d_n9;
            locals.var_t3_dn10 = assign79440_body20_e121488_d_n10;
            locals.var_t3_dn13 = assign79440_body20_e121488_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign79440_body21_e121498, assign79440_body21_e121498_d_n0, assign79440_body21_e121498_d_n2, assign79440_body21_e121498_d_n4, assign79440_body21_e121498_d_n5, assign79440_body21_e121498_d_n6, assign79440_body21_e121498_d_n7, assign79440_body21_e121498_d_n8, assign79440_body21_e121498_d_n9, assign79440_body21_e121498_d_n10, assign79440_body21_e121498_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body21_e121496: f64 = (locals.var_t0 - locals.var_t2);
        (assign79440_body21_e121496, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk1770, locals.var_fbsq__blk1770_dn0, locals.var_fbsq__blk1770_dn2, locals.var_fbsq__blk1770_dn4, locals.var_fbsq__blk1770_dn5, locals.var_fbsq__blk1770_dn6, locals.var_fbsq__blk1770_dn7, locals.var_fbsq__blk1770_dn8, locals.var_fbsq__blk1770_dn9, locals.var_fbsq__blk1770_dn10, locals.var_fbsq__blk1770_dn13,)
    }
};
            locals.var_fbsq__blk1770 = assign79440_body21_e121498;
            locals.var_fbsq__blk1770_dn0 = assign79440_body21_e121498_d_n0;
            locals.var_fbsq__blk1770_dn2 = assign79440_body21_e121498_d_n2;
            locals.var_fbsq__blk1770_dn4 = assign79440_body21_e121498_d_n4;
            locals.var_fbsq__blk1770_dn5 = assign79440_body21_e121498_d_n5;
            locals.var_fbsq__blk1770_dn6 = assign79440_body21_e121498_d_n6;
            locals.var_fbsq__blk1770_dn7 = assign79440_body21_e121498_d_n7;
            locals.var_fbsq__blk1770_dn8 = assign79440_body21_e121498_d_n8;
            locals.var_fbsq__blk1770_dn9 = assign79440_body21_e121498_d_n9;
            locals.var_fbsq__blk1770_dn10 = assign79440_body21_e121498_d_n10;
            locals.var_fbsq__blk1770_dn13 = assign79440_body21_e121498_d_n13;
            locals.var_fbsq__blk1770_rv = 0.0;
            let (assign79440_body22_e121512, assign79440_body22_e121512_d_n0, assign79440_body22_e121512_d_n2, assign79440_body22_e121512_d_n4, assign79440_body22_e121512_d_n5, assign79440_body22_e121512_d_n6, assign79440_body22_e121512_d_n7, assign79440_body22_e121512_d_n8, assign79440_body22_e121512_d_n9, assign79440_body22_e121512_d_n10, assign79440_body22_e121512_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body22_e121508: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign79440_body22_e121509: f64 = (locals.var_t1 - assign79440_body22_e121508);
        let assign79440_body22_e121510: f64 = (locals.var_beta * assign79440_body22_e121509);
        (assign79440_body22_e121510, ((locals.var_beta_dn0 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk1771, locals.var_fbsq_dpss__blk1771_dn0, locals.var_fbsq_dpss__blk1771_dn2, locals.var_fbsq_dpss__blk1771_dn4, locals.var_fbsq_dpss__blk1771_dn5, locals.var_fbsq_dpss__blk1771_dn6, locals.var_fbsq_dpss__blk1771_dn7, locals.var_fbsq_dpss__blk1771_dn8, locals.var_fbsq_dpss__blk1771_dn9, locals.var_fbsq_dpss__blk1771_dn10, locals.var_fbsq_dpss__blk1771_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1771 = assign79440_body22_e121512;
            locals.var_fbsq_dpss__blk1771_dn0 = assign79440_body22_e121512_d_n0;
            locals.var_fbsq_dpss__blk1771_dn2 = assign79440_body22_e121512_d_n2;
            locals.var_fbsq_dpss__blk1771_dn4 = assign79440_body22_e121512_d_n4;
            locals.var_fbsq_dpss__blk1771_dn5 = assign79440_body22_e121512_d_n5;
            locals.var_fbsq_dpss__blk1771_dn6 = assign79440_body22_e121512_d_n6;
            locals.var_fbsq_dpss__blk1771_dn7 = assign79440_body22_e121512_d_n7;
            locals.var_fbsq_dpss__blk1771_dn8 = assign79440_body22_e121512_d_n8;
            locals.var_fbsq_dpss__blk1771_dn9 = assign79440_body22_e121512_d_n9;
            locals.var_fbsq_dpss__blk1771_dn10 = assign79440_body22_e121512_d_n10;
            locals.var_fbsq_dpss__blk1771_dn13 = assign79440_body22_e121512_d_n13;
            locals.var_fbsq_dpss__blk1771_rv = 0.0;
            let (assign79440_body24_e121540, assign79440_body24_e121540_d_n0, assign79440_body24_e121540_d_n2, assign79440_body24_e121540_d_n4, assign79440_body24_e121540_d_n5, assign79440_body24_e121540_d_n6, assign79440_body24_e121540_d_n7, assign79440_body24_e121540_d_n8, assign79440_body24_e121540_d_n9, assign79440_body24_e121540_d_n10, assign79440_body24_e121540_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body24_e121537: f64 = (-locals.var_chi);
        let assign79440_body24_e121538: f64 = (assign79440_body24_e121537).exp();
        (assign79440_body24_e121538, (assign79440_body24_e121538 * (-locals.var_chi_dn0)), (assign79440_body24_e121538 * (-locals.var_chi_dn2)), (assign79440_body24_e121538 * (-locals.var_chi_dn4)), (assign79440_body24_e121538 * (-locals.var_chi_dn5)), (assign79440_body24_e121538 * (-locals.var_chi_dn6)), (assign79440_body24_e121538 * (-locals.var_chi_dn7)), (assign79440_body24_e121538 * (-locals.var_chi_dn8)), (assign79440_body24_e121538 * (-locals.var_chi_dn9)), (assign79440_body24_e121538 * (-locals.var_chi_dn10)), (assign79440_body24_e121538 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body24_e121540;
            locals.var_t0_dn0 = assign79440_body24_e121540_d_n0;
            locals.var_t0_dn2 = assign79440_body24_e121540_d_n2;
            locals.var_t0_dn4 = assign79440_body24_e121540_d_n4;
            locals.var_t0_dn5 = assign79440_body24_e121540_d_n5;
            locals.var_t0_dn6 = assign79440_body24_e121540_d_n6;
            locals.var_t0_dn7 = assign79440_body24_e121540_d_n7;
            locals.var_t0_dn8 = assign79440_body24_e121540_d_n8;
            locals.var_t0_dn9 = assign79440_body24_e121540_d_n9;
            locals.var_t0_dn10 = assign79440_body24_e121540_d_n10;
            locals.var_t0_dn13 = assign79440_body24_e121540_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79440_body25_e121551, assign79440_body25_e121551_d_n0, assign79440_body25_e121551_d_n2, assign79440_body25_e121551_d_n4, assign79440_body25_e121551_d_n5, assign79440_body25_e121551_d_n6, assign79440_body25_e121551_d_n7, assign79440_body25_e121551_d_n8, assign79440_body25_e121551_d_n9, assign79440_body25_e121551_d_n10, assign79440_body25_e121551_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body25_e121548: f64 = (-locals.var_chib);
        let assign79440_body25_e121549: f64 = (assign79440_body25_e121548).exp();
        (assign79440_body25_e121549, (assign79440_body25_e121549 * (-locals.var_chib_dn0)), (assign79440_body25_e121549 * (-locals.var_chib_dn2)), (assign79440_body25_e121549 * (-locals.var_chib_dn4)), (assign79440_body25_e121549 * (-locals.var_chib_dn5)), (assign79440_body25_e121549 * (-locals.var_chib_dn6)), (assign79440_body25_e121549 * (-locals.var_chib_dn7)), (assign79440_body25_e121549 * (-locals.var_chib_dn8)), (assign79440_body25_e121549 * (-locals.var_chib_dn9)), (assign79440_body25_e121549 * (-locals.var_chib_dn10)), (assign79440_body25_e121549 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body25_e121551;
            locals.var_t1_dn0 = assign79440_body25_e121551_d_n0;
            locals.var_t1_dn2 = assign79440_body25_e121551_d_n2;
            locals.var_t1_dn4 = assign79440_body25_e121551_d_n4;
            locals.var_t1_dn5 = assign79440_body25_e121551_d_n5;
            locals.var_t1_dn6 = assign79440_body25_e121551_d_n6;
            locals.var_t1_dn7 = assign79440_body25_e121551_d_n7;
            locals.var_t1_dn8 = assign79440_body25_e121551_d_n8;
            locals.var_t1_dn9 = assign79440_body25_e121551_d_n9;
            locals.var_t1_dn10 = assign79440_body25_e121551_d_n10;
            locals.var_t1_dn13 = assign79440_body25_e121551_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79440_body26_e121566, assign79440_body26_e121566_d_n0, assign79440_body26_e121566_d_n2, assign79440_body26_e121566_d_n4, assign79440_body26_e121566_d_n5, assign79440_body26_e121566_d_n6, assign79440_body26_e121566_d_n7, assign79440_body26_e121566_d_n8, assign79440_body26_e121566_d_n9, assign79440_body26_e121566_d_n10, assign79440_body26_e121566_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body26_e121560: f64 = (locals.var_chi - locals.var_chib);
        let assign79440_body26_e121563: f64 = (locals.var_t0 - locals.var_t1);
        let assign79440_body26_e121564: f64 = (assign79440_body26_e121560 + assign79440_body26_e121563);
        (assign79440_body26_e121564, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk1770, locals.var_fbsq__blk1770_dn0, locals.var_fbsq__blk1770_dn2, locals.var_fbsq__blk1770_dn4, locals.var_fbsq__blk1770_dn5, locals.var_fbsq__blk1770_dn6, locals.var_fbsq__blk1770_dn7, locals.var_fbsq__blk1770_dn8, locals.var_fbsq__blk1770_dn9, locals.var_fbsq__blk1770_dn10, locals.var_fbsq__blk1770_dn13,)
    }
};
            locals.var_fbsq__blk1770 = assign79440_body26_e121566;
            locals.var_fbsq__blk1770_dn0 = assign79440_body26_e121566_d_n0;
            locals.var_fbsq__blk1770_dn2 = assign79440_body26_e121566_d_n2;
            locals.var_fbsq__blk1770_dn4 = assign79440_body26_e121566_d_n4;
            locals.var_fbsq__blk1770_dn5 = assign79440_body26_e121566_d_n5;
            locals.var_fbsq__blk1770_dn6 = assign79440_body26_e121566_d_n6;
            locals.var_fbsq__blk1770_dn7 = assign79440_body26_e121566_d_n7;
            locals.var_fbsq__blk1770_dn8 = assign79440_body26_e121566_d_n8;
            locals.var_fbsq__blk1770_dn9 = assign79440_body26_e121566_d_n9;
            locals.var_fbsq__blk1770_dn10 = assign79440_body26_e121566_d_n10;
            locals.var_fbsq__blk1770_dn13 = assign79440_body26_e121566_d_n13;
            locals.var_fbsq__blk1770_rv = 0.0;
            let (assign79440_body27_e121585, assign79440_body27_e121585_d_n0, assign79440_body27_e121585_d_n2, assign79440_body27_e121585_d_n4, assign79440_body27_e121585_d_n5, assign79440_body27_e121585_d_n6, assign79440_body27_e121585_d_n7, assign79440_body27_e121585_d_n8, assign79440_body27_e121585_d_n9, assign79440_body27_e121585_d_n10, assign79440_body27_e121585_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body27_e121576: f64 = (1.0 - locals.var_t0);
        let assign79440_body27_e121580: f64 = (1.0 - locals.var_t1);
        let assign79440_body27_e121581: f64 = (locals.var_phi_b_dpss * assign79440_body27_e121580);
        let assign79440_body27_e121582: f64 = (assign79440_body27_e121576 - assign79440_body27_e121581);
        let assign79440_body27_e121583: f64 = (locals.var_beta * assign79440_body27_e121582);
        (assign79440_body27_e121583, ((locals.var_beta_dn0 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk1771, locals.var_fbsq_dpss__blk1771_dn0, locals.var_fbsq_dpss__blk1771_dn2, locals.var_fbsq_dpss__blk1771_dn4, locals.var_fbsq_dpss__blk1771_dn5, locals.var_fbsq_dpss__blk1771_dn6, locals.var_fbsq_dpss__blk1771_dn7, locals.var_fbsq_dpss__blk1771_dn8, locals.var_fbsq_dpss__blk1771_dn9, locals.var_fbsq_dpss__blk1771_dn10, locals.var_fbsq_dpss__blk1771_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1771 = assign79440_body27_e121585;
            locals.var_fbsq_dpss__blk1771_dn0 = assign79440_body27_e121585_d_n0;
            locals.var_fbsq_dpss__blk1771_dn2 = assign79440_body27_e121585_d_n2;
            locals.var_fbsq_dpss__blk1771_dn4 = assign79440_body27_e121585_d_n4;
            locals.var_fbsq_dpss__blk1771_dn5 = assign79440_body27_e121585_d_n5;
            locals.var_fbsq_dpss__blk1771_dn6 = assign79440_body27_e121585_d_n6;
            locals.var_fbsq_dpss__blk1771_dn7 = assign79440_body27_e121585_d_n7;
            locals.var_fbsq_dpss__blk1771_dn8 = assign79440_body27_e121585_d_n8;
            locals.var_fbsq_dpss__blk1771_dn9 = assign79440_body27_e121585_d_n9;
            locals.var_fbsq_dpss__blk1771_dn10 = assign79440_body27_e121585_d_n10;
            locals.var_fbsq_dpss__blk1771_dn13 = assign79440_body27_e121585_d_n13;
            locals.var_fbsq_dpss__blk1771_rv = 0.0;
            let assign79440_body28_e121587: f64 = (locals.var_chi).abs();
            let assign79440_body28_e121589: f64 = if assign79440_body28_e121587 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1850 = assign79440_body28_e121589;
            locals.var_guard1850_rv = 0.0;
            let (assign79440_body29_e121619, assign79440_body29_e121619_d_n0, assign79440_body29_e121619_d_n2, assign79440_body29_e121619_d_n4, assign79440_body29_e121619_d_n5, assign79440_body29_e121619_d_n6, assign79440_body29_e121619_d_n7, assign79440_body29_e121619_d_n8, assign79440_body29_e121619_d_n9, assign79440_body29_e121619_d_n10, assign79440_body29_e121619_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body29_e121597: f64 = (locals.var_chi * locals.var_chi);
        let assign79440_body29_e121599: f64 = (assign79440_body29_e121597 / 2.0);
        let assign79440_body29_e121603: f64 = (locals.var_chi / 3.0);
        let assign79440_body29_e121607: f64 = (locals.var_chi / 4.0);
        let assign79440_body29_e121611: f64 = (locals.var_chi / 5.0);
        let assign79440_body29_e121612: f64 = (1.0 + assign79440_body29_e121611);
        let assign79440_body29_e121613: f64 = (assign79440_body29_e121607 * assign79440_body29_e121612);
        let assign79440_body29_e121614: f64 = (1.0 + assign79440_body29_e121613);
        let assign79440_body29_e121615: f64 = (assign79440_body29_e121603 * assign79440_body29_e121614);
        let assign79440_body29_e121616: f64 = (1.0 + assign79440_body29_e121615);
        let assign79440_body29_e121617: f64 = (assign79440_body29_e121599 * assign79440_body29_e121616);
        (assign79440_body29_e121617, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn0 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn0 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn2 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn2 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn4 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn4 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn5 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn5 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn6 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn6 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn7 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn7 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn8 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn8 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn9 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn9 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn10 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn10 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn13 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn13 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body29_e121619;
            locals.var_t0_dn0 = assign79440_body29_e121619_d_n0;
            locals.var_t0_dn2 = assign79440_body29_e121619_d_n2;
            locals.var_t0_dn4 = assign79440_body29_e121619_d_n4;
            locals.var_t0_dn5 = assign79440_body29_e121619_d_n5;
            locals.var_t0_dn6 = assign79440_body29_e121619_d_n6;
            locals.var_t0_dn7 = assign79440_body29_e121619_d_n7;
            locals.var_t0_dn8 = assign79440_body29_e121619_d_n8;
            locals.var_t0_dn9 = assign79440_body29_e121619_d_n9;
            locals.var_t0_dn10 = assign79440_body29_e121619_d_n10;
            locals.var_t0_dn13 = assign79440_body29_e121619_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign79440_body30_e121645, assign79440_body30_e121645_d_n0, assign79440_body30_e121645_d_n2, assign79440_body30_e121645_d_n4, assign79440_body30_e121645_d_n5, assign79440_body30_e121645_d_n6, assign79440_body30_e121645_d_n7, assign79440_body30_e121645_d_n8, assign79440_body30_e121645_d_n9, assign79440_body30_e121645_d_n10, assign79440_body30_e121645_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body30_e121629: f64 = (locals.var_chi / 2.0);
        let assign79440_body30_e121633: f64 = (locals.var_chi / 3.0);
        let assign79440_body30_e121637: f64 = (locals.var_chi / 4.0);
        let assign79440_body30_e121638: f64 = (1.0 + assign79440_body30_e121637);
        let assign79440_body30_e121639: f64 = (assign79440_body30_e121633 * assign79440_body30_e121638);
        let assign79440_body30_e121640: f64 = (1.0 + assign79440_body30_e121639);
        let assign79440_body30_e121641: f64 = (assign79440_body30_e121629 * assign79440_body30_e121640);
        let assign79440_body30_e121642: f64 = (1.0 + assign79440_body30_e121641);
        let assign79440_body30_e121643: f64 = (locals.var_chi * assign79440_body30_e121642);
        (assign79440_body30_e121643, ((locals.var_chi_dn0 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn0 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn2 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn4 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn5 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn6 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn7 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn8 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn9 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn10 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn13 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body30_e121645;
            locals.var_t1_dn0 = assign79440_body30_e121645_d_n0;
            locals.var_t1_dn2 = assign79440_body30_e121645_d_n2;
            locals.var_t1_dn4 = assign79440_body30_e121645_d_n4;
            locals.var_t1_dn5 = assign79440_body30_e121645_d_n5;
            locals.var_t1_dn6 = assign79440_body30_e121645_d_n6;
            locals.var_t1_dn7 = assign79440_body30_e121645_d_n7;
            locals.var_t1_dn8 = assign79440_body30_e121645_d_n8;
            locals.var_t1_dn9 = assign79440_body30_e121645_d_n9;
            locals.var_t1_dn10 = assign79440_body30_e121645_d_n10;
            locals.var_t1_dn13 = assign79440_body30_e121645_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79440_body31_e121655, assign79440_body31_e121655_d_n0, assign79440_body31_e121655_d_n2, assign79440_body31_e121655_d_n4, assign79440_body31_e121655_d_n5, assign79440_body31_e121655_d_n6, assign79440_body31_e121655_d_n7, assign79440_body31_e121655_d_n8, assign79440_body31_e121655_d_n9, assign79440_body31_e121655_d_n10, assign79440_body31_e121655_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body31_e121653: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign79440_body31_e121653, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79440_body31_e121655;
            locals.var_fs01_dn0 = assign79440_body31_e121655_d_n0;
            locals.var_fs01_dn2 = assign79440_body31_e121655_d_n2;
            locals.var_fs01_dn4 = assign79440_body31_e121655_d_n4;
            locals.var_fs01_dn5 = assign79440_body31_e121655_d_n5;
            locals.var_fs01_dn6 = assign79440_body31_e121655_d_n6;
            locals.var_fs01_dn7 = assign79440_body31_e121655_d_n7;
            locals.var_fs01_dn8 = assign79440_body31_e121655_d_n8;
            locals.var_fs01_dn9 = assign79440_body31_e121655_d_n9;
            locals.var_fs01_dn10 = assign79440_body31_e121655_d_n10;
            locals.var_fs01_dn13 = assign79440_body31_e121655_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign79440_body32_e121667, assign79440_body32_e121667_d_n0, assign79440_body32_e121667_d_n2, assign79440_body32_e121667_d_n4, assign79440_body32_e121667_d_n5, assign79440_body32_e121667_d_n6, assign79440_body32_e121667_d_n7, assign79440_body32_e121667_d_n8, assign79440_body32_e121667_d_n9, assign79440_body32_e121667_d_n10, assign79440_body32_e121667_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body32_e121663: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign79440_body32_e121665: f64 = (assign79440_body32_e121663 * locals.var_beta);
        (assign79440_body32_e121665, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79440_body32_e121667;
            locals.var_fs01_dps0_dn0 = assign79440_body32_e121667_d_n0;
            locals.var_fs01_dps0_dn2 = assign79440_body32_e121667_d_n2;
            locals.var_fs01_dps0_dn4 = assign79440_body32_e121667_d_n4;
            locals.var_fs01_dps0_dn5 = assign79440_body32_e121667_d_n5;
            locals.var_fs01_dps0_dn6 = assign79440_body32_e121667_d_n6;
            locals.var_fs01_dps0_dn7 = assign79440_body32_e121667_d_n7;
            locals.var_fs01_dps0_dn8 = assign79440_body32_e121667_d_n8;
            locals.var_fs01_dps0_dn9 = assign79440_body32_e121667_d_n9;
            locals.var_fs01_dps0_dn10 = assign79440_body32_e121667_d_n10;
            locals.var_fs01_dps0_dn13 = assign79440_body32_e121667_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign79440_body33_e121669: f64 = (locals.var_chi).abs();
            let assign79440_body33_e121671: f64 = if assign79440_body33_e121669 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1851 = assign79440_body33_e121671;
            locals.var_guard1851_rv = 0.0;
            let (assign79440_body35_e121702, assign79440_body35_e121702_d_n0, assign79440_body35_e121702_d_n2, assign79440_body35_e121702_d_n4, assign79440_body35_e121702_d_n5, assign79440_body35_e121702_d_n6, assign79440_body35_e121702_d_n7, assign79440_body35_e121702_d_n8, assign79440_body35_e121702_d_n9, assign79440_body35_e121702_d_n10, assign79440_body35_e121702_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body35_e121700: f64 = (locals.var_chi).exp();
        (assign79440_body35_e121700, (assign79440_body35_e121700 * locals.var_chi_dn0), (assign79440_body35_e121700 * locals.var_chi_dn2), (assign79440_body35_e121700 * locals.var_chi_dn4), (assign79440_body35_e121700 * locals.var_chi_dn5), (assign79440_body35_e121700 * locals.var_chi_dn6), (assign79440_body35_e121700 * locals.var_chi_dn7), (assign79440_body35_e121700 * locals.var_chi_dn8), (assign79440_body35_e121700 * locals.var_chi_dn9), (assign79440_body35_e121700 * locals.var_chi_dn10), (assign79440_body35_e121700 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign79440_body35_e121702;
            locals.var_exp_chi_dn0 = assign79440_body35_e121702_d_n0;
            locals.var_exp_chi_dn2 = assign79440_body35_e121702_d_n2;
            locals.var_exp_chi_dn4 = assign79440_body35_e121702_d_n4;
            locals.var_exp_chi_dn5 = assign79440_body35_e121702_d_n5;
            locals.var_exp_chi_dn6 = assign79440_body35_e121702_d_n6;
            locals.var_exp_chi_dn7 = assign79440_body35_e121702_d_n7;
            locals.var_exp_chi_dn8 = assign79440_body35_e121702_d_n8;
            locals.var_exp_chi_dn9 = assign79440_body35_e121702_d_n9;
            locals.var_exp_chi_dn10 = assign79440_body35_e121702_d_n10;
            locals.var_exp_chi_dn13 = assign79440_body35_e121702_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign79440_body36_e121715, assign79440_body36_e121715_d_n0, assign79440_body36_e121715_d_n2, assign79440_body36_e121715_d_n4, assign79440_body36_e121715_d_n5, assign79440_body36_e121715_d_n6, assign79440_body36_e121715_d_n7, assign79440_body36_e121715_d_n8, assign79440_body36_e121715_d_n9, assign79440_body36_e121715_d_n10, assign79440_body36_e121715_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body36_e121713: f64 = (locals.var_exp_chi - 1.0);
        (assign79440_body36_e121713, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body36_e121715;
            locals.var_t1_dn0 = assign79440_body36_e121715_d_n0;
            locals.var_t1_dn2 = assign79440_body36_e121715_d_n2;
            locals.var_t1_dn4 = assign79440_body36_e121715_d_n4;
            locals.var_t1_dn5 = assign79440_body36_e121715_d_n5;
            locals.var_t1_dn6 = assign79440_body36_e121715_d_n6;
            locals.var_t1_dn7 = assign79440_body36_e121715_d_n7;
            locals.var_t1_dn8 = assign79440_body36_e121715_d_n8;
            locals.var_t1_dn9 = assign79440_body36_e121715_d_n9;
            locals.var_t1_dn10 = assign79440_body36_e121715_d_n10;
            locals.var_t1_dn13 = assign79440_body36_e121715_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign79440_body37_e121730, assign79440_body37_e121730_d_n0, assign79440_body37_e121730_d_n2, assign79440_body37_e121730_d_n4, assign79440_body37_e121730_d_n5, assign79440_body37_e121730_d_n6, assign79440_body37_e121730_d_n7, assign79440_body37_e121730_d_n8, assign79440_body37_e121730_d_n9, assign79440_body37_e121730_d_n10, assign79440_body37_e121730_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body37_e121727: f64 = (locals.var_t1 - locals.var_chi);
        let assign79440_body37_e121728: f64 = (locals.var_cfs1 * assign79440_body37_e121727);
        (assign79440_body37_e121728, ((locals.var_cfs1_dn0 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79440_body37_e121730;
            locals.var_fs01_dn0 = assign79440_body37_e121730_d_n0;
            locals.var_fs01_dn2 = assign79440_body37_e121730_d_n2;
            locals.var_fs01_dn4 = assign79440_body37_e121730_d_n4;
            locals.var_fs01_dn5 = assign79440_body37_e121730_d_n5;
            locals.var_fs01_dn6 = assign79440_body37_e121730_d_n6;
            locals.var_fs01_dn7 = assign79440_body37_e121730_d_n7;
            locals.var_fs01_dn8 = assign79440_body37_e121730_d_n8;
            locals.var_fs01_dn9 = assign79440_body37_e121730_d_n9;
            locals.var_fs01_dn10 = assign79440_body37_e121730_d_n10;
            locals.var_fs01_dn13 = assign79440_body37_e121730_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign79440_body38_e121745, assign79440_body38_e121745_d_n0, assign79440_body38_e121745_d_n2, assign79440_body38_e121745_d_n4, assign79440_body38_e121745_d_n5, assign79440_body38_e121745_d_n6, assign79440_body38_e121745_d_n7, assign79440_body38_e121745_d_n8, assign79440_body38_e121745_d_n9, assign79440_body38_e121745_d_n10, assign79440_body38_e121745_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body38_e121741: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign79440_body38_e121743: f64 = (assign79440_body38_e121741 * locals.var_t1);
        (assign79440_body38_e121743, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79440_body38_e121745;
            locals.var_fs01_dps0_dn0 = assign79440_body38_e121745_d_n0;
            locals.var_fs01_dps0_dn2 = assign79440_body38_e121745_d_n2;
            locals.var_fs01_dps0_dn4 = assign79440_body38_e121745_d_n4;
            locals.var_fs01_dps0_dn5 = assign79440_body38_e121745_d_n5;
            locals.var_fs01_dps0_dn6 = assign79440_body38_e121745_d_n6;
            locals.var_fs01_dps0_dn7 = assign79440_body38_e121745_d_n7;
            locals.var_fs01_dps0_dn8 = assign79440_body38_e121745_d_n8;
            locals.var_fs01_dps0_dn9 = assign79440_body38_e121745_d_n9;
            locals.var_fs01_dps0_dn10 = assign79440_body38_e121745_d_n10;
            locals.var_fs01_dps0_dn13 = assign79440_body38_e121745_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign79440_body40_e121780, assign79440_body40_e121780_d_n0, assign79440_body40_e121780_d_n2, assign79440_body40_e121780_d_n4, assign79440_body40_e121780_d_n5, assign79440_body40_e121780_d_n6, assign79440_body40_e121780_d_n7, assign79440_body40_e121780_d_n8, assign79440_body40_e121780_d_n9, assign79440_body40_e121780_d_n10, assign79440_body40_e121780_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 == 0.0)) {
        let assign79440_body40_e121777: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign79440_body40_e121778: f64 = (assign79440_body40_e121777).exp();
        (assign79440_body40_e121778, (assign79440_body40_e121778 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign79440_body40_e121778 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign79440_body40_e121778 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign79440_body40_e121778 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign79440_body40_e121778 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign79440_body40_e121778 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign79440_body40_e121778 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign79440_body40_e121778 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign79440_body40_e121778 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign79440_body40_e121778 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign79440_body40_e121780;
            locals.var_exp_bps0_dn0 = assign79440_body40_e121780_d_n0;
            locals.var_exp_bps0_dn2 = assign79440_body40_e121780_d_n2;
            locals.var_exp_bps0_dn4 = assign79440_body40_e121780_d_n4;
            locals.var_exp_bps0_dn5 = assign79440_body40_e121780_d_n5;
            locals.var_exp_bps0_dn6 = assign79440_body40_e121780_d_n6;
            locals.var_exp_bps0_dn7 = assign79440_body40_e121780_d_n7;
            locals.var_exp_bps0_dn8 = assign79440_body40_e121780_d_n8;
            locals.var_exp_bps0_dn9 = assign79440_body40_e121780_d_n9;
            locals.var_exp_bps0_dn10 = assign79440_body40_e121780_d_n10;
            locals.var_exp_bps0_dn13 = assign79440_body40_e121780_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign79440_body41_e121800, assign79440_body41_e121800_d_n0, assign79440_body41_e121800_d_n2, assign79440_body41_e121800_d_n4, assign79440_body41_e121800_d_n5, assign79440_body41_e121800_d_n6, assign79440_body41_e121800_d_n7, assign79440_body41_e121800_d_n8, assign79440_body41_e121800_d_n9, assign79440_body41_e121800_d_n10, assign79440_body41_e121800_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 == 0.0)) {
        let assign79440_body41_e121795: f64 = (locals.var_chi + 1.0);
        let assign79440_body41_e121796: f64 = (locals.var_exp_bvbs * assign79440_body41_e121795);
        let assign79440_body41_e121797: f64 = (locals.var_exp_bps0 - assign79440_body41_e121796);
        let assign79440_body41_e121798: f64 = (locals.var_cnst1over * assign79440_body41_e121797);
        (assign79440_body41_e121798, ((locals.var_cnst1over_dn0 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79440_body41_e121800;
            locals.var_fs01_dn0 = assign79440_body41_e121800_d_n0;
            locals.var_fs01_dn2 = assign79440_body41_e121800_d_n2;
            locals.var_fs01_dn4 = assign79440_body41_e121800_d_n4;
            locals.var_fs01_dn5 = assign79440_body41_e121800_d_n5;
            locals.var_fs01_dn6 = assign79440_body41_e121800_d_n6;
            locals.var_fs01_dn7 = assign79440_body41_e121800_d_n7;
            locals.var_fs01_dn8 = assign79440_body41_e121800_d_n8;
            locals.var_fs01_dn9 = assign79440_body41_e121800_d_n9;
            locals.var_fs01_dn10 = assign79440_body41_e121800_d_n10;
            locals.var_fs01_dn13 = assign79440_body41_e121800_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign79440_body42_e121818, assign79440_body42_e121818_d_n0, assign79440_body42_e121818_d_n2, assign79440_body42_e121818_d_n4, assign79440_body42_e121818_d_n5, assign79440_body42_e121818_d_n6, assign79440_body42_e121818_d_n7, assign79440_body42_e121818_d_n8, assign79440_body42_e121818_d_n9, assign79440_body42_e121818_d_n10, assign79440_body42_e121818_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 == 0.0)) {
        let assign79440_body42_e121812: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign79440_body42_e121815: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign79440_body42_e121816: f64 = (assign79440_body42_e121812 * assign79440_body42_e121815);
        (assign79440_body42_e121816, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79440_body42_e121818;
            locals.var_fs01_dps0_dn0 = assign79440_body42_e121818_d_n0;
            locals.var_fs01_dps0_dn2 = assign79440_body42_e121818_d_n2;
            locals.var_fs01_dps0_dn4 = assign79440_body42_e121818_d_n4;
            locals.var_fs01_dps0_dn5 = assign79440_body42_e121818_d_n5;
            locals.var_fs01_dps0_dn6 = assign79440_body42_e121818_d_n6;
            locals.var_fs01_dps0_dn7 = assign79440_body42_e121818_d_n7;
            locals.var_fs01_dps0_dn8 = assign79440_body42_e121818_d_n8;
            locals.var_fs01_dps0_dn9 = assign79440_body42_e121818_d_n9;
            locals.var_fs01_dps0_dn10 = assign79440_body42_e121818_d_n10;
            locals.var_fs01_dps0_dn13 = assign79440_body42_e121818_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign79440_body43_e121821: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1852 = assign79440_body43_e121821;
            locals.var_guard1852_rv = 0.0;
            let (assign79440_body44_e121832, assign79440_body44_e121832_d_n0, assign79440_body44_e121832_d_n2, assign79440_body44_e121832_d_n4, assign79440_body44_e121832_d_n5, assign79440_body44_e121832_d_n6, assign79440_body44_e121832_d_n7, assign79440_body44_e121832_d_n8, assign79440_body44_e121832_d_n9, assign79440_body44_e121832_d_n10, assign79440_body44_e121832_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 != 0.0)) {
        let assign79440_body44_e121829: f64 = (locals.var_fbsq__blk1770 + locals.var_fs01);
        let assign79440_body44_e121830: f64 = (assign79440_body44_e121829).sqrt();
        (assign79440_body44_e121830, ((locals.var_fbsq__blk1770_dn0 + locals.var_fs01_dn0) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn2 + locals.var_fs01_dn2) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn4 + locals.var_fs01_dn4) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn5 + locals.var_fs01_dn5) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn6 + locals.var_fs01_dn6) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn7 + locals.var_fs01_dn7) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn8 + locals.var_fs01_dn8) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn9 + locals.var_fs01_dn9) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn10 + locals.var_fs01_dn10) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn13 + locals.var_fs01_dn13) / (2.0 * assign79440_body44_e121830)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body44_e121832;
            locals.var_fs02_dn0 = assign79440_body44_e121832_d_n0;
            locals.var_fs02_dn2 = assign79440_body44_e121832_d_n2;
            locals.var_fs02_dn4 = assign79440_body44_e121832_d_n4;
            locals.var_fs02_dn5 = assign79440_body44_e121832_d_n5;
            locals.var_fs02_dn6 = assign79440_body44_e121832_d_n6;
            locals.var_fs02_dn7 = assign79440_body44_e121832_d_n7;
            locals.var_fs02_dn8 = assign79440_body44_e121832_d_n8;
            locals.var_fs02_dn9 = assign79440_body44_e121832_d_n9;
            locals.var_fs02_dn10 = assign79440_body44_e121832_d_n10;
            locals.var_fs02_dn13 = assign79440_body44_e121832_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign79440_body45_e121846, assign79440_body45_e121846_d_n0, assign79440_body45_e121846_d_n2, assign79440_body45_e121846_d_n4, assign79440_body45_e121846_d_n5, assign79440_body45_e121846_d_n6, assign79440_body45_e121846_d_n7, assign79440_body45_e121846_d_n8, assign79440_body45_e121846_d_n9, assign79440_body45_e121846_d_n10, assign79440_body45_e121846_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 != 0.0)) {
        let assign79440_body45_e121841: f64 = (locals.var_fbsq_dpss__blk1771 + locals.var_fs01_dps0);
        let assign79440_body45_e121842: f64 = (0.5 * assign79440_body45_e121841);
        let assign79440_body45_e121844: f64 = (assign79440_body45_e121842 / locals.var_fs02);
        (assign79440_body45_e121844, ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body45_e121846;
            locals.var_fs02_dps0_dn0 = assign79440_body45_e121846_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body45_e121846_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body45_e121846_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body45_e121846_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body45_e121846_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body45_e121846_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body45_e121846_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body45_e121846_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body45_e121846_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body45_e121846_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign79440_body46_e121849: f64 = if locals.var_fbsq__blk1770 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1853 = assign79440_body46_e121849;
            locals.var_guard1853_rv = 0.0;
            let (assign79440_body47_e121861, assign79440_body47_e121861_d_n0, assign79440_body47_e121861_d_n2, assign79440_body47_e121861_d_n4, assign79440_body47_e121861_d_n5, assign79440_body47_e121861_d_n6, assign79440_body47_e121861_d_n7, assign79440_body47_e121861_d_n8, assign79440_body47_e121861_d_n9, assign79440_body47_e121861_d_n10, assign79440_body47_e121861_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 != 0.0)) {
        let assign79440_body47_e121859: f64 = (locals.var_fbsq__blk1770).sqrt();
        (assign79440_body47_e121859, (locals.var_fbsq__blk1770_dn0 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn2 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn4 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn5 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn6 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn7 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn8 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn9 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn10 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn13 / (2.0 * assign79440_body47_e121859)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body47_e121861;
            locals.var_fs02_dn0 = assign79440_body47_e121861_d_n0;
            locals.var_fs02_dn2 = assign79440_body47_e121861_d_n2;
            locals.var_fs02_dn4 = assign79440_body47_e121861_d_n4;
            locals.var_fs02_dn5 = assign79440_body47_e121861_d_n5;
            locals.var_fs02_dn6 = assign79440_body47_e121861_d_n6;
            locals.var_fs02_dn7 = assign79440_body47_e121861_d_n7;
            locals.var_fs02_dn8 = assign79440_body47_e121861_d_n8;
            locals.var_fs02_dn9 = assign79440_body47_e121861_d_n9;
            locals.var_fs02_dn10 = assign79440_body47_e121861_d_n10;
            locals.var_fs02_dn13 = assign79440_body47_e121861_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign79440_body48_e121876, assign79440_body48_e121876_d_n0, assign79440_body48_e121876_d_n2, assign79440_body48_e121876_d_n4, assign79440_body48_e121876_d_n5, assign79440_body48_e121876_d_n6, assign79440_body48_e121876_d_n7, assign79440_body48_e121876_d_n8, assign79440_body48_e121876_d_n9, assign79440_body48_e121876_d_n10, assign79440_body48_e121876_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 != 0.0)) {
        let assign79440_body48_e121872: f64 = (0.5 * locals.var_fbsq_dpss__blk1771);
        let assign79440_body48_e121874: f64 = (assign79440_body48_e121872 / locals.var_fs02);
        (assign79440_body48_e121874, ((((0.5 * locals.var_fbsq_dpss__blk1771_dn0) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn2) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn4) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn5) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn6) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn7) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn8) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn9) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn10) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn13) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body48_e121876;
            locals.var_fs02_dps0_dn0 = assign79440_body48_e121876_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body48_e121876_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body48_e121876_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body48_e121876_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body48_e121876_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body48_e121876_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body48_e121876_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body48_e121876_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body48_e121876_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body48_e121876_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign79440_body49_e121888, assign79440_body49_e121888_d_n0, assign79440_body49_e121888_d_n2, assign79440_body49_e121888_d_n4, assign79440_body49_e121888_d_n5, assign79440_body49_e121888_d_n6, assign79440_body49_e121888_d_n7, assign79440_body49_e121888_d_n8, assign79440_body49_e121888_d_n9, assign79440_body49_e121888_d_n10, assign79440_body49_e121888_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body49_e121888;
            locals.var_fs02_dn0 = assign79440_body49_e121888_d_n0;
            locals.var_fs02_dn2 = assign79440_body49_e121888_d_n2;
            locals.var_fs02_dn4 = assign79440_body49_e121888_d_n4;
            locals.var_fs02_dn5 = assign79440_body49_e121888_d_n5;
            locals.var_fs02_dn6 = assign79440_body49_e121888_d_n6;
            locals.var_fs02_dn7 = assign79440_body49_e121888_d_n7;
            locals.var_fs02_dn8 = assign79440_body49_e121888_d_n8;
            locals.var_fs02_dn9 = assign79440_body49_e121888_d_n9;
            locals.var_fs02_dn10 = assign79440_body49_e121888_d_n10;
            locals.var_fs02_dn13 = assign79440_body49_e121888_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign79440_body50_e121900, assign79440_body50_e121900_d_n0, assign79440_body50_e121900_d_n2, assign79440_body50_e121900_d_n4, assign79440_body50_e121900_d_n5, assign79440_body50_e121900_d_n6, assign79440_body50_e121900_d_n7, assign79440_body50_e121900_d_n8, assign79440_body50_e121900_d_n9, assign79440_body50_e121900_d_n10, assign79440_body50_e121900_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body50_e121900;
            locals.var_fs02_dps0_dn0 = assign79440_body50_e121900_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body50_e121900_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body50_e121900_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body50_e121900_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body50_e121900_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body50_e121900_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body50_e121900_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body50_e121900_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body50_e121900_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body50_e121900_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign79440_body51_e121914, assign79440_body51_e121914_d_n0, assign79440_body51_e121914_d_n2, assign79440_body51_e121914_d_n4, assign79440_body51_e121914_d_n5, assign79440_body51_e121914_d_n6, assign79440_body51_e121914_d_n7, assign79440_body51_e121914_d_n8, assign79440_body51_e121914_d_n9, assign79440_body51_e121914_d_n10, assign79440_body51_e121914_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let (assign79440_body51_e121910,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign79440_body51_e121909: f64 = (-1.0);
                (assign79440_body51_e121909,)
            }
        };
        let assign79440_body51_e121912: f64 = (assign79440_body51_e121910 * locals.var_fs02);
        (assign79440_body51_e121912, (assign79440_body51_e121910 * locals.var_fs02_dn0), (assign79440_body51_e121910 * locals.var_fs02_dn2), (assign79440_body51_e121910 * locals.var_fs02_dn4), (assign79440_body51_e121910 * locals.var_fs02_dn5), (assign79440_body51_e121910 * locals.var_fs02_dn6), (assign79440_body51_e121910 * locals.var_fs02_dn7), (assign79440_body51_e121910 * locals.var_fs02_dn8), (assign79440_body51_e121910 * locals.var_fs02_dn9), (assign79440_body51_e121910 * locals.var_fs02_dn10), (assign79440_body51_e121910 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body51_e121914;
            locals.var_fs02_dn0 = assign79440_body51_e121914_d_n0;
            locals.var_fs02_dn2 = assign79440_body51_e121914_d_n2;
            locals.var_fs02_dn4 = assign79440_body51_e121914_d_n4;
            locals.var_fs02_dn5 = assign79440_body51_e121914_d_n5;
            locals.var_fs02_dn6 = assign79440_body51_e121914_d_n6;
            locals.var_fs02_dn7 = assign79440_body51_e121914_d_n7;
            locals.var_fs02_dn8 = assign79440_body51_e121914_d_n8;
            locals.var_fs02_dn9 = assign79440_body51_e121914_d_n9;
            locals.var_fs02_dn10 = assign79440_body51_e121914_d_n10;
            locals.var_fs02_dn13 = assign79440_body51_e121914_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign79440_body52_e121928, assign79440_body52_e121928_d_n0, assign79440_body52_e121928_d_n2, assign79440_body52_e121928_d_n4, assign79440_body52_e121928_d_n5, assign79440_body52_e121928_d_n6, assign79440_body52_e121928_d_n7, assign79440_body52_e121928_d_n8, assign79440_body52_e121928_d_n9, assign79440_body52_e121928_d_n10, assign79440_body52_e121928_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let (assign79440_body52_e121924,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign79440_body52_e121923: f64 = (-1.0);
                (assign79440_body52_e121923,)
            }
        };
        let assign79440_body52_e121926: f64 = (assign79440_body52_e121924 * locals.var_fs02_dps0);
        (assign79440_body52_e121926, (assign79440_body52_e121924 * locals.var_fs02_dps0_dn0), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn2), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn4), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn5), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn6), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn7), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn8), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn9), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn10), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body52_e121928;
            locals.var_fs02_dps0_dn0 = assign79440_body52_e121928_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body52_e121928_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body52_e121928_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body52_e121928_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body52_e121928_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body52_e121928_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body52_e121928_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body52_e121928_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body52_e121928_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body52_e121928_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign79440_body53_e121941, assign79440_body53_e121941_d_n0, assign79440_body53_e121941_d_n2, assign79440_body53_e121941_d_n4, assign79440_body53_e121941_d_n5, assign79440_body53_e121941_d_n6, assign79440_body53_e121941_d_n7, assign79440_body53_e121941_d_n8, assign79440_body53_e121941_d_n9, assign79440_body53_e121941_d_n10, assign79440_body53_e121941_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body53_e121933: f64 = (-locals.var_vgpld);
        let assign79440_body53_e121935: f64 = (assign79440_body53_e121933 + locals.var_ps0ld);
        let assign79440_body53_e121938: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign79440_body53_e121939: f64 = (assign79440_body53_e121935 + assign79440_body53_e121938);
        (assign79440_body53_e121939, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign79440_body53_e121941;
            locals.var_fs0_dn0 = assign79440_body53_e121941_d_n0;
            locals.var_fs0_dn2 = assign79440_body53_e121941_d_n2;
            locals.var_fs0_dn4 = assign79440_body53_e121941_d_n4;
            locals.var_fs0_dn5 = assign79440_body53_e121941_d_n5;
            locals.var_fs0_dn6 = assign79440_body53_e121941_d_n6;
            locals.var_fs0_dn7 = assign79440_body53_e121941_d_n7;
            locals.var_fs0_dn8 = assign79440_body53_e121941_d_n8;
            locals.var_fs0_dn9 = assign79440_body53_e121941_d_n9;
            locals.var_fs0_dn10 = assign79440_body53_e121941_d_n10;
            locals.var_fs0_dn13 = assign79440_body53_e121941_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign79440_body54_e121951, assign79440_body54_e121951_d_n0, assign79440_body54_e121951_d_n2, assign79440_body54_e121951_d_n4, assign79440_body54_e121951_d_n5, assign79440_body54_e121951_d_n6, assign79440_body54_e121951_d_n7, assign79440_body54_e121951_d_n8, assign79440_body54_e121951_d_n9, assign79440_body54_e121951_d_n10, assign79440_body54_e121951_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body54_e121948: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign79440_body54_e121949: f64 = (1.0 + assign79440_body54_e121948);
        (assign79440_body54_e121949, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign79440_body54_e121951;
            locals.var_fs0_dps0_dn0 = assign79440_body54_e121951_d_n0;
            locals.var_fs0_dps0_dn2 = assign79440_body54_e121951_d_n2;
            locals.var_fs0_dps0_dn4 = assign79440_body54_e121951_d_n4;
            locals.var_fs0_dps0_dn5 = assign79440_body54_e121951_d_n5;
            locals.var_fs0_dps0_dn6 = assign79440_body54_e121951_d_n6;
            locals.var_fs0_dps0_dn7 = assign79440_body54_e121951_d_n7;
            locals.var_fs0_dps0_dn8 = assign79440_body54_e121951_d_n8;
            locals.var_fs0_dps0_dn9 = assign79440_body54_e121951_d_n9;
            locals.var_fs0_dps0_dn10 = assign79440_body54_e121951_d_n10;
            locals.var_fs0_dps0_dn13 = assign79440_body54_e121951_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign79440_body55_e121954: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1854 = assign79440_body55_e121954;
            locals.var_guard1854_rv = 0.0;
            let (assign79440_body56_e121964,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 != 0.0)) {
        let assign79440_body56_e121962: f64 = (locals.var_lp_s0_max + 1.0);
        (assign79440_body56_e121962,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79440_body56_e121964;
            locals.var_lp_s0_rv = 0.0;
            let (assign79440_body57_e121976, assign79440_body57_e121976_d_n0, assign79440_body57_e121976_d_n2, assign79440_body57_e121976_d_n4, assign79440_body57_e121976_d_n5, assign79440_body57_e121976_d_n6, assign79440_body57_e121976_d_n7, assign79440_body57_e121976_d_n8, assign79440_body57_e121976_d_n9, assign79440_body57_e121976_d_n10, assign79440_body57_e121976_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) {
        let assign79440_body57_e121972: f64 = (-locals.var_fs0);
        let assign79440_body57_e121974: f64 = (assign79440_body57_e121972 / locals.var_fs0_dps0);
        (assign79440_body57_e121974, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79440_body57_e121976;
            locals.var_dps0_dn0 = assign79440_body57_e121976_d_n0;
            locals.var_dps0_dn2 = assign79440_body57_e121976_d_n2;
            locals.var_dps0_dn4 = assign79440_body57_e121976_d_n4;
            locals.var_dps0_dn5 = assign79440_body57_e121976_d_n5;
            locals.var_dps0_dn6 = assign79440_body57_e121976_d_n6;
            locals.var_dps0_dn7 = assign79440_body57_e121976_d_n7;
            locals.var_dps0_dn8 = assign79440_body57_e121976_d_n8;
            locals.var_dps0_dn9 = assign79440_body57_e121976_d_n9;
            locals.var_dps0_dn10 = assign79440_body57_e121976_d_n10;
            locals.var_dps0_dn13 = assign79440_body57_e121976_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign79440_body58_e121998, assign79440_body58_e121998_d_n0, assign79440_body58_e121998_d_n2, assign79440_body58_e121998_d_n4, assign79440_body58_e121998_d_n5, assign79440_body58_e121998_d_n6, assign79440_body58_e121998_d_n7, assign79440_body58_e121998_d_n8, assign79440_body58_e121998_d_n9, assign79440_body58_e121998_d_n10, assign79440_body58_e121998_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) {
        let assign79440_body58_e121985: f64 = (0.5 * 0.1);
        let assign79440_body58_e121989: f64 = (locals.var_ps0ld).abs();
        let (assign79440_body58_e121994, assign79440_body58_e121994_d_n0, assign79440_body58_e121994_d_n2, assign79440_body58_e121994_d_n4, assign79440_body58_e121994_d_n5, assign79440_body58_e121994_d_n6, assign79440_body58_e121994_d_n7, assign79440_body58_e121994_d_n8, assign79440_body58_e121994_d_n9, assign79440_body58_e121994_d_n10, assign79440_body58_e121994_d_n13,) = {
            if (1.0 >= assign79440_body58_e121989) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign79440_body58_e121993: f64 = (locals.var_ps0ld).abs();
                (assign79440_body58_e121993, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign79440_body58_e121995: f64 = (1.0 + assign79440_body58_e121994);
        let assign79440_body58_e121996: f64 = (assign79440_body58_e121985 * assign79440_body58_e121995);
        (assign79440_body58_e121996, (assign79440_body58_e121985 * assign79440_body58_e121994_d_n0), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n2), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n4), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n5), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n6), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n7), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n8), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n9), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n10), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign79440_body58_e121998;
            locals.var_dplim_dn0 = assign79440_body58_e121998_d_n0;
            locals.var_dplim_dn2 = assign79440_body58_e121998_d_n2;
            locals.var_dplim_dn4 = assign79440_body58_e121998_d_n4;
            locals.var_dplim_dn5 = assign79440_body58_e121998_d_n5;
            locals.var_dplim_dn6 = assign79440_body58_e121998_d_n6;
            locals.var_dplim_dn7 = assign79440_body58_e121998_d_n7;
            locals.var_dplim_dn8 = assign79440_body58_e121998_d_n8;
            locals.var_dplim_dn9 = assign79440_body58_e121998_d_n9;
            locals.var_dplim_dn10 = assign79440_body58_e121998_d_n10;
            locals.var_dplim_dn13 = assign79440_body58_e121998_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign79440_body59_e122000: f64 = (locals.var_dps0).abs();
            let assign79440_body59_e122002: f64 = if assign79440_body59_e122000 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1855 = assign79440_body59_e122002;
            locals.var_guard1855_rv = 0.0;
            let (assign79440_body60_e122021, assign79440_body60_e122021_d_n0, assign79440_body60_e122021_d_n2, assign79440_body60_e122021_d_n4, assign79440_body60_e122021_d_n5, assign79440_body60_e122021_d_n6, assign79440_body60_e122021_d_n7, assign79440_body60_e122021_d_n8, assign79440_body60_e122021_d_n9, assign79440_body60_e122021_d_n10, assign79440_body60_e122021_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) && (locals.var_guard1855 != 0.0)) {
        let (assign79440_body60_e122018,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign79440_body60_e122017: f64 = (-1.0);
                (assign79440_body60_e122017,)
            }
        };
        let assign79440_body60_e122019: f64 = (locals.var_dplim * assign79440_body60_e122018);
        (assign79440_body60_e122019, (locals.var_dplim_dn0 * assign79440_body60_e122018), (locals.var_dplim_dn2 * assign79440_body60_e122018), (locals.var_dplim_dn4 * assign79440_body60_e122018), (locals.var_dplim_dn5 * assign79440_body60_e122018), (locals.var_dplim_dn6 * assign79440_body60_e122018), (locals.var_dplim_dn7 * assign79440_body60_e122018), (locals.var_dplim_dn8 * assign79440_body60_e122018), (locals.var_dplim_dn9 * assign79440_body60_e122018), (locals.var_dplim_dn10 * assign79440_body60_e122018), (locals.var_dplim_dn13 * assign79440_body60_e122018),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79440_body60_e122021;
            locals.var_dps0_dn0 = assign79440_body60_e122021_d_n0;
            locals.var_dps0_dn2 = assign79440_body60_e122021_d_n2;
            locals.var_dps0_dn4 = assign79440_body60_e122021_d_n4;
            locals.var_dps0_dn5 = assign79440_body60_e122021_d_n5;
            locals.var_dps0_dn6 = assign79440_body60_e122021_d_n6;
            locals.var_dps0_dn7 = assign79440_body60_e122021_d_n7;
            locals.var_dps0_dn8 = assign79440_body60_e122021_d_n8;
            locals.var_dps0_dn9 = assign79440_body60_e122021_d_n9;
            locals.var_dps0_dn10 = assign79440_body60_e122021_d_n10;
            locals.var_dps0_dn13 = assign79440_body60_e122021_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign79440_body61_e122032, assign79440_body61_e122032_d_n0, assign79440_body61_e122032_d_n2, assign79440_body61_e122032_d_n4, assign79440_body61_e122032_d_n5, assign79440_body61_e122032_d_n6, assign79440_body61_e122032_d_n7, assign79440_body61_e122032_d_n8, assign79440_body61_e122032_d_n9, assign79440_body61_e122032_d_n10, assign79440_body61_e122032_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) {
        let assign79440_body61_e122030: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign79440_body61_e122030, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign79440_body61_e122032;
            locals.var_ps0ld_dn0 = assign79440_body61_e122032_d_n0;
            locals.var_ps0ld_dn2 = assign79440_body61_e122032_d_n2;
            locals.var_ps0ld_dn4 = assign79440_body61_e122032_d_n4;
            locals.var_ps0ld_dn5 = assign79440_body61_e122032_d_n5;
            locals.var_ps0ld_dn6 = assign79440_body61_e122032_d_n6;
            locals.var_ps0ld_dn7 = assign79440_body61_e122032_d_n7;
            locals.var_ps0ld_dn8 = assign79440_body61_e122032_d_n8;
            locals.var_ps0ld_dn9 = assign79440_body61_e122032_d_n9;
            locals.var_ps0ld_dn10 = assign79440_body61_e122032_d_n10;
            locals.var_ps0ld_dn13 = assign79440_body61_e122032_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign79440_body62_e122034: f64 = (locals.var_dps0).abs();
            let assign79440_body62_e122038: f64 = (locals.var_fs0).abs();
            let assign79440_body62_e122041: f64 = if ((assign79440_body62_e122034 <= 1e-12) && (assign79440_body62_e122038 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1856 = assign79440_body62_e122041;
            locals.var_guard1856_rv = 0.0;
            let (assign79440_body63_e122054,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) && (locals.var_guard1856 != 0.0)) {
        let assign79440_body63_e122052: f64 = (locals.var_flg_conv + 2.0);
        (assign79440_body63_e122052,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign79440_body63_e122054;
            locals.var_flg_conv_rv = 0.0;
            let (assign79440_body64_e122062,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body64_e122060: f64 = (locals.var_lp_s0 + 1.0);
        (assign79440_body64_e122060,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79440_body64_e122062;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_292(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79460_e122085, assign79460_e122085_d_n0, assign79460_e122085_d_n2, assign79460_e122085_d_n4, assign79460_e122085_d_n5, assign79460_e122085_d_n6, assign79460_e122085_d_n7, assign79460_e122085_d_n8, assign79460_e122085_d_n9, assign79460_e122085_d_n10, assign79460_e122085_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let (assign79460_e122083, assign79460_e122083_d_n0, assign79460_e122083_d_n2, assign79460_e122083_d_n4, assign79460_e122083_d_n5, assign79460_e122083_d_n6, assign79460_e122083_d_n7, assign79460_e122083_d_n8, assign79460_e122083_d_n9, assign79460_e122083_d_n10, assign79460_e122083_d_n13,) = {
            if (locals.var_fbsq__blk1770 >= 0.0) {
                let (assign79460_e122078,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign79460_e122077: f64 = (-1.0);
                        (assign79460_e122077,)
                    }
                };
                let assign79460_e122080: f64 = (locals.var_fbsq__blk1770).sqrt();
                let assign79460_e122081: f64 = (assign79460_e122078 * assign79460_e122080);
                (assign79460_e122081, (assign79460_e122078 * (locals.var_fbsq__blk1770_dn0 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn2 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn4 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn5 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn6 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn7 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn8 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn9 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn10 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn13 / (2.0 * assign79460_e122080))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign79460_e122083, assign79460_e122083_d_n0, assign79460_e122083_d_n2, assign79460_e122083_d_n4, assign79460_e122083_d_n5, assign79460_e122083_d_n6, assign79460_e122083_d_n7, assign79460_e122083_d_n8, assign79460_e122083_d_n9, assign79460_e122083_d_n10, assign79460_e122083_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign79460_e122085;
        locals.var_fb_dn0 = assign79460_e122085_d_n0;
        locals.var_fb_dn2 = assign79460_e122085_d_n2;
        locals.var_fb_dn4 = assign79460_e122085_d_n4;
        locals.var_fb_dn5 = assign79460_e122085_d_n5;
        locals.var_fb_dn6 = assign79460_e122085_d_n6;
        locals.var_fb_dn7 = assign79460_e122085_d_n7;
        locals.var_fb_dn8 = assign79460_e122085_d_n8;
        locals.var_fb_dn9 = assign79460_e122085_d_n9;
        locals.var_fb_dn10 = assign79460_e122085_d_n10;
        locals.var_fb_dn13 = assign79460_e122085_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign79470_e122093, assign79470_e122093_d_n0, assign79470_e122093_d_n2, assign79470_e122093_d_n4, assign79470_e122093_d_n5, assign79470_e122093_d_n6, assign79470_e122093_d_n7, assign79470_e122093_d_n8, assign79470_e122093_d_n9, assign79470_e122093_d_n10, assign79470_e122093_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79470_e122091: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign79470_e122091, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1760, locals.var_wdld__blk1760_dn0, locals.var_wdld__blk1760_dn2, locals.var_wdld__blk1760_dn4, locals.var_wdld__blk1760_dn5, locals.var_wdld__blk1760_dn6, locals.var_wdld__blk1760_dn7, locals.var_wdld__blk1760_dn8, locals.var_wdld__blk1760_dn9, locals.var_wdld__blk1760_dn10, locals.var_wdld__blk1760_dn13,)
    }
};
        locals.var_wdld__blk1760 = assign79470_e122093;
        locals.var_wdld__blk1760_dn0 = assign79470_e122093_d_n0;
        locals.var_wdld__blk1760_dn2 = assign79470_e122093_d_n2;
        locals.var_wdld__blk1760_dn4 = assign79470_e122093_d_n4;
        locals.var_wdld__blk1760_dn5 = assign79470_e122093_d_n5;
        locals.var_wdld__blk1760_dn6 = assign79470_e122093_d_n6;
        locals.var_wdld__blk1760_dn7 = assign79470_e122093_d_n7;
        locals.var_wdld__blk1760_dn8 = assign79470_e122093_d_n8;
        locals.var_wdld__blk1760_dn9 = assign79470_e122093_d_n9;
        locals.var_wdld__blk1760_dn10 = assign79470_e122093_d_n10;
        locals.var_wdld__blk1760_dn13 = assign79470_e122093_d_n13;
        locals.var_wdld__blk1760_rv = 0.0;

        let (assign79480_e122101, assign79480_e122101_d_n0, assign79480_e122101_d_n2, assign79480_e122101_d_n4, assign79480_e122101_d_n5, assign79480_e122101_d_n6, assign79480_e122101_d_n7, assign79480_e122101_d_n8, assign79480_e122101_d_n9, assign79480_e122101_d_n10, assign79480_e122101_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79480_e122099: f64 = (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760);
        (assign79480_e122099, (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn0), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn2), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn4), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn5), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn6), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn7), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn8), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn9), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn10), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1761, locals.var_q_dep_ld__blk1761_dn0, locals.var_q_dep_ld__blk1761_dn2, locals.var_q_dep_ld__blk1761_dn4, locals.var_q_dep_ld__blk1761_dn5, locals.var_q_dep_ld__blk1761_dn6, locals.var_q_dep_ld__blk1761_dn7, locals.var_q_dep_ld__blk1761_dn8, locals.var_q_dep_ld__blk1761_dn9, locals.var_q_dep_ld__blk1761_dn10, locals.var_q_dep_ld__blk1761_dn13,)
    }
};
        locals.var_q_dep_ld__blk1761 = assign79480_e122101;
        locals.var_q_dep_ld__blk1761_dn0 = assign79480_e122101_d_n0;
        locals.var_q_dep_ld__blk1761_dn2 = assign79480_e122101_d_n2;
        locals.var_q_dep_ld__blk1761_dn4 = assign79480_e122101_d_n4;
        locals.var_q_dep_ld__blk1761_dn5 = assign79480_e122101_d_n5;
        locals.var_q_dep_ld__blk1761_dn6 = assign79480_e122101_d_n6;
        locals.var_q_dep_ld__blk1761_dn7 = assign79480_e122101_d_n7;
        locals.var_q_dep_ld__blk1761_dn8 = assign79480_e122101_d_n8;
        locals.var_q_dep_ld__blk1761_dn9 = assign79480_e122101_d_n9;
        locals.var_q_dep_ld__blk1761_dn10 = assign79480_e122101_d_n10;
        locals.var_q_dep_ld__blk1761_dn13 = assign79480_e122101_d_n13;
        locals.var_q_dep_ld__blk1761_rv = 0.0;

        let (assign79490_e122113, assign79490_e122113_d_n0, assign79490_e122113_d_n2, assign79490_e122113_d_n4, assign79490_e122113_d_n5, assign79490_e122113_d_n6, assign79490_e122113_d_n7, assign79490_e122113_d_n8, assign79490_e122113_d_n9, assign79490_e122113_d_n10, assign79490_e122113_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79490_e122107: f64 = (locals.var_q_dep_ld__blk1761 / locals.var_cnst0over_func);
        let assign79490_e122110: f64 = (10.0 * 2.220446049250313e-16);
        let assign79490_e122111: f64 = (assign79490_e122107 + assign79490_e122110);
        (assign79490_e122111, (((locals.var_q_dep_ld__blk1761_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign79490_e122113;
        locals.var_xi0p12_dn0 = assign79490_e122113_d_n0;
        locals.var_xi0p12_dn2 = assign79490_e122113_d_n2;
        locals.var_xi0p12_dn4 = assign79490_e122113_d_n4;
        locals.var_xi0p12_dn5 = assign79490_e122113_d_n5;
        locals.var_xi0p12_dn6 = assign79490_e122113_d_n6;
        locals.var_xi0p12_dn7 = assign79490_e122113_d_n7;
        locals.var_xi0p12_dn8 = assign79490_e122113_d_n8;
        locals.var_xi0p12_dn9 = assign79490_e122113_d_n9;
        locals.var_xi0p12_dn10 = assign79490_e122113_d_n10;
        locals.var_xi0p12_dn13 = assign79490_e122113_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign79500_e122121, assign79500_e122121_d_n0, assign79500_e122121_d_n2, assign79500_e122121_d_n4, assign79500_e122121_d_n5, assign79500_e122121_d_n6, assign79500_e122121_d_n7, assign79500_e122121_d_n8, assign79500_e122121_d_n9, assign79500_e122121_d_n10, assign79500_e122121_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79500_e122119: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign79500_e122119, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign79500_e122121;
        locals.var_qbuld_dn0 = assign79500_e122121_d_n0;
        locals.var_qbuld_dn2 = assign79500_e122121_d_n2;
        locals.var_qbuld_dn4 = assign79500_e122121_d_n4;
        locals.var_qbuld_dn5 = assign79500_e122121_d_n5;
        locals.var_qbuld_dn6 = assign79500_e122121_d_n6;
        locals.var_qbuld_dn7 = assign79500_e122121_d_n7;
        locals.var_qbuld_dn8 = assign79500_e122121_d_n8;
        locals.var_qbuld_dn9 = assign79500_e122121_d_n9;
        locals.var_qbuld_dn10 = assign79500_e122121_d_n10;
        locals.var_qbuld_dn13 = assign79500_e122121_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign79510_e122131, assign79510_e122131_d_n0, assign79510_e122131_d_n2, assign79510_e122131_d_n4, assign79510_e122131_d_n5, assign79510_e122131_d_n6, assign79510_e122131_d_n7, assign79510_e122131_d_n8, assign79510_e122131_d_n9, assign79510_e122131_d_n10, assign79510_e122131_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79510_e122128: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign79510_e122129: f64 = (1.0 / assign79510_e122128);
        (assign79510_e122129, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign79510_e122128 * assign79510_e122128))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79510_e122131;
        locals.var_t1_dn0 = assign79510_e122131_d_n0;
        locals.var_t1_dn2 = assign79510_e122131_d_n2;
        locals.var_t1_dn4 = assign79510_e122131_d_n4;
        locals.var_t1_dn5 = assign79510_e122131_d_n5;
        locals.var_t1_dn6 = assign79510_e122131_d_n6;
        locals.var_t1_dn7 = assign79510_e122131_d_n7;
        locals.var_t1_dn8 = assign79510_e122131_d_n8;
        locals.var_t1_dn9 = assign79510_e122131_d_n9;
        locals.var_t1_dn10 = assign79510_e122131_d_n10;
        locals.var_t1_dn13 = assign79510_e122131_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign79520_e122141, assign79520_e122141_d_n0, assign79520_e122141_d_n2, assign79520_e122141_d_n4, assign79520_e122141_d_n5, assign79520_e122141_d_n6, assign79520_e122141_d_n7, assign79520_e122141_d_n8, assign79520_e122141_d_n9, assign79520_e122141_d_n10, assign79520_e122141_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79520_e122137: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign79520_e122139: f64 = (assign79520_e122137 * locals.var_t1);
        (assign79520_e122139, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign79520_e122141;
        locals.var_qiuld_dn0 = assign79520_e122141_d_n0;
        locals.var_qiuld_dn2 = assign79520_e122141_d_n2;
        locals.var_qiuld_dn4 = assign79520_e122141_d_n4;
        locals.var_qiuld_dn5 = assign79520_e122141_d_n5;
        locals.var_qiuld_dn6 = assign79520_e122141_d_n6;
        locals.var_qiuld_dn7 = assign79520_e122141_d_n7;
        locals.var_qiuld_dn8 = assign79520_e122141_d_n8;
        locals.var_qiuld_dn9 = assign79520_e122141_d_n9;
        locals.var_qiuld_dn10 = assign79520_e122141_d_n10;
        locals.var_qiuld_dn13 = assign79520_e122141_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign79530_e122149, assign79530_e122149_d_n0, assign79530_e122149_d_n2, assign79530_e122149_d_n4, assign79530_e122149_d_n5, assign79530_e122149_d_n6, assign79530_e122149_d_n7, assign79530_e122149_d_n8, assign79530_e122149_d_n9, assign79530_e122149_d_n10, assign79530_e122149_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79530_e122147: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign79530_e122147, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign79530_e122149;
        locals.var_qsuld_dn0 = assign79530_e122149_d_n0;
        locals.var_qsuld_dn2 = assign79530_e122149_d_n2;
        locals.var_qsuld_dn4 = assign79530_e122149_d_n4;
        locals.var_qsuld_dn5 = assign79530_e122149_d_n5;
        locals.var_qsuld_dn6 = assign79530_e122149_d_n6;
        locals.var_qsuld_dn7 = assign79530_e122149_d_n7;
        locals.var_qsuld_dn8 = assign79530_e122149_d_n8;
        locals.var_qsuld_dn9 = assign79530_e122149_d_n9;
        locals.var_qsuld_dn10 = assign79530_e122149_d_n10;
        locals.var_qsuld_dn13 = assign79530_e122149_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign79540_e122155, assign79540_e122155_d_n0, assign79540_e122155_d_n2, assign79540_e122155_d_n4, assign79540_e122155_d_n5, assign79540_e122155_d_n6, assign79540_e122155_d_n7, assign79540_e122155_d_n8, assign79540_e122155_d_n9, assign79540_e122155_d_n10, assign79540_e122155_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign79540_e122153: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign79540_e122153, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign79540_e122155;
        locals.var_qiuld_dn0 = assign79540_e122155_d_n0;
        locals.var_qiuld_dn2 = assign79540_e122155_d_n2;
        locals.var_qiuld_dn4 = assign79540_e122155_d_n4;
        locals.var_qiuld_dn5 = assign79540_e122155_d_n5;
        locals.var_qiuld_dn6 = assign79540_e122155_d_n6;
        locals.var_qiuld_dn7 = assign79540_e122155_d_n7;
        locals.var_qiuld_dn8 = assign79540_e122155_d_n8;
        locals.var_qiuld_dn9 = assign79540_e122155_d_n9;
        locals.var_qiuld_dn10 = assign79540_e122155_d_n10;
        locals.var_qiuld_dn13 = assign79540_e122155_d_n13;
        locals.var_qiuld_rv = 0.0;

        let assign79550_e122158: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1858 = assign79550_e122158;
        locals.var_guard1858_rv = 0.0;

        let (assign79560_e122165, assign79560_e122165_d_n0, assign79560_e122165_d_n2, assign79560_e122165_d_n4, assign79560_e122165_d_n5, assign79560_e122165_d_n6, assign79560_e122165_d_n7, assign79560_e122165_d_n8, assign79560_e122165_d_n9, assign79560_e122165_d_n10, assign79560_e122165_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) {
        let assign79560_e122163: f64 = (-locals.var_lover_func);
        (assign79560_e122163, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign79560_e122165;
        locals.var_lover_func_dn0 = assign79560_e122165_d_n0;
        locals.var_lover_func_dn2 = assign79560_e122165_d_n2;
        locals.var_lover_func_dn4 = assign79560_e122165_d_n4;
        locals.var_lover_func_dn5 = assign79560_e122165_d_n5;
        locals.var_lover_func_dn6 = assign79560_e122165_d_n6;
        locals.var_lover_func_dn7 = assign79560_e122165_d_n7;
        locals.var_lover_func_dn8 = assign79560_e122165_d_n8;
        locals.var_lover_func_dn9 = assign79560_e122165_d_n9;
        locals.var_lover_func_dn10 = assign79560_e122165_d_n10;
        locals.var_lover_func_dn13 = assign79560_e122165_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign79570_e122168: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1859 = assign79570_e122168;
        locals.var_guard1859_rv = 0.0;

        let assign79580_e122171: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1860 = assign79580_e122171;
        locals.var_guard1860_rv = 0.0;

        let (assign79590_e122182, assign79590_e122182_d_n0, assign79590_e122182_d_n2, assign79590_e122182_d_n4, assign79590_e122182_d_n5, assign79590_e122182_d_n6, assign79590_e122182_d_n7, assign79590_e122182_d_n8, assign79590_e122182_d_n9, assign79590_e122182_d_n10, assign79590_e122182_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1860 != 0.0)) {
        let assign79590_e122180: f64 = (-locals.var_ps0ld);
        (assign79590_e122180, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk1763, locals.var_vx__blk1763_dn0, locals.var_vx__blk1763_dn2, locals.var_vx__blk1763_dn4, locals.var_vx__blk1763_dn5, locals.var_vx__blk1763_dn6, locals.var_vx__blk1763_dn7, locals.var_vx__blk1763_dn8, locals.var_vx__blk1763_dn9, locals.var_vx__blk1763_dn10, locals.var_vx__blk1763_dn13,)
    }
};
        locals.var_vx__blk1763 = assign79590_e122182;
        locals.var_vx__blk1763_dn0 = assign79590_e122182_d_n0;
        locals.var_vx__blk1763_dn2 = assign79590_e122182_d_n2;
        locals.var_vx__blk1763_dn4 = assign79590_e122182_d_n4;
        locals.var_vx__blk1763_dn5 = assign79590_e122182_d_n5;
        locals.var_vx__blk1763_dn6 = assign79590_e122182_d_n6;
        locals.var_vx__blk1763_dn7 = assign79590_e122182_d_n7;
        locals.var_vx__blk1763_dn8 = assign79590_e122182_d_n8;
        locals.var_vx__blk1763_dn9 = assign79590_e122182_d_n9;
        locals.var_vx__blk1763_dn10 = assign79590_e122182_d_n10;
        locals.var_vx__blk1763_dn13 = assign79590_e122182_d_n13;
        locals.var_vx__blk1763_rv = 0.0;

        let (assign79600_e122193, assign79600_e122193_d_n0, assign79600_e122193_d_n2, assign79600_e122193_d_n4, assign79600_e122193_d_n5, assign79600_e122193_d_n6, assign79600_e122193_d_n7, assign79600_e122193_d_n8, assign79600_e122193_d_n9, assign79600_e122193_d_n10, assign79600_e122193_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1860 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk1763, locals.var_vx__blk1763_dn0, locals.var_vx__blk1763_dn2, locals.var_vx__blk1763_dn4, locals.var_vx__blk1763_dn5, locals.var_vx__blk1763_dn6, locals.var_vx__blk1763_dn7, locals.var_vx__blk1763_dn8, locals.var_vx__blk1763_dn9, locals.var_vx__blk1763_dn10, locals.var_vx__blk1763_dn13,)
    }
};
        locals.var_vx__blk1763 = assign79600_e122193;
        locals.var_vx__blk1763_dn0 = assign79600_e122193_d_n0;
        locals.var_vx__blk1763_dn2 = assign79600_e122193_d_n2;
        locals.var_vx__blk1763_dn4 = assign79600_e122193_d_n4;
        locals.var_vx__blk1763_dn5 = assign79600_e122193_d_n5;
        locals.var_vx__blk1763_dn6 = assign79600_e122193_d_n6;
        locals.var_vx__blk1763_dn7 = assign79600_e122193_d_n7;
        locals.var_vx__blk1763_dn8 = assign79600_e122193_d_n8;
        locals.var_vx__blk1763_dn9 = assign79600_e122193_d_n9;
        locals.var_vx__blk1763_dn10 = assign79600_e122193_d_n10;
        locals.var_vx__blk1763_dn13 = assign79600_e122193_d_n13;
        locals.var_vx__blk1763_rv = 0.0;

        let (assign79610_e122214, assign79610_e122214_d_n0, assign79610_e122214_d_n2, assign79610_e122214_d_n4, assign79610_e122214_d_n5, assign79610_e122214_d_n6, assign79610_e122214_d_n7, assign79610_e122214_d_n8, assign79610_e122214_d_n9, assign79610_e122214_d_n10, assign79610_e122214_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79610_e122201: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79610_e122204: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79610_e122205: f64 = (assign79610_e122201 * assign79610_e122204);
        let assign79610_e122208: f64 = (4.0 * 0.1);
        let assign79610_e122210: f64 = (assign79610_e122208 * 0.1);
        let assign79610_e122211: f64 = (assign79610_e122205 + assign79610_e122210);
        let assign79610_e122212: f64 = (assign79610_e122211).sqrt();
        (assign79610_e122212, (((locals.var_vx__blk1763_dn0 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn0)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn2 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn2)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn4 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn4)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn5 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn5)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn6 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn6)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn7 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn7)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn8 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn8)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn9 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn9)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn10 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn10)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn13 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn13)) / (2.0 * assign79610_e122212)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79610_e122214;
        locals.var_tmf2_dn0 = assign79610_e122214_d_n0;
        locals.var_tmf2_dn2 = assign79610_e122214_d_n2;
        locals.var_tmf2_dn4 = assign79610_e122214_d_n4;
        locals.var_tmf2_dn5 = assign79610_e122214_d_n5;
        locals.var_tmf2_dn6 = assign79610_e122214_d_n6;
        locals.var_tmf2_dn7 = assign79610_e122214_d_n7;
        locals.var_tmf2_dn8 = assign79610_e122214_d_n8;
        locals.var_tmf2_dn9 = assign79610_e122214_d_n9;
        locals.var_tmf2_dn10 = assign79610_e122214_d_n10;
        locals.var_tmf2_dn13 = assign79610_e122214_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign79620_e122230, assign79620_e122230_d_n0, assign79620_e122230_d_n2, assign79620_e122230_d_n4, assign79620_e122230_d_n5, assign79620_e122230_d_n6, assign79620_e122230_d_n7, assign79620_e122230_d_n8, assign79620_e122230_d_n9, assign79620_e122230_d_n10, assign79620_e122230_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79620_e122224: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79620_e122226: f64 = (assign79620_e122224 / locals.var_tmf2);
        let assign79620_e122227: f64 = (1.0 + assign79620_e122226);
        let assign79620_e122228: f64 = (0.5 * assign79620_e122227);
        (assign79620_e122228, (0.5 * (((locals.var_vx__blk1763_dn0 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn2 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn4 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn5 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn6 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn7 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn8 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn9 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn10 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn13 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79620_e122230;
        locals.var_t9_dn0 = assign79620_e122230_d_n0;
        locals.var_t9_dn2 = assign79620_e122230_d_n2;
        locals.var_t9_dn4 = assign79620_e122230_d_n4;
        locals.var_t9_dn5 = assign79620_e122230_d_n5;
        locals.var_t9_dn6 = assign79620_e122230_d_n6;
        locals.var_t9_dn7 = assign79620_e122230_d_n7;
        locals.var_t9_dn8 = assign79620_e122230_d_n8;
        locals.var_t9_dn9 = assign79620_e122230_d_n9;
        locals.var_t9_dn10 = assign79620_e122230_d_n10;
        locals.var_t9_dn13 = assign79620_e122230_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign79630_e122244, assign79630_e122244_d_n0, assign79630_e122244_d_n2, assign79630_e122244_d_n4, assign79630_e122244_d_n5, assign79630_e122244_d_n6, assign79630_e122244_d_n7, assign79630_e122244_d_n8, assign79630_e122244_d_n9, assign79630_e122244_d_n10, assign79630_e122244_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79630_e122239: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79630_e122241: f64 = (assign79630_e122239 + locals.var_tmf2);
        let assign79630_e122242: f64 = (0.5 * assign79630_e122241);
        (assign79630_e122242, (0.5 * (locals.var_vx__blk1763_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk1763_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk1763_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk1763_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk1763_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk1763_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk1763_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk1763_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk1763_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk1763_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79630_e122244;
        locals.var_t2_dn0 = assign79630_e122244_d_n0;
        locals.var_t2_dn2 = assign79630_e122244_d_n2;
        locals.var_t2_dn4 = assign79630_e122244_d_n4;
        locals.var_t2_dn5 = assign79630_e122244_d_n5;
        locals.var_t2_dn6 = assign79630_e122244_d_n6;
        locals.var_t2_dn7 = assign79630_e122244_d_n7;
        locals.var_t2_dn8 = assign79630_e122244_d_n8;
        locals.var_t2_dn9 = assign79630_e122244_d_n9;
        locals.var_t2_dn10 = assign79630_e122244_d_n10;
        locals.var_t2_dn13 = assign79630_e122244_d_n13;
        locals.var_t2_rv = 0.0;

        let assign79640_e122247: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1861 = assign79640_e122247;
        locals.var_guard1861_rv = 0.0;

        let (assign79650_e122257, assign79650_e122257_d_n0, assign79650_e122257_d_n2, assign79650_e122257_d_n4, assign79650_e122257_d_n5, assign79650_e122257_d_n6, assign79650_e122257_d_n7, assign79650_e122257_d_n8, assign79650_e122257_d_n9, assign79650_e122257_d_n10, assign79650_e122257_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79650_e122257;
        locals.var_t2_dn0 = assign79650_e122257_d_n0;
        locals.var_t2_dn2 = assign79650_e122257_d_n2;
        locals.var_t2_dn4 = assign79650_e122257_d_n4;
        locals.var_t2_dn5 = assign79650_e122257_d_n5;
        locals.var_t2_dn6 = assign79650_e122257_d_n6;
        locals.var_t2_dn7 = assign79650_e122257_d_n7;
        locals.var_t2_dn8 = assign79650_e122257_d_n8;
        locals.var_t2_dn9 = assign79650_e122257_d_n9;
        locals.var_t2_dn10 = assign79650_e122257_d_n10;
        locals.var_t2_dn13 = assign79650_e122257_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign79660_e122267, assign79660_e122267_d_n0, assign79660_e122267_d_n2, assign79660_e122267_d_n4, assign79660_e122267_d_n5, assign79660_e122267_d_n6, assign79660_e122267_d_n7, assign79660_e122267_d_n8, assign79660_e122267_d_n9, assign79660_e122267_d_n10, assign79660_e122267_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79660_e122267;
        locals.var_t9_dn0 = assign79660_e122267_d_n0;
        locals.var_t9_dn2 = assign79660_e122267_d_n2;
        locals.var_t9_dn4 = assign79660_e122267_d_n4;
        locals.var_t9_dn5 = assign79660_e122267_d_n5;
        locals.var_t9_dn6 = assign79660_e122267_d_n6;
        locals.var_t9_dn7 = assign79660_e122267_d_n7;
        locals.var_t9_dn8 = assign79660_e122267_d_n8;
        locals.var_t9_dn9 = assign79660_e122267_d_n9;
        locals.var_t9_dn10 = assign79660_e122267_d_n10;
        locals.var_t9_dn13 = assign79660_e122267_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign79670_e122280, assign79670_e122280_d_n0, assign79670_e122280_d_n2, assign79670_e122280_d_n4, assign79670_e122280_d_n5, assign79670_e122280_d_n6, assign79670_e122280_d_n7, assign79670_e122280_d_n8, assign79670_e122280_d_n9, assign79670_e122280_d_n10, assign79670_e122280_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79670_e122275: f64 = (locals.var_kjunc * locals.var_t2);
        let assign79670_e122276: f64 = (assign79670_e122275).sqrt();
        let assign79670_e122278: f64 = (assign79670_e122276 * p.p432);
        (assign79670_e122278, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign79670_e122276)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign79670_e122280;
        locals.var_wjunc0_dn0 = assign79670_e122280_d_n0;
        locals.var_wjunc0_dn2 = assign79670_e122280_d_n2;
        locals.var_wjunc0_dn4 = assign79670_e122280_d_n4;
        locals.var_wjunc0_dn5 = assign79670_e122280_d_n5;
        locals.var_wjunc0_dn6 = assign79670_e122280_d_n6;
        locals.var_wjunc0_dn7 = assign79670_e122280_d_n7;
        locals.var_wjunc0_dn8 = assign79670_e122280_d_n8;
        locals.var_wjunc0_dn9 = assign79670_e122280_d_n9;
        locals.var_wjunc0_dn10 = assign79670_e122280_d_n10;
        locals.var_wjunc0_dn13 = assign79670_e122280_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign79680_e122294, assign79680_e122294_d_n0, assign79680_e122294_d_n2, assign79680_e122294_d_n4, assign79680_e122294_d_n5, assign79680_e122294_d_n6, assign79680_e122294_d_n7, assign79680_e122294_d_n8, assign79680_e122294_d_n9, assign79680_e122294_d_n10, assign79680_e122294_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79680_e122288: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign79680_e122291: f64 = (0.1 * locals.var_lover_func);
        let assign79680_e122292: f64 = (assign79680_e122288 - assign79680_e122291);
        (assign79680_e122292, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign79680_e122294;
        locals.var_tmf1_dn0 = assign79680_e122294_d_n0;
        locals.var_tmf1_dn2 = assign79680_e122294_d_n2;
        locals.var_tmf1_dn4 = assign79680_e122294_d_n4;
        locals.var_tmf1_dn5 = assign79680_e122294_d_n5;
        locals.var_tmf1_dn6 = assign79680_e122294_d_n6;
        locals.var_tmf1_dn7 = assign79680_e122294_d_n7;
        locals.var_tmf1_dn8 = assign79680_e122294_d_n8;
        locals.var_tmf1_dn9 = assign79680_e122294_d_n9;
        locals.var_tmf1_dn10 = assign79680_e122294_d_n10;
        locals.var_tmf1_dn13 = assign79680_e122294_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign79690_e122308, assign79690_e122308_d_n0, assign79690_e122308_d_n2, assign79690_e122308_d_n4, assign79690_e122308_d_n5, assign79690_e122308_d_n6, assign79690_e122308_d_n7, assign79690_e122308_d_n8, assign79690_e122308_d_n9, assign79690_e122308_d_n10, assign79690_e122308_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79690_e122302: f64 = (4.0 * locals.var_lover_func);
        let assign79690_e122305: f64 = (0.1 * locals.var_lover_func);
        let assign79690_e122306: f64 = (assign79690_e122302 * assign79690_e122305);
        (assign79690_e122306, (((4.0 * locals.var_lover_func_dn0) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79690_e122308;
        locals.var_tmf2_dn0 = assign79690_e122308_d_n0;
        locals.var_tmf2_dn2 = assign79690_e122308_d_n2;
        locals.var_tmf2_dn4 = assign79690_e122308_d_n4;
        locals.var_tmf2_dn5 = assign79690_e122308_d_n5;
        locals.var_tmf2_dn6 = assign79690_e122308_d_n6;
        locals.var_tmf2_dn7 = assign79690_e122308_d_n7;
        locals.var_tmf2_dn8 = assign79690_e122308_d_n8;
        locals.var_tmf2_dn9 = assign79690_e122308_d_n9;
        locals.var_tmf2_dn10 = assign79690_e122308_d_n10;
        locals.var_tmf2_dn13 = assign79690_e122308_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign79700_e122322, assign79700_e122322_d_n0, assign79700_e122322_d_n2, assign79700_e122322_d_n4, assign79700_e122322_d_n5, assign79700_e122322_d_n6, assign79700_e122322_d_n7, assign79700_e122322_d_n8, assign79700_e122322_d_n9, assign79700_e122322_d_n10, assign79700_e122322_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let (assign79700_e122320, assign79700_e122320_d_n0, assign79700_e122320_d_n2, assign79700_e122320_d_n4, assign79700_e122320_d_n5, assign79700_e122320_d_n6, assign79700_e122320_d_n7, assign79700_e122320_d_n8, assign79700_e122320_d_n9, assign79700_e122320_d_n10, assign79700_e122320_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign79700_e122319: f64 = (-locals.var_tmf2);
                (assign79700_e122319, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign79700_e122320, assign79700_e122320_d_n0, assign79700_e122320_d_n2, assign79700_e122320_d_n4, assign79700_e122320_d_n5, assign79700_e122320_d_n6, assign79700_e122320_d_n7, assign79700_e122320_d_n8, assign79700_e122320_d_n9, assign79700_e122320_d_n10, assign79700_e122320_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79700_e122322;
        locals.var_tmf2_dn0 = assign79700_e122322_d_n0;
        locals.var_tmf2_dn2 = assign79700_e122322_d_n2;
        locals.var_tmf2_dn4 = assign79700_e122322_d_n4;
        locals.var_tmf2_dn5 = assign79700_e122322_d_n5;
        locals.var_tmf2_dn6 = assign79700_e122322_d_n6;
        locals.var_tmf2_dn7 = assign79700_e122322_d_n7;
        locals.var_tmf2_dn8 = assign79700_e122322_d_n8;
        locals.var_tmf2_dn9 = assign79700_e122322_d_n9;
        locals.var_tmf2_dn10 = assign79700_e122322_d_n10;
        locals.var_tmf2_dn13 = assign79700_e122322_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_293(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79710_e122335, assign79710_e122335_d_n0, assign79710_e122335_d_n2, assign79710_e122335_d_n4, assign79710_e122335_d_n5, assign79710_e122335_d_n6, assign79710_e122335_d_n7, assign79710_e122335_d_n8, assign79710_e122335_d_n9, assign79710_e122335_d_n10, assign79710_e122335_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79710_e122330: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign79710_e122332: f64 = (assign79710_e122330 + locals.var_tmf2);
        let assign79710_e122333: f64 = (assign79710_e122332).sqrt();
        (assign79710_e122333, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign79710_e122333)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79710_e122335;
        locals.var_tmf2_dn0 = assign79710_e122335_d_n0;
        locals.var_tmf2_dn2 = assign79710_e122335_d_n2;
        locals.var_tmf2_dn4 = assign79710_e122335_d_n4;
        locals.var_tmf2_dn5 = assign79710_e122335_d_n5;
        locals.var_tmf2_dn6 = assign79710_e122335_d_n6;
        locals.var_tmf2_dn7 = assign79710_e122335_d_n7;
        locals.var_tmf2_dn8 = assign79710_e122335_d_n8;
        locals.var_tmf2_dn9 = assign79710_e122335_d_n9;
        locals.var_tmf2_dn10 = assign79710_e122335_d_n10;
        locals.var_tmf2_dn13 = assign79710_e122335_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign79720_e122349, assign79720_e122349_d_n0, assign79720_e122349_d_n2, assign79720_e122349_d_n4, assign79720_e122349_d_n5, assign79720_e122349_d_n6, assign79720_e122349_d_n7, assign79720_e122349_d_n8, assign79720_e122349_d_n9, assign79720_e122349_d_n10, assign79720_e122349_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79720_e122345: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign79720_e122346: f64 = (1.0 + assign79720_e122345);
        let assign79720_e122347: f64 = (0.5 * assign79720_e122346);
        (assign79720_e122347, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign79720_e122349;
        locals.var_t0_dn0 = assign79720_e122349_d_n0;
        locals.var_t0_dn2 = assign79720_e122349_d_n2;
        locals.var_t0_dn4 = assign79720_e122349_d_n4;
        locals.var_t0_dn5 = assign79720_e122349_d_n5;
        locals.var_t0_dn6 = assign79720_e122349_d_n6;
        locals.var_t0_dn7 = assign79720_e122349_d_n7;
        locals.var_t0_dn8 = assign79720_e122349_d_n8;
        locals.var_t0_dn9 = assign79720_e122349_d_n9;
        locals.var_t0_dn10 = assign79720_e122349_d_n10;
        locals.var_t0_dn13 = assign79720_e122349_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign79730_e122363, assign79730_e122363_d_n0, assign79730_e122363_d_n2, assign79730_e122363_d_n4, assign79730_e122363_d_n5, assign79730_e122363_d_n6, assign79730_e122363_d_n7, assign79730_e122363_d_n8, assign79730_e122363_d_n9, assign79730_e122363_d_n10, assign79730_e122363_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79730_e122359: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign79730_e122360: f64 = (0.5 * assign79730_e122359);
        let assign79730_e122361: f64 = (locals.var_lover_func - assign79730_e122360);
        (assign79730_e122361, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign79730_e122363;
        locals.var_wjuncld_dn0 = assign79730_e122363_d_n0;
        locals.var_wjuncld_dn2 = assign79730_e122363_d_n2;
        locals.var_wjuncld_dn4 = assign79730_e122363_d_n4;
        locals.var_wjuncld_dn5 = assign79730_e122363_d_n5;
        locals.var_wjuncld_dn6 = assign79730_e122363_d_n6;
        locals.var_wjuncld_dn7 = assign79730_e122363_d_n7;
        locals.var_wjuncld_dn8 = assign79730_e122363_d_n8;
        locals.var_wjuncld_dn9 = assign79730_e122363_d_n9;
        locals.var_wjuncld_dn10 = assign79730_e122363_d_n10;
        locals.var_wjuncld_dn13 = assign79730_e122363_d_n13;
        locals.var_wjuncld_rv = 0.0;

        let (assign79740_e122373, assign79740_e122373_d_n0, assign79740_e122373_d_n2, assign79740_e122373_d_n4, assign79740_e122373_d_n5, assign79740_e122373_d_n6, assign79740_e122373_d_n7, assign79740_e122373_d_n8, assign79740_e122373_d_n9, assign79740_e122373_d_n10, assign79740_e122373_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79740_e122371: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign79740_e122371, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign79740_e122373;
        locals.var_lover_func_dn0 = assign79740_e122373_d_n0;
        locals.var_lover_func_dn2 = assign79740_e122373_d_n2;
        locals.var_lover_func_dn4 = assign79740_e122373_d_n4;
        locals.var_lover_func_dn5 = assign79740_e122373_d_n5;
        locals.var_lover_func_dn6 = assign79740_e122373_d_n6;
        locals.var_lover_func_dn7 = assign79740_e122373_d_n7;
        locals.var_lover_func_dn8 = assign79740_e122373_d_n8;
        locals.var_lover_func_dn9 = assign79740_e122373_d_n9;
        locals.var_lover_func_dn10 = assign79740_e122373_d_n10;
        locals.var_lover_func_dn13 = assign79740_e122373_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign79750_e122376: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1862 = assign79750_e122376;
        locals.var_guard1862_rv = 0.0;

        let assign79760_e122379: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1863 = assign79760_e122379;
        locals.var_guard1863_rv = 0.0;

        let assign79770_e122382: f64 = if 2.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1864 = assign79770_e122382;
        locals.var_guard1864_rv = 0.0;

        let assign79780_e122385: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1865 = assign79780_e122385;
        locals.var_guard1865_rv = 0.0;

        let assign79790_e122388: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1866 = assign79790_e122388;
        locals.var_guard1866_rv = 0.0;

        let (assign79800_e122398, assign79800_e122398_d_n0, assign79800_e122398_d_n2, assign79800_e122398_d_n4, assign79800_e122398_d_n5, assign79800_e122398_d_n6, assign79800_e122398_d_n7, assign79800_e122398_d_n8, assign79800_e122398_d_n9, assign79800_e122398_d_n10, assign79800_e122398_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) && (locals.var_guard1866 != 0.0)) {
        let assign79800_e122396: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign79800_e122396, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79800_e122398;
        locals.var_t4_dn0 = assign79800_e122398_d_n0;
        locals.var_t4_dn2 = assign79800_e122398_d_n2;
        locals.var_t4_dn4 = assign79800_e122398_d_n4;
        locals.var_t4_dn5 = assign79800_e122398_d_n5;
        locals.var_t4_dn6 = assign79800_e122398_d_n6;
        locals.var_t4_dn7 = assign79800_e122398_d_n7;
        locals.var_t4_dn8 = assign79800_e122398_d_n8;
        locals.var_t4_dn9 = assign79800_e122398_d_n9;
        locals.var_t4_dn10 = assign79800_e122398_d_n10;
        locals.var_t4_dn13 = assign79800_e122398_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign79810_e122413, assign79810_e122413_d_n0, assign79810_e122413_d_n2, assign79810_e122413_d_n4, assign79810_e122413_d_n5, assign79810_e122413_d_n6, assign79810_e122413_d_n7, assign79810_e122413_d_n8, assign79810_e122413_d_n9, assign79810_e122413_d_n10, assign79810_e122413_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) && (locals.var_guard1866 == 0.0)) {
        let assign79810_e122407: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79810_e122410: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign79810_e122411: f64 = (assign79810_e122407 * assign79810_e122410);
        (assign79810_e122411, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign79810_e122410),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79810_e122413;
        locals.var_t4_dn0 = assign79810_e122413_d_n0;
        locals.var_t4_dn2 = assign79810_e122413_d_n2;
        locals.var_t4_dn4 = assign79810_e122413_d_n4;
        locals.var_t4_dn5 = assign79810_e122413_d_n5;
        locals.var_t4_dn6 = assign79810_e122413_d_n6;
        locals.var_t4_dn7 = assign79810_e122413_d_n7;
        locals.var_t4_dn8 = assign79810_e122413_d_n8;
        locals.var_t4_dn9 = assign79810_e122413_d_n9;
        locals.var_t4_dn10 = assign79810_e122413_d_n10;
        locals.var_t4_dn13 = assign79810_e122413_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign79820_e122421, assign79820_e122421_d_n0, assign79820_e122421_d_n2, assign79820_e122421_d_n4, assign79820_e122421_d_n5, assign79820_e122421_d_n6, assign79820_e122421_d_n7, assign79820_e122421_d_n8, assign79820_e122421_d_n9, assign79820_e122421_d_n10, assign79820_e122421_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) {
        let assign79820_e122419: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79820_e122419, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn13,)
    }
};
        locals.var_qovs = assign79820_e122421;
        locals.var_qovs_dn0 = assign79820_e122421_d_n0;
        locals.var_qovs_dn2 = assign79820_e122421_d_n2;
        locals.var_qovs_dn4 = assign79820_e122421_d_n4;
        locals.var_qovs_dn5 = assign79820_e122421_d_n5;
        locals.var_qovs_dn6 = assign79820_e122421_d_n6;
        locals.var_qovs_dn7 = assign79820_e122421_d_n7;
        locals.var_qovs_dn8 = assign79820_e122421_d_n8;
        locals.var_qovs_dn9 = assign79820_e122421_d_n9;
        locals.var_qovs_dn10 = assign79820_e122421_d_n10;
        locals.var_qovs_dn13 = assign79820_e122421_d_n13;
        locals.var_qovs_rv = 0.0;

        let (assign79830_e122429, assign79830_e122429_d_n0, assign79830_e122429_d_n2, assign79830_e122429_d_n4, assign79830_e122429_d_n5, assign79830_e122429_d_n6, assign79830_e122429_d_n7, assign79830_e122429_d_n8, assign79830_e122429_d_n9, assign79830_e122429_d_n10, assign79830_e122429_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) {
        let assign79830_e122427: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79830_e122427, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn13,)
    }
};
        locals.var_qbsld = assign79830_e122429;
        locals.var_qbsld_dn0 = assign79830_e122429_d_n0;
        locals.var_qbsld_dn2 = assign79830_e122429_d_n2;
        locals.var_qbsld_dn4 = assign79830_e122429_d_n4;
        locals.var_qbsld_dn5 = assign79830_e122429_d_n5;
        locals.var_qbsld_dn6 = assign79830_e122429_d_n6;
        locals.var_qbsld_dn7 = assign79830_e122429_d_n7;
        locals.var_qbsld_dn8 = assign79830_e122429_d_n8;
        locals.var_qbsld_dn9 = assign79830_e122429_d_n9;
        locals.var_qbsld_dn10 = assign79830_e122429_d_n10;
        locals.var_qbsld_dn13 = assign79830_e122429_d_n13;
        locals.var_qbsld_rv = 0.0;

        let (assign79860_e122454, assign79860_e122454_d_n0, assign79860_e122454_d_n2, assign79860_e122454_d_n4, assign79860_e122454_d_n5, assign79860_e122454_d_n6, assign79860_e122454_d_n7, assign79860_e122454_d_n8, assign79860_e122454_d_n9, assign79860_e122454_d_n10, assign79860_e122454_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1863 != 0.0) && (locals.var_guard1862 == 0.0))) {
        let assign79860_e122450: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79860_e122452: f64 = (assign79860_e122450 * locals.var_uc_cvdsover);
        (assign79860_e122452, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79860_e122454;
        locals.var_t4_dn0 = assign79860_e122454_d_n0;
        locals.var_t4_dn2 = assign79860_e122454_d_n2;
        locals.var_t4_dn4 = assign79860_e122454_d_n4;
        locals.var_t4_dn5 = assign79860_e122454_d_n5;
        locals.var_t4_dn6 = assign79860_e122454_d_n6;
        locals.var_t4_dn7 = assign79860_e122454_d_n7;
        locals.var_t4_dn8 = assign79860_e122454_d_n8;
        locals.var_t4_dn9 = assign79860_e122454_d_n9;
        locals.var_t4_dn10 = assign79860_e122454_d_n10;
        locals.var_t4_dn13 = assign79860_e122454_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign79870_e122465, assign79870_e122465_d_n0, assign79870_e122465_d_n2, assign79870_e122465_d_n4, assign79870_e122465_d_n5, assign79870_e122465_d_n6, assign79870_e122465_d_n7, assign79870_e122465_d_n8, assign79870_e122465_d_n9, assign79870_e122465_d_n10, assign79870_e122465_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1863 != 0.0) && (locals.var_guard1862 == 0.0))) {
        let assign79870_e122463: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79870_e122463, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn13,)
    }
};
        locals.var_qovsext = assign79870_e122465;
        locals.var_qovsext_dn0 = assign79870_e122465_d_n0;
        locals.var_qovsext_dn2 = assign79870_e122465_d_n2;
        locals.var_qovsext_dn4 = assign79870_e122465_d_n4;
        locals.var_qovsext_dn5 = assign79870_e122465_d_n5;
        locals.var_qovsext_dn6 = assign79870_e122465_d_n6;
        locals.var_qovsext_dn7 = assign79870_e122465_d_n7;
        locals.var_qovsext_dn8 = assign79870_e122465_d_n8;
        locals.var_qovsext_dn9 = assign79870_e122465_d_n9;
        locals.var_qovsext_dn10 = assign79870_e122465_d_n10;
        locals.var_qovsext_dn13 = assign79870_e122465_d_n13;
        locals.var_qovsext_rv = 0.0;

        let (assign79880_e122476, assign79880_e122476_d_n0, assign79880_e122476_d_n2, assign79880_e122476_d_n4, assign79880_e122476_d_n5, assign79880_e122476_d_n6, assign79880_e122476_d_n7, assign79880_e122476_d_n8, assign79880_e122476_d_n9, assign79880_e122476_d_n10, assign79880_e122476_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1863 != 0.0) && (locals.var_guard1862 == 0.0))) {
        let assign79880_e122474: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79880_e122474, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn13,)
    }
};
        locals.var_qbsldext = assign79880_e122476;
        locals.var_qbsldext_dn0 = assign79880_e122476_d_n0;
        locals.var_qbsldext_dn2 = assign79880_e122476_d_n2;
        locals.var_qbsldext_dn4 = assign79880_e122476_d_n4;
        locals.var_qbsldext_dn5 = assign79880_e122476_d_n5;
        locals.var_qbsldext_dn6 = assign79880_e122476_d_n6;
        locals.var_qbsldext_dn7 = assign79880_e122476_d_n7;
        locals.var_qbsldext_dn8 = assign79880_e122476_d_n8;
        locals.var_qbsldext_dn9 = assign79880_e122476_d_n9;
        locals.var_qbsldext_dn10 = assign79880_e122476_d_n10;
        locals.var_qbsldext_dn13 = assign79880_e122476_d_n13;
        locals.var_qbsldext_rv = 0.0;

        let assign79890_e122479: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1867 = assign79890_e122479;
        locals.var_guard1867_rv = 0.0;

        let (assign79900_e122494, assign79900_e122494_d_n0, assign79900_e122494_d_n2, assign79900_e122494_d_n4, assign79900_e122494_d_n5, assign79900_e122494_d_n6, assign79900_e122494_d_n7, assign79900_e122494_d_n8, assign79900_e122494_d_n9, assign79900_e122494_d_n10, assign79900_e122494_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) && (locals.var_guard1867 != 0.0)) {
        let assign79900_e122492: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign79900_e122492, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79900_e122494;
        locals.var_t4_dn0 = assign79900_e122494_d_n0;
        locals.var_t4_dn2 = assign79900_e122494_d_n2;
        locals.var_t4_dn4 = assign79900_e122494_d_n4;
        locals.var_t4_dn5 = assign79900_e122494_d_n5;
        locals.var_t4_dn6 = assign79900_e122494_d_n6;
        locals.var_t4_dn7 = assign79900_e122494_d_n7;
        locals.var_t4_dn8 = assign79900_e122494_d_n8;
        locals.var_t4_dn9 = assign79900_e122494_d_n9;
        locals.var_t4_dn10 = assign79900_e122494_d_n10;
        locals.var_t4_dn13 = assign79900_e122494_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign79910_e122514, assign79910_e122514_d_n0, assign79910_e122514_d_n2, assign79910_e122514_d_n4, assign79910_e122514_d_n5, assign79910_e122514_d_n6, assign79910_e122514_d_n7, assign79910_e122514_d_n8, assign79910_e122514_d_n9, assign79910_e122514_d_n10, assign79910_e122514_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) && (locals.var_guard1867 == 0.0)) {
        let assign79910_e122508: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79910_e122511: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign79910_e122512: f64 = (assign79910_e122508 * assign79910_e122511);
        (assign79910_e122512, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign79910_e122511),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79910_e122514;
        locals.var_t4_dn0 = assign79910_e122514_d_n0;
        locals.var_t4_dn2 = assign79910_e122514_d_n2;
        locals.var_t4_dn4 = assign79910_e122514_d_n4;
        locals.var_t4_dn5 = assign79910_e122514_d_n5;
        locals.var_t4_dn6 = assign79910_e122514_d_n6;
        locals.var_t4_dn7 = assign79910_e122514_d_n7;
        locals.var_t4_dn8 = assign79910_e122514_d_n8;
        locals.var_t4_dn9 = assign79910_e122514_d_n9;
        locals.var_t4_dn10 = assign79910_e122514_d_n10;
        locals.var_t4_dn13 = assign79910_e122514_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign79920_e122525, assign79920_e122525_d_n0, assign79920_e122525_d_n2, assign79920_e122525_d_n4, assign79920_e122525_d_n5, assign79920_e122525_d_n6, assign79920_e122525_d_n7, assign79920_e122525_d_n8, assign79920_e122525_d_n9, assign79920_e122525_d_n10, assign79920_e122525_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn13,)
    }
};
        locals.var_rd_ps0ld = assign79920_e122525;
        locals.var_rd_ps0ld_dn0 = assign79920_e122525_d_n0;
        locals.var_rd_ps0ld_dn2 = assign79920_e122525_d_n2;
        locals.var_rd_ps0ld_dn4 = assign79920_e122525_d_n4;
        locals.var_rd_ps0ld_dn5 = assign79920_e122525_d_n5;
        locals.var_rd_ps0ld_dn6 = assign79920_e122525_d_n6;
        locals.var_rd_ps0ld_dn7 = assign79920_e122525_d_n7;
        locals.var_rd_ps0ld_dn8 = assign79920_e122525_d_n8;
        locals.var_rd_ps0ld_dn9 = assign79920_e122525_d_n9;
        locals.var_rd_ps0ld_dn10 = assign79920_e122525_d_n10;
        locals.var_rd_ps0ld_dn13 = assign79920_e122525_d_n13;
        locals.var_rd_ps0ld_rv = 0.0;

        let assign79930_e122528: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1868 = assign79930_e122528;
        locals.var_guard1868_rv = 0.0;

        let (assign79940_e122541, assign79940_e122541_d_n0, assign79940_e122541_d_n2, assign79940_e122541_d_n4, assign79940_e122541_d_n5, assign79940_e122541_d_n6, assign79940_e122541_d_n7, assign79940_e122541_d_n8, assign79940_e122541_d_n9, assign79940_e122541_d_n10, assign79940_e122541_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) && (locals.var_guard1868 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign79940_e122541;
        locals.var_rd_qbuld_dn0 = assign79940_e122541_d_n0;
        locals.var_rd_qbuld_dn2 = assign79940_e122541_d_n2;
        locals.var_rd_qbuld_dn4 = assign79940_e122541_d_n4;
        locals.var_rd_qbuld_dn5 = assign79940_e122541_d_n5;
        locals.var_rd_qbuld_dn6 = assign79940_e122541_d_n6;
        locals.var_rd_qbuld_dn7 = assign79940_e122541_d_n7;
        locals.var_rd_qbuld_dn8 = assign79940_e122541_d_n8;
        locals.var_rd_qbuld_dn9 = assign79940_e122541_d_n9;
        locals.var_rd_qbuld_dn10 = assign79940_e122541_d_n10;
        locals.var_rd_qbuld_dn13 = assign79940_e122541_d_n13;
        locals.var_rd_qbuld_rv = 0.0;

        let (assign79950_e122554, assign79950_e122554_d_n0, assign79950_e122554_d_n2, assign79950_e122554_d_n4, assign79950_e122554_d_n5, assign79950_e122554_d_n6, assign79950_e122554_d_n7, assign79950_e122554_d_n8, assign79950_e122554_d_n9, assign79950_e122554_d_n10, assign79950_e122554_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        let assign79950_e122552: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79950_e122552, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign79950_e122554;
        locals.var_qovd_dn0 = assign79950_e122554_d_n0;
        locals.var_qovd_dn2 = assign79950_e122554_d_n2;
        locals.var_qovd_dn4 = assign79950_e122554_d_n4;
        locals.var_qovd_dn5 = assign79950_e122554_d_n5;
        locals.var_qovd_dn6 = assign79950_e122554_d_n6;
        locals.var_qovd_dn7 = assign79950_e122554_d_n7;
        locals.var_qovd_dn8 = assign79950_e122554_d_n8;
        locals.var_qovd_dn9 = assign79950_e122554_d_n9;
        locals.var_qovd_dn10 = assign79950_e122554_d_n10;
        locals.var_qovd_dn13 = assign79950_e122554_d_n13;
        locals.var_qovd_rv = 0.0;

        let (assign79960_e122567, assign79960_e122567_d_n0, assign79960_e122567_d_n2, assign79960_e122567_d_n4, assign79960_e122567_d_n5, assign79960_e122567_d_n6, assign79960_e122567_d_n7, assign79960_e122567_d_n8, assign79960_e122567_d_n9, assign79960_e122567_d_n10, assign79960_e122567_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        let assign79960_e122565: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79960_e122565, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign79960_e122567;
        locals.var_qbdld_dn0 = assign79960_e122567_d_n0;
        locals.var_qbdld_dn2 = assign79960_e122567_d_n2;
        locals.var_qbdld_dn4 = assign79960_e122567_d_n4;
        locals.var_qbdld_dn5 = assign79960_e122567_d_n5;
        locals.var_qbdld_dn6 = assign79960_e122567_d_n6;
        locals.var_qbdld_dn7 = assign79960_e122567_d_n7;
        locals.var_qbdld_dn8 = assign79960_e122567_d_n8;
        locals.var_qbdld_dn9 = assign79960_e122567_d_n9;
        locals.var_qbdld_dn10 = assign79960_e122567_d_n10;
        locals.var_qbdld_dn13 = assign79960_e122567_d_n13;
        locals.var_qbdld_rv = 0.0;

        let (assign79970_e122578, assign79970_e122578_d_n0, assign79970_e122578_d_n2, assign79970_e122578_d_n4, assign79970_e122578_d_n5, assign79970_e122578_d_n6, assign79970_e122578_d_n7, assign79970_e122578_d_n8, assign79970_e122578_d_n9, assign79970_e122578_d_n10, assign79970_e122578_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    }
};
        locals.var_qbd_qs = assign79970_e122578;
        locals.var_qbd_qs_dn0 = assign79970_e122578_d_n0;
        locals.var_qbd_qs_dn2 = assign79970_e122578_d_n2;
        locals.var_qbd_qs_dn4 = assign79970_e122578_d_n4;
        locals.var_qbd_qs_dn5 = assign79970_e122578_d_n5;
        locals.var_qbd_qs_dn6 = assign79970_e122578_d_n6;
        locals.var_qbd_qs_dn7 = assign79970_e122578_d_n7;
        locals.var_qbd_qs_dn8 = assign79970_e122578_d_n8;
        locals.var_qbd_qs_dn9 = assign79970_e122578_d_n9;
        locals.var_qbd_qs_dn10 = assign79970_e122578_d_n10;
        locals.var_qbd_qs_dn13 = assign79970_e122578_d_n13;
        locals.var_qbd_qs_rv = 0.0;

        let (assign79980_e122595, assign79980_e122595_d_n0, assign79980_e122595_d_n2, assign79980_e122595_d_n4, assign79980_e122595_d_n5, assign79980_e122595_d_n6, assign79980_e122595_d_n7, assign79980_e122595_d_n8, assign79980_e122595_d_n9, assign79980_e122595_d_n10, assign79980_e122595_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (!(((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0)) || (locals.var_guard1864 != 0.0))))) {
        let assign79980_e122591: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79980_e122593: f64 = (assign79980_e122591 * locals.var_uc_cvdsover);
        (assign79980_e122593, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79980_e122595;
        locals.var_t4_dn0 = assign79980_e122595_d_n0;
        locals.var_t4_dn2 = assign79980_e122595_d_n2;
        locals.var_t4_dn4 = assign79980_e122595_d_n4;
        locals.var_t4_dn5 = assign79980_e122595_d_n5;
        locals.var_t4_dn6 = assign79980_e122595_d_n6;
        locals.var_t4_dn7 = assign79980_e122595_d_n7;
        locals.var_t4_dn8 = assign79980_e122595_d_n8;
        locals.var_t4_dn9 = assign79980_e122595_d_n9;
        locals.var_t4_dn10 = assign79980_e122595_d_n10;
        locals.var_t4_dn13 = assign79980_e122595_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign79990_e122610, assign79990_e122610_d_n0, assign79990_e122610_d_n2, assign79990_e122610_d_n4, assign79990_e122610_d_n5, assign79990_e122610_d_n6, assign79990_e122610_d_n7, assign79990_e122610_d_n8, assign79990_e122610_d_n9, assign79990_e122610_d_n10, assign79990_e122610_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (!(((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0)) || (locals.var_guard1864 != 0.0))))) {
        let assign79990_e122608: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79990_e122608, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn13,)
    }
};
        locals.var_qovdext = assign79990_e122610;
        locals.var_qovdext_dn0 = assign79990_e122610_d_n0;
        locals.var_qovdext_dn2 = assign79990_e122610_d_n2;
        locals.var_qovdext_dn4 = assign79990_e122610_d_n4;
        locals.var_qovdext_dn5 = assign79990_e122610_d_n5;
        locals.var_qovdext_dn6 = assign79990_e122610_d_n6;
        locals.var_qovdext_dn7 = assign79990_e122610_d_n7;
        locals.var_qovdext_dn8 = assign79990_e122610_d_n8;
        locals.var_qovdext_dn9 = assign79990_e122610_d_n9;
        locals.var_qovdext_dn10 = assign79990_e122610_d_n10;
        locals.var_qovdext_dn13 = assign79990_e122610_d_n13;
        locals.var_qovdext_rv = 0.0;

        let (assign80000_e122625, assign80000_e122625_d_n0, assign80000_e122625_d_n2, assign80000_e122625_d_n4, assign80000_e122625_d_n5, assign80000_e122625_d_n6, assign80000_e122625_d_n7, assign80000_e122625_d_n8, assign80000_e122625_d_n9, assign80000_e122625_d_n10, assign80000_e122625_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (!(((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0)) || (locals.var_guard1864 != 0.0))))) {
        let assign80000_e122623: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign80000_e122623, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn13,)
    }
};
        locals.var_qbdldext = assign80000_e122625;
        locals.var_qbdldext_dn0 = assign80000_e122625_d_n0;
        locals.var_qbdldext_dn2 = assign80000_e122625_d_n2;
        locals.var_qbdldext_dn4 = assign80000_e122625_d_n4;
        locals.var_qbdldext_dn5 = assign80000_e122625_d_n5;
        locals.var_qbdldext_dn6 = assign80000_e122625_d_n6;
        locals.var_qbdldext_dn7 = assign80000_e122625_d_n7;
        locals.var_qbdldext_dn8 = assign80000_e122625_d_n8;
        locals.var_qbdldext_dn9 = assign80000_e122625_d_n9;
        locals.var_qbdldext_dn10 = assign80000_e122625_d_n10;
        locals.var_qbdldext_dn13 = assign80000_e122625_d_n13;
        locals.var_qbdldext_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        let assign80020_e122629: f64 = if 3.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1869 = assign80020_e122629;
        locals.var_guard1869_rv = 0.0;

        let assign80030_e122632: f64 = if 3.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1870 = assign80030_e122632;
        locals.var_guard1870_rv = 0.0;

        let assign80040_e122635: f64 = if 3.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1871 = assign80040_e122635;
        locals.var_guard1871_rv = 0.0;

        let assign80050_e122638: f64 = if 3.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1872 = assign80050_e122638;
        locals.var_guard1872_rv = 0.0;

        let assign80060_e122649: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1873 = assign80060_e122649;
        locals.var_guard1873_rv = 0.0;

        let (assign80070_e122655,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80070_e122655;
        locals.var_flg_calcqover_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_294(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign80080_e122661,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign80080_e122661;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign80090_e122669, assign80090_e122669_d_n2, assign80090_e122669_d_n6, assign80090_e122669_d_n7, assign80090_e122669_d_n8,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        let assign80090_e122667: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign80090_e122667, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign80090_e122669;
        locals.var_vgbgmt_dn2 = assign80090_e122669_d_n2;
        locals.var_vgbgmt_dn6 = assign80090_e122669_d_n6;
        locals.var_vgbgmt_dn7 = assign80090_e122669_d_n7;
        locals.var_vgbgmt_dn8 = assign80090_e122669_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign80100_e122676, assign80100_e122676_d_n0, assign80100_e122676_d_n2, assign80100_e122676_d_n4, assign80100_e122676_d_n5, assign80100_e122676_d_n6, assign80100_e122676_d_n7, assign80100_e122676_d_n8, assign80100_e122676_d_n9, assign80100_e122676_d_n10, assign80100_e122676_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        let assign80100_e122674: f64 = (-locals.var_vbsi);
        (assign80100_e122674, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80100_e122676;
        locals.var_vxbgmt_dn0 = assign80100_e122676_d_n0;
        locals.var_vxbgmt_dn2 = assign80100_e122676_d_n2;
        locals.var_vxbgmt_dn4 = assign80100_e122676_d_n4;
        locals.var_vxbgmt_dn5 = assign80100_e122676_d_n5;
        locals.var_vxbgmt_dn6 = assign80100_e122676_d_n6;
        locals.var_vxbgmt_dn7 = assign80100_e122676_d_n7;
        locals.var_vxbgmt_dn8 = assign80100_e122676_d_n8;
        locals.var_vxbgmt_dn9 = assign80100_e122676_d_n9;
        locals.var_vxbgmt_dn10 = assign80100_e122676_d_n10;
        locals.var_vxbgmt_dn13 = assign80100_e122676_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign80110_e122682,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign80110_e122682;
        locals.var_nover_func_rv = 0.0;

        let (assign80120_e122688, assign80120_e122688_d_n0, assign80120_e122688_d_n2, assign80120_e122688_d_n4, assign80120_e122688_d_n5, assign80120_e122688_d_n6, assign80120_e122688_d_n7, assign80120_e122688_d_n8, assign80120_e122688_d_n9, assign80120_e122688_d_n10, assign80120_e122688_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80120_e122688;
        locals.var_lover_func_dn0 = assign80120_e122688_d_n0;
        locals.var_lover_func_dn2 = assign80120_e122688_d_n2;
        locals.var_lover_func_dn4 = assign80120_e122688_d_n4;
        locals.var_lover_func_dn5 = assign80120_e122688_d_n5;
        locals.var_lover_func_dn6 = assign80120_e122688_d_n6;
        locals.var_lover_func_dn7 = assign80120_e122688_d_n7;
        locals.var_lover_func_dn8 = assign80120_e122688_d_n8;
        locals.var_lover_func_dn9 = assign80120_e122688_d_n9;
        locals.var_lover_func_dn10 = assign80120_e122688_d_n10;
        locals.var_lover_func_dn13 = assign80120_e122688_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign80130_e122694, assign80130_e122694_d_n0, assign80130_e122694_d_n2, assign80130_e122694_d_n4, assign80130_e122694_d_n5, assign80130_e122694_d_n6, assign80130_e122694_d_n7, assign80130_e122694_d_n8, assign80130_e122694_d_n9, assign80130_e122694_d_n10, assign80130_e122694_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign80130_e122694;
        locals.var_wdep_func_dn0 = assign80130_e122694_d_n0;
        locals.var_wdep_func_dn2 = assign80130_e122694_d_n2;
        locals.var_wdep_func_dn4 = assign80130_e122694_d_n4;
        locals.var_wdep_func_dn5 = assign80130_e122694_d_n5;
        locals.var_wdep_func_dn6 = assign80130_e122694_d_n6;
        locals.var_wdep_func_dn7 = assign80130_e122694_d_n7;
        locals.var_wdep_func_dn8 = assign80130_e122694_d_n8;
        locals.var_wdep_func_dn9 = assign80130_e122694_d_n9;
        locals.var_wdep_func_dn10 = assign80130_e122694_d_n10;
        locals.var_wdep_func_dn13 = assign80130_e122694_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign80140_e122700, assign80140_e122700_d_n0, assign80140_e122700_d_n2, assign80140_e122700_d_n4, assign80140_e122700_d_n5, assign80140_e122700_d_n6, assign80140_e122700_d_n7, assign80140_e122700_d_n8, assign80140_e122700_d_n9, assign80140_e122700_d_n10, assign80140_e122700_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign80140_e122700;
        locals.var_cnst0over_func_dn0 = assign80140_e122700_d_n0;
        locals.var_cnst0over_func_dn2 = assign80140_e122700_d_n2;
        locals.var_cnst0over_func_dn4 = assign80140_e122700_d_n4;
        locals.var_cnst0over_func_dn5 = assign80140_e122700_d_n5;
        locals.var_cnst0over_func_dn6 = assign80140_e122700_d_n6;
        locals.var_cnst0over_func_dn7 = assign80140_e122700_d_n7;
        locals.var_cnst0over_func_dn8 = assign80140_e122700_d_n8;
        locals.var_cnst0over_func_dn9 = assign80140_e122700_d_n9;
        locals.var_cnst0over_func_dn10 = assign80140_e122700_d_n10;
        locals.var_cnst0over_func_dn13 = assign80140_e122700_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign80150_e122706,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign80150_e122706;
        locals.var_cox0_func_rv = 0.0;

        let assign80160_e122725: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1874 = assign80160_e122725;
        locals.var_guard1874_rv = 0.0;

        let (assign80170_e122734,) = {
    if (((locals.var_guard1870 != 0.0) && (locals.var_guard1869 == 0.0)) && (locals.var_guard1874 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80170_e122734;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign80180_e122745, assign80180_e122745_d_n2, assign80180_e122745_d_n6, assign80180_e122745_d_n7, assign80180_e122745_d_n8,) = {
    if (((locals.var_guard1870 != 0.0) && (locals.var_guard1869 == 0.0)) && (locals.var_guard1874 != 0.0)) {
        let assign80180_e122743: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign80180_e122743, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign80180_e122745;
        locals.var_vgbgmt_dn2 = assign80180_e122745_d_n2;
        locals.var_vgbgmt_dn6 = assign80180_e122745_d_n6;
        locals.var_vgbgmt_dn7 = assign80180_e122745_d_n7;
        locals.var_vgbgmt_dn8 = assign80180_e122745_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign80190_e122755, assign80190_e122755_d_n0, assign80190_e122755_d_n2, assign80190_e122755_d_n4, assign80190_e122755_d_n5, assign80190_e122755_d_n6, assign80190_e122755_d_n7, assign80190_e122755_d_n8, assign80190_e122755_d_n9, assign80190_e122755_d_n10, assign80190_e122755_d_n13,) = {
    if (((locals.var_guard1870 != 0.0) && (locals.var_guard1869 == 0.0)) && (locals.var_guard1874 != 0.0)) {
        let assign80190_e122753: f64 = (-locals.var_vbsei);
        (assign80190_e122753, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80190_e122755;
        locals.var_vxbgmt_dn0 = assign80190_e122755_d_n0;
        locals.var_vxbgmt_dn2 = assign80190_e122755_d_n2;
        locals.var_vxbgmt_dn4 = assign80190_e122755_d_n4;
        locals.var_vxbgmt_dn5 = assign80190_e122755_d_n5;
        locals.var_vxbgmt_dn6 = assign80190_e122755_d_n6;
        locals.var_vxbgmt_dn7 = assign80190_e122755_d_n7;
        locals.var_vxbgmt_dn8 = assign80190_e122755_d_n8;
        locals.var_vxbgmt_dn9 = assign80190_e122755_d_n9;
        locals.var_vxbgmt_dn10 = assign80190_e122755_d_n10;
        locals.var_vxbgmt_dn13 = assign80190_e122755_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let assign80200_e122766: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1875 = assign80200_e122766;
        locals.var_guard1875_rv = 0.0;

        let (assign80210_e122777,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80210_e122777;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign80220_e122788,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign80220_e122788;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign80230_e122801, assign80230_e122801_d_n2, assign80230_e122801_d_n6, assign80230_e122801_d_n7, assign80230_e122801_d_n8,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80230_e122799: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign80230_e122799, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign80230_e122801;
        locals.var_vgbgmt_dn2 = assign80230_e122801_d_n2;
        locals.var_vgbgmt_dn6 = assign80230_e122801_d_n6;
        locals.var_vgbgmt_dn7 = assign80230_e122801_d_n7;
        locals.var_vgbgmt_dn8 = assign80230_e122801_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign80240_e122814, assign80240_e122814_d_n0, assign80240_e122814_d_n2, assign80240_e122814_d_n4, assign80240_e122814_d_n5, assign80240_e122814_d_n6, assign80240_e122814_d_n7, assign80240_e122814_d_n8, assign80240_e122814_d_n9, assign80240_e122814_d_n10, assign80240_e122814_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80240_e122812: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign80240_e122812, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80240_e122814;
        locals.var_vxbgmt_dn0 = assign80240_e122814_d_n0;
        locals.var_vxbgmt_dn2 = assign80240_e122814_d_n2;
        locals.var_vxbgmt_dn4 = assign80240_e122814_d_n4;
        locals.var_vxbgmt_dn5 = assign80240_e122814_d_n5;
        locals.var_vxbgmt_dn6 = assign80240_e122814_d_n6;
        locals.var_vxbgmt_dn7 = assign80240_e122814_d_n7;
        locals.var_vxbgmt_dn8 = assign80240_e122814_d_n8;
        locals.var_vxbgmt_dn9 = assign80240_e122814_d_n9;
        locals.var_vxbgmt_dn10 = assign80240_e122814_d_n10;
        locals.var_vxbgmt_dn13 = assign80240_e122814_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign80250_e122825,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign80250_e122825;
        locals.var_nover_func_rv = 0.0;

        let (assign80260_e122840, assign80260_e122840_d_n0, assign80260_e122840_d_n2, assign80260_e122840_d_n4, assign80260_e122840_d_n5, assign80260_e122840_d_n6, assign80260_e122840_d_n7, assign80260_e122840_d_n8, assign80260_e122840_d_n9, assign80260_e122840_d_n10, assign80260_e122840_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80260_e122837: f64 = (p.p64 * p.p55);
        let assign80260_e122838: f64 = (p.p63 + assign80260_e122837);
        (assign80260_e122838, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80260_e122840;
        locals.var_lover_func_dn0 = assign80260_e122840_d_n0;
        locals.var_lover_func_dn2 = assign80260_e122840_d_n2;
        locals.var_lover_func_dn4 = assign80260_e122840_d_n4;
        locals.var_lover_func_dn5 = assign80260_e122840_d_n5;
        locals.var_lover_func_dn6 = assign80260_e122840_d_n6;
        locals.var_lover_func_dn7 = assign80260_e122840_d_n7;
        locals.var_lover_func_dn8 = assign80260_e122840_d_n8;
        locals.var_lover_func_dn9 = assign80260_e122840_d_n9;
        locals.var_lover_func_dn10 = assign80260_e122840_d_n10;
        locals.var_lover_func_dn13 = assign80260_e122840_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign80270_e122851, assign80270_e122851_d_n0, assign80270_e122851_d_n2, assign80270_e122851_d_n4, assign80270_e122851_d_n5, assign80270_e122851_d_n6, assign80270_e122851_d_n7, assign80270_e122851_d_n8, assign80270_e122851_d_n9, assign80270_e122851_d_n10, assign80270_e122851_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign80270_e122851;
        locals.var_wdep_func_dn0 = assign80270_e122851_d_n0;
        locals.var_wdep_func_dn2 = assign80270_e122851_d_n2;
        locals.var_wdep_func_dn4 = assign80270_e122851_d_n4;
        locals.var_wdep_func_dn5 = assign80270_e122851_d_n5;
        locals.var_wdep_func_dn6 = assign80270_e122851_d_n6;
        locals.var_wdep_func_dn7 = assign80270_e122851_d_n7;
        locals.var_wdep_func_dn8 = assign80270_e122851_d_n8;
        locals.var_wdep_func_dn9 = assign80270_e122851_d_n9;
        locals.var_wdep_func_dn10 = assign80270_e122851_d_n10;
        locals.var_wdep_func_dn13 = assign80270_e122851_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign80280_e122862, assign80280_e122862_d_n0, assign80280_e122862_d_n2, assign80280_e122862_d_n4, assign80280_e122862_d_n5, assign80280_e122862_d_n6, assign80280_e122862_d_n7, assign80280_e122862_d_n8, assign80280_e122862_d_n9, assign80280_e122862_d_n10, assign80280_e122862_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign80280_e122862;
        locals.var_cnst0over_func_dn0 = assign80280_e122862_d_n0;
        locals.var_cnst0over_func_dn2 = assign80280_e122862_d_n2;
        locals.var_cnst0over_func_dn4 = assign80280_e122862_d_n4;
        locals.var_cnst0over_func_dn5 = assign80280_e122862_d_n5;
        locals.var_cnst0over_func_dn6 = assign80280_e122862_d_n6;
        locals.var_cnst0over_func_dn7 = assign80280_e122862_d_n7;
        locals.var_cnst0over_func_dn8 = assign80280_e122862_d_n8;
        locals.var_cnst0over_func_dn9 = assign80280_e122862_d_n9;
        locals.var_cnst0over_func_dn10 = assign80280_e122862_d_n10;
        locals.var_cnst0over_func_dn13 = assign80280_e122862_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign80290_e122873,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign80290_e122873;
        locals.var_cox0_func_rv = 0.0;

        let (assign80300_e122885, assign80300_e122885_d_n0, assign80300_e122885_d_n2, assign80300_e122885_d_n4, assign80300_e122885_d_n5, assign80300_e122885_d_n6, assign80300_e122885_d_n7, assign80300_e122885_d_n8, assign80300_e122885_d_n9, assign80300_e122885_d_n10, assign80300_e122885_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80300_e122883: f64 = (-locals.var_lover_func);
        (assign80300_e122883, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80300_e122885;
        locals.var_lover_func_dn0 = assign80300_e122885_d_n0;
        locals.var_lover_func_dn2 = assign80300_e122885_d_n2;
        locals.var_lover_func_dn4 = assign80300_e122885_d_n4;
        locals.var_lover_func_dn5 = assign80300_e122885_d_n5;
        locals.var_lover_func_dn6 = assign80300_e122885_d_n6;
        locals.var_lover_func_dn7 = assign80300_e122885_d_n7;
        locals.var_lover_func_dn8 = assign80300_e122885_d_n8;
        locals.var_lover_func_dn9 = assign80300_e122885_d_n9;
        locals.var_lover_func_dn10 = assign80300_e122885_d_n10;
        locals.var_lover_func_dn13 = assign80300_e122885_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign80310_e122896: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1876 = assign80310_e122896;
        locals.var_guard1876_rv = 0.0;

        let (assign80320_e122910, assign80320_e122910_d_n0, assign80320_e122910_d_n2, assign80320_e122910_d_n4, assign80320_e122910_d_n5, assign80320_e122910_d_n6, assign80320_e122910_d_n7, assign80320_e122910_d_n8, assign80320_e122910_d_n9, assign80320_e122910_d_n10, assign80320_e122910_d_n13,) = {
    if ((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) {
        let assign80320_e122908: f64 = (-locals.var_lover_func);
        (assign80320_e122908, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80320_e122910;
        locals.var_lover_func_dn0 = assign80320_e122910_d_n0;
        locals.var_lover_func_dn2 = assign80320_e122910_d_n2;
        locals.var_lover_func_dn4 = assign80320_e122910_d_n4;
        locals.var_lover_func_dn5 = assign80320_e122910_d_n5;
        locals.var_lover_func_dn6 = assign80320_e122910_d_n6;
        locals.var_lover_func_dn7 = assign80320_e122910_d_n7;
        locals.var_lover_func_dn8 = assign80320_e122910_d_n8;
        locals.var_lover_func_dn9 = assign80320_e122910_d_n9;
        locals.var_lover_func_dn10 = assign80320_e122910_d_n10;
        locals.var_lover_func_dn13 = assign80320_e122910_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign80330_e122923, assign80330_e122923_d_n0, assign80330_e122923_d_n2, assign80330_e122923_d_n4, assign80330_e122923_d_n5, assign80330_e122923_d_n6, assign80330_e122923_d_n7, assign80330_e122923_d_n8, assign80330_e122923_d_n9, assign80330_e122923_d_n10, assign80330_e122923_d_n13,) = {
    if ((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign80330_e122923;
        locals.var_t1_dn0 = assign80330_e122923_d_n0;
        locals.var_t1_dn2 = assign80330_e122923_d_n2;
        locals.var_t1_dn4 = assign80330_e122923_d_n4;
        locals.var_t1_dn5 = assign80330_e122923_d_n5;
        locals.var_t1_dn6 = assign80330_e122923_d_n6;
        locals.var_t1_dn7 = assign80330_e122923_d_n7;
        locals.var_t1_dn8 = assign80330_e122923_d_n8;
        locals.var_t1_dn9 = assign80330_e122923_d_n9;
        locals.var_t1_dn10 = assign80330_e122923_d_n10;
        locals.var_t1_dn13 = assign80330_e122923_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign80340_e122942, assign80340_e122942_d_n0, assign80340_e122942_d_n2, assign80340_e122942_d_n4, assign80340_e122942_d_n5, assign80340_e122942_d_n6, assign80340_e122942_d_n7, assign80340_e122942_d_n8, assign80340_e122942_d_n9, assign80340_e122942_d_n10, assign80340_e122942_d_n13,) = {
    if ((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) {
        let assign80340_e122936: f64 = (locals.var_t1 * locals.var_t1);
        let assign80340_e122938: f64 = (assign80340_e122936 / locals.var_kjunc);
        let assign80340_e122940: f64 = (assign80340_e122938 - p.p137);
        (assign80340_e122940, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn13)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn13,)
    }
};
        locals.var_vxb_lim = assign80340_e122942;
        locals.var_vxb_lim_dn0 = assign80340_e122942_d_n0;
        locals.var_vxb_lim_dn2 = assign80340_e122942_d_n2;
        locals.var_vxb_lim_dn4 = assign80340_e122942_d_n4;
        locals.var_vxb_lim_dn5 = assign80340_e122942_d_n5;
        locals.var_vxb_lim_dn6 = assign80340_e122942_d_n6;
        locals.var_vxb_lim_dn7 = assign80340_e122942_d_n7;
        locals.var_vxb_lim_dn8 = assign80340_e122942_d_n8;
        locals.var_vxb_lim_dn9 = assign80340_e122942_d_n9;
        locals.var_vxb_lim_dn10 = assign80340_e122942_d_n10;
        locals.var_vxb_lim_dn13 = assign80340_e122942_d_n13;
        locals.var_vxb_lim_rv = 0.0;

        let assign80350_e122945: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1877 = assign80350_e122945;
        locals.var_guard1877_rv = 0.0;

        let assign80360_e122952: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1878 = assign80360_e122952;
        locals.var_guard1878_rv = 0.0;

        let (assign80370_e122969, assign80370_e122969_d_n0, assign80370_e122969_d_n2, assign80370_e122969_d_n4, assign80370_e122969_d_n5, assign80370_e122969_d_n6, assign80370_e122969_d_n7, assign80370_e122969_d_n8, assign80370_e122969_d_n9, assign80370_e122969_d_n10, assign80370_e122969_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80370_e122969;
        locals.var_vxbgmt_dn0 = assign80370_e122969_d_n0;
        locals.var_vxbgmt_dn2 = assign80370_e122969_d_n2;
        locals.var_vxbgmt_dn4 = assign80370_e122969_d_n4;
        locals.var_vxbgmt_dn5 = assign80370_e122969_d_n5;
        locals.var_vxbgmt_dn6 = assign80370_e122969_d_n6;
        locals.var_vxbgmt_dn7 = assign80370_e122969_d_n7;
        locals.var_vxbgmt_dn8 = assign80370_e122969_d_n8;
        locals.var_vxbgmt_dn9 = assign80370_e122969_d_n9;
        locals.var_vxbgmt_dn10 = assign80370_e122969_d_n10;
        locals.var_vxbgmt_dn13 = assign80370_e122969_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign80380_e122993, assign80380_e122993_d_n0, assign80380_e122993_d_n2, assign80380_e122993_d_n4, assign80380_e122993_d_n5, assign80380_e122993_d_n6, assign80380_e122993_d_n7, assign80380_e122993_d_n8, assign80380_e122993_d_n9, assign80380_e122993_d_n10, assign80380_e122993_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let (assign80380_e122991,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign80380_e122989: f64 = (-1.0);
                (assign80380_e122989,)
            } else {
                (1.0,)
            }
        };
        (assign80380_e122991, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign80380_e122993;
        locals.var_tmf3_dn0 = assign80380_e122993_d_n0;
        locals.var_tmf3_dn2 = assign80380_e122993_d_n2;
        locals.var_tmf3_dn4 = assign80380_e122993_d_n4;
        locals.var_tmf3_dn5 = assign80380_e122993_d_n5;
        locals.var_tmf3_dn6 = assign80380_e122993_d_n6;
        locals.var_tmf3_dn7 = assign80380_e122993_d_n7;
        locals.var_tmf3_dn8 = assign80380_e122993_d_n8;
        locals.var_tmf3_dn9 = assign80380_e122993_d_n9;
        locals.var_tmf3_dn10 = assign80380_e122993_d_n10;
        locals.var_tmf3_dn13 = assign80380_e122993_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign80390_e123013, assign80390_e123013_d_n0, assign80390_e123013_d_n2, assign80390_e123013_d_n4, assign80390_e123013_d_n5, assign80390_e123013_d_n6, assign80390_e123013_d_n7, assign80390_e123013_d_n8, assign80390_e123013_d_n9, assign80390_e123013_d_n10, assign80390_e123013_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let assign80390_e123011: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign80390_e123011, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn13 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign80390_e123013;
        locals.var_tmf4_dn0 = assign80390_e123013_d_n0;
        locals.var_tmf4_dn2 = assign80390_e123013_d_n2;
        locals.var_tmf4_dn4 = assign80390_e123013_d_n4;
        locals.var_tmf4_dn5 = assign80390_e123013_d_n5;
        locals.var_tmf4_dn6 = assign80390_e123013_d_n6;
        locals.var_tmf4_dn7 = assign80390_e123013_d_n7;
        locals.var_tmf4_dn8 = assign80390_e123013_d_n8;
        locals.var_tmf4_dn9 = assign80390_e123013_d_n9;
        locals.var_tmf4_dn10 = assign80390_e123013_d_n10;
        locals.var_tmf4_dn13 = assign80390_e123013_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign80400_e123037, assign80400_e123037_d_n0, assign80400_e123037_d_n2, assign80400_e123037_d_n4, assign80400_e123037_d_n5, assign80400_e123037_d_n6, assign80400_e123037_d_n7, assign80400_e123037_d_n8, assign80400_e123037_d_n9, assign80400_e123037_d_n10, assign80400_e123037_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let assign80400_e123032: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign80400_e123034: f64 = (assign80400_e123032).powf(p.p113);
        let assign80400_e123035: f64 = (1.0 + assign80400_e123034);
        (assign80400_e123035, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign80400_e123037;
        locals.var_tmf1_dn0 = assign80400_e123037_d_n0;
        locals.var_tmf1_dn2 = assign80400_e123037_d_n2;
        locals.var_tmf1_dn4 = assign80400_e123037_d_n4;
        locals.var_tmf1_dn5 = assign80400_e123037_d_n5;
        locals.var_tmf1_dn6 = assign80400_e123037_d_n6;
        locals.var_tmf1_dn7 = assign80400_e123037_d_n7;
        locals.var_tmf1_dn8 = assign80400_e123037_d_n8;
        locals.var_tmf1_dn9 = assign80400_e123037_d_n9;
        locals.var_tmf1_dn10 = assign80400_e123037_d_n10;
        locals.var_tmf1_dn13 = assign80400_e123037_d_n13;
        locals.var_tmf1_rv = 0.0;

    }
}
