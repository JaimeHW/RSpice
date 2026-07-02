#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6870_e4567, assign6870_e4567_d_n0, assign6870_e4567_d_n2, assign6870_e4567_d_n6, assign6870_e4567_d_n7, assign6870_e4567_d_n10, assign6870_e4567_d_n11, assign6870_e4567_d_n12, assign6870_e4567_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard110 == 0.0)) && (locals.var_guard111 == 0.0)) {
        let assign6870_e4553: f64 = (1.0 / 3.0);
        let assign6870_e4558: f64 = (locals.var_t3__blk107 * 0.148148111111111);
        let assign6870_e4559: f64 = (0.0402052934513951 + assign6870_e4558);
        let assign6870_e4560: f64 = (locals.var_t3__blk107 * assign6870_e4559);
        let assign6870_e4561: f64 = (assign6870_e4553 + assign6870_e4560);
        let assign6870_e4562: f64 = (locals.var_t3__blk107 * assign6870_e4561);
        let assign6870_e4563: f64 = (1.0 + assign6870_e4562);
        let assign6870_e4564: f64 = (locals.var_t3__blk107 * assign6870_e4563);
        let assign6870_e4565: f64 = (1.0 + assign6870_e4564);
        (assign6870_e4565, ((locals.var_t3__blk107_dn0 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn0 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn0 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn0 * 0.148148111111111))))))), ((locals.var_t3__blk107_dn2 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn2 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn2 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn2 * 0.148148111111111))))))), ((locals.var_t3__blk107_dn6 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn6 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn6 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn6 * 0.148148111111111))))))), ((locals.var_t3__blk107_dn7 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn7 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn7 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn7 * 0.148148111111111))))))), ((locals.var_t3__blk107_dn10 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn10 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn10 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn10 * 0.148148111111111))))))), ((locals.var_t3__blk107_dn11 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn11 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn11 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn11 * 0.148148111111111))))))), ((locals.var_t3__blk107_dn12 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn12 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn12 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn12 * 0.148148111111111))))))), ((locals.var_t3__blk107_dn17 * assign6870_e4563) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn17 * assign6870_e4561) + (locals.var_t3__blk107 * ((locals.var_t3__blk107_dn17 * assign6870_e4559) + (locals.var_t3__blk107 * (locals.var_t3__blk107_dn17 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6870_e4567;
        locals.var_dppg_dn0 = assign6870_e4567_d_n0;
        locals.var_dppg_dn2 = assign6870_e4567_d_n2;
        locals.var_dppg_dn6 = assign6870_e4567_d_n6;
        locals.var_dppg_dn7 = assign6870_e4567_d_n7;
        locals.var_dppg_dn10 = assign6870_e4567_d_n10;
        locals.var_dppg_dn11 = assign6870_e4567_d_n11;
        locals.var_dppg_dn12 = assign6870_e4567_d_n12;
        locals.var_dppg_dn17 = assign6870_e4567_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6880_e4585, assign6880_e4585_d_n0, assign6880_e4585_d_n2, assign6880_e4585_d_n6, assign6880_e4585_d_n7, assign6880_e4585_d_n10, assign6880_e4585_d_n11, assign6880_e4585_d_n12, assign6880_e4585_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6880_e4572: f64 = (locals.var_dppg - 1.0);
        let assign6880_e4575: f64 = (locals.var_dppg - 1.0);
        let assign6880_e4576: f64 = (assign6880_e4572 * assign6880_e4575);
        let assign6880_e4579: f64 = (4.0 * 0.1);
        let assign6880_e4581: f64 = (assign6880_e4579 * 0.1);
        let assign6880_e4582: f64 = (assign6880_e4576 + assign6880_e4581);
        let assign6880_e4583: f64 = (assign6880_e4582).sqrt();
        (assign6880_e4583, (((locals.var_dppg_dn0 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn0)) / (2.0 * assign6880_e4583)), (((locals.var_dppg_dn2 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn2)) / (2.0 * assign6880_e4583)), (((locals.var_dppg_dn6 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn6)) / (2.0 * assign6880_e4583)), (((locals.var_dppg_dn7 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn7)) / (2.0 * assign6880_e4583)), (((locals.var_dppg_dn10 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn10)) / (2.0 * assign6880_e4583)), (((locals.var_dppg_dn11 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn11)) / (2.0 * assign6880_e4583)), (((locals.var_dppg_dn12 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn12)) / (2.0 * assign6880_e4583)), (((locals.var_dppg_dn17 * assign6880_e4575) + (assign6880_e4572 * locals.var_dppg_dn17)) / (2.0 * assign6880_e4583)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6880_e4585;
        locals.var_tmf1_dn0 = assign6880_e4585_d_n0;
        locals.var_tmf1_dn2 = assign6880_e4585_d_n2;
        locals.var_tmf1_dn6 = assign6880_e4585_d_n6;
        locals.var_tmf1_dn7 = assign6880_e4585_d_n7;
        locals.var_tmf1_dn10 = assign6880_e4585_d_n10;
        locals.var_tmf1_dn11 = assign6880_e4585_d_n11;
        locals.var_tmf1_dn12 = assign6880_e4585_d_n12;
        locals.var_tmf1_dn17 = assign6880_e4585_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6890_e4600, assign6890_e4600_d_n0, assign6890_e4600_d_n2, assign6890_e4600_d_n6, assign6890_e4600_d_n7, assign6890_e4600_d_n10, assign6890_e4600_d_n11, assign6890_e4600_d_n12, assign6890_e4600_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6890_e4591: f64 = (locals.var_dppg - 1.0);
        let assign6890_e4593: f64 = (assign6890_e4591 + locals.var_tmf1);
        let assign6890_e4594: f64 = (0.5 * assign6890_e4593);
        let assign6890_e4597: f64 = (1e-10 * 0.1);
        let assign6890_e4598: f64 = (assign6890_e4594 + assign6890_e4597);
        (assign6890_e4598, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_dppg_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_dppg_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6890_e4600;
        locals.var_dppg_dn0 = assign6890_e4600_d_n0;
        locals.var_dppg_dn2 = assign6890_e4600_d_n2;
        locals.var_dppg_dn6 = assign6890_e4600_d_n6;
        locals.var_dppg_dn7 = assign6890_e4600_d_n7;
        locals.var_dppg_dn10 = assign6890_e4600_d_n10;
        locals.var_dppg_dn11 = assign6890_e4600_d_n11;
        locals.var_dppg_dn12 = assign6890_e4600_d_n12;
        locals.var_dppg_dn17 = assign6890_e4600_d_n17;
        locals.var_dppg_rv = 0.0;

        let assign6900_e4603: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6900_e4603;
        locals.var_guard112_rv = 0.0;

        let (assign6910_e4610, assign6910_e4610_d_n0, assign6910_e4610_d_n2, assign6910_e4610_d_n6, assign6910_e4610_d_n7, assign6910_e4610_d_n10, assign6910_e4610_d_n11, assign6910_e4610_d_n12, assign6910_e4610_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard112 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6910_e4610;
        locals.var_dppg_dn0 = assign6910_e4610_d_n0;
        locals.var_dppg_dn2 = assign6910_e4610_d_n2;
        locals.var_dppg_dn6 = assign6910_e4610_d_n6;
        locals.var_dppg_dn7 = assign6910_e4610_d_n7;
        locals.var_dppg_dn10 = assign6910_e4610_d_n10;
        locals.var_dppg_dn11 = assign6910_e4610_d_n11;
        locals.var_dppg_dn12 = assign6910_e4610_d_n12;
        locals.var_dppg_dn17 = assign6910_e4610_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6920_e4617, assign6920_e4617_d_n0, assign6920_e4617_d_n2, assign6920_e4617_d_n6, assign6920_e4617_d_n7, assign6920_e4617_d_n10, assign6920_e4617_d_n11, assign6920_e4617_d_n12, assign6920_e4617_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6920_e4615: f64 = (locals.var_dppg * locals.var_t0__blk106);
        (assign6920_e4615, (locals.var_dppg_dn0 * locals.var_t0__blk106), (locals.var_dppg_dn2 * locals.var_t0__blk106), (locals.var_dppg_dn6 * locals.var_t0__blk106), (locals.var_dppg_dn7 * locals.var_t0__blk106), (locals.var_dppg_dn10 * locals.var_t0__blk106), (locals.var_dppg_dn11 * locals.var_t0__blk106), (locals.var_dppg_dn12 * locals.var_t0__blk106), (locals.var_dppg_dn17 * locals.var_t0__blk106),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6920_e4617;
        locals.var_dppg_dn0 = assign6920_e4617_d_n0;
        locals.var_dppg_dn2 = assign6920_e4617_d_n2;
        locals.var_dppg_dn6 = assign6920_e4617_d_n6;
        locals.var_dppg_dn7 = assign6920_e4617_d_n7;
        locals.var_dppg_dn10 = assign6920_e4617_d_n10;
        locals.var_dppg_dn11 = assign6920_e4617_d_n11;
        locals.var_dppg_dn12 = assign6920_e4617_d_n12;
        locals.var_dppg_dn17 = assign6920_e4617_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6930_e4626, assign6930_e4626_d_n0, assign6930_e4626_d_n2, assign6930_e4626_d_n6, assign6930_e4626_d_n7, assign6930_e4626_d_n10, assign6930_e4626_d_n11, assign6930_e4626_d_n12, assign6930_e4626_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6930_e4622: f64 = (1.0 - locals.var_dppg);
        let assign6930_e4624: f64 = (assign6930_e4622 - 0.05);
        (assign6930_e4624, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn12), (-locals.var_dppg_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6930_e4626;
        locals.var_tmf1_dn0 = assign6930_e4626_d_n0;
        locals.var_tmf1_dn2 = assign6930_e4626_d_n2;
        locals.var_tmf1_dn6 = assign6930_e4626_d_n6;
        locals.var_tmf1_dn7 = assign6930_e4626_d_n7;
        locals.var_tmf1_dn10 = assign6930_e4626_d_n10;
        locals.var_tmf1_dn11 = assign6930_e4626_d_n11;
        locals.var_tmf1_dn12 = assign6930_e4626_d_n12;
        locals.var_tmf1_dn17 = assign6930_e4626_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6940_e4635, assign6940_e4635_d_n0, assign6940_e4635_d_n2, assign6940_e4635_d_n6, assign6940_e4635_d_n7, assign6940_e4635_d_n10, assign6940_e4635_d_n11, assign6940_e4635_d_n12, assign6940_e4635_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6940_e4631: f64 = 4.0;
        let assign6940_e4633: f64 = (assign6940_e4631 * 0.05);
        (assign6940_e4633, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6940_e4635;
        locals.var_tmf2_dn0 = assign6940_e4635_d_n0;
        locals.var_tmf2_dn2 = assign6940_e4635_d_n2;
        locals.var_tmf2_dn6 = assign6940_e4635_d_n6;
        locals.var_tmf2_dn7 = assign6940_e4635_d_n7;
        locals.var_tmf2_dn10 = assign6940_e4635_d_n10;
        locals.var_tmf2_dn11 = assign6940_e4635_d_n11;
        locals.var_tmf2_dn12 = assign6940_e4635_d_n12;
        locals.var_tmf2_dn17 = assign6940_e4635_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6950_e4646, assign6950_e4646_d_n0, assign6950_e4646_d_n2, assign6950_e4646_d_n6, assign6950_e4646_d_n7, assign6950_e4646_d_n10, assign6950_e4646_d_n11, assign6950_e4646_d_n12, assign6950_e4646_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let (assign6950_e4644, assign6950_e4644_d_n0, assign6950_e4644_d_n2, assign6950_e4644_d_n6, assign6950_e4644_d_n7, assign6950_e4644_d_n10, assign6950_e4644_d_n11, assign6950_e4644_d_n12, assign6950_e4644_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6950_e4643: f64 = (-locals.var_tmf2);
                (assign6950_e4643, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6950_e4644, assign6950_e4644_d_n0, assign6950_e4644_d_n2, assign6950_e4644_d_n6, assign6950_e4644_d_n7, assign6950_e4644_d_n10, assign6950_e4644_d_n11, assign6950_e4644_d_n12, assign6950_e4644_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6950_e4646;
        locals.var_tmf2_dn0 = assign6950_e4646_d_n0;
        locals.var_tmf2_dn2 = assign6950_e4646_d_n2;
        locals.var_tmf2_dn6 = assign6950_e4646_d_n6;
        locals.var_tmf2_dn7 = assign6950_e4646_d_n7;
        locals.var_tmf2_dn10 = assign6950_e4646_d_n10;
        locals.var_tmf2_dn11 = assign6950_e4646_d_n11;
        locals.var_tmf2_dn12 = assign6950_e4646_d_n12;
        locals.var_tmf2_dn17 = assign6950_e4646_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6960_e4656, assign6960_e4656_d_n0, assign6960_e4656_d_n2, assign6960_e4656_d_n6, assign6960_e4656_d_n7, assign6960_e4656_d_n10, assign6960_e4656_d_n11, assign6960_e4656_d_n12, assign6960_e4656_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6960_e4651: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6960_e4653: f64 = (assign6960_e4651 + locals.var_tmf2);
        let assign6960_e4654: f64 = (assign6960_e4653).sqrt();
        (assign6960_e4654, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6960_e4654)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6960_e4654)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6960_e4654)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6960_e4654)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6960_e4654)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6960_e4654)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6960_e4654)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6960_e4654)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6960_e4656;
        locals.var_tmf2_dn0 = assign6960_e4656_d_n0;
        locals.var_tmf2_dn2 = assign6960_e4656_d_n2;
        locals.var_tmf2_dn6 = assign6960_e4656_d_n6;
        locals.var_tmf2_dn7 = assign6960_e4656_d_n7;
        locals.var_tmf2_dn10 = assign6960_e4656_d_n10;
        locals.var_tmf2_dn11 = assign6960_e4656_d_n11;
        locals.var_tmf2_dn12 = assign6960_e4656_d_n12;
        locals.var_tmf2_dn17 = assign6960_e4656_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6970_e4667, assign6970_e4667_d_n0, assign6970_e4667_d_n2, assign6970_e4667_d_n6, assign6970_e4667_d_n7, assign6970_e4667_d_n10, assign6970_e4667_d_n11, assign6970_e4667_d_n12, assign6970_e4667_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6970_e4663: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6970_e4664: f64 = (0.5 * assign6970_e4663);
        let assign6970_e4665: f64 = (1.0 - assign6970_e4664);
        (assign6970_e4665, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6970_e4667;
        locals.var_dppg_dn0 = assign6970_e4667_d_n0;
        locals.var_dppg_dn2 = assign6970_e4667_d_n2;
        locals.var_dppg_dn6 = assign6970_e4667_d_n6;
        locals.var_dppg_dn7 = assign6970_e4667_d_n7;
        locals.var_dppg_dn10 = assign6970_e4667_d_n10;
        locals.var_dppg_dn11 = assign6970_e4667_d_n11;
        locals.var_dppg_dn12 = assign6970_e4667_d_n12;
        locals.var_dppg_dn17 = assign6970_e4667_d_n17;
        locals.var_dppg_rv = 0.0;

        let assign6980_e4670: f64 = (locals.var_vgs - locals.var_vfb);
        let assign6980_e4672: f64 = (assign6980_e4670 + locals.var_dvth);
        let assign6980_e4674: f64 = (assign6980_e4672 - locals.var_dppg);
        locals.var_vgp = assign6980_e4674;
        locals.var_vgp_dn0 = (locals.var_dvth_dn0 - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (locals.var_dvth_dn2 - locals.var_dppg_dn2);
        locals.var_vgp_dn6 = ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn7 = ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7);
        locals.var_vgp_dn10 = (locals.var_dvth_dn10 - locals.var_dppg_dn10);
        locals.var_vgp_dn11 = ((locals.var_vgs_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11);
        locals.var_vgp_dn12 = (locals.var_dvth_dn12 - locals.var_dppg_dn12);
        locals.var_vgp_dn17 = (locals.var_dvth_dn17 - locals.var_dppg_dn17);
        locals.var_vgp_rv = 0.0;

        locals.var_vgpz = locals.var_vgp;
        locals.var_vgpz_dn0 = locals.var_vgp_dn0;
        locals.var_vgpz_dn2 = locals.var_vgp_dn2;
        locals.var_vgpz_dn6 = locals.var_vgp_dn6;
        locals.var_vgpz_dn7 = locals.var_vgp_dn7;
        locals.var_vgpz_dn10 = locals.var_vgp_dn10;
        locals.var_vgpz_dn11 = locals.var_vgp_dn11;
        locals.var_vgpz_dn12 = locals.var_vgp_dn12;
        locals.var_vgpz_dn17 = locals.var_vgp_dn17;
        locals.var_vgpz_rv = 0.0;

        let assign7000_e4678: f64 = (locals.var_uc_nsubs / locals.var_mks_nsubb);
        let assign7000_e4679: f64 = (assign7000_e4678).ln();
        locals.var_t1 = assign7000_e4679;
        locals.var_t1_dn0 = ((locals.var_uc_nsubs_dn0 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_dn2 = ((locals.var_uc_nsubs_dn2 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_dn6 = ((locals.var_uc_nsubs_dn6 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_dn7 = ((locals.var_uc_nsubs_dn7 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_dn10 = ((locals.var_uc_nsubs_dn10 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_dn11 = ((locals.var_uc_nsubs_dn11 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_dn12 = ((locals.var_uc_nsubs_dn12 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_dn17 = ((locals.var_uc_nsubs_dn17 / locals.var_mks_nsubb) / assign7000_e4678);
        locals.var_t1_rv = 0.0;

        let assign7010_e4682: f64 = (locals.var_beta_inv * locals.var_t1);
        locals.var_vbi_soi = assign7010_e4682;
        locals.var_vbi_soi_dn0 = (locals.var_beta_inv * locals.var_t1_dn0);
        locals.var_vbi_soi_dn2 = (locals.var_beta_inv * locals.var_t1_dn2);
        locals.var_vbi_soi_dn6 = (locals.var_beta_inv * locals.var_t1_dn6);
        locals.var_vbi_soi_dn7 = (locals.var_beta_inv * locals.var_t1_dn7);
        locals.var_vbi_soi_dn10 = ((locals.var_beta_inv_dn10 * locals.var_t1) + (locals.var_beta_inv * locals.var_t1_dn10));
        locals.var_vbi_soi_dn11 = (locals.var_beta_inv * locals.var_t1_dn11);
        locals.var_vbi_soi_dn12 = (locals.var_beta_inv * locals.var_t1_dn12);
        locals.var_vbi_soi_dn17 = (locals.var_beta_inv * locals.var_t1_dn17);
        locals.var_vbi_soi_rv = 0.0;

        let assign7020_e4685: f64 = (locals.var_vfb - locals.var_dvth);
        let assign7020_e4687: f64 = (assign7020_e4685 + locals.var_dppg);
        locals.var_vgs_fb = assign7020_e4687;
        locals.var_vgs_fb_dn0 = ((-locals.var_dvth_dn0) + locals.var_dppg_dn0);
        locals.var_vgs_fb_dn2 = ((-locals.var_dvth_dn2) + locals.var_dppg_dn2);
        locals.var_vgs_fb_dn6 = ((-locals.var_dvth_dn6) + locals.var_dppg_dn6);
        locals.var_vgs_fb_dn7 = ((-locals.var_dvth_dn7) + locals.var_dppg_dn7);
        locals.var_vgs_fb_dn10 = ((-locals.var_dvth_dn10) + locals.var_dppg_dn10);
        locals.var_vgs_fb_dn11 = ((-locals.var_dvth_dn11) + locals.var_dppg_dn11);
        locals.var_vgs_fb_dn12 = ((-locals.var_dvth_dn12) + locals.var_dppg_dn12);
        locals.var_vgs_fb_dn17 = ((-locals.var_dvth_dn17) + locals.var_dppg_dn17);
        locals.var_vgs_fb_rv = 0.0;

        let assign7030_e4690: f64 = (locals.var_cnst0soi * locals.var_c_fox_inv);
        locals.var_fac1 = assign7030_e4690;
        locals.var_fac1_dn0 = ((locals.var_cnst0soi_dn0 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0soi_dn2 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn2));
        locals.var_fac1_dn6 = ((locals.var_cnst0soi_dn6 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0soi_dn7 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn7));
        locals.var_fac1_dn10 = ((locals.var_cnst0soi_dn10 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0soi_dn11 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn11));
        locals.var_fac1_dn12 = ((locals.var_cnst0soi_dn12 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn12));
        locals.var_fac1_dn17 = ((locals.var_cnst0soi_dn17 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn17));
        locals.var_fac1_rv = 0.0;

        let assign7040_e4693: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign7040_e4693;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn12 = ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12));
        locals.var_fac1p2_dn17 = ((locals.var_fac1_dn17 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn17));
        locals.var_fac1p2_rv = 0.0;

        let assign7050_e4696: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign7050_e4696;
        locals.var_guard113_rv = 0.0;

        let (assign7060_e4700,) = {
    if (locals.var_guard113 != 0.0) {
        (7.0,)
    } else {
        (locals.var_qdepb_dlt,)
    }
};
        locals.var_qdepb_dlt = assign7060_e4700;
        locals.var_qdepb_dlt_rv = 0.0;

        let (assign7070_e4706, assign7070_e4706_d_n0, assign7070_e4706_d_n2, assign7070_e4706_d_n6, assign7070_e4706_d_n7, assign7070_e4706_d_n10, assign7070_e4706_d_n11, assign7070_e4706_d_n12, assign7070_e4706_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7070_e4704: f64 = (locals.var_pb2 + 1.0);
        (assign7070_e4704, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    } else {
        (locals.var_vgp_ini, locals.var_vgp_ini_dn0, locals.var_vgp_ini_dn2, locals.var_vgp_ini_dn6, locals.var_vgp_ini_dn7, locals.var_vgp_ini_dn10, locals.var_vgp_ini_dn11, locals.var_vgp_ini_dn12, locals.var_vgp_ini_dn17,)
    }
};
        locals.var_vgp_ini = assign7070_e4706;
        locals.var_vgp_ini_dn0 = assign7070_e4706_d_n0;
        locals.var_vgp_ini_dn2 = assign7070_e4706_d_n2;
        locals.var_vgp_ini_dn6 = assign7070_e4706_d_n6;
        locals.var_vgp_ini_dn7 = assign7070_e4706_d_n7;
        locals.var_vgp_ini_dn10 = assign7070_e4706_d_n10;
        locals.var_vgp_ini_dn11 = assign7070_e4706_d_n11;
        locals.var_vgp_ini_dn12 = assign7070_e4706_d_n12;
        locals.var_vgp_ini_dn17 = assign7070_e4706_d_n17;
        locals.var_vgp_ini_rv = 0.0;

        let (assign7080_e4714, assign7080_e4714_d_n0, assign7080_e4714_d_n2, assign7080_e4714_d_n6, assign7080_e4714_d_n7, assign7080_e4714_d_n10, assign7080_e4714_d_n11, assign7080_e4714_d_n12, assign7080_e4714_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7080_e4710: f64 = (1.0 / locals.var_cnst1soi);
        let assign7080_e4712: f64 = (assign7080_e4710 / locals.var_cnstc_foxi);
        (assign7080_e4712, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7080_e4710 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign7080_e4714;
        locals.var_t1_dn0 = assign7080_e4714_d_n0;
        locals.var_t1_dn2 = assign7080_e4714_d_n2;
        locals.var_t1_dn6 = assign7080_e4714_d_n6;
        locals.var_t1_dn7 = assign7080_e4714_d_n7;
        locals.var_t1_dn10 = assign7080_e4714_d_n10;
        locals.var_t1_dn11 = assign7080_e4714_d_n11;
        locals.var_t1_dn12 = assign7080_e4714_d_n12;
        locals.var_t1_dn17 = assign7080_e4714_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign7090_e4726, assign7090_e4726_d_n0, assign7090_e4726_d_n2, assign7090_e4726_d_n6, assign7090_e4726_d_n7, assign7090_e4726_d_n10, assign7090_e4726_d_n11, assign7090_e4726_d_n12, assign7090_e4726_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7090_e4719: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7090_e4720: f64 = (locals.var_t1 * assign7090_e4719);
        let assign7090_e4723: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7090_e4724: f64 = (assign7090_e4720 * assign7090_e4723);
        (assign7090_e4724, ((((locals.var_t1_dn0 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign7090_e4719) + (locals.var_t1 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))) * assign7090_e4723) + (assign7090_e4720 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign7090_e4726;
        locals.var_t2_dn0 = assign7090_e4726_d_n0;
        locals.var_t2_dn2 = assign7090_e4726_d_n2;
        locals.var_t2_dn6 = assign7090_e4726_d_n6;
        locals.var_t2_dn7 = assign7090_e4726_d_n7;
        locals.var_t2_dn10 = assign7090_e4726_d_n10;
        locals.var_t2_dn11 = assign7090_e4726_d_n11;
        locals.var_t2_dn12 = assign7090_e4726_d_n12;
        locals.var_t2_dn17 = assign7090_e4726_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign7100_e4736, assign7100_e4736_d_n0, assign7100_e4736_d_n2, assign7100_e4736_d_n6, assign7100_e4736_d_n7, assign7100_e4736_d_n10, assign7100_e4736_d_n11, assign7100_e4736_d_n12, assign7100_e4736_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7100_e4732: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7100_e4733: f64 = (2.0 / assign7100_e4732);
        let assign7100_e4734: f64 = (locals.var_beta + assign7100_e4733);
        (assign7100_e4734, (-((2.0 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0)) / (assign7100_e4732 * assign7100_e4732))), (-((2.0 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2)) / (assign7100_e4732 * assign7100_e4732))), (-((2.0 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6)) / (assign7100_e4732 * assign7100_e4732))), (-((2.0 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7)) / (assign7100_e4732 * assign7100_e4732))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10)) / (assign7100_e4732 * assign7100_e4732)))), (-((2.0 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11)) / (assign7100_e4732 * assign7100_e4732))), (-((2.0 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12)) / (assign7100_e4732 * assign7100_e4732))), (-((2.0 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17)) / (assign7100_e4732 * assign7100_e4732))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign7100_e4736;
        locals.var_t3_dn0 = assign7100_e4736_d_n0;
        locals.var_t3_dn2 = assign7100_e4736_d_n2;
        locals.var_t3_dn6 = assign7100_e4736_d_n6;
        locals.var_t3_dn7 = assign7100_e4736_d_n7;
        locals.var_t3_dn10 = assign7100_e4736_d_n10;
        locals.var_t3_dn11 = assign7100_e4736_d_n11;
        locals.var_t3_dn12 = assign7100_e4736_d_n12;
        locals.var_t3_dn17 = assign7100_e4736_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign7110_e4743, assign7110_e4743_d_n0, assign7110_e4743_d_n2, assign7110_e4743_d_n6, assign7110_e4743_d_n7, assign7110_e4743_d_n10, assign7110_e4743_d_n11, assign7110_e4743_d_n12, assign7110_e4743_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7110_e4739: f64 = (locals.var_t2).ln();
        let assign7110_e4741: f64 = (assign7110_e4739 / locals.var_t3);
        (assign7110_e4741, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign7110_e4739 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inic, locals.var_ps0_inic_dn0, locals.var_ps0_inic_dn2, locals.var_ps0_inic_dn6, locals.var_ps0_inic_dn7, locals.var_ps0_inic_dn10, locals.var_ps0_inic_dn11, locals.var_ps0_inic_dn12, locals.var_ps0_inic_dn17,)
    }
};
        locals.var_ps0_inic = assign7110_e4743;
        locals.var_ps0_inic_dn0 = assign7110_e4743_d_n0;
        locals.var_ps0_inic_dn2 = assign7110_e4743_d_n2;
        locals.var_ps0_inic_dn6 = assign7110_e4743_d_n6;
        locals.var_ps0_inic_dn7 = assign7110_e4743_d_n7;
        locals.var_ps0_inic_dn10 = assign7110_e4743_d_n10;
        locals.var_ps0_inic_dn11 = assign7110_e4743_d_n11;
        locals.var_ps0_inic_dn12 = assign7110_e4743_d_n12;
        locals.var_ps0_inic_dn17 = assign7110_e4743_d_n17;
        locals.var_ps0_inic_rv = 0.0;

        let (assign7120_e4750, assign7120_e4750_d_n0, assign7120_e4750_d_n2, assign7120_e4750_d_n6, assign7120_e4750_d_n7, assign7120_e4750_d_n10, assign7120_e4750_d_n11, assign7120_e4750_d_n12, assign7120_e4750_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7120_e4747: f64 = (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic);
        let assign7120_e4748: f64 = (assign7120_e4747).sqrt();
        (assign7120_e4748, (((locals.var_cnst_2esi_q_nsubs_dn0 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn0)) / (2.0 * assign7120_e4748)), (((locals.var_cnst_2esi_q_nsubs_dn2 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn2)) / (2.0 * assign7120_e4748)), (((locals.var_cnst_2esi_q_nsubs_dn6 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn6)) / (2.0 * assign7120_e4748)), (((locals.var_cnst_2esi_q_nsubs_dn7 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn7)) / (2.0 * assign7120_e4748)), (((locals.var_cnst_2esi_q_nsubs_dn10 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn10)) / (2.0 * assign7120_e4748)), (((locals.var_cnst_2esi_q_nsubs_dn11 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn11)) / (2.0 * assign7120_e4748)), (((locals.var_cnst_2esi_q_nsubs_dn12 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn12)) / (2.0 * assign7120_e4748)), (((locals.var_cnst_2esi_q_nsubs_dn17 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn17)) / (2.0 * assign7120_e4748)),)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7120_e4750;
        locals.var_wdsoi_ini0_dn0 = assign7120_e4750_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7120_e4750_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7120_e4750_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7120_e4750_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7120_e4750_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7120_e4750_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7120_e4750_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7120_e4750_d_n17;
        locals.var_wdsoi_ini0_rv = 0.0;

        let (assign7130_e4759, assign7130_e4759_d_n0, assign7130_e4759_d_n2, assign7130_e4759_d_n6, assign7130_e4759_d_n7, assign7130_e4759_d_n10, assign7130_e4759_d_n11, assign7130_e4759_d_n12, assign7130_e4759_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let (assign7130_e4757, assign7130_e4757_d_n0, assign7130_e4757_d_n2, assign7130_e4757_d_n6, assign7130_e4757_d_n7, assign7130_e4757_d_n10, assign7130_e4757_d_n11, assign7130_e4757_d_n12, assign7130_e4757_d_n17,) = {
            if (locals.var_wdsoi_ini0 > p.p237) {
                (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
            }
        };
        (assign7130_e4757, assign7130_e4757_d_n0, assign7130_e4757_d_n2, assign7130_e4757_d_n6, assign7130_e4757_d_n7, assign7130_e4757_d_n10, assign7130_e4757_d_n11, assign7130_e4757_d_n12, assign7130_e4757_d_n17,)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7130_e4759;
        locals.var_wdsoi_ini0_dn0 = assign7130_e4759_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7130_e4759_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7130_e4759_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7130_e4759_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7130_e4759_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7130_e4759_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7130_e4759_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7130_e4759_d_n17;
        locals.var_wdsoi_ini0_rv = 0.0;

        let (assign7140_e4768, assign7140_e4768_d_n0, assign7140_e4768_d_n2, assign7140_e4768_d_n6, assign7140_e4768_d_n7, assign7140_e4768_d_n10, assign7140_e4768_d_n11, assign7140_e4768_d_n12, assign7140_e4768_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7140_e4762: f64 = (-1.6021918e-19);
        let assign7140_e4764: f64 = (assign7140_e4762 * locals.var_uc_nsubs);
        let assign7140_e4766: f64 = (assign7140_e4764 * locals.var_wdsoi_ini0);
        (assign7140_e4766, (((assign7140_e4762 * locals.var_uc_nsubs_dn0) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn0)), (((assign7140_e4762 * locals.var_uc_nsubs_dn2) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn2)), (((assign7140_e4762 * locals.var_uc_nsubs_dn6) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn6)), (((assign7140_e4762 * locals.var_uc_nsubs_dn7) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn7)), (((assign7140_e4762 * locals.var_uc_nsubs_dn10) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn10)), (((assign7140_e4762 * locals.var_uc_nsubs_dn11) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn11)), (((assign7140_e4762 * locals.var_uc_nsubs_dn12) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn12)), (((assign7140_e4762 * locals.var_uc_nsubs_dn17) * locals.var_wdsoi_ini0) + (assign7140_e4764 * locals.var_wdsoi_ini0_dn17)),)
    } else {
        (locals.var_q_wdsoi_max, locals.var_q_wdsoi_max_dn0, locals.var_q_wdsoi_max_dn2, locals.var_q_wdsoi_max_dn6, locals.var_q_wdsoi_max_dn7, locals.var_q_wdsoi_max_dn10, locals.var_q_wdsoi_max_dn11, locals.var_q_wdsoi_max_dn12, locals.var_q_wdsoi_max_dn17,)
    }
};
        locals.var_q_wdsoi_max = assign7140_e4768;
        locals.var_q_wdsoi_max_dn0 = assign7140_e4768_d_n0;
        locals.var_q_wdsoi_max_dn2 = assign7140_e4768_d_n2;
        locals.var_q_wdsoi_max_dn6 = assign7140_e4768_d_n6;
        locals.var_q_wdsoi_max_dn7 = assign7140_e4768_d_n7;
        locals.var_q_wdsoi_max_dn10 = assign7140_e4768_d_n10;
        locals.var_q_wdsoi_max_dn11 = assign7140_e4768_d_n11;
        locals.var_q_wdsoi_max_dn12 = assign7140_e4768_d_n12;
        locals.var_q_wdsoi_max_dn17 = assign7140_e4768_d_n17;
        locals.var_q_wdsoi_max_rv = 0.0;

        let (assign7150_e4772,) = {
    if (locals.var_guard113 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_t_soi,)
    }
};
        locals.var_t_soi = assign7150_e4772;
        locals.var_t_soi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7160_e4781, assign7160_e4781_d_n0, assign7160_e4781_d_n2, assign7160_e4781_d_n6, assign7160_e4781_d_n7, assign7160_e4781_d_n10, assign7160_e4781_d_n11, assign7160_e4781_d_n12, assign7160_e4781_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7160_e4775: f64 = (-1.6021918e-19);
        let assign7160_e4777: f64 = (assign7160_e4775 * locals.var_uc_nsubs);
        let assign7160_e4779: f64 = (assign7160_e4777 * locals.var_t_soi);
        (assign7160_e4779, ((assign7160_e4775 * locals.var_uc_nsubs_dn0) * locals.var_t_soi), ((assign7160_e4775 * locals.var_uc_nsubs_dn2) * locals.var_t_soi), ((assign7160_e4775 * locals.var_uc_nsubs_dn6) * locals.var_t_soi), ((assign7160_e4775 * locals.var_uc_nsubs_dn7) * locals.var_t_soi), ((assign7160_e4775 * locals.var_uc_nsubs_dn10) * locals.var_t_soi), ((assign7160_e4775 * locals.var_uc_nsubs_dn11) * locals.var_t_soi), ((assign7160_e4775 * locals.var_uc_nsubs_dn12) * locals.var_t_soi), ((assign7160_e4775 * locals.var_uc_nsubs_dn17) * locals.var_t_soi),)
    } else {
        (locals.var_q_fd_soi, locals.var_q_fd_soi_dn0, locals.var_q_fd_soi_dn2, locals.var_q_fd_soi_dn6, locals.var_q_fd_soi_dn7, locals.var_q_fd_soi_dn10, locals.var_q_fd_soi_dn11, locals.var_q_fd_soi_dn12, locals.var_q_fd_soi_dn17,)
    }
};
        locals.var_q_fd_soi = assign7160_e4781;
        locals.var_q_fd_soi_dn0 = assign7160_e4781_d_n0;
        locals.var_q_fd_soi_dn2 = assign7160_e4781_d_n2;
        locals.var_q_fd_soi_dn6 = assign7160_e4781_d_n6;
        locals.var_q_fd_soi_dn7 = assign7160_e4781_d_n7;
        locals.var_q_fd_soi_dn10 = assign7160_e4781_d_n10;
        locals.var_q_fd_soi_dn11 = assign7160_e4781_d_n11;
        locals.var_q_fd_soi_dn12 = assign7160_e4781_d_n12;
        locals.var_q_fd_soi_dn17 = assign7160_e4781_d_n17;
        locals.var_q_fd_soi_rv = 0.0;

        let (assign7170_e4785,) = {
    if (locals.var_guard113 != 0.0) {
        (1.5,)
    } else {
        (locals.var_wdsoi_ini1_dlt,)
    }
};
        locals.var_wdsoi_ini1_dlt = assign7170_e4785;
        locals.var_wdsoi_ini1_dlt_rv = 0.0;

        let (assign7180_e4791,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7180_e4789: f64 = (1.034943e-10 / locals.var_t_soi);
        (assign7180_e4789,)
    } else {
        (locals.var_c_soi__blk114,)
    }
};
        locals.var_c_soi__blk114 = assign7180_e4791;
        locals.var_c_soi__blk114_rv = 0.0;

        let (assign7190_e4797,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7190_e4795: f64 = (1.0 / locals.var_c_soi__blk114);
        (assign7190_e4795,)
    } else {
        (locals.var_c_soi_inv__blk115,)
    }
};
        locals.var_c_soi_inv__blk115 = assign7190_e4797;
        locals.var_c_soi_inv__blk115_rv = 0.0;

        let (assign7200_e4804, assign7200_e4804_d_n0, assign7200_e4804_d_n2, assign7200_e4804_d_n6, assign7200_e4804_d_n7, assign7200_e4804_d_n10, assign7200_e4804_d_n11, assign7200_e4804_d_n12, assign7200_e4804_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7200_e4800: f64 = (-locals.var_q_fd_soi);
        let assign7200_e4802: f64 = (assign7200_e4800 * 0.001);
        (assign7200_e4802, ((-locals.var_q_fd_soi_dn0) * 0.001), ((-locals.var_q_fd_soi_dn2) * 0.001), ((-locals.var_q_fd_soi_dn6) * 0.001), ((-locals.var_q_fd_soi_dn7) * 0.001), ((-locals.var_q_fd_soi_dn10) * 0.001), ((-locals.var_q_fd_soi_dn11) * 0.001), ((-locals.var_q_fd_soi_dn12) * 0.001), ((-locals.var_q_fd_soi_dn17) * 0.001),)
    } else {
        (locals.var_q_fd_dlt1, locals.var_q_fd_dlt1_dn0, locals.var_q_fd_dlt1_dn2, locals.var_q_fd_dlt1_dn6, locals.var_q_fd_dlt1_dn7, locals.var_q_fd_dlt1_dn10, locals.var_q_fd_dlt1_dn11, locals.var_q_fd_dlt1_dn12, locals.var_q_fd_dlt1_dn17,)
    }
};
        locals.var_q_fd_dlt1 = assign7200_e4804;
        locals.var_q_fd_dlt1_dn0 = assign7200_e4804_d_n0;
        locals.var_q_fd_dlt1_dn2 = assign7200_e4804_d_n2;
        locals.var_q_fd_dlt1_dn6 = assign7200_e4804_d_n6;
        locals.var_q_fd_dlt1_dn7 = assign7200_e4804_d_n7;
        locals.var_q_fd_dlt1_dn10 = assign7200_e4804_d_n10;
        locals.var_q_fd_dlt1_dn11 = assign7200_e4804_d_n11;
        locals.var_q_fd_dlt1_dn12 = assign7200_e4804_d_n12;
        locals.var_q_fd_dlt1_dn17 = assign7200_e4804_d_n17;
        locals.var_q_fd_dlt1_rv = 0.0;

        let (assign7210_e4811, assign7210_e4811_d_n0, assign7210_e4811_d_n2, assign7210_e4811_d_n6, assign7210_e4811_d_n7, assign7210_e4811_d_n10, assign7210_e4811_d_n11, assign7210_e4811_d_n12, assign7210_e4811_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7210_e4807: f64 = (-locals.var_q_fd_soi);
        let assign7210_e4809: f64 = (assign7210_e4807 * 1e-5);
        (assign7210_e4809, ((-locals.var_q_fd_soi_dn0) * 1e-5), ((-locals.var_q_fd_soi_dn2) * 1e-5), ((-locals.var_q_fd_soi_dn6) * 1e-5), ((-locals.var_q_fd_soi_dn7) * 1e-5), ((-locals.var_q_fd_soi_dn10) * 1e-5), ((-locals.var_q_fd_soi_dn11) * 1e-5), ((-locals.var_q_fd_soi_dn12) * 1e-5), ((-locals.var_q_fd_soi_dn17) * 1e-5),)
    } else {
        (locals.var_q_fd_dlt2, locals.var_q_fd_dlt2_dn0, locals.var_q_fd_dlt2_dn2, locals.var_q_fd_dlt2_dn6, locals.var_q_fd_dlt2_dn7, locals.var_q_fd_dlt2_dn10, locals.var_q_fd_dlt2_dn11, locals.var_q_fd_dlt2_dn12, locals.var_q_fd_dlt2_dn17,)
    }
};
        locals.var_q_fd_dlt2 = assign7210_e4811;
        locals.var_q_fd_dlt2_dn0 = assign7210_e4811_d_n0;
        locals.var_q_fd_dlt2_dn2 = assign7210_e4811_d_n2;
        locals.var_q_fd_dlt2_dn6 = assign7210_e4811_d_n6;
        locals.var_q_fd_dlt2_dn7 = assign7210_e4811_d_n7;
        locals.var_q_fd_dlt2_dn10 = assign7210_e4811_d_n10;
        locals.var_q_fd_dlt2_dn11 = assign7210_e4811_d_n11;
        locals.var_q_fd_dlt2_dn12 = assign7210_e4811_d_n12;
        locals.var_q_fd_dlt2_dn17 = assign7210_e4811_d_n17;
        locals.var_q_fd_dlt2_rv = 0.0;

        let (assign7220_e4819, assign7220_e4819_d_n0, assign7220_e4819_d_n2, assign7220_e4819_d_n6, assign7220_e4819_d_n7, assign7220_e4819_d_n10, assign7220_e4819_d_n11, assign7220_e4819_d_n12, assign7220_e4819_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (p.p39 != 0.0)) {
        let assign7220_e4817: f64 = (locals.var_vbsz + locals.var_vbi_soi);
        (assign7220_e4817, (locals.var_vbsz_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbsz_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbsz_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbsz_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbsz_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbsz_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbsz_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbsz_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7220_e4819;
        locals.var_vbsbiz_dn0 = assign7220_e4819_d_n0;
        locals.var_vbsbiz_dn2 = assign7220_e4819_d_n2;
        locals.var_vbsbiz_dn6 = assign7220_e4819_d_n6;
        locals.var_vbsbiz_dn7 = assign7220_e4819_d_n7;
        locals.var_vbsbiz_dn10 = assign7220_e4819_d_n10;
        locals.var_vbsbiz_dn11 = assign7220_e4819_d_n11;
        locals.var_vbsbiz_dn12 = assign7220_e4819_d_n12;
        locals.var_vbsbiz_dn17 = assign7220_e4819_d_n17;
        locals.var_vbsbiz_rv = 0.0;

        let (assign7230_e4828, assign7230_e4828_d_n0, assign7230_e4828_d_n2, assign7230_e4828_d_n6, assign7230_e4828_d_n7, assign7230_e4828_d_n10, assign7230_e4828_d_n11, assign7230_e4828_d_n12, assign7230_e4828_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (p.p39 == 0.0)) {
        let assign7230_e4826: f64 = (locals.var_vbs + locals.var_vbi_soi);
        (assign7230_e4826, (locals.var_vbs_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbs_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbs_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbs_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbs_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbs_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbs_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbs_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7230_e4828;
        locals.var_vbsbiz_dn0 = assign7230_e4828_d_n0;
        locals.var_vbsbiz_dn2 = assign7230_e4828_d_n2;
        locals.var_vbsbiz_dn6 = assign7230_e4828_d_n6;
        locals.var_vbsbiz_dn7 = assign7230_e4828_d_n7;
        locals.var_vbsbiz_dn10 = assign7230_e4828_d_n10;
        locals.var_vbsbiz_dn11 = assign7230_e4828_d_n11;
        locals.var_vbsbiz_dn12 = assign7230_e4828_d_n12;
        locals.var_vbsbiz_dn17 = assign7230_e4828_d_n17;
        locals.var_vbsbiz_rv = 0.0;

        let (assign7240_e4839, assign7240_e4839_d_n0, assign7240_e4839_d_n2, assign7240_e4839_d_n6, assign7240_e4839_d_n7, assign7240_e4839_d_n10, assign7240_e4839_d_n11, assign7240_e4839_d_n12, assign7240_e4839_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7240_e4832: f64 = (2.0 / locals.var_beta);
        let assign7240_e4835: f64 = (locals.var_mks_nsubb / locals.var_nin);
        let assign7240_e4836: f64 = (assign7240_e4835).ln();
        let assign7240_e4837: f64 = (assign7240_e4832 * assign7240_e4836);
        (assign7240_e4837, (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign7240_e4835)), (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign7240_e4835)), (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign7240_e4835)), (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign7240_e4835)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign7240_e4836) + (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign7240_e4835))), (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign7240_e4835)), (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) / assign7240_e4835)), (assign7240_e4832 * ((-((locals.var_mks_nsubb * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))) / assign7240_e4835)),)
    } else {
        (locals.var_pb2_bulk, locals.var_pb2_bulk_dn0, locals.var_pb2_bulk_dn2, locals.var_pb2_bulk_dn6, locals.var_pb2_bulk_dn7, locals.var_pb2_bulk_dn10, locals.var_pb2_bulk_dn11, locals.var_pb2_bulk_dn12, locals.var_pb2_bulk_dn17,)
    }
};
        locals.var_pb2_bulk = assign7240_e4839;
        locals.var_pb2_bulk_dn0 = assign7240_e4839_d_n0;
        locals.var_pb2_bulk_dn2 = assign7240_e4839_d_n2;
        locals.var_pb2_bulk_dn6 = assign7240_e4839_d_n6;
        locals.var_pb2_bulk_dn7 = assign7240_e4839_d_n7;
        locals.var_pb2_bulk_dn10 = assign7240_e4839_d_n10;
        locals.var_pb2_bulk_dn11 = assign7240_e4839_d_n11;
        locals.var_pb2_bulk_dn12 = assign7240_e4839_d_n12;
        locals.var_pb2_bulk_dn17 = assign7240_e4839_d_n17;
        locals.var_pb2_bulk_rv = 0.0;

        let (assign7250_e4849, assign7250_e4849_d_n10,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7250_e4843: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign7250_e4845: f64 = (assign7250_e4843 * locals.var_c_box_fd_inv);
        let assign7250_e4847: f64 = (assign7250_e4845 * locals.var_c_box_fd_inv);
        (assign7250_e4847, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv),)
    } else {
        (locals.var_t0__blk121, locals.var_t0__blk121_dn10,)
    }
};
        locals.var_t0__blk121 = assign7250_e4849;
        locals.var_t0__blk121_dn10 = assign7250_e4849_d_n10;
        locals.var_t0__blk121_rv = 0.0;

        let (assign7260_e4854, assign7260_e4854_d_n0, assign7260_e4854_d_n2, assign7260_e4854_d_n6, assign7260_e4854_d_n7, assign7260_e4854_d_n10, assign7260_e4854_d_n11, assign7260_e4854_d_n12, assign7260_e4854_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7260_e4852: f64 = (-locals.var_vbsbiz);
        (assign7260_e4852, (-locals.var_vbsbiz_dn0), (-locals.var_vbsbiz_dn2), (-locals.var_vbsbiz_dn6), (-locals.var_vbsbiz_dn7), (-locals.var_vbsbiz_dn10), (-locals.var_vbsbiz_dn11), (-locals.var_vbsbiz_dn12), (-locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk122, locals.var_t1__blk122_dn0, locals.var_t1__blk122_dn2, locals.var_t1__blk122_dn6, locals.var_t1__blk122_dn7, locals.var_t1__blk122_dn10, locals.var_t1__blk122_dn11, locals.var_t1__blk122_dn12, locals.var_t1__blk122_dn17,)
    }
};
        locals.var_t1__blk122 = assign7260_e4854;
        locals.var_t1__blk122_dn0 = assign7260_e4854_d_n0;
        locals.var_t1__blk122_dn2 = assign7260_e4854_d_n2;
        locals.var_t1__blk122_dn6 = assign7260_e4854_d_n6;
        locals.var_t1__blk122_dn7 = assign7260_e4854_d_n7;
        locals.var_t1__blk122_dn10 = assign7260_e4854_d_n10;
        locals.var_t1__blk122_dn11 = assign7260_e4854_d_n11;
        locals.var_t1__blk122_dn12 = assign7260_e4854_d_n12;
        locals.var_t1__blk122_dn17 = assign7260_e4854_d_n17;
        locals.var_t1__blk122_rv = 0.0;

        let (assign7270_e4880, assign7270_e4880_d_n0, assign7270_e4880_d_n2, assign7270_e4880_d_n6, assign7270_e4880_d_n7, assign7270_e4880_d_n10, assign7270_e4880_d_n11, assign7270_e4880_d_n12, assign7270_e4880_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7270_e4858: f64 = (2.0 * locals.var_t1__blk122);
        let assign7270_e4861: f64 = (locals.var_t0__blk121 * locals.var_beta);
        let assign7270_e4862: f64 = (assign7270_e4858 + assign7270_e4861);
        let assign7270_e4865: f64 = (2.0 * locals.var_t1__blk122);
        let assign7270_e4868: f64 = (locals.var_t0__blk121 * locals.var_beta);
        let assign7270_e4869: f64 = (assign7270_e4865 + assign7270_e4868);
        let assign7270_e4870: f64 = (assign7270_e4862 * assign7270_e4869);
        let assign7270_e4874: f64 = (locals.var_t1__blk122 * locals.var_t1__blk122);
        let assign7270_e4876: f64 = (assign7270_e4874 + locals.var_t0__blk121);
        let assign7270_e4877: f64 = (4.0 * assign7270_e4876);
        let assign7270_e4878: f64 = (assign7270_e4870 - assign7270_e4877);
        (assign7270_e4878, ((((2.0 * locals.var_t1__blk122_dn0) * assign7270_e4869) + (assign7270_e4862 * (2.0 * locals.var_t1__blk122_dn0))) - (4.0 * ((locals.var_t1__blk122_dn0 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn0)))), ((((2.0 * locals.var_t1__blk122_dn2) * assign7270_e4869) + (assign7270_e4862 * (2.0 * locals.var_t1__blk122_dn2))) - (4.0 * ((locals.var_t1__blk122_dn2 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn2)))), ((((2.0 * locals.var_t1__blk122_dn6) * assign7270_e4869) + (assign7270_e4862 * (2.0 * locals.var_t1__blk122_dn6))) - (4.0 * ((locals.var_t1__blk122_dn6 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn6)))), ((((2.0 * locals.var_t1__blk122_dn7) * assign7270_e4869) + (assign7270_e4862 * (2.0 * locals.var_t1__blk122_dn7))) - (4.0 * ((locals.var_t1__blk122_dn7 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn7)))), (((((2.0 * locals.var_t1__blk122_dn10) + ((locals.var_t0__blk121_dn10 * locals.var_beta) + (locals.var_t0__blk121 * locals.var_beta_dn10))) * assign7270_e4869) + (assign7270_e4862 * ((2.0 * locals.var_t1__blk122_dn10) + ((locals.var_t0__blk121_dn10 * locals.var_beta) + (locals.var_t0__blk121 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk122_dn10 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn10)) + locals.var_t0__blk121_dn10))), ((((2.0 * locals.var_t1__blk122_dn11) * assign7270_e4869) + (assign7270_e4862 * (2.0 * locals.var_t1__blk122_dn11))) - (4.0 * ((locals.var_t1__blk122_dn11 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn11)))), ((((2.0 * locals.var_t1__blk122_dn12) * assign7270_e4869) + (assign7270_e4862 * (2.0 * locals.var_t1__blk122_dn12))) - (4.0 * ((locals.var_t1__blk122_dn12 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn12)))), ((((2.0 * locals.var_t1__blk122_dn17) * assign7270_e4869) + (assign7270_e4862 * (2.0 * locals.var_t1__blk122_dn17))) - (4.0 * ((locals.var_t1__blk122_dn17 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn17)))),)
    } else {
        (locals.var_t2__blk123, locals.var_t2__blk123_dn0, locals.var_t2__blk123_dn2, locals.var_t2__blk123_dn6, locals.var_t2__blk123_dn7, locals.var_t2__blk123_dn10, locals.var_t2__blk123_dn11, locals.var_t2__blk123_dn12, locals.var_t2__blk123_dn17,)
    }
};
        locals.var_t2__blk123 = assign7270_e4880;
        locals.var_t2__blk123_dn0 = assign7270_e4880_d_n0;
        locals.var_t2__blk123_dn2 = assign7270_e4880_d_n2;
        locals.var_t2__blk123_dn6 = assign7270_e4880_d_n6;
        locals.var_t2__blk123_dn7 = assign7270_e4880_d_n7;
        locals.var_t2__blk123_dn10 = assign7270_e4880_d_n10;
        locals.var_t2__blk123_dn11 = assign7270_e4880_d_n11;
        locals.var_t2__blk123_dn12 = assign7270_e4880_d_n12;
        locals.var_t2__blk123_dn17 = assign7270_e4880_d_n17;
        locals.var_t2__blk123_rv = 0.0;

        let (assign7280_e4893, assign7280_e4893_d_n0, assign7280_e4893_d_n2, assign7280_e4893_d_n6, assign7280_e4893_d_n7, assign7280_e4893_d_n10, assign7280_e4893_d_n11, assign7280_e4893_d_n12, assign7280_e4893_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7280_e4885: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7280_e4891, assign7280_e4891_d_n0, assign7280_e4891_d_n2, assign7280_e4891_d_n6, assign7280_e4891_d_n7, assign7280_e4891_d_n10, assign7280_e4891_d_n11, assign7280_e4891_d_n12, assign7280_e4891_d_n17,) = {
            if (locals.var_t2__blk123 >= assign7280_e4885) {
                (locals.var_t2__blk123, locals.var_t2__blk123_dn0, locals.var_t2__blk123_dn2, locals.var_t2__blk123_dn6, locals.var_t2__blk123_dn7, locals.var_t2__blk123_dn10, locals.var_t2__blk123_dn11, locals.var_t2__blk123_dn12, locals.var_t2__blk123_dn17,)
            } else {
                let assign7280_e4890: f64 = (10.0 * 2.220446049250313e-16);
                (assign7280_e4890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7280_e4891, assign7280_e4891_d_n0, assign7280_e4891_d_n2, assign7280_e4891_d_n6, assign7280_e4891_d_n7, assign7280_e4891_d_n10, assign7280_e4891_d_n11, assign7280_e4891_d_n12, assign7280_e4891_d_n17,)
    } else {
        (locals.var_t2__blk123, locals.var_t2__blk123_dn0, locals.var_t2__blk123_dn2, locals.var_t2__blk123_dn6, locals.var_t2__blk123_dn7, locals.var_t2__blk123_dn10, locals.var_t2__blk123_dn11, locals.var_t2__blk123_dn12, locals.var_t2__blk123_dn17,)
    }
};
        locals.var_t2__blk123 = assign7280_e4893;
        locals.var_t2__blk123_dn0 = assign7280_e4893_d_n0;
        locals.var_t2__blk123_dn2 = assign7280_e4893_d_n2;
        locals.var_t2__blk123_dn6 = assign7280_e4893_d_n6;
        locals.var_t2__blk123_dn7 = assign7280_e4893_d_n7;
        locals.var_t2__blk123_dn10 = assign7280_e4893_d_n10;
        locals.var_t2__blk123_dn11 = assign7280_e4893_d_n11;
        locals.var_t2__blk123_dn12 = assign7280_e4893_d_n12;
        locals.var_t2__blk123_dn17 = assign7280_e4893_d_n17;
        locals.var_t2__blk123_rv = 0.0;

        let (assign7290_e4898, assign7290_e4898_d_n0, assign7290_e4898_d_n2, assign7290_e4898_d_n6, assign7290_e4898_d_n7, assign7290_e4898_d_n10, assign7290_e4898_d_n11, assign7290_e4898_d_n12, assign7290_e4898_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7290_e4896: f64 = (locals.var_t2__blk123).sqrt();
        (assign7290_e4896, (locals.var_t2__blk123_dn0 / (2.0 * assign7290_e4896)), (locals.var_t2__blk123_dn2 / (2.0 * assign7290_e4896)), (locals.var_t2__blk123_dn6 / (2.0 * assign7290_e4896)), (locals.var_t2__blk123_dn7 / (2.0 * assign7290_e4896)), (locals.var_t2__blk123_dn10 / (2.0 * assign7290_e4896)), (locals.var_t2__blk123_dn11 / (2.0 * assign7290_e4896)), (locals.var_t2__blk123_dn12 / (2.0 * assign7290_e4896)), (locals.var_t2__blk123_dn17 / (2.0 * assign7290_e4896)),)
    } else {
        (locals.var_t2__blk123, locals.var_t2__blk123_dn0, locals.var_t2__blk123_dn2, locals.var_t2__blk123_dn6, locals.var_t2__blk123_dn7, locals.var_t2__blk123_dn10, locals.var_t2__blk123_dn11, locals.var_t2__blk123_dn12, locals.var_t2__blk123_dn17,)
    }
};
        locals.var_t2__blk123 = assign7290_e4898;
        locals.var_t2__blk123_dn0 = assign7290_e4898_d_n0;
        locals.var_t2__blk123_dn2 = assign7290_e4898_d_n2;
        locals.var_t2__blk123_dn6 = assign7290_e4898_d_n6;
        locals.var_t2__blk123_dn7 = assign7290_e4898_d_n7;
        locals.var_t2__blk123_dn10 = assign7290_e4898_d_n10;
        locals.var_t2__blk123_dn11 = assign7290_e4898_d_n11;
        locals.var_t2__blk123_dn12 = assign7290_e4898_d_n12;
        locals.var_t2__blk123_dn17 = assign7290_e4898_d_n17;
        locals.var_t2__blk123_rv = 0.0;

        let (assign7300_e4908, assign7300_e4908_d_n0, assign7300_e4908_d_n2, assign7300_e4908_d_n6, assign7300_e4908_d_n7, assign7300_e4908_d_n10, assign7300_e4908_d_n11, assign7300_e4908_d_n12, assign7300_e4908_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7300_e4902: f64 = (2.0 * locals.var_t1__blk122);
        let assign7300_e4905: f64 = (locals.var_t0__blk121 * locals.var_beta);
        let assign7300_e4906: f64 = (assign7300_e4902 + assign7300_e4905);
        (assign7300_e4906, (2.0 * locals.var_t1__blk122_dn0), (2.0 * locals.var_t1__blk122_dn2), (2.0 * locals.var_t1__blk122_dn6), (2.0 * locals.var_t1__blk122_dn7), ((2.0 * locals.var_t1__blk122_dn10) + ((locals.var_t0__blk121_dn10 * locals.var_beta) + (locals.var_t0__blk121 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk122_dn11), (2.0 * locals.var_t1__blk122_dn12), (2.0 * locals.var_t1__blk122_dn17),)
    } else {
        (locals.var_t3__blk124, locals.var_t3__blk124_dn0, locals.var_t3__blk124_dn2, locals.var_t3__blk124_dn6, locals.var_t3__blk124_dn7, locals.var_t3__blk124_dn10, locals.var_t3__blk124_dn11, locals.var_t3__blk124_dn12, locals.var_t3__blk124_dn17,)
    }
};
        locals.var_t3__blk124 = assign7300_e4908;
        locals.var_t3__blk124_dn0 = assign7300_e4908_d_n0;
        locals.var_t3__blk124_dn2 = assign7300_e4908_d_n2;
        locals.var_t3__blk124_dn6 = assign7300_e4908_d_n6;
        locals.var_t3__blk124_dn7 = assign7300_e4908_d_n7;
        locals.var_t3__blk124_dn10 = assign7300_e4908_d_n10;
        locals.var_t3__blk124_dn11 = assign7300_e4908_d_n11;
        locals.var_t3__blk124_dn12 = assign7300_e4908_d_n12;
        locals.var_t3__blk124_dn17 = assign7300_e4908_d_n17;
        locals.var_t3__blk124_rv = 0.0;

        let (assign7310_e4916, assign7310_e4916_d_n0, assign7310_e4916_d_n2, assign7310_e4916_d_n6, assign7310_e4916_d_n7, assign7310_e4916_d_n10, assign7310_e4916_d_n11, assign7310_e4916_d_n12, assign7310_e4916_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7310_e4912: f64 = (locals.var_t3__blk124 - locals.var_t2__blk123);
        let assign7310_e4914: f64 = (assign7310_e4912 / 2.0);
        (assign7310_e4914, ((locals.var_t3__blk124_dn0 - locals.var_t2__blk123_dn0) / 2.0), ((locals.var_t3__blk124_dn2 - locals.var_t2__blk123_dn2) / 2.0), ((locals.var_t3__blk124_dn6 - locals.var_t2__blk123_dn6) / 2.0), ((locals.var_t3__blk124_dn7 - locals.var_t2__blk123_dn7) / 2.0), ((locals.var_t3__blk124_dn10 - locals.var_t2__blk123_dn10) / 2.0), ((locals.var_t3__blk124_dn11 - locals.var_t2__blk123_dn11) / 2.0), ((locals.var_t3__blk124_dn12 - locals.var_t2__blk123_dn12) / 2.0), ((locals.var_t3__blk124_dn17 - locals.var_t2__blk123_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk125, locals.var_psb_inia__blk125_dn0, locals.var_psb_inia__blk125_dn2, locals.var_psb_inia__blk125_dn6, locals.var_psb_inia__blk125_dn7, locals.var_psb_inia__blk125_dn10, locals.var_psb_inia__blk125_dn11, locals.var_psb_inia__blk125_dn12, locals.var_psb_inia__blk125_dn17,)
    }
};
        locals.var_psb_inia__blk125 = assign7310_e4916;
        locals.var_psb_inia__blk125_dn0 = assign7310_e4916_d_n0;
        locals.var_psb_inia__blk125_dn2 = assign7310_e4916_d_n2;
        locals.var_psb_inia__blk125_dn6 = assign7310_e4916_d_n6;
        locals.var_psb_inia__blk125_dn7 = assign7310_e4916_d_n7;
        locals.var_psb_inia__blk125_dn10 = assign7310_e4916_d_n10;
        locals.var_psb_inia__blk125_dn11 = assign7310_e4916_d_n11;
        locals.var_psb_inia__blk125_dn12 = assign7310_e4916_d_n12;
        locals.var_psb_inia__blk125_dn17 = assign7310_e4916_d_n17;
        locals.var_psb_inia__blk125_rv = 0.0;

        let (assign7320_e4933, assign7320_e4933_d_n0, assign7320_e4933_d_n2, assign7320_e4933_d_n6, assign7320_e4933_d_n7, assign7320_e4933_d_n10, assign7320_e4933_d_n11, assign7320_e4933_d_n12, assign7320_e4933_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7320_e4920: f64 = (locals.var_t1__blk122 * locals.var_t1__blk122);
        let assign7320_e4922: f64 = (assign7320_e4920 / locals.var_t0__blk121);
        let assign7320_e4924: f64 = (assign7320_e4922 / locals.var_cnst1bulk);
        let assign7320_e4925: f64 = (assign7320_e4924).ln();
        let assign7320_e4929: f64 = (2.0 / locals.var_t1__blk122);
        let assign7320_e4930: f64 = (locals.var_beta + assign7320_e4929);
        let assign7320_e4931: f64 = (assign7320_e4925 / assign7320_e4930);
        (assign7320_e4931, ((((((((((locals.var_t1__blk122_dn0 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn0)) / locals.var_t0__blk121) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (-((2.0 * locals.var_t1__blk122_dn0) / (locals.var_t1__blk122 * locals.var_t1__blk122))))) / (assign7320_e4930 * assign7320_e4930)), ((((((((((locals.var_t1__blk122_dn2 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn2)) / locals.var_t0__blk121) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (-((2.0 * locals.var_t1__blk122_dn2) / (locals.var_t1__blk122 * locals.var_t1__blk122))))) / (assign7320_e4930 * assign7320_e4930)), ((((((((((locals.var_t1__blk122_dn6 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn6)) / locals.var_t0__blk121) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (-((2.0 * locals.var_t1__blk122_dn6) / (locals.var_t1__blk122 * locals.var_t1__blk122))))) / (assign7320_e4930 * assign7320_e4930)), ((((((((((locals.var_t1__blk122_dn7 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn7)) / locals.var_t0__blk121) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (-((2.0 * locals.var_t1__blk122_dn7) / (locals.var_t1__blk122 * locals.var_t1__blk122))))) / (assign7320_e4930 * assign7320_e4930)), ((((((((((((locals.var_t1__blk122_dn10 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn10)) * locals.var_t0__blk121) - (assign7320_e4920 * locals.var_t0__blk121_dn10)) / (locals.var_t0__blk121 * locals.var_t0__blk121)) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk122_dn10) / (locals.var_t1__blk122 * locals.var_t1__blk122)))))) / (assign7320_e4930 * assign7320_e4930)), ((((((((((locals.var_t1__blk122_dn11 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn11)) / locals.var_t0__blk121) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (-((2.0 * locals.var_t1__blk122_dn11) / (locals.var_t1__blk122 * locals.var_t1__blk122))))) / (assign7320_e4930 * assign7320_e4930)), ((((((((((locals.var_t1__blk122_dn12 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn12)) / locals.var_t0__blk121) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (-((2.0 * locals.var_t1__blk122_dn12) / (locals.var_t1__blk122 * locals.var_t1__blk122))))) / (assign7320_e4930 * assign7320_e4930)), ((((((((((locals.var_t1__blk122_dn17 * locals.var_t1__blk122) + (locals.var_t1__blk122 * locals.var_t1__blk122_dn17)) / locals.var_t0__blk121) * locals.var_cnst1bulk) - (assign7320_e4922 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7320_e4924) * assign7320_e4930) - (assign7320_e4925 * (-((2.0 * locals.var_t1__blk122_dn17) / (locals.var_t1__blk122 * locals.var_t1__blk122))))) / (assign7320_e4930 * assign7320_e4930)),)
    } else {
        (locals.var_psb_inib__blk126, locals.var_psb_inib__blk126_dn0, locals.var_psb_inib__blk126_dn2, locals.var_psb_inib__blk126_dn6, locals.var_psb_inib__blk126_dn7, locals.var_psb_inib__blk126_dn10, locals.var_psb_inib__blk126_dn11, locals.var_psb_inib__blk126_dn12, locals.var_psb_inib__blk126_dn17,)
    }
};
        locals.var_psb_inib__blk126 = assign7320_e4933;
        locals.var_psb_inib__blk126_dn0 = assign7320_e4933_d_n0;
        locals.var_psb_inib__blk126_dn2 = assign7320_e4933_d_n2;
        locals.var_psb_inib__blk126_dn6 = assign7320_e4933_d_n6;
        locals.var_psb_inib__blk126_dn7 = assign7320_e4933_d_n7;
        locals.var_psb_inib__blk126_dn10 = assign7320_e4933_d_n10;
        locals.var_psb_inib__blk126_dn11 = assign7320_e4933_d_n11;
        locals.var_psb_inib__blk126_dn12 = assign7320_e4933_d_n12;
        locals.var_psb_inib__blk126_dn17 = assign7320_e4933_d_n17;
        locals.var_psb_inib__blk126_rv = 0.0;

        let assign7330_e4936: f64 = if locals.var_psb_inia__blk125 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard127 = assign7330_e4936;
        locals.var_guard127_rv = 0.0;

        let (assign7340_e4942, assign7340_e4942_d_n0, assign7340_e4942_d_n2, assign7340_e4942_d_n6, assign7340_e4942_d_n7, assign7340_e4942_d_n10, assign7340_e4942_d_n11, assign7340_e4942_d_n12, assign7340_e4942_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard127 != 0.0)) {
        (locals.var_psb_inia__blk125, locals.var_psb_inia__blk125_dn0, locals.var_psb_inia__blk125_dn2, locals.var_psb_inia__blk125_dn6, locals.var_psb_inia__blk125_dn7, locals.var_psb_inia__blk125_dn10, locals.var_psb_inia__blk125_dn11, locals.var_psb_inia__blk125_dn12, locals.var_psb_inia__blk125_dn17,)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7340_e4942;
        locals.var_phi_s0_bulk_0_dn0 = assign7340_e4942_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7340_e4942_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7340_e4942_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7340_e4942_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7340_e4942_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7340_e4942_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7340_e4942_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7340_e4942_d_n17;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign7350_e4953, assign7350_e4953_d_n0, assign7350_e4953_d_n2, assign7350_e4953_d_n6, assign7350_e4953_d_n7, assign7350_e4953_d_n10, assign7350_e4953_d_n11, assign7350_e4953_d_n12, assign7350_e4953_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign7350_e4949: f64 = (locals.var_psb_inib__blk126 - locals.var_psb_inia__blk125);
        let assign7350_e4951: f64 = (assign7350_e4949 - 0.0008);
        (assign7350_e4951, (locals.var_psb_inib__blk126_dn0 - locals.var_psb_inia__blk125_dn0), (locals.var_psb_inib__blk126_dn2 - locals.var_psb_inia__blk125_dn2), (locals.var_psb_inib__blk126_dn6 - locals.var_psb_inia__blk125_dn6), (locals.var_psb_inib__blk126_dn7 - locals.var_psb_inia__blk125_dn7), (locals.var_psb_inib__blk126_dn10 - locals.var_psb_inia__blk125_dn10), (locals.var_psb_inib__blk126_dn11 - locals.var_psb_inia__blk125_dn11), (locals.var_psb_inib__blk126_dn12 - locals.var_psb_inia__blk125_dn12), (locals.var_psb_inib__blk126_dn17 - locals.var_psb_inia__blk125_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7350_e4953;
        locals.var_tmf1_dn0 = assign7350_e4953_d_n0;
        locals.var_tmf1_dn2 = assign7350_e4953_d_n2;
        locals.var_tmf1_dn6 = assign7350_e4953_d_n6;
        locals.var_tmf1_dn7 = assign7350_e4953_d_n7;
        locals.var_tmf1_dn10 = assign7350_e4953_d_n10;
        locals.var_tmf1_dn11 = assign7350_e4953_d_n11;
        locals.var_tmf1_dn12 = assign7350_e4953_d_n12;
        locals.var_tmf1_dn17 = assign7350_e4953_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7360_e4964, assign7360_e4964_d_n0, assign7360_e4964_d_n2, assign7360_e4964_d_n6, assign7360_e4964_d_n7, assign7360_e4964_d_n10, assign7360_e4964_d_n11, assign7360_e4964_d_n12, assign7360_e4964_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign7360_e4960: f64 = (4.0 * locals.var_psb_inib__blk126);
        let assign7360_e4962: f64 = (assign7360_e4960 * 0.0008);
        (assign7360_e4962, ((4.0 * locals.var_psb_inib__blk126_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk126_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk126_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk126_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk126_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk126_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk126_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk126_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7360_e4964;
        locals.var_tmf2_dn0 = assign7360_e4964_d_n0;
        locals.var_tmf2_dn2 = assign7360_e4964_d_n2;
        locals.var_tmf2_dn6 = assign7360_e4964_d_n6;
        locals.var_tmf2_dn7 = assign7360_e4964_d_n7;
        locals.var_tmf2_dn10 = assign7360_e4964_d_n10;
        locals.var_tmf2_dn11 = assign7360_e4964_d_n11;
        locals.var_tmf2_dn12 = assign7360_e4964_d_n12;
        locals.var_tmf2_dn17 = assign7360_e4964_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7370_e4977, assign7370_e4977_d_n0, assign7370_e4977_d_n2, assign7370_e4977_d_n6, assign7370_e4977_d_n7, assign7370_e4977_d_n10, assign7370_e4977_d_n11, assign7370_e4977_d_n12, assign7370_e4977_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard127 == 0.0)) {
        let (assign7370_e4975, assign7370_e4975_d_n0, assign7370_e4975_d_n2, assign7370_e4975_d_n6, assign7370_e4975_d_n7, assign7370_e4975_d_n10, assign7370_e4975_d_n11, assign7370_e4975_d_n12, assign7370_e4975_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign7370_e4974: f64 = (-locals.var_tmf2);
                (assign7370_e4974, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign7370_e4975, assign7370_e4975_d_n0, assign7370_e4975_d_n2, assign7370_e4975_d_n6, assign7370_e4975_d_n7, assign7370_e4975_d_n10, assign7370_e4975_d_n11, assign7370_e4975_d_n12, assign7370_e4975_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7370_e4977;
        locals.var_tmf2_dn0 = assign7370_e4977_d_n0;
        locals.var_tmf2_dn2 = assign7370_e4977_d_n2;
        locals.var_tmf2_dn6 = assign7370_e4977_d_n6;
        locals.var_tmf2_dn7 = assign7370_e4977_d_n7;
        locals.var_tmf2_dn10 = assign7370_e4977_d_n10;
        locals.var_tmf2_dn11 = assign7370_e4977_d_n11;
        locals.var_tmf2_dn12 = assign7370_e4977_d_n12;
        locals.var_tmf2_dn17 = assign7370_e4977_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7380_e4989, assign7380_e4989_d_n0, assign7380_e4989_d_n2, assign7380_e4989_d_n6, assign7380_e4989_d_n7, assign7380_e4989_d_n10, assign7380_e4989_d_n11, assign7380_e4989_d_n12, assign7380_e4989_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign7380_e4984: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign7380_e4986: f64 = (assign7380_e4984 + locals.var_tmf2);
        let assign7380_e4987: f64 = (assign7380_e4986).sqrt();
        (assign7380_e4987, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign7380_e4987)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign7380_e4987)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign7380_e4987)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign7380_e4987)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign7380_e4987)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign7380_e4987)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign7380_e4987)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign7380_e4987)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7380_e4989;
        locals.var_tmf2_dn0 = assign7380_e4989_d_n0;
        locals.var_tmf2_dn2 = assign7380_e4989_d_n2;
        locals.var_tmf2_dn6 = assign7380_e4989_d_n6;
        locals.var_tmf2_dn7 = assign7380_e4989_d_n7;
        locals.var_tmf2_dn10 = assign7380_e4989_d_n10;
        locals.var_tmf2_dn11 = assign7380_e4989_d_n11;
        locals.var_tmf2_dn12 = assign7380_e4989_d_n12;
        locals.var_tmf2_dn17 = assign7380_e4989_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7390_e5002, assign7390_e5002_d_n0, assign7390_e5002_d_n2, assign7390_e5002_d_n6, assign7390_e5002_d_n7, assign7390_e5002_d_n10, assign7390_e5002_d_n11, assign7390_e5002_d_n12, assign7390_e5002_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign7390_e4998: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign7390_e4999: f64 = (0.5 * assign7390_e4998);
        let assign7390_e5000: f64 = (locals.var_psb_inib__blk126 - assign7390_e4999);
        (assign7390_e5000, (locals.var_psb_inib__blk126_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk126_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk126_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk126_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk126_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk126_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk126_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk126_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7390_e5002;
        locals.var_phi_s0_bulk_0_dn0 = assign7390_e5002_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7390_e5002_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7390_e5002_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7390_e5002_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7390_e5002_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7390_e5002_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7390_e5002_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7390_e5002_d_n17;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign7400_e5006,) = {
    if (locals.var_guard113 != 0.0) {
        (0.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign7400_e5006;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        locals: &mut StampLocals,
    ) {
        let mut assign7410_loop_guard: usize = 0;
        while {
            let assign7410_cond_e5011: f64 = if ((locals.var_guard113 != 0.0) && (locals.var_lp_s0 < locals.var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign7410_cond_e5011 != 0.0
        } {
            assign7410_loop_guard += 1;
            assert!(assign7410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7410_body0_e5015, assign7410_body0_e5015_d_n10,) = {
    if (locals.var_guard113 != 0.0) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk128, locals.var_t1__blk128_dn10,)
    }
};
            locals.var_t1__blk128 = assign7410_body0_e5015;
            locals.var_t1__blk128_dn10 = assign7410_body0_e5015_d_n10;
            locals.var_t1__blk128_rv = 0.0;
            let (assign7410_body1_e5021, assign7410_body1_e5021_d_n0, assign7410_body1_e5021_d_n2, assign7410_body1_e5021_d_n6, assign7410_body1_e5021_d_n7, assign7410_body1_e5021_d_n10, assign7410_body1_e5021_d_n11, assign7410_body1_e5021_d_n12, assign7410_body1_e5021_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body1_e5019: f64 = (locals.var_beta * locals.var_phi_s0_bulk_0);
        (assign7410_body1_e5019, (locals.var_beta * locals.var_phi_s0_bulk_0_dn0), (locals.var_beta * locals.var_phi_s0_bulk_0_dn2), (locals.var_beta * locals.var_phi_s0_bulk_0_dn6), (locals.var_beta * locals.var_phi_s0_bulk_0_dn7), ((locals.var_beta_dn10 * locals.var_phi_s0_bulk_0) + (locals.var_beta * locals.var_phi_s0_bulk_0_dn10)), (locals.var_beta * locals.var_phi_s0_bulk_0_dn11), (locals.var_beta * locals.var_phi_s0_bulk_0_dn12), (locals.var_beta * locals.var_phi_s0_bulk_0_dn17),)
    } else {
        (locals.var_t2__blk129, locals.var_t2__blk129_dn0, locals.var_t2__blk129_dn2, locals.var_t2__blk129_dn6, locals.var_t2__blk129_dn7, locals.var_t2__blk129_dn10, locals.var_t2__blk129_dn11, locals.var_t2__blk129_dn12, locals.var_t2__blk129_dn17,)
    }
};
            locals.var_t2__blk129 = assign7410_body1_e5021;
            locals.var_t2__blk129_dn0 = assign7410_body1_e5021_d_n0;
            locals.var_t2__blk129_dn2 = assign7410_body1_e5021_d_n2;
            locals.var_t2__blk129_dn6 = assign7410_body1_e5021_d_n6;
            locals.var_t2__blk129_dn7 = assign7410_body1_e5021_d_n7;
            locals.var_t2__blk129_dn10 = assign7410_body1_e5021_d_n10;
            locals.var_t2__blk129_dn11 = assign7410_body1_e5021_d_n11;
            locals.var_t2__blk129_dn12 = assign7410_body1_e5021_d_n12;
            locals.var_t2__blk129_dn17 = assign7410_body1_e5021_d_n17;
            locals.var_t2__blk129_rv = 0.0;
            let (assign7410_body2_e5027, assign7410_body2_e5027_d_n0, assign7410_body2_e5027_d_n2, assign7410_body2_e5027_d_n6, assign7410_body2_e5027_d_n7, assign7410_body2_e5027_d_n10, assign7410_body2_e5027_d_n11, assign7410_body2_e5027_d_n12, assign7410_body2_e5027_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body2_e5024: f64 = (-locals.var_t2__blk129);
        let assign7410_body2_e5025: f64 = (assign7410_body2_e5024).exp();
        (assign7410_body2_e5025, (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn0)), (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn2)), (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn6)), (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn7)), (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn10)), (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn11)), (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn12)), (assign7410_body2_e5025 * (-locals.var_t2__blk129_dn17)),)
    } else {
        (locals.var_t3__blk130, locals.var_t3__blk130_dn0, locals.var_t3__blk130_dn2, locals.var_t3__blk130_dn6, locals.var_t3__blk130_dn7, locals.var_t3__blk130_dn10, locals.var_t3__blk130_dn11, locals.var_t3__blk130_dn12, locals.var_t3__blk130_dn17,)
    }
};
            locals.var_t3__blk130 = assign7410_body2_e5027;
            locals.var_t3__blk130_dn0 = assign7410_body2_e5027_d_n0;
            locals.var_t3__blk130_dn2 = assign7410_body2_e5027_d_n2;
            locals.var_t3__blk130_dn6 = assign7410_body2_e5027_d_n6;
            locals.var_t3__blk130_dn7 = assign7410_body2_e5027_d_n7;
            locals.var_t3__blk130_dn10 = assign7410_body2_e5027_d_n10;
            locals.var_t3__blk130_dn11 = assign7410_body2_e5027_d_n11;
            locals.var_t3__blk130_dn12 = assign7410_body2_e5027_d_n12;
            locals.var_t3__blk130_dn17 = assign7410_body2_e5027_d_n17;
            locals.var_t3__blk130_rv = 0.0;
            let assign7410_body3_e5030: f64 = if locals.var_phi_s0_bulk_0 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard136 = assign7410_body3_e5030;
            locals.var_guard136_rv = 0.0;
            let (assign7410_body4_e5039, assign7410_body4_e5039_d_n0, assign7410_body4_e5039_d_n2, assign7410_body4_e5039_d_n6, assign7410_body4_e5039_d_n7, assign7410_body4_e5039_d_n10, assign7410_body4_e5039_d_n11, assign7410_body4_e5039_d_n12, assign7410_body4_e5039_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard136 != 0.0)) {
        let assign7410_body4_e5036: f64 = (locals.var_beta * locals.var_phi_s0_bulk_0);
        let assign7410_body4_e5037: f64 = (assign7410_body4_e5036).exp();
        (assign7410_body4_e5037, (assign7410_body4_e5037 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn0)), (assign7410_body4_e5037 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn2)), (assign7410_body4_e5037 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn6)), (assign7410_body4_e5037 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn7)), (assign7410_body4_e5037 * ((locals.var_beta_dn10 * locals.var_phi_s0_bulk_0) + (locals.var_beta * locals.var_phi_s0_bulk_0_dn10))), (assign7410_body4_e5037 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn11)), (assign7410_body4_e5037 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn12)), (assign7410_body4_e5037 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn17)),)
    } else {
        (locals.var_t0__blk131, locals.var_t0__blk131_dn0, locals.var_t0__blk131_dn2, locals.var_t0__blk131_dn6, locals.var_t0__blk131_dn7, locals.var_t0__blk131_dn10, locals.var_t0__blk131_dn11, locals.var_t0__blk131_dn12, locals.var_t0__blk131_dn17,)
    }
};
            locals.var_t0__blk131 = assign7410_body4_e5039;
            locals.var_t0__blk131_dn0 = assign7410_body4_e5039_d_n0;
            locals.var_t0__blk131_dn2 = assign7410_body4_e5039_d_n2;
            locals.var_t0__blk131_dn6 = assign7410_body4_e5039_d_n6;
            locals.var_t0__blk131_dn7 = assign7410_body4_e5039_d_n7;
            locals.var_t0__blk131_dn10 = assign7410_body4_e5039_d_n10;
            locals.var_t0__blk131_dn11 = assign7410_body4_e5039_d_n11;
            locals.var_t0__blk131_dn12 = assign7410_body4_e5039_d_n12;
            locals.var_t0__blk131_dn17 = assign7410_body4_e5039_d_n17;
            locals.var_t0__blk131_rv = 0.0;
            let (assign7410_body5_e5059, assign7410_body5_e5059_d_n0, assign7410_body5_e5059_d_n2, assign7410_body5_e5059_d_n6, assign7410_body5_e5059_d_n7, assign7410_body5_e5059_d_n10, assign7410_body5_e5059_d_n11, assign7410_body5_e5059_d_n12, assign7410_body5_e5059_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard136 != 0.0)) {
        let assign7410_body5_e5044: f64 = (-locals.var_t1__blk128);
        let assign7410_body5_e5047: f64 = (locals.var_t3__blk130 + locals.var_t2__blk129);
        let assign7410_body5_e5049: f64 = (assign7410_body5_e5047 - 1.0);
        let assign7410_body5_e5053: f64 = (locals.var_t0__blk131 - 1.0);
        let assign7410_body5_e5054: f64 = (locals.var_cnst1bulk * assign7410_body5_e5053);
        let assign7410_body5_e5055: f64 = (assign7410_body5_e5049 + assign7410_body5_e5054);
        let assign7410_body5_e5056: f64 = (assign7410_body5_e5055).sqrt();
        let assign7410_body5_e5057: f64 = (assign7410_body5_e5044 * assign7410_body5_e5056);
        (assign7410_body5_e5057, (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn0 + locals.var_t2__blk129_dn0) + ((locals.var_cnst1bulk_dn0 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn0))) / (2.0 * assign7410_body5_e5056))), (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn2 + locals.var_t2__blk129_dn2) + ((locals.var_cnst1bulk_dn2 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn2))) / (2.0 * assign7410_body5_e5056))), (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn6 + locals.var_t2__blk129_dn6) + ((locals.var_cnst1bulk_dn6 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn6))) / (2.0 * assign7410_body5_e5056))), (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn7 + locals.var_t2__blk129_dn7) + ((locals.var_cnst1bulk_dn7 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn7))) / (2.0 * assign7410_body5_e5056))), (((-locals.var_t1__blk128_dn10) * assign7410_body5_e5056) + (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn10 + locals.var_t2__blk129_dn10) + ((locals.var_cnst1bulk_dn10 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn10))) / (2.0 * assign7410_body5_e5056)))), (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn11 + locals.var_t2__blk129_dn11) + ((locals.var_cnst1bulk_dn11 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn11))) / (2.0 * assign7410_body5_e5056))), (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn12 + locals.var_t2__blk129_dn12) + ((locals.var_cnst1bulk_dn12 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn12))) / (2.0 * assign7410_body5_e5056))), (assign7410_body5_e5044 * (((locals.var_t3__blk130_dn17 + locals.var_t2__blk129_dn17) + ((locals.var_cnst1bulk_dn17 * assign7410_body5_e5053) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn17))) / (2.0 * assign7410_body5_e5056))),)
    } else {
        (locals.var_t4__blk132, locals.var_t4__blk132_dn0, locals.var_t4__blk132_dn2, locals.var_t4__blk132_dn6, locals.var_t4__blk132_dn7, locals.var_t4__blk132_dn10, locals.var_t4__blk132_dn11, locals.var_t4__blk132_dn12, locals.var_t4__blk132_dn17,)
    }
};
            locals.var_t4__blk132 = assign7410_body5_e5059;
            locals.var_t4__blk132_dn0 = assign7410_body5_e5059_d_n0;
            locals.var_t4__blk132_dn2 = assign7410_body5_e5059_d_n2;
            locals.var_t4__blk132_dn6 = assign7410_body5_e5059_d_n6;
            locals.var_t4__blk132_dn7 = assign7410_body5_e5059_d_n7;
            locals.var_t4__blk132_dn10 = assign7410_body5_e5059_d_n10;
            locals.var_t4__blk132_dn11 = assign7410_body5_e5059_d_n11;
            locals.var_t4__blk132_dn12 = assign7410_body5_e5059_d_n12;
            locals.var_t4__blk132_dn17 = assign7410_body5_e5059_d_n17;
            locals.var_t4__blk132_rv = 0.0;
            let (assign7410_body6_e5076, assign7410_body6_e5076_d_n0, assign7410_body6_e5076_d_n2, assign7410_body6_e5076_d_n6, assign7410_body6_e5076_d_n7, assign7410_body6_e5076_d_n10, assign7410_body6_e5076_d_n11, assign7410_body6_e5076_d_n12, assign7410_body6_e5076_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard136 != 0.0)) {
        let assign7410_body6_e5065: f64 = (locals.var_c0bulk / locals.var_t4__blk132);
        let assign7410_body6_e5067: f64 = (-locals.var_t3__blk130);
        let assign7410_body6_e5069: f64 = (assign7410_body6_e5067 + 1.0);
        let assign7410_body6_e5072: f64 = (locals.var_cnst1bulk * locals.var_t0__blk131);
        let assign7410_body6_e5073: f64 = (assign7410_body6_e5069 + assign7410_body6_e5072);
        let assign7410_body6_e5074: f64 = (assign7410_body6_e5065 * assign7410_body6_e5073);
        (assign7410_body6_e5074, (((-((locals.var_c0bulk * locals.var_t4__blk132_dn0) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn2) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn6) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn7) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn10) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn11) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn12) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn17) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body6_e5073) + (assign7410_body6_e5065 * ((-locals.var_t3__blk130_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk131) + (locals.var_cnst1bulk * locals.var_t0__blk131_dn17))))),)
    } else {
        (locals.var_t5__blk133, locals.var_t5__blk133_dn0, locals.var_t5__blk133_dn2, locals.var_t5__blk133_dn6, locals.var_t5__blk133_dn7, locals.var_t5__blk133_dn10, locals.var_t5__blk133_dn11, locals.var_t5__blk133_dn12, locals.var_t5__blk133_dn17,)
    }
};
            locals.var_t5__blk133 = assign7410_body6_e5076;
            locals.var_t5__blk133_dn0 = assign7410_body6_e5076_d_n0;
            locals.var_t5__blk133_dn2 = assign7410_body6_e5076_d_n2;
            locals.var_t5__blk133_dn6 = assign7410_body6_e5076_d_n6;
            locals.var_t5__blk133_dn7 = assign7410_body6_e5076_d_n7;
            locals.var_t5__blk133_dn10 = assign7410_body6_e5076_d_n10;
            locals.var_t5__blk133_dn11 = assign7410_body6_e5076_d_n11;
            locals.var_t5__blk133_dn12 = assign7410_body6_e5076_d_n12;
            locals.var_t5__blk133_dn17 = assign7410_body6_e5076_d_n17;
            locals.var_t5__blk133_rv = 0.0;
            let assign7410_body7_e5079: f64 = (-1e-9);
            let assign7410_body7_e5080: f64 = if locals.var_phi_s0_bulk_0 < assign7410_body7_e5079 { 1.0 } else { 0.0 };
            locals.var_guard137 = assign7410_body7_e5080;
            locals.var_guard137_rv = 0.0;
            let (assign7410_body8_e5096, assign7410_body8_e5096_d_n0, assign7410_body8_e5096_d_n2, assign7410_body8_e5096_d_n6, assign7410_body8_e5096_d_n7, assign7410_body8_e5096_d_n10, assign7410_body8_e5096_d_n11, assign7410_body8_e5096_d_n12, assign7410_body8_e5096_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard136 == 0.0)) && (locals.var_guard137 != 0.0)) {
        let assign7410_body8_e5090: f64 = (locals.var_t3__blk130 + locals.var_t2__blk129);
        let assign7410_body8_e5092: f64 = (assign7410_body8_e5090 - 1.0);
        let assign7410_body8_e5093: f64 = (assign7410_body8_e5092).sqrt();
        let assign7410_body8_e5094: f64 = (locals.var_t1__blk128 * assign7410_body8_e5093);
        (assign7410_body8_e5094, (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn0 + locals.var_t2__blk129_dn0) / (2.0 * assign7410_body8_e5093))), (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn2 + locals.var_t2__blk129_dn2) / (2.0 * assign7410_body8_e5093))), (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn6 + locals.var_t2__blk129_dn6) / (2.0 * assign7410_body8_e5093))), (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn7 + locals.var_t2__blk129_dn7) / (2.0 * assign7410_body8_e5093))), ((locals.var_t1__blk128_dn10 * assign7410_body8_e5093) + (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn10 + locals.var_t2__blk129_dn10) / (2.0 * assign7410_body8_e5093)))), (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn11 + locals.var_t2__blk129_dn11) / (2.0 * assign7410_body8_e5093))), (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn12 + locals.var_t2__blk129_dn12) / (2.0 * assign7410_body8_e5093))), (locals.var_t1__blk128 * ((locals.var_t3__blk130_dn17 + locals.var_t2__blk129_dn17) / (2.0 * assign7410_body8_e5093))),)
    } else {
        (locals.var_t4__blk132, locals.var_t4__blk132_dn0, locals.var_t4__blk132_dn2, locals.var_t4__blk132_dn6, locals.var_t4__blk132_dn7, locals.var_t4__blk132_dn10, locals.var_t4__blk132_dn11, locals.var_t4__blk132_dn12, locals.var_t4__blk132_dn17,)
    }
};
            locals.var_t4__blk132 = assign7410_body8_e5096;
            locals.var_t4__blk132_dn0 = assign7410_body8_e5096_d_n0;
            locals.var_t4__blk132_dn2 = assign7410_body8_e5096_d_n2;
            locals.var_t4__blk132_dn6 = assign7410_body8_e5096_d_n6;
            locals.var_t4__blk132_dn7 = assign7410_body8_e5096_d_n7;
            locals.var_t4__blk132_dn10 = assign7410_body8_e5096_d_n10;
            locals.var_t4__blk132_dn11 = assign7410_body8_e5096_d_n11;
            locals.var_t4__blk132_dn12 = assign7410_body8_e5096_d_n12;
            locals.var_t4__blk132_dn17 = assign7410_body8_e5096_d_n17;
            locals.var_t4__blk132_rv = 0.0;
            let (assign7410_body9_e5112, assign7410_body9_e5112_d_n0, assign7410_body9_e5112_d_n2, assign7410_body9_e5112_d_n6, assign7410_body9_e5112_d_n7, assign7410_body9_e5112_d_n10, assign7410_body9_e5112_d_n11, assign7410_body9_e5112_d_n12, assign7410_body9_e5112_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard136 == 0.0)) && (locals.var_guard137 != 0.0)) {
        let assign7410_body9_e5105: f64 = (locals.var_c0bulk / locals.var_t4__blk132);
        let assign7410_body9_e5107: f64 = (-locals.var_t3__blk130);
        let assign7410_body9_e5109: f64 = (assign7410_body9_e5107 + 1.0);
        let assign7410_body9_e5110: f64 = (assign7410_body9_e5105 * assign7410_body9_e5109);
        (assign7410_body9_e5110, (((-((locals.var_c0bulk * locals.var_t4__blk132_dn0) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn2) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn6) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn7) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn10) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn11) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn12) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk132_dn17) / (locals.var_t4__blk132 * locals.var_t4__blk132))) * assign7410_body9_e5109) + (assign7410_body9_e5105 * (-locals.var_t3__blk130_dn17))),)
    } else {
        (locals.var_t5__blk133, locals.var_t5__blk133_dn0, locals.var_t5__blk133_dn2, locals.var_t5__blk133_dn6, locals.var_t5__blk133_dn7, locals.var_t5__blk133_dn10, locals.var_t5__blk133_dn11, locals.var_t5__blk133_dn12, locals.var_t5__blk133_dn17,)
    }
};
            locals.var_t5__blk133 = assign7410_body9_e5112;
            locals.var_t5__blk133_dn0 = assign7410_body9_e5112_d_n0;
            locals.var_t5__blk133_dn2 = assign7410_body9_e5112_d_n2;
            locals.var_t5__blk133_dn6 = assign7410_body9_e5112_d_n6;
            locals.var_t5__blk133_dn7 = assign7410_body9_e5112_d_n7;
            locals.var_t5__blk133_dn10 = assign7410_body9_e5112_d_n10;
            locals.var_t5__blk133_dn11 = assign7410_body9_e5112_d_n11;
            locals.var_t5__blk133_dn12 = assign7410_body9_e5112_d_n12;
            locals.var_t5__blk133_dn17 = assign7410_body9_e5112_d_n17;
            locals.var_t5__blk133_rv = 0.0;
            let (assign7410_body10_e5130, assign7410_body10_e5130_d_n0, assign7410_body10_e5130_d_n2, assign7410_body10_e5130_d_n6, assign7410_body10_e5130_d_n7, assign7410_body10_e5130_d_n10, assign7410_body10_e5130_d_n11, assign7410_body10_e5130_d_n12, assign7410_body10_e5130_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard136 == 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign7410_body10_e5122: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign7410_body10_e5123: f64 = (assign7410_body10_e5122).sqrt();
        let assign7410_body10_e5124: f64 = (-assign7410_body10_e5123);
        let assign7410_body10_e5126: f64 = (assign7410_body10_e5124 * locals.var_beta);
        let assign7410_body10_e5128: f64 = (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0);
        (assign7410_body10_e5128, (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn0), (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn2), (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn6), (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign7410_body10_e5123))) * locals.var_beta) + (assign7410_body10_e5124 * locals.var_beta_dn10)) * locals.var_phi_s0_bulk_0) + (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn10)), (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn11), (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn12), (assign7410_body10_e5126 * locals.var_phi_s0_bulk_0_dn17),)
    } else {
        (locals.var_t4__blk132, locals.var_t4__blk132_dn0, locals.var_t4__blk132_dn2, locals.var_t4__blk132_dn6, locals.var_t4__blk132_dn7, locals.var_t4__blk132_dn10, locals.var_t4__blk132_dn11, locals.var_t4__blk132_dn12, locals.var_t4__blk132_dn17,)
    }
};
            locals.var_t4__blk132 = assign7410_body10_e5130;
            locals.var_t4__blk132_dn0 = assign7410_body10_e5130_d_n0;
            locals.var_t4__blk132_dn2 = assign7410_body10_e5130_d_n2;
            locals.var_t4__blk132_dn6 = assign7410_body10_e5130_d_n6;
            locals.var_t4__blk132_dn7 = assign7410_body10_e5130_d_n7;
            locals.var_t4__blk132_dn10 = assign7410_body10_e5130_d_n10;
            locals.var_t4__blk132_dn11 = assign7410_body10_e5130_d_n11;
            locals.var_t4__blk132_dn12 = assign7410_body10_e5130_d_n12;
            locals.var_t4__blk132_dn17 = assign7410_body10_e5130_d_n17;
            locals.var_t4__blk132_rv = 0.0;
            let (assign7410_body11_e5144, assign7410_body11_e5144_d_n0, assign7410_body11_e5144_d_n2, assign7410_body11_e5144_d_n6, assign7410_body11_e5144_d_n7, assign7410_body11_e5144_d_n10, assign7410_body11_e5144_d_n11, assign7410_body11_e5144_d_n12, assign7410_body11_e5144_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard136 == 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign7410_body11_e5140: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign7410_body11_e5141: f64 = (assign7410_body11_e5140).sqrt();
        let assign7410_body11_e5142: f64 = (-assign7410_body11_e5141);
        (assign7410_body11_e5142, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign7410_body11_e5141))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk133, locals.var_t5__blk133_dn0, locals.var_t5__blk133_dn2, locals.var_t5__blk133_dn6, locals.var_t5__blk133_dn7, locals.var_t5__blk133_dn10, locals.var_t5__blk133_dn11, locals.var_t5__blk133_dn12, locals.var_t5__blk133_dn17,)
    }
};
            locals.var_t5__blk133 = assign7410_body11_e5144;
            locals.var_t5__blk133_dn0 = assign7410_body11_e5144_d_n0;
            locals.var_t5__blk133_dn2 = assign7410_body11_e5144_d_n2;
            locals.var_t5__blk133_dn6 = assign7410_body11_e5144_d_n6;
            locals.var_t5__blk133_dn7 = assign7410_body11_e5144_d_n7;
            locals.var_t5__blk133_dn10 = assign7410_body11_e5144_d_n10;
            locals.var_t5__blk133_dn11 = assign7410_body11_e5144_d_n11;
            locals.var_t5__blk133_dn12 = assign7410_body11_e5144_d_n12;
            locals.var_t5__blk133_dn17 = assign7410_body11_e5144_d_n17;
            locals.var_t5__blk133_rv = 0.0;
            let (assign7410_body12_e5157, assign7410_body12_e5157_d_n0, assign7410_body12_e5157_d_n2, assign7410_body12_e5157_d_n6, assign7410_body12_e5157_d_n7, assign7410_body12_e5157_d_n10, assign7410_body12_e5157_d_n11, assign7410_body12_e5157_d_n12, assign7410_body12_e5157_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body12_e5148: f64 = (locals.var_t4__blk132 * locals.var_t4__blk132);
        let assign7410_body12_e5151: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign7410_body12_e5153: f64 = (assign7410_body12_e5151 * locals.var_q_fd_dlt1);
        let assign7410_body12_e5154: f64 = (assign7410_body12_e5148 + assign7410_body12_e5153);
        let assign7410_body12_e5155: f64 = (assign7410_body12_e5154).sqrt();
        (assign7410_body12_e5155, ((((locals.var_t4__blk132_dn0 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign7410_body12_e5155)), ((((locals.var_t4__blk132_dn2 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign7410_body12_e5155)), ((((locals.var_t4__blk132_dn6 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign7410_body12_e5155)), ((((locals.var_t4__blk132_dn7 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign7410_body12_e5155)), ((((locals.var_t4__blk132_dn10 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign7410_body12_e5155)), ((((locals.var_t4__blk132_dn11 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign7410_body12_e5155)), ((((locals.var_t4__blk132_dn12 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign7410_body12_e5155)), ((((locals.var_t4__blk132_dn17 * locals.var_t4__blk132) + (locals.var_t4__blk132 * locals.var_t4__blk132_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign7410_body12_e5151 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign7410_body12_e5155)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7410_body12_e5157;
            locals.var_tmf2_dn0 = assign7410_body12_e5157_d_n0;
            locals.var_tmf2_dn2 = assign7410_body12_e5157_d_n2;
            locals.var_tmf2_dn6 = assign7410_body12_e5157_d_n6;
            locals.var_tmf2_dn7 = assign7410_body12_e5157_d_n7;
            locals.var_tmf2_dn10 = assign7410_body12_e5157_d_n10;
            locals.var_tmf2_dn11 = assign7410_body12_e5157_d_n11;
            locals.var_tmf2_dn12 = assign7410_body12_e5157_d_n12;
            locals.var_tmf2_dn17 = assign7410_body12_e5157_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7410_body13_e5167, assign7410_body13_e5167_d_n0, assign7410_body13_e5167_d_n2, assign7410_body13_e5167_d_n6, assign7410_body13_e5167_d_n7, assign7410_body13_e5167_d_n10, assign7410_body13_e5167_d_n11, assign7410_body13_e5167_d_n12, assign7410_body13_e5167_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body13_e5163: f64 = (locals.var_t4__blk132 / locals.var_tmf2);
        let assign7410_body13_e5164: f64 = (1.0 + assign7410_body13_e5163);
        let assign7410_body13_e5165: f64 = (0.5 * assign7410_body13_e5164);
        (assign7410_body13_e5165, (0.5 * (((locals.var_t4__blk132_dn0 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk132_dn2 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk132_dn6 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk132_dn7 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk132_dn10 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk132_dn11 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk132_dn12 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk132_dn17 * locals.var_tmf2) - (locals.var_t4__blk132 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk135, locals.var_t7__blk135_dn0, locals.var_t7__blk135_dn2, locals.var_t7__blk135_dn6, locals.var_t7__blk135_dn7, locals.var_t7__blk135_dn10, locals.var_t7__blk135_dn11, locals.var_t7__blk135_dn12, locals.var_t7__blk135_dn17,)
    }
};
            locals.var_t7__blk135 = assign7410_body13_e5167;
            locals.var_t7__blk135_dn0 = assign7410_body13_e5167_d_n0;
            locals.var_t7__blk135_dn2 = assign7410_body13_e5167_d_n2;
            locals.var_t7__blk135_dn6 = assign7410_body13_e5167_d_n6;
            locals.var_t7__blk135_dn7 = assign7410_body13_e5167_d_n7;
            locals.var_t7__blk135_dn10 = assign7410_body13_e5167_d_n10;
            locals.var_t7__blk135_dn11 = assign7410_body13_e5167_d_n11;
            locals.var_t7__blk135_dn12 = assign7410_body13_e5167_d_n12;
            locals.var_t7__blk135_dn17 = assign7410_body13_e5167_d_n17;
            locals.var_t7__blk135_rv = 0.0;
            let (assign7410_body14_e5179, assign7410_body14_e5179_d_n0, assign7410_body14_e5179_d_n2, assign7410_body14_e5179_d_n6, assign7410_body14_e5179_d_n7, assign7410_body14_e5179_d_n10, assign7410_body14_e5179_d_n11, assign7410_body14_e5179_d_n12, assign7410_body14_e5179_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body14_e5172: f64 = (locals.var_t4__blk132 + locals.var_tmf2);
        let assign7410_body14_e5173: f64 = (0.5 * assign7410_body14_e5172);
        let assign7410_body14_e5176: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign7410_body14_e5177: f64 = (assign7410_body14_e5173 + assign7410_body14_e5176);
        (assign7410_body14_e5177, ((0.5 * (locals.var_t4__blk132_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk132_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk132_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk132_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk132_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk132_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk132_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk132_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk134, locals.var_t6__blk134_dn0, locals.var_t6__blk134_dn2, locals.var_t6__blk134_dn6, locals.var_t6__blk134_dn7, locals.var_t6__blk134_dn10, locals.var_t6__blk134_dn11, locals.var_t6__blk134_dn12, locals.var_t6__blk134_dn17,)
    }
};
            locals.var_t6__blk134 = assign7410_body14_e5179;
            locals.var_t6__blk134_dn0 = assign7410_body14_e5179_d_n0;
            locals.var_t6__blk134_dn2 = assign7410_body14_e5179_d_n2;
            locals.var_t6__blk134_dn6 = assign7410_body14_e5179_d_n6;
            locals.var_t6__blk134_dn7 = assign7410_body14_e5179_d_n7;
            locals.var_t6__blk134_dn10 = assign7410_body14_e5179_d_n10;
            locals.var_t6__blk134_dn11 = assign7410_body14_e5179_d_n11;
            locals.var_t6__blk134_dn12 = assign7410_body14_e5179_d_n12;
            locals.var_t6__blk134_dn17 = assign7410_body14_e5179_d_n17;
            locals.var_t6__blk134_rv = 0.0;
            let assign7410_body15_e5182: f64 = if locals.var_t6__blk134 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard138 = assign7410_body15_e5182;
            locals.var_guard138_rv = 0.0;
            let (assign7410_body16_e5188, assign7410_body16_e5188_d_n0, assign7410_body16_e5188_d_n2, assign7410_body16_e5188_d_n6, assign7410_body16_e5188_d_n7, assign7410_body16_e5188_d_n10, assign7410_body16_e5188_d_n11, assign7410_body16_e5188_d_n12, assign7410_body16_e5188_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard138 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk134, locals.var_t6__blk134_dn0, locals.var_t6__blk134_dn2, locals.var_t6__blk134_dn6, locals.var_t6__blk134_dn7, locals.var_t6__blk134_dn10, locals.var_t6__blk134_dn11, locals.var_t6__blk134_dn12, locals.var_t6__blk134_dn17,)
    }
};
            locals.var_t6__blk134 = assign7410_body16_e5188;
            locals.var_t6__blk134_dn0 = assign7410_body16_e5188_d_n0;
            locals.var_t6__blk134_dn2 = assign7410_body16_e5188_d_n2;
            locals.var_t6__blk134_dn6 = assign7410_body16_e5188_d_n6;
            locals.var_t6__blk134_dn7 = assign7410_body16_e5188_d_n7;
            locals.var_t6__blk134_dn10 = assign7410_body16_e5188_d_n10;
            locals.var_t6__blk134_dn11 = assign7410_body16_e5188_d_n11;
            locals.var_t6__blk134_dn12 = assign7410_body16_e5188_d_n12;
            locals.var_t6__blk134_dn17 = assign7410_body16_e5188_d_n17;
            locals.var_t6__blk134_rv = 0.0;
            let (assign7410_body17_e5194, assign7410_body17_e5194_d_n0, assign7410_body17_e5194_d_n2, assign7410_body17_e5194_d_n6, assign7410_body17_e5194_d_n7, assign7410_body17_e5194_d_n10, assign7410_body17_e5194_d_n11, assign7410_body17_e5194_d_n12, assign7410_body17_e5194_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard138 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk135, locals.var_t7__blk135_dn0, locals.var_t7__blk135_dn2, locals.var_t7__blk135_dn6, locals.var_t7__blk135_dn7, locals.var_t7__blk135_dn10, locals.var_t7__blk135_dn11, locals.var_t7__blk135_dn12, locals.var_t7__blk135_dn17,)
    }
};
            locals.var_t7__blk135 = assign7410_body17_e5194;
            locals.var_t7__blk135_dn0 = assign7410_body17_e5194_d_n0;
            locals.var_t7__blk135_dn2 = assign7410_body17_e5194_d_n2;
            locals.var_t7__blk135_dn6 = assign7410_body17_e5194_d_n6;
            locals.var_t7__blk135_dn7 = assign7410_body17_e5194_d_n7;
            locals.var_t7__blk135_dn10 = assign7410_body17_e5194_d_n10;
            locals.var_t7__blk135_dn11 = assign7410_body17_e5194_d_n11;
            locals.var_t7__blk135_dn12 = assign7410_body17_e5194_d_n12;
            locals.var_t7__blk135_dn17 = assign7410_body17_e5194_d_n17;
            locals.var_t7__blk135_rv = 0.0;
            let (assign7410_body18_e5203, assign7410_body18_e5203_d_n0, assign7410_body18_e5203_d_n2, assign7410_body18_e5203_d_n6, assign7410_body18_e5203_d_n7, assign7410_body18_e5203_d_n10, assign7410_body18_e5203_d_n11, assign7410_body18_e5203_d_n12, assign7410_body18_e5203_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body18_e5197: f64 = (-locals.var_q_fd_soi);
        let assign7410_body18_e5199: f64 = (assign7410_body18_e5197 - locals.var_t6__blk134);
        let assign7410_body18_e5201: f64 = (assign7410_body18_e5199 - locals.var_q_fd_dlt2);
        (assign7410_body18_e5201, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk134_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk134_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk134_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk134_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk134_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk134_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk134_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk134_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign7410_body18_e5203;
            locals.var_tmf1_dn0 = assign7410_body18_e5203_d_n0;
            locals.var_tmf1_dn2 = assign7410_body18_e5203_d_n2;
            locals.var_tmf1_dn6 = assign7410_body18_e5203_d_n6;
            locals.var_tmf1_dn7 = assign7410_body18_e5203_d_n7;
            locals.var_tmf1_dn10 = assign7410_body18_e5203_d_n10;
            locals.var_tmf1_dn11 = assign7410_body18_e5203_d_n11;
            locals.var_tmf1_dn12 = assign7410_body18_e5203_d_n12;
            locals.var_tmf1_dn17 = assign7410_body18_e5203_d_n17;
            locals.var_tmf1_rv = 0.0;
            let (assign7410_body19_e5212, assign7410_body19_e5212_d_n0, assign7410_body19_e5212_d_n2, assign7410_body19_e5212_d_n6, assign7410_body19_e5212_d_n7, assign7410_body19_e5212_d_n10, assign7410_body19_e5212_d_n11, assign7410_body19_e5212_d_n12, assign7410_body19_e5212_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body19_e5207: f64 = (-locals.var_q_fd_soi);
        let assign7410_body19_e5208: f64 = (4.0 * assign7410_body19_e5207);
        let assign7410_body19_e5210: f64 = (assign7410_body19_e5208 * locals.var_q_fd_dlt2);
        (assign7410_body19_e5210, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign7410_body19_e5208 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7410_body19_e5212;
            locals.var_tmf2_dn0 = assign7410_body19_e5212_d_n0;
            locals.var_tmf2_dn2 = assign7410_body19_e5212_d_n2;
            locals.var_tmf2_dn6 = assign7410_body19_e5212_d_n6;
            locals.var_tmf2_dn7 = assign7410_body19_e5212_d_n7;
            locals.var_tmf2_dn10 = assign7410_body19_e5212_d_n10;
            locals.var_tmf2_dn11 = assign7410_body19_e5212_d_n11;
            locals.var_tmf2_dn12 = assign7410_body19_e5212_d_n12;
            locals.var_tmf2_dn17 = assign7410_body19_e5212_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7410_body20_e5222, assign7410_body20_e5222_d_n0, assign7410_body20_e5222_d_n2, assign7410_body20_e5222_d_n6, assign7410_body20_e5222_d_n7, assign7410_body20_e5222_d_n10, assign7410_body20_e5222_d_n11, assign7410_body20_e5222_d_n12, assign7410_body20_e5222_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let (assign7410_body20_e5220, assign7410_body20_e5220_d_n0, assign7410_body20_e5220_d_n2, assign7410_body20_e5220_d_n6, assign7410_body20_e5220_d_n7, assign7410_body20_e5220_d_n10, assign7410_body20_e5220_d_n11, assign7410_body20_e5220_d_n12, assign7410_body20_e5220_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign7410_body20_e5219: f64 = (-locals.var_tmf2);
                (assign7410_body20_e5219, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign7410_body20_e5220, assign7410_body20_e5220_d_n0, assign7410_body20_e5220_d_n2, assign7410_body20_e5220_d_n6, assign7410_body20_e5220_d_n7, assign7410_body20_e5220_d_n10, assign7410_body20_e5220_d_n11, assign7410_body20_e5220_d_n12, assign7410_body20_e5220_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7410_body20_e5222;
            locals.var_tmf2_dn0 = assign7410_body20_e5222_d_n0;
            locals.var_tmf2_dn2 = assign7410_body20_e5222_d_n2;
            locals.var_tmf2_dn6 = assign7410_body20_e5222_d_n6;
            locals.var_tmf2_dn7 = assign7410_body20_e5222_d_n7;
            locals.var_tmf2_dn10 = assign7410_body20_e5222_d_n10;
            locals.var_tmf2_dn11 = assign7410_body20_e5222_d_n11;
            locals.var_tmf2_dn12 = assign7410_body20_e5222_d_n12;
            locals.var_tmf2_dn17 = assign7410_body20_e5222_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7410_body21_e5231, assign7410_body21_e5231_d_n0, assign7410_body21_e5231_d_n2, assign7410_body21_e5231_d_n6, assign7410_body21_e5231_d_n7, assign7410_body21_e5231_d_n10, assign7410_body21_e5231_d_n11, assign7410_body21_e5231_d_n12, assign7410_body21_e5231_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body21_e5226: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign7410_body21_e5228: f64 = (assign7410_body21_e5226 + locals.var_tmf2);
        let assign7410_body21_e5229: f64 = (assign7410_body21_e5228).sqrt();
        (assign7410_body21_e5229, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign7410_body21_e5229)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign7410_body21_e5229)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign7410_body21_e5229)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign7410_body21_e5229)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign7410_body21_e5229)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign7410_body21_e5229)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign7410_body21_e5229)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign7410_body21_e5229)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7410_body21_e5231;
            locals.var_tmf2_dn0 = assign7410_body21_e5231_d_n0;
            locals.var_tmf2_dn2 = assign7410_body21_e5231_d_n2;
            locals.var_tmf2_dn6 = assign7410_body21_e5231_d_n6;
            locals.var_tmf2_dn7 = assign7410_body21_e5231_d_n7;
            locals.var_tmf2_dn10 = assign7410_body21_e5231_d_n10;
            locals.var_tmf2_dn11 = assign7410_body21_e5231_d_n11;
            locals.var_tmf2_dn12 = assign7410_body21_e5231_d_n12;
            locals.var_tmf2_dn17 = assign7410_body21_e5231_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7410_body22_e5241, assign7410_body22_e5241_d_n0, assign7410_body22_e5241_d_n2, assign7410_body22_e5241_d_n6, assign7410_body22_e5241_d_n7, assign7410_body22_e5241_d_n10, assign7410_body22_e5241_d_n11, assign7410_body22_e5241_d_n12, assign7410_body22_e5241_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body22_e5237: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign7410_body22_e5238: f64 = (1.0 + assign7410_body22_e5237);
        let assign7410_body22_e5239: f64 = (0.5 * assign7410_body22_e5238);
        (assign7410_body22_e5239, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign7410_body22_e5241;
            locals.var_t8_dn0 = assign7410_body22_e5241_d_n0;
            locals.var_t8_dn2 = assign7410_body22_e5241_d_n2;
            locals.var_t8_dn6 = assign7410_body22_e5241_d_n6;
            locals.var_t8_dn7 = assign7410_body22_e5241_d_n7;
            locals.var_t8_dn10 = assign7410_body22_e5241_d_n10;
            locals.var_t8_dn11 = assign7410_body22_e5241_d_n11;
            locals.var_t8_dn12 = assign7410_body22_e5241_d_n12;
            locals.var_t8_dn17 = assign7410_body22_e5241_d_n17;
            locals.var_t8_rv = 0.0;
            let (assign7410_body23_e5252, assign7410_body23_e5252_d_n0, assign7410_body23_e5252_d_n2, assign7410_body23_e5252_d_n6, assign7410_body23_e5252_d_n7, assign7410_body23_e5252_d_n10, assign7410_body23_e5252_d_n11, assign7410_body23_e5252_d_n12, assign7410_body23_e5252_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body23_e5244: f64 = (-locals.var_q_fd_soi);
        let assign7410_body23_e5248: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign7410_body23_e5249: f64 = (0.5 * assign7410_body23_e5248);
        let assign7410_body23_e5250: f64 = (assign7410_body23_e5244 - assign7410_body23_e5249);
        (assign7410_body23_e5250, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk134, locals.var_t6__blk134_dn0, locals.var_t6__blk134_dn2, locals.var_t6__blk134_dn6, locals.var_t6__blk134_dn7, locals.var_t6__blk134_dn10, locals.var_t6__blk134_dn11, locals.var_t6__blk134_dn12, locals.var_t6__blk134_dn17,)
    }
};
            locals.var_t6__blk134 = assign7410_body23_e5252;
            locals.var_t6__blk134_dn0 = assign7410_body23_e5252_d_n0;
            locals.var_t6__blk134_dn2 = assign7410_body23_e5252_d_n2;
            locals.var_t6__blk134_dn6 = assign7410_body23_e5252_d_n6;
            locals.var_t6__blk134_dn7 = assign7410_body23_e5252_d_n7;
            locals.var_t6__blk134_dn10 = assign7410_body23_e5252_d_n10;
            locals.var_t6__blk134_dn11 = assign7410_body23_e5252_d_n11;
            locals.var_t6__blk134_dn12 = assign7410_body23_e5252_d_n12;
            locals.var_t6__blk134_dn17 = assign7410_body23_e5252_d_n17;
            locals.var_t6__blk134_rv = 0.0;
            let (assign7410_body24_e5260, assign7410_body24_e5260_d_n0, assign7410_body24_e5260_d_n2, assign7410_body24_e5260_d_n6, assign7410_body24_e5260_d_n7, assign7410_body24_e5260_d_n10, assign7410_body24_e5260_d_n11, assign7410_body24_e5260_d_n12, assign7410_body24_e5260_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body24_e5257: f64 = (locals.var_t5__blk133 * locals.var_t8);
        let assign7410_body24_e5258: f64 = (locals.var_t7__blk135 * assign7410_body24_e5257);
        (assign7410_body24_e5258, ((locals.var_t7__blk135_dn0 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn0 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn0)))), ((locals.var_t7__blk135_dn2 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn2 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn2)))), ((locals.var_t7__blk135_dn6 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn6 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn6)))), ((locals.var_t7__blk135_dn7 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn7 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn7)))), ((locals.var_t7__blk135_dn10 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn10 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn10)))), ((locals.var_t7__blk135_dn11 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn11 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn11)))), ((locals.var_t7__blk135_dn12 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn12 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn12)))), ((locals.var_t7__blk135_dn17 * assign7410_body24_e5257) + (locals.var_t7__blk135 * ((locals.var_t5__blk133_dn17 * locals.var_t8) + (locals.var_t5__blk133 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk135, locals.var_t7__blk135_dn0, locals.var_t7__blk135_dn2, locals.var_t7__blk135_dn6, locals.var_t7__blk135_dn7, locals.var_t7__blk135_dn10, locals.var_t7__blk135_dn11, locals.var_t7__blk135_dn12, locals.var_t7__blk135_dn17,)
    }
};
            locals.var_t7__blk135 = assign7410_body24_e5260;
            locals.var_t7__blk135_dn0 = assign7410_body24_e5260_d_n0;
            locals.var_t7__blk135_dn2 = assign7410_body24_e5260_d_n2;
            locals.var_t7__blk135_dn6 = assign7410_body24_e5260_d_n6;
            locals.var_t7__blk135_dn7 = assign7410_body24_e5260_d_n7;
            locals.var_t7__blk135_dn10 = assign7410_body24_e5260_d_n10;
            locals.var_t7__blk135_dn11 = assign7410_body24_e5260_d_n11;
            locals.var_t7__blk135_dn12 = assign7410_body24_e5260_d_n12;
            locals.var_t7__blk135_dn17 = assign7410_body24_e5260_d_n17;
            locals.var_t7__blk135_rv = 0.0;
            let (assign7410_body25_e5274, assign7410_body25_e5274_d_n0, assign7410_body25_e5274_d_n2, assign7410_body25_e5274_d_n6, assign7410_body25_e5274_d_n7, assign7410_body25_e5274_d_n10, assign7410_body25_e5274_d_n11, assign7410_body25_e5274_d_n12, assign7410_body25_e5274_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body25_e5264: f64 = (locals.var_t6__blk134 * locals.var_t6__blk134);
        let assign7410_body25_e5266: f64 = (assign7410_body25_e5264 / 2.0);
        let assign7410_body25_e5268: f64 = (assign7410_body25_e5266 / 1.034943e-10);
        let assign7410_body25_e5270: f64 = (assign7410_body25_e5268 / 1.6021918e-19);
        let assign7410_body25_e5272: f64 = (assign7410_body25_e5270 / locals.var_uc_nsubs);
        (assign7410_body25_e5272, ((((((((locals.var_t6__blk134_dn0 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk134_dn2 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk134_dn6 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk134_dn7 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk134_dn10 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk134_dn11 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk134_dn12 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk134_dn17 * locals.var_t6__blk134) + (locals.var_t6__blk134 * locals.var_t6__blk134_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7410_body25_e5270 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep0, locals.var_phi_b_dep0_dn0, locals.var_phi_b_dep0_dn2, locals.var_phi_b_dep0_dn6, locals.var_phi_b_dep0_dn7, locals.var_phi_b_dep0_dn10, locals.var_phi_b_dep0_dn11, locals.var_phi_b_dep0_dn12, locals.var_phi_b_dep0_dn17,)
    }
};
            locals.var_phi_b_dep0 = assign7410_body25_e5274;
            locals.var_phi_b_dep0_dn0 = assign7410_body25_e5274_d_n0;
            locals.var_phi_b_dep0_dn2 = assign7410_body25_e5274_d_n2;
            locals.var_phi_b_dep0_dn6 = assign7410_body25_e5274_d_n6;
            locals.var_phi_b_dep0_dn7 = assign7410_body25_e5274_d_n7;
            locals.var_phi_b_dep0_dn10 = assign7410_body25_e5274_d_n10;
            locals.var_phi_b_dep0_dn11 = assign7410_body25_e5274_d_n11;
            locals.var_phi_b_dep0_dn12 = assign7410_body25_e5274_d_n12;
            locals.var_phi_b_dep0_dn17 = assign7410_body25_e5274_d_n17;
            locals.var_phi_b_dep0_rv = 0.0;
            let (assign7410_body26_e5284, assign7410_body26_e5284_d_n0, assign7410_body26_e5284_d_n2, assign7410_body26_e5284_d_n6, assign7410_body26_e5284_d_n7, assign7410_body26_e5284_d_n10, assign7410_body26_e5284_d_n11, assign7410_body26_e5284_d_n12, assign7410_body26_e5284_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body26_e5278: f64 = (2.0 * locals.var_phi_b_dep0);
        let assign7410_body26_e5280: f64 = (assign7410_body26_e5278 * locals.var_t7__blk135);
        let assign7410_body26_e5282: f64 = (assign7410_body26_e5280 / locals.var_t6__blk134);
        (assign7410_body26_e5282, ((((((2.0 * locals.var_phi_b_dep0_dn0) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn0)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn0)) / (locals.var_t6__blk134 * locals.var_t6__blk134)), ((((((2.0 * locals.var_phi_b_dep0_dn2) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn2)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn2)) / (locals.var_t6__blk134 * locals.var_t6__blk134)), ((((((2.0 * locals.var_phi_b_dep0_dn6) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn6)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn6)) / (locals.var_t6__blk134 * locals.var_t6__blk134)), ((((((2.0 * locals.var_phi_b_dep0_dn7) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn7)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn7)) / (locals.var_t6__blk134 * locals.var_t6__blk134)), ((((((2.0 * locals.var_phi_b_dep0_dn10) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn10)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn10)) / (locals.var_t6__blk134 * locals.var_t6__blk134)), ((((((2.0 * locals.var_phi_b_dep0_dn11) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn11)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn11)) / (locals.var_t6__blk134 * locals.var_t6__blk134)), ((((((2.0 * locals.var_phi_b_dep0_dn12) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn12)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn12)) / (locals.var_t6__blk134 * locals.var_t6__blk134)), ((((((2.0 * locals.var_phi_b_dep0_dn17) * locals.var_t7__blk135) + (assign7410_body26_e5278 * locals.var_t7__blk135_dn17)) * locals.var_t6__blk134) - (assign7410_body26_e5280 * locals.var_t6__blk134_dn17)) / (locals.var_t6__blk134 * locals.var_t6__blk134)),)
    } else {
        (locals.var_phi_b_dep0_dpsb, locals.var_phi_b_dep0_dpsb_dn0, locals.var_phi_b_dep0_dpsb_dn2, locals.var_phi_b_dep0_dpsb_dn6, locals.var_phi_b_dep0_dpsb_dn7, locals.var_phi_b_dep0_dpsb_dn10, locals.var_phi_b_dep0_dpsb_dn11, locals.var_phi_b_dep0_dpsb_dn12, locals.var_phi_b_dep0_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep0_dpsb = assign7410_body26_e5284;
            locals.var_phi_b_dep0_dpsb_dn0 = assign7410_body26_e5284_d_n0;
            locals.var_phi_b_dep0_dpsb_dn2 = assign7410_body26_e5284_d_n2;
            locals.var_phi_b_dep0_dpsb_dn6 = assign7410_body26_e5284_d_n6;
            locals.var_phi_b_dep0_dpsb_dn7 = assign7410_body26_e5284_d_n7;
            locals.var_phi_b_dep0_dpsb_dn10 = assign7410_body26_e5284_d_n10;
            locals.var_phi_b_dep0_dpsb_dn11 = assign7410_body26_e5284_d_n11;
            locals.var_phi_b_dep0_dpsb_dn12 = assign7410_body26_e5284_d_n12;
            locals.var_phi_b_dep0_dpsb_dn17 = assign7410_body26_e5284_d_n17;
            locals.var_phi_b_dep0_dpsb_rv = 0.0;
            let (assign7410_body27_e5308, assign7410_body27_e5308_d_n0, assign7410_body27_e5308_d_n2, assign7410_body27_e5308_d_n6, assign7410_body27_e5308_d_n7, assign7410_body27_e5308_d_n10, assign7410_body27_e5308_d_n11, assign7410_body27_e5308_d_n12, assign7410_body27_e5308_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body27_e5288: f64 = (-locals.var_phi_s0_bulk_0);
        let assign7410_body27_e5291: f64 = (locals.var_t4__blk132 / locals.var_c_box);
        let assign7410_body27_e5292: f64 = (assign7410_body27_e5288 + assign7410_body27_e5291);
        let assign7410_body27_e5294: f64 = (assign7410_body27_e5292 - locals.var_vbsbiz);
        let assign7410_body27_e5296: f64 = (assign7410_body27_e5294 + locals.var_phi_b_dep0);
        let assign7410_body27_e5298: f64 = (-1.0);
        let assign7410_body27_e5301: f64 = (locals.var_t5__blk133 / locals.var_c_box);
        let assign7410_body27_e5302: f64 = (assign7410_body27_e5298 + assign7410_body27_e5301);
        let assign7410_body27_e5304: f64 = (assign7410_body27_e5302 + locals.var_phi_b_dep0_dpsb);
        let assign7410_body27_e5305: f64 = (assign7410_body27_e5296 / assign7410_body27_e5304);
        let assign7410_body27_e5306: f64 = (locals.var_phi_s0_bulk_0 - assign7410_body27_e5305);
        (assign7410_body27_e5306, (locals.var_phi_s0_bulk_0_dn0 - (((((((-locals.var_phi_s0_bulk_0_dn0) + (locals.var_t4__blk132_dn0 / locals.var_c_box)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep0_dn0) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn0 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn0))) / (assign7410_body27_e5304 * assign7410_body27_e5304))), (locals.var_phi_s0_bulk_0_dn2 - (((((((-locals.var_phi_s0_bulk_0_dn2) + (locals.var_t4__blk132_dn2 / locals.var_c_box)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep0_dn2) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn2 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn2))) / (assign7410_body27_e5304 * assign7410_body27_e5304))), (locals.var_phi_s0_bulk_0_dn6 - (((((((-locals.var_phi_s0_bulk_0_dn6) + (locals.var_t4__blk132_dn6 / locals.var_c_box)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep0_dn6) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn6 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn6))) / (assign7410_body27_e5304 * assign7410_body27_e5304))), (locals.var_phi_s0_bulk_0_dn7 - (((((((-locals.var_phi_s0_bulk_0_dn7) + (locals.var_t4__blk132_dn7 / locals.var_c_box)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep0_dn7) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn7 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn7))) / (assign7410_body27_e5304 * assign7410_body27_e5304))), (locals.var_phi_s0_bulk_0_dn10 - (((((((-locals.var_phi_s0_bulk_0_dn10) + (locals.var_t4__blk132_dn10 / locals.var_c_box)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep0_dn10) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn10 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn10))) / (assign7410_body27_e5304 * assign7410_body27_e5304))), (locals.var_phi_s0_bulk_0_dn11 - (((((((-locals.var_phi_s0_bulk_0_dn11) + (locals.var_t4__blk132_dn11 / locals.var_c_box)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep0_dn11) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn11 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn11))) / (assign7410_body27_e5304 * assign7410_body27_e5304))), (locals.var_phi_s0_bulk_0_dn12 - (((((((-locals.var_phi_s0_bulk_0_dn12) + (locals.var_t4__blk132_dn12 / locals.var_c_box)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep0_dn12) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn12 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn12))) / (assign7410_body27_e5304 * assign7410_body27_e5304))), (locals.var_phi_s0_bulk_0_dn17 - (((((((-locals.var_phi_s0_bulk_0_dn17) + (locals.var_t4__blk132_dn17 / locals.var_c_box)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep0_dn17) * assign7410_body27_e5304) - (assign7410_body27_e5296 * ((locals.var_t5__blk133_dn17 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn17))) / (assign7410_body27_e5304 * assign7410_body27_e5304))),)
    } else {
        (locals.var_t6__blk134, locals.var_t6__blk134_dn0, locals.var_t6__blk134_dn2, locals.var_t6__blk134_dn6, locals.var_t6__blk134_dn7, locals.var_t6__blk134_dn10, locals.var_t6__blk134_dn11, locals.var_t6__blk134_dn12, locals.var_t6__blk134_dn17,)
    }
};
            locals.var_t6__blk134 = assign7410_body27_e5308;
            locals.var_t6__blk134_dn0 = assign7410_body27_e5308_d_n0;
            locals.var_t6__blk134_dn2 = assign7410_body27_e5308_d_n2;
            locals.var_t6__blk134_dn6 = assign7410_body27_e5308_d_n6;
            locals.var_t6__blk134_dn7 = assign7410_body27_e5308_d_n7;
            locals.var_t6__blk134_dn10 = assign7410_body27_e5308_d_n10;
            locals.var_t6__blk134_dn11 = assign7410_body27_e5308_d_n11;
            locals.var_t6__blk134_dn12 = assign7410_body27_e5308_d_n12;
            locals.var_t6__blk134_dn17 = assign7410_body27_e5308_d_n17;
            locals.var_t6__blk134_rv = 0.0;
            let assign7410_body28_e5311: f64 = (locals.var_t6__blk134 - locals.var_phi_s0_bulk_0);
            let assign7410_body28_e5312: f64 = (assign7410_body28_e5311).abs();
            let assign7410_body28_e5314: f64 = if assign7410_body28_e5312 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard139 = assign7410_body28_e5314;
            locals.var_guard139_rv = 0.0;
            let (assign7410_body29_e5320,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard139 != 0.0)) {
        (locals.var_lp_s0_max,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign7410_body29_e5320;
            locals.var_lp_s0_rv = 0.0;
            let (assign7410_body30_e5324, assign7410_body30_e5324_d_n0, assign7410_body30_e5324_d_n2, assign7410_body30_e5324_d_n6, assign7410_body30_e5324_d_n7, assign7410_body30_e5324_d_n10, assign7410_body30_e5324_d_n11, assign7410_body30_e5324_d_n12, assign7410_body30_e5324_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        (locals.var_t6__blk134, locals.var_t6__blk134_dn0, locals.var_t6__blk134_dn2, locals.var_t6__blk134_dn6, locals.var_t6__blk134_dn7, locals.var_t6__blk134_dn10, locals.var_t6__blk134_dn11, locals.var_t6__blk134_dn12, locals.var_t6__blk134_dn17,)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
            locals.var_phi_s0_bulk_0 = assign7410_body30_e5324;
            locals.var_phi_s0_bulk_0_dn0 = assign7410_body30_e5324_d_n0;
            locals.var_phi_s0_bulk_0_dn2 = assign7410_body30_e5324_d_n2;
            locals.var_phi_s0_bulk_0_dn6 = assign7410_body30_e5324_d_n6;
            locals.var_phi_s0_bulk_0_dn7 = assign7410_body30_e5324_d_n7;
            locals.var_phi_s0_bulk_0_dn10 = assign7410_body30_e5324_d_n10;
            locals.var_phi_s0_bulk_0_dn11 = assign7410_body30_e5324_d_n11;
            locals.var_phi_s0_bulk_0_dn12 = assign7410_body30_e5324_d_n12;
            locals.var_phi_s0_bulk_0_dn17 = assign7410_body30_e5324_d_n17;
            locals.var_phi_s0_bulk_0_rv = 0.0;
            let (assign7410_body31_e5328, assign7410_body31_e5328_d_n0, assign7410_body31_e5328_d_n2, assign7410_body31_e5328_d_n6, assign7410_body31_e5328_d_n7, assign7410_body31_e5328_d_n10, assign7410_body31_e5328_d_n11, assign7410_body31_e5328_d_n12, assign7410_body31_e5328_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        (locals.var_t4__blk132, locals.var_t4__blk132_dn0, locals.var_t4__blk132_dn2, locals.var_t4__blk132_dn6, locals.var_t4__blk132_dn7, locals.var_t4__blk132_dn10, locals.var_t4__blk132_dn11, locals.var_t4__blk132_dn12, locals.var_t4__blk132_dn17,)
    } else {
        (locals.var_q_s0_bulk_0, locals.var_q_s0_bulk_0_dn0, locals.var_q_s0_bulk_0_dn2, locals.var_q_s0_bulk_0_dn6, locals.var_q_s0_bulk_0_dn7, locals.var_q_s0_bulk_0_dn10, locals.var_q_s0_bulk_0_dn11, locals.var_q_s0_bulk_0_dn12, locals.var_q_s0_bulk_0_dn17,)
    }
};
            locals.var_q_s0_bulk_0 = assign7410_body31_e5328;
            locals.var_q_s0_bulk_0_dn0 = assign7410_body31_e5328_d_n0;
            locals.var_q_s0_bulk_0_dn2 = assign7410_body31_e5328_d_n2;
            locals.var_q_s0_bulk_0_dn6 = assign7410_body31_e5328_d_n6;
            locals.var_q_s0_bulk_0_dn7 = assign7410_body31_e5328_d_n7;
            locals.var_q_s0_bulk_0_dn10 = assign7410_body31_e5328_d_n10;
            locals.var_q_s0_bulk_0_dn11 = assign7410_body31_e5328_d_n11;
            locals.var_q_s0_bulk_0_dn12 = assign7410_body31_e5328_d_n12;
            locals.var_q_s0_bulk_0_dn17 = assign7410_body31_e5328_d_n17;
            locals.var_q_s0_bulk_0_rv = 0.0;
            let (assign7410_body32_e5334,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7410_body32_e5332: f64 = (locals.var_lp_s0 + 1.0);
        (assign7410_body32_e5332,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign7410_body32_e5334;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_18(
        locals: &mut StampLocals,
    ) {
        let (assign7420_e5338, assign7420_e5338_d_n0, assign7420_e5338_d_n2, assign7420_e5338_d_n6, assign7420_e5338_d_n7, assign7420_e5338_d_n10, assign7420_e5338_d_n11, assign7420_e5338_d_n12, assign7420_e5338_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        (locals.var_phi_b_dep0, locals.var_phi_b_dep0_dn0, locals.var_phi_b_dep0_dn2, locals.var_phi_b_dep0_dn6, locals.var_phi_b_dep0_dn7, locals.var_phi_b_dep0_dn10, locals.var_phi_b_dep0_dn11, locals.var_phi_b_dep0_dn12, locals.var_phi_b_dep0_dn17,)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
        locals.var_phi_b_dep = assign7420_e5338;
        locals.var_phi_b_dep_dn0 = assign7420_e5338_d_n0;
        locals.var_phi_b_dep_dn2 = assign7420_e5338_d_n2;
        locals.var_phi_b_dep_dn6 = assign7420_e5338_d_n6;
        locals.var_phi_b_dep_dn7 = assign7420_e5338_d_n7;
        locals.var_phi_b_dep_dn10 = assign7420_e5338_d_n10;
        locals.var_phi_b_dep_dn11 = assign7420_e5338_d_n11;
        locals.var_phi_b_dep_dn12 = assign7420_e5338_d_n12;
        locals.var_phi_b_dep_dn17 = assign7420_e5338_d_n17;
        locals.var_phi_b_dep_rv = 0.0;

        let (assign7430_e5351, assign7430_e5351_d_n0, assign7430_e5351_d_n2, assign7430_e5351_d_n6, assign7430_e5351_d_n7, assign7430_e5351_d_n10, assign7430_e5351_d_n11, assign7430_e5351_d_n12, assign7430_e5351_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7430_e5342: f64 = (2.0 * 1.034943e-10);
        let assign7430_e5344: f64 = (assign7430_e5342 / 1.6021918e-19);
        let assign7430_e5346: f64 = (assign7430_e5344 * locals.var_phi_b_dep);
        let assign7430_e5348: f64 = (assign7430_e5346 / locals.var_uc_nsubs);
        let assign7430_e5349: f64 = (assign7430_e5348).sqrt();
        (assign7430_e5349, (((((assign7430_e5344 * locals.var_phi_b_dep_dn0) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)), (((((assign7430_e5344 * locals.var_phi_b_dep_dn2) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)), (((((assign7430_e5344 * locals.var_phi_b_dep_dn6) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)), (((((assign7430_e5344 * locals.var_phi_b_dep_dn7) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)), (((((assign7430_e5344 * locals.var_phi_b_dep_dn10) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)), (((((assign7430_e5344 * locals.var_phi_b_dep_dn11) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)), (((((assign7430_e5344 * locals.var_phi_b_dep_dn12) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)), (((((assign7430_e5344 * locals.var_phi_b_dep_dn17) * locals.var_uc_nsubs) - (assign7430_e5346 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7430_e5349)),)
    } else {
        (locals.var_t1__blk141, locals.var_t1__blk141_dn0, locals.var_t1__blk141_dn2, locals.var_t1__blk141_dn6, locals.var_t1__blk141_dn7, locals.var_t1__blk141_dn10, locals.var_t1__blk141_dn11, locals.var_t1__blk141_dn12, locals.var_t1__blk141_dn17,)
    }
};
        locals.var_t1__blk141 = assign7430_e5351;
        locals.var_t1__blk141_dn0 = assign7430_e5351_d_n0;
        locals.var_t1__blk141_dn2 = assign7430_e5351_d_n2;
        locals.var_t1__blk141_dn6 = assign7430_e5351_d_n6;
        locals.var_t1__blk141_dn7 = assign7430_e5351_d_n7;
        locals.var_t1__blk141_dn10 = assign7430_e5351_d_n10;
        locals.var_t1__blk141_dn11 = assign7430_e5351_d_n11;
        locals.var_t1__blk141_dn12 = assign7430_e5351_d_n12;
        locals.var_t1__blk141_dn17 = assign7430_e5351_d_n17;
        locals.var_t1__blk141_rv = 0.0;

        let assign7440_e5355: f64 = (0.99 * locals.var_t_soi);
        let assign7440_e5356: f64 = if locals.var_t1__blk141 > assign7440_e5355 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7440_e5356;
        locals.var_guard146_rv = 0.0;

        let (assign7450_e5364, assign7450_e5364_d_n0, assign7450_e5364_d_n2, assign7450_e5364_d_n6, assign7450_e5364_d_n7, assign7450_e5364_d_n10, assign7450_e5364_d_n11, assign7450_e5364_d_n12, assign7450_e5364_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7450_e5362: f64 = (1.0 / locals.var_c_fox);
        (assign7450_e5362, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0__blk140, locals.var_t0__blk140_dn0, locals.var_t0__blk140_dn2, locals.var_t0__blk140_dn6, locals.var_t0__blk140_dn7, locals.var_t0__blk140_dn10, locals.var_t0__blk140_dn11, locals.var_t0__blk140_dn12, locals.var_t0__blk140_dn17,)
    }
};
        locals.var_t0__blk140 = assign7450_e5364;
        locals.var_t0__blk140_dn0 = assign7450_e5364_d_n0;
        locals.var_t0__blk140_dn2 = assign7450_e5364_d_n2;
        locals.var_t0__blk140_dn6 = assign7450_e5364_d_n6;
        locals.var_t0__blk140_dn7 = assign7450_e5364_d_n7;
        locals.var_t0__blk140_dn10 = assign7450_e5364_d_n10;
        locals.var_t0__blk140_dn11 = assign7450_e5364_d_n11;
        locals.var_t0__blk140_dn12 = assign7450_e5364_d_n12;
        locals.var_t0__blk140_dn17 = assign7450_e5364_d_n17;
        locals.var_t0__blk140_rv = 0.0;

        let (assign7460_e5372, assign7460_e5372_d_n0, assign7460_e5372_d_n2, assign7460_e5372_d_n6, assign7460_e5372_d_n7, assign7460_e5372_d_n10, assign7460_e5372_d_n11, assign7460_e5372_d_n12, assign7460_e5372_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7460_e5370: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign7460_e5370, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk141, locals.var_t1__blk141_dn0, locals.var_t1__blk141_dn2, locals.var_t1__blk141_dn6, locals.var_t1__blk141_dn7, locals.var_t1__blk141_dn10, locals.var_t1__blk141_dn11, locals.var_t1__blk141_dn12, locals.var_t1__blk141_dn17,)
    }
};
        locals.var_t1__blk141 = assign7460_e5372;
        locals.var_t1__blk141_dn0 = assign7460_e5372_d_n0;
        locals.var_t1__blk141_dn2 = assign7460_e5372_d_n2;
        locals.var_t1__blk141_dn6 = assign7460_e5372_d_n6;
        locals.var_t1__blk141_dn7 = assign7460_e5372_d_n7;
        locals.var_t1__blk141_dn10 = assign7460_e5372_d_n10;
        locals.var_t1__blk141_dn11 = assign7460_e5372_d_n11;
        locals.var_t1__blk141_dn12 = assign7460_e5372_d_n12;
        locals.var_t1__blk141_dn17 = assign7460_e5372_d_n17;
        locals.var_t1__blk141_rv = 0.0;

        let (assign7470_e5380,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7470_e5378: f64 = (1.0 / locals.var_c_box);
        (assign7470_e5378,)
    } else {
        (locals.var_t2__blk142,)
    }
};
        locals.var_t2__blk142 = assign7470_e5380;
        locals.var_t2__blk142_rv = 0.0;

        let (assign7480_e5392, assign7480_e5392_d_n0, assign7480_e5392_d_n2, assign7480_e5392_d_n6, assign7480_e5392_d_n7, assign7480_e5392_d_n10, assign7480_e5392_d_n11, assign7480_e5392_d_n12, assign7480_e5392_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7480_e5387: f64 = (locals.var_t0__blk140 + locals.var_t1__blk141);
        let assign7480_e5389: f64 = (assign7480_e5387 + locals.var_t2__blk142);
        let assign7480_e5390: f64 = (1.0 / assign7480_e5389);
        (assign7480_e5390, (-((locals.var_t0__blk140_dn0 + locals.var_t1__blk141_dn0) / (assign7480_e5389 * assign7480_e5389))), (-((locals.var_t0__blk140_dn2 + locals.var_t1__blk141_dn2) / (assign7480_e5389 * assign7480_e5389))), (-((locals.var_t0__blk140_dn6 + locals.var_t1__blk141_dn6) / (assign7480_e5389 * assign7480_e5389))), (-((locals.var_t0__blk140_dn7 + locals.var_t1__blk141_dn7) / (assign7480_e5389 * assign7480_e5389))), (-((locals.var_t0__blk140_dn10 + locals.var_t1__blk141_dn10) / (assign7480_e5389 * assign7480_e5389))), (-((locals.var_t0__blk140_dn11 + locals.var_t1__blk141_dn11) / (assign7480_e5389 * assign7480_e5389))), (-((locals.var_t0__blk140_dn12 + locals.var_t1__blk141_dn12) / (assign7480_e5389 * assign7480_e5389))), (-((locals.var_t0__blk140_dn17 + locals.var_t1__blk141_dn17) / (assign7480_e5389 * assign7480_e5389))),)
    } else {
        (locals.var_t3__blk143, locals.var_t3__blk143_dn0, locals.var_t3__blk143_dn2, locals.var_t3__blk143_dn6, locals.var_t3__blk143_dn7, locals.var_t3__blk143_dn10, locals.var_t3__blk143_dn11, locals.var_t3__blk143_dn12, locals.var_t3__blk143_dn17,)
    }
};
        locals.var_t3__blk143 = assign7480_e5392;
        locals.var_t3__blk143_dn0 = assign7480_e5392_d_n0;
        locals.var_t3__blk143_dn2 = assign7480_e5392_d_n2;
        locals.var_t3__blk143_dn6 = assign7480_e5392_d_n6;
        locals.var_t3__blk143_dn7 = assign7480_e5392_d_n7;
        locals.var_t3__blk143_dn10 = assign7480_e5392_d_n10;
        locals.var_t3__blk143_dn11 = assign7480_e5392_d_n11;
        locals.var_t3__blk143_dn12 = assign7480_e5392_d_n12;
        locals.var_t3__blk143_dn17 = assign7480_e5392_d_n17;
        locals.var_t3__blk143_rv = 0.0;

        let (assign7490_e5402, assign7490_e5402_d_n0, assign7490_e5402_d_n2, assign7490_e5402_d_n6, assign7490_e5402_d_n7, assign7490_e5402_d_n10, assign7490_e5402_d_n11, assign7490_e5402_d_n12, assign7490_e5402_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7490_e5399: f64 = (locals.var_t3__blk143 * locals.var_t0__blk140);
        let assign7490_e5400: f64 = (1.0 - assign7490_e5399);
        (assign7490_e5400, (-((locals.var_t3__blk143_dn0 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn0))), (-((locals.var_t3__blk143_dn2 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn2))), (-((locals.var_t3__blk143_dn6 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn6))), (-((locals.var_t3__blk143_dn7 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn7))), (-((locals.var_t3__blk143_dn10 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn10))), (-((locals.var_t3__blk143_dn11 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn11))), (-((locals.var_t3__blk143_dn12 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn12))), (-((locals.var_t3__blk143_dn17 * locals.var_t0__blk140) + (locals.var_t3__blk143 * locals.var_t0__blk140_dn17))),)
    } else {
        (locals.var_t4__blk144, locals.var_t4__blk144_dn0, locals.var_t4__blk144_dn2, locals.var_t4__blk144_dn6, locals.var_t4__blk144_dn7, locals.var_t4__blk144_dn10, locals.var_t4__blk144_dn11, locals.var_t4__blk144_dn12, locals.var_t4__blk144_dn17,)
    }
};
        locals.var_t4__blk144 = assign7490_e5402;
        locals.var_t4__blk144_dn0 = assign7490_e5402_d_n0;
        locals.var_t4__blk144_dn2 = assign7490_e5402_d_n2;
        locals.var_t4__blk144_dn6 = assign7490_e5402_d_n6;
        locals.var_t4__blk144_dn7 = assign7490_e5402_d_n7;
        locals.var_t4__blk144_dn10 = assign7490_e5402_d_n10;
        locals.var_t4__blk144_dn11 = assign7490_e5402_d_n11;
        locals.var_t4__blk144_dn12 = assign7490_e5402_d_n12;
        locals.var_t4__blk144_dn17 = assign7490_e5402_d_n17;
        locals.var_t4__blk144_rv = 0.0;

        let (assign7500_e5422, assign7500_e5422_d_n0, assign7500_e5422_d_n2, assign7500_e5422_d_n6, assign7500_e5422_d_n7, assign7500_e5422_d_n10, assign7500_e5422_d_n11, assign7500_e5422_d_n12, assign7500_e5422_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7500_e5409: f64 = (-locals.var_vbsbiz);
        let assign7500_e5413: f64 = (0.5 * locals.var_t1__blk141);
        let assign7500_e5414: f64 = (locals.var_t2__blk142 + assign7500_e5413);
        let assign7500_e5416: f64 = (-locals.var_q_fd_soi);
        let assign7500_e5417: f64 = (assign7500_e5414 * assign7500_e5416);
        let assign7500_e5418: f64 = (assign7500_e5409 + assign7500_e5417);
        let assign7500_e5419: f64 = (locals.var_t3__blk143 * assign7500_e5418);
        let assign7500_e5420: f64 = (locals.var_t0__blk140 * assign7500_e5419);
        (assign7500_e5420, ((locals.var_t0__blk140_dn0 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn0 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn0) + (((0.5 * locals.var_t1__blk141_dn0) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn0)))))))), ((locals.var_t0__blk140_dn2 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn2 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn2) + (((0.5 * locals.var_t1__blk141_dn2) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn2)))))))), ((locals.var_t0__blk140_dn6 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn6 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn6) + (((0.5 * locals.var_t1__blk141_dn6) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn6)))))))), ((locals.var_t0__blk140_dn7 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn7 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn7) + (((0.5 * locals.var_t1__blk141_dn7) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn7)))))))), ((locals.var_t0__blk140_dn10 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn10 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn10) + (((0.5 * locals.var_t1__blk141_dn10) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn10)))))))), ((locals.var_t0__blk140_dn11 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn11 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn11) + (((0.5 * locals.var_t1__blk141_dn11) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn11)))))))), ((locals.var_t0__blk140_dn12 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn12 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn12) + (((0.5 * locals.var_t1__blk141_dn12) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn12)))))))), ((locals.var_t0__blk140_dn17 * assign7500_e5419) + (locals.var_t0__blk140 * ((locals.var_t3__blk143_dn17 * assign7500_e5418) + (locals.var_t3__blk143 * ((-locals.var_vbsbiz_dn17) + (((0.5 * locals.var_t1__blk141_dn17) * assign7500_e5416) + (assign7500_e5414 * (-locals.var_q_fd_soi_dn17)))))))),)
    } else {
        (locals.var_t5__blk145, locals.var_t5__blk145_dn0, locals.var_t5__blk145_dn2, locals.var_t5__blk145_dn6, locals.var_t5__blk145_dn7, locals.var_t5__blk145_dn10, locals.var_t5__blk145_dn11, locals.var_t5__blk145_dn12, locals.var_t5__blk145_dn17,)
    }
};
        locals.var_t5__blk145 = assign7500_e5422;
        locals.var_t5__blk145_dn0 = assign7500_e5422_d_n0;
        locals.var_t5__blk145_dn2 = assign7500_e5422_d_n2;
        locals.var_t5__blk145_dn6 = assign7500_e5422_d_n6;
        locals.var_t5__blk145_dn7 = assign7500_e5422_d_n7;
        locals.var_t5__blk145_dn10 = assign7500_e5422_d_n10;
        locals.var_t5__blk145_dn11 = assign7500_e5422_d_n11;
        locals.var_t5__blk145_dn12 = assign7500_e5422_d_n12;
        locals.var_t5__blk145_dn17 = assign7500_e5422_d_n17;
        locals.var_t5__blk145_rv = 0.0;

        let (assign7510_e5430, assign7510_e5430_d_n0, assign7510_e5430_d_n2, assign7510_e5430_d_n6, assign7510_e5430_d_n7, assign7510_e5430_d_n10, assign7510_e5430_d_n11, assign7510_e5430_d_n12, assign7510_e5430_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7510_e5428: f64 = (locals.var_t5__blk145 / locals.var_t4__blk144);
        (assign7510_e5428, (((locals.var_t5__blk145_dn0 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn0)) / (locals.var_t4__blk144 * locals.var_t4__blk144)), (((locals.var_t5__blk145_dn2 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn2)) / (locals.var_t4__blk144 * locals.var_t4__blk144)), (((locals.var_t5__blk145_dn6 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn6)) / (locals.var_t4__blk144 * locals.var_t4__blk144)), (((locals.var_t5__blk145_dn7 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn7)) / (locals.var_t4__blk144 * locals.var_t4__blk144)), (((locals.var_t5__blk145_dn10 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn10)) / (locals.var_t4__blk144 * locals.var_t4__blk144)), (((locals.var_t5__blk145_dn11 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn11)) / (locals.var_t4__blk144 * locals.var_t4__blk144)), (((locals.var_t5__blk145_dn12 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn12)) / (locals.var_t4__blk144 * locals.var_t4__blk144)), (((locals.var_t5__blk145_dn17 * locals.var_t4__blk144) - (locals.var_t5__blk145 * locals.var_t4__blk144_dn17)) / (locals.var_t4__blk144 * locals.var_t4__blk144)),)
    } else {
        (locals.var_shift, locals.var_shift_dn0, locals.var_shift_dn2, locals.var_shift_dn6, locals.var_shift_dn7, locals.var_shift_dn10, locals.var_shift_dn11, locals.var_shift_dn12, locals.var_shift_dn17,)
    }
};
        locals.var_shift = assign7510_e5430;
        locals.var_shift_dn0 = assign7510_e5430_d_n0;
        locals.var_shift_dn2 = assign7510_e5430_d_n2;
        locals.var_shift_dn6 = assign7510_e5430_d_n6;
        locals.var_shift_dn7 = assign7510_e5430_d_n7;
        locals.var_shift_dn10 = assign7510_e5430_d_n10;
        locals.var_shift_dn11 = assign7510_e5430_d_n11;
        locals.var_shift_dn12 = assign7510_e5430_d_n12;
        locals.var_shift_dn17 = assign7510_e5430_d_n17;
        locals.var_shift_rv = 0.0;

        let (assign7520_e5438, assign7520_e5438_d_n0, assign7520_e5438_d_n2, assign7520_e5438_d_n6, assign7520_e5438_d_n7, assign7520_e5438_d_n10, assign7520_e5438_d_n11, assign7520_e5438_d_n12, assign7520_e5438_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7520_e5436: f64 = (locals.var_vgs_fb + locals.var_shift);
        (assign7520_e5436, (locals.var_vgs_fb_dn0 + locals.var_shift_dn0), (locals.var_vgs_fb_dn2 + locals.var_shift_dn2), (locals.var_vgs_fb_dn6 + locals.var_shift_dn6), (locals.var_vgs_fb_dn7 + locals.var_shift_dn7), (locals.var_vgs_fb_dn10 + locals.var_shift_dn10), (locals.var_vgs_fb_dn11 + locals.var_shift_dn11), (locals.var_vgs_fb_dn12 + locals.var_shift_dn12), (locals.var_vgs_fb_dn17 + locals.var_shift_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign7520_e5438;
        locals.var_vgs_fb_dn0 = assign7520_e5438_d_n0;
        locals.var_vgs_fb_dn2 = assign7520_e5438_d_n2;
        locals.var_vgs_fb_dn6 = assign7520_e5438_d_n6;
        locals.var_vgs_fb_dn7 = assign7520_e5438_d_n7;
        locals.var_vgs_fb_dn10 = assign7520_e5438_d_n10;
        locals.var_vgs_fb_dn11 = assign7520_e5438_d_n11;
        locals.var_vgs_fb_dn12 = assign7520_e5438_d_n12;
        locals.var_vgs_fb_dn17 = assign7520_e5438_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let (assign7530_e5446, assign7530_e5446_d_n0, assign7530_e5446_d_n2, assign7530_e5446_d_n6, assign7530_e5446_d_n7, assign7530_e5446_d_n10, assign7530_e5446_d_n11, assign7530_e5446_d_n12, assign7530_e5446_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7530_e5442: f64 = (locals.var_vbsc_dvbse * locals.var_vds);
        let assign7530_e5444: f64 = (assign7530_e5442 / 2.0);
        (assign7530_e5444, (((locals.var_vbsc_dvbse_dn0 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn0)) / 2.0), (((locals.var_vbsc_dvbse_dn2 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn2)) / 2.0), (((locals.var_vbsc_dvbse_dn6 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn6)) / 2.0), (((locals.var_vbsc_dvbse_dn7 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn7)) / 2.0), (((locals.var_vbsc_dvbse_dn10 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn10)) / 2.0), (((locals.var_vbsc_dvbse_dn11 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn11)) / 2.0), (((locals.var_vbsc_dvbse_dn12 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn12)) / 2.0), (((locals.var_vbsc_dvbse_dn17 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn17)) / 2.0),)
    } else {
        (locals.var_t1__blk147, locals.var_t1__blk147_dn0, locals.var_t1__blk147_dn2, locals.var_t1__blk147_dn6, locals.var_t1__blk147_dn7, locals.var_t1__blk147_dn10, locals.var_t1__blk147_dn11, locals.var_t1__blk147_dn12, locals.var_t1__blk147_dn17,)
    }
};
        locals.var_t1__blk147 = assign7530_e5446;
        locals.var_t1__blk147_dn0 = assign7530_e5446_d_n0;
        locals.var_t1__blk147_dn2 = assign7530_e5446_d_n2;
        locals.var_t1__blk147_dn6 = assign7530_e5446_d_n6;
        locals.var_t1__blk147_dn7 = assign7530_e5446_d_n7;
        locals.var_t1__blk147_dn10 = assign7530_e5446_d_n10;
        locals.var_t1__blk147_dn11 = assign7530_e5446_d_n11;
        locals.var_t1__blk147_dn12 = assign7530_e5446_d_n12;
        locals.var_t1__blk147_dn17 = assign7530_e5446_d_n17;
        locals.var_t1__blk147_rv = 0.0;

        let (assign7540_e5454, assign7540_e5454_d_n0, assign7540_e5454_d_n2, assign7540_e5454_d_n6, assign7540_e5454_d_n7, assign7540_e5454_d_n10, assign7540_e5454_d_n11, assign7540_e5454_d_n12, assign7540_e5454_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7540_e5450: f64 = (2.0 * locals.var_t1__blk147);
        let assign7540_e5452: f64 = (assign7540_e5450 / 0.1);
        (assign7540_e5452, ((2.0 * locals.var_t1__blk147_dn0) / 0.1), ((2.0 * locals.var_t1__blk147_dn2) / 0.1), ((2.0 * locals.var_t1__blk147_dn6) / 0.1), ((2.0 * locals.var_t1__blk147_dn7) / 0.1), ((2.0 * locals.var_t1__blk147_dn10) / 0.1), ((2.0 * locals.var_t1__blk147_dn11) / 0.1), ((2.0 * locals.var_t1__blk147_dn12) / 0.1), ((2.0 * locals.var_t1__blk147_dn17) / 0.1),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7540_e5454;
        locals.var_tmf1_dn0 = assign7540_e5454_d_n0;
        locals.var_tmf1_dn2 = assign7540_e5454_d_n2;
        locals.var_tmf1_dn6 = assign7540_e5454_d_n6;
        locals.var_tmf1_dn7 = assign7540_e5454_d_n7;
        locals.var_tmf1_dn10 = assign7540_e5454_d_n10;
        locals.var_tmf1_dn11 = assign7540_e5454_d_n11;
        locals.var_tmf1_dn12 = assign7540_e5454_d_n12;
        locals.var_tmf1_dn17 = assign7540_e5454_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7550_e5494, assign7550_e5494_d_n0, assign7550_e5494_d_n2, assign7550_e5494_d_n6, assign7550_e5494_d_n7, assign7550_e5494_d_n10, assign7550_e5494_d_n11, assign7550_e5494_d_n12, assign7550_e5494_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7550_e5460: f64 = (1.0 / 2.0);
        let assign7550_e5464: f64 = (1.0 / 6.0);
        let assign7550_e5468: f64 = (1.0 / 24.0);
        let assign7550_e5472: f64 = (1.0 / 120.0);
        let assign7550_e5476: f64 = (1.0 / 720.0);
        let assign7550_e5480: f64 = (1.0 / 5040.0);
        let assign7550_e5481: f64 = (locals.var_tmf1 * assign7550_e5480);
        let assign7550_e5482: f64 = (assign7550_e5476 + assign7550_e5481);
        let assign7550_e5483: f64 = (locals.var_tmf1 * assign7550_e5482);
        let assign7550_e5484: f64 = (assign7550_e5472 + assign7550_e5483);
        let assign7550_e5485: f64 = (locals.var_tmf1 * assign7550_e5484);
        let assign7550_e5486: f64 = (assign7550_e5468 + assign7550_e5485);
        let assign7550_e5487: f64 = (locals.var_tmf1 * assign7550_e5486);
        let assign7550_e5488: f64 = (assign7550_e5464 + assign7550_e5487);
        let assign7550_e5489: f64 = (locals.var_tmf1 * assign7550_e5488);
        let assign7550_e5490: f64 = (assign7550_e5460 + assign7550_e5489);
        let assign7550_e5491: f64 = (locals.var_tmf1 * assign7550_e5490);
        let assign7550_e5492: f64 = (1.0 + assign7550_e5491);
        (assign7550_e5492, ((locals.var_tmf1_dn0 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign7550_e5480))))))))))), ((locals.var_tmf1_dn2 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign7550_e5480))))))))))), ((locals.var_tmf1_dn6 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign7550_e5480))))))))))), ((locals.var_tmf1_dn7 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign7550_e5480))))))))))), ((locals.var_tmf1_dn10 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign7550_e5480))))))))))), ((locals.var_tmf1_dn11 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign7550_e5480))))))))))), ((locals.var_tmf1_dn12 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign7550_e5480))))))))))), ((locals.var_tmf1_dn17 * assign7550_e5490) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7550_e5488) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7550_e5486) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7550_e5484) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7550_e5482) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign7550_e5480))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7550_e5494;
        locals.var_tmf2_dn0 = assign7550_e5494_d_n0;
        locals.var_tmf2_dn2 = assign7550_e5494_d_n2;
        locals.var_tmf2_dn6 = assign7550_e5494_d_n6;
        locals.var_tmf2_dn7 = assign7550_e5494_d_n7;
        locals.var_tmf2_dn10 = assign7550_e5494_d_n10;
        locals.var_tmf2_dn11 = assign7550_e5494_d_n11;
        locals.var_tmf2_dn12 = assign7550_e5494_d_n12;
        locals.var_tmf2_dn17 = assign7550_e5494_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7560_e5500, assign7560_e5500_d_n0, assign7560_e5500_d_n2, assign7560_e5500_d_n6, assign7560_e5500_d_n7, assign7560_e5500_d_n10, assign7560_e5500_d_n11, assign7560_e5500_d_n12, assign7560_e5500_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7560_e5498: f64 = (0.1 / locals.var_tmf2);
        (assign7560_e5498, (-((0.1 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd__blk148, locals.var_vzadd__blk148_dn0, locals.var_vzadd__blk148_dn2, locals.var_vzadd__blk148_dn6, locals.var_vzadd__blk148_dn7, locals.var_vzadd__blk148_dn10, locals.var_vzadd__blk148_dn11, locals.var_vzadd__blk148_dn12, locals.var_vzadd__blk148_dn17,)
    }
};
        locals.var_vzadd__blk148 = assign7560_e5500;
        locals.var_vzadd__blk148_dn0 = assign7560_e5500_d_n0;
        locals.var_vzadd__blk148_dn2 = assign7560_e5500_d_n2;
        locals.var_vzadd__blk148_dn6 = assign7560_e5500_d_n6;
        locals.var_vzadd__blk148_dn7 = assign7560_e5500_d_n7;
        locals.var_vzadd__blk148_dn10 = assign7560_e5500_d_n10;
        locals.var_vzadd__blk148_dn11 = assign7560_e5500_d_n11;
        locals.var_vzadd__blk148_dn12 = assign7560_e5500_d_n12;
        locals.var_vzadd__blk148_dn17 = assign7560_e5500_d_n17;
        locals.var_vzadd__blk148_rv = 0.0;

        let assign7570_e5503: f64 = if locals.var_vzadd__blk148 < 5e-12 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7570_e5503;
        locals.var_guard149_rv = 0.0;

        let (assign7580_e5509, assign7580_e5509_d_n0, assign7580_e5509_d_n2, assign7580_e5509_d_n6, assign7580_e5509_d_n7, assign7580_e5509_d_n10, assign7580_e5509_d_n11, assign7580_e5509_d_n12, assign7580_e5509_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard149 != 0.0)) {
        (5e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd__blk148, locals.var_vzadd__blk148_dn0, locals.var_vzadd__blk148_dn2, locals.var_vzadd__blk148_dn6, locals.var_vzadd__blk148_dn7, locals.var_vzadd__blk148_dn10, locals.var_vzadd__blk148_dn11, locals.var_vzadd__blk148_dn12, locals.var_vzadd__blk148_dn17,)
    }
};
        locals.var_vzadd__blk148 = assign7580_e5509;
        locals.var_vzadd__blk148_dn0 = assign7580_e5509_d_n0;
        locals.var_vzadd__blk148_dn2 = assign7580_e5509_d_n2;
        locals.var_vzadd__blk148_dn6 = assign7580_e5509_d_n6;
        locals.var_vzadd__blk148_dn7 = assign7580_e5509_d_n7;
        locals.var_vzadd__blk148_dn10 = assign7580_e5509_d_n10;
        locals.var_vzadd__blk148_dn11 = assign7580_e5509_d_n11;
        locals.var_vzadd__blk148_dn12 = assign7580_e5509_d_n12;
        locals.var_vzadd__blk148_dn17 = assign7580_e5509_d_n17;
        locals.var_vzadd__blk148_rv = 0.0;

        let (assign7590_e5513, assign7590_e5513_d_n0, assign7590_e5513_d_n2, assign7590_e5513_d_n6, assign7590_e5513_d_n7, assign7590_e5513_d_n10, assign7590_e5513_d_n11, assign7590_e5513_d_n12, assign7590_e5513_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        (locals.var_vzadd__blk148, locals.var_vzadd__blk148_dn0, locals.var_vzadd__blk148_dn2, locals.var_vzadd__blk148_dn6, locals.var_vzadd__blk148_dn7, locals.var_vzadd__blk148_dn10, locals.var_vzadd__blk148_dn11, locals.var_vzadd__blk148_dn12, locals.var_vzadd__blk148_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign7590_e5513;
        locals.var_t3_dn0 = assign7590_e5513_d_n0;
        locals.var_t3_dn2 = assign7590_e5513_d_n2;
        locals.var_t3_dn6 = assign7590_e5513_d_n6;
        locals.var_t3_dn7 = assign7590_e5513_d_n7;
        locals.var_t3_dn10 = assign7590_e5513_d_n10;
        locals.var_t3_dn11 = assign7590_e5513_d_n11;
        locals.var_t3_dn12 = assign7590_e5513_d_n12;
        locals.var_t3_dn17 = assign7590_e5513_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign7600_e5525, assign7600_e5525_d_n0, assign7600_e5525_d_n2, assign7600_e5525_d_n6, assign7600_e5525_d_n7, assign7600_e5525_d_n10, assign7600_e5525_d_n11, assign7600_e5525_d_n12, assign7600_e5525_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7600_e5517: f64 = (locals.var_vgs + locals.var_t3);
        let assign7600_e5519: f64 = (assign7600_e5517 - locals.var_vfb);
        let assign7600_e5521: f64 = (assign7600_e5519 + locals.var_dvth);
        let assign7600_e5523: f64 = (assign7600_e5521 - locals.var_dppg);
        (assign7600_e5523, ((locals.var_t3_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_t3_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), (((locals.var_vgs_dn6 + locals.var_t3_dn6) + locals.var_dvth_dn6) - locals.var_dppg_dn6), (((locals.var_vgs_dn7 + locals.var_t3_dn7) + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_t3_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), (((locals.var_vgs_dn11 + locals.var_t3_dn11) + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_t3_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_t3_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpd, locals.var_vgpd_dn0, locals.var_vgpd_dn2, locals.var_vgpd_dn6, locals.var_vgpd_dn7, locals.var_vgpd_dn10, locals.var_vgpd_dn11, locals.var_vgpd_dn12, locals.var_vgpd_dn17,)
    }
};
        locals.var_vgpd = assign7600_e5525;
        locals.var_vgpd_dn0 = assign7600_e5525_d_n0;
        locals.var_vgpd_dn2 = assign7600_e5525_d_n2;
        locals.var_vgpd_dn6 = assign7600_e5525_d_n6;
        locals.var_vgpd_dn7 = assign7600_e5525_d_n7;
        locals.var_vgpd_dn10 = assign7600_e5525_d_n10;
        locals.var_vgpd_dn11 = assign7600_e5525_d_n11;
        locals.var_vgpd_dn12 = assign7600_e5525_d_n12;
        locals.var_vgpd_dn17 = assign7600_e5525_d_n17;
        locals.var_vgpd_rv = 0.0;

        let (assign7610_e5535, assign7610_e5535_d_n0, assign7610_e5535_d_n2, assign7610_e5535_d_n6, assign7610_e5535_d_n7, assign7610_e5535_d_n10, assign7610_e5535_d_n11, assign7610_e5535_d_n12, assign7610_e5535_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign7610_e5530: f64 = (locals.var_wdsoi_ini1_dlt * locals.var_pb2);
        let assign7610_e5531: f64 = (locals.var_wdsoi_ini0 / assign7610_e5530);
        let assign7610_e5533: f64 = (assign7610_e5531 * locals.var_vgpd);
        (assign7610_e5533, (((((locals.var_wdsoi_ini0_dn0 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn0))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn0)), (((((locals.var_wdsoi_ini0_dn2 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn2))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn2)), (((((locals.var_wdsoi_ini0_dn6 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn6))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn6)), (((((locals.var_wdsoi_ini0_dn7 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn7))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn7)), (((((locals.var_wdsoi_ini0_dn10 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn10))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn10)), (((((locals.var_wdsoi_ini0_dn11 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn11))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn11)), (((((locals.var_wdsoi_ini0_dn12 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn12))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn12)), (((((locals.var_wdsoi_ini0_dn17 * assign7610_e5530) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn17))) / (assign7610_e5530 * assign7610_e5530)) * locals.var_vgpd) + (assign7610_e5531 * locals.var_vgpd_dn17)),)
    } else {
        (locals.var_wdsoi_ini1, locals.var_wdsoi_ini1_dn0, locals.var_wdsoi_ini1_dn2, locals.var_wdsoi_ini1_dn6, locals.var_wdsoi_ini1_dn7, locals.var_wdsoi_ini1_dn10, locals.var_wdsoi_ini1_dn11, locals.var_wdsoi_ini1_dn12, locals.var_wdsoi_ini1_dn17,)
    }
};
        locals.var_wdsoi_ini1 = assign7610_e5535;
        locals.var_wdsoi_ini1_dn0 = assign7610_e5535_d_n0;
        locals.var_wdsoi_ini1_dn2 = assign7610_e5535_d_n2;
        locals.var_wdsoi_ini1_dn6 = assign7610_e5535_d_n6;
        locals.var_wdsoi_ini1_dn7 = assign7610_e5535_d_n7;
        locals.var_wdsoi_ini1_dn10 = assign7610_e5535_d_n10;
        locals.var_wdsoi_ini1_dn11 = assign7610_e5535_d_n11;
        locals.var_wdsoi_ini1_dn12 = assign7610_e5535_d_n12;
        locals.var_wdsoi_ini1_dn17 = assign7610_e5535_d_n17;
        locals.var_wdsoi_ini1_rv = 0.0;

        let assign7620_e5540: f64 = (locals.var_t_soi * 7.0);
        let assign7620_e5541: f64 = assign7620_e5540;
        let assign7620_e5545: f64 = (locals.var_t_soi * 7.0);
        let assign7620_e5548: f64 = if ((locals.var_wdsoi_ini1 < assign7620_e5541) && (assign7620_e5545 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7620_e5548;
        locals.var_guard150_rv = 0.0;

        let (assign7630_e5560, assign7630_e5560_d_n0, assign7630_e5560_d_n2, assign7630_e5560_d_n6, assign7630_e5560_d_n7, assign7630_e5560_d_n10, assign7630_e5560_d_n11, assign7630_e5560_d_n12, assign7630_e5560_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7630_e5555: f64 = (locals.var_t_soi * 7.0);
        let assign7630_e5556: f64 = assign7630_e5555;
        let assign7630_e5558: f64 = (assign7630_e5556 - locals.var_wdsoi_ini1);
        (assign7630_e5558, (-locals.var_wdsoi_ini1_dn0), (-locals.var_wdsoi_ini1_dn2), (-locals.var_wdsoi_ini1_dn6), (-locals.var_wdsoi_ini1_dn7), (-locals.var_wdsoi_ini1_dn10), (-locals.var_wdsoi_ini1_dn11), (-locals.var_wdsoi_ini1_dn12), (-locals.var_wdsoi_ini1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7630_e5560;
        locals.var_tmf1_dn0 = assign7630_e5560_d_n0;
        locals.var_tmf1_dn2 = assign7630_e5560_d_n2;
        locals.var_tmf1_dn6 = assign7630_e5560_d_n6;
        locals.var_tmf1_dn7 = assign7630_e5560_d_n7;
        locals.var_tmf1_dn10 = assign7630_e5560_d_n10;
        locals.var_tmf1_dn11 = assign7630_e5560_d_n11;
        locals.var_tmf1_dn12 = assign7630_e5560_d_n12;
        locals.var_tmf1_dn17 = assign7630_e5560_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7640_e5568, assign7640_e5568_d_n0, assign7640_e5568_d_n2, assign7640_e5568_d_n6, assign7640_e5568_d_n7, assign7640_e5568_d_n10, assign7640_e5568_d_n11, assign7640_e5568_d_n12, assign7640_e5568_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7640_e5566: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign7640_e5566, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign7640_e5568;
        locals.var_x2_dn0 = assign7640_e5568_d_n0;
        locals.var_x2_dn2 = assign7640_e5568_d_n2;
        locals.var_x2_dn6 = assign7640_e5568_d_n6;
        locals.var_x2_dn7 = assign7640_e5568_d_n7;
        locals.var_x2_dn10 = assign7640_e5568_d_n10;
        locals.var_x2_dn11 = assign7640_e5568_d_n11;
        locals.var_x2_dn12 = assign7640_e5568_d_n12;
        locals.var_x2_dn17 = assign7640_e5568_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign7650_e5580, assign7650_e5580_d_n0, assign7650_e5580_d_n2, assign7650_e5580_d_n6, assign7650_e5580_d_n7, assign7650_e5580_d_n10, assign7650_e5580_d_n11, assign7650_e5580_d_n12, assign7650_e5580_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7650_e5574: f64 = (locals.var_t_soi * 7.0);
        let assign7650_e5577: f64 = (locals.var_t_soi * 7.0);
        let assign7650_e5578: f64 = (assign7650_e5574 * assign7650_e5577);
        (assign7650_e5578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign7650_e5580;
        locals.var_xmax2_dn0 = assign7650_e5580_d_n0;
        locals.var_xmax2_dn2 = assign7650_e5580_d_n2;
        locals.var_xmax2_dn6 = assign7650_e5580_d_n6;
        locals.var_xmax2_dn7 = assign7650_e5580_d_n7;
        locals.var_xmax2_dn10 = assign7650_e5580_d_n10;
        locals.var_xmax2_dn11 = assign7650_e5580_d_n11;
        locals.var_xmax2_dn12 = assign7650_e5580_d_n12;
        locals.var_xmax2_dn17 = assign7650_e5580_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign7660_e5586, assign7660_e5586_d_n0, assign7660_e5586_d_n2, assign7660_e5586_d_n6, assign7660_e5586_d_n7, assign7660_e5586_d_n10, assign7660_e5586_d_n11, assign7660_e5586_d_n12, assign7660_e5586_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7660_e5586;
        locals.var_xp_dn0 = assign7660_e5586_d_n0;
        locals.var_xp_dn2 = assign7660_e5586_d_n2;
        locals.var_xp_dn6 = assign7660_e5586_d_n6;
        locals.var_xp_dn7 = assign7660_e5586_d_n7;
        locals.var_xp_dn10 = assign7660_e5586_d_n10;
        locals.var_xp_dn11 = assign7660_e5586_d_n11;
        locals.var_xp_dn12 = assign7660_e5586_d_n12;
        locals.var_xp_dn17 = assign7660_e5586_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7670_e5592, assign7670_e5592_d_n0, assign7670_e5592_d_n2, assign7670_e5592_d_n6, assign7670_e5592_d_n7, assign7670_e5592_d_n10, assign7670_e5592_d_n11, assign7670_e5592_d_n12, assign7670_e5592_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7670_e5592;
        locals.var_xmp_dn0 = assign7670_e5592_d_n0;
        locals.var_xmp_dn2 = assign7670_e5592_d_n2;
        locals.var_xmp_dn6 = assign7670_e5592_d_n6;
        locals.var_xmp_dn7 = assign7670_e5592_d_n7;
        locals.var_xmp_dn10 = assign7670_e5592_d_n10;
        locals.var_xmp_dn11 = assign7670_e5592_d_n11;
        locals.var_xmp_dn12 = assign7670_e5592_d_n12;
        locals.var_xmp_dn17 = assign7670_e5592_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign7680_e5598,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign7680_e5598;
        locals.var_m0_rv = 0.0;

        let (assign7690_e5604,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7690_e5604;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign7700_e5610, assign7700_e5610_d_n0, assign7700_e5610_d_n2, assign7700_e5610_d_n6, assign7700_e5610_d_n7, assign7700_e5610_d_n10, assign7700_e5610_d_n11, assign7700_e5610_d_n12, assign7700_e5610_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign7700_e5610;
        locals.var_arg_dn0 = assign7700_e5610_d_n0;
        locals.var_arg_dn2 = assign7700_e5610_d_n2;
        locals.var_arg_dn6 = assign7700_e5610_d_n6;
        locals.var_arg_dn7 = assign7700_e5610_d_n7;
        locals.var_arg_dn10 = assign7700_e5610_d_n10;
        locals.var_arg_dn11 = assign7700_e5610_d_n11;
        locals.var_arg_dn12 = assign7700_e5610_d_n12;
        locals.var_arg_dn17 = assign7700_e5610_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign7710_e5616, assign7710_e5616_d_n0, assign7710_e5616_d_n2, assign7710_e5616_d_n6, assign7710_e5616_d_n7, assign7710_e5616_d_n10, assign7710_e5616_d_n11, assign7710_e5616_d_n12, assign7710_e5616_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7710_e5616;
        locals.var_dnm_dn0 = assign7710_e5616_d_n0;
        locals.var_dnm_dn2 = assign7710_e5616_d_n2;
        locals.var_dnm_dn6 = assign7710_e5616_d_n6;
        locals.var_dnm_dn7 = assign7710_e5616_d_n7;
        locals.var_dnm_dn10 = assign7710_e5616_d_n10;
        locals.var_dnm_dn11 = assign7710_e5616_d_n11;
        locals.var_dnm_dn12 = assign7710_e5616_d_n12;
        locals.var_dnm_dn17 = assign7710_e5616_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign7720_e5624, assign7720_e5624_d_n0, assign7720_e5624_d_n2, assign7720_e5624_d_n6, assign7720_e5624_d_n7, assign7720_e5624_d_n10, assign7720_e5624_d_n11, assign7720_e5624_d_n12, assign7720_e5624_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7720_e5622: f64 = (locals.var_xp * locals.var_x2);
        (assign7720_e5622, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7720_e5624;
        locals.var_xp_dn0 = assign7720_e5624_d_n0;
        locals.var_xp_dn2 = assign7720_e5624_d_n2;
        locals.var_xp_dn6 = assign7720_e5624_d_n6;
        locals.var_xp_dn7 = assign7720_e5624_d_n7;
        locals.var_xp_dn10 = assign7720_e5624_d_n10;
        locals.var_xp_dn11 = assign7720_e5624_d_n11;
        locals.var_xp_dn12 = assign7720_e5624_d_n12;
        locals.var_xp_dn17 = assign7720_e5624_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7730_e5632, assign7730_e5632_d_n0, assign7730_e5632_d_n2, assign7730_e5632_d_n6, assign7730_e5632_d_n7, assign7730_e5632_d_n10, assign7730_e5632_d_n11, assign7730_e5632_d_n12, assign7730_e5632_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7730_e5630: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign7730_e5630, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7730_e5632;
        locals.var_xmp_dn0 = assign7730_e5632_d_n0;
        locals.var_xmp_dn2 = assign7730_e5632_d_n2;
        locals.var_xmp_dn6 = assign7730_e5632_d_n6;
        locals.var_xmp_dn7 = assign7730_e5632_d_n7;
        locals.var_xmp_dn10 = assign7730_e5632_d_n10;
        locals.var_xmp_dn11 = assign7730_e5632_d_n11;
        locals.var_xmp_dn12 = assign7730_e5632_d_n12;
        locals.var_xmp_dn17 = assign7730_e5632_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign7740_e5640, assign7740_e5640_d_n0, assign7740_e5640_d_n2, assign7740_e5640_d_n6, assign7740_e5640_d_n7, assign7740_e5640_d_n10, assign7740_e5640_d_n11, assign7740_e5640_d_n12, assign7740_e5640_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7740_e5638: f64 = (locals.var_xp * locals.var_x2);
        (assign7740_e5638, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7740_e5640;
        locals.var_xp_dn0 = assign7740_e5640_d_n0;
        locals.var_xp_dn2 = assign7740_e5640_d_n2;
        locals.var_xp_dn6 = assign7740_e5640_d_n6;
        locals.var_xp_dn7 = assign7740_e5640_d_n7;
        locals.var_xp_dn10 = assign7740_e5640_d_n10;
        locals.var_xp_dn11 = assign7740_e5640_d_n11;
        locals.var_xp_dn12 = assign7740_e5640_d_n12;
        locals.var_xp_dn17 = assign7740_e5640_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7750_e5648, assign7750_e5648_d_n0, assign7750_e5648_d_n2, assign7750_e5648_d_n6, assign7750_e5648_d_n7, assign7750_e5648_d_n10, assign7750_e5648_d_n11, assign7750_e5648_d_n12, assign7750_e5648_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7750_e5646: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign7750_e5646, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7750_e5648;
        locals.var_xmp_dn0 = assign7750_e5648_d_n0;
        locals.var_xmp_dn2 = assign7750_e5648_d_n2;
        locals.var_xmp_dn6 = assign7750_e5648_d_n6;
        locals.var_xmp_dn7 = assign7750_e5648_d_n7;
        locals.var_xmp_dn10 = assign7750_e5648_d_n10;
        locals.var_xmp_dn11 = assign7750_e5648_d_n11;
        locals.var_xmp_dn12 = assign7750_e5648_d_n12;
        locals.var_xmp_dn17 = assign7750_e5648_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign7760_e5656, assign7760_e5656_d_n0, assign7760_e5656_d_n2, assign7760_e5656_d_n6, assign7760_e5656_d_n7, assign7760_e5656_d_n10, assign7760_e5656_d_n11, assign7760_e5656_d_n12, assign7760_e5656_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7760_e5654: f64 = (locals.var_xp + locals.var_xmp);
        (assign7760_e5654, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign7760_e5656;
        locals.var_arg_dn0 = assign7760_e5656_d_n0;
        locals.var_arg_dn2 = assign7760_e5656_d_n2;
        locals.var_arg_dn6 = assign7760_e5656_d_n6;
        locals.var_arg_dn7 = assign7760_e5656_d_n7;
        locals.var_arg_dn10 = assign7760_e5656_d_n10;
        locals.var_arg_dn11 = assign7760_e5656_d_n11;
        locals.var_arg_dn12 = assign7760_e5656_d_n12;
        locals.var_arg_dn17 = assign7760_e5656_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign7770_e5662, assign7770_e5662_d_n0, assign7770_e5662_d_n2, assign7770_e5662_d_n6, assign7770_e5662_d_n7, assign7770_e5662_d_n10, assign7770_e5662_d_n11, assign7770_e5662_d_n12, assign7770_e5662_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7770_e5662;
        locals.var_dnm_dn0 = assign7770_e5662_d_n0;
        locals.var_dnm_dn2 = assign7770_e5662_d_n2;
        locals.var_dnm_dn6 = assign7770_e5662_d_n6;
        locals.var_dnm_dn7 = assign7770_e5662_d_n7;
        locals.var_dnm_dn10 = assign7770_e5662_d_n10;
        locals.var_dnm_dn11 = assign7770_e5662_d_n11;
        locals.var_dnm_dn12 = assign7770_e5662_d_n12;
        locals.var_dnm_dn17 = assign7770_e5662_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign7780_e5677: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7780_e5677;
        locals.var_guard151_rv = 0.0;

        let assign7790_e5680: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign7790_e5680;
        locals.var_guard152_rv = 0.0;

        let (assign7800_e5690,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7800_e5690;
        locals.var_mm_rv = 0.0;

        let assign7810_e5693: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign7810_e5693;
        locals.var_guard153_rv = 0.0;

        let (assign7820_e5706,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 == 0.0)) && (locals.var_guard153 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7820_e5706;
        locals.var_mm_rv = 0.0;

        let assign7830_e5709: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign7830_e5709;
        locals.var_guard154_rv = 0.0;

        let (assign7840_e5725,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 == 0.0)) && (locals.var_guard153 == 0.0)) && (locals.var_guard154 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7840_e5725;
        locals.var_mm_rv = 0.0;

        let assign7850_e5728: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign7850_e5728;
        locals.var_guard155_rv = 0.0;

        let (assign7860_e5747,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 == 0.0)) && (locals.var_guard153 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard155 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7860_e5747;
        locals.var_mm_rv = 0.0;

        let (assign7870_e5755,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign7870_e5755;
        locals.var_m0_rv = 0.0;

        let mut assign7880_loop_guard: usize = 0;
        while {
            let assign7880_cond_e5764: f64 = if ((((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign7880_cond_e5764 != 0.0
        } {
            assign7880_loop_guard += 1;
            assert!(assign7880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7880_body0_e5773, assign7880_body0_e5773_d_n0, assign7880_body0_e5773_d_n2, assign7880_body0_e5773_d_n6, assign7880_body0_e5773_d_n7, assign7880_body0_e5773_d_n10, assign7880_body0_e5773_d_n11, assign7880_body0_e5773_d_n12, assign7880_body0_e5773_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) {
        let assign7880_body0_e5771: f64 = (locals.var_dnm).sqrt();
        (assign7880_body0_e5771, (locals.var_dnm_dn0 / (2.0 * assign7880_body0_e5771)), (locals.var_dnm_dn2 / (2.0 * assign7880_body0_e5771)), (locals.var_dnm_dn6 / (2.0 * assign7880_body0_e5771)), (locals.var_dnm_dn7 / (2.0 * assign7880_body0_e5771)), (locals.var_dnm_dn10 / (2.0 * assign7880_body0_e5771)), (locals.var_dnm_dn11 / (2.0 * assign7880_body0_e5771)), (locals.var_dnm_dn12 / (2.0 * assign7880_body0_e5771)), (locals.var_dnm_dn17 / (2.0 * assign7880_body0_e5771)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign7880_body0_e5773;
            locals.var_dnm_dn0 = assign7880_body0_e5773_d_n0;
            locals.var_dnm_dn2 = assign7880_body0_e5773_d_n2;
            locals.var_dnm_dn6 = assign7880_body0_e5773_d_n6;
            locals.var_dnm_dn7 = assign7880_body0_e5773_d_n7;
            locals.var_dnm_dn10 = assign7880_body0_e5773_d_n10;
            locals.var_dnm_dn11 = assign7880_body0_e5773_d_n11;
            locals.var_dnm_dn12 = assign7880_body0_e5773_d_n12;
            locals.var_dnm_dn17 = assign7880_body0_e5773_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign7880_body1_e5783,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 != 0.0)) {
        let assign7880_body1_e5781: f64 = (locals.var_m0 + 1.0);
        (assign7880_body1_e5781,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign7880_body1_e5783;
            locals.var_m0_rv = 0.0;
        }

        let (assign7890_e5798, assign7890_e5798_d_n0, assign7890_e5798_d_n2, assign7890_e5798_d_n6, assign7890_e5798_d_n7, assign7890_e5798_d_n10, assign7890_e5798_d_n11, assign7890_e5798_d_n12, assign7890_e5798_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) && (locals.var_guard151 == 0.0)) {
        let assign7890_e5794: f64 = (2.0 * 2.0);
        let assign7890_e5795: f64 = (1.0 / assign7890_e5794);
        let assign7890_e5796: f64 = (locals.var_dnm).powf(assign7890_e5795);
        (assign7890_e5796, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn0)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn2)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn6)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn7)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn10)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn11)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn12)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7890_e5795) as f64).is_finite() && ((assign7890_e5795) as f64).fract() == 0.0 { if assign7890_e5795 == 0.0 { 0.0 } else { (assign7890_e5795 * ((locals.var_dnm).powf(assign7890_e5795 - 1.0) * locals.var_dnm_dn17)) } } else { (assign7890_e5796 * (assign7890_e5795 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7890_e5798;
        locals.var_dnm_dn0 = assign7890_e5798_d_n0;
        locals.var_dnm_dn2 = assign7890_e5798_d_n2;
        locals.var_dnm_dn6 = assign7890_e5798_d_n6;
        locals.var_dnm_dn7 = assign7890_e5798_d_n7;
        locals.var_dnm_dn10 = assign7890_e5798_d_n10;
        locals.var_dnm_dn11 = assign7890_e5798_d_n11;
        locals.var_dnm_dn12 = assign7890_e5798_d_n12;
        locals.var_dnm_dn17 = assign7890_e5798_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign7900_e5806, assign7900_e5806_d_n0, assign7900_e5806_d_n2, assign7900_e5806_d_n6, assign7900_e5806_d_n7, assign7900_e5806_d_n10, assign7900_e5806_d_n11, assign7900_e5806_d_n12, assign7900_e5806_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7900_e5804: f64 = (1.0 / locals.var_dnm);
        (assign7900_e5804, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7900_e5806;
        locals.var_dnm_dn0 = assign7900_e5806_d_n0;
        locals.var_dnm_dn2 = assign7900_e5806_d_n2;
        locals.var_dnm_dn6 = assign7900_e5806_d_n6;
        locals.var_dnm_dn7 = assign7900_e5806_d_n7;
        locals.var_dnm_dn10 = assign7900_e5806_d_n10;
        locals.var_dnm_dn11 = assign7900_e5806_d_n11;
        locals.var_dnm_dn12 = assign7900_e5806_d_n12;
        locals.var_dnm_dn17 = assign7900_e5806_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign7910_e5818, assign7910_e5818_d_n0, assign7910_e5818_d_n2, assign7910_e5818_d_n6, assign7910_e5818_d_n7, assign7910_e5818_d_n10, assign7910_e5818_d_n11, assign7910_e5818_d_n12, assign7910_e5818_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7910_e5813: f64 = (locals.var_t_soi * 7.0);
        let assign7910_e5814: f64 = (locals.var_tmf1 * assign7910_e5813);
        let assign7910_e5816: f64 = (assign7910_e5814 * locals.var_dnm);
        (assign7910_e5816, (((locals.var_tmf1_dn0 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * assign7910_e5813) * locals.var_dnm) + (assign7910_e5814 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign7910_e5818;
        locals.var_tmf0_dn0 = assign7910_e5818_d_n0;
        locals.var_tmf0_dn2 = assign7910_e5818_d_n2;
        locals.var_tmf0_dn6 = assign7910_e5818_d_n6;
        locals.var_tmf0_dn7 = assign7910_e5818_d_n7;
        locals.var_tmf0_dn10 = assign7910_e5818_d_n10;
        locals.var_tmf0_dn11 = assign7910_e5818_d_n11;
        locals.var_tmf0_dn12 = assign7910_e5818_d_n12;
        locals.var_tmf0_dn17 = assign7910_e5818_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign7920_e5830, assign7920_e5830_d_n0, assign7920_e5830_d_n2, assign7920_e5830_d_n6, assign7920_e5830_d_n7, assign7920_e5830_d_n10, assign7920_e5830_d_n11, assign7920_e5830_d_n12, assign7920_e5830_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign7920_e5825: f64 = (locals.var_t_soi * 7.0);
        let assign7920_e5826: f64 = assign7920_e5825;
        let assign7920_e5828: f64 = (assign7920_e5826 - locals.var_tmf0);
        (assign7920_e5828, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign7920_e5830;
        locals.var_wdsoi_ini2_dn0 = assign7920_e5830_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign7920_e5830_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign7920_e5830_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign7920_e5830_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign7920_e5830_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign7920_e5830_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign7920_e5830_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign7920_e5830_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let (assign7930_e5837, assign7930_e5837_d_n0, assign7930_e5837_d_n2, assign7930_e5837_d_n6, assign7930_e5837_d_n7, assign7930_e5837_d_n10, assign7930_e5837_d_n11, assign7930_e5837_d_n12, assign7930_e5837_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard150 == 0.0)) {
        (locals.var_wdsoi_ini1, locals.var_wdsoi_ini1_dn0, locals.var_wdsoi_ini1_dn2, locals.var_wdsoi_ini1_dn6, locals.var_wdsoi_ini1_dn7, locals.var_wdsoi_ini1_dn10, locals.var_wdsoi_ini1_dn11, locals.var_wdsoi_ini1_dn12, locals.var_wdsoi_ini1_dn17,)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign7930_e5837;
        locals.var_wdsoi_ini2_dn0 = assign7930_e5837_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign7930_e5837_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign7930_e5837_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign7930_e5837_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign7930_e5837_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign7930_e5837_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign7930_e5837_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign7930_e5837_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let assign7940_e5842: f64 = locals.var_t_soi;
        let assign7940_e5843: f64 = (locals.var_wdsoi_ini0 - assign7940_e5842);
        let assign7940_e5847: f64 = locals.var_t_soi;
        let assign7940_e5850: f64 = if ((locals.var_wdsoi_ini2 > assign7940_e5843) && (assign7940_e5847 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard156 = assign7940_e5850;
        locals.var_guard156_rv = 0.0;

        let (assign7950_e5862, assign7950_e5862_d_n0, assign7950_e5862_d_n2, assign7950_e5862_d_n6, assign7950_e5862_d_n7, assign7950_e5862_d_n10, assign7950_e5862_d_n11, assign7950_e5862_d_n12, assign7950_e5862_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign7950_e5856: f64 = (locals.var_wdsoi_ini2 - locals.var_wdsoi_ini0);
        let assign7950_e5859: f64 = locals.var_t_soi;
        let assign7950_e5860: f64 = (assign7950_e5856 + assign7950_e5859);
        (assign7950_e5860, (locals.var_wdsoi_ini2_dn0 - locals.var_wdsoi_ini0_dn0), (locals.var_wdsoi_ini2_dn2 - locals.var_wdsoi_ini0_dn2), (locals.var_wdsoi_ini2_dn6 - locals.var_wdsoi_ini0_dn6), (locals.var_wdsoi_ini2_dn7 - locals.var_wdsoi_ini0_dn7), (locals.var_wdsoi_ini2_dn10 - locals.var_wdsoi_ini0_dn10), (locals.var_wdsoi_ini2_dn11 - locals.var_wdsoi_ini0_dn11), (locals.var_wdsoi_ini2_dn12 - locals.var_wdsoi_ini0_dn12), (locals.var_wdsoi_ini2_dn17 - locals.var_wdsoi_ini0_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7950_e5862;
        locals.var_tmf1_dn0 = assign7950_e5862_d_n0;
        locals.var_tmf1_dn2 = assign7950_e5862_d_n2;
        locals.var_tmf1_dn6 = assign7950_e5862_d_n6;
        locals.var_tmf1_dn7 = assign7950_e5862_d_n7;
        locals.var_tmf1_dn10 = assign7950_e5862_d_n10;
        locals.var_tmf1_dn11 = assign7950_e5862_d_n11;
        locals.var_tmf1_dn12 = assign7950_e5862_d_n12;
        locals.var_tmf1_dn17 = assign7950_e5862_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7960_e5870, assign7960_e5870_d_n0, assign7960_e5870_d_n2, assign7960_e5870_d_n6, assign7960_e5870_d_n7, assign7960_e5870_d_n10, assign7960_e5870_d_n11, assign7960_e5870_d_n12, assign7960_e5870_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign7960_e5868: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign7960_e5868, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign7960_e5870;
        locals.var_x2_dn0 = assign7960_e5870_d_n0;
        locals.var_x2_dn2 = assign7960_e5870_d_n2;
        locals.var_x2_dn6 = assign7960_e5870_d_n6;
        locals.var_x2_dn7 = assign7960_e5870_d_n7;
        locals.var_x2_dn10 = assign7960_e5870_d_n10;
        locals.var_x2_dn11 = assign7960_e5870_d_n11;
        locals.var_x2_dn12 = assign7960_e5870_d_n12;
        locals.var_x2_dn17 = assign7960_e5870_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign7970_e5882, assign7970_e5882_d_n0, assign7970_e5882_d_n2, assign7970_e5882_d_n6, assign7970_e5882_d_n7, assign7970_e5882_d_n10, assign7970_e5882_d_n11, assign7970_e5882_d_n12, assign7970_e5882_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign7970_e5876: f64 = locals.var_t_soi;
        let assign7970_e5879: f64 = locals.var_t_soi;
        let assign7970_e5880: f64 = (assign7970_e5876 * assign7970_e5879);
        (assign7970_e5880, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign7970_e5882;
        locals.var_xmax2_dn0 = assign7970_e5882_d_n0;
        locals.var_xmax2_dn2 = assign7970_e5882_d_n2;
        locals.var_xmax2_dn6 = assign7970_e5882_d_n6;
        locals.var_xmax2_dn7 = assign7970_e5882_d_n7;
        locals.var_xmax2_dn10 = assign7970_e5882_d_n10;
        locals.var_xmax2_dn11 = assign7970_e5882_d_n11;
        locals.var_xmax2_dn12 = assign7970_e5882_d_n12;
        locals.var_xmax2_dn17 = assign7970_e5882_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign7980_e5888, assign7980_e5888_d_n0, assign7980_e5888_d_n2, assign7980_e5888_d_n6, assign7980_e5888_d_n7, assign7980_e5888_d_n10, assign7980_e5888_d_n11, assign7980_e5888_d_n12, assign7980_e5888_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7980_e5888;
        locals.var_xp_dn0 = assign7980_e5888_d_n0;
        locals.var_xp_dn2 = assign7980_e5888_d_n2;
        locals.var_xp_dn6 = assign7980_e5888_d_n6;
        locals.var_xp_dn7 = assign7980_e5888_d_n7;
        locals.var_xp_dn10 = assign7980_e5888_d_n10;
        locals.var_xp_dn11 = assign7980_e5888_d_n11;
        locals.var_xp_dn12 = assign7980_e5888_d_n12;
        locals.var_xp_dn17 = assign7980_e5888_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7990_e5894, assign7990_e5894_d_n0, assign7990_e5894_d_n2, assign7990_e5894_d_n6, assign7990_e5894_d_n7, assign7990_e5894_d_n10, assign7990_e5894_d_n11, assign7990_e5894_d_n12, assign7990_e5894_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7990_e5894;
        locals.var_xmp_dn0 = assign7990_e5894_d_n0;
        locals.var_xmp_dn2 = assign7990_e5894_d_n2;
        locals.var_xmp_dn6 = assign7990_e5894_d_n6;
        locals.var_xmp_dn7 = assign7990_e5894_d_n7;
        locals.var_xmp_dn10 = assign7990_e5894_d_n10;
        locals.var_xmp_dn11 = assign7990_e5894_d_n11;
        locals.var_xmp_dn12 = assign7990_e5894_d_n12;
        locals.var_xmp_dn17 = assign7990_e5894_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8000_e5900,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign8000_e5900;
        locals.var_m0_rv = 0.0;

        let (assign8010_e5906,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8010_e5906;
        locals.var_mm_rv = 0.0;

        let (assign8020_e5912, assign8020_e5912_d_n0, assign8020_e5912_d_n2, assign8020_e5912_d_n6, assign8020_e5912_d_n7, assign8020_e5912_d_n10, assign8020_e5912_d_n11, assign8020_e5912_d_n12, assign8020_e5912_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign8020_e5912;
        locals.var_arg_dn0 = assign8020_e5912_d_n0;
        locals.var_arg_dn2 = assign8020_e5912_d_n2;
        locals.var_arg_dn6 = assign8020_e5912_d_n6;
        locals.var_arg_dn7 = assign8020_e5912_d_n7;
        locals.var_arg_dn10 = assign8020_e5912_d_n10;
        locals.var_arg_dn11 = assign8020_e5912_d_n11;
        locals.var_arg_dn12 = assign8020_e5912_d_n12;
        locals.var_arg_dn17 = assign8020_e5912_d_n17;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign8030_e5918, assign8030_e5918_d_n0, assign8030_e5918_d_n2, assign8030_e5918_d_n6, assign8030_e5918_d_n7, assign8030_e5918_d_n10, assign8030_e5918_d_n11, assign8030_e5918_d_n12, assign8030_e5918_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8030_e5918;
        locals.var_dnm_dn0 = assign8030_e5918_d_n0;
        locals.var_dnm_dn2 = assign8030_e5918_d_n2;
        locals.var_dnm_dn6 = assign8030_e5918_d_n6;
        locals.var_dnm_dn7 = assign8030_e5918_d_n7;
        locals.var_dnm_dn10 = assign8030_e5918_d_n10;
        locals.var_dnm_dn11 = assign8030_e5918_d_n11;
        locals.var_dnm_dn12 = assign8030_e5918_d_n12;
        locals.var_dnm_dn17 = assign8030_e5918_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign8040_e5926, assign8040_e5926_d_n0, assign8040_e5926_d_n2, assign8040_e5926_d_n6, assign8040_e5926_d_n7, assign8040_e5926_d_n10, assign8040_e5926_d_n11, assign8040_e5926_d_n12, assign8040_e5926_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8040_e5924: f64 = (locals.var_xp * locals.var_x2);
        (assign8040_e5924, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8040_e5926;
        locals.var_xp_dn0 = assign8040_e5926_d_n0;
        locals.var_xp_dn2 = assign8040_e5926_d_n2;
        locals.var_xp_dn6 = assign8040_e5926_d_n6;
        locals.var_xp_dn7 = assign8040_e5926_d_n7;
        locals.var_xp_dn10 = assign8040_e5926_d_n10;
        locals.var_xp_dn11 = assign8040_e5926_d_n11;
        locals.var_xp_dn12 = assign8040_e5926_d_n12;
        locals.var_xp_dn17 = assign8040_e5926_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8050_e5934, assign8050_e5934_d_n0, assign8050_e5934_d_n2, assign8050_e5934_d_n6, assign8050_e5934_d_n7, assign8050_e5934_d_n10, assign8050_e5934_d_n11, assign8050_e5934_d_n12, assign8050_e5934_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8050_e5932: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign8050_e5932, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8050_e5934;
        locals.var_xmp_dn0 = assign8050_e5934_d_n0;
        locals.var_xmp_dn2 = assign8050_e5934_d_n2;
        locals.var_xmp_dn6 = assign8050_e5934_d_n6;
        locals.var_xmp_dn7 = assign8050_e5934_d_n7;
        locals.var_xmp_dn10 = assign8050_e5934_d_n10;
        locals.var_xmp_dn11 = assign8050_e5934_d_n11;
        locals.var_xmp_dn12 = assign8050_e5934_d_n12;
        locals.var_xmp_dn17 = assign8050_e5934_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8060_e5942, assign8060_e5942_d_n0, assign8060_e5942_d_n2, assign8060_e5942_d_n6, assign8060_e5942_d_n7, assign8060_e5942_d_n10, assign8060_e5942_d_n11, assign8060_e5942_d_n12, assign8060_e5942_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8060_e5940: f64 = (locals.var_xp * locals.var_x2);
        (assign8060_e5940, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8060_e5942;
        locals.var_xp_dn0 = assign8060_e5942_d_n0;
        locals.var_xp_dn2 = assign8060_e5942_d_n2;
        locals.var_xp_dn6 = assign8060_e5942_d_n6;
        locals.var_xp_dn7 = assign8060_e5942_d_n7;
        locals.var_xp_dn10 = assign8060_e5942_d_n10;
        locals.var_xp_dn11 = assign8060_e5942_d_n11;
        locals.var_xp_dn12 = assign8060_e5942_d_n12;
        locals.var_xp_dn17 = assign8060_e5942_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8070_e5950, assign8070_e5950_d_n0, assign8070_e5950_d_n2, assign8070_e5950_d_n6, assign8070_e5950_d_n7, assign8070_e5950_d_n10, assign8070_e5950_d_n11, assign8070_e5950_d_n12, assign8070_e5950_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8070_e5948: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign8070_e5948, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8070_e5950;
        locals.var_xmp_dn0 = assign8070_e5950_d_n0;
        locals.var_xmp_dn2 = assign8070_e5950_d_n2;
        locals.var_xmp_dn6 = assign8070_e5950_d_n6;
        locals.var_xmp_dn7 = assign8070_e5950_d_n7;
        locals.var_xmp_dn10 = assign8070_e5950_d_n10;
        locals.var_xmp_dn11 = assign8070_e5950_d_n11;
        locals.var_xmp_dn12 = assign8070_e5950_d_n12;
        locals.var_xmp_dn17 = assign8070_e5950_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8080_e5958, assign8080_e5958_d_n0, assign8080_e5958_d_n2, assign8080_e5958_d_n6, assign8080_e5958_d_n7, assign8080_e5958_d_n10, assign8080_e5958_d_n11, assign8080_e5958_d_n12, assign8080_e5958_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8080_e5956: f64 = (locals.var_xp + locals.var_xmp);
        (assign8080_e5956, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign8080_e5958;
        locals.var_arg_dn0 = assign8080_e5958_d_n0;
        locals.var_arg_dn2 = assign8080_e5958_d_n2;
        locals.var_arg_dn6 = assign8080_e5958_d_n6;
        locals.var_arg_dn7 = assign8080_e5958_d_n7;
        locals.var_arg_dn10 = assign8080_e5958_d_n10;
        locals.var_arg_dn11 = assign8080_e5958_d_n11;
        locals.var_arg_dn12 = assign8080_e5958_d_n12;
        locals.var_arg_dn17 = assign8080_e5958_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign8090_e5964, assign8090_e5964_d_n0, assign8090_e5964_d_n2, assign8090_e5964_d_n6, assign8090_e5964_d_n7, assign8090_e5964_d_n10, assign8090_e5964_d_n11, assign8090_e5964_d_n12, assign8090_e5964_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8090_e5964;
        locals.var_dnm_dn0 = assign8090_e5964_d_n0;
        locals.var_dnm_dn2 = assign8090_e5964_d_n2;
        locals.var_dnm_dn6 = assign8090_e5964_d_n6;
        locals.var_dnm_dn7 = assign8090_e5964_d_n7;
        locals.var_dnm_dn10 = assign8090_e5964_d_n10;
        locals.var_dnm_dn11 = assign8090_e5964_d_n11;
        locals.var_dnm_dn12 = assign8090_e5964_d_n12;
        locals.var_dnm_dn17 = assign8090_e5964_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign8100_e5979: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard157 = assign8100_e5979;
        locals.var_guard157_rv = 0.0;

        let assign8110_e5982: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign8110_e5982;
        locals.var_guard158_rv = 0.0;

        let (assign8120_e5992,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) && (locals.var_guard158 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8120_e5992;
        locals.var_mm_rv = 0.0;

        let assign8130_e5995: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign8130_e5995;
        locals.var_guard159_rv = 0.0;

        let (assign8140_e6008,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8140_e6008;
        locals.var_mm_rv = 0.0;

        let assign8150_e6011: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign8150_e6011;
        locals.var_guard160_rv = 0.0;

        let (assign8160_e6027,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8160_e6027;
        locals.var_mm_rv = 0.0;

        let assign8170_e6030: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign8170_e6030;
        locals.var_guard161_rv = 0.0;

        let (assign8180_e6049,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) && (locals.var_guard161 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8180_e6049;
        locals.var_mm_rv = 0.0;

        let (assign8190_e6057,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign8190_e6057;
        locals.var_m0_rv = 0.0;

        let mut assign8200_loop_guard: usize = 0;
        while {
            let assign8200_cond_e6066: f64 = if ((((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign8200_cond_e6066 != 0.0
        } {
            assign8200_loop_guard += 1;
            assert!(assign8200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign8200_body0_e6075, assign8200_body0_e6075_d_n0, assign8200_body0_e6075_d_n2, assign8200_body0_e6075_d_n6, assign8200_body0_e6075_d_n7, assign8200_body0_e6075_d_n10, assign8200_body0_e6075_d_n11, assign8200_body0_e6075_d_n12, assign8200_body0_e6075_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8200_body0_e6073: f64 = (locals.var_dnm).sqrt();
        (assign8200_body0_e6073, (locals.var_dnm_dn0 / (2.0 * assign8200_body0_e6073)), (locals.var_dnm_dn2 / (2.0 * assign8200_body0_e6073)), (locals.var_dnm_dn6 / (2.0 * assign8200_body0_e6073)), (locals.var_dnm_dn7 / (2.0 * assign8200_body0_e6073)), (locals.var_dnm_dn10 / (2.0 * assign8200_body0_e6073)), (locals.var_dnm_dn11 / (2.0 * assign8200_body0_e6073)), (locals.var_dnm_dn12 / (2.0 * assign8200_body0_e6073)), (locals.var_dnm_dn17 / (2.0 * assign8200_body0_e6073)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign8200_body0_e6075;
            locals.var_dnm_dn0 = assign8200_body0_e6075_d_n0;
            locals.var_dnm_dn2 = assign8200_body0_e6075_d_n2;
            locals.var_dnm_dn6 = assign8200_body0_e6075_d_n6;
            locals.var_dnm_dn7 = assign8200_body0_e6075_d_n7;
            locals.var_dnm_dn10 = assign8200_body0_e6075_d_n10;
            locals.var_dnm_dn11 = assign8200_body0_e6075_d_n11;
            locals.var_dnm_dn12 = assign8200_body0_e6075_d_n12;
            locals.var_dnm_dn17 = assign8200_body0_e6075_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign8200_body1_e6085,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 != 0.0)) {
        let assign8200_body1_e6083: f64 = (locals.var_m0 + 1.0);
        (assign8200_body1_e6083,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign8200_body1_e6085;
            locals.var_m0_rv = 0.0;
        }

        let (assign8210_e6100, assign8210_e6100_d_n0, assign8210_e6100_d_n2, assign8210_e6100_d_n6, assign8210_e6100_d_n7, assign8210_e6100_d_n10, assign8210_e6100_d_n11, assign8210_e6100_d_n12, assign8210_e6100_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) && (locals.var_guard157 == 0.0)) {
        let assign8210_e6096: f64 = (2.0 * 2.0);
        let assign8210_e6097: f64 = (1.0 / assign8210_e6096);
        let assign8210_e6098: f64 = (locals.var_dnm).powf(assign8210_e6097);
        (assign8210_e6098, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn0)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn2)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn6)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn7)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn10)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn11)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn12)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8210_e6097) as f64).is_finite() && ((assign8210_e6097) as f64).fract() == 0.0 { if assign8210_e6097 == 0.0 { 0.0 } else { (assign8210_e6097 * ((locals.var_dnm).powf(assign8210_e6097 - 1.0) * locals.var_dnm_dn17)) } } else { (assign8210_e6098 * (assign8210_e6097 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8210_e6100;
        locals.var_dnm_dn0 = assign8210_e6100_d_n0;
        locals.var_dnm_dn2 = assign8210_e6100_d_n2;
        locals.var_dnm_dn6 = assign8210_e6100_d_n6;
        locals.var_dnm_dn7 = assign8210_e6100_d_n7;
        locals.var_dnm_dn10 = assign8210_e6100_d_n10;
        locals.var_dnm_dn11 = assign8210_e6100_d_n11;
        locals.var_dnm_dn12 = assign8210_e6100_d_n12;
        locals.var_dnm_dn17 = assign8210_e6100_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign8220_e6108, assign8220_e6108_d_n0, assign8220_e6108_d_n2, assign8220_e6108_d_n6, assign8220_e6108_d_n7, assign8220_e6108_d_n10, assign8220_e6108_d_n11, assign8220_e6108_d_n12, assign8220_e6108_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8220_e6106: f64 = (1.0 / locals.var_dnm);
        (assign8220_e6106, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8220_e6108;
        locals.var_dnm_dn0 = assign8220_e6108_d_n0;
        locals.var_dnm_dn2 = assign8220_e6108_d_n2;
        locals.var_dnm_dn6 = assign8220_e6108_d_n6;
        locals.var_dnm_dn7 = assign8220_e6108_d_n7;
        locals.var_dnm_dn10 = assign8220_e6108_d_n10;
        locals.var_dnm_dn11 = assign8220_e6108_d_n11;
        locals.var_dnm_dn12 = assign8220_e6108_d_n12;
        locals.var_dnm_dn17 = assign8220_e6108_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign8230_e6120, assign8230_e6120_d_n0, assign8230_e6120_d_n2, assign8230_e6120_d_n6, assign8230_e6120_d_n7, assign8230_e6120_d_n10, assign8230_e6120_d_n11, assign8230_e6120_d_n12, assign8230_e6120_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8230_e6115: f64 = locals.var_t_soi;
        let assign8230_e6116: f64 = (locals.var_tmf1 * assign8230_e6115);
        let assign8230_e6118: f64 = (assign8230_e6116 * locals.var_dnm);
        (assign8230_e6118, (((locals.var_tmf1_dn0 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * assign8230_e6115) * locals.var_dnm) + (assign8230_e6116 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign8230_e6120;
        locals.var_tmf0_dn0 = assign8230_e6120_d_n0;
        locals.var_tmf0_dn2 = assign8230_e6120_d_n2;
        locals.var_tmf0_dn6 = assign8230_e6120_d_n6;
        locals.var_tmf0_dn7 = assign8230_e6120_d_n7;
        locals.var_tmf0_dn10 = assign8230_e6120_d_n10;
        locals.var_tmf0_dn11 = assign8230_e6120_d_n11;
        locals.var_tmf0_dn12 = assign8230_e6120_d_n12;
        locals.var_tmf0_dn17 = assign8230_e6120_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign8240_e6132, assign8240_e6132_d_n0, assign8240_e6132_d_n2, assign8240_e6132_d_n6, assign8240_e6132_d_n7, assign8240_e6132_d_n10, assign8240_e6132_d_n11, assign8240_e6132_d_n12, assign8240_e6132_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 != 0.0)) {
        let assign8240_e6127: f64 = locals.var_t_soi;
        let assign8240_e6128: f64 = (locals.var_wdsoi_ini0 - assign8240_e6127);
        let assign8240_e6130: f64 = (assign8240_e6128 + locals.var_tmf0);
        (assign8240_e6130, (locals.var_wdsoi_ini0_dn0 + locals.var_tmf0_dn0), (locals.var_wdsoi_ini0_dn2 + locals.var_tmf0_dn2), (locals.var_wdsoi_ini0_dn6 + locals.var_tmf0_dn6), (locals.var_wdsoi_ini0_dn7 + locals.var_tmf0_dn7), (locals.var_wdsoi_ini0_dn10 + locals.var_tmf0_dn10), (locals.var_wdsoi_ini0_dn11 + locals.var_tmf0_dn11), (locals.var_wdsoi_ini0_dn12 + locals.var_tmf0_dn12), (locals.var_wdsoi_ini0_dn17 + locals.var_tmf0_dn17),)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign8240_e6132;
        locals.var_wdsoi_ini2_dn0 = assign8240_e6132_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign8240_e6132_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign8240_e6132_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign8240_e6132_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign8240_e6132_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign8240_e6132_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign8240_e6132_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign8240_e6132_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let (assign8250_e6139, assign8250_e6139_d_n0, assign8250_e6139_d_n2, assign8250_e6139_d_n6, assign8250_e6139_d_n7, assign8250_e6139_d_n10, assign8250_e6139_d_n11, assign8250_e6139_d_n12, assign8250_e6139_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard156 == 0.0)) {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign8250_e6139;
        locals.var_wdsoi_ini2_dn0 = assign8250_e6139_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign8250_e6139_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign8250_e6139_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign8250_e6139_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign8250_e6139_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign8250_e6139_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign8250_e6139_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign8250_e6139_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let (assign8260_e6146, assign8260_e6146_d_n0, assign8260_e6146_d_n2, assign8260_e6146_d_n6, assign8260_e6146_d_n7, assign8260_e6146_d_n10, assign8260_e6146_d_n11, assign8260_e6146_d_n12, assign8260_e6146_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign8260_e6142: f64 = (-locals.var_wdsoi_ini2);
        let assign8260_e6144: f64 = (assign8260_e6142 * locals.var_q_nsub);
        (assign8260_e6144, (((-locals.var_wdsoi_ini2_dn0) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn0)), (((-locals.var_wdsoi_ini2_dn2) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn2)), (((-locals.var_wdsoi_ini2_dn6) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn6)), (((-locals.var_wdsoi_ini2_dn7) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn7)), (((-locals.var_wdsoi_ini2_dn10) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn10)), (((-locals.var_wdsoi_ini2_dn11) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn11)), (((-locals.var_wdsoi_ini2_dn12) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn12)), (((-locals.var_wdsoi_ini2_dn17) * locals.var_q_nsub) + (assign8260_e6142 * locals.var_q_nsub_dn17)),)
    } else {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    }
};
        locals.var_q_s0_dep_ini = assign8260_e6146;
        locals.var_q_s0_dep_ini_dn0 = assign8260_e6146_d_n0;
        locals.var_q_s0_dep_ini_dn2 = assign8260_e6146_d_n2;
        locals.var_q_s0_dep_ini_dn6 = assign8260_e6146_d_n6;
        locals.var_q_s0_dep_ini_dn7 = assign8260_e6146_d_n7;
        locals.var_q_s0_dep_ini_dn10 = assign8260_e6146_d_n10;
        locals.var_q_s0_dep_ini_dn11 = assign8260_e6146_d_n11;
        locals.var_q_s0_dep_ini_dn12 = assign8260_e6146_d_n12;
        locals.var_q_s0_dep_ini_dn17 = assign8260_e6146_d_n17;
        locals.var_q_s0_dep_ini_rv = 0.0;

        let (assign8270_e6159, assign8270_e6159_d_n0, assign8270_e6159_d_n2, assign8270_e6159_d_n6, assign8270_e6159_d_n7, assign8270_e6159_d_n10, assign8270_e6159_d_n11, assign8270_e6159_d_n12, assign8270_e6159_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign8270_e6149: f64 = (-locals.var_q_fd_soi);
        let assign8270_e6151: f64 = (assign8270_e6149 * locals.var_t_soi);
        let assign8270_e6153: f64 = (assign8270_e6151 / 2.0);
        let assign8270_e6155: f64 = (assign8270_e6153 / 1.034943e-10);
        let assign8270_e6157: f64 = (assign8270_e6155 + locals.var_beta_inv);
        (assign8270_e6157, ((((-locals.var_q_fd_soi_dn0) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn2) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn6) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn7) * locals.var_t_soi) / 2.0) / 1.034943e-10), (((((-locals.var_q_fd_soi_dn10) * locals.var_t_soi) / 2.0) / 1.034943e-10) + locals.var_beta_inv_dn10), ((((-locals.var_q_fd_soi_dn11) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn12) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn17) * locals.var_t_soi) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_fd_start, locals.var_fd_start_dn0, locals.var_fd_start_dn2, locals.var_fd_start_dn6, locals.var_fd_start_dn7, locals.var_fd_start_dn10, locals.var_fd_start_dn11, locals.var_fd_start_dn12, locals.var_fd_start_dn17,)
    }
};
        locals.var_fd_start = assign8270_e6159;
        locals.var_fd_start_dn0 = assign8270_e6159_d_n0;
        locals.var_fd_start_dn2 = assign8270_e6159_d_n2;
        locals.var_fd_start_dn6 = assign8270_e6159_d_n6;
        locals.var_fd_start_dn7 = assign8270_e6159_d_n7;
        locals.var_fd_start_dn10 = assign8270_e6159_d_n10;
        locals.var_fd_start_dn11 = assign8270_e6159_d_n11;
        locals.var_fd_start_dn12 = assign8270_e6159_d_n12;
        locals.var_fd_start_dn17 = assign8270_e6159_d_n17;
        locals.var_fd_start_rv = 0.0;

        let (assign8280_e6169, assign8280_e6169_d_n0, assign8280_e6169_d_n2, assign8280_e6169_d_n6, assign8280_e6169_d_n7, assign8280_e6169_d_n10, assign8280_e6169_d_n11, assign8280_e6169_d_n12, assign8280_e6169_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign8280_e6164: f64 = (locals.var_q_s0_bulk_0 * locals.var_t_soi);
        let assign8280_e6166: f64 = (assign8280_e6164 / 1.034943e-10);
        let assign8280_e6167: f64 = (locals.var_fd_start - assign8280_e6166);
        (assign8280_e6167, (locals.var_fd_start_dn0 - ((locals.var_q_s0_bulk_0_dn0 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn2 - ((locals.var_q_s0_bulk_0_dn2 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn6 - ((locals.var_q_s0_bulk_0_dn6 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn7 - ((locals.var_q_s0_bulk_0_dn7 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn10 - ((locals.var_q_s0_bulk_0_dn10 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn11 - ((locals.var_q_s0_bulk_0_dn11 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn12 - ((locals.var_q_s0_bulk_0_dn12 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn17 - ((locals.var_q_s0_bulk_0_dn17 * locals.var_t_soi) / 1.034943e-10)),)
    } else {
        (locals.var_fd_end, locals.var_fd_end_dn0, locals.var_fd_end_dn2, locals.var_fd_end_dn6, locals.var_fd_end_dn7, locals.var_fd_end_dn10, locals.var_fd_end_dn11, locals.var_fd_end_dn12, locals.var_fd_end_dn17,)
    }
};
        locals.var_fd_end = assign8280_e6169;
        locals.var_fd_end_dn0 = assign8280_e6169_d_n0;
        locals.var_fd_end_dn2 = assign8280_e6169_d_n2;
        locals.var_fd_end_dn6 = assign8280_e6169_d_n6;
        locals.var_fd_end_dn7 = assign8280_e6169_d_n7;
        locals.var_fd_end_dn10 = assign8280_e6169_d_n10;
        locals.var_fd_end_dn11 = assign8280_e6169_d_n11;
        locals.var_fd_end_dn12 = assign8280_e6169_d_n12;
        locals.var_fd_end_dn17 = assign8280_e6169_d_n17;
        locals.var_fd_end_rv = 0.0;

        let assign8290_e6172: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign8290_e6172;
        locals.var_guard162_rv = 0.0;

        let (assign8300_e6178, assign8300_e6178_d_n0, assign8300_e6178_d_n2, assign8300_e6178_d_n6, assign8300_e6178_d_n7, assign8300_e6178_d_n10, assign8300_e6178_d_n11, assign8300_e6178_d_n12, assign8300_e6178_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign8300_e6178;
        locals.var_phi_s0_soi_dn0 = assign8300_e6178_d_n0;
        locals.var_phi_s0_soi_dn2 = assign8300_e6178_d_n2;
        locals.var_phi_s0_soi_dn6 = assign8300_e6178_d_n6;
        locals.var_phi_s0_soi_dn7 = assign8300_e6178_d_n7;
        locals.var_phi_s0_soi_dn10 = assign8300_e6178_d_n10;
        locals.var_phi_s0_soi_dn11 = assign8300_e6178_d_n11;
        locals.var_phi_s0_soi_dn12 = assign8300_e6178_d_n12;
        locals.var_phi_s0_soi_dn17 = assign8300_e6178_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign8310_e6184, assign8310_e6184_d_n0, assign8310_e6184_d_n2, assign8310_e6184_d_n6, assign8310_e6184_d_n7, assign8310_e6184_d_n10, assign8310_e6184_d_n11, assign8310_e6184_d_n12, assign8310_e6184_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 != 0.0)) {
        (locals.var_pbs0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    }
};
        locals.var_phi_b0_soi = assign8310_e6184;
        locals.var_phi_b0_soi_dn0 = assign8310_e6184_d_n0;
        locals.var_phi_b0_soi_dn2 = assign8310_e6184_d_n2;
        locals.var_phi_b0_soi_dn6 = assign8310_e6184_d_n6;
        locals.var_phi_b0_soi_dn7 = assign8310_e6184_d_n7;
        locals.var_phi_b0_soi_dn10 = assign8310_e6184_d_n10;
        locals.var_phi_b0_soi_dn11 = assign8310_e6184_d_n11;
        locals.var_phi_b0_soi_dn12 = assign8310_e6184_d_n12;
        locals.var_phi_b0_soi_dn17 = assign8310_e6184_d_n17;
        locals.var_phi_b0_soi_rv = 0.0;

        let (assign8320_e6190, assign8320_e6190_d_n0, assign8320_e6190_d_n2, assign8320_e6190_d_n6, assign8320_e6190_d_n7, assign8320_e6190_d_n10, assign8320_e6190_d_n11, assign8320_e6190_d_n12, assign8320_e6190_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 != 0.0)) {
        (locals.var_psb0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign8320_e6190;
        locals.var_phi_s0_bulk_dn0 = assign8320_e6190_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign8320_e6190_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign8320_e6190_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign8320_e6190_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign8320_e6190_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign8320_e6190_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign8320_e6190_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign8320_e6190_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign8330_e6201,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 != 0.0)) {
        let (assign8330_e6199,) = {
            if (locals.var_phi_s0_soi < locals.var_fd_end) {
                (1.0,)
            } else {
                (2.0,)
            }
        };
        (assign8330_e6199,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign8330_e6201;
        locals.var_flg_depmode_rv = 0.0;

        let (assign8340_e6220, assign8340_e6220_d_n0, assign8340_e6220_d_n2, assign8340_e6220_d_n6, assign8340_e6220_d_n7, assign8340_e6220_d_n10, assign8340_e6220_d_n11, assign8340_e6220_d_n12, assign8340_e6220_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign8340_e6210: f64 = (locals.var_beta * locals.var_vgpz);
        let assign8340_e6212: f64 = (assign8340_e6210 - 1.0);
        let assign8340_e6213: f64 = (4.0 * assign8340_e6212);
        let assign8340_e6216: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign8340_e6217: f64 = (assign8340_e6213 / assign8340_e6216);
        let assign8340_e6218: f64 = (1.0 + assign8340_e6217);
        (assign8340_e6218, ((((4.0 * (locals.var_beta * locals.var_vgpz_dn0)) * assign8340_e6216) - (assign8340_e6213 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign8340_e6216 * assign8340_e6216)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn2)) * assign8340_e6216) - (assign8340_e6213 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign8340_e6216 * assign8340_e6216)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn6)) * assign8340_e6216) - (assign8340_e6213 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign8340_e6216 * assign8340_e6216)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn7)) * assign8340_e6216) - (assign8340_e6213 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign8340_e6216 * assign8340_e6216)), ((((4.0 * ((locals.var_beta_dn10 * locals.var_vgpz) + (locals.var_beta * locals.var_vgpz_dn10))) * assign8340_e6216) - (assign8340_e6213 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign8340_e6216 * assign8340_e6216)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn11)) * assign8340_e6216) - (assign8340_e6213 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign8340_e6216 * assign8340_e6216)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn12)) * assign8340_e6216) - (assign8340_e6213 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign8340_e6216 * assign8340_e6216)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn17)) * assign8340_e6216) - (assign8340_e6213 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign8340_e6216 * assign8340_e6216)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign8340_e6220;
        locals.var_tx_dn0 = assign8340_e6220_d_n0;
        locals.var_tx_dn2 = assign8340_e6220_d_n2;
        locals.var_tx_dn6 = assign8340_e6220_d_n6;
        locals.var_tx_dn7 = assign8340_e6220_d_n7;
        locals.var_tx_dn10 = assign8340_e6220_d_n10;
        locals.var_tx_dn11 = assign8340_e6220_d_n11;
        locals.var_tx_dn12 = assign8340_e6220_d_n12;
        locals.var_tx_dn17 = assign8340_e6220_d_n17;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign8350_e6236, assign8350_e6236_d_n0, assign8350_e6236_d_n2, assign8350_e6236_d_n6, assign8350_e6236_d_n7, assign8350_e6236_d_n10, assign8350_e6236_d_n11, assign8350_e6236_d_n12, assign8350_e6236_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign8350_e6228: f64 = (10.0 * 2.220446049250313e-16);
        let (assign8350_e6234, assign8350_e6234_d_n0, assign8350_e6234_d_n2, assign8350_e6234_d_n6, assign8350_e6234_d_n7, assign8350_e6234_d_n10, assign8350_e6234_d_n11, assign8350_e6234_d_n12, assign8350_e6234_d_n17,) = {
            if (locals.var_tx >= assign8350_e6228) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign8350_e6233: f64 = (10.0 * 2.220446049250313e-16);
                (assign8350_e6233, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign8350_e6234, assign8350_e6234_d_n0, assign8350_e6234_d_n2, assign8350_e6234_d_n6, assign8350_e6234_d_n7, assign8350_e6234_d_n10, assign8350_e6234_d_n11, assign8350_e6234_d_n12, assign8350_e6234_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign8350_e6236;
        locals.var_tx_dn0 = assign8350_e6236_d_n0;
        locals.var_tx_dn2 = assign8350_e6236_d_n2;
        locals.var_tx_dn6 = assign8350_e6236_d_n6;
        locals.var_tx_dn7 = assign8350_e6236_d_n7;
        locals.var_tx_dn10 = assign8350_e6236_d_n10;
        locals.var_tx_dn11 = assign8350_e6236_d_n11;
        locals.var_tx_dn12 = assign8350_e6236_d_n12;
        locals.var_tx_dn17 = assign8350_e6236_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign8360_e6254, assign8360_e6254_d_n0, assign8360_e6254_d_n2, assign8360_e6254_d_n6, assign8360_e6254_d_n7, assign8360_e6254_d_n10, assign8360_e6254_d_n11, assign8360_e6254_d_n12, assign8360_e6254_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign8360_e6244: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign8360_e6246: f64 = (assign8360_e6244 * 0.5);
        let assign8360_e6249: f64 = (locals.var_tx).sqrt();
        let assign8360_e6250: f64 = (1.0 - assign8360_e6249);
        let assign8360_e6251: f64 = (assign8360_e6246 * assign8360_e6250);
        let assign8360_e6252: f64 = (locals.var_vgpz + assign8360_e6251);
        (assign8360_e6252, (locals.var_vgpz_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn0 / (2.0 * assign8360_e6249)))))), (locals.var_vgpz_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn2 / (2.0 * assign8360_e6249)))))), (locals.var_vgpz_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn6 / (2.0 * assign8360_e6249)))))), (locals.var_vgpz_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn7 / (2.0 * assign8360_e6249)))))), (locals.var_vgpz_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn10 / (2.0 * assign8360_e6249)))))), (locals.var_vgpz_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn11 / (2.0 * assign8360_e6249)))))), (locals.var_vgpz_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn12 / (2.0 * assign8360_e6249)))))), (locals.var_vgpz_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign8360_e6250) + (assign8360_e6246 * (-(locals.var_tx_dn17 / (2.0 * assign8360_e6249)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8360_e6254;
        locals.var_ps0_inia_dn0 = assign8360_e6254_d_n0;
        locals.var_ps0_inia_dn2 = assign8360_e6254_d_n2;
        locals.var_ps0_inia_dn6 = assign8360_e6254_d_n6;
        locals.var_ps0_inia_dn7 = assign8360_e6254_d_n7;
        locals.var_ps0_inia_dn10 = assign8360_e6254_d_n10;
        locals.var_ps0_inia_dn11 = assign8360_e6254_d_n11;
        locals.var_ps0_inia_dn12 = assign8360_e6254_d_n12;
        locals.var_ps0_inia_dn17 = assign8360_e6254_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8370_e6263, assign8370_e6263_d_n0, assign8370_e6263_d_n2, assign8370_e6263_d_n6, assign8370_e6263_d_n7, assign8370_e6263_d_n10, assign8370_e6263_d_n11, assign8370_e6263_d_n12, assign8370_e6263_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign8370_e6261: f64 = (locals.var_beta * locals.var_ps0_inia);
        (assign8370_e6261, (locals.var_beta * locals.var_ps0_inia_dn0), (locals.var_beta * locals.var_ps0_inia_dn2), (locals.var_beta * locals.var_ps0_inia_dn6), (locals.var_beta * locals.var_ps0_inia_dn7), ((locals.var_beta_dn10 * locals.var_ps0_inia) + (locals.var_beta * locals.var_ps0_inia_dn10)), (locals.var_beta * locals.var_ps0_inia_dn11), (locals.var_beta * locals.var_ps0_inia_dn12), (locals.var_beta * locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign8370_e6263;
        locals.var_chi_dn0 = assign8370_e6263_d_n0;
        locals.var_chi_dn2 = assign8370_e6263_d_n2;
        locals.var_chi_dn6 = assign8370_e6263_d_n6;
        locals.var_chi_dn7 = assign8370_e6263_d_n7;
        locals.var_chi_dn10 = assign8370_e6263_d_n10;
        locals.var_chi_dn11 = assign8370_e6263_d_n11;
        locals.var_chi_dn12 = assign8370_e6263_d_n12;
        locals.var_chi_dn17 = assign8370_e6263_d_n17;
        locals.var_chi_rv = 0.0;

        let assign8380_e6266: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign8380_e6266;
        locals.var_guard163_rv = 0.0;

        let (assign8390_e6279, assign8390_e6279_d_n0, assign8390_e6279_d_n2, assign8390_e6279_d_n6, assign8390_e6279_d_n7, assign8390_e6279_d_n10, assign8390_e6279_d_n11, assign8390_e6279_d_n12, assign8390_e6279_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8390_e6276: f64 = (locals.var_vgpz - locals.var_vbs);
        let assign8390_e6277: f64 = (locals.var_beta * assign8390_e6276);
        (assign8390_e6277, (locals.var_beta * (locals.var_vgpz_dn0 - locals.var_vbs_dn0)), (locals.var_beta * (locals.var_vgpz_dn2 - locals.var_vbs_dn2)), (locals.var_beta * (locals.var_vgpz_dn6 - locals.var_vbs_dn6)), (locals.var_beta * (locals.var_vgpz_dn7 - locals.var_vbs_dn7)), ((locals.var_beta_dn10 * assign8390_e6276) + (locals.var_beta * (locals.var_vgpz_dn10 - locals.var_vbs_dn10))), (locals.var_beta * (locals.var_vgpz_dn11 - locals.var_vbs_dn11)), (locals.var_beta * (locals.var_vgpz_dn12 - locals.var_vbs_dn12)), (locals.var_beta * (locals.var_vgpz_dn17 - locals.var_vbs_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign8390_e6279;
        locals.var_ty_dn0 = assign8390_e6279_d_n0;
        locals.var_ty_dn2 = assign8390_e6279_d_n2;
        locals.var_ty_dn6 = assign8390_e6279_d_n6;
        locals.var_ty_dn7 = assign8390_e6279_d_n7;
        locals.var_ty_dn10 = assign8390_e6279_d_n10;
        locals.var_ty_dn11 = assign8390_e6279_d_n11;
        locals.var_ty_dn12 = assign8390_e6279_d_n12;
        locals.var_ty_dn17 = assign8390_e6279_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign8400_e6296, assign8400_e6296_d_n0, assign8400_e6296_d_n2, assign8400_e6296_d_n6, assign8400_e6296_d_n7, assign8400_e6296_d_n10, assign8400_e6296_d_n11, assign8400_e6296_d_n12, assign8400_e6296_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8400_e6289: f64 = (1.414213562373095 / 108.0);
        let assign8400_e6291: f64 = (assign8400_e6289 * locals.var_beta);
        let assign8400_e6293: f64 = (assign8400_e6291 * locals.var_fac1);
        let assign8400_e6294: f64 = (1.0 / assign8400_e6293);
        (assign8400_e6294, (-((assign8400_e6291 * locals.var_fac1_dn0) / (assign8400_e6293 * assign8400_e6293))), (-((assign8400_e6291 * locals.var_fac1_dn2) / (assign8400_e6293 * assign8400_e6293))), (-((assign8400_e6291 * locals.var_fac1_dn6) / (assign8400_e6293 * assign8400_e6293))), (-((assign8400_e6291 * locals.var_fac1_dn7) / (assign8400_e6293 * assign8400_e6293))), (-((((assign8400_e6289 * locals.var_beta_dn10) * locals.var_fac1) + (assign8400_e6291 * locals.var_fac1_dn10)) / (assign8400_e6293 * assign8400_e6293))), (-((assign8400_e6291 * locals.var_fac1_dn11) / (assign8400_e6293 * assign8400_e6293))), (-((assign8400_e6291 * locals.var_fac1_dn12) / (assign8400_e6293 * assign8400_e6293))), (-((assign8400_e6291 * locals.var_fac1_dn17) / (assign8400_e6293 * assign8400_e6293))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8400_e6296;
        locals.var_t1_dn0 = assign8400_e6296_d_n0;
        locals.var_t1_dn2 = assign8400_e6296_d_n2;
        locals.var_t1_dn6 = assign8400_e6296_d_n6;
        locals.var_t1_dn7 = assign8400_e6296_d_n7;
        locals.var_t1_dn10 = assign8400_e6296_d_n10;
        locals.var_t1_dn11 = assign8400_e6296_d_n11;
        locals.var_t1_dn12 = assign8400_e6296_d_n12;
        locals.var_t1_dn17 = assign8400_e6296_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8410_e6309, assign8410_e6309_d_n0, assign8410_e6309_d_n2, assign8410_e6309_d_n6, assign8410_e6309_d_n7, assign8410_e6309_d_n10, assign8410_e6309_d_n11, assign8410_e6309_d_n12, assign8410_e6309_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8410_e6306: f64 = (3.0 * locals.var_t1);
        let assign8410_e6307: f64 = (81.0 + assign8410_e6306);
        (assign8410_e6307, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8410_e6309;
        locals.var_t2_dn0 = assign8410_e6309_d_n0;
        locals.var_t2_dn2 = assign8410_e6309_d_n2;
        locals.var_t2_dn6 = assign8410_e6309_d_n6;
        locals.var_t2_dn7 = assign8410_e6309_d_n7;
        locals.var_t2_dn10 = assign8410_e6309_d_n10;
        locals.var_t2_dn11 = assign8410_e6309_d_n11;
        locals.var_t2_dn12 = assign8410_e6309_d_n12;
        locals.var_t2_dn17 = assign8410_e6309_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8420_e6329, assign8420_e6329_d_n0, assign8420_e6329_d_n2, assign8420_e6329_d_n6, assign8420_e6329_d_n7, assign8420_e6329_d_n10, assign8420_e6329_d_n11, assign8420_e6329_d_n12, assign8420_e6329_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8420_e6317: f64 = (-2916.0);
        let assign8420_e6320: f64 = (81.0 * locals.var_t1);
        let assign8420_e6321: f64 = (assign8420_e6317 - assign8420_e6320);
        let assign8420_e6324: f64 = (27.0 * locals.var_t1);
        let assign8420_e6326: f64 = (assign8420_e6324 * locals.var_ty);
        let assign8420_e6327: f64 = (assign8420_e6321 + assign8420_e6326);
        (assign8420_e6327, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign8420_e6324 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8420_e6329;
        locals.var_t3_dn0 = assign8420_e6329_d_n0;
        locals.var_t3_dn2 = assign8420_e6329_d_n2;
        locals.var_t3_dn6 = assign8420_e6329_d_n6;
        locals.var_t3_dn7 = assign8420_e6329_d_n7;
        locals.var_t3_dn10 = assign8420_e6329_d_n10;
        locals.var_t3_dn11 = assign8420_e6329_d_n11;
        locals.var_t3_dn12 = assign8420_e6329_d_n12;
        locals.var_t3_dn17 = assign8420_e6329_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8430_e6350, assign8430_e6350_d_n0, assign8430_e6350_d_n2, assign8430_e6350_d_n6, assign8430_e6350_d_n7, assign8430_e6350_d_n10, assign8430_e6350_d_n11, assign8430_e6350_d_n12, assign8430_e6350_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8430_e6340: f64 = (54.0 + locals.var_t1);
        let assign8430_e6341: f64 = (81.0 * assign8430_e6340);
        let assign8430_e6342: f64 = (1458.0 - assign8430_e6341);
        let assign8430_e6345: f64 = (27.0 * locals.var_t1);
        let assign8430_e6347: f64 = (assign8430_e6345 * locals.var_ty);
        let assign8430_e6348: f64 = (assign8430_e6342 + assign8430_e6347);
        (assign8430_e6348, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign8430_e6345 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8430_e6350;
        locals.var_t4_dn0 = assign8430_e6350_d_n0;
        locals.var_t4_dn2 = assign8430_e6350_d_n2;
        locals.var_t4_dn6 = assign8430_e6350_d_n6;
        locals.var_t4_dn7 = assign8430_e6350_d_n7;
        locals.var_t4_dn10 = assign8430_e6350_d_n10;
        locals.var_t4_dn11 = assign8430_e6350_d_n11;
        locals.var_t4_dn12 = assign8430_e6350_d_n12;
        locals.var_t4_dn17 = assign8430_e6350_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8440_e6361, assign8440_e6361_d_n0, assign8440_e6361_d_n2, assign8440_e6361_d_n6, assign8440_e6361_d_n7, assign8440_e6361_d_n10, assign8440_e6361_d_n11, assign8440_e6361_d_n12, assign8440_e6361_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8440_e6359: f64 = (locals.var_t4 * locals.var_t4);
        (assign8440_e6359, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8440_e6361;
        locals.var_t4_dn0 = assign8440_e6361_d_n0;
        locals.var_t4_dn2 = assign8440_e6361_d_n2;
        locals.var_t4_dn6 = assign8440_e6361_d_n6;
        locals.var_t4_dn7 = assign8440_e6361_d_n7;
        locals.var_t4_dn10 = assign8440_e6361_d_n10;
        locals.var_t4_dn11 = assign8440_e6361_d_n11;
        locals.var_t4_dn12 = assign8440_e6361_d_n12;
        locals.var_t4_dn17 = assign8440_e6361_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8450_e6383, assign8450_e6383_d_n0, assign8450_e6383_d_n2, assign8450_e6383_d_n6, assign8450_e6383_d_n7, assign8450_e6383_d_n10, assign8450_e6383_d_n11, assign8450_e6383_d_n12, assign8450_e6383_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8450_e6371: f64 = (4.0 * locals.var_t2);
        let assign8450_e6373: f64 = (assign8450_e6371 * locals.var_t2);
        let assign8450_e6375: f64 = (assign8450_e6373 * locals.var_t2);
        let assign8450_e6377: f64 = (assign8450_e6375 + locals.var_t4);
        let assign8450_e6378: f64 = (assign8450_e6377).sqrt();
        let assign8450_e6379: f64 = (locals.var_t3 + assign8450_e6378);
        let assign8450_e6381: f64 = (assign8450_e6379).powf(0.3333333333333333);
        (assign8450_e6381, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn0)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn0)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign8450_e6378))) / assign8450_e6379))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn2)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn2)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign8450_e6378))) / assign8450_e6379))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn6)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn6)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign8450_e6378))) / assign8450_e6379))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn7)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn7)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign8450_e6378))) / assign8450_e6379))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn10)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn10)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign8450_e6378))) / assign8450_e6379))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn11)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn11)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign8450_e6378))) / assign8450_e6379))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn12)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn12)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign8450_e6378))) / assign8450_e6379))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8450_e6379).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn17)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign8450_e6378))))) } } else { (assign8450_e6381 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign8450_e6371 * locals.var_t2_dn17)) * locals.var_t2) + (assign8450_e6373 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign8450_e6378))) / assign8450_e6379))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign8450_e6383;
        locals.var_t5_dn0 = assign8450_e6383_d_n0;
        locals.var_t5_dn2 = assign8450_e6383_d_n2;
        locals.var_t5_dn6 = assign8450_e6383_d_n6;
        locals.var_t5_dn7 = assign8450_e6383_d_n7;
        locals.var_t5_dn10 = assign8450_e6383_d_n10;
        locals.var_t5_dn11 = assign8450_e6383_d_n11;
        locals.var_t5_dn12 = assign8450_e6383_d_n12;
        locals.var_t5_dn17 = assign8450_e6383_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign8460_e6408, assign8460_e6408_d_n0, assign8460_e6408_d_n2, assign8460_e6408_d_n6, assign8460_e6408_d_n7, assign8460_e6408_d_n10, assign8460_e6408_d_n11, assign8460_e6408_d_n12, assign8460_e6408_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8460_e6393: f64 = (1.259921049894873 * locals.var_t2);
        let assign8460_e6396: f64 = (3.0 * locals.var_t5);
        let assign8460_e6397: f64 = (assign8460_e6393 / assign8460_e6396);
        let assign8460_e6398: f64 = (3.0 - assign8460_e6397);
        let assign8460_e6402: f64 = (3.0 * 1.259921049894873);
        let assign8460_e6403: f64 = (1.0 / assign8460_e6402);
        let assign8460_e6405: f64 = (assign8460_e6403 * locals.var_t5);
        let assign8460_e6406: f64 = (assign8460_e6398 + assign8460_e6405);
        (assign8460_e6406, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn0))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn2))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn6))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn7))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn10))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn11))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn12))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign8460_e6396) - (assign8460_e6393 * (3.0 * locals.var_t5_dn17))) / (assign8460_e6396 * assign8460_e6396))) + (assign8460_e6403 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign8460_e6408;
        locals.var_tx_dn0 = assign8460_e6408_d_n0;
        locals.var_tx_dn2 = assign8460_e6408_d_n2;
        locals.var_tx_dn6 = assign8460_e6408_d_n6;
        locals.var_tx_dn7 = assign8460_e6408_d_n7;
        locals.var_tx_dn10 = assign8460_e6408_d_n10;
        locals.var_tx_dn11 = assign8460_e6408_d_n11;
        locals.var_tx_dn12 = assign8460_e6408_d_n12;
        locals.var_tx_dn17 = assign8460_e6408_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign8470_e6421, assign8470_e6421_d_n0, assign8470_e6421_d_n2, assign8470_e6421_d_n6, assign8470_e6421_d_n7, assign8470_e6421_d_n10, assign8470_e6421_d_n11, assign8470_e6421_d_n12, assign8470_e6421_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8470_e6417: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign8470_e6419: f64 = (assign8470_e6417 + locals.var_vbs);
        (assign8470_e6419, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbs_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbs_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbs_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbs_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbs_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbs_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbs_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbs_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8470_e6421;
        locals.var_ps0_inia_dn0 = assign8470_e6421_d_n0;
        locals.var_ps0_inia_dn2 = assign8470_e6421_d_n2;
        locals.var_ps0_inia_dn6 = assign8470_e6421_d_n6;
        locals.var_ps0_inia_dn7 = assign8470_e6421_d_n7;
        locals.var_ps0_inia_dn10 = assign8470_e6421_d_n10;
        locals.var_ps0_inia_dn11 = assign8470_e6421_d_n11;
        locals.var_ps0_inia_dn12 = assign8470_e6421_d_n12;
        locals.var_ps0_inia_dn17 = assign8470_e6421_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8480_e6430, assign8480_e6430_d_n0, assign8480_e6430_d_n2, assign8480_e6430_d_n6, assign8480_e6430_d_n7, assign8480_e6430_d_n10, assign8480_e6430_d_n11, assign8480_e6430_d_n12, assign8480_e6430_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8480_e6430;
        locals.var_ps0_ini_dn0 = assign8480_e6430_d_n0;
        locals.var_ps0_ini_dn2 = assign8480_e6430_d_n2;
        locals.var_ps0_ini_dn6 = assign8480_e6430_d_n6;
        locals.var_ps0_ini_dn7 = assign8480_e6430_d_n7;
        locals.var_ps0_ini_dn10 = assign8480_e6430_d_n10;
        locals.var_ps0_ini_dn11 = assign8480_e6430_d_n11;
        locals.var_ps0_ini_dn12 = assign8480_e6430_d_n12;
        locals.var_ps0_ini_dn17 = assign8480_e6430_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign8490_e6433: f64 = (locals.var_vgs - locals.var_shift);
        let assign8490_e6435: f64 = if assign8490_e6433 <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard164 = assign8490_e6435;
        locals.var_guard164_rv = 0.0;

        let (assign8500_e6449, assign8500_e6449_d_n0, assign8500_e6449_d_n2, assign8500_e6449_d_n6, assign8500_e6449_d_n7, assign8500_e6449_d_n10, assign8500_e6449_d_n11, assign8500_e6449_d_n12, assign8500_e6449_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8500_e6447: f64 = (1.0 / locals.var_c_fox);
        (assign8500_e6447, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign8500_e6449;
        locals.var_t0_dn0 = assign8500_e6449_d_n0;
        locals.var_t0_dn2 = assign8500_e6449_d_n2;
        locals.var_t0_dn6 = assign8500_e6449_d_n6;
        locals.var_t0_dn7 = assign8500_e6449_d_n7;
        locals.var_t0_dn10 = assign8500_e6449_d_n10;
        locals.var_t0_dn11 = assign8500_e6449_d_n11;
        locals.var_t0_dn12 = assign8500_e6449_d_n12;
        locals.var_t0_dn17 = assign8500_e6449_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign8510_e6463, assign8510_e6463_d_n0, assign8510_e6463_d_n2, assign8510_e6463_d_n6, assign8510_e6463_d_n7, assign8510_e6463_d_n10, assign8510_e6463_d_n11, assign8510_e6463_d_n12, assign8510_e6463_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8510_e6461: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign8510_e6461, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8510_e6463;
        locals.var_t1_dn0 = assign8510_e6463_d_n0;
        locals.var_t1_dn2 = assign8510_e6463_d_n2;
        locals.var_t1_dn6 = assign8510_e6463_d_n6;
        locals.var_t1_dn7 = assign8510_e6463_d_n7;
        locals.var_t1_dn10 = assign8510_e6463_d_n10;
        locals.var_t1_dn11 = assign8510_e6463_d_n11;
        locals.var_t1_dn12 = assign8510_e6463_d_n12;
        locals.var_t1_dn17 = assign8510_e6463_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8520_e6477, assign8520_e6477_d_n0, assign8520_e6477_d_n2, assign8520_e6477_d_n6, assign8520_e6477_d_n7, assign8520_e6477_d_n10, assign8520_e6477_d_n11, assign8520_e6477_d_n12, assign8520_e6477_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8520_e6475: f64 = (1.0 / locals.var_c_box);
        (assign8520_e6475, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8520_e6477;
        locals.var_t2_dn0 = assign8520_e6477_d_n0;
        locals.var_t2_dn2 = assign8520_e6477_d_n2;
        locals.var_t2_dn6 = assign8520_e6477_d_n6;
        locals.var_t2_dn7 = assign8520_e6477_d_n7;
        locals.var_t2_dn10 = assign8520_e6477_d_n10;
        locals.var_t2_dn11 = assign8520_e6477_d_n11;
        locals.var_t2_dn12 = assign8520_e6477_d_n12;
        locals.var_t2_dn17 = assign8520_e6477_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8530_e6495, assign8530_e6495_d_n0, assign8530_e6495_d_n2, assign8530_e6495_d_n6, assign8530_e6495_d_n7, assign8530_e6495_d_n10, assign8530_e6495_d_n11, assign8530_e6495_d_n12, assign8530_e6495_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8530_e6490: f64 = (locals.var_t0 + locals.var_t1);
        let assign8530_e6492: f64 = (assign8530_e6490 + locals.var_t2);
        let assign8530_e6493: f64 = (1.0 / assign8530_e6492);
        (assign8530_e6493, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign8530_e6492 * assign8530_e6492))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign8530_e6492 * assign8530_e6492))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign8530_e6492 * assign8530_e6492))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign8530_e6492 * assign8530_e6492))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign8530_e6492 * assign8530_e6492))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign8530_e6492 * assign8530_e6492))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign8530_e6492 * assign8530_e6492))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign8530_e6492 * assign8530_e6492))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8530_e6495;
        locals.var_t3_dn0 = assign8530_e6495_d_n0;
        locals.var_t3_dn2 = assign8530_e6495_d_n2;
        locals.var_t3_dn6 = assign8530_e6495_d_n6;
        locals.var_t3_dn7 = assign8530_e6495_d_n7;
        locals.var_t3_dn10 = assign8530_e6495_d_n10;
        locals.var_t3_dn11 = assign8530_e6495_d_n11;
        locals.var_t3_dn12 = assign8530_e6495_d_n12;
        locals.var_t3_dn17 = assign8530_e6495_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8540_e6520, assign8540_e6520_d_n0, assign8540_e6520_d_n2, assign8540_e6520_d_n6, assign8540_e6520_d_n7, assign8540_e6520_d_n10, assign8540_e6520_d_n11, assign8540_e6520_d_n12, assign8540_e6520_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8540_e6508: f64 = (locals.var_vgpz - locals.var_vbsbiz);
        let assign8540_e6512: f64 = (0.5 * locals.var_t1);
        let assign8540_e6513: f64 = (locals.var_t2 + assign8540_e6512);
        let assign8540_e6515: f64 = (-locals.var_q_s0_dep_ini);
        let assign8540_e6516: f64 = (assign8540_e6513 * assign8540_e6515);
        let assign8540_e6517: f64 = (assign8540_e6508 + assign8540_e6516);
        let assign8540_e6518: f64 = (locals.var_t3 * assign8540_e6517);
        (assign8540_e6518, ((locals.var_t3_dn0 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign8540_e6517) + (locals.var_t3 * ((locals.var_vgpz_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign8540_e6515) + (assign8540_e6513 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8540_e6520;
        locals.var_t4_dn0 = assign8540_e6520_d_n0;
        locals.var_t4_dn2 = assign8540_e6520_d_n2;
        locals.var_t4_dn6 = assign8540_e6520_d_n6;
        locals.var_t4_dn7 = assign8540_e6520_d_n7;
        locals.var_t4_dn10 = assign8540_e6520_d_n10;
        locals.var_t4_dn11 = assign8540_e6520_d_n11;
        locals.var_t4_dn12 = assign8540_e6520_d_n12;
        locals.var_t4_dn17 = assign8540_e6520_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8550_e6536, assign8550_e6536_d_n0, assign8550_e6536_d_n2, assign8550_e6536_d_n6, assign8550_e6536_d_n7, assign8550_e6536_d_n10, assign8550_e6536_d_n11, assign8550_e6536_d_n12, assign8550_e6536_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8550_e6533: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign8550_e6534: f64 = (locals.var_vgpz - assign8550_e6533);
        (assign8550_e6534, (locals.var_vgpz_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8550_e6536;
        locals.var_ps0_inia_dn0 = assign8550_e6536_d_n0;
        locals.var_ps0_inia_dn2 = assign8550_e6536_d_n2;
        locals.var_ps0_inia_dn6 = assign8550_e6536_d_n6;
        locals.var_ps0_inia_dn7 = assign8550_e6536_d_n7;
        locals.var_ps0_inia_dn10 = assign8550_e6536_d_n10;
        locals.var_ps0_inia_dn11 = assign8550_e6536_d_n11;
        locals.var_ps0_inia_dn12 = assign8550_e6536_d_n12;
        locals.var_ps0_inia_dn17 = assign8550_e6536_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8560_e6548, assign8560_e6548_d_n0, assign8560_e6548_d_n2, assign8560_e6548_d_n6, assign8560_e6548_d_n7, assign8560_e6548_d_n10, assign8560_e6548_d_n11, assign8560_e6548_d_n12, assign8560_e6548_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8560_e6548;
        locals.var_ps0_ini_dn0 = assign8560_e6548_d_n0;
        locals.var_ps0_ini_dn2 = assign8560_e6548_d_n2;
        locals.var_ps0_ini_dn6 = assign8560_e6548_d_n6;
        locals.var_ps0_ini_dn7 = assign8560_e6548_d_n7;
        locals.var_ps0_ini_dn10 = assign8560_e6548_d_n10;
        locals.var_ps0_ini_dn11 = assign8560_e6548_d_n11;
        locals.var_ps0_ini_dn12 = assign8560_e6548_d_n12;
        locals.var_ps0_ini_dn17 = assign8560_e6548_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign8570_e6565, assign8570_e6565_d_n0, assign8570_e6565_d_n2, assign8570_e6565_d_n6, assign8570_e6565_d_n7, assign8570_e6565_d_n10, assign8570_e6565_d_n11, assign8570_e6565_d_n12, assign8570_e6565_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8570_e6561: f64 = (1.0 / locals.var_cnst1soi);
        let assign8570_e6563: f64 = (assign8570_e6561 / locals.var_cnstc_foxi);
        (assign8570_e6563, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8570_e6561 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8570_e6565;
        locals.var_t1_dn0 = assign8570_e6565_d_n0;
        locals.var_t1_dn2 = assign8570_e6565_d_n2;
        locals.var_t1_dn6 = assign8570_e6565_d_n6;
        locals.var_t1_dn7 = assign8570_e6565_d_n7;
        locals.var_t1_dn10 = assign8570_e6565_d_n10;
        locals.var_t1_dn11 = assign8570_e6565_d_n11;
        locals.var_t1_dn12 = assign8570_e6565_d_n12;
        locals.var_t1_dn17 = assign8570_e6565_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8580_e6586, assign8580_e6586_d_n0, assign8580_e6586_d_n2, assign8580_e6586_d_n6, assign8580_e6586_d_n7, assign8580_e6586_d_n10, assign8580_e6586_d_n11, assign8580_e6586_d_n12, assign8580_e6586_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8580_e6579: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8580_e6580: f64 = (locals.var_t1 * assign8580_e6579);
        let assign8580_e6583: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8580_e6584: f64 = (assign8580_e6580 * assign8580_e6583);
        (assign8580_e6584, ((((locals.var_t1_dn0 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign8580_e6579) + (locals.var_t1 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))) * assign8580_e6583) + (assign8580_e6580 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8580_e6586;
        locals.var_t2_dn0 = assign8580_e6586_d_n0;
        locals.var_t2_dn2 = assign8580_e6586_d_n2;
        locals.var_t2_dn6 = assign8580_e6586_d_n6;
        locals.var_t2_dn7 = assign8580_e6586_d_n7;
        locals.var_t2_dn10 = assign8580_e6586_d_n10;
        locals.var_t2_dn11 = assign8580_e6586_d_n11;
        locals.var_t2_dn12 = assign8580_e6586_d_n12;
        locals.var_t2_dn17 = assign8580_e6586_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8590_e6605, assign8590_e6605_d_n0, assign8590_e6605_d_n2, assign8590_e6605_d_n6, assign8590_e6605_d_n7, assign8590_e6605_d_n10, assign8590_e6605_d_n11, assign8590_e6605_d_n12, assign8590_e6605_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8590_e6601: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8590_e6602: f64 = (2.0 / assign8590_e6601);
        let assign8590_e6603: f64 = (locals.var_beta + assign8590_e6602);
        (assign8590_e6603, (-((2.0 * (locals.var_vgpz_dn0 - locals.var_shift_dn0)) / (assign8590_e6601 * assign8590_e6601))), (-((2.0 * (locals.var_vgpz_dn2 - locals.var_shift_dn2)) / (assign8590_e6601 * assign8590_e6601))), (-((2.0 * (locals.var_vgpz_dn6 - locals.var_shift_dn6)) / (assign8590_e6601 * assign8590_e6601))), (-((2.0 * (locals.var_vgpz_dn7 - locals.var_shift_dn7)) / (assign8590_e6601 * assign8590_e6601))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgpz_dn10 - locals.var_shift_dn10)) / (assign8590_e6601 * assign8590_e6601)))), (-((2.0 * (locals.var_vgpz_dn11 - locals.var_shift_dn11)) / (assign8590_e6601 * assign8590_e6601))), (-((2.0 * (locals.var_vgpz_dn12 - locals.var_shift_dn12)) / (assign8590_e6601 * assign8590_e6601))), (-((2.0 * (locals.var_vgpz_dn17 - locals.var_shift_dn17)) / (assign8590_e6601 * assign8590_e6601))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8590_e6605;
        locals.var_t3_dn0 = assign8590_e6605_d_n0;
        locals.var_t3_dn2 = assign8590_e6605_d_n2;
        locals.var_t3_dn6 = assign8590_e6605_d_n6;
        locals.var_t3_dn7 = assign8590_e6605_d_n7;
        locals.var_t3_dn10 = assign8590_e6605_d_n10;
        locals.var_t3_dn11 = assign8590_e6605_d_n11;
        locals.var_t3_dn12 = assign8590_e6605_d_n12;
        locals.var_t3_dn17 = assign8590_e6605_d_n17;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign8600_e6621, assign8600_e6621_d_n0, assign8600_e6621_d_n2, assign8600_e6621_d_n6, assign8600_e6621_d_n7, assign8600_e6621_d_n10, assign8600_e6621_d_n11, assign8600_e6621_d_n12, assign8600_e6621_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8600_e6617: f64 = (locals.var_t2).ln();
        let assign8600_e6619: f64 = (assign8600_e6617 / locals.var_t3);
        (assign8600_e6619, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign8600_e6617 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign8600_e6621;
        locals.var_ps0_inib_dn0 = assign8600_e6621_d_n0;
        locals.var_ps0_inib_dn2 = assign8600_e6621_d_n2;
        locals.var_ps0_inib_dn6 = assign8600_e6621_d_n6;
        locals.var_ps0_inib_dn7 = assign8600_e6621_d_n7;
        locals.var_ps0_inib_dn10 = assign8600_e6621_d_n10;
        locals.var_ps0_inib_dn11 = assign8600_e6621_d_n11;
        locals.var_ps0_inib_dn12 = assign8600_e6621_d_n12;
        locals.var_ps0_inib_dn17 = assign8600_e6621_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign8610_e6638, assign8610_e6638_d_n0, assign8610_e6638_d_n2, assign8610_e6638_d_n6, assign8610_e6638_d_n7, assign8610_e6638_d_n10, assign8610_e6638_d_n11, assign8610_e6638_d_n12, assign8610_e6638_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8610_e6634: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign8610_e6636: f64 = (assign8610_e6634 - 0.0008);
        (assign8610_e6636, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign8610_e6638;
        locals.var_tmf1_dn0 = assign8610_e6638_d_n0;
        locals.var_tmf1_dn2 = assign8610_e6638_d_n2;
        locals.var_tmf1_dn6 = assign8610_e6638_d_n6;
        locals.var_tmf1_dn7 = assign8610_e6638_d_n7;
        locals.var_tmf1_dn10 = assign8610_e6638_d_n10;
        locals.var_tmf1_dn11 = assign8610_e6638_d_n11;
        locals.var_tmf1_dn12 = assign8610_e6638_d_n12;
        locals.var_tmf1_dn17 = assign8610_e6638_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign8620_e6655, assign8620_e6655_d_n0, assign8620_e6655_d_n2, assign8620_e6655_d_n6, assign8620_e6655_d_n7, assign8620_e6655_d_n10, assign8620_e6655_d_n11, assign8620_e6655_d_n12, assign8620_e6655_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8620_e6651: f64 = (4.0 * locals.var_ps0_inib);
        let assign8620_e6653: f64 = (assign8620_e6651 * 0.0008);
        (assign8620_e6653, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign8620_e6655;
        locals.var_tmf2_dn0 = assign8620_e6655_d_n0;
        locals.var_tmf2_dn2 = assign8620_e6655_d_n2;
        locals.var_tmf2_dn6 = assign8620_e6655_d_n6;
        locals.var_tmf2_dn7 = assign8620_e6655_d_n7;
        locals.var_tmf2_dn10 = assign8620_e6655_d_n10;
        locals.var_tmf2_dn11 = assign8620_e6655_d_n11;
        locals.var_tmf2_dn12 = assign8620_e6655_d_n12;
        locals.var_tmf2_dn17 = assign8620_e6655_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign8630_e6674, assign8630_e6674_d_n0, assign8630_e6674_d_n2, assign8630_e6674_d_n6, assign8630_e6674_d_n7, assign8630_e6674_d_n10, assign8630_e6674_d_n11, assign8630_e6674_d_n12, assign8630_e6674_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let (assign8630_e6672, assign8630_e6672_d_n0, assign8630_e6672_d_n2, assign8630_e6672_d_n6, assign8630_e6672_d_n7, assign8630_e6672_d_n10, assign8630_e6672_d_n11, assign8630_e6672_d_n12, assign8630_e6672_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign8630_e6671: f64 = (-locals.var_tmf2);
                (assign8630_e6671, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign8630_e6672, assign8630_e6672_d_n0, assign8630_e6672_d_n2, assign8630_e6672_d_n6, assign8630_e6672_d_n7, assign8630_e6672_d_n10, assign8630_e6672_d_n11, assign8630_e6672_d_n12, assign8630_e6672_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign8630_e6674;
        locals.var_tmf2_dn0 = assign8630_e6674_d_n0;
        locals.var_tmf2_dn2 = assign8630_e6674_d_n2;
        locals.var_tmf2_dn6 = assign8630_e6674_d_n6;
        locals.var_tmf2_dn7 = assign8630_e6674_d_n7;
        locals.var_tmf2_dn10 = assign8630_e6674_d_n10;
        locals.var_tmf2_dn11 = assign8630_e6674_d_n11;
        locals.var_tmf2_dn12 = assign8630_e6674_d_n12;
        locals.var_tmf2_dn17 = assign8630_e6674_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign8640_e6692, assign8640_e6692_d_n0, assign8640_e6692_d_n2, assign8640_e6692_d_n6, assign8640_e6692_d_n7, assign8640_e6692_d_n10, assign8640_e6692_d_n11, assign8640_e6692_d_n12, assign8640_e6692_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8640_e6687: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign8640_e6689: f64 = (assign8640_e6687 + locals.var_tmf2);
        let assign8640_e6690: f64 = (assign8640_e6689).sqrt();
        (assign8640_e6690, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign8640_e6690)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign8640_e6690)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign8640_e6690)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign8640_e6690)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign8640_e6690)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign8640_e6690)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign8640_e6690)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign8640_e6690)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign8640_e6692;
        locals.var_tmf2_dn0 = assign8640_e6692_d_n0;
        locals.var_tmf2_dn2 = assign8640_e6692_d_n2;
        locals.var_tmf2_dn6 = assign8640_e6692_d_n6;
        locals.var_tmf2_dn7 = assign8640_e6692_d_n7;
        locals.var_tmf2_dn10 = assign8640_e6692_d_n10;
        locals.var_tmf2_dn11 = assign8640_e6692_d_n11;
        locals.var_tmf2_dn12 = assign8640_e6692_d_n12;
        locals.var_tmf2_dn17 = assign8640_e6692_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign8650_e6711, assign8650_e6711_d_n0, assign8650_e6711_d_n2, assign8650_e6711_d_n6, assign8650_e6711_d_n7, assign8650_e6711_d_n10, assign8650_e6711_d_n11, assign8650_e6711_d_n12, assign8650_e6711_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 == 0.0)) && (locals.var_guard164 == 0.0)) {
        let assign8650_e6707: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign8650_e6708: f64 = (0.5 * assign8650_e6707);
        let assign8650_e6709: f64 = (locals.var_ps0_inib - assign8650_e6708);
        (assign8650_e6709, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8650_e6711;
        locals.var_ps0_ini_dn0 = assign8650_e6711_d_n0;
        locals.var_ps0_ini_dn2 = assign8650_e6711_d_n2;
        locals.var_ps0_ini_dn6 = assign8650_e6711_d_n6;
        locals.var_ps0_ini_dn7 = assign8650_e6711_d_n7;
        locals.var_ps0_ini_dn10 = assign8650_e6711_d_n10;
        locals.var_ps0_ini_dn11 = assign8650_e6711_d_n11;
        locals.var_ps0_ini_dn12 = assign8650_e6711_d_n12;
        locals.var_ps0_ini_dn17 = assign8650_e6711_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign8660_e6732, assign8660_e6732_d_n0, assign8660_e6732_d_n2, assign8660_e6732_d_n6, assign8660_e6732_d_n7, assign8660_e6732_d_n10, assign8660_e6732_d_n11, assign8660_e6732_d_n12, assign8660_e6732_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let (assign8660_e6730, assign8660_e6730_d_n0, assign8660_e6730_d_n2, assign8660_e6730_d_n6, assign8660_e6730_d_n7, assign8660_e6730_d_n10, assign8660_e6730_d_n11, assign8660_e6730_d_n12, assign8660_e6730_d_n17,) = {
            if (locals.var_ps0_ini > 0.0) {
                let assign8660_e6721: f64 = (2.0 * 1.034943e-10);
                let assign8660_e6723: f64 = (assign8660_e6721 / 1.6021918e-19);
                let assign8660_e6725: f64 = (assign8660_e6723 * locals.var_ps0_ini);
                let assign8660_e6727: f64 = (assign8660_e6725 / locals.var_uc_nsubs);
                let assign8660_e6728: f64 = (assign8660_e6727).sqrt();
                (assign8660_e6728, (((((assign8660_e6723 * locals.var_ps0_ini_dn0) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)), (((((assign8660_e6723 * locals.var_ps0_ini_dn2) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)), (((((assign8660_e6723 * locals.var_ps0_ini_dn6) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)), (((((assign8660_e6723 * locals.var_ps0_ini_dn7) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)), (((((assign8660_e6723 * locals.var_ps0_ini_dn10) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)), (((((assign8660_e6723 * locals.var_ps0_ini_dn11) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)), (((((assign8660_e6723 * locals.var_ps0_ini_dn12) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)), (((((assign8660_e6723 * locals.var_ps0_ini_dn17) * locals.var_uc_nsubs) - (assign8660_e6725 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8660_e6728)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign8660_e6730, assign8660_e6730_d_n0, assign8660_e6730_d_n2, assign8660_e6730_d_n6, assign8660_e6730_d_n7, assign8660_e6730_d_n10, assign8660_e6730_d_n11, assign8660_e6730_d_n12, assign8660_e6730_d_n17,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign8660_e6732;
        locals.var_wdsoi_dn0 = assign8660_e6732_d_n0;
        locals.var_wdsoi_dn2 = assign8660_e6732_d_n2;
        locals.var_wdsoi_dn6 = assign8660_e6732_d_n6;
        locals.var_wdsoi_dn7 = assign8660_e6732_d_n7;
        locals.var_wdsoi_dn10 = assign8660_e6732_d_n10;
        locals.var_wdsoi_dn11 = assign8660_e6732_d_n11;
        locals.var_wdsoi_dn12 = assign8660_e6732_d_n12;
        locals.var_wdsoi_dn17 = assign8660_e6732_d_n17;
        locals.var_wdsoi_rv = 0.0;

        let assign8670_e6735: f64 = if locals.var_wdsoi < locals.var_t_soi { 1.0 } else { 0.0 };
        locals.var_guard165 = assign8670_e6735;
        locals.var_guard165_rv = 0.0;

        let (assign8680_e6744,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard165 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign8680_e6744;
        locals.var_flg_depmode_rv = 0.0;

        let (assign8690_e6754,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard165 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign8690_e6754;
        locals.var_flg_depmode_rv = 0.0;

        let assign8700_e6757: f64 = (locals.var_vgs - locals.var_shift);
        let assign8700_e6759: f64 = if assign8700_e6757 <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard166 = assign8700_e6759;
        locals.var_guard166_rv = 0.0;

        let (assign8710_e6770, assign8710_e6770_d_n0, assign8710_e6770_d_n2, assign8710_e6770_d_n6, assign8710_e6770_d_n7, assign8710_e6770_d_n10, assign8710_e6770_d_n11, assign8710_e6770_d_n12, assign8710_e6770_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign8710_e6768: f64 = (1.0 / locals.var_c_fox);
        (assign8710_e6768, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign8710_e6770;
        locals.var_t0_dn0 = assign8710_e6770_d_n0;
        locals.var_t0_dn2 = assign8710_e6770_d_n2;
        locals.var_t0_dn6 = assign8710_e6770_d_n6;
        locals.var_t0_dn7 = assign8710_e6770_d_n7;
        locals.var_t0_dn10 = assign8710_e6770_d_n10;
        locals.var_t0_dn11 = assign8710_e6770_d_n11;
        locals.var_t0_dn12 = assign8710_e6770_d_n12;
        locals.var_t0_dn17 = assign8710_e6770_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign8720_e6781, assign8720_e6781_d_n0, assign8720_e6781_d_n2, assign8720_e6781_d_n6, assign8720_e6781_d_n7, assign8720_e6781_d_n10, assign8720_e6781_d_n11, assign8720_e6781_d_n12, assign8720_e6781_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign8720_e6779: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign8720_e6779, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8720_e6781;
        locals.var_t1_dn0 = assign8720_e6781_d_n0;
        locals.var_t1_dn2 = assign8720_e6781_d_n2;
        locals.var_t1_dn6 = assign8720_e6781_d_n6;
        locals.var_t1_dn7 = assign8720_e6781_d_n7;
        locals.var_t1_dn10 = assign8720_e6781_d_n10;
        locals.var_t1_dn11 = assign8720_e6781_d_n11;
        locals.var_t1_dn12 = assign8720_e6781_d_n12;
        locals.var_t1_dn17 = assign8720_e6781_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8730_e6792, assign8730_e6792_d_n0, assign8730_e6792_d_n2, assign8730_e6792_d_n6, assign8730_e6792_d_n7, assign8730_e6792_d_n10, assign8730_e6792_d_n11, assign8730_e6792_d_n12, assign8730_e6792_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign8730_e6790: f64 = (1.0 / locals.var_c_box);
        (assign8730_e6790, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8730_e6792;
        locals.var_t2_dn0 = assign8730_e6792_d_n0;
        locals.var_t2_dn2 = assign8730_e6792_d_n2;
        locals.var_t2_dn6 = assign8730_e6792_d_n6;
        locals.var_t2_dn7 = assign8730_e6792_d_n7;
        locals.var_t2_dn10 = assign8730_e6792_d_n10;
        locals.var_t2_dn11 = assign8730_e6792_d_n11;
        locals.var_t2_dn12 = assign8730_e6792_d_n12;
        locals.var_t2_dn17 = assign8730_e6792_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8740_e6807, assign8740_e6807_d_n0, assign8740_e6807_d_n2, assign8740_e6807_d_n6, assign8740_e6807_d_n7, assign8740_e6807_d_n10, assign8740_e6807_d_n11, assign8740_e6807_d_n12, assign8740_e6807_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign8740_e6802: f64 = (locals.var_t0 + locals.var_t1);
        let assign8740_e6804: f64 = (assign8740_e6802 + locals.var_t2);
        let assign8740_e6805: f64 = (1.0 / assign8740_e6804);
        (assign8740_e6805, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign8740_e6804 * assign8740_e6804))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign8740_e6804 * assign8740_e6804))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign8740_e6804 * assign8740_e6804))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign8740_e6804 * assign8740_e6804))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign8740_e6804 * assign8740_e6804))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign8740_e6804 * assign8740_e6804))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign8740_e6804 * assign8740_e6804))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign8740_e6804 * assign8740_e6804))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8740_e6807;
        locals.var_t3_dn0 = assign8740_e6807_d_n0;
        locals.var_t3_dn2 = assign8740_e6807_d_n2;
        locals.var_t3_dn6 = assign8740_e6807_d_n6;
        locals.var_t3_dn7 = assign8740_e6807_d_n7;
        locals.var_t3_dn10 = assign8740_e6807_d_n10;
        locals.var_t3_dn11 = assign8740_e6807_d_n11;
        locals.var_t3_dn12 = assign8740_e6807_d_n12;
        locals.var_t3_dn17 = assign8740_e6807_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8750_e6829, assign8750_e6829_d_n0, assign8750_e6829_d_n2, assign8750_e6829_d_n6, assign8750_e6829_d_n7, assign8750_e6829_d_n10, assign8750_e6829_d_n11, assign8750_e6829_d_n12, assign8750_e6829_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign8750_e6817: f64 = (locals.var_vgpz - locals.var_vbsbiz);
        let assign8750_e6821: f64 = (0.5 * locals.var_t1);
        let assign8750_e6822: f64 = (locals.var_t2 + assign8750_e6821);
        let assign8750_e6824: f64 = (-locals.var_q_s0_dep_ini);
        let assign8750_e6825: f64 = (assign8750_e6822 * assign8750_e6824);
        let assign8750_e6826: f64 = (assign8750_e6817 + assign8750_e6825);
        let assign8750_e6827: f64 = (locals.var_t3 * assign8750_e6826);
        (assign8750_e6827, ((locals.var_t3_dn0 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign8750_e6826) + (locals.var_t3 * ((locals.var_vgpz_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign8750_e6824) + (assign8750_e6822 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8750_e6829;
        locals.var_t4_dn0 = assign8750_e6829_d_n0;
        locals.var_t4_dn2 = assign8750_e6829_d_n2;
        locals.var_t4_dn6 = assign8750_e6829_d_n6;
        locals.var_t4_dn7 = assign8750_e6829_d_n7;
        locals.var_t4_dn10 = assign8750_e6829_d_n10;
        locals.var_t4_dn11 = assign8750_e6829_d_n11;
        locals.var_t4_dn12 = assign8750_e6829_d_n12;
        locals.var_t4_dn17 = assign8750_e6829_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8760_e6842, assign8760_e6842_d_n0, assign8760_e6842_d_n2, assign8760_e6842_d_n6, assign8760_e6842_d_n7, assign8760_e6842_d_n10, assign8760_e6842_d_n11, assign8760_e6842_d_n12, assign8760_e6842_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 != 0.0)) {
        let assign8760_e6839: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign8760_e6840: f64 = (locals.var_vgpz - assign8760_e6839);
        (assign8760_e6840, (locals.var_vgpz_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8760_e6842;
        locals.var_ps0_inia_dn0 = assign8760_e6842_d_n0;
        locals.var_ps0_inia_dn2 = assign8760_e6842_d_n2;
        locals.var_ps0_inia_dn6 = assign8760_e6842_d_n6;
        locals.var_ps0_inia_dn7 = assign8760_e6842_d_n7;
        locals.var_ps0_inia_dn10 = assign8760_e6842_d_n10;
        locals.var_ps0_inia_dn11 = assign8760_e6842_d_n11;
        locals.var_ps0_inia_dn12 = assign8760_e6842_d_n12;
        locals.var_ps0_inia_dn17 = assign8760_e6842_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8770_e6851, assign8770_e6851_d_n0, assign8770_e6851_d_n2, assign8770_e6851_d_n6, assign8770_e6851_d_n7, assign8770_e6851_d_n10, assign8770_e6851_d_n11, assign8770_e6851_d_n12, assign8770_e6851_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8770_e6851;
        locals.var_ps0_ini_dn0 = assign8770_e6851_d_n0;
        locals.var_ps0_ini_dn2 = assign8770_e6851_d_n2;
        locals.var_ps0_ini_dn6 = assign8770_e6851_d_n6;
        locals.var_ps0_ini_dn7 = assign8770_e6851_d_n7;
        locals.var_ps0_ini_dn10 = assign8770_e6851_d_n10;
        locals.var_ps0_ini_dn11 = assign8770_e6851_d_n11;
        locals.var_ps0_ini_dn12 = assign8770_e6851_d_n12;
        locals.var_ps0_ini_dn17 = assign8770_e6851_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign8780_e6863, assign8780_e6863_d_n0, assign8780_e6863_d_n2, assign8780_e6863_d_n6, assign8780_e6863_d_n7, assign8780_e6863_d_n10, assign8780_e6863_d_n11, assign8780_e6863_d_n12, assign8780_e6863_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign8780_e6861: f64 = (1.0 / locals.var_c_fox);
        (assign8780_e6861, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign8780_e6863;
        locals.var_t0_dn0 = assign8780_e6863_d_n0;
        locals.var_t0_dn2 = assign8780_e6863_d_n2;
        locals.var_t0_dn6 = assign8780_e6863_d_n6;
        locals.var_t0_dn7 = assign8780_e6863_d_n7;
        locals.var_t0_dn10 = assign8780_e6863_d_n10;
        locals.var_t0_dn11 = assign8780_e6863_d_n11;
        locals.var_t0_dn12 = assign8780_e6863_d_n12;
        locals.var_t0_dn17 = assign8780_e6863_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign8790_e6875, assign8790_e6875_d_n0, assign8790_e6875_d_n2, assign8790_e6875_d_n6, assign8790_e6875_d_n7, assign8790_e6875_d_n10, assign8790_e6875_d_n11, assign8790_e6875_d_n12, assign8790_e6875_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign8790_e6873: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign8790_e6873, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8790_e6875;
        locals.var_t1_dn0 = assign8790_e6875_d_n0;
        locals.var_t1_dn2 = assign8790_e6875_d_n2;
        locals.var_t1_dn6 = assign8790_e6875_d_n6;
        locals.var_t1_dn7 = assign8790_e6875_d_n7;
        locals.var_t1_dn10 = assign8790_e6875_d_n10;
        locals.var_t1_dn11 = assign8790_e6875_d_n11;
        locals.var_t1_dn12 = assign8790_e6875_d_n12;
        locals.var_t1_dn17 = assign8790_e6875_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8800_e6887, assign8800_e6887_d_n0, assign8800_e6887_d_n2, assign8800_e6887_d_n6, assign8800_e6887_d_n7, assign8800_e6887_d_n10, assign8800_e6887_d_n11, assign8800_e6887_d_n12, assign8800_e6887_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign8800_e6885: f64 = (1.0 / locals.var_c_box);
        (assign8800_e6885, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8800_e6887;
        locals.var_t2_dn0 = assign8800_e6887_d_n0;
        locals.var_t2_dn2 = assign8800_e6887_d_n2;
        locals.var_t2_dn6 = assign8800_e6887_d_n6;
        locals.var_t2_dn7 = assign8800_e6887_d_n7;
        locals.var_t2_dn10 = assign8800_e6887_d_n10;
        locals.var_t2_dn11 = assign8800_e6887_d_n11;
        locals.var_t2_dn12 = assign8800_e6887_d_n12;
        locals.var_t2_dn17 = assign8800_e6887_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8810_e6903, assign8810_e6903_d_n0, assign8810_e6903_d_n2, assign8810_e6903_d_n6, assign8810_e6903_d_n7, assign8810_e6903_d_n10, assign8810_e6903_d_n11, assign8810_e6903_d_n12, assign8810_e6903_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign8810_e6898: f64 = (locals.var_t0 + locals.var_t1);
        let assign8810_e6900: f64 = (assign8810_e6898 + locals.var_t2);
        let assign8810_e6901: f64 = (1.0 / assign8810_e6900);
        (assign8810_e6901, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign8810_e6900 * assign8810_e6900))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign8810_e6900 * assign8810_e6900))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign8810_e6900 * assign8810_e6900))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign8810_e6900 * assign8810_e6900))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign8810_e6900 * assign8810_e6900))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign8810_e6900 * assign8810_e6900))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign8810_e6900 * assign8810_e6900))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign8810_e6900 * assign8810_e6900))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8810_e6903;
        locals.var_t3_dn0 = assign8810_e6903_d_n0;
        locals.var_t3_dn2 = assign8810_e6903_d_n2;
        locals.var_t3_dn6 = assign8810_e6903_d_n6;
        locals.var_t3_dn7 = assign8810_e6903_d_n7;
        locals.var_t3_dn10 = assign8810_e6903_d_n10;
        locals.var_t3_dn11 = assign8810_e6903_d_n11;
        locals.var_t3_dn12 = assign8810_e6903_d_n12;
        locals.var_t3_dn17 = assign8810_e6903_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8820_e6926, assign8820_e6926_d_n0, assign8820_e6926_d_n2, assign8820_e6926_d_n6, assign8820_e6926_d_n7, assign8820_e6926_d_n10, assign8820_e6926_d_n11, assign8820_e6926_d_n12, assign8820_e6926_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign8820_e6914: f64 = (locals.var_vgpz - locals.var_vbsbiz);
        let assign8820_e6918: f64 = (0.5 * locals.var_t1);
        let assign8820_e6919: f64 = (locals.var_t2 + assign8820_e6918);
        let assign8820_e6921: f64 = (-locals.var_q_s0_dep_ini);
        let assign8820_e6922: f64 = (assign8820_e6919 * assign8820_e6921);
        let assign8820_e6923: f64 = (assign8820_e6914 + assign8820_e6922);
        let assign8820_e6924: f64 = (locals.var_t3 * assign8820_e6923);
        (assign8820_e6924, ((locals.var_t3_dn0 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign8820_e6923) + (locals.var_t3 * ((locals.var_vgpz_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign8820_e6921) + (assign8820_e6919 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8820_e6926;
        locals.var_t4_dn0 = assign8820_e6926_d_n0;
        locals.var_t4_dn2 = assign8820_e6926_d_n2;
        locals.var_t4_dn6 = assign8820_e6926_d_n6;
        locals.var_t4_dn7 = assign8820_e6926_d_n7;
        locals.var_t4_dn10 = assign8820_e6926_d_n10;
        locals.var_t4_dn11 = assign8820_e6926_d_n11;
        locals.var_t4_dn12 = assign8820_e6926_d_n12;
        locals.var_t4_dn17 = assign8820_e6926_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8830_e6940, assign8830_e6940_d_n0, assign8830_e6940_d_n2, assign8830_e6940_d_n6, assign8830_e6940_d_n7, assign8830_e6940_d_n10, assign8830_e6940_d_n11, assign8830_e6940_d_n12, assign8830_e6940_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) {
        let assign8830_e6937: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign8830_e6938: f64 = (locals.var_vgpz - assign8830_e6937);
        (assign8830_e6938, (locals.var_vgpz_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8830_e6940;
        locals.var_ps0_inia_dn0 = assign8830_e6940_d_n0;
        locals.var_ps0_inia_dn2 = assign8830_e6940_d_n2;
        locals.var_ps0_inia_dn6 = assign8830_e6940_d_n6;
        locals.var_ps0_inia_dn7 = assign8830_e6940_d_n7;
        locals.var_ps0_inia_dn10 = assign8830_e6940_d_n10;
        locals.var_ps0_inia_dn11 = assign8830_e6940_d_n11;
        locals.var_ps0_inia_dn12 = assign8830_e6940_d_n12;
        locals.var_ps0_inia_dn17 = assign8830_e6940_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8840_e6950, assign8840_e6950_d_n0, assign8840_e6950_d_n2, assign8840_e6950_d_n6, assign8840_e6950_d_n7, assign8840_e6950_d_n10, assign8840_e6950_d_n11, assign8840_e6950_d_n12, assign8840_e6950_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8840_e6950;
        locals.var_ps0_ini_dn0 = assign8840_e6950_d_n0;
        locals.var_ps0_ini_dn2 = assign8840_e6950_d_n2;
        locals.var_ps0_ini_dn6 = assign8840_e6950_d_n6;
        locals.var_ps0_ini_dn7 = assign8840_e6950_d_n7;
        locals.var_ps0_ini_dn10 = assign8840_e6950_d_n10;
        locals.var_ps0_ini_dn11 = assign8840_e6950_d_n11;
        locals.var_ps0_ini_dn12 = assign8840_e6950_d_n12;
        locals.var_ps0_ini_dn17 = assign8840_e6950_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign8850_e6953: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8850_e6955: f64 = if assign8850_e6953 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign8850_e6955;
        locals.var_guard167_rv = 0.0;

        let (assign8860_e6971, assign8860_e6971_d_n0, assign8860_e6971_d_n2, assign8860_e6971_d_n6, assign8860_e6971_d_n7, assign8860_e6971_d_n10, assign8860_e6971_d_n11, assign8860_e6971_d_n12, assign8860_e6971_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) {
        let assign8860_e6967: f64 = (1.0 / locals.var_cnst1soi);
        let assign8860_e6969: f64 = (assign8860_e6967 / locals.var_cnstc_foxi);
        (assign8860_e6969, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8860_e6967 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8860_e6971;
        locals.var_t1_dn0 = assign8860_e6971_d_n0;
        locals.var_t1_dn2 = assign8860_e6971_d_n2;
        locals.var_t1_dn6 = assign8860_e6971_d_n6;
        locals.var_t1_dn7 = assign8860_e6971_d_n7;
        locals.var_t1_dn10 = assign8860_e6971_d_n10;
        locals.var_t1_dn11 = assign8860_e6971_d_n11;
        locals.var_t1_dn12 = assign8860_e6971_d_n12;
        locals.var_t1_dn17 = assign8860_e6971_d_n17;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign8870_e6991, assign8870_e6991_d_n0, assign8870_e6991_d_n2, assign8870_e6991_d_n6, assign8870_e6991_d_n7, assign8870_e6991_d_n10, assign8870_e6991_d_n11, assign8870_e6991_d_n12, assign8870_e6991_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) {
        let assign8870_e6984: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8870_e6985: f64 = (locals.var_t1 * assign8870_e6984);
        let assign8870_e6988: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8870_e6989: f64 = (assign8870_e6985 * assign8870_e6988);
        (assign8870_e6989, ((((locals.var_t1_dn0 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign8870_e6984) + (locals.var_t1 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))) * assign8870_e6988) + (assign8870_e6985 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8870_e6991;
        locals.var_t2_dn0 = assign8870_e6991_d_n0;
        locals.var_t2_dn2 = assign8870_e6991_d_n2;
        locals.var_t2_dn6 = assign8870_e6991_d_n6;
        locals.var_t2_dn7 = assign8870_e6991_d_n7;
        locals.var_t2_dn10 = assign8870_e6991_d_n10;
        locals.var_t2_dn11 = assign8870_e6991_d_n11;
        locals.var_t2_dn12 = assign8870_e6991_d_n12;
        locals.var_t2_dn17 = assign8870_e6991_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8880_e7009, assign8880_e7009_d_n0, assign8880_e7009_d_n2, assign8880_e7009_d_n6, assign8880_e7009_d_n7, assign8880_e7009_d_n10, assign8880_e7009_d_n11, assign8880_e7009_d_n12, assign8880_e7009_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) {
        let assign8880_e7005: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8880_e7006: f64 = (2.0 / assign8880_e7005);
        let assign8880_e7007: f64 = (locals.var_beta + assign8880_e7006);
        (assign8880_e7007, (-((2.0 * (locals.var_vgpz_dn0 - locals.var_shift_dn0)) / (assign8880_e7005 * assign8880_e7005))), (-((2.0 * (locals.var_vgpz_dn2 - locals.var_shift_dn2)) / (assign8880_e7005 * assign8880_e7005))), (-((2.0 * (locals.var_vgpz_dn6 - locals.var_shift_dn6)) / (assign8880_e7005 * assign8880_e7005))), (-((2.0 * (locals.var_vgpz_dn7 - locals.var_shift_dn7)) / (assign8880_e7005 * assign8880_e7005))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgpz_dn10 - locals.var_shift_dn10)) / (assign8880_e7005 * assign8880_e7005)))), (-((2.0 * (locals.var_vgpz_dn11 - locals.var_shift_dn11)) / (assign8880_e7005 * assign8880_e7005))), (-((2.0 * (locals.var_vgpz_dn12 - locals.var_shift_dn12)) / (assign8880_e7005 * assign8880_e7005))), (-((2.0 * (locals.var_vgpz_dn17 - locals.var_shift_dn17)) / (assign8880_e7005 * assign8880_e7005))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8880_e7009;
        locals.var_t3_dn0 = assign8880_e7009_d_n0;
        locals.var_t3_dn2 = assign8880_e7009_d_n2;
        locals.var_t3_dn6 = assign8880_e7009_d_n6;
        locals.var_t3_dn7 = assign8880_e7009_d_n7;
        locals.var_t3_dn10 = assign8880_e7009_d_n10;
        locals.var_t3_dn11 = assign8880_e7009_d_n11;
        locals.var_t3_dn12 = assign8880_e7009_d_n12;
        locals.var_t3_dn17 = assign8880_e7009_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8890_e7024, assign8890_e7024_d_n0, assign8890_e7024_d_n2, assign8890_e7024_d_n6, assign8890_e7024_d_n7, assign8890_e7024_d_n10, assign8890_e7024_d_n11, assign8890_e7024_d_n12, assign8890_e7024_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) {
        let assign8890_e7020: f64 = (locals.var_t2).ln();
        let assign8890_e7022: f64 = (assign8890_e7020 / locals.var_t3);
        (assign8890_e7022, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign8890_e7020 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign8890_e7024;
        locals.var_ps0_inib_dn0 = assign8890_e7024_d_n0;
        locals.var_ps0_inib_dn2 = assign8890_e7024_d_n2;
        locals.var_ps0_inib_dn6 = assign8890_e7024_d_n6;
        locals.var_ps0_inib_dn7 = assign8890_e7024_d_n7;
        locals.var_ps0_inib_dn10 = assign8890_e7024_d_n10;
        locals.var_ps0_inib_dn11 = assign8890_e7024_d_n11;
        locals.var_ps0_inib_dn12 = assign8890_e7024_d_n12;
        locals.var_ps0_inib_dn17 = assign8890_e7024_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let assign8900_e7028: f64 = (locals.var_ps0_inib * 0.98);
        let assign8900_e7030: f64 = (assign8900_e7028 - 0.4);
        let assign8900_e7035: f64 = if ((locals.var_ps0_inia > assign8900_e7030) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard168 = assign8900_e7035;
        locals.var_guard168_rv = 0.0;

        let (assign8910_e7055, assign8910_e7055_d_n0, assign8910_e7055_d_n2, assign8910_e7055_d_n6, assign8910_e7055_d_n7, assign8910_e7055_d_n10, assign8910_e7055_d_n11, assign8910_e7055_d_n12, assign8910_e7055_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign8910_e7050: f64 = (locals.var_ps0_inib * 0.98);
        let assign8910_e7051: f64 = (locals.var_ps0_inia - assign8910_e7050);
        let assign8910_e7053: f64 = (assign8910_e7051 + 0.4);
        (assign8910_e7053, (locals.var_ps0_inia_dn0 - (locals.var_ps0_inib_dn0 * 0.98)), (locals.var_ps0_inia_dn2 - (locals.var_ps0_inib_dn2 * 0.98)), (locals.var_ps0_inia_dn6 - (locals.var_ps0_inib_dn6 * 0.98)), (locals.var_ps0_inia_dn7 - (locals.var_ps0_inib_dn7 * 0.98)), (locals.var_ps0_inia_dn10 - (locals.var_ps0_inib_dn10 * 0.98)), (locals.var_ps0_inia_dn11 - (locals.var_ps0_inib_dn11 * 0.98)), (locals.var_ps0_inia_dn12 - (locals.var_ps0_inib_dn12 * 0.98)), (locals.var_ps0_inia_dn17 - (locals.var_ps0_inib_dn17 * 0.98)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign8910_e7055;
        locals.var_tmf1_dn0 = assign8910_e7055_d_n0;
        locals.var_tmf1_dn2 = assign8910_e7055_d_n2;
        locals.var_tmf1_dn6 = assign8910_e7055_d_n6;
        locals.var_tmf1_dn7 = assign8910_e7055_d_n7;
        locals.var_tmf1_dn10 = assign8910_e7055_d_n10;
        locals.var_tmf1_dn11 = assign8910_e7055_d_n11;
        locals.var_tmf1_dn12 = assign8910_e7055_d_n12;
        locals.var_tmf1_dn17 = assign8910_e7055_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign8920_e7071, assign8920_e7071_d_n0, assign8920_e7071_d_n2, assign8920_e7071_d_n6, assign8920_e7071_d_n7, assign8920_e7071_d_n10, assign8920_e7071_d_n11, assign8920_e7071_d_n12, assign8920_e7071_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign8920_e7069: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign8920_e7069, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign8920_e7071;
        locals.var_x2_dn0 = assign8920_e7071_d_n0;
        locals.var_x2_dn2 = assign8920_e7071_d_n2;
        locals.var_x2_dn6 = assign8920_e7071_d_n6;
        locals.var_x2_dn7 = assign8920_e7071_d_n7;
        locals.var_x2_dn10 = assign8920_e7071_d_n10;
        locals.var_x2_dn11 = assign8920_e7071_d_n11;
        locals.var_x2_dn12 = assign8920_e7071_d_n12;
        locals.var_x2_dn17 = assign8920_e7071_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign8930_e7087, assign8930_e7087_d_n0, assign8930_e7087_d_n2, assign8930_e7087_d_n6, assign8930_e7087_d_n7, assign8930_e7087_d_n10, assign8930_e7087_d_n11, assign8930_e7087_d_n12, assign8930_e7087_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign8930_e7085: f64 = (0.4 * 0.4);
        (assign8930_e7085, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign8930_e7087;
        locals.var_xmax2_dn0 = assign8930_e7087_d_n0;
        locals.var_xmax2_dn2 = assign8930_e7087_d_n2;
        locals.var_xmax2_dn6 = assign8930_e7087_d_n6;
        locals.var_xmax2_dn7 = assign8930_e7087_d_n7;
        locals.var_xmax2_dn10 = assign8930_e7087_d_n10;
        locals.var_xmax2_dn11 = assign8930_e7087_d_n11;
        locals.var_xmax2_dn12 = assign8930_e7087_d_n12;
        locals.var_xmax2_dn17 = assign8930_e7087_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign8940_e7101, assign8940_e7101_d_n0, assign8940_e7101_d_n2, assign8940_e7101_d_n6, assign8940_e7101_d_n7, assign8940_e7101_d_n10, assign8940_e7101_d_n11, assign8940_e7101_d_n12, assign8940_e7101_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8940_e7101;
        locals.var_xp_dn0 = assign8940_e7101_d_n0;
        locals.var_xp_dn2 = assign8940_e7101_d_n2;
        locals.var_xp_dn6 = assign8940_e7101_d_n6;
        locals.var_xp_dn7 = assign8940_e7101_d_n7;
        locals.var_xp_dn10 = assign8940_e7101_d_n10;
        locals.var_xp_dn11 = assign8940_e7101_d_n11;
        locals.var_xp_dn12 = assign8940_e7101_d_n12;
        locals.var_xp_dn17 = assign8940_e7101_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8950_e7115, assign8950_e7115_d_n0, assign8950_e7115_d_n2, assign8950_e7115_d_n6, assign8950_e7115_d_n7, assign8950_e7115_d_n10, assign8950_e7115_d_n11, assign8950_e7115_d_n12, assign8950_e7115_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8950_e7115;
        locals.var_xmp_dn0 = assign8950_e7115_d_n0;
        locals.var_xmp_dn2 = assign8950_e7115_d_n2;
        locals.var_xmp_dn6 = assign8950_e7115_d_n6;
        locals.var_xmp_dn7 = assign8950_e7115_d_n7;
        locals.var_xmp_dn10 = assign8950_e7115_d_n10;
        locals.var_xmp_dn11 = assign8950_e7115_d_n11;
        locals.var_xmp_dn12 = assign8950_e7115_d_n12;
        locals.var_xmp_dn17 = assign8950_e7115_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8960_e7129,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign8960_e7129;
        locals.var_m0_rv = 0.0;

        let (assign8970_e7143,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8970_e7143;
        locals.var_mm_rv = 0.0;

        let (assign8980_e7157, assign8980_e7157_d_n0, assign8980_e7157_d_n2, assign8980_e7157_d_n6, assign8980_e7157_d_n7, assign8980_e7157_d_n10, assign8980_e7157_d_n11, assign8980_e7157_d_n12, assign8980_e7157_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign8980_e7157;
        locals.var_arg_dn0 = assign8980_e7157_d_n0;
        locals.var_arg_dn2 = assign8980_e7157_d_n2;
        locals.var_arg_dn6 = assign8980_e7157_d_n6;
        locals.var_arg_dn7 = assign8980_e7157_d_n7;
        locals.var_arg_dn10 = assign8980_e7157_d_n10;
        locals.var_arg_dn11 = assign8980_e7157_d_n11;
        locals.var_arg_dn12 = assign8980_e7157_d_n12;
        locals.var_arg_dn17 = assign8980_e7157_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign8990_e7171, assign8990_e7171_d_n0, assign8990_e7171_d_n2, assign8990_e7171_d_n6, assign8990_e7171_d_n7, assign8990_e7171_d_n10, assign8990_e7171_d_n11, assign8990_e7171_d_n12, assign8990_e7171_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8990_e7171;
        locals.var_dnm_dn0 = assign8990_e7171_d_n0;
        locals.var_dnm_dn2 = assign8990_e7171_d_n2;
        locals.var_dnm_dn6 = assign8990_e7171_d_n6;
        locals.var_dnm_dn7 = assign8990_e7171_d_n7;
        locals.var_dnm_dn10 = assign8990_e7171_d_n10;
        locals.var_dnm_dn11 = assign8990_e7171_d_n11;
        locals.var_dnm_dn12 = assign8990_e7171_d_n12;
        locals.var_dnm_dn17 = assign8990_e7171_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign9000_e7187, assign9000_e7187_d_n0, assign9000_e7187_d_n2, assign9000_e7187_d_n6, assign9000_e7187_d_n7, assign9000_e7187_d_n10, assign9000_e7187_d_n11, assign9000_e7187_d_n12, assign9000_e7187_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9000_e7185: f64 = (locals.var_xp * locals.var_x2);
        (assign9000_e7185, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign9000_e7187;
        locals.var_xp_dn0 = assign9000_e7187_d_n0;
        locals.var_xp_dn2 = assign9000_e7187_d_n2;
        locals.var_xp_dn6 = assign9000_e7187_d_n6;
        locals.var_xp_dn7 = assign9000_e7187_d_n7;
        locals.var_xp_dn10 = assign9000_e7187_d_n10;
        locals.var_xp_dn11 = assign9000_e7187_d_n11;
        locals.var_xp_dn12 = assign9000_e7187_d_n12;
        locals.var_xp_dn17 = assign9000_e7187_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign9010_e7203, assign9010_e7203_d_n0, assign9010_e7203_d_n2, assign9010_e7203_d_n6, assign9010_e7203_d_n7, assign9010_e7203_d_n10, assign9010_e7203_d_n11, assign9010_e7203_d_n12, assign9010_e7203_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9010_e7201: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign9010_e7201, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign9010_e7203;
        locals.var_xmp_dn0 = assign9010_e7203_d_n0;
        locals.var_xmp_dn2 = assign9010_e7203_d_n2;
        locals.var_xmp_dn6 = assign9010_e7203_d_n6;
        locals.var_xmp_dn7 = assign9010_e7203_d_n7;
        locals.var_xmp_dn10 = assign9010_e7203_d_n10;
        locals.var_xmp_dn11 = assign9010_e7203_d_n11;
        locals.var_xmp_dn12 = assign9010_e7203_d_n12;
        locals.var_xmp_dn17 = assign9010_e7203_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign9020_e7219, assign9020_e7219_d_n0, assign9020_e7219_d_n2, assign9020_e7219_d_n6, assign9020_e7219_d_n7, assign9020_e7219_d_n10, assign9020_e7219_d_n11, assign9020_e7219_d_n12, assign9020_e7219_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9020_e7217: f64 = (locals.var_xp * locals.var_x2);
        (assign9020_e7217, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign9020_e7219;
        locals.var_xp_dn0 = assign9020_e7219_d_n0;
        locals.var_xp_dn2 = assign9020_e7219_d_n2;
        locals.var_xp_dn6 = assign9020_e7219_d_n6;
        locals.var_xp_dn7 = assign9020_e7219_d_n7;
        locals.var_xp_dn10 = assign9020_e7219_d_n10;
        locals.var_xp_dn11 = assign9020_e7219_d_n11;
        locals.var_xp_dn12 = assign9020_e7219_d_n12;
        locals.var_xp_dn17 = assign9020_e7219_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign9030_e7235, assign9030_e7235_d_n0, assign9030_e7235_d_n2, assign9030_e7235_d_n6, assign9030_e7235_d_n7, assign9030_e7235_d_n10, assign9030_e7235_d_n11, assign9030_e7235_d_n12, assign9030_e7235_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9030_e7233: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign9030_e7233, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign9030_e7235;
        locals.var_xmp_dn0 = assign9030_e7235_d_n0;
        locals.var_xmp_dn2 = assign9030_e7235_d_n2;
        locals.var_xmp_dn6 = assign9030_e7235_d_n6;
        locals.var_xmp_dn7 = assign9030_e7235_d_n7;
        locals.var_xmp_dn10 = assign9030_e7235_d_n10;
        locals.var_xmp_dn11 = assign9030_e7235_d_n11;
        locals.var_xmp_dn12 = assign9030_e7235_d_n12;
        locals.var_xmp_dn17 = assign9030_e7235_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign9040_e7251, assign9040_e7251_d_n0, assign9040_e7251_d_n2, assign9040_e7251_d_n6, assign9040_e7251_d_n7, assign9040_e7251_d_n10, assign9040_e7251_d_n11, assign9040_e7251_d_n12, assign9040_e7251_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9040_e7249: f64 = (locals.var_xp + locals.var_xmp);
        (assign9040_e7249, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign9040_e7251;
        locals.var_arg_dn0 = assign9040_e7251_d_n0;
        locals.var_arg_dn2 = assign9040_e7251_d_n2;
        locals.var_arg_dn6 = assign9040_e7251_d_n6;
        locals.var_arg_dn7 = assign9040_e7251_d_n7;
        locals.var_arg_dn10 = assign9040_e7251_d_n10;
        locals.var_arg_dn11 = assign9040_e7251_d_n11;
        locals.var_arg_dn12 = assign9040_e7251_d_n12;
        locals.var_arg_dn17 = assign9040_e7251_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign9050_e7265, assign9050_e7265_d_n0, assign9050_e7265_d_n2, assign9050_e7265_d_n6, assign9050_e7265_d_n7, assign9050_e7265_d_n10, assign9050_e7265_d_n11, assign9050_e7265_d_n12, assign9050_e7265_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign9050_e7265;
        locals.var_dnm_dn0 = assign9050_e7265_d_n0;
        locals.var_dnm_dn2 = assign9050_e7265_d_n2;
        locals.var_dnm_dn6 = assign9050_e7265_d_n6;
        locals.var_dnm_dn7 = assign9050_e7265_d_n7;
        locals.var_dnm_dn10 = assign9050_e7265_d_n10;
        locals.var_dnm_dn11 = assign9050_e7265_d_n11;
        locals.var_dnm_dn12 = assign9050_e7265_d_n12;
        locals.var_dnm_dn17 = assign9050_e7265_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign9060_e7280: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard169 = assign9060_e7280;
        locals.var_guard169_rv = 0.0;

        let assign9070_e7283: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign9070_e7283;
        locals.var_guard170_rv = 0.0;

        let (assign9080_e7301,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) && (locals.var_guard170 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9080_e7301;
        locals.var_mm_rv = 0.0;

        let assign9090_e7304: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard171 = assign9090_e7304;
        locals.var_guard171_rv = 0.0;

        let (assign9100_e7325,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9100_e7325;
        locals.var_mm_rv = 0.0;

        let assign9110_e7328: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard172 = assign9110_e7328;
        locals.var_guard172_rv = 0.0;

        let (assign9120_e7352,) = {
    if (((((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 == 0.0)) && (locals.var_guard172 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9120_e7352;
        locals.var_mm_rv = 0.0;

        let assign9130_e7355: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard173 = assign9130_e7355;
        locals.var_guard173_rv = 0.0;

        let (assign9140_e7382,) = {
    if ((((((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 == 0.0)) && (locals.var_guard172 == 0.0)) && (locals.var_guard173 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9140_e7382;
        locals.var_mm_rv = 0.0;

        let (assign9150_e7398,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign9150_e7398;
        locals.var_m0_rv = 0.0;

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e7415: f64 = if (((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign9160_cond_e7415 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9160_body0_e7432, assign9160_body0_e7432_d_n0, assign9160_body0_e7432_d_n2, assign9160_body0_e7432_d_n6, assign9160_body0_e7432_d_n7, assign9160_body0_e7432_d_n10, assign9160_body0_e7432_d_n11, assign9160_body0_e7432_d_n12, assign9160_body0_e7432_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign9160_body0_e7430: f64 = (locals.var_dnm).sqrt();
        (assign9160_body0_e7430, (locals.var_dnm_dn0 / (2.0 * assign9160_body0_e7430)), (locals.var_dnm_dn2 / (2.0 * assign9160_body0_e7430)), (locals.var_dnm_dn6 / (2.0 * assign9160_body0_e7430)), (locals.var_dnm_dn7 / (2.0 * assign9160_body0_e7430)), (locals.var_dnm_dn10 / (2.0 * assign9160_body0_e7430)), (locals.var_dnm_dn11 / (2.0 * assign9160_body0_e7430)), (locals.var_dnm_dn12 / (2.0 * assign9160_body0_e7430)), (locals.var_dnm_dn17 / (2.0 * assign9160_body0_e7430)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign9160_body0_e7432;
            locals.var_dnm_dn0 = assign9160_body0_e7432_d_n0;
            locals.var_dnm_dn2 = assign9160_body0_e7432_d_n2;
            locals.var_dnm_dn6 = assign9160_body0_e7432_d_n6;
            locals.var_dnm_dn7 = assign9160_body0_e7432_d_n7;
            locals.var_dnm_dn10 = assign9160_body0_e7432_d_n10;
            locals.var_dnm_dn11 = assign9160_body0_e7432_d_n11;
            locals.var_dnm_dn12 = assign9160_body0_e7432_d_n12;
            locals.var_dnm_dn17 = assign9160_body0_e7432_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign9160_body1_e7450,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 != 0.0)) {
        let assign9160_body1_e7448: f64 = (locals.var_m0 + 1.0);
        (assign9160_body1_e7448,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign9160_body1_e7450;
            locals.var_m0_rv = 0.0;
        }

        let (assign9170_e7473, assign9170_e7473_d_n0, assign9170_e7473_d_n2, assign9170_e7473_d_n6, assign9170_e7473_d_n7, assign9170_e7473_d_n10, assign9170_e7473_d_n11, assign9170_e7473_d_n12, assign9170_e7473_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard169 == 0.0)) {
        let assign9170_e7469: f64 = (2.0 * 2.0);
        let assign9170_e7470: f64 = (1.0 / assign9170_e7469);
        let assign9170_e7471: f64 = (locals.var_dnm).powf(assign9170_e7470);
        (assign9170_e7471, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn0)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn2)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn6)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn7)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn10)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn11)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn12)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9170_e7470) as f64).is_finite() && ((assign9170_e7470) as f64).fract() == 0.0 { if assign9170_e7470 == 0.0 { 0.0 } else { (assign9170_e7470 * ((locals.var_dnm).powf(assign9170_e7470 - 1.0) * locals.var_dnm_dn17)) } } else { (assign9170_e7471 * (assign9170_e7470 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign9170_e7473;
        locals.var_dnm_dn0 = assign9170_e7473_d_n0;
        locals.var_dnm_dn2 = assign9170_e7473_d_n2;
        locals.var_dnm_dn6 = assign9170_e7473_d_n6;
        locals.var_dnm_dn7 = assign9170_e7473_d_n7;
        locals.var_dnm_dn10 = assign9170_e7473_d_n10;
        locals.var_dnm_dn11 = assign9170_e7473_d_n11;
        locals.var_dnm_dn12 = assign9170_e7473_d_n12;
        locals.var_dnm_dn17 = assign9170_e7473_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign9180_e7489, assign9180_e7489_d_n0, assign9180_e7489_d_n2, assign9180_e7489_d_n6, assign9180_e7489_d_n7, assign9180_e7489_d_n10, assign9180_e7489_d_n11, assign9180_e7489_d_n12, assign9180_e7489_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9180_e7487: f64 = (1.0 / locals.var_dnm);
        (assign9180_e7487, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign9180_e7489;
        locals.var_dnm_dn0 = assign9180_e7489_d_n0;
        locals.var_dnm_dn2 = assign9180_e7489_d_n2;
        locals.var_dnm_dn6 = assign9180_e7489_d_n6;
        locals.var_dnm_dn7 = assign9180_e7489_d_n7;
        locals.var_dnm_dn10 = assign9180_e7489_d_n10;
        locals.var_dnm_dn11 = assign9180_e7489_d_n11;
        locals.var_dnm_dn12 = assign9180_e7489_d_n12;
        locals.var_dnm_dn17 = assign9180_e7489_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign9190_e7507, assign9190_e7507_d_n0, assign9190_e7507_d_n2, assign9190_e7507_d_n6, assign9190_e7507_d_n7, assign9190_e7507_d_n10, assign9190_e7507_d_n11, assign9190_e7507_d_n12, assign9190_e7507_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9190_e7503: f64 = (locals.var_tmf1 * 0.4);
        let assign9190_e7505: f64 = (assign9190_e7503 * locals.var_dnm);
        (assign9190_e7505, (((locals.var_tmf1_dn0 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 0.4) * locals.var_dnm) + (assign9190_e7503 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign9190_e7507;
        locals.var_tmf0_dn0 = assign9190_e7507_d_n0;
        locals.var_tmf0_dn2 = assign9190_e7507_d_n2;
        locals.var_tmf0_dn6 = assign9190_e7507_d_n6;
        locals.var_tmf0_dn7 = assign9190_e7507_d_n7;
        locals.var_tmf0_dn10 = assign9190_e7507_d_n10;
        locals.var_tmf0_dn11 = assign9190_e7507_d_n11;
        locals.var_tmf0_dn12 = assign9190_e7507_d_n12;
        locals.var_tmf0_dn17 = assign9190_e7507_d_n17;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        locals: &mut StampLocals,
    ) {
        let (assign9200_e7527, assign9200_e7527_d_n0, assign9200_e7527_d_n2, assign9200_e7527_d_n6, assign9200_e7527_d_n7, assign9200_e7527_d_n10, assign9200_e7527_d_n11, assign9200_e7527_d_n12, assign9200_e7527_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) {
        let assign9200_e7521: f64 = (locals.var_ps0_inib * 0.98);
        let assign9200_e7523: f64 = (assign9200_e7521 - 0.4);
        let assign9200_e7525: f64 = (assign9200_e7523 + locals.var_tmf0);
        (assign9200_e7525, ((locals.var_ps0_inib_dn0 * 0.98) + locals.var_tmf0_dn0), ((locals.var_ps0_inib_dn2 * 0.98) + locals.var_tmf0_dn2), ((locals.var_ps0_inib_dn6 * 0.98) + locals.var_tmf0_dn6), ((locals.var_ps0_inib_dn7 * 0.98) + locals.var_tmf0_dn7), ((locals.var_ps0_inib_dn10 * 0.98) + locals.var_tmf0_dn10), ((locals.var_ps0_inib_dn11 * 0.98) + locals.var_tmf0_dn11), ((locals.var_ps0_inib_dn12 * 0.98) + locals.var_tmf0_dn12), ((locals.var_ps0_inib_dn17 * 0.98) + locals.var_tmf0_dn17),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign9200_e7527;
        locals.var_ps0_ini_dn0 = assign9200_e7527_d_n0;
        locals.var_ps0_ini_dn2 = assign9200_e7527_d_n2;
        locals.var_ps0_ini_dn6 = assign9200_e7527_d_n6;
        locals.var_ps0_ini_dn7 = assign9200_e7527_d_n7;
        locals.var_ps0_ini_dn10 = assign9200_e7527_d_n10;
        locals.var_ps0_ini_dn11 = assign9200_e7527_d_n11;
        locals.var_ps0_ini_dn12 = assign9200_e7527_d_n12;
        locals.var_ps0_ini_dn17 = assign9200_e7527_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign9210_e7542, assign9210_e7542_d_n0, assign9210_e7542_d_n2, assign9210_e7542_d_n6, assign9210_e7542_d_n7, assign9210_e7542_d_n10, assign9210_e7542_d_n11, assign9210_e7542_d_n12, assign9210_e7542_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign9210_e7542;
        locals.var_ps0_ini_dn0 = assign9210_e7542_d_n0;
        locals.var_ps0_ini_dn2 = assign9210_e7542_d_n2;
        locals.var_ps0_ini_dn6 = assign9210_e7542_d_n6;
        locals.var_ps0_ini_dn7 = assign9210_e7542_d_n7;
        locals.var_ps0_ini_dn10 = assign9210_e7542_d_n10;
        locals.var_ps0_ini_dn11 = assign9210_e7542_d_n11;
        locals.var_ps0_ini_dn12 = assign9210_e7542_d_n12;
        locals.var_ps0_ini_dn17 = assign9210_e7542_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign9220_e7549, assign9220_e7549_d_n0, assign9220_e7549_d_n2, assign9220_e7549_d_n6, assign9220_e7549_d_n7, assign9220_e7549_d_n10, assign9220_e7549_d_n11, assign9220_e7549_d_n12, assign9220_e7549_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign9220_e7549;
        locals.var_phi_s0_soi_dn0 = assign9220_e7549_d_n0;
        locals.var_phi_s0_soi_dn2 = assign9220_e7549_d_n2;
        locals.var_phi_s0_soi_dn6 = assign9220_e7549_d_n6;
        locals.var_phi_s0_soi_dn7 = assign9220_e7549_d_n7;
        locals.var_phi_s0_soi_dn10 = assign9220_e7549_d_n10;
        locals.var_phi_s0_soi_dn11 = assign9220_e7549_d_n11;
        locals.var_phi_s0_soi_dn12 = assign9220_e7549_d_n12;
        locals.var_phi_s0_soi_dn17 = assign9220_e7549_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign9230_e7556, assign9230_e7556_d_n0, assign9230_e7556_d_n2, assign9230_e7556_d_n6, assign9230_e7556_d_n7, assign9230_e7556_d_n10, assign9230_e7556_d_n11, assign9230_e7556_d_n12, assign9230_e7556_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign9230_e7556;
        locals.var_psl_lim_dn0 = assign9230_e7556_d_n0;
        locals.var_psl_lim_dn2 = assign9230_e7556_d_n2;
        locals.var_psl_lim_dn6 = assign9230_e7556_d_n6;
        locals.var_psl_lim_dn7 = assign9230_e7556_d_n7;
        locals.var_psl_lim_dn10 = assign9230_e7556_d_n10;
        locals.var_psl_lim_dn11 = assign9230_e7556_d_n11;
        locals.var_psl_lim_dn12 = assign9230_e7556_d_n12;
        locals.var_psl_lim_dn17 = assign9230_e7556_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let (assign9240_e7571, assign9240_e7571_d_n0, assign9240_e7571_d_n2, assign9240_e7571_d_n6, assign9240_e7571_d_n7, assign9240_e7571_d_n10, assign9240_e7571_d_n11, assign9240_e7571_d_n12, assign9240_e7571_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9240_e7564: f64 = (0.5 * locals.var_q_fd_soi);
        let assign9240_e7566: f64 = (assign9240_e7564 * locals.var_c_soi_inv__blk115);
        let assign9240_e7567: f64 = (locals.var_phi_s0_soi + assign9240_e7566);
        let assign9240_e7569: f64 = (assign9240_e7567 - locals.var_vbsbiz);
        (assign9240_e7569, ((locals.var_phi_s0_soi_dn0 + ((0.5 * locals.var_q_fd_soi_dn0) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn0), ((locals.var_phi_s0_soi_dn2 + ((0.5 * locals.var_q_fd_soi_dn2) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn2), ((locals.var_phi_s0_soi_dn6 + ((0.5 * locals.var_q_fd_soi_dn6) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn6), ((locals.var_phi_s0_soi_dn7 + ((0.5 * locals.var_q_fd_soi_dn7) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn7), ((locals.var_phi_s0_soi_dn10 + ((0.5 * locals.var_q_fd_soi_dn10) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn10), ((locals.var_phi_s0_soi_dn11 + ((0.5 * locals.var_q_fd_soi_dn11) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn11), ((locals.var_phi_s0_soi_dn12 + ((0.5 * locals.var_q_fd_soi_dn12) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn12), ((locals.var_phi_s0_soi_dn17 + ((0.5 * locals.var_q_fd_soi_dn17) * locals.var_c_soi_inv__blk115)) - locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign9240_e7571;
        locals.var_t1_dn0 = assign9240_e7571_d_n0;
        locals.var_t1_dn2 = assign9240_e7571_d_n2;
        locals.var_t1_dn6 = assign9240_e7571_d_n6;
        locals.var_t1_dn7 = assign9240_e7571_d_n7;
        locals.var_t1_dn10 = assign9240_e7571_d_n10;
        locals.var_t1_dn11 = assign9240_e7571_d_n11;
        locals.var_t1_dn12 = assign9240_e7571_d_n12;
        locals.var_t1_dn17 = assign9240_e7571_d_n17;
        locals.var_t1_rv = 0.0;

        let assign9250_e7574: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign9250_e7574;
        locals.var_guard174_rv = 0.0;

        let (assign9260_e7587, assign9260_e7587_d_n0, assign9260_e7587_d_n2, assign9260_e7587_d_n6, assign9260_e7587_d_n7, assign9260_e7587_d_n10, assign9260_e7587_d_n11, assign9260_e7587_d_n12, assign9260_e7587_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9260_e7584: f64 = (locals.var_c_box_inv + locals.var_c_soi_inv__blk115);
        let assign9260_e7585: f64 = (locals.var_cnst0bulk * assign9260_e7584);
        (assign9260_e7585, 0.0, 0.0, 0.0, 0.0, (locals.var_cnst0bulk_dn10 * assign9260_e7584), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9260_e7587;
        locals.var_t2_dn0 = assign9260_e7587_d_n0;
        locals.var_t2_dn2 = assign9260_e7587_d_n2;
        locals.var_t2_dn6 = assign9260_e7587_d_n6;
        locals.var_t2_dn7 = assign9260_e7587_d_n7;
        locals.var_t2_dn10 = assign9260_e7587_d_n10;
        locals.var_t2_dn11 = assign9260_e7587_d_n11;
        locals.var_t2_dn12 = assign9260_e7587_d_n12;
        locals.var_t2_dn17 = assign9260_e7587_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9270_e7598, assign9270_e7598_d_n0, assign9270_e7598_d_n2, assign9270_e7598_d_n6, assign9270_e7598_d_n7, assign9270_e7598_d_n10, assign9270_e7598_d_n11, assign9270_e7598_d_n12, assign9270_e7598_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9270_e7596: f64 = (locals.var_t2 * locals.var_t2);
        (assign9270_e7596, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9270_e7598;
        locals.var_t2_dn0 = assign9270_e7598_d_n0;
        locals.var_t2_dn2 = assign9270_e7598_d_n2;
        locals.var_t2_dn6 = assign9270_e7598_d_n6;
        locals.var_t2_dn7 = assign9270_e7598_d_n7;
        locals.var_t2_dn10 = assign9270_e7598_d_n10;
        locals.var_t2_dn11 = assign9270_e7598_d_n11;
        locals.var_t2_dn12 = assign9270_e7598_d_n12;
        locals.var_t2_dn17 = assign9270_e7598_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9280_e7612, assign9280_e7612_d_n0, assign9280_e7612_d_n2, assign9280_e7612_d_n6, assign9280_e7612_d_n7, assign9280_e7612_d_n10, assign9280_e7612_d_n11, assign9280_e7612_d_n12, assign9280_e7612_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9280_e7606: f64 = (-1.6);
        let assign9280_e7608: f64 = (assign9280_e7606 * locals.var_t1);
        let assign9280_e7610: f64 = (assign9280_e7608 + 0.6);
        (assign9280_e7610, (assign9280_e7606 * locals.var_t1_dn0), (assign9280_e7606 * locals.var_t1_dn2), (assign9280_e7606 * locals.var_t1_dn6), (assign9280_e7606 * locals.var_t1_dn7), (assign9280_e7606 * locals.var_t1_dn10), (assign9280_e7606 * locals.var_t1_dn11), (assign9280_e7606 * locals.var_t1_dn12), (assign9280_e7606 * locals.var_t1_dn17),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign9280_e7612;
        locals.var_t5_dn0 = assign9280_e7612_d_n0;
        locals.var_t5_dn2 = assign9280_e7612_d_n2;
        locals.var_t5_dn6 = assign9280_e7612_d_n6;
        locals.var_t5_dn7 = assign9280_e7612_d_n7;
        locals.var_t5_dn10 = assign9280_e7612_d_n10;
        locals.var_t5_dn11 = assign9280_e7612_d_n11;
        locals.var_t5_dn12 = assign9280_e7612_d_n12;
        locals.var_t5_dn17 = assign9280_e7612_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign9290_e7621, assign9290_e7621_d_n0, assign9290_e7621_d_n2, assign9290_e7621_d_n6, assign9290_e7621_d_n7, assign9290_e7621_d_n10, assign9290_e7621_d_n11, assign9290_e7621_d_n12, assign9290_e7621_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign9290_e7621;
        locals.var_t4_dn0 = assign9290_e7621_d_n0;
        locals.var_t4_dn2 = assign9290_e7621_d_n2;
        locals.var_t4_dn6 = assign9290_e7621_d_n6;
        locals.var_t4_dn7 = assign9290_e7621_d_n7;
        locals.var_t4_dn10 = assign9290_e7621_d_n10;
        locals.var_t4_dn11 = assign9290_e7621_d_n11;
        locals.var_t4_dn12 = assign9290_e7621_d_n12;
        locals.var_t4_dn17 = assign9290_e7621_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign9300_e7636, assign9300_e7636_d_n0, assign9300_e7636_d_n2, assign9300_e7636_d_n6, assign9300_e7636_d_n7, assign9300_e7636_d_n10, assign9300_e7636_d_n11, assign9300_e7636_d_n12, assign9300_e7636_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9300_e7630: f64 = (locals.var_t5 - locals.var_t4);
        let assign9300_e7633: f64 = (locals.var_t5 * 0.001);
        let assign9300_e7634: f64 = (assign9300_e7630 - assign9300_e7633);
        (assign9300_e7634, ((locals.var_t5_dn0 - locals.var_t4_dn0) - (locals.var_t5_dn0 * 0.001)), ((locals.var_t5_dn2 - locals.var_t4_dn2) - (locals.var_t5_dn2 * 0.001)), ((locals.var_t5_dn6 - locals.var_t4_dn6) - (locals.var_t5_dn6 * 0.001)), ((locals.var_t5_dn7 - locals.var_t4_dn7) - (locals.var_t5_dn7 * 0.001)), ((locals.var_t5_dn10 - locals.var_t4_dn10) - (locals.var_t5_dn10 * 0.001)), ((locals.var_t5_dn11 - locals.var_t4_dn11) - (locals.var_t5_dn11 * 0.001)), ((locals.var_t5_dn12 - locals.var_t4_dn12) - (locals.var_t5_dn12 * 0.001)), ((locals.var_t5_dn17 - locals.var_t4_dn17) - (locals.var_t5_dn17 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign9300_e7636;
        locals.var_tmf1_dn0 = assign9300_e7636_d_n0;
        locals.var_tmf1_dn2 = assign9300_e7636_d_n2;
        locals.var_tmf1_dn6 = assign9300_e7636_d_n6;
        locals.var_tmf1_dn7 = assign9300_e7636_d_n7;
        locals.var_tmf1_dn10 = assign9300_e7636_d_n10;
        locals.var_tmf1_dn11 = assign9300_e7636_d_n11;
        locals.var_tmf1_dn12 = assign9300_e7636_d_n12;
        locals.var_tmf1_dn17 = assign9300_e7636_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign9310_e7651, assign9310_e7651_d_n0, assign9310_e7651_d_n2, assign9310_e7651_d_n6, assign9310_e7651_d_n7, assign9310_e7651_d_n10, assign9310_e7651_d_n11, assign9310_e7651_d_n12, assign9310_e7651_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9310_e7645: f64 = (4.0 * locals.var_t5);
        let assign9310_e7648: f64 = (locals.var_t5 * 0.001);
        let assign9310_e7649: f64 = (assign9310_e7645 * assign9310_e7648);
        (assign9310_e7649, (((4.0 * locals.var_t5_dn0) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn0 * 0.001))), (((4.0 * locals.var_t5_dn2) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn2 * 0.001))), (((4.0 * locals.var_t5_dn6) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn6 * 0.001))), (((4.0 * locals.var_t5_dn7) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn7 * 0.001))), (((4.0 * locals.var_t5_dn10) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn10 * 0.001))), (((4.0 * locals.var_t5_dn11) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn11 * 0.001))), (((4.0 * locals.var_t5_dn12) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn12 * 0.001))), (((4.0 * locals.var_t5_dn17) * assign9310_e7648) + (assign9310_e7645 * (locals.var_t5_dn17 * 0.001))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9310_e7651;
        locals.var_tmf2_dn0 = assign9310_e7651_d_n0;
        locals.var_tmf2_dn2 = assign9310_e7651_d_n2;
        locals.var_tmf2_dn6 = assign9310_e7651_d_n6;
        locals.var_tmf2_dn7 = assign9310_e7651_d_n7;
        locals.var_tmf2_dn10 = assign9310_e7651_d_n10;
        locals.var_tmf2_dn11 = assign9310_e7651_d_n11;
        locals.var_tmf2_dn12 = assign9310_e7651_d_n12;
        locals.var_tmf2_dn17 = assign9310_e7651_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9320_e7666, assign9320_e7666_d_n0, assign9320_e7666_d_n2, assign9320_e7666_d_n6, assign9320_e7666_d_n7, assign9320_e7666_d_n10, assign9320_e7666_d_n11, assign9320_e7666_d_n12, assign9320_e7666_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let (assign9320_e7664, assign9320_e7664_d_n0, assign9320_e7664_d_n2, assign9320_e7664_d_n6, assign9320_e7664_d_n7, assign9320_e7664_d_n10, assign9320_e7664_d_n11, assign9320_e7664_d_n12, assign9320_e7664_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign9320_e7663: f64 = (-locals.var_tmf2);
                (assign9320_e7663, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign9320_e7664, assign9320_e7664_d_n0, assign9320_e7664_d_n2, assign9320_e7664_d_n6, assign9320_e7664_d_n7, assign9320_e7664_d_n10, assign9320_e7664_d_n11, assign9320_e7664_d_n12, assign9320_e7664_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9320_e7666;
        locals.var_tmf2_dn0 = assign9320_e7666_d_n0;
        locals.var_tmf2_dn2 = assign9320_e7666_d_n2;
        locals.var_tmf2_dn6 = assign9320_e7666_d_n6;
        locals.var_tmf2_dn7 = assign9320_e7666_d_n7;
        locals.var_tmf2_dn10 = assign9320_e7666_d_n10;
        locals.var_tmf2_dn11 = assign9320_e7666_d_n11;
        locals.var_tmf2_dn12 = assign9320_e7666_d_n12;
        locals.var_tmf2_dn17 = assign9320_e7666_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9330_e7680, assign9330_e7680_d_n0, assign9330_e7680_d_n2, assign9330_e7680_d_n6, assign9330_e7680_d_n7, assign9330_e7680_d_n10, assign9330_e7680_d_n11, assign9330_e7680_d_n12, assign9330_e7680_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9330_e7675: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9330_e7677: f64 = (assign9330_e7675 + locals.var_tmf2);
        let assign9330_e7678: f64 = (assign9330_e7677).sqrt();
        (assign9330_e7678, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9330_e7678)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9330_e7678)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign9330_e7678)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign9330_e7678)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign9330_e7678)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign9330_e7678)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign9330_e7678)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign9330_e7678)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9330_e7680;
        locals.var_tmf2_dn0 = assign9330_e7680_d_n0;
        locals.var_tmf2_dn2 = assign9330_e7680_d_n2;
        locals.var_tmf2_dn6 = assign9330_e7680_d_n6;
        locals.var_tmf2_dn7 = assign9330_e7680_d_n7;
        locals.var_tmf2_dn10 = assign9330_e7680_d_n10;
        locals.var_tmf2_dn11 = assign9330_e7680_d_n11;
        locals.var_tmf2_dn12 = assign9330_e7680_d_n12;
        locals.var_tmf2_dn17 = assign9330_e7680_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9340_e7695, assign9340_e7695_d_n0, assign9340_e7695_d_n2, assign9340_e7695_d_n6, assign9340_e7695_d_n7, assign9340_e7695_d_n10, assign9340_e7695_d_n11, assign9340_e7695_d_n12, assign9340_e7695_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9340_e7691: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9340_e7692: f64 = (0.5 * assign9340_e7691);
        let assign9340_e7693: f64 = (locals.var_t5 - assign9340_e7692);
        (assign9340_e7693, (locals.var_t5_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t5_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t5_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t5_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t5_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t5_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t5_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_t5_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign9340_e7695;
        locals.var_t4_dn0 = assign9340_e7695_d_n0;
        locals.var_t4_dn2 = assign9340_e7695_d_n2;
        locals.var_t4_dn6 = assign9340_e7695_d_n6;
        locals.var_t4_dn7 = assign9340_e7695_d_n7;
        locals.var_t4_dn10 = assign9340_e7695_d_n10;
        locals.var_t4_dn11 = assign9340_e7695_d_n11;
        locals.var_t4_dn12 = assign9340_e7695_d_n12;
        locals.var_t4_dn17 = assign9340_e7695_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign9350_e7708, assign9350_e7708_d_n0, assign9350_e7708_d_n2, assign9350_e7708_d_n6, assign9350_e7708_d_n7, assign9350_e7708_d_n10, assign9350_e7708_d_n11, assign9350_e7708_d_n12, assign9350_e7708_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9350_e7704: f64 = (locals.var_t2 * locals.var_t4);
        let assign9350_e7706: f64 = (assign9350_e7704 * locals.var_beta2);
        (assign9350_e7706, (((locals.var_t2_dn0 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn0)) * locals.var_beta2), (((locals.var_t2_dn2 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn2)) * locals.var_beta2), (((locals.var_t2_dn6 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn6)) * locals.var_beta2), (((locals.var_t2_dn7 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn7)) * locals.var_beta2), ((((locals.var_t2_dn10 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn10)) * locals.var_beta2) + (assign9350_e7704 * locals.var_beta2_dn10)), (((locals.var_t2_dn11 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn11)) * locals.var_beta2), (((locals.var_t2_dn12 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn12)) * locals.var_beta2), (((locals.var_t2_dn17 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn17)) * locals.var_beta2),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign9350_e7708;
        locals.var_t3_dn0 = assign9350_e7708_d_n0;
        locals.var_t3_dn2 = assign9350_e7708_d_n2;
        locals.var_t3_dn6 = assign9350_e7708_d_n6;
        locals.var_t3_dn7 = assign9350_e7708_d_n7;
        locals.var_t3_dn10 = assign9350_e7708_d_n10;
        locals.var_t3_dn11 = assign9350_e7708_d_n11;
        locals.var_t3_dn12 = assign9350_e7708_d_n12;
        locals.var_t3_dn17 = assign9350_e7708_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign9360_e7726, assign9360_e7726_d_n0, assign9360_e7726_d_n2, assign9360_e7726_d_n6, assign9360_e7726_d_n7, assign9360_e7726_d_n10, assign9360_e7726_d_n11, assign9360_e7726_d_n12, assign9360_e7726_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 != 0.0)) {
        let assign9360_e7718: f64 = (locals.var_t3).sqrt();
        let assign9360_e7719: f64 = (1.0 - assign9360_e7718);
        let assign9360_e7720: f64 = (locals.var_t1 * assign9360_e7719);
        let assign9360_e7723: f64 = (1.0 - locals.var_t3);
        let assign9360_e7724: f64 = (assign9360_e7720 / assign9360_e7723);
        (assign9360_e7724, (((((locals.var_t1_dn0 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn0 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn0))) / (assign9360_e7723 * assign9360_e7723)), (((((locals.var_t1_dn2 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn2 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn2))) / (assign9360_e7723 * assign9360_e7723)), (((((locals.var_t1_dn6 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn6 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn6))) / (assign9360_e7723 * assign9360_e7723)), (((((locals.var_t1_dn7 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn7 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn7))) / (assign9360_e7723 * assign9360_e7723)), (((((locals.var_t1_dn10 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn10 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn10))) / (assign9360_e7723 * assign9360_e7723)), (((((locals.var_t1_dn11 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn11 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn11))) / (assign9360_e7723 * assign9360_e7723)), (((((locals.var_t1_dn12 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn12 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn12))) / (assign9360_e7723 * assign9360_e7723)), (((((locals.var_t1_dn17 * assign9360_e7719) + (locals.var_t1 * (-(locals.var_t3_dn17 / (2.0 * assign9360_e7718))))) * assign9360_e7723) - (assign9360_e7720 * (-locals.var_t3_dn17))) / (assign9360_e7723 * assign9360_e7723)),)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9360_e7726;
        locals.var_phi_s0_bulk_dn0 = assign9360_e7726_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9360_e7726_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9360_e7726_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9360_e7726_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9360_e7726_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9360_e7726_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9360_e7726_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9360_e7726_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9370_e7742, assign9370_e7742_d_n0, assign9370_e7742_d_n2, assign9370_e7742_d_n6, assign9370_e7742_d_n7, assign9370_e7742_d_n10, assign9370_e7742_d_n11, assign9370_e7742_d_n12, assign9370_e7742_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9370_e7736: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign9370_e7738: f64 = (assign9370_e7736 * locals.var_c_box_fd_inv);
        let assign9370_e7740: f64 = (assign9370_e7738 * locals.var_c_box_fd_inv);
        (assign9370_e7740, 0.0, 0.0, 0.0, 0.0, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign9370_e7742;
        locals.var_t0_dn0 = assign9370_e7742_d_n0;
        locals.var_t0_dn2 = assign9370_e7742_d_n2;
        locals.var_t0_dn6 = assign9370_e7742_d_n6;
        locals.var_t0_dn7 = assign9370_e7742_d_n7;
        locals.var_t0_dn10 = assign9370_e7742_d_n10;
        locals.var_t0_dn11 = assign9370_e7742_d_n11;
        locals.var_t0_dn12 = assign9370_e7742_d_n12;
        locals.var_t0_dn17 = assign9370_e7742_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign9380_e7763, assign9380_e7763_d_n0, assign9380_e7763_d_n2, assign9380_e7763_d_n6, assign9380_e7763_d_n7, assign9380_e7763_d_n10, assign9380_e7763_d_n11, assign9380_e7763_d_n12, assign9380_e7763_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9380_e7752: f64 = (locals.var_vbsbiz - locals.var_phi_s0_soi);
        let assign9380_e7755: f64 = (locals.var_q_fd_soi / 2.0);
        let assign9380_e7757: f64 = (assign9380_e7755 * locals.var_t_soi);
        let assign9380_e7759: f64 = (assign9380_e7757 / 1.034943e-10);
        let assign9380_e7760: f64 = (assign9380_e7752 - assign9380_e7759);
        let assign9380_e7761: f64 = (-assign9380_e7760);
        (assign9380_e7761, (-((locals.var_vbsbiz_dn0 - locals.var_phi_s0_soi_dn0) - (((locals.var_q_fd_soi_dn0 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn2 - locals.var_phi_s0_soi_dn2) - (((locals.var_q_fd_soi_dn2 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn6 - locals.var_phi_s0_soi_dn6) - (((locals.var_q_fd_soi_dn6 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn7 - locals.var_phi_s0_soi_dn7) - (((locals.var_q_fd_soi_dn7 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn10 - locals.var_phi_s0_soi_dn10) - (((locals.var_q_fd_soi_dn10 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn11 - locals.var_phi_s0_soi_dn11) - (((locals.var_q_fd_soi_dn11 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn12 - locals.var_phi_s0_soi_dn12) - (((locals.var_q_fd_soi_dn12 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn17 - locals.var_phi_s0_soi_dn17) - (((locals.var_q_fd_soi_dn17 / 2.0) * locals.var_t_soi) / 1.034943e-10))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign9380_e7763;
        locals.var_t1_dn0 = assign9380_e7763_d_n0;
        locals.var_t1_dn2 = assign9380_e7763_d_n2;
        locals.var_t1_dn6 = assign9380_e7763_d_n6;
        locals.var_t1_dn7 = assign9380_e7763_d_n7;
        locals.var_t1_dn10 = assign9380_e7763_d_n10;
        locals.var_t1_dn11 = assign9380_e7763_d_n11;
        locals.var_t1_dn12 = assign9380_e7763_d_n12;
        locals.var_t1_dn17 = assign9380_e7763_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign9390_e7795, assign9390_e7795_d_n0, assign9390_e7795_d_n2, assign9390_e7795_d_n6, assign9390_e7795_d_n7, assign9390_e7795_d_n10, assign9390_e7795_d_n11, assign9390_e7795_d_n12, assign9390_e7795_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9390_e7773: f64 = (2.0 * locals.var_t1);
        let assign9390_e7776: f64 = (locals.var_t0 * locals.var_beta);
        let assign9390_e7777: f64 = (assign9390_e7773 + assign9390_e7776);
        let assign9390_e7780: f64 = (2.0 * locals.var_t1);
        let assign9390_e7783: f64 = (locals.var_t0 * locals.var_beta);
        let assign9390_e7784: f64 = (assign9390_e7780 + assign9390_e7783);
        let assign9390_e7785: f64 = (assign9390_e7777 * assign9390_e7784);
        let assign9390_e7789: f64 = (locals.var_t1 * locals.var_t1);
        let assign9390_e7791: f64 = (assign9390_e7789 + locals.var_t0);
        let assign9390_e7792: f64 = (4.0 * assign9390_e7791);
        let assign9390_e7793: f64 = (assign9390_e7785 - assign9390_e7792);
        (assign9390_e7793, (((((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + locals.var_t0_dn0))), (((((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + locals.var_t0_dn2))), (((((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + locals.var_t0_dn6))), (((((2.0 * locals.var_t1_dn7) + (locals.var_t0_dn7 * locals.var_beta)) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn7) + (locals.var_t0_dn7 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + locals.var_t0_dn7))), (((((2.0 * locals.var_t1_dn10) + ((locals.var_t0_dn10 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn10))) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn10) + ((locals.var_t0_dn10 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + locals.var_t0_dn10))), (((((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + locals.var_t0_dn11))), (((((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + locals.var_t0_dn12))), (((((2.0 * locals.var_t1_dn17) + (locals.var_t0_dn17 * locals.var_beta)) * assign9390_e7784) + (assign9390_e7777 * ((2.0 * locals.var_t1_dn17) + (locals.var_t0_dn17 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) + locals.var_t0_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9390_e7795;
        locals.var_t2_dn0 = assign9390_e7795_d_n0;
        locals.var_t2_dn2 = assign9390_e7795_d_n2;
        locals.var_t2_dn6 = assign9390_e7795_d_n6;
        locals.var_t2_dn7 = assign9390_e7795_d_n7;
        locals.var_t2_dn10 = assign9390_e7795_d_n10;
        locals.var_t2_dn11 = assign9390_e7795_d_n11;
        locals.var_t2_dn12 = assign9390_e7795_d_n12;
        locals.var_t2_dn17 = assign9390_e7795_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9400_e7814, assign9400_e7814_d_n0, assign9400_e7814_d_n2, assign9400_e7814_d_n6, assign9400_e7814_d_n7, assign9400_e7814_d_n10, assign9400_e7814_d_n11, assign9400_e7814_d_n12, assign9400_e7814_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9400_e7806: f64 = (10.0 * 2.220446049250313e-16);
        let (assign9400_e7812, assign9400_e7812_d_n0, assign9400_e7812_d_n2, assign9400_e7812_d_n6, assign9400_e7812_d_n7, assign9400_e7812_d_n10, assign9400_e7812_d_n11, assign9400_e7812_d_n12, assign9400_e7812_d_n17,) = {
            if (locals.var_t2 >= assign9400_e7806) {
                (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
            } else {
                let assign9400_e7811: f64 = (10.0 * 2.220446049250313e-16);
                (assign9400_e7811, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign9400_e7812, assign9400_e7812_d_n0, assign9400_e7812_d_n2, assign9400_e7812_d_n6, assign9400_e7812_d_n7, assign9400_e7812_d_n10, assign9400_e7812_d_n11, assign9400_e7812_d_n12, assign9400_e7812_d_n17,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9400_e7814;
        locals.var_t2_dn0 = assign9400_e7814_d_n0;
        locals.var_t2_dn2 = assign9400_e7814_d_n2;
        locals.var_t2_dn6 = assign9400_e7814_d_n6;
        locals.var_t2_dn7 = assign9400_e7814_d_n7;
        locals.var_t2_dn10 = assign9400_e7814_d_n10;
        locals.var_t2_dn11 = assign9400_e7814_d_n11;
        locals.var_t2_dn12 = assign9400_e7814_d_n12;
        locals.var_t2_dn17 = assign9400_e7814_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9410_e7825, assign9410_e7825_d_n0, assign9410_e7825_d_n2, assign9410_e7825_d_n6, assign9410_e7825_d_n7, assign9410_e7825_d_n10, assign9410_e7825_d_n11, assign9410_e7825_d_n12, assign9410_e7825_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9410_e7823: f64 = (locals.var_t2).sqrt();
        (assign9410_e7823, (locals.var_t2_dn0 / (2.0 * assign9410_e7823)), (locals.var_t2_dn2 / (2.0 * assign9410_e7823)), (locals.var_t2_dn6 / (2.0 * assign9410_e7823)), (locals.var_t2_dn7 / (2.0 * assign9410_e7823)), (locals.var_t2_dn10 / (2.0 * assign9410_e7823)), (locals.var_t2_dn11 / (2.0 * assign9410_e7823)), (locals.var_t2_dn12 / (2.0 * assign9410_e7823)), (locals.var_t2_dn17 / (2.0 * assign9410_e7823)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9410_e7825;
        locals.var_t2_dn0 = assign9410_e7825_d_n0;
        locals.var_t2_dn2 = assign9410_e7825_d_n2;
        locals.var_t2_dn6 = assign9410_e7825_d_n6;
        locals.var_t2_dn7 = assign9410_e7825_d_n7;
        locals.var_t2_dn10 = assign9410_e7825_d_n10;
        locals.var_t2_dn11 = assign9410_e7825_d_n11;
        locals.var_t2_dn12 = assign9410_e7825_d_n12;
        locals.var_t2_dn17 = assign9410_e7825_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9420_e7841, assign9420_e7841_d_n0, assign9420_e7841_d_n2, assign9420_e7841_d_n6, assign9420_e7841_d_n7, assign9420_e7841_d_n10, assign9420_e7841_d_n11, assign9420_e7841_d_n12, assign9420_e7841_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9420_e7835: f64 = (2.0 * locals.var_t1);
        let assign9420_e7838: f64 = (locals.var_t0 * locals.var_beta);
        let assign9420_e7839: f64 = (assign9420_e7835 + assign9420_e7838);
        (assign9420_e7839, ((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)), ((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)), ((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)), ((2.0 * locals.var_t1_dn7) + (locals.var_t0_dn7 * locals.var_beta)), ((2.0 * locals.var_t1_dn10) + ((locals.var_t0_dn10 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn10))), ((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)), ((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)), ((2.0 * locals.var_t1_dn17) + (locals.var_t0_dn17 * locals.var_beta)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign9420_e7841;
        locals.var_t3_dn0 = assign9420_e7841_d_n0;
        locals.var_t3_dn2 = assign9420_e7841_d_n2;
        locals.var_t3_dn6 = assign9420_e7841_d_n6;
        locals.var_t3_dn7 = assign9420_e7841_d_n7;
        locals.var_t3_dn10 = assign9420_e7841_d_n10;
        locals.var_t3_dn11 = assign9420_e7841_d_n11;
        locals.var_t3_dn12 = assign9420_e7841_d_n12;
        locals.var_t3_dn17 = assign9420_e7841_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign9430_e7855, assign9430_e7855_d_n0, assign9430_e7855_d_n2, assign9430_e7855_d_n6, assign9430_e7855_d_n7, assign9430_e7855_d_n10, assign9430_e7855_d_n11, assign9430_e7855_d_n12, assign9430_e7855_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9430_e7851: f64 = (locals.var_t3 - locals.var_t2);
        let assign9430_e7853: f64 = (assign9430_e7851 / 2.0);
        (assign9430_e7853, ((locals.var_t3_dn0 - locals.var_t2_dn0) / 2.0), ((locals.var_t3_dn2 - locals.var_t2_dn2) / 2.0), ((locals.var_t3_dn6 - locals.var_t2_dn6) / 2.0), ((locals.var_t3_dn7 - locals.var_t2_dn7) / 2.0), ((locals.var_t3_dn10 - locals.var_t2_dn10) / 2.0), ((locals.var_t3_dn11 - locals.var_t2_dn11) / 2.0), ((locals.var_t3_dn12 - locals.var_t2_dn12) / 2.0), ((locals.var_t3_dn17 - locals.var_t2_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia, locals.var_psb_inia_dn0, locals.var_psb_inia_dn2, locals.var_psb_inia_dn6, locals.var_psb_inia_dn7, locals.var_psb_inia_dn10, locals.var_psb_inia_dn11, locals.var_psb_inia_dn12, locals.var_psb_inia_dn17,)
    }
};
        locals.var_psb_inia = assign9430_e7855;
        locals.var_psb_inia_dn0 = assign9430_e7855_d_n0;
        locals.var_psb_inia_dn2 = assign9430_e7855_d_n2;
        locals.var_psb_inia_dn6 = assign9430_e7855_d_n6;
        locals.var_psb_inia_dn7 = assign9430_e7855_d_n7;
        locals.var_psb_inia_dn10 = assign9430_e7855_d_n10;
        locals.var_psb_inia_dn11 = assign9430_e7855_d_n11;
        locals.var_psb_inia_dn12 = assign9430_e7855_d_n12;
        locals.var_psb_inia_dn17 = assign9430_e7855_d_n17;
        locals.var_psb_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        locals: &mut StampLocals,
    ) {
        let (assign9440_e7878, assign9440_e7878_d_n0, assign9440_e7878_d_n2, assign9440_e7878_d_n6, assign9440_e7878_d_n7, assign9440_e7878_d_n10, assign9440_e7878_d_n11, assign9440_e7878_d_n12, assign9440_e7878_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign9440_e7865: f64 = (locals.var_t1 * locals.var_t1);
        let assign9440_e7867: f64 = (assign9440_e7865 / locals.var_t0);
        let assign9440_e7869: f64 = (assign9440_e7867 / locals.var_cnst1bulk);
        let assign9440_e7870: f64 = (assign9440_e7869).ln();
        let assign9440_e7874: f64 = (2.0 / locals.var_t1);
        let assign9440_e7875: f64 = (locals.var_beta + assign9440_e7874);
        let assign9440_e7876: f64 = (assign9440_e7870 / assign9440_e7875);
        (assign9440_e7876, ((((((((((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))))) / (assign9440_e7875 * assign9440_e7875)), ((((((((((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))))) / (assign9440_e7875 * assign9440_e7875)), ((((((((((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))))) / (assign9440_e7875 * assign9440_e7875)), ((((((((((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))))) / (assign9440_e7875 * assign9440_e7875)), ((((((((((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))))) / (assign9440_e7875 * assign9440_e7875)), ((((((((((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))))) / (assign9440_e7875 * assign9440_e7875)), ((((((((((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (-((2.0 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))))) / (assign9440_e7875 * assign9440_e7875)), ((((((((((((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) * locals.var_t0) - (assign9440_e7865 * locals.var_t0_dn17)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9440_e7867 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9440_e7869) * assign9440_e7875) - (assign9440_e7870 * (-((2.0 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))))) / (assign9440_e7875 * assign9440_e7875)),)
    } else {
        (locals.var_psb_inib, locals.var_psb_inib_dn0, locals.var_psb_inib_dn2, locals.var_psb_inib_dn6, locals.var_psb_inib_dn7, locals.var_psb_inib_dn10, locals.var_psb_inib_dn11, locals.var_psb_inib_dn12, locals.var_psb_inib_dn17,)
    }
};
        locals.var_psb_inib = assign9440_e7878;
        locals.var_psb_inib_dn0 = assign9440_e7878_d_n0;
        locals.var_psb_inib_dn2 = assign9440_e7878_d_n2;
        locals.var_psb_inib_dn6 = assign9440_e7878_d_n6;
        locals.var_psb_inib_dn7 = assign9440_e7878_d_n7;
        locals.var_psb_inib_dn10 = assign9440_e7878_d_n10;
        locals.var_psb_inib_dn11 = assign9440_e7878_d_n11;
        locals.var_psb_inib_dn12 = assign9440_e7878_d_n12;
        locals.var_psb_inib_dn17 = assign9440_e7878_d_n17;
        locals.var_psb_inib_rv = 0.0;

        let assign9450_e7881: f64 = if locals.var_psb_inia < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard175 = assign9450_e7881;
        locals.var_guard175_rv = 0.0;

        let (assign9460_e7893, assign9460_e7893_d_n0, assign9460_e7893_d_n2, assign9460_e7893_d_n6, assign9460_e7893_d_n7, assign9460_e7893_d_n10, assign9460_e7893_d_n11, assign9460_e7893_d_n12, assign9460_e7893_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) && (locals.var_guard175 != 0.0)) {
        (locals.var_psb_inia, locals.var_psb_inia_dn0, locals.var_psb_inia_dn2, locals.var_psb_inia_dn6, locals.var_psb_inia_dn7, locals.var_psb_inia_dn10, locals.var_psb_inia_dn11, locals.var_psb_inia_dn12, locals.var_psb_inia_dn17,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9460_e7893;
        locals.var_phi_s0_bulk_dn0 = assign9460_e7893_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9460_e7893_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9460_e7893_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9460_e7893_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9460_e7893_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9460_e7893_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9460_e7893_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9460_e7893_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9470_e7910, assign9470_e7910_d_n0, assign9470_e7910_d_n2, assign9470_e7910_d_n6, assign9470_e7910_d_n7, assign9470_e7910_d_n10, assign9470_e7910_d_n11, assign9470_e7910_d_n12, assign9470_e7910_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) && (locals.var_guard175 == 0.0)) {
        let assign9470_e7906: f64 = (locals.var_psb_inib - locals.var_psb_inia);
        let assign9470_e7908: f64 = (assign9470_e7906 - 0.0008);
        (assign9470_e7908, (locals.var_psb_inib_dn0 - locals.var_psb_inia_dn0), (locals.var_psb_inib_dn2 - locals.var_psb_inia_dn2), (locals.var_psb_inib_dn6 - locals.var_psb_inia_dn6), (locals.var_psb_inib_dn7 - locals.var_psb_inia_dn7), (locals.var_psb_inib_dn10 - locals.var_psb_inia_dn10), (locals.var_psb_inib_dn11 - locals.var_psb_inia_dn11), (locals.var_psb_inib_dn12 - locals.var_psb_inia_dn12), (locals.var_psb_inib_dn17 - locals.var_psb_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign9470_e7910;
        locals.var_tmf1_dn0 = assign9470_e7910_d_n0;
        locals.var_tmf1_dn2 = assign9470_e7910_d_n2;
        locals.var_tmf1_dn6 = assign9470_e7910_d_n6;
        locals.var_tmf1_dn7 = assign9470_e7910_d_n7;
        locals.var_tmf1_dn10 = assign9470_e7910_d_n10;
        locals.var_tmf1_dn11 = assign9470_e7910_d_n11;
        locals.var_tmf1_dn12 = assign9470_e7910_d_n12;
        locals.var_tmf1_dn17 = assign9470_e7910_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign9480_e7927, assign9480_e7927_d_n0, assign9480_e7927_d_n2, assign9480_e7927_d_n6, assign9480_e7927_d_n7, assign9480_e7927_d_n10, assign9480_e7927_d_n11, assign9480_e7927_d_n12, assign9480_e7927_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) && (locals.var_guard175 == 0.0)) {
        let assign9480_e7923: f64 = (4.0 * locals.var_psb_inib);
        let assign9480_e7925: f64 = (assign9480_e7923 * 0.0008);
        (assign9480_e7925, ((4.0 * locals.var_psb_inib_dn0) * 0.0008), ((4.0 * locals.var_psb_inib_dn2) * 0.0008), ((4.0 * locals.var_psb_inib_dn6) * 0.0008), ((4.0 * locals.var_psb_inib_dn7) * 0.0008), ((4.0 * locals.var_psb_inib_dn10) * 0.0008), ((4.0 * locals.var_psb_inib_dn11) * 0.0008), ((4.0 * locals.var_psb_inib_dn12) * 0.0008), ((4.0 * locals.var_psb_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9480_e7927;
        locals.var_tmf2_dn0 = assign9480_e7927_d_n0;
        locals.var_tmf2_dn2 = assign9480_e7927_d_n2;
        locals.var_tmf2_dn6 = assign9480_e7927_d_n6;
        locals.var_tmf2_dn7 = assign9480_e7927_d_n7;
        locals.var_tmf2_dn10 = assign9480_e7927_d_n10;
        locals.var_tmf2_dn11 = assign9480_e7927_d_n11;
        locals.var_tmf2_dn12 = assign9480_e7927_d_n12;
        locals.var_tmf2_dn17 = assign9480_e7927_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9490_e7946, assign9490_e7946_d_n0, assign9490_e7946_d_n2, assign9490_e7946_d_n6, assign9490_e7946_d_n7, assign9490_e7946_d_n10, assign9490_e7946_d_n11, assign9490_e7946_d_n12, assign9490_e7946_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) && (locals.var_guard175 == 0.0)) {
        let (assign9490_e7944, assign9490_e7944_d_n0, assign9490_e7944_d_n2, assign9490_e7944_d_n6, assign9490_e7944_d_n7, assign9490_e7944_d_n10, assign9490_e7944_d_n11, assign9490_e7944_d_n12, assign9490_e7944_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign9490_e7943: f64 = (-locals.var_tmf2);
                (assign9490_e7943, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign9490_e7944, assign9490_e7944_d_n0, assign9490_e7944_d_n2, assign9490_e7944_d_n6, assign9490_e7944_d_n7, assign9490_e7944_d_n10, assign9490_e7944_d_n11, assign9490_e7944_d_n12, assign9490_e7944_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9490_e7946;
        locals.var_tmf2_dn0 = assign9490_e7946_d_n0;
        locals.var_tmf2_dn2 = assign9490_e7946_d_n2;
        locals.var_tmf2_dn6 = assign9490_e7946_d_n6;
        locals.var_tmf2_dn7 = assign9490_e7946_d_n7;
        locals.var_tmf2_dn10 = assign9490_e7946_d_n10;
        locals.var_tmf2_dn11 = assign9490_e7946_d_n11;
        locals.var_tmf2_dn12 = assign9490_e7946_d_n12;
        locals.var_tmf2_dn17 = assign9490_e7946_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9500_e7964, assign9500_e7964_d_n0, assign9500_e7964_d_n2, assign9500_e7964_d_n6, assign9500_e7964_d_n7, assign9500_e7964_d_n10, assign9500_e7964_d_n11, assign9500_e7964_d_n12, assign9500_e7964_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) && (locals.var_guard175 == 0.0)) {
        let assign9500_e7959: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9500_e7961: f64 = (assign9500_e7959 + locals.var_tmf2);
        let assign9500_e7962: f64 = (assign9500_e7961).sqrt();
        (assign9500_e7962, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9500_e7962)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9500_e7962)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign9500_e7962)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign9500_e7962)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign9500_e7962)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign9500_e7962)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign9500_e7962)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign9500_e7962)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9500_e7964;
        locals.var_tmf2_dn0 = assign9500_e7964_d_n0;
        locals.var_tmf2_dn2 = assign9500_e7964_d_n2;
        locals.var_tmf2_dn6 = assign9500_e7964_d_n6;
        locals.var_tmf2_dn7 = assign9500_e7964_d_n7;
        locals.var_tmf2_dn10 = assign9500_e7964_d_n10;
        locals.var_tmf2_dn11 = assign9500_e7964_d_n11;
        locals.var_tmf2_dn12 = assign9500_e7964_d_n12;
        locals.var_tmf2_dn17 = assign9500_e7964_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9510_e7983, assign9510_e7983_d_n0, assign9510_e7983_d_n2, assign9510_e7983_d_n6, assign9510_e7983_d_n7, assign9510_e7983_d_n10, assign9510_e7983_d_n11, assign9510_e7983_d_n12, assign9510_e7983_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard174 == 0.0)) && (locals.var_guard175 == 0.0)) {
        let assign9510_e7979: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9510_e7980: f64 = (0.5 * assign9510_e7979);
        let assign9510_e7981: f64 = (locals.var_psb_inib - assign9510_e7980);
        (assign9510_e7981, (locals.var_psb_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9510_e7983;
        locals.var_phi_s0_bulk_dn0 = assign9510_e7983_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9510_e7983_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9510_e7983_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9510_e7983_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9510_e7983_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9510_e7983_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9510_e7983_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9510_e7983_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9520_e7990,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign9520_e7990;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        locals: &mut StampLocals,
    ) {
        let mut assign9530_loop_guard: usize = 0;
        while {
            let assign9530_cond_e7998: f64 = if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_lp_s0 < locals.var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign9530_cond_e7998 != 0.0
        } {
            assign9530_loop_guard += 1;
            assert!(assign9530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9530_body0_e8005, assign9530_body0_e8005_d_n0, assign9530_body0_e8005_d_n2, assign9530_body0_e8005_d_n6, assign9530_body0_e8005_d_n7, assign9530_body0_e8005_d_n10, assign9530_body0_e8005_d_n11, assign9530_body0_e8005_d_n12, assign9530_body0_e8005_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        (locals.var_cnst0bulk, 0.0, 0.0, 0.0, 0.0, locals.var_cnst0bulk_dn10, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign9530_body0_e8005;
            locals.var_t1_dn0 = assign9530_body0_e8005_d_n0;
            locals.var_t1_dn2 = assign9530_body0_e8005_d_n2;
            locals.var_t1_dn6 = assign9530_body0_e8005_d_n6;
            locals.var_t1_dn7 = assign9530_body0_e8005_d_n7;
            locals.var_t1_dn10 = assign9530_body0_e8005_d_n10;
            locals.var_t1_dn11 = assign9530_body0_e8005_d_n11;
            locals.var_t1_dn12 = assign9530_body0_e8005_d_n12;
            locals.var_t1_dn17 = assign9530_body0_e8005_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign9530_body1_e8014, assign9530_body1_e8014_d_n0, assign9530_body1_e8014_d_n2, assign9530_body1_e8014_d_n6, assign9530_body1_e8014_d_n7, assign9530_body1_e8014_d_n10, assign9530_body1_e8014_d_n11, assign9530_body1_e8014_d_n12, assign9530_body1_e8014_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body1_e8012: f64 = (locals.var_beta * locals.var_phi_s0_bulk);
        (assign9530_body1_e8012, (locals.var_beta * locals.var_phi_s0_bulk_dn0), (locals.var_beta * locals.var_phi_s0_bulk_dn2), (locals.var_beta * locals.var_phi_s0_bulk_dn6), (locals.var_beta * locals.var_phi_s0_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_s0_bulk) + (locals.var_beta * locals.var_phi_s0_bulk_dn10)), (locals.var_beta * locals.var_phi_s0_bulk_dn11), (locals.var_beta * locals.var_phi_s0_bulk_dn12), (locals.var_beta * locals.var_phi_s0_bulk_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign9530_body1_e8014;
            locals.var_t2_dn0 = assign9530_body1_e8014_d_n0;
            locals.var_t2_dn2 = assign9530_body1_e8014_d_n2;
            locals.var_t2_dn6 = assign9530_body1_e8014_d_n6;
            locals.var_t2_dn7 = assign9530_body1_e8014_d_n7;
            locals.var_t2_dn10 = assign9530_body1_e8014_d_n10;
            locals.var_t2_dn11 = assign9530_body1_e8014_d_n11;
            locals.var_t2_dn12 = assign9530_body1_e8014_d_n12;
            locals.var_t2_dn17 = assign9530_body1_e8014_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign9530_body2_e8023, assign9530_body2_e8023_d_n0, assign9530_body2_e8023_d_n2, assign9530_body2_e8023_d_n6, assign9530_body2_e8023_d_n7, assign9530_body2_e8023_d_n10, assign9530_body2_e8023_d_n11, assign9530_body2_e8023_d_n12, assign9530_body2_e8023_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body2_e8020: f64 = (-locals.var_t2);
        let assign9530_body2_e8021: f64 = (assign9530_body2_e8020).exp();
        (assign9530_body2_e8021, (assign9530_body2_e8021 * (-locals.var_t2_dn0)), (assign9530_body2_e8021 * (-locals.var_t2_dn2)), (assign9530_body2_e8021 * (-locals.var_t2_dn6)), (assign9530_body2_e8021 * (-locals.var_t2_dn7)), (assign9530_body2_e8021 * (-locals.var_t2_dn10)), (assign9530_body2_e8021 * (-locals.var_t2_dn11)), (assign9530_body2_e8021 * (-locals.var_t2_dn12)), (assign9530_body2_e8021 * (-locals.var_t2_dn17)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign9530_body2_e8023;
            locals.var_t3_dn0 = assign9530_body2_e8023_d_n0;
            locals.var_t3_dn2 = assign9530_body2_e8023_d_n2;
            locals.var_t3_dn6 = assign9530_body2_e8023_d_n6;
            locals.var_t3_dn7 = assign9530_body2_e8023_d_n7;
            locals.var_t3_dn10 = assign9530_body2_e8023_d_n10;
            locals.var_t3_dn11 = assign9530_body2_e8023_d_n11;
            locals.var_t3_dn12 = assign9530_body2_e8023_d_n12;
            locals.var_t3_dn17 = assign9530_body2_e8023_d_n17;
            locals.var_t3_rv = 0.0;
            let assign9530_body3_e8026: f64 = if locals.var_phi_s0_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard176 = assign9530_body3_e8026;
            locals.var_guard176_rv = 0.0;
            let (assign9530_body4_e8038, assign9530_body4_e8038_d_n0, assign9530_body4_e8038_d_n2, assign9530_body4_e8038_d_n6, assign9530_body4_e8038_d_n7, assign9530_body4_e8038_d_n10, assign9530_body4_e8038_d_n11, assign9530_body4_e8038_d_n12, assign9530_body4_e8038_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard176 != 0.0)) {
        let assign9530_body4_e8035: f64 = (locals.var_beta * locals.var_phi_s0_bulk);
        let assign9530_body4_e8036: f64 = (assign9530_body4_e8035).exp();
        (assign9530_body4_e8036, (assign9530_body4_e8036 * (locals.var_beta * locals.var_phi_s0_bulk_dn0)), (assign9530_body4_e8036 * (locals.var_beta * locals.var_phi_s0_bulk_dn2)), (assign9530_body4_e8036 * (locals.var_beta * locals.var_phi_s0_bulk_dn6)), (assign9530_body4_e8036 * (locals.var_beta * locals.var_phi_s0_bulk_dn7)), (assign9530_body4_e8036 * ((locals.var_beta_dn10 * locals.var_phi_s0_bulk) + (locals.var_beta * locals.var_phi_s0_bulk_dn10))), (assign9530_body4_e8036 * (locals.var_beta * locals.var_phi_s0_bulk_dn11)), (assign9530_body4_e8036 * (locals.var_beta * locals.var_phi_s0_bulk_dn12)), (assign9530_body4_e8036 * (locals.var_beta * locals.var_phi_s0_bulk_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign9530_body4_e8038;
            locals.var_t0_dn0 = assign9530_body4_e8038_d_n0;
            locals.var_t0_dn2 = assign9530_body4_e8038_d_n2;
            locals.var_t0_dn6 = assign9530_body4_e8038_d_n6;
            locals.var_t0_dn7 = assign9530_body4_e8038_d_n7;
            locals.var_t0_dn10 = assign9530_body4_e8038_d_n10;
            locals.var_t0_dn11 = assign9530_body4_e8038_d_n11;
            locals.var_t0_dn12 = assign9530_body4_e8038_d_n12;
            locals.var_t0_dn17 = assign9530_body4_e8038_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign9530_body5_e8061, assign9530_body5_e8061_d_n0, assign9530_body5_e8061_d_n2, assign9530_body5_e8061_d_n6, assign9530_body5_e8061_d_n7, assign9530_body5_e8061_d_n10, assign9530_body5_e8061_d_n11, assign9530_body5_e8061_d_n12, assign9530_body5_e8061_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard176 != 0.0)) {
        let assign9530_body5_e8046: f64 = (-locals.var_t1);
        let assign9530_body5_e8049: f64 = (locals.var_t3 + locals.var_t2);
        let assign9530_body5_e8051: f64 = (assign9530_body5_e8049 - 1.0);
        let assign9530_body5_e8055: f64 = (locals.var_t0 - 1.0);
        let assign9530_body5_e8056: f64 = (locals.var_cnst1bulk * assign9530_body5_e8055);
        let assign9530_body5_e8057: f64 = (assign9530_body5_e8051 + assign9530_body5_e8056);
        let assign9530_body5_e8058: f64 = (assign9530_body5_e8057).sqrt();
        let assign9530_body5_e8059: f64 = (assign9530_body5_e8046 * assign9530_body5_e8058);
        (assign9530_body5_e8059, (((-locals.var_t1_dn0) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn0 + locals.var_t2_dn0) + ((locals.var_cnst1bulk_dn0 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn0))) / (2.0 * assign9530_body5_e8058)))), (((-locals.var_t1_dn2) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn2 + locals.var_t2_dn2) + ((locals.var_cnst1bulk_dn2 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn2))) / (2.0 * assign9530_body5_e8058)))), (((-locals.var_t1_dn6) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn6 + locals.var_t2_dn6) + ((locals.var_cnst1bulk_dn6 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn6))) / (2.0 * assign9530_body5_e8058)))), (((-locals.var_t1_dn7) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn7 + locals.var_t2_dn7) + ((locals.var_cnst1bulk_dn7 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn7))) / (2.0 * assign9530_body5_e8058)))), (((-locals.var_t1_dn10) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn10 + locals.var_t2_dn10) + ((locals.var_cnst1bulk_dn10 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn10))) / (2.0 * assign9530_body5_e8058)))), (((-locals.var_t1_dn11) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn11 + locals.var_t2_dn11) + ((locals.var_cnst1bulk_dn11 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn11))) / (2.0 * assign9530_body5_e8058)))), (((-locals.var_t1_dn12) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn12 + locals.var_t2_dn12) + ((locals.var_cnst1bulk_dn12 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn12))) / (2.0 * assign9530_body5_e8058)))), (((-locals.var_t1_dn17) * assign9530_body5_e8058) + (assign9530_body5_e8046 * (((locals.var_t3_dn17 + locals.var_t2_dn17) + ((locals.var_cnst1bulk_dn17 * assign9530_body5_e8055) + (locals.var_cnst1bulk * locals.var_t0_dn17))) / (2.0 * assign9530_body5_e8058)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
            locals.var_t4 = assign9530_body5_e8061;
            locals.var_t4_dn0 = assign9530_body5_e8061_d_n0;
            locals.var_t4_dn2 = assign9530_body5_e8061_d_n2;
            locals.var_t4_dn6 = assign9530_body5_e8061_d_n6;
            locals.var_t4_dn7 = assign9530_body5_e8061_d_n7;
            locals.var_t4_dn10 = assign9530_body5_e8061_d_n10;
            locals.var_t4_dn11 = assign9530_body5_e8061_d_n11;
            locals.var_t4_dn12 = assign9530_body5_e8061_d_n12;
            locals.var_t4_dn17 = assign9530_body5_e8061_d_n17;
            locals.var_t4_rv = 0.0;
            let (assign9530_body6_e8081, assign9530_body6_e8081_d_n0, assign9530_body6_e8081_d_n2, assign9530_body6_e8081_d_n6, assign9530_body6_e8081_d_n7, assign9530_body6_e8081_d_n10, assign9530_body6_e8081_d_n11, assign9530_body6_e8081_d_n12, assign9530_body6_e8081_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard176 != 0.0)) {
        let assign9530_body6_e8070: f64 = (locals.var_c0bulk / locals.var_t4);
        let assign9530_body6_e8072: f64 = (-locals.var_t3);
        let assign9530_body6_e8074: f64 = (assign9530_body6_e8072 + 1.0);
        let assign9530_body6_e8077: f64 = (locals.var_cnst1bulk * locals.var_t0);
        let assign9530_body6_e8078: f64 = (assign9530_body6_e8074 + assign9530_body6_e8077);
        let assign9530_body6_e8079: f64 = (assign9530_body6_e8070 * assign9530_body6_e8078);
        (assign9530_body6_e8079, (((-((locals.var_c0bulk * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn0))))), (((-((locals.var_c0bulk * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn2))))), (((-((locals.var_c0bulk * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn6))))), (((-((locals.var_c0bulk * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn7))))), (((-((locals.var_c0bulk * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn10))))), (((-((locals.var_c0bulk * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn11))))), (((-((locals.var_c0bulk * locals.var_t4_dn12) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn12))))), (((-((locals.var_c0bulk * locals.var_t4_dn17) / (locals.var_t4 * locals.var_t4))) * assign9530_body6_e8078) + (assign9530_body6_e8070 * ((-locals.var_t3_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn17))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
            locals.var_t5 = assign9530_body6_e8081;
            locals.var_t5_dn0 = assign9530_body6_e8081_d_n0;
            locals.var_t5_dn2 = assign9530_body6_e8081_d_n2;
            locals.var_t5_dn6 = assign9530_body6_e8081_d_n6;
            locals.var_t5_dn7 = assign9530_body6_e8081_d_n7;
            locals.var_t5_dn10 = assign9530_body6_e8081_d_n10;
            locals.var_t5_dn11 = assign9530_body6_e8081_d_n11;
            locals.var_t5_dn12 = assign9530_body6_e8081_d_n12;
            locals.var_t5_dn17 = assign9530_body6_e8081_d_n17;
            locals.var_t5_rv = 0.0;
            let assign9530_body7_e8084: f64 = (-1e-9);
            let assign9530_body7_e8085: f64 = if locals.var_phi_s0_bulk < assign9530_body7_e8084 { 1.0 } else { 0.0 };
            locals.var_guard177 = assign9530_body7_e8085;
            locals.var_guard177_rv = 0.0;
            let (assign9530_body8_e8104, assign9530_body8_e8104_d_n0, assign9530_body8_e8104_d_n2, assign9530_body8_e8104_d_n6, assign9530_body8_e8104_d_n7, assign9530_body8_e8104_d_n10, assign9530_body8_e8104_d_n11, assign9530_body8_e8104_d_n12, assign9530_body8_e8104_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard176 == 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9530_body8_e8098: f64 = (locals.var_t3 + locals.var_t2);
        let assign9530_body8_e8100: f64 = (assign9530_body8_e8098 - 1.0);
        let assign9530_body8_e8101: f64 = (assign9530_body8_e8100).sqrt();
        let assign9530_body8_e8102: f64 = (locals.var_t1 * assign9530_body8_e8101);
        (assign9530_body8_e8102, ((locals.var_t1_dn0 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn0 + locals.var_t2_dn0) / (2.0 * assign9530_body8_e8101)))), ((locals.var_t1_dn2 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn2 + locals.var_t2_dn2) / (2.0 * assign9530_body8_e8101)))), ((locals.var_t1_dn6 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn6 + locals.var_t2_dn6) / (2.0 * assign9530_body8_e8101)))), ((locals.var_t1_dn7 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn7 + locals.var_t2_dn7) / (2.0 * assign9530_body8_e8101)))), ((locals.var_t1_dn10 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn10 + locals.var_t2_dn10) / (2.0 * assign9530_body8_e8101)))), ((locals.var_t1_dn11 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn11 + locals.var_t2_dn11) / (2.0 * assign9530_body8_e8101)))), ((locals.var_t1_dn12 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn12 + locals.var_t2_dn12) / (2.0 * assign9530_body8_e8101)))), ((locals.var_t1_dn17 * assign9530_body8_e8101) + (locals.var_t1 * ((locals.var_t3_dn17 + locals.var_t2_dn17) / (2.0 * assign9530_body8_e8101)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
            locals.var_t4 = assign9530_body8_e8104;
            locals.var_t4_dn0 = assign9530_body8_e8104_d_n0;
            locals.var_t4_dn2 = assign9530_body8_e8104_d_n2;
            locals.var_t4_dn6 = assign9530_body8_e8104_d_n6;
            locals.var_t4_dn7 = assign9530_body8_e8104_d_n7;
            locals.var_t4_dn10 = assign9530_body8_e8104_d_n10;
            locals.var_t4_dn11 = assign9530_body8_e8104_d_n11;
            locals.var_t4_dn12 = assign9530_body8_e8104_d_n12;
            locals.var_t4_dn17 = assign9530_body8_e8104_d_n17;
            locals.var_t4_rv = 0.0;
            let (assign9530_body9_e8123, assign9530_body9_e8123_d_n0, assign9530_body9_e8123_d_n2, assign9530_body9_e8123_d_n6, assign9530_body9_e8123_d_n7, assign9530_body9_e8123_d_n10, assign9530_body9_e8123_d_n11, assign9530_body9_e8123_d_n12, assign9530_body9_e8123_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard176 == 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9530_body9_e8116: f64 = (locals.var_c0bulk / locals.var_t4);
        let assign9530_body9_e8118: f64 = (-locals.var_t3);
        let assign9530_body9_e8120: f64 = (assign9530_body9_e8118 + 1.0);
        let assign9530_body9_e8121: f64 = (assign9530_body9_e8116 * assign9530_body9_e8120);
        (assign9530_body9_e8121, (((-((locals.var_c0bulk * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn0))), (((-((locals.var_c0bulk * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn2))), (((-((locals.var_c0bulk * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn6))), (((-((locals.var_c0bulk * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn7))), (((-((locals.var_c0bulk * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn10))), (((-((locals.var_c0bulk * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn11))), (((-((locals.var_c0bulk * locals.var_t4_dn12) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn12))), (((-((locals.var_c0bulk * locals.var_t4_dn17) / (locals.var_t4 * locals.var_t4))) * assign9530_body9_e8120) + (assign9530_body9_e8116 * (-locals.var_t3_dn17))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
            locals.var_t5 = assign9530_body9_e8123;
            locals.var_t5_dn0 = assign9530_body9_e8123_d_n0;
            locals.var_t5_dn2 = assign9530_body9_e8123_d_n2;
            locals.var_t5_dn6 = assign9530_body9_e8123_d_n6;
            locals.var_t5_dn7 = assign9530_body9_e8123_d_n7;
            locals.var_t5_dn10 = assign9530_body9_e8123_d_n10;
            locals.var_t5_dn11 = assign9530_body9_e8123_d_n11;
            locals.var_t5_dn12 = assign9530_body9_e8123_d_n12;
            locals.var_t5_dn17 = assign9530_body9_e8123_d_n17;
            locals.var_t5_rv = 0.0;
            let (assign9530_body10_e8144, assign9530_body10_e8144_d_n0, assign9530_body10_e8144_d_n2, assign9530_body10_e8144_d_n6, assign9530_body10_e8144_d_n7, assign9530_body10_e8144_d_n10, assign9530_body10_e8144_d_n11, assign9530_body10_e8144_d_n12, assign9530_body10_e8144_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard176 == 0.0)) && (locals.var_guard177 == 0.0)) {
        let assign9530_body10_e8136: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign9530_body10_e8137: f64 = (assign9530_body10_e8136).sqrt();
        let assign9530_body10_e8138: f64 = (-assign9530_body10_e8137);
        let assign9530_body10_e8140: f64 = (assign9530_body10_e8138 * locals.var_beta);
        let assign9530_body10_e8142: f64 = (assign9530_body10_e8140 * locals.var_phi_s0_bulk);
        (assign9530_body10_e8142, (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn0), (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn2), (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn6), (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign9530_body10_e8137))) * locals.var_beta) + (assign9530_body10_e8138 * locals.var_beta_dn10)) * locals.var_phi_s0_bulk) + (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn10)), (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn11), (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn12), (assign9530_body10_e8140 * locals.var_phi_s0_bulk_dn17),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
            locals.var_t4 = assign9530_body10_e8144;
            locals.var_t4_dn0 = assign9530_body10_e8144_d_n0;
            locals.var_t4_dn2 = assign9530_body10_e8144_d_n2;
            locals.var_t4_dn6 = assign9530_body10_e8144_d_n6;
            locals.var_t4_dn7 = assign9530_body10_e8144_d_n7;
            locals.var_t4_dn10 = assign9530_body10_e8144_d_n10;
            locals.var_t4_dn11 = assign9530_body10_e8144_d_n11;
            locals.var_t4_dn12 = assign9530_body10_e8144_d_n12;
            locals.var_t4_dn17 = assign9530_body10_e8144_d_n17;
            locals.var_t4_rv = 0.0;
            let (assign9530_body11_e8161, assign9530_body11_e8161_d_n0, assign9530_body11_e8161_d_n2, assign9530_body11_e8161_d_n6, assign9530_body11_e8161_d_n7, assign9530_body11_e8161_d_n10, assign9530_body11_e8161_d_n11, assign9530_body11_e8161_d_n12, assign9530_body11_e8161_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard176 == 0.0)) && (locals.var_guard177 == 0.0)) {
        let assign9530_body11_e8157: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign9530_body11_e8158: f64 = (assign9530_body11_e8157).sqrt();
        let assign9530_body11_e8159: f64 = (-assign9530_body11_e8158);
        (assign9530_body11_e8159, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign9530_body11_e8158))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
            locals.var_t5 = assign9530_body11_e8161;
            locals.var_t5_dn0 = assign9530_body11_e8161_d_n0;
            locals.var_t5_dn2 = assign9530_body11_e8161_d_n2;
            locals.var_t5_dn6 = assign9530_body11_e8161_d_n6;
            locals.var_t5_dn7 = assign9530_body11_e8161_d_n7;
            locals.var_t5_dn10 = assign9530_body11_e8161_d_n10;
            locals.var_t5_dn11 = assign9530_body11_e8161_d_n11;
            locals.var_t5_dn12 = assign9530_body11_e8161_d_n12;
            locals.var_t5_dn17 = assign9530_body11_e8161_d_n17;
            locals.var_t5_rv = 0.0;
            let (assign9530_body12_e8177, assign9530_body12_e8177_d_n0, assign9530_body12_e8177_d_n2, assign9530_body12_e8177_d_n6, assign9530_body12_e8177_d_n7, assign9530_body12_e8177_d_n10, assign9530_body12_e8177_d_n11, assign9530_body12_e8177_d_n12, assign9530_body12_e8177_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body12_e8168: f64 = (locals.var_t4 * locals.var_t4);
        let assign9530_body12_e8171: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign9530_body12_e8173: f64 = (assign9530_body12_e8171 * locals.var_q_fd_dlt1);
        let assign9530_body12_e8174: f64 = (assign9530_body12_e8168 + assign9530_body12_e8173);
        let assign9530_body12_e8175: f64 = (assign9530_body12_e8174).sqrt();
        (assign9530_body12_e8175, ((((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign9530_body12_e8175)), ((((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign9530_body12_e8175)), ((((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign9530_body12_e8175)), ((((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign9530_body12_e8175)), ((((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign9530_body12_e8175)), ((((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign9530_body12_e8175)), ((((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign9530_body12_e8175)), ((((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign9530_body12_e8171 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign9530_body12_e8175)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9530_body12_e8177;
            locals.var_tmf2_dn0 = assign9530_body12_e8177_d_n0;
            locals.var_tmf2_dn2 = assign9530_body12_e8177_d_n2;
            locals.var_tmf2_dn6 = assign9530_body12_e8177_d_n6;
            locals.var_tmf2_dn7 = assign9530_body12_e8177_d_n7;
            locals.var_tmf2_dn10 = assign9530_body12_e8177_d_n10;
            locals.var_tmf2_dn11 = assign9530_body12_e8177_d_n11;
            locals.var_tmf2_dn12 = assign9530_body12_e8177_d_n12;
            locals.var_tmf2_dn17 = assign9530_body12_e8177_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9530_body13_e8190, assign9530_body13_e8190_d_n0, assign9530_body13_e8190_d_n2, assign9530_body13_e8190_d_n6, assign9530_body13_e8190_d_n7, assign9530_body13_e8190_d_n10, assign9530_body13_e8190_d_n11, assign9530_body13_e8190_d_n12, assign9530_body13_e8190_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body13_e8186: f64 = (locals.var_t4 / locals.var_tmf2);
        let assign9530_body13_e8187: f64 = (1.0 + assign9530_body13_e8186);
        let assign9530_body13_e8188: f64 = (0.5 * assign9530_body13_e8187);
        (assign9530_body13_e8188, (0.5 * (((locals.var_t4_dn0 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn2 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn6 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn7 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn10 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn11 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn12 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn17 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9530_body13_e8190;
            locals.var_t7_dn0 = assign9530_body13_e8190_d_n0;
            locals.var_t7_dn2 = assign9530_body13_e8190_d_n2;
            locals.var_t7_dn6 = assign9530_body13_e8190_d_n6;
            locals.var_t7_dn7 = assign9530_body13_e8190_d_n7;
            locals.var_t7_dn10 = assign9530_body13_e8190_d_n10;
            locals.var_t7_dn11 = assign9530_body13_e8190_d_n11;
            locals.var_t7_dn12 = assign9530_body13_e8190_d_n12;
            locals.var_t7_dn17 = assign9530_body13_e8190_d_n17;
            locals.var_t7_rv = 0.0;
            let (assign9530_body14_e8205, assign9530_body14_e8205_d_n0, assign9530_body14_e8205_d_n2, assign9530_body14_e8205_d_n6, assign9530_body14_e8205_d_n7, assign9530_body14_e8205_d_n10, assign9530_body14_e8205_d_n11, assign9530_body14_e8205_d_n12, assign9530_body14_e8205_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body14_e8198: f64 = (locals.var_t4 + locals.var_tmf2);
        let assign9530_body14_e8199: f64 = (0.5 * assign9530_body14_e8198);
        let assign9530_body14_e8202: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign9530_body14_e8203: f64 = (assign9530_body14_e8199 + assign9530_body14_e8202);
        (assign9530_body14_e8203, ((0.5 * (locals.var_t4_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9530_body14_e8205;
            locals.var_t6_dn0 = assign9530_body14_e8205_d_n0;
            locals.var_t6_dn2 = assign9530_body14_e8205_d_n2;
            locals.var_t6_dn6 = assign9530_body14_e8205_d_n6;
            locals.var_t6_dn7 = assign9530_body14_e8205_d_n7;
            locals.var_t6_dn10 = assign9530_body14_e8205_d_n10;
            locals.var_t6_dn11 = assign9530_body14_e8205_d_n11;
            locals.var_t6_dn12 = assign9530_body14_e8205_d_n12;
            locals.var_t6_dn17 = assign9530_body14_e8205_d_n17;
            locals.var_t6_rv = 0.0;
            let assign9530_body15_e8208: f64 = if locals.var_t6 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard178 = assign9530_body15_e8208;
            locals.var_guard178_rv = 0.0;
            let (assign9530_body16_e8217, assign9530_body16_e8217_d_n0, assign9530_body16_e8217_d_n2, assign9530_body16_e8217_d_n6, assign9530_body16_e8217_d_n7, assign9530_body16_e8217_d_n10, assign9530_body16_e8217_d_n11, assign9530_body16_e8217_d_n12, assign9530_body16_e8217_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard178 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9530_body16_e8217;
            locals.var_t6_dn0 = assign9530_body16_e8217_d_n0;
            locals.var_t6_dn2 = assign9530_body16_e8217_d_n2;
            locals.var_t6_dn6 = assign9530_body16_e8217_d_n6;
            locals.var_t6_dn7 = assign9530_body16_e8217_d_n7;
            locals.var_t6_dn10 = assign9530_body16_e8217_d_n10;
            locals.var_t6_dn11 = assign9530_body16_e8217_d_n11;
            locals.var_t6_dn12 = assign9530_body16_e8217_d_n12;
            locals.var_t6_dn17 = assign9530_body16_e8217_d_n17;
            locals.var_t6_rv = 0.0;
            let (assign9530_body17_e8226, assign9530_body17_e8226_d_n0, assign9530_body17_e8226_d_n2, assign9530_body17_e8226_d_n6, assign9530_body17_e8226_d_n7, assign9530_body17_e8226_d_n10, assign9530_body17_e8226_d_n11, assign9530_body17_e8226_d_n12, assign9530_body17_e8226_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard178 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9530_body17_e8226;
            locals.var_t7_dn0 = assign9530_body17_e8226_d_n0;
            locals.var_t7_dn2 = assign9530_body17_e8226_d_n2;
            locals.var_t7_dn6 = assign9530_body17_e8226_d_n6;
            locals.var_t7_dn7 = assign9530_body17_e8226_d_n7;
            locals.var_t7_dn10 = assign9530_body17_e8226_d_n10;
            locals.var_t7_dn11 = assign9530_body17_e8226_d_n11;
            locals.var_t7_dn12 = assign9530_body17_e8226_d_n12;
            locals.var_t7_dn17 = assign9530_body17_e8226_d_n17;
            locals.var_t7_rv = 0.0;
            let (assign9530_body18_e8238, assign9530_body18_e8238_d_n0, assign9530_body18_e8238_d_n2, assign9530_body18_e8238_d_n6, assign9530_body18_e8238_d_n7, assign9530_body18_e8238_d_n10, assign9530_body18_e8238_d_n11, assign9530_body18_e8238_d_n12, assign9530_body18_e8238_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body18_e8232: f64 = (-locals.var_q_fd_soi);
        let assign9530_body18_e8234: f64 = (assign9530_body18_e8232 - locals.var_t6);
        let assign9530_body18_e8236: f64 = (assign9530_body18_e8234 - locals.var_q_fd_dlt2);
        (assign9530_body18_e8236, (((-locals.var_q_fd_soi_dn0) - locals.var_t6_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign9530_body18_e8238;
            locals.var_tmf1_dn0 = assign9530_body18_e8238_d_n0;
            locals.var_tmf1_dn2 = assign9530_body18_e8238_d_n2;
            locals.var_tmf1_dn6 = assign9530_body18_e8238_d_n6;
            locals.var_tmf1_dn7 = assign9530_body18_e8238_d_n7;
            locals.var_tmf1_dn10 = assign9530_body18_e8238_d_n10;
            locals.var_tmf1_dn11 = assign9530_body18_e8238_d_n11;
            locals.var_tmf1_dn12 = assign9530_body18_e8238_d_n12;
            locals.var_tmf1_dn17 = assign9530_body18_e8238_d_n17;
            locals.var_tmf1_rv = 0.0;
            let (assign9530_body19_e8250, assign9530_body19_e8250_d_n0, assign9530_body19_e8250_d_n2, assign9530_body19_e8250_d_n6, assign9530_body19_e8250_d_n7, assign9530_body19_e8250_d_n10, assign9530_body19_e8250_d_n11, assign9530_body19_e8250_d_n12, assign9530_body19_e8250_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body19_e8245: f64 = (-locals.var_q_fd_soi);
        let assign9530_body19_e8246: f64 = (4.0 * assign9530_body19_e8245);
        let assign9530_body19_e8248: f64 = (assign9530_body19_e8246 * locals.var_q_fd_dlt2);
        (assign9530_body19_e8248, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign9530_body19_e8246 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9530_body19_e8250;
            locals.var_tmf2_dn0 = assign9530_body19_e8250_d_n0;
            locals.var_tmf2_dn2 = assign9530_body19_e8250_d_n2;
            locals.var_tmf2_dn6 = assign9530_body19_e8250_d_n6;
            locals.var_tmf2_dn7 = assign9530_body19_e8250_d_n7;
            locals.var_tmf2_dn10 = assign9530_body19_e8250_d_n10;
            locals.var_tmf2_dn11 = assign9530_body19_e8250_d_n11;
            locals.var_tmf2_dn12 = assign9530_body19_e8250_d_n12;
            locals.var_tmf2_dn17 = assign9530_body19_e8250_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9530_body20_e8263, assign9530_body20_e8263_d_n0, assign9530_body20_e8263_d_n2, assign9530_body20_e8263_d_n6, assign9530_body20_e8263_d_n7, assign9530_body20_e8263_d_n10, assign9530_body20_e8263_d_n11, assign9530_body20_e8263_d_n12, assign9530_body20_e8263_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let (assign9530_body20_e8261, assign9530_body20_e8261_d_n0, assign9530_body20_e8261_d_n2, assign9530_body20_e8261_d_n6, assign9530_body20_e8261_d_n7, assign9530_body20_e8261_d_n10, assign9530_body20_e8261_d_n11, assign9530_body20_e8261_d_n12, assign9530_body20_e8261_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign9530_body20_e8260: f64 = (-locals.var_tmf2);
                (assign9530_body20_e8260, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign9530_body20_e8261, assign9530_body20_e8261_d_n0, assign9530_body20_e8261_d_n2, assign9530_body20_e8261_d_n6, assign9530_body20_e8261_d_n7, assign9530_body20_e8261_d_n10, assign9530_body20_e8261_d_n11, assign9530_body20_e8261_d_n12, assign9530_body20_e8261_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9530_body20_e8263;
            locals.var_tmf2_dn0 = assign9530_body20_e8263_d_n0;
            locals.var_tmf2_dn2 = assign9530_body20_e8263_d_n2;
            locals.var_tmf2_dn6 = assign9530_body20_e8263_d_n6;
            locals.var_tmf2_dn7 = assign9530_body20_e8263_d_n7;
            locals.var_tmf2_dn10 = assign9530_body20_e8263_d_n10;
            locals.var_tmf2_dn11 = assign9530_body20_e8263_d_n11;
            locals.var_tmf2_dn12 = assign9530_body20_e8263_d_n12;
            locals.var_tmf2_dn17 = assign9530_body20_e8263_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9530_body21_e8275, assign9530_body21_e8275_d_n0, assign9530_body21_e8275_d_n2, assign9530_body21_e8275_d_n6, assign9530_body21_e8275_d_n7, assign9530_body21_e8275_d_n10, assign9530_body21_e8275_d_n11, assign9530_body21_e8275_d_n12, assign9530_body21_e8275_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body21_e8270: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9530_body21_e8272: f64 = (assign9530_body21_e8270 + locals.var_tmf2);
        let assign9530_body21_e8273: f64 = (assign9530_body21_e8272).sqrt();
        (assign9530_body21_e8273, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9530_body21_e8273)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9530_body21_e8273)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign9530_body21_e8273)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign9530_body21_e8273)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign9530_body21_e8273)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign9530_body21_e8273)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign9530_body21_e8273)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign9530_body21_e8273)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9530_body21_e8275;
            locals.var_tmf2_dn0 = assign9530_body21_e8275_d_n0;
            locals.var_tmf2_dn2 = assign9530_body21_e8275_d_n2;
            locals.var_tmf2_dn6 = assign9530_body21_e8275_d_n6;
            locals.var_tmf2_dn7 = assign9530_body21_e8275_d_n7;
            locals.var_tmf2_dn10 = assign9530_body21_e8275_d_n10;
            locals.var_tmf2_dn11 = assign9530_body21_e8275_d_n11;
            locals.var_tmf2_dn12 = assign9530_body21_e8275_d_n12;
            locals.var_tmf2_dn17 = assign9530_body21_e8275_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9530_body22_e8288, assign9530_body22_e8288_d_n0, assign9530_body22_e8288_d_n2, assign9530_body22_e8288_d_n6, assign9530_body22_e8288_d_n7, assign9530_body22_e8288_d_n10, assign9530_body22_e8288_d_n11, assign9530_body22_e8288_d_n12, assign9530_body22_e8288_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body22_e8284: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign9530_body22_e8285: f64 = (1.0 + assign9530_body22_e8284);
        let assign9530_body22_e8286: f64 = (0.5 * assign9530_body22_e8285);
        (assign9530_body22_e8286, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign9530_body22_e8288;
            locals.var_t8_dn0 = assign9530_body22_e8288_d_n0;
            locals.var_t8_dn2 = assign9530_body22_e8288_d_n2;
            locals.var_t8_dn6 = assign9530_body22_e8288_d_n6;
            locals.var_t8_dn7 = assign9530_body22_e8288_d_n7;
            locals.var_t8_dn10 = assign9530_body22_e8288_d_n10;
            locals.var_t8_dn11 = assign9530_body22_e8288_d_n11;
            locals.var_t8_dn12 = assign9530_body22_e8288_d_n12;
            locals.var_t8_dn17 = assign9530_body22_e8288_d_n17;
            locals.var_t8_rv = 0.0;
            let (assign9530_body23_e8302, assign9530_body23_e8302_d_n0, assign9530_body23_e8302_d_n2, assign9530_body23_e8302_d_n6, assign9530_body23_e8302_d_n7, assign9530_body23_e8302_d_n10, assign9530_body23_e8302_d_n11, assign9530_body23_e8302_d_n12, assign9530_body23_e8302_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body23_e8294: f64 = (-locals.var_q_fd_soi);
        let assign9530_body23_e8298: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9530_body23_e8299: f64 = (0.5 * assign9530_body23_e8298);
        let assign9530_body23_e8300: f64 = (assign9530_body23_e8294 - assign9530_body23_e8299);
        (assign9530_body23_e8300, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9530_body23_e8302;
            locals.var_t6_dn0 = assign9530_body23_e8302_d_n0;
            locals.var_t6_dn2 = assign9530_body23_e8302_d_n2;
            locals.var_t6_dn6 = assign9530_body23_e8302_d_n6;
            locals.var_t6_dn7 = assign9530_body23_e8302_d_n7;
            locals.var_t6_dn10 = assign9530_body23_e8302_d_n10;
            locals.var_t6_dn11 = assign9530_body23_e8302_d_n11;
            locals.var_t6_dn12 = assign9530_body23_e8302_d_n12;
            locals.var_t6_dn17 = assign9530_body23_e8302_d_n17;
            locals.var_t6_rv = 0.0;
            let (assign9530_body24_e8313, assign9530_body24_e8313_d_n0, assign9530_body24_e8313_d_n2, assign9530_body24_e8313_d_n6, assign9530_body24_e8313_d_n7, assign9530_body24_e8313_d_n10, assign9530_body24_e8313_d_n11, assign9530_body24_e8313_d_n12, assign9530_body24_e8313_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body24_e8310: f64 = (locals.var_t5 * locals.var_t8);
        let assign9530_body24_e8311: f64 = (locals.var_t7 * assign9530_body24_e8310);
        (assign9530_body24_e8311, ((locals.var_t7_dn0 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn0 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn0)))), ((locals.var_t7_dn2 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn2 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn2)))), ((locals.var_t7_dn6 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn6 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn6)))), ((locals.var_t7_dn7 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn7 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn7)))), ((locals.var_t7_dn10 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn10 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn10)))), ((locals.var_t7_dn11 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn11 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn11)))), ((locals.var_t7_dn12 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn12 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn12)))), ((locals.var_t7_dn17 * assign9530_body24_e8310) + (locals.var_t7 * ((locals.var_t5_dn17 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9530_body24_e8313;
            locals.var_t7_dn0 = assign9530_body24_e8313_d_n0;
            locals.var_t7_dn2 = assign9530_body24_e8313_d_n2;
            locals.var_t7_dn6 = assign9530_body24_e8313_d_n6;
            locals.var_t7_dn7 = assign9530_body24_e8313_d_n7;
            locals.var_t7_dn10 = assign9530_body24_e8313_d_n10;
            locals.var_t7_dn11 = assign9530_body24_e8313_d_n11;
            locals.var_t7_dn12 = assign9530_body24_e8313_d_n12;
            locals.var_t7_dn17 = assign9530_body24_e8313_d_n17;
            locals.var_t7_rv = 0.0;
            let (assign9530_body25_e8330, assign9530_body25_e8330_d_n0, assign9530_body25_e8330_d_n2, assign9530_body25_e8330_d_n6, assign9530_body25_e8330_d_n7, assign9530_body25_e8330_d_n10, assign9530_body25_e8330_d_n11, assign9530_body25_e8330_d_n12, assign9530_body25_e8330_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body25_e8320: f64 = (locals.var_t6 * locals.var_t6);
        let assign9530_body25_e8322: f64 = (assign9530_body25_e8320 / 2.0);
        let assign9530_body25_e8324: f64 = (assign9530_body25_e8322 / 1.034943e-10);
        let assign9530_body25_e8326: f64 = (assign9530_body25_e8324 / 1.6021918e-19);
        let assign9530_body25_e8328: f64 = (assign9530_body25_e8326 / locals.var_uc_nsubs);
        (assign9530_body25_e8328, ((((((((locals.var_t6_dn0 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn2 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn12 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn17 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9530_body25_e8326 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign9530_body25_e8330;
            locals.var_phi_b_dep_dn0 = assign9530_body25_e8330_d_n0;
            locals.var_phi_b_dep_dn2 = assign9530_body25_e8330_d_n2;
            locals.var_phi_b_dep_dn6 = assign9530_body25_e8330_d_n6;
            locals.var_phi_b_dep_dn7 = assign9530_body25_e8330_d_n7;
            locals.var_phi_b_dep_dn10 = assign9530_body25_e8330_d_n10;
            locals.var_phi_b_dep_dn11 = assign9530_body25_e8330_d_n11;
            locals.var_phi_b_dep_dn12 = assign9530_body25_e8330_d_n12;
            locals.var_phi_b_dep_dn17 = assign9530_body25_e8330_d_n17;
            locals.var_phi_b_dep_rv = 0.0;
            let (assign9530_body26_e8343, assign9530_body26_e8343_d_n0, assign9530_body26_e8343_d_n2, assign9530_body26_e8343_d_n6, assign9530_body26_e8343_d_n7, assign9530_body26_e8343_d_n10, assign9530_body26_e8343_d_n11, assign9530_body26_e8343_d_n12, assign9530_body26_e8343_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body26_e8337: f64 = (2.0 * locals.var_phi_b_dep);
        let assign9530_body26_e8339: f64 = (assign9530_body26_e8337 * locals.var_t7);
        let assign9530_body26_e8341: f64 = (assign9530_body26_e8339 / locals.var_t6);
        (assign9530_body26_e8341, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn0)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn2)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn6)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn7)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn10)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn11)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn12)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn12)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7) + (assign9530_body26_e8337 * locals.var_t7_dn17)) * locals.var_t6) - (assign9530_body26_e8339 * locals.var_t6_dn17)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign9530_body26_e8343;
            locals.var_phi_b_dep_dpsb_dn0 = assign9530_body26_e8343_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign9530_body26_e8343_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign9530_body26_e8343_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign9530_body26_e8343_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign9530_body26_e8343_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign9530_body26_e8343_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign9530_body26_e8343_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign9530_body26_e8343_d_n17;
            locals.var_phi_b_dep_dpsb_rv = 0.0;
            let (assign9530_body27_e8387, assign9530_body27_e8387_d_n0, assign9530_body27_e8387_d_n2, assign9530_body27_e8387_d_n6, assign9530_body27_e8387_d_n7, assign9530_body27_e8387_d_n10, assign9530_body27_e8387_d_n11, assign9530_body27_e8387_d_n12, assign9530_body27_e8387_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body27_e8351: f64 = (locals.var_phi_s0_soi - locals.var_phi_s0_bulk);
        let assign9530_body27_e8354: f64 = (locals.var_t4 / locals.var_c_box);
        let assign9530_body27_e8355: f64 = (assign9530_body27_e8351 + assign9530_body27_e8354);
        let assign9530_body27_e8359: f64 = (locals.var_q_fd_soi / 2.0);
        let assign9530_body27_e8360: f64 = (locals.var_t4 + assign9530_body27_e8359);
        let assign9530_body27_e8362: f64 = (assign9530_body27_e8360 * locals.var_t_soi);
        let assign9530_body27_e8364: f64 = (assign9530_body27_e8362 / 1.034943e-10);
        let assign9530_body27_e8365: f64 = (assign9530_body27_e8355 + assign9530_body27_e8364);
        let assign9530_body27_e8367: f64 = (assign9530_body27_e8365 - locals.var_vbsbiz);
        let assign9530_body27_e8369: f64 = (assign9530_body27_e8367 + locals.var_phi_b_dep);
        let assign9530_body27_e8371: f64 = (-1.0);
        let assign9530_body27_e8374: f64 = (locals.var_t5 / locals.var_c_box);
        let assign9530_body27_e8375: f64 = (assign9530_body27_e8371 + assign9530_body27_e8374);
        let assign9530_body27_e8378: f64 = (locals.var_t5 * locals.var_t_soi);
        let assign9530_body27_e8380: f64 = (assign9530_body27_e8378 / 1.034943e-10);
        let assign9530_body27_e8381: f64 = (assign9530_body27_e8375 + assign9530_body27_e8380);
        let assign9530_body27_e8383: f64 = (assign9530_body27_e8381 + locals.var_phi_b_dep_dpsb);
        let assign9530_body27_e8384: f64 = (assign9530_body27_e8369 / assign9530_body27_e8383);
        let assign9530_body27_e8385: f64 = (locals.var_phi_s0_bulk - assign9530_body27_e8384);
        (assign9530_body27_e8385, (locals.var_phi_s0_bulk_dn0 - ((((((((locals.var_phi_s0_soi_dn0 - locals.var_phi_s0_bulk_dn0) + (locals.var_t4_dn0 / locals.var_c_box)) + (((locals.var_t4_dn0 + (locals.var_q_fd_soi_dn0 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn0 / locals.var_c_box) + ((locals.var_t5_dn0 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn0))) / (assign9530_body27_e8383 * assign9530_body27_e8383))), (locals.var_phi_s0_bulk_dn2 - ((((((((locals.var_phi_s0_soi_dn2 - locals.var_phi_s0_bulk_dn2) + (locals.var_t4_dn2 / locals.var_c_box)) + (((locals.var_t4_dn2 + (locals.var_q_fd_soi_dn2 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn2 / locals.var_c_box) + ((locals.var_t5_dn2 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn2))) / (assign9530_body27_e8383 * assign9530_body27_e8383))), (locals.var_phi_s0_bulk_dn6 - ((((((((locals.var_phi_s0_soi_dn6 - locals.var_phi_s0_bulk_dn6) + (locals.var_t4_dn6 / locals.var_c_box)) + (((locals.var_t4_dn6 + (locals.var_q_fd_soi_dn6 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn6 / locals.var_c_box) + ((locals.var_t5_dn6 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn6))) / (assign9530_body27_e8383 * assign9530_body27_e8383))), (locals.var_phi_s0_bulk_dn7 - ((((((((locals.var_phi_s0_soi_dn7 - locals.var_phi_s0_bulk_dn7) + (locals.var_t4_dn7 / locals.var_c_box)) + (((locals.var_t4_dn7 + (locals.var_q_fd_soi_dn7 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn7 / locals.var_c_box) + ((locals.var_t5_dn7 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn7))) / (assign9530_body27_e8383 * assign9530_body27_e8383))), (locals.var_phi_s0_bulk_dn10 - ((((((((locals.var_phi_s0_soi_dn10 - locals.var_phi_s0_bulk_dn10) + (locals.var_t4_dn10 / locals.var_c_box)) + (((locals.var_t4_dn10 + (locals.var_q_fd_soi_dn10 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn10 / locals.var_c_box) + ((locals.var_t5_dn10 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn10))) / (assign9530_body27_e8383 * assign9530_body27_e8383))), (locals.var_phi_s0_bulk_dn11 - ((((((((locals.var_phi_s0_soi_dn11 - locals.var_phi_s0_bulk_dn11) + (locals.var_t4_dn11 / locals.var_c_box)) + (((locals.var_t4_dn11 + (locals.var_q_fd_soi_dn11 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn11 / locals.var_c_box) + ((locals.var_t5_dn11 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn11))) / (assign9530_body27_e8383 * assign9530_body27_e8383))), (locals.var_phi_s0_bulk_dn12 - ((((((((locals.var_phi_s0_soi_dn12 - locals.var_phi_s0_bulk_dn12) + (locals.var_t4_dn12 / locals.var_c_box)) + (((locals.var_t4_dn12 + (locals.var_q_fd_soi_dn12 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn12 / locals.var_c_box) + ((locals.var_t5_dn12 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn12))) / (assign9530_body27_e8383 * assign9530_body27_e8383))), (locals.var_phi_s0_bulk_dn17 - ((((((((locals.var_phi_s0_soi_dn17 - locals.var_phi_s0_bulk_dn17) + (locals.var_t4_dn17 / locals.var_c_box)) + (((locals.var_t4_dn17 + (locals.var_q_fd_soi_dn17 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign9530_body27_e8383) - (assign9530_body27_e8369 * (((locals.var_t5_dn17 / locals.var_c_box) + ((locals.var_t5_dn17 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn17))) / (assign9530_body27_e8383 * assign9530_body27_e8383))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9530_body27_e8387;
            locals.var_t6_dn0 = assign9530_body27_e8387_d_n0;
            locals.var_t6_dn2 = assign9530_body27_e8387_d_n2;
            locals.var_t6_dn6 = assign9530_body27_e8387_d_n6;
            locals.var_t6_dn7 = assign9530_body27_e8387_d_n7;
            locals.var_t6_dn10 = assign9530_body27_e8387_d_n10;
            locals.var_t6_dn11 = assign9530_body27_e8387_d_n11;
            locals.var_t6_dn12 = assign9530_body27_e8387_d_n12;
            locals.var_t6_dn17 = assign9530_body27_e8387_d_n17;
            locals.var_t6_rv = 0.0;
            let (assign9530_body28_e8394, assign9530_body28_e8394_d_n0, assign9530_body28_e8394_d_n2, assign9530_body28_e8394_d_n6, assign9530_body28_e8394_d_n7, assign9530_body28_e8394_d_n10, assign9530_body28_e8394_d_n11, assign9530_body28_e8394_d_n12, assign9530_body28_e8394_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        (locals.var_lp_s0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9530_body28_e8394;
            locals.var_t7_dn0 = assign9530_body28_e8394_d_n0;
            locals.var_t7_dn2 = assign9530_body28_e8394_d_n2;
            locals.var_t7_dn6 = assign9530_body28_e8394_d_n6;
            locals.var_t7_dn7 = assign9530_body28_e8394_d_n7;
            locals.var_t7_dn10 = assign9530_body28_e8394_d_n10;
            locals.var_t7_dn11 = assign9530_body28_e8394_d_n11;
            locals.var_t7_dn12 = assign9530_body28_e8394_d_n12;
            locals.var_t7_dn17 = assign9530_body28_e8394_d_n17;
            locals.var_t7_rv = 0.0;
            let assign9530_body29_e8397: f64 = (locals.var_t6 - locals.var_phi_s0_bulk);
            let assign9530_body29_e8398: f64 = (assign9530_body29_e8397).abs();
            let assign9530_body29_e8400: f64 = if assign9530_body29_e8398 < 0.001 { 1.0 } else { 0.0 };
            locals.var_guard179 = assign9530_body29_e8400;
            locals.var_guard179_rv = 0.0;
            let (assign9530_body30_e8409,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) && (locals.var_guard179 != 0.0)) {
        (locals.var_lp_s0_max,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign9530_body30_e8409;
            locals.var_lp_s0_rv = 0.0;
            let (assign9530_body31_e8416, assign9530_body31_e8416_d_n0, assign9530_body31_e8416_d_n2, assign9530_body31_e8416_d_n6, assign9530_body31_e8416_d_n7, assign9530_body31_e8416_d_n10, assign9530_body31_e8416_d_n11, assign9530_body31_e8416_d_n12, assign9530_body31_e8416_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
            locals.var_phi_s0_bulk = assign9530_body31_e8416;
            locals.var_phi_s0_bulk_dn0 = assign9530_body31_e8416_d_n0;
            locals.var_phi_s0_bulk_dn2 = assign9530_body31_e8416_d_n2;
            locals.var_phi_s0_bulk_dn6 = assign9530_body31_e8416_d_n6;
            locals.var_phi_s0_bulk_dn7 = assign9530_body31_e8416_d_n7;
            locals.var_phi_s0_bulk_dn10 = assign9530_body31_e8416_d_n10;
            locals.var_phi_s0_bulk_dn11 = assign9530_body31_e8416_d_n11;
            locals.var_phi_s0_bulk_dn12 = assign9530_body31_e8416_d_n12;
            locals.var_phi_s0_bulk_dn17 = assign9530_body31_e8416_d_n17;
            locals.var_phi_s0_bulk_rv = 0.0;
            let (assign9530_body32_e8423, assign9530_body32_e8423_d_n0, assign9530_body32_e8423_d_n2, assign9530_body32_e8423_d_n6, assign9530_body32_e8423_d_n7, assign9530_body32_e8423_d_n10, assign9530_body32_e8423_d_n11, assign9530_body32_e8423_d_n12, assign9530_body32_e8423_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_q_s0_bulk, locals.var_q_s0_bulk_dn0, locals.var_q_s0_bulk_dn2, locals.var_q_s0_bulk_dn6, locals.var_q_s0_bulk_dn7, locals.var_q_s0_bulk_dn10, locals.var_q_s0_bulk_dn11, locals.var_q_s0_bulk_dn12, locals.var_q_s0_bulk_dn17,)
    }
};
            locals.var_q_s0_bulk = assign9530_body32_e8423;
            locals.var_q_s0_bulk_dn0 = assign9530_body32_e8423_d_n0;
            locals.var_q_s0_bulk_dn2 = assign9530_body32_e8423_d_n2;
            locals.var_q_s0_bulk_dn6 = assign9530_body32_e8423_d_n6;
            locals.var_q_s0_bulk_dn7 = assign9530_body32_e8423_d_n7;
            locals.var_q_s0_bulk_dn10 = assign9530_body32_e8423_d_n10;
            locals.var_q_s0_bulk_dn11 = assign9530_body32_e8423_d_n11;
            locals.var_q_s0_bulk_dn12 = assign9530_body32_e8423_d_n12;
            locals.var_q_s0_bulk_dn17 = assign9530_body32_e8423_d_n17;
            locals.var_q_s0_bulk_rv = 0.0;
            let (assign9530_body33_e8432,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9530_body33_e8430: f64 = (locals.var_lp_s0 + 1.0);
        (assign9530_body33_e8430,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign9530_body33_e8432;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9540_e8441, assign9540_e8441_d_n0, assign9540_e8441_d_n2, assign9540_e8441_d_n6, assign9540_e8441_d_n7, assign9540_e8441_d_n10, assign9540_e8441_d_n11, assign9540_e8441_d_n12, assign9540_e8441_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9540_e8439: f64 = (locals.var_vbsbiz + locals.var_phi_s0_bulk);
        (assign9540_e8439, (locals.var_vbsbiz_dn0 + locals.var_phi_s0_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_s0_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_s0_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_s0_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_s0_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_s0_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_s0_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_s0_bulk_dn17),)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9540_e8441;
        locals.var_phi_s0_bulk_dn0 = assign9540_e8441_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9540_e8441_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9540_e8441_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9540_e8441_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9540_e8441_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9540_e8441_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9540_e8441_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9540_e8441_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9550_e8456, assign9550_e8456_d_n0, assign9550_e8456_d_n2, assign9550_e8456_d_n6, assign9550_e8456_d_n7, assign9550_e8456_d_n10, assign9550_e8456_d_n11, assign9550_e8456_d_n12, assign9550_e8456_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard162 == 0.0)) {
        let assign9550_e8450: f64 = (0.5 * locals.var_q_fd_soi);
        let assign9550_e8452: f64 = (assign9550_e8450 + locals.var_q_s0_bulk);
        let assign9550_e8453: f64 = (locals.var_c_soi_inv__blk115 * assign9550_e8452);
        let assign9550_e8454: f64 = (locals.var_phi_s0_soi + assign9550_e8453);
        (assign9550_e8454, (locals.var_phi_s0_soi_dn0 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_s0_bulk_dn0))), (locals.var_phi_s0_soi_dn2 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_s0_bulk_dn2))), (locals.var_phi_s0_soi_dn6 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_s0_bulk_dn6))), (locals.var_phi_s0_soi_dn7 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_s0_bulk_dn7))), (locals.var_phi_s0_soi_dn10 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_s0_bulk_dn10))), (locals.var_phi_s0_soi_dn11 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_s0_bulk_dn11))), (locals.var_phi_s0_soi_dn12 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_s0_bulk_dn12))), (locals.var_phi_s0_soi_dn17 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_s0_bulk_dn17))),)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    }
};
        locals.var_phi_b0_soi = assign9550_e8456;
        locals.var_phi_b0_soi_dn0 = assign9550_e8456_d_n0;
        locals.var_phi_b0_soi_dn2 = assign9550_e8456_d_n2;
        locals.var_phi_b0_soi_dn6 = assign9550_e8456_d_n6;
        locals.var_phi_b0_soi_dn7 = assign9550_e8456_d_n7;
        locals.var_phi_b0_soi_dn10 = assign9550_e8456_d_n10;
        locals.var_phi_b0_soi_dn11 = assign9550_e8456_d_n11;
        locals.var_phi_b0_soi_dn12 = assign9550_e8456_d_n12;
        locals.var_phi_b0_soi_dn17 = assign9550_e8456_d_n17;
        locals.var_phi_b0_soi_rv = 0.0;

        let assign9560_e8463: f64 = (locals.var_vgs_fb + 0.2);
        let assign9560_e8465: f64 = if ((p.p25 == 1.0) && (locals.var_vgs > assign9560_e8463)) { 1.0 } else { 0.0 };
        locals.var_guard180 = assign9560_e8465;
        locals.var_guard180_rv = 0.0;

        let (assign9570_e8471,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        (locals.var_vfbsub0,)
    } else {
        (locals.var_vfbsub1,)
    }
};
        locals.var_vfbsub1 = assign9570_e8471;
        locals.var_vfbsub1_rv = 0.0;

        let (assign9580_e8483, assign9580_e8483_d_n0, assign9580_e8483_d_n2, assign9580_e8483_d_n6, assign9580_e8483_d_n7, assign9580_e8483_d_n10, assign9580_e8483_d_n11, assign9580_e8483_d_n12, assign9580_e8483_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9580_e8477: f64 = (locals.var_vgsz - locals.var_vfbsub1);
        let assign9580_e8479: f64 = (assign9580_e8477 + locals.var_dvth);
        let assign9580_e8481: f64 = (assign9580_e8479 - locals.var_dppg);
        (assign9580_e8481, ((locals.var_vgsz_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_vgsz_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    }
};
        locals.var_vgpsub = assign9580_e8483;
        locals.var_vgpsub_dn0 = assign9580_e8483_d_n0;
        locals.var_vgpsub_dn2 = assign9580_e8483_d_n2;
        locals.var_vgpsub_dn6 = assign9580_e8483_d_n6;
        locals.var_vgpsub_dn7 = assign9580_e8483_d_n7;
        locals.var_vgpsub_dn10 = assign9580_e8483_d_n10;
        locals.var_vgpsub_dn11 = assign9580_e8483_d_n11;
        locals.var_vgpsub_dn12 = assign9580_e8483_d_n12;
        locals.var_vgpsub_dn17 = assign9580_e8483_d_n17;
        locals.var_vgpsub_rv = 0.0;

        let (assign9590_e8489,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        (p.p137,)
    } else {
        (locals.var_sti2_dlt,)
    }
};
        locals.var_sti2_dlt = assign9590_e8489;
        locals.var_sti2_dlt_rv = 0.0;

        let (assign9600_e8495, assign9600_e8495_d_n0, assign9600_e8495_d_n2, assign9600_e8495_d_n6, assign9600_e8495_d_n7, assign9600_e8495_d_n10, assign9600_e8495_d_n11, assign9600_e8495_d_n12, assign9600_e8495_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn12, locals.var_vgssti_dn17,)
    }
};
        locals.var_vgssti = assign9600_e8495;
        locals.var_vgssti_dn0 = assign9600_e8495_d_n0;
        locals.var_vgssti_dn2 = assign9600_e8495_d_n2;
        locals.var_vgssti_dn6 = assign9600_e8495_d_n6;
        locals.var_vgssti_dn7 = assign9600_e8495_d_n7;
        locals.var_vgssti_dn10 = assign9600_e8495_d_n10;
        locals.var_vgssti_dn11 = assign9600_e8495_d_n11;
        locals.var_vgssti_dn12 = assign9600_e8495_d_n12;
        locals.var_vgssti_dn17 = assign9600_e8495_d_n17;
        locals.var_vgssti_rv = 0.0;

        let (assign9610_e8510, assign9610_e8510_d_n0, assign9610_e8510_d_n2, assign9610_e8510_d_n6, assign9610_e8510_d_n7, assign9610_e8510_d_n10, assign9610_e8510_d_n11, assign9610_e8510_d_n12, assign9610_e8510_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9610_e8501: f64 = (2.0 * 1.6021918e-19);
        let assign9610_e8503: f64 = (assign9610_e8501 * locals.var_uc_nsubs);
        let assign9610_e8505: f64 = (assign9610_e8503 * 1.034943e-10);
        let assign9610_e8507: f64 = (assign9610_e8505 / locals.var_beta);
        let assign9610_e8508: f64 = (assign9610_e8507).sqrt();
        (assign9610_e8508, ((((assign9610_e8501 * locals.var_uc_nsubs_dn0) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9610_e8508)), ((((assign9610_e8501 * locals.var_uc_nsubs_dn2) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9610_e8508)), ((((assign9610_e8501 * locals.var_uc_nsubs_dn6) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9610_e8508)), ((((assign9610_e8501 * locals.var_uc_nsubs_dn7) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9610_e8508)), ((((((assign9610_e8501 * locals.var_uc_nsubs_dn10) * 1.034943e-10) * locals.var_beta) - (assign9610_e8505 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign9610_e8508)), ((((assign9610_e8501 * locals.var_uc_nsubs_dn11) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9610_e8508)), ((((assign9610_e8501 * locals.var_uc_nsubs_dn12) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9610_e8508)), ((((assign9610_e8501 * locals.var_uc_nsubs_dn17) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9610_e8508)),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn10, locals.var_costi0_dn11, locals.var_costi0_dn12, locals.var_costi0_dn17,)
    }
};
        locals.var_costi0 = assign9610_e8510;
        locals.var_costi0_dn0 = assign9610_e8510_d_n0;
        locals.var_costi0_dn2 = assign9610_e8510_d_n2;
        locals.var_costi0_dn6 = assign9610_e8510_d_n6;
        locals.var_costi0_dn7 = assign9610_e8510_d_n7;
        locals.var_costi0_dn10 = assign9610_e8510_d_n10;
        locals.var_costi0_dn11 = assign9610_e8510_d_n11;
        locals.var_costi0_dn12 = assign9610_e8510_d_n12;
        locals.var_costi0_dn17 = assign9610_e8510_d_n17;
        locals.var_costi0_rv = 0.0;

        let (assign9620_e8522, assign9620_e8522_d_n0, assign9620_e8522_d_n2, assign9620_e8522_d_n6, assign9620_e8522_d_n7, assign9620_e8522_d_n10, assign9620_e8522_d_n11, assign9620_e8522_d_n12, assign9620_e8522_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9620_e8516: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_nsubs;
        let assign9620_e8518: f64 = (assign9620_e8516 * __rspice_inv_cse_0);
        let assign9620_e8520: f64 = (assign9620_e8518 * __rspice_inv_cse_0);
        (assign9620_e8520, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn12 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn12)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn17 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn17)) * locals.var_uc_nsubs) - (assign9620_e8516 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9620_e8518 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn10, locals.var_costi1_dn11, locals.var_costi1_dn12, locals.var_costi1_dn17,)
    }
};
        locals.var_costi1 = assign9620_e8522;
        locals.var_costi1_dn0 = assign9620_e8522_d_n0;
        locals.var_costi1_dn2 = assign9620_e8522_d_n2;
        locals.var_costi1_dn6 = assign9620_e8522_d_n6;
        locals.var_costi1_dn7 = assign9620_e8522_d_n7;
        locals.var_costi1_dn10 = assign9620_e8522_d_n10;
        locals.var_costi1_dn11 = assign9620_e8522_d_n11;
        locals.var_costi1_dn12 = assign9620_e8522_d_n12;
        locals.var_costi1_dn17 = assign9620_e8522_d_n17;
        locals.var_costi1_rv = 0.0;

        let (assign9630_e8534, assign9630_e8534_d_n0, assign9630_e8534_d_n2, assign9630_e8534_d_n6, assign9630_e8534_d_n7, assign9630_e8534_d_n10, assign9630_e8534_d_n11, assign9630_e8534_d_n12, assign9630_e8534_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9630_e8528: f64 = (locals.var_costi0 * locals.var_costi0);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_c_fox;
        let assign9630_e8530: f64 = (assign9630_e8528 * __rspice_inv_cse_1);
        let assign9630_e8532: f64 = (assign9630_e8530 * __rspice_inv_cse_1);
        (assign9630_e8532, ((((((((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn12 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn12)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn17 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn17)) * locals.var_c_fox) - (assign9630_e8528 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9630_e8530 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn12, locals.var_costi3_dn17,)
    }
};
        locals.var_costi3 = assign9630_e8534;
        locals.var_costi3_dn0 = assign9630_e8534_d_n0;
        locals.var_costi3_dn2 = assign9630_e8534_d_n2;
        locals.var_costi3_dn6 = assign9630_e8534_d_n6;
        locals.var_costi3_dn7 = assign9630_e8534_d_n7;
        locals.var_costi3_dn10 = assign9630_e8534_d_n10;
        locals.var_costi3_dn11 = assign9630_e8534_d_n11;
        locals.var_costi3_dn12 = assign9630_e8534_d_n12;
        locals.var_costi3_dn17 = assign9630_e8534_d_n17;
        locals.var_costi3_rv = 0.0;

        let (assign9640_e8544, assign9640_e8544_d_n0, assign9640_e8544_d_n2, assign9640_e8544_d_n6, assign9640_e8544_d_n7, assign9640_e8544_d_n10, assign9640_e8544_d_n11, assign9640_e8544_d_n12, assign9640_e8544_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9640_e8540: f64 = (locals.var_costi3 * locals.var_beta);
        let assign9640_e8542: f64 = (assign9640_e8540 / 2.0);
        (assign9640_e8542, ((locals.var_costi3_dn0 * locals.var_beta) / 2.0), ((locals.var_costi3_dn2 * locals.var_beta) / 2.0), ((locals.var_costi3_dn6 * locals.var_beta) / 2.0), ((locals.var_costi3_dn7 * locals.var_beta) / 2.0), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) / 2.0), ((locals.var_costi3_dn11 * locals.var_beta) / 2.0), ((locals.var_costi3_dn12 * locals.var_beta) / 2.0), ((locals.var_costi3_dn17 * locals.var_beta) / 2.0),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn12, locals.var_costi4_dn17,)
    }
};
        locals.var_costi4 = assign9640_e8544;
        locals.var_costi4_dn0 = assign9640_e8544_d_n0;
        locals.var_costi4_dn2 = assign9640_e8544_d_n2;
        locals.var_costi4_dn6 = assign9640_e8544_d_n6;
        locals.var_costi4_dn7 = assign9640_e8544_d_n7;
        locals.var_costi4_dn10 = assign9640_e8544_d_n10;
        locals.var_costi4_dn11 = assign9640_e8544_d_n11;
        locals.var_costi4_dn12 = assign9640_e8544_d_n12;
        locals.var_costi4_dn17 = assign9640_e8544_d_n17;
        locals.var_costi4_rv = 0.0;

        let (assign9650_e8554, assign9650_e8554_d_n0, assign9650_e8554_d_n2, assign9650_e8554_d_n6, assign9650_e8554_d_n7, assign9650_e8554_d_n10, assign9650_e8554_d_n11, assign9650_e8554_d_n12, assign9650_e8554_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9650_e8550: f64 = (locals.var_costi4 * locals.var_beta);
        let assign9650_e8552: f64 = (assign9650_e8550 * 2.0);
        (assign9650_e8552, ((locals.var_costi4_dn0 * locals.var_beta) * 2.0), ((locals.var_costi4_dn2 * locals.var_beta) * 2.0), ((locals.var_costi4_dn6 * locals.var_beta) * 2.0), ((locals.var_costi4_dn7 * locals.var_beta) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), ((locals.var_costi4_dn11 * locals.var_beta) * 2.0), ((locals.var_costi4_dn12 * locals.var_beta) * 2.0), ((locals.var_costi4_dn17 * locals.var_beta) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn12, locals.var_costi5_dn17,)
    }
};
        locals.var_costi5 = assign9650_e8554;
        locals.var_costi5_dn0 = assign9650_e8554_d_n0;
        locals.var_costi5_dn2 = assign9650_e8554_d_n2;
        locals.var_costi5_dn6 = assign9650_e8554_d_n6;
        locals.var_costi5_dn7 = assign9650_e8554_d_n7;
        locals.var_costi5_dn10 = assign9650_e8554_d_n10;
        locals.var_costi5_dn11 = assign9650_e8554_d_n11;
        locals.var_costi5_dn12 = assign9650_e8554_d_n12;
        locals.var_costi5_dn17 = assign9650_e8554_d_n17;
        locals.var_costi5_rv = 0.0;

        let (assign9660_e8571, assign9660_e8571_d_n0, assign9660_e8571_d_n2, assign9660_e8571_d_n6, assign9660_e8571_d_n7, assign9660_e8571_d_n10, assign9660_e8571_d_n11, assign9660_e8571_d_n12, assign9660_e8571_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9660_e8562: f64 = (locals.var_beta * locals.var_vgssti);
        let assign9660_e8564: f64 = (assign9660_e8562 - 1.0);
        let assign9660_e8565: f64 = (4.0 * assign9660_e8564);
        let assign9660_e8567: f64 = (assign9660_e8565 / locals.var_costi5);
        let assign9660_e8568: f64 = (1.0 + assign9660_e8567);
        let assign9660_e8569: f64 = (assign9660_e8568).sqrt();
        (assign9660_e8569, (((((4.0 * (locals.var_beta * locals.var_vgssti_dn0)) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn0)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn2)) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn2)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn6)) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn6)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn7)) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn7)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)), (((((4.0 * ((locals.var_beta_dn10 * locals.var_vgssti) + (locals.var_beta * locals.var_vgssti_dn10))) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn10)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn11)) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn11)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn12)) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn12)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn17)) * locals.var_costi5) - (assign9660_e8565 * locals.var_costi5_dn17)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9660_e8569)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn12, locals.var_costi6_dn17,)
    }
};
        locals.var_costi6 = assign9660_e8571;
        locals.var_costi6_dn0 = assign9660_e8571_d_n0;
        locals.var_costi6_dn2 = assign9660_e8571_d_n2;
        locals.var_costi6_dn6 = assign9660_e8571_d_n6;
        locals.var_costi6_dn7 = assign9660_e8571_d_n7;
        locals.var_costi6_dn10 = assign9660_e8571_d_n10;
        locals.var_costi6_dn11 = assign9660_e8571_d_n11;
        locals.var_costi6_dn12 = assign9660_e8571_d_n12;
        locals.var_costi6_dn17 = assign9660_e8571_d_n17;
        locals.var_costi6_rv = 0.0;

        let (assign9670_e8583, assign9670_e8583_d_n0, assign9670_e8583_d_n2, assign9670_e8583_d_n6, assign9670_e8583_d_n7, assign9670_e8583_d_n10, assign9670_e8583_d_n11, assign9670_e8583_d_n12, assign9670_e8583_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9670_e8579: f64 = (1.0 - locals.var_costi6);
        let assign9670_e8580: f64 = (locals.var_costi4 * assign9670_e8579);
        let assign9670_e8581: f64 = (locals.var_vgssti + assign9670_e8580);
        (assign9670_e8581, (locals.var_vgssti_dn0 + ((locals.var_costi4_dn0 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn0)))), (locals.var_vgssti_dn2 + ((locals.var_costi4_dn2 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn2)))), (locals.var_vgssti_dn6 + ((locals.var_costi4_dn6 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn6)))), (locals.var_vgssti_dn7 + ((locals.var_costi4_dn7 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn7)))), (locals.var_vgssti_dn10 + ((locals.var_costi4_dn10 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn10)))), (locals.var_vgssti_dn11 + ((locals.var_costi4_dn11 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn11)))), (locals.var_vgssti_dn12 + ((locals.var_costi4_dn12 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn12)))), (locals.var_vgssti_dn17 + ((locals.var_costi4_dn17 * assign9670_e8579) + (locals.var_costi4 * (-locals.var_costi6_dn17)))),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn12, locals.var_psasti_dn17,)
    }
};
        locals.var_psasti = assign9670_e8583;
        locals.var_psasti_dn0 = assign9670_e8583_d_n0;
        locals.var_psasti_dn2 = assign9670_e8583_d_n2;
        locals.var_psasti_dn6 = assign9670_e8583_d_n6;
        locals.var_psasti_dn7 = assign9670_e8583_d_n7;
        locals.var_psasti_dn10 = assign9670_e8583_d_n10;
        locals.var_psasti_dn11 = assign9670_e8583_d_n11;
        locals.var_psasti_dn12 = assign9670_e8583_d_n12;
        locals.var_psasti_dn17 = assign9670_e8583_d_n17;
        locals.var_psasti_rv = 0.0;

        let (assign9680_e8593, assign9680_e8593_d_n0, assign9680_e8593_d_n2, assign9680_e8593_d_n6, assign9680_e8593_d_n7, assign9680_e8593_d_n10, assign9680_e8593_d_n11, assign9680_e8593_d_n12, assign9680_e8593_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9680_e8589: f64 = (1.0 / locals.var_costi1);
        let assign9680_e8591: f64 = (assign9680_e8589 / locals.var_costi3);
        (assign9680_e8591, ((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn12 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn12)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn17 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9680_e8589 * locals.var_costi3_dn17)) / (locals.var_costi3 * locals.var_costi3)),)
    } else {
        (locals.var_asti, locals.var_asti_dn0, locals.var_asti_dn2, locals.var_asti_dn6, locals.var_asti_dn7, locals.var_asti_dn10, locals.var_asti_dn11, locals.var_asti_dn12, locals.var_asti_dn17,)
    }
};
        locals.var_asti = assign9680_e8593;
        locals.var_asti_dn0 = assign9680_e8593_d_n0;
        locals.var_asti_dn2 = assign9680_e8593_d_n2;
        locals.var_asti_dn6 = assign9680_e8593_d_n6;
        locals.var_asti_dn7 = assign9680_e8593_d_n7;
        locals.var_asti_dn10 = assign9680_e8593_d_n10;
        locals.var_asti_dn11 = assign9680_e8593_d_n11;
        locals.var_asti_dn12 = assign9680_e8593_d_n12;
        locals.var_asti_dn17 = assign9680_e8593_d_n17;
        locals.var_asti_rv = 0.0;

        let (assign9690_e8610, assign9690_e8610_d_n0, assign9690_e8610_d_n2, assign9690_e8610_d_n6, assign9690_e8610_d_n7, assign9690_e8610_d_n10, assign9690_e8610_d_n11, assign9690_e8610_d_n12, assign9690_e8610_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9690_e8600: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign9690_e8601: f64 = (locals.var_asti * assign9690_e8600);
        let assign9690_e8602: f64 = (assign9690_e8601).ln();
        let assign9690_e8606: f64 = (2.0 / locals.var_vgssti);
        let assign9690_e8607: f64 = (locals.var_beta + assign9690_e8606);
        let assign9690_e8608: f64 = (assign9690_e8602 / assign9690_e8607);
        (assign9690_e8608, ((((((locals.var_asti_dn0 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (-((2.0 * locals.var_vgssti_dn0) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9690_e8607 * assign9690_e8607)), ((((((locals.var_asti_dn2 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (-((2.0 * locals.var_vgssti_dn2) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9690_e8607 * assign9690_e8607)), ((((((locals.var_asti_dn6 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (-((2.0 * locals.var_vgssti_dn6) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9690_e8607 * assign9690_e8607)), ((((((locals.var_asti_dn7 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (-((2.0 * locals.var_vgssti_dn7) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9690_e8607 * assign9690_e8607)), ((((((locals.var_asti_dn10 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (locals.var_vgssti * locals.var_vgssti)))))) / (assign9690_e8607 * assign9690_e8607)), ((((((locals.var_asti_dn11 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (-((2.0 * locals.var_vgssti_dn11) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9690_e8607 * assign9690_e8607)), ((((((locals.var_asti_dn12 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn12 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn12)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (-((2.0 * locals.var_vgssti_dn12) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9690_e8607 * assign9690_e8607)), ((((((locals.var_asti_dn17 * assign9690_e8600) + (locals.var_asti * ((locals.var_vgssti_dn17 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn17)))) / assign9690_e8601) * assign9690_e8607) - (assign9690_e8602 * (-((2.0 * locals.var_vgssti_dn17) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9690_e8607 * assign9690_e8607)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn12, locals.var_psbsti_dn17,)
    }
};
        locals.var_psbsti = assign9690_e8610;
        locals.var_psbsti_dn0 = assign9690_e8610_d_n0;
        locals.var_psbsti_dn2 = assign9690_e8610_d_n2;
        locals.var_psbsti_dn6 = assign9690_e8610_d_n6;
        locals.var_psbsti_dn7 = assign9690_e8610_d_n7;
        locals.var_psbsti_dn10 = assign9690_e8610_d_n10;
        locals.var_psbsti_dn11 = assign9690_e8610_d_n11;
        locals.var_psbsti_dn12 = assign9690_e8610_d_n12;
        locals.var_psbsti_dn17 = assign9690_e8610_d_n17;
        locals.var_psbsti_rv = 0.0;

        let (assign9700_e8620, assign9700_e8620_d_n0, assign9700_e8620_d_n2, assign9700_e8620_d_n6, assign9700_e8620_d_n7, assign9700_e8620_d_n10, assign9700_e8620_d_n11, assign9700_e8620_d_n12, assign9700_e8620_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9700_e8616: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign9700_e8618: f64 = (assign9700_e8616 - locals.var_sti2_dlt);
        (assign9700_e8618, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn12 - locals.var_psasti_dn12), (locals.var_psbsti_dn17 - locals.var_psasti_dn17),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn12, locals.var_psab_dn17,)
    }
};
        locals.var_psab = assign9700_e8620;
        locals.var_psab_dn0 = assign9700_e8620_d_n0;
        locals.var_psab_dn2 = assign9700_e8620_d_n2;
        locals.var_psab_dn6 = assign9700_e8620_d_n6;
        locals.var_psab_dn7 = assign9700_e8620_d_n7;
        locals.var_psab_dn10 = assign9700_e8620_d_n10;
        locals.var_psab_dn11 = assign9700_e8620_d_n11;
        locals.var_psab_dn12 = assign9700_e8620_d_n12;
        locals.var_psab_dn17 = assign9700_e8620_d_n17;
        locals.var_psab_rv = 0.0;

        let (assign9710_e8641, assign9710_e8641_d_n0, assign9710_e8641_d_n2, assign9710_e8641_d_n6, assign9710_e8641_d_n7, assign9710_e8641_d_n10, assign9710_e8641_d_n11, assign9710_e8641_d_n12, assign9710_e8641_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9710_e8629: f64 = (locals.var_psab * locals.var_psab);
        let assign9710_e8632: f64 = (4.0 * locals.var_sti2_dlt);
        let assign9710_e8634: f64 = (assign9710_e8632 * locals.var_psbsti);
        let assign9710_e8635: f64 = (assign9710_e8629 + assign9710_e8634);
        let assign9710_e8636: f64 = (assign9710_e8635).sqrt();
        let assign9710_e8637: f64 = (locals.var_psab + assign9710_e8636);
        let assign9710_e8638: f64 = (0.5 * assign9710_e8637);
        let assign9710_e8639: f64 = (locals.var_psbsti - assign9710_e8638);
        (assign9710_e8639, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign9710_e8632 * locals.var_psbsti_dn0)) / (2.0 * assign9710_e8636))))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign9710_e8632 * locals.var_psbsti_dn2)) / (2.0 * assign9710_e8636))))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign9710_e8632 * locals.var_psbsti_dn6)) / (2.0 * assign9710_e8636))))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign9710_e8632 * locals.var_psbsti_dn7)) / (2.0 * assign9710_e8636))))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign9710_e8632 * locals.var_psbsti_dn10)) / (2.0 * assign9710_e8636))))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign9710_e8632 * locals.var_psbsti_dn11)) / (2.0 * assign9710_e8636))))), (locals.var_psbsti_dn12 - (0.5 * (locals.var_psab_dn12 + ((((locals.var_psab_dn12 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn12)) + (assign9710_e8632 * locals.var_psbsti_dn12)) / (2.0 * assign9710_e8636))))), (locals.var_psbsti_dn17 - (0.5 * (locals.var_psab_dn17 + ((((locals.var_psab_dn17 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn17)) + (assign9710_e8632 * locals.var_psbsti_dn17)) / (2.0 * assign9710_e8636))))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    }
};
        locals.var_psti = assign9710_e8641;
        locals.var_psti_dn0 = assign9710_e8641_d_n0;
        locals.var_psti_dn2 = assign9710_e8641_d_n2;
        locals.var_psti_dn6 = assign9710_e8641_d_n6;
        locals.var_psti_dn7 = assign9710_e8641_d_n7;
        locals.var_psti_dn10 = assign9710_e8641_d_n10;
        locals.var_psti_dn11 = assign9710_e8641_d_n11;
        locals.var_psti_dn12 = assign9710_e8641_d_n12;
        locals.var_psti_dn17 = assign9710_e8641_d_n17;
        locals.var_psti_rv = 0.0;

        let (assign9720_e8650, assign9720_e8650_d_n0, assign9720_e8650_d_n2, assign9720_e8650_d_n6, assign9720_e8650_d_n7, assign9720_e8650_d_n10, assign9720_e8650_d_n11, assign9720_e8650_d_n12, assign9720_e8650_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9720_e8647: f64 = (locals.var_beta * locals.var_psti);
        let assign9720_e8648: f64 = (assign9720_e8647).exp();
        (assign9720_e8648, (assign9720_e8648 * (locals.var_beta * locals.var_psti_dn0)), (assign9720_e8648 * (locals.var_beta * locals.var_psti_dn2)), (assign9720_e8648 * (locals.var_beta * locals.var_psti_dn6)), (assign9720_e8648 * (locals.var_beta * locals.var_psti_dn7)), (assign9720_e8648 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))), (assign9720_e8648 * (locals.var_beta * locals.var_psti_dn11)), (assign9720_e8648 * (locals.var_beta * locals.var_psti_dn12)), (assign9720_e8648 * (locals.var_beta * locals.var_psti_dn17)),)
    } else {
        (locals.var_expsti, locals.var_expsti_dn0, locals.var_expsti_dn2, locals.var_expsti_dn6, locals.var_expsti_dn7, locals.var_expsti_dn10, locals.var_expsti_dn11, locals.var_expsti_dn12, locals.var_expsti_dn17,)
    }
};
        locals.var_expsti = assign9720_e8650;
        locals.var_expsti_dn0 = assign9720_e8650_d_n0;
        locals.var_expsti_dn2 = assign9720_e8650_d_n2;
        locals.var_expsti_dn6 = assign9720_e8650_d_n6;
        locals.var_expsti_dn7 = assign9720_e8650_d_n7;
        locals.var_expsti_dn10 = assign9720_e8650_d_n10;
        locals.var_expsti_dn11 = assign9720_e8650_d_n11;
        locals.var_expsti_dn12 = assign9720_e8650_d_n12;
        locals.var_expsti_dn17 = assign9720_e8650_d_n17;
        locals.var_expsti_rv = 0.0;

        let (assign9730_e8664, assign9730_e8664_d_n0, assign9730_e8664_d_n2, assign9730_e8664_d_n6, assign9730_e8664_d_n7, assign9730_e8664_d_n10, assign9730_e8664_d_n11, assign9730_e8664_d_n12, assign9730_e8664_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9730_e8656: f64 = (locals.var_beta * locals.var_psti);
        let assign9730_e8658: f64 = (assign9730_e8656 - 1.0);
        let assign9730_e8661: f64 = (locals.var_costi1 * locals.var_expsti);
        let assign9730_e8662: f64 = (assign9730_e8658 + assign9730_e8661);
        (assign9730_e8662, ((locals.var_beta * locals.var_psti_dn0) + ((locals.var_costi1_dn0 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn0))), ((locals.var_beta * locals.var_psti_dn2) + ((locals.var_costi1_dn2 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn2))), ((locals.var_beta * locals.var_psti_dn6) + ((locals.var_costi1_dn6 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn6))), ((locals.var_beta * locals.var_psti_dn7) + ((locals.var_costi1_dn7 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn7))), (((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)) + ((locals.var_costi1_dn10 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn10))), ((locals.var_beta * locals.var_psti_dn11) + ((locals.var_costi1_dn11 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn11))), ((locals.var_beta * locals.var_psti_dn12) + ((locals.var_costi1_dn12 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn12))), ((locals.var_beta * locals.var_psti_dn17) + ((locals.var_costi1_dn17 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn17))),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12, locals.var_sq1sti_dn17,)
    }
};
        locals.var_sq1sti = assign9730_e8664;
        locals.var_sq1sti_dn0 = assign9730_e8664_d_n0;
        locals.var_sq1sti_dn2 = assign9730_e8664_d_n2;
        locals.var_sq1sti_dn6 = assign9730_e8664_d_n6;
        locals.var_sq1sti_dn7 = assign9730_e8664_d_n7;
        locals.var_sq1sti_dn10 = assign9730_e8664_d_n10;
        locals.var_sq1sti_dn11 = assign9730_e8664_d_n11;
        locals.var_sq1sti_dn12 = assign9730_e8664_d_n12;
        locals.var_sq1sti_dn17 = assign9730_e8664_d_n17;
        locals.var_sq1sti_rv = 0.0;

        let (assign9740_e8674, assign9740_e8674_d_n0, assign9740_e8674_d_n2, assign9740_e8674_d_n6, assign9740_e8674_d_n7, assign9740_e8674_d_n10, assign9740_e8674_d_n11, assign9740_e8674_d_n12, assign9740_e8674_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) {
        let assign9740_e8670: f64 = (locals.var_beta * locals.var_psti);
        let assign9740_e8672: f64 = (assign9740_e8670 - 1.0);
        (assign9740_e8672, (locals.var_beta * locals.var_psti_dn0), (locals.var_beta * locals.var_psti_dn2), (locals.var_beta * locals.var_psti_dn6), (locals.var_beta * locals.var_psti_dn7), ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)), (locals.var_beta * locals.var_psti_dn11), (locals.var_beta * locals.var_psti_dn12), (locals.var_beta * locals.var_psti_dn17),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12, locals.var_sq2sti_dn17,)
    }
};
        locals.var_sq2sti = assign9740_e8674;
        locals.var_sq2sti_dn0 = assign9740_e8674_d_n0;
        locals.var_sq2sti_dn2 = assign9740_e8674_d_n2;
        locals.var_sq2sti_dn6 = assign9740_e8674_d_n6;
        locals.var_sq2sti_dn7 = assign9740_e8674_d_n7;
        locals.var_sq2sti_dn10 = assign9740_e8674_d_n10;
        locals.var_sq2sti_dn11 = assign9740_e8674_d_n11;
        locals.var_sq2sti_dn12 = assign9740_e8674_d_n12;
        locals.var_sq2sti_dn17 = assign9740_e8674_d_n17;
        locals.var_sq2sti_rv = 0.0;

        let assign9750_e8681: f64 = if ((locals.var_sq1sti > 0.0) && (locals.var_sq2sti > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard181 = assign9750_e8681;
        locals.var_guard181_rv = 0.0;

        let (assign9760_e8698, assign9760_e8698_d_n0, assign9760_e8698_d_n2, assign9760_e8698_d_n6, assign9760_e8698_d_n7, assign9760_e8698_d_n10, assign9760_e8698_d_n11, assign9760_e8698_d_n12, assign9760_e8698_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9760_e8689: f64 = (locals.var_beta * locals.var_psti);
        let assign9760_e8691: f64 = (assign9760_e8689 - 1.0);
        let assign9760_e8694: f64 = (locals.var_costi1 * locals.var_expsti);
        let assign9760_e8695: f64 = (assign9760_e8691 + assign9760_e8694);
        let assign9760_e8696: f64 = (assign9760_e8695).sqrt();
        (assign9760_e8696, (((locals.var_beta * locals.var_psti_dn0) + ((locals.var_costi1_dn0 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn0))) / (2.0 * assign9760_e8696)), (((locals.var_beta * locals.var_psti_dn2) + ((locals.var_costi1_dn2 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn2))) / (2.0 * assign9760_e8696)), (((locals.var_beta * locals.var_psti_dn6) + ((locals.var_costi1_dn6 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn6))) / (2.0 * assign9760_e8696)), (((locals.var_beta * locals.var_psti_dn7) + ((locals.var_costi1_dn7 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn7))) / (2.0 * assign9760_e8696)), ((((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)) + ((locals.var_costi1_dn10 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn10))) / (2.0 * assign9760_e8696)), (((locals.var_beta * locals.var_psti_dn11) + ((locals.var_costi1_dn11 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn11))) / (2.0 * assign9760_e8696)), (((locals.var_beta * locals.var_psti_dn12) + ((locals.var_costi1_dn12 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn12))) / (2.0 * assign9760_e8696)), (((locals.var_beta * locals.var_psti_dn17) + ((locals.var_costi1_dn17 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn17))) / (2.0 * assign9760_e8696)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12, locals.var_sq1sti_dn17,)
    }
};
        locals.var_sq1sti = assign9760_e8698;
        locals.var_sq1sti_dn0 = assign9760_e8698_d_n0;
        locals.var_sq1sti_dn2 = assign9760_e8698_d_n2;
        locals.var_sq1sti_dn6 = assign9760_e8698_d_n6;
        locals.var_sq1sti_dn7 = assign9760_e8698_d_n7;
        locals.var_sq1sti_dn10 = assign9760_e8698_d_n10;
        locals.var_sq1sti_dn11 = assign9760_e8698_d_n11;
        locals.var_sq1sti_dn12 = assign9760_e8698_d_n12;
        locals.var_sq1sti_dn17 = assign9760_e8698_d_n17;
        locals.var_sq1sti_rv = 0.0;

        let (assign9770_e8711, assign9770_e8711_d_n0, assign9770_e8711_d_n2, assign9770_e8711_d_n6, assign9770_e8711_d_n7, assign9770_e8711_d_n10, assign9770_e8711_d_n11, assign9770_e8711_d_n12, assign9770_e8711_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9770_e8706: f64 = (locals.var_beta * locals.var_psti);
        let assign9770_e8708: f64 = (assign9770_e8706 - 1.0);
        let assign9770_e8709: f64 = (assign9770_e8708).sqrt();
        (assign9770_e8709, ((locals.var_beta * locals.var_psti_dn0) / (2.0 * assign9770_e8709)), ((locals.var_beta * locals.var_psti_dn2) / (2.0 * assign9770_e8709)), ((locals.var_beta * locals.var_psti_dn6) / (2.0 * assign9770_e8709)), ((locals.var_beta * locals.var_psti_dn7) / (2.0 * assign9770_e8709)), (((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)) / (2.0 * assign9770_e8709)), ((locals.var_beta * locals.var_psti_dn11) / (2.0 * assign9770_e8709)), ((locals.var_beta * locals.var_psti_dn12) / (2.0 * assign9770_e8709)), ((locals.var_beta * locals.var_psti_dn17) / (2.0 * assign9770_e8709)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12, locals.var_sq2sti_dn17,)
    }
};
        locals.var_sq2sti = assign9770_e8711;
        locals.var_sq2sti_dn0 = assign9770_e8711_d_n0;
        locals.var_sq2sti_dn2 = assign9770_e8711_d_n2;
        locals.var_sq2sti_dn6 = assign9770_e8711_d_n6;
        locals.var_sq2sti_dn7 = assign9770_e8711_d_n7;
        locals.var_sq2sti_dn10 = assign9770_e8711_d_n10;
        locals.var_sq2sti_dn11 = assign9770_e8711_d_n11;
        locals.var_sq2sti_dn12 = assign9770_e8711_d_n12;
        locals.var_sq2sti_dn17 = assign9770_e8711_d_n17;
        locals.var_sq2sti_rv = 0.0;

        let (assign9780_e8723, assign9780_e8723_d_n0, assign9780_e8723_d_n2, assign9780_e8723_d_n6, assign9780_e8723_d_n7, assign9780_e8723_d_n10, assign9780_e8723_d_n11, assign9780_e8723_d_n12, assign9780_e8723_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9780_e8720: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign9780_e8721: f64 = (locals.var_costi0 * assign9780_e8720);
        (assign9780_e8721, ((locals.var_costi0_dn0 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn6 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn10 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn12 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn12 - locals.var_sq2sti_dn12))), ((locals.var_costi0_dn17 * assign9780_e8720) + (locals.var_costi0 * (locals.var_sq1sti_dn17 - locals.var_sq2sti_dn17))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn12, locals.var_qn0sti_dn17,)
    }
};
        locals.var_qn0sti = assign9780_e8723;
        locals.var_qn0sti_dn0 = assign9780_e8723_d_n0;
        locals.var_qn0sti_dn2 = assign9780_e8723_d_n2;
        locals.var_qn0sti_dn6 = assign9780_e8723_d_n6;
        locals.var_qn0sti_dn7 = assign9780_e8723_d_n7;
        locals.var_qn0sti_dn10 = assign9780_e8723_d_n10;
        locals.var_qn0sti_dn11 = assign9780_e8723_d_n11;
        locals.var_qn0sti_dn12 = assign9780_e8723_d_n12;
        locals.var_qn0sti_dn17 = assign9780_e8723_d_n17;
        locals.var_qn0sti_rv = 0.0;

        let (assign9790_e8735, assign9790_e8735_d_n10,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9790_e8731: f64 = (2.0 * locals.var_weff);
        let assign9790_e8733: f64 = (assign9790_e8731 / locals.var_beta);
        (assign9790_e8733, (-((assign9790_e8731 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn10,)
    }
};
        locals.var_costi7 = assign9790_e8735;
        locals.var_costi7_dn10 = assign9790_e8735_d_n10;
        locals.var_costi7_rv = 0.0;

        let (assign9800_e8745, assign9800_e8745_d_n0, assign9800_e8745_d_n2, assign9800_e8745_d_n6, assign9800_e8745_d_n7, assign9800_e8745_d_n10, assign9800_e8745_d_n11, assign9800_e8745_d_n12, assign9800_e8745_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9800_e8743: f64 = (300.0 * 0.0001);
        (assign9800_e8743, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn12, locals.var_mu_dn17,)
    }
};
        locals.var_mu = assign9800_e8745;
        locals.var_mu_dn0 = assign9800_e8745_d_n0;
        locals.var_mu_dn2 = assign9800_e8745_d_n2;
        locals.var_mu_dn6 = assign9800_e8745_d_n6;
        locals.var_mu_dn7 = assign9800_e8745_d_n7;
        locals.var_mu_dn10 = assign9800_e8745_d_n10;
        locals.var_mu_dn11 = assign9800_e8745_d_n11;
        locals.var_mu_dn12 = assign9800_e8745_d_n12;
        locals.var_mu_dn17 = assign9800_e8745_d_n17;
        locals.var_mu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9810_e8753, assign9810_e8753_d_n0, assign9810_e8753_d_n2, assign9810_e8753_d_n6, assign9810_e8753_d_n7, assign9810_e8753_d_n10, assign9810_e8753_d_n11, assign9810_e8753_d_n12, assign9810_e8753_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign9810_e8753;
        locals.var_lred_dn0 = assign9810_e8753_d_n0;
        locals.var_lred_dn2 = assign9810_e8753_d_n2;
        locals.var_lred_dn6 = assign9810_e8753_d_n6;
        locals.var_lred_dn7 = assign9810_e8753_d_n7;
        locals.var_lred_dn10 = assign9810_e8753_d_n10;
        locals.var_lred_dn11 = assign9810_e8753_d_n11;
        locals.var_lred_dn12 = assign9810_e8753_d_n12;
        locals.var_lred_dn17 = assign9810_e8753_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign9820_e8762, assign9820_e8762_d_n0, assign9820_e8762_d_n2, assign9820_e8762_d_n6, assign9820_e8762_d_n7, assign9820_e8762_d_n10, assign9820_e8762_d_n11, assign9820_e8762_d_n12, assign9820_e8762_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9820_e8760: f64 = 0.0;
        (assign9820_e8760, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign9820_e8762;
        locals.var_t1_dn0 = assign9820_e8762_d_n0;
        locals.var_t1_dn2 = assign9820_e8762_d_n2;
        locals.var_t1_dn6 = assign9820_e8762_d_n6;
        locals.var_t1_dn7 = assign9820_e8762_d_n7;
        locals.var_t1_dn10 = assign9820_e8762_d_n10;
        locals.var_t1_dn11 = assign9820_e8762_d_n11;
        locals.var_t1_dn12 = assign9820_e8762_d_n12;
        locals.var_t1_dn17 = assign9820_e8762_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign9830_e8774, assign9830_e8774_d_n0, assign9830_e8774_d_n2, assign9830_e8774_d_n6, assign9830_e8774_d_n7, assign9830_e8774_d_n10, assign9830_e8774_d_n11, assign9830_e8774_d_n12, assign9830_e8774_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9830_e8771: f64 = (locals.var_leff - locals.var_lred);
        let assign9830_e8772: f64 = (1.0 / assign9830_e8771);
        (assign9830_e8772, (-((-locals.var_lred_dn0) / (assign9830_e8771 * assign9830_e8771))), (-((-locals.var_lred_dn2) / (assign9830_e8771 * assign9830_e8771))), (-((-locals.var_lred_dn6) / (assign9830_e8771 * assign9830_e8771))), (-((-locals.var_lred_dn7) / (assign9830_e8771 * assign9830_e8771))), (-((-locals.var_lred_dn10) / (assign9830_e8771 * assign9830_e8771))), (-((-locals.var_lred_dn11) / (assign9830_e8771 * assign9830_e8771))), (-((-locals.var_lred_dn12) / (assign9830_e8771 * assign9830_e8771))), (-((-locals.var_lred_dn17) / (assign9830_e8771 * assign9830_e8771))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9830_e8774;
        locals.var_t2_dn0 = assign9830_e8774_d_n0;
        locals.var_t2_dn2 = assign9830_e8774_d_n2;
        locals.var_t2_dn6 = assign9830_e8774_d_n6;
        locals.var_t2_dn7 = assign9830_e8774_d_n7;
        locals.var_t2_dn10 = assign9830_e8774_d_n10;
        locals.var_t2_dn11 = assign9830_e8774_d_n11;
        locals.var_t2_dn12 = assign9830_e8774_d_n12;
        locals.var_t2_dn17 = assign9830_e8774_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9840_e8790, assign9840_e8790_d_n0, assign9840_e8790_d_n2, assign9840_e8790_d_n6, assign9840_e8790_d_n7, assign9840_e8790_d_n10, assign9840_e8790_d_n11, assign9840_e8790_d_n12, assign9840_e8790_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9840_e8782: f64 = (locals.var_costi7 * locals.var_mu);
        let assign9840_e8784: f64 = (assign9840_e8782 * locals.var_qn0sti);
        let assign9840_e8786: f64 = (assign9840_e8784 * locals.var_t1);
        let assign9840_e8788: f64 = (assign9840_e8786 * locals.var_t2);
        (assign9840_e8788, (((((((locals.var_costi7 * locals.var_mu_dn0) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn0)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn0)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn0)), (((((((locals.var_costi7 * locals.var_mu_dn2) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn2)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn2)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn2)), (((((((locals.var_costi7 * locals.var_mu_dn6) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn6)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn6)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn6)), (((((((locals.var_costi7 * locals.var_mu_dn7) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn7)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn7)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn7)), ((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn10)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn10)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn10)), (((((((locals.var_costi7 * locals.var_mu_dn11) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn11)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn11)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn11)), (((((((locals.var_costi7 * locals.var_mu_dn12) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn12)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn12)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn12)), (((((((locals.var_costi7 * locals.var_mu_dn17) * locals.var_qn0sti) + (assign9840_e8782 * locals.var_qn0sti_dn17)) * locals.var_t1) + (assign9840_e8784 * locals.var_t1_dn17)) * locals.var_t2) + (assign9840_e8786 * locals.var_t2_dn17)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12, locals.var_idssti_dn17,)
    }
};
        locals.var_idssti = assign9840_e8790;
        locals.var_idssti_dn0 = assign9840_e8790_d_n0;
        locals.var_idssti_dn2 = assign9840_e8790_d_n2;
        locals.var_idssti_dn6 = assign9840_e8790_d_n6;
        locals.var_idssti_dn7 = assign9840_e8790_d_n7;
        locals.var_idssti_dn10 = assign9840_e8790_d_n10;
        locals.var_idssti_dn11 = assign9840_e8790_d_n11;
        locals.var_idssti_dn12 = assign9840_e8790_d_n12;
        locals.var_idssti_dn17 = assign9840_e8790_d_n17;
        locals.var_idssti_rv = 0.0;

        let (assign9850_e8798, assign9850_e8798_d_n0, assign9850_e8798_d_n2, assign9850_e8798_d_n6, assign9850_e8798_d_n7, assign9850_e8798_d_n10, assign9850_e8798_d_n11, assign9850_e8798_d_n12, assign9850_e8798_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12, locals.var_idssti_dn17,)
    } else {
        (locals.var_ids_isub, locals.var_ids_isub_dn0, locals.var_ids_isub_dn2, locals.var_ids_isub_dn6, locals.var_ids_isub_dn7, locals.var_ids_isub_dn10, locals.var_ids_isub_dn11, locals.var_ids_isub_dn12, locals.var_ids_isub_dn17,)
    }
};
        locals.var_ids_isub = assign9850_e8798;
        locals.var_ids_isub_dn0 = assign9850_e8798_d_n0;
        locals.var_ids_isub_dn2 = assign9850_e8798_d_n2;
        locals.var_ids_isub_dn6 = assign9850_e8798_d_n6;
        locals.var_ids_isub_dn7 = assign9850_e8798_d_n7;
        locals.var_ids_isub_dn10 = assign9850_e8798_d_n10;
        locals.var_ids_isub_dn11 = assign9850_e8798_d_n11;
        locals.var_ids_isub_dn12 = assign9850_e8798_d_n12;
        locals.var_ids_isub_dn17 = assign9850_e8798_d_n17;
        locals.var_ids_isub_rv = 0.0;

        let (assign9860_e8806, assign9860_e8806_d_n0, assign9860_e8806_d_n2, assign9860_e8806_d_n6, assign9860_e8806_d_n7, assign9860_e8806_d_n10, assign9860_e8806_d_n11, assign9860_e8806_d_n12, assign9860_e8806_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    } else {
        (locals.var_ps0_isub, locals.var_ps0_isub_dn0, locals.var_ps0_isub_dn2, locals.var_ps0_isub_dn6, locals.var_ps0_isub_dn7, locals.var_ps0_isub_dn10, locals.var_ps0_isub_dn11, locals.var_ps0_isub_dn12, locals.var_ps0_isub_dn17,)
    }
};
        locals.var_ps0_isub = assign9860_e8806;
        locals.var_ps0_isub_dn0 = assign9860_e8806_d_n0;
        locals.var_ps0_isub_dn2 = assign9860_e8806_d_n2;
        locals.var_ps0_isub_dn6 = assign9860_e8806_d_n6;
        locals.var_ps0_isub_dn7 = assign9860_e8806_d_n7;
        locals.var_ps0_isub_dn10 = assign9860_e8806_d_n10;
        locals.var_ps0_isub_dn11 = assign9860_e8806_d_n11;
        locals.var_ps0_isub_dn12 = assign9860_e8806_d_n12;
        locals.var_ps0_isub_dn17 = assign9860_e8806_d_n17;
        locals.var_ps0_isub_rv = 0.0;

        let (assign9870_e8826, assign9870_e8826_d_n0, assign9870_e8826_d_n2, assign9870_e8826_d_n6, assign9870_e8826_d_n7, assign9870_e8826_d_n10, assign9870_e8826_d_n11, assign9870_e8826_d_n12, assign9870_e8826_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9870_e8816: f64 = (locals.var_beta * locals.var_vgpz);
        let assign9870_e8818: f64 = (assign9870_e8816 - 1.0);
        let assign9870_e8819: f64 = (4.0 * assign9870_e8818);
        let assign9870_e8822: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign9870_e8823: f64 = (assign9870_e8819 / assign9870_e8822);
        let assign9870_e8824: f64 = (1.0 + assign9870_e8823);
        (assign9870_e8824, ((((4.0 * (locals.var_beta * locals.var_vgpz_dn0)) * assign9870_e8822) - (assign9870_e8819 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign9870_e8822 * assign9870_e8822)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn2)) * assign9870_e8822) - (assign9870_e8819 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign9870_e8822 * assign9870_e8822)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn6)) * assign9870_e8822) - (assign9870_e8819 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign9870_e8822 * assign9870_e8822)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn7)) * assign9870_e8822) - (assign9870_e8819 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign9870_e8822 * assign9870_e8822)), ((((4.0 * ((locals.var_beta_dn10 * locals.var_vgpz) + (locals.var_beta * locals.var_vgpz_dn10))) * assign9870_e8822) - (assign9870_e8819 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign9870_e8822 * assign9870_e8822)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn11)) * assign9870_e8822) - (assign9870_e8819 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign9870_e8822 * assign9870_e8822)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn12)) * assign9870_e8822) - (assign9870_e8819 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign9870_e8822 * assign9870_e8822)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn17)) * assign9870_e8822) - (assign9870_e8819 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign9870_e8822 * assign9870_e8822)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign9870_e8826;
        locals.var_tx_dn0 = assign9870_e8826_d_n0;
        locals.var_tx_dn2 = assign9870_e8826_d_n2;
        locals.var_tx_dn6 = assign9870_e8826_d_n6;
        locals.var_tx_dn7 = assign9870_e8826_d_n7;
        locals.var_tx_dn10 = assign9870_e8826_d_n10;
        locals.var_tx_dn11 = assign9870_e8826_d_n11;
        locals.var_tx_dn12 = assign9870_e8826_d_n12;
        locals.var_tx_dn17 = assign9870_e8826_d_n17;
        locals.var_tx_rv = 0.0;

        let assign9880_e8830: f64 = (10.0 * 2.220446049250313e-16);
        let assign9880_e8831: f64 = if locals.var_tx < assign9880_e8830 { 1.0 } else { 0.0 };
        locals.var_guard182 = assign9880_e8831;
        locals.var_guard182_rv = 0.0;

        let (assign9890_e8843, assign9890_e8843_d_n0, assign9890_e8843_d_n2, assign9890_e8843_d_n6, assign9890_e8843_d_n7, assign9890_e8843_d_n10, assign9890_e8843_d_n11, assign9890_e8843_d_n12, assign9890_e8843_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard182 != 0.0)) {
        let assign9890_e8841: f64 = (10.0 * 2.220446049250313e-16);
        (assign9890_e8841, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign9890_e8843;
        locals.var_tx_dn0 = assign9890_e8843_d_n0;
        locals.var_tx_dn2 = assign9890_e8843_d_n2;
        locals.var_tx_dn6 = assign9890_e8843_d_n6;
        locals.var_tx_dn7 = assign9890_e8843_d_n7;
        locals.var_tx_dn10 = assign9890_e8843_d_n10;
        locals.var_tx_dn11 = assign9890_e8843_d_n11;
        locals.var_tx_dn12 = assign9890_e8843_d_n12;
        locals.var_tx_dn17 = assign9890_e8843_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign9900_e8862, assign9900_e8862_d_n0, assign9900_e8862_d_n2, assign9900_e8862_d_n6, assign9900_e8862_d_n7, assign9900_e8862_d_n10, assign9900_e8862_d_n11, assign9900_e8862_d_n12, assign9900_e8862_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9900_e8852: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign9900_e8854: f64 = (assign9900_e8852 * 0.5);
        let assign9900_e8857: f64 = (locals.var_tx).sqrt();
        let assign9900_e8858: f64 = (1.0 - assign9900_e8857);
        let assign9900_e8859: f64 = (assign9900_e8854 * assign9900_e8858);
        let assign9900_e8860: f64 = (locals.var_vgpz + assign9900_e8859);
        (assign9900_e8860, (locals.var_vgpz_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn0 / (2.0 * assign9900_e8857)))))), (locals.var_vgpz_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn2 / (2.0 * assign9900_e8857)))))), (locals.var_vgpz_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn6 / (2.0 * assign9900_e8857)))))), (locals.var_vgpz_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn7 / (2.0 * assign9900_e8857)))))), (locals.var_vgpz_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn10 / (2.0 * assign9900_e8857)))))), (locals.var_vgpz_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn11 / (2.0 * assign9900_e8857)))))), (locals.var_vgpz_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn12 / (2.0 * assign9900_e8857)))))), (locals.var_vgpz_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign9900_e8858) + (assign9900_e8854 * (-(locals.var_tx_dn17 / (2.0 * assign9900_e8857)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign9900_e8862;
        locals.var_ps0_inia_dn0 = assign9900_e8862_d_n0;
        locals.var_ps0_inia_dn2 = assign9900_e8862_d_n2;
        locals.var_ps0_inia_dn6 = assign9900_e8862_d_n6;
        locals.var_ps0_inia_dn7 = assign9900_e8862_d_n7;
        locals.var_ps0_inia_dn10 = assign9900_e8862_d_n10;
        locals.var_ps0_inia_dn11 = assign9900_e8862_d_n11;
        locals.var_ps0_inia_dn12 = assign9900_e8862_d_n12;
        locals.var_ps0_inia_dn17 = assign9900_e8862_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign9910_e8870, assign9910_e8870_d_n0, assign9910_e8870_d_n2, assign9910_e8870_d_n6, assign9910_e8870_d_n7, assign9910_e8870_d_n10, assign9910_e8870_d_n11, assign9910_e8870_d_n12, assign9910_e8870_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign9910_e8870;
        locals.var_psl_lim_dn0 = assign9910_e8870_d_n0;
        locals.var_psl_lim_dn2 = assign9910_e8870_d_n2;
        locals.var_psl_lim_dn6 = assign9910_e8870_d_n6;
        locals.var_psl_lim_dn7 = assign9910_e8870_d_n7;
        locals.var_psl_lim_dn10 = assign9910_e8870_d_n10;
        locals.var_psl_lim_dn11 = assign9910_e8870_d_n11;
        locals.var_psl_lim_dn12 = assign9910_e8870_d_n12;
        locals.var_psl_lim_dn17 = assign9910_e8870_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let (assign9920_e8880, assign9920_e8880_d_n0, assign9920_e8880_d_n2, assign9920_e8880_d_n6, assign9920_e8880_d_n7, assign9920_e8880_d_n10, assign9920_e8880_d_n11, assign9920_e8880_d_n12, assign9920_e8880_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9920_e8878: f64 = (locals.var_ps0_inia - locals.var_ps0_isub);
        (assign9920_e8878, (locals.var_ps0_inia_dn0 - locals.var_ps0_isub_dn0), (locals.var_ps0_inia_dn2 - locals.var_ps0_isub_dn2), (locals.var_ps0_inia_dn6 - locals.var_ps0_isub_dn6), (locals.var_ps0_inia_dn7 - locals.var_ps0_isub_dn7), (locals.var_ps0_inia_dn10 - locals.var_ps0_isub_dn10), (locals.var_ps0_inia_dn11 - locals.var_ps0_isub_dn11), (locals.var_ps0_inia_dn12 - locals.var_ps0_isub_dn12), (locals.var_ps0_inia_dn17 - locals.var_ps0_isub_dn17),)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign9920_e8880;
        locals.var_pds_max_dn0 = assign9920_e8880_d_n0;
        locals.var_pds_max_dn2 = assign9920_e8880_d_n2;
        locals.var_pds_max_dn6 = assign9920_e8880_d_n6;
        locals.var_pds_max_dn7 = assign9920_e8880_d_n7;
        locals.var_pds_max_dn10 = assign9920_e8880_d_n10;
        locals.var_pds_max_dn11 = assign9920_e8880_d_n11;
        locals.var_pds_max_dn12 = assign9920_e8880_d_n12;
        locals.var_pds_max_dn17 = assign9920_e8880_d_n17;
        locals.var_pds_max_rv = 0.0;

        let assign9930_e8883: f64 = if locals.var_pds_max < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign9930_e8883;
        locals.var_guard183_rv = 0.0;

        let (assign9940_e8893, assign9940_e8893_d_n0, assign9940_e8893_d_n2, assign9940_e8893_d_n6, assign9940_e8893_d_n7, assign9940_e8893_d_n10, assign9940_e8893_d_n11, assign9940_e8893_d_n12, assign9940_e8893_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard183 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign9940_e8893;
        locals.var_pds_max_dn0 = assign9940_e8893_d_n0;
        locals.var_pds_max_dn2 = assign9940_e8893_d_n2;
        locals.var_pds_max_dn6 = assign9940_e8893_d_n6;
        locals.var_pds_max_dn7 = assign9940_e8893_d_n7;
        locals.var_pds_max_dn10 = assign9940_e8893_d_n10;
        locals.var_pds_max_dn11 = assign9940_e8893_d_n11;
        locals.var_pds_max_dn12 = assign9940_e8893_d_n12;
        locals.var_pds_max_dn17 = assign9940_e8893_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign9950_e8905, assign9950_e8905_d_n0, assign9950_e8905_d_n2, assign9950_e8905_d_n6, assign9950_e8905_d_n7, assign9950_e8905_d_n10, assign9950_e8905_d_n11, assign9950_e8905_d_n12, assign9950_e8905_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9950_e8901: f64 = (1.0 + 0.3);
        let assign9950_e8903: f64 = (assign9950_e8901 * locals.var_pds_max);
        (assign9950_e8903, (assign9950_e8901 * locals.var_pds_max_dn0), (assign9950_e8901 * locals.var_pds_max_dn2), (assign9950_e8901 * locals.var_pds_max_dn6), (assign9950_e8901 * locals.var_pds_max_dn7), (assign9950_e8901 * locals.var_pds_max_dn10), (assign9950_e8901 * locals.var_pds_max_dn11), (assign9950_e8901 * locals.var_pds_max_dn12), (assign9950_e8901 * locals.var_pds_max_dn17),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign9950_e8905;
        locals.var_t5_dn0 = assign9950_e8905_d_n0;
        locals.var_t5_dn2 = assign9950_e8905_d_n2;
        locals.var_t5_dn6 = assign9950_e8905_d_n6;
        locals.var_t5_dn7 = assign9950_e8905_d_n7;
        locals.var_t5_dn10 = assign9950_e8905_d_n10;
        locals.var_t5_dn11 = assign9950_e8905_d_n11;
        locals.var_t5_dn12 = assign9950_e8905_d_n12;
        locals.var_t5_dn17 = assign9950_e8905_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign9960_e8917, assign9960_e8917_d_n0, assign9960_e8917_d_n2, assign9960_e8917_d_n6, assign9960_e8917_d_n7, assign9960_e8917_d_n10, assign9960_e8917_d_n11, assign9960_e8917_d_n12, assign9960_e8917_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9960_e8913: f64 = (locals.var_t5 - locals.var_vdsz);
        let assign9960_e8915: f64 = (assign9960_e8913 - 0.03);
        (assign9960_e8915, (locals.var_t5_dn0 - locals.var_vdsz_dn0), (locals.var_t5_dn2 - locals.var_vdsz_dn2), (locals.var_t5_dn6 - locals.var_vdsz_dn6), (locals.var_t5_dn7 - locals.var_vdsz_dn7), (locals.var_t5_dn10 - locals.var_vdsz_dn10), (locals.var_t5_dn11 - locals.var_vdsz_dn11), (locals.var_t5_dn12 - locals.var_vdsz_dn12), (locals.var_t5_dn17 - locals.var_vdsz_dn17),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign9960_e8917;
        locals.var_t6_dn0 = assign9960_e8917_d_n0;
        locals.var_t6_dn2 = assign9960_e8917_d_n2;
        locals.var_t6_dn6 = assign9960_e8917_d_n6;
        locals.var_t6_dn7 = assign9960_e8917_d_n7;
        locals.var_t6_dn10 = assign9960_e8917_d_n10;
        locals.var_t6_dn11 = assign9960_e8917_d_n11;
        locals.var_t6_dn12 = assign9960_e8917_d_n12;
        locals.var_t6_dn17 = assign9960_e8917_d_n17;
        locals.var_t6_rv = 0.0;

        let (assign9970_e8934, assign9970_e8934_d_n0, assign9970_e8934_d_n2, assign9970_e8934_d_n6, assign9970_e8934_d_n7, assign9970_e8934_d_n10, assign9970_e8934_d_n11, assign9970_e8934_d_n12, assign9970_e8934_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9970_e8925: f64 = (locals.var_t6 * locals.var_t6);
        let assign9970_e8928: f64 = (4.0 * locals.var_t5);
        let assign9970_e8930: f64 = (assign9970_e8928 * 0.03);
        let assign9970_e8931: f64 = (assign9970_e8925 + assign9970_e8930);
        let assign9970_e8932: f64 = (assign9970_e8931).sqrt();
        (assign9970_e8932, ((((locals.var_t6_dn0 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn0)) + ((4.0 * locals.var_t5_dn0) * 0.03)) / (2.0 * assign9970_e8932)), ((((locals.var_t6_dn2 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn2)) + ((4.0 * locals.var_t5_dn2) * 0.03)) / (2.0 * assign9970_e8932)), ((((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) + ((4.0 * locals.var_t5_dn6) * 0.03)) / (2.0 * assign9970_e8932)), ((((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) + ((4.0 * locals.var_t5_dn7) * 0.03)) / (2.0 * assign9970_e8932)), ((((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) + ((4.0 * locals.var_t5_dn10) * 0.03)) / (2.0 * assign9970_e8932)), ((((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) + ((4.0 * locals.var_t5_dn11) * 0.03)) / (2.0 * assign9970_e8932)), ((((locals.var_t6_dn12 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn12)) + ((4.0 * locals.var_t5_dn12) * 0.03)) / (2.0 * assign9970_e8932)), ((((locals.var_t6_dn17 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn17)) + ((4.0 * locals.var_t5_dn17) * 0.03)) / (2.0 * assign9970_e8932)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
        locals.var_t7 = assign9970_e8934;
        locals.var_t7_dn0 = assign9970_e8934_d_n0;
        locals.var_t7_dn2 = assign9970_e8934_d_n2;
        locals.var_t7_dn6 = assign9970_e8934_d_n6;
        locals.var_t7_dn7 = assign9970_e8934_d_n7;
        locals.var_t7_dn10 = assign9970_e8934_d_n10;
        locals.var_t7_dn11 = assign9970_e8934_d_n11;
        locals.var_t7_dn12 = assign9970_e8934_d_n12;
        locals.var_t7_dn17 = assign9970_e8934_d_n17;
        locals.var_t7_rv = 0.0;

        let (assign9980_e8948, assign9980_e8948_d_n0, assign9980_e8948_d_n2, assign9980_e8948_d_n6, assign9980_e8948_d_n7, assign9980_e8948_d_n10, assign9980_e8948_d_n11, assign9980_e8948_d_n12, assign9980_e8948_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign9980_e8944: f64 = (locals.var_t6 + locals.var_t7);
        let assign9980_e8945: f64 = (0.5 * assign9980_e8944);
        let assign9980_e8946: f64 = (locals.var_t5 - assign9980_e8945);
        (assign9980_e8946, (locals.var_t5_dn0 - (0.5 * (locals.var_t6_dn0 + locals.var_t7_dn0))), (locals.var_t5_dn2 - (0.5 * (locals.var_t6_dn2 + locals.var_t7_dn2))), (locals.var_t5_dn6 - (0.5 * (locals.var_t6_dn6 + locals.var_t7_dn6))), (locals.var_t5_dn7 - (0.5 * (locals.var_t6_dn7 + locals.var_t7_dn7))), (locals.var_t5_dn10 - (0.5 * (locals.var_t6_dn10 + locals.var_t7_dn10))), (locals.var_t5_dn11 - (0.5 * (locals.var_t6_dn11 + locals.var_t7_dn11))), (locals.var_t5_dn12 - (0.5 * (locals.var_t6_dn12 + locals.var_t7_dn12))), (locals.var_t5_dn17 - (0.5 * (locals.var_t6_dn17 + locals.var_t7_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign9980_e8948;
        locals.var_pds_ini_dn0 = assign9980_e8948_d_n0;
        locals.var_pds_ini_dn2 = assign9980_e8948_d_n2;
        locals.var_pds_ini_dn6 = assign9980_e8948_d_n6;
        locals.var_pds_ini_dn7 = assign9980_e8948_d_n7;
        locals.var_pds_ini_dn10 = assign9980_e8948_d_n10;
        locals.var_pds_ini_dn11 = assign9980_e8948_d_n11;
        locals.var_pds_ini_dn12 = assign9980_e8948_d_n12;
        locals.var_pds_ini_dn17 = assign9980_e8948_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign9990_e8951: f64 = if locals.var_pds_ini > locals.var_pds_max { 1.0 } else { 0.0 };
        locals.var_guard184 = assign9990_e8951;
        locals.var_guard184_rv = 0.0;

        let (assign10000_e8961, assign10000_e8961_d_n0, assign10000_e8961_d_n2, assign10000_e8961_d_n6, assign10000_e8961_d_n7, assign10000_e8961_d_n10, assign10000_e8961_d_n11, assign10000_e8961_d_n12, assign10000_e8961_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard184 != 0.0)) {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign10000_e8961;
        locals.var_pds_ini_dn0 = assign10000_e8961_d_n0;
        locals.var_pds_ini_dn2 = assign10000_e8961_d_n2;
        locals.var_pds_ini_dn6 = assign10000_e8961_d_n6;
        locals.var_pds_ini_dn7 = assign10000_e8961_d_n7;
        locals.var_pds_ini_dn10 = assign10000_e8961_d_n10;
        locals.var_pds_ini_dn11 = assign10000_e8961_d_n11;
        locals.var_pds_ini_dn12 = assign10000_e8961_d_n12;
        locals.var_pds_ini_dn17 = assign10000_e8961_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign10010_e8969, assign10010_e8969_d_n0, assign10010_e8969_d_n2, assign10010_e8969_d_n6, assign10010_e8969_d_n7, assign10010_e8969_d_n10, assign10010_e8969_d_n11, assign10010_e8969_d_n12, assign10010_e8969_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds_qwe, locals.var_pds_qwe_dn0, locals.var_pds_qwe_dn2, locals.var_pds_qwe_dn6, locals.var_pds_qwe_dn7, locals.var_pds_qwe_dn10, locals.var_pds_qwe_dn11, locals.var_pds_qwe_dn12, locals.var_pds_qwe_dn17,)
    }
};
        locals.var_pds_qwe = assign10010_e8969;
        locals.var_pds_qwe_dn0 = assign10010_e8969_d_n0;
        locals.var_pds_qwe_dn2 = assign10010_e8969_d_n2;
        locals.var_pds_qwe_dn6 = assign10010_e8969_d_n6;
        locals.var_pds_qwe_dn7 = assign10010_e8969_d_n7;
        locals.var_pds_qwe_dn10 = assign10010_e8969_d_n10;
        locals.var_pds_qwe_dn11 = assign10010_e8969_d_n11;
        locals.var_pds_qwe_dn12 = assign10010_e8969_d_n12;
        locals.var_pds_qwe_dn17 = assign10010_e8969_d_n17;
        locals.var_pds_qwe_rv = 0.0;

        let (assign10020_e8979,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign10020_e8977: f64 = (locals.var_tfox0 * 100.0);
        (assign10020_e8977,)
    } else {
        (locals.var_cgs_tfox0,)
    }
};
        locals.var_cgs_tfox0 = assign10020_e8979;
        locals.var_cgs_tfox0_rv = 0.0;

        let (assign10030_e8989,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign10030_e8987: f64 = (locals.var_weff_nf * 100.0);
        (assign10030_e8987,)
    } else {
        (locals.var_cgs_weff_nf,)
    }
};
        locals.var_cgs_weff_nf = assign10030_e8989;
        locals.var_cgs_weff_nf_rv = 0.0;

        let (assign10040_e8999,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) {
        let assign10040_e8997: f64 = (locals.var_leff * 100.0);
        (assign10040_e8997,)
    } else {
        (locals.var_cgs_leff,)
    }
};
        locals.var_cgs_leff = assign10040_e8999;
        locals.var_cgs_leff_rv = 0.0;

        let assign10050_e9002: f64 = if p.p36 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard205 = assign10050_e9002;
        locals.var_guard205_rv = 0.0;

        let (assign10070_e9023,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        (4.12,)
    } else {
        (locals.var_phib,)
    }
};
        locals.var_phib = assign10070_e9023;
        locals.var_phib_rv = 0.0;

        let (assign10080_e9040,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10080_e9034: f64 = (p.p142 * 1.6021918e-19);
        let assign10080_e9036: f64 = (assign10080_e9034 * locals.var_cgs_weff_nf);
        let assign10080_e9038: f64 = (assign10080_e9036 * locals.var_cgs_leff);
        (assign10080_e9038,)
    } else {
        (locals.var_evb1_qe_wl,)
    }
};
        locals.var_evb1_qe_wl = assign10080_e9040;
        locals.var_evb1_qe_wl_rv = 0.0;

        let (assign10090_e9053, assign10090_e9053_d_n0, assign10090_e9053_d_n2, assign10090_e9053_d_n6, assign10090_e9053_d_n7, assign10090_e9053_d_n10, assign10090_e9053_d_n11, assign10090_e9053_d_n12, assign10090_e9053_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10090_e9051: f64 = (locals.var_evb1_qe_wl / locals.var_egp12);
        (assign10090_e9051, (-((locals.var_evb1_qe_wl * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_evb1_qe_wl_p_egp12, locals.var_evb1_qe_wl_p_egp12_dn0, locals.var_evb1_qe_wl_p_egp12_dn2, locals.var_evb1_qe_wl_p_egp12_dn6, locals.var_evb1_qe_wl_p_egp12_dn7, locals.var_evb1_qe_wl_p_egp12_dn10, locals.var_evb1_qe_wl_p_egp12_dn11, locals.var_evb1_qe_wl_p_egp12_dn12, locals.var_evb1_qe_wl_p_egp12_dn17,)
    }
};
        locals.var_evb1_qe_wl_p_egp12 = assign10090_e9053;
        locals.var_evb1_qe_wl_p_egp12_dn0 = assign10090_e9053_d_n0;
        locals.var_evb1_qe_wl_p_egp12_dn2 = assign10090_e9053_d_n2;
        locals.var_evb1_qe_wl_p_egp12_dn6 = assign10090_e9053_d_n6;
        locals.var_evb1_qe_wl_p_egp12_dn7 = assign10090_e9053_d_n7;
        locals.var_evb1_qe_wl_p_egp12_dn10 = assign10090_e9053_d_n10;
        locals.var_evb1_qe_wl_p_egp12_dn11 = assign10090_e9053_d_n11;
        locals.var_evb1_qe_wl_p_egp12_dn12 = assign10090_e9053_d_n12;
        locals.var_evb1_qe_wl_p_egp12_dn17 = assign10090_e9053_d_n17;
        locals.var_evb1_qe_wl_p_egp12_rv = 0.0;

        let (assign10100_e9077, assign10100_e9077_d_n0, assign10100_e9077_d_n2, assign10100_e9077_d_n6, assign10100_e9077_d_n7, assign10100_e9077_d_n10, assign10100_e9077_d_n11, assign10100_e9077_d_n12, assign10100_e9077_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10100_e9064: f64 = (p.p145 * locals.var_vbspz);
        let assign10100_e9066: f64 = (assign10100_e9064 + locals.var_dvthsc);
        let assign10100_e9068: f64 = (assign10100_e9066 + locals.var_dvthlp);
        let assign10100_e9070: f64 = (assign10100_e9068 + locals.var_eg);
        let assign10100_e9072: f64 = (assign10100_e9070 + p.p144);
        let assign10100_e9073: f64 = (-assign10100_e9072);
        let assign10100_e9075: f64 = (assign10100_e9073 / locals.var_cgs_tfox0);
        (assign10100_e9075, ((-((((p.p145 * locals.var_vbspz_dn0) + locals.var_dvthsc_dn0) + locals.var_dvthlp_dn0) + locals.var_eg_dn0)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn2) + locals.var_dvthsc_dn2) + locals.var_dvthlp_dn2) + locals.var_eg_dn2)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn6) + locals.var_dvthsc_dn6) + locals.var_dvthlp_dn6) + locals.var_eg_dn6)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn7) + locals.var_dvthsc_dn7) + locals.var_dvthlp_dn7) + locals.var_eg_dn7)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn10) + locals.var_dvthsc_dn10) + locals.var_dvthlp_dn10) + locals.var_eg_dn10)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn11) + locals.var_dvthsc_dn11) + locals.var_dvthlp_dn11) + locals.var_eg_dn11)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn12) + locals.var_dvthsc_dn12) + locals.var_dvthlp_dn12) + locals.var_eg_dn12)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn17) + locals.var_dvthsc_dn17) + locals.var_dvthlp_dn17) + locals.var_eg_dn17)) / locals.var_cgs_tfox0),)
    } else {
        (locals.var_eevb_wo_vox, locals.var_eevb_wo_vox_dn0, locals.var_eevb_wo_vox_dn2, locals.var_eevb_wo_vox_dn6, locals.var_eevb_wo_vox_dn7, locals.var_eevb_wo_vox_dn10, locals.var_eevb_wo_vox_dn11, locals.var_eevb_wo_vox_dn12, locals.var_eevb_wo_vox_dn17,)
    }
};
        locals.var_eevb_wo_vox = assign10100_e9077;
        locals.var_eevb_wo_vox_dn0 = assign10100_e9077_d_n0;
        locals.var_eevb_wo_vox_dn2 = assign10100_e9077_d_n2;
        locals.var_eevb_wo_vox_dn6 = assign10100_e9077_d_n6;
        locals.var_eevb_wo_vox_dn7 = assign10100_e9077_d_n7;
        locals.var_eevb_wo_vox_dn10 = assign10100_e9077_d_n10;
        locals.var_eevb_wo_vox_dn11 = assign10100_e9077_d_n11;
        locals.var_eevb_wo_vox_dn12 = assign10100_e9077_d_n12;
        locals.var_eevb_wo_vox_dn17 = assign10100_e9077_d_n17;
        locals.var_eevb_wo_vox_rv = 0.0;

        let (assign10110_e9088,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign10110_e9088;
        locals.var_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign10120_loop_guard: usize = 0;
        while {
            let assign10120_cond_e9100: f64 = (100.0 - 1.0);
            let assign10120_cond_e9102: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_i <= assign10120_cond_e9100)) { 1.0 } else { 0.0 };
            assign10120_cond_e9102 != 0.0
        } {
            assign10120_loop_guard += 1;
            assert!(assign10120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10120_body0_e9113,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        (locals.var_i,)
    } else {
        (locals.var_reali,)
    }
};
            locals.var_reali = assign10120_body0_e9113;
            locals.var_reali_rv = 0.0;
            let (assign10120_body1_e9124,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        (100.0,)
    } else {
        (locals.var_realn,)
    }
};
            locals.var_realn = assign10120_body1_e9124;
            locals.var_realn_rv = 0.0;
            let (assign10120_body2_e9137,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body2_e9135: f64 = (locals.var_reali / locals.var_realn);
        (assign10120_body2_e9135,)
    } else {
        (locals.var_r,)
    }
};
            locals.var_r = assign10120_body2_e9137;
            locals.var_r_rv = 0.0;
            let (assign10120_body3_e9156, assign10120_body3_e9156_d_n0, assign10120_body3_e9156_d_n2, assign10120_body3_e9156_d_n6, assign10120_body3_e9156_d_n7, assign10120_body3_e9156_d_n10, assign10120_body3_e9156_d_n11, assign10120_body3_e9156_d_n12, assign10120_body3_e9156_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body3_e9148: f64 = (locals.var_vgp + locals.var_vzadd);
        let assign10120_body3_e9151: f64 = (locals.var_pds_qwe * locals.var_r);
        let assign10120_body3_e9153: f64 = (assign10120_body3_e9151 + locals.var_ps0_isub);
        let assign10120_body3_e9154: f64 = (assign10120_body3_e9148 - assign10120_body3_e9153);
        (assign10120_body3_e9154, ((locals.var_vgp_dn0 + locals.var_vzadd_dn0) - ((locals.var_pds_qwe_dn0 * locals.var_r) + locals.var_ps0_isub_dn0)), ((locals.var_vgp_dn2 + locals.var_vzadd_dn2) - ((locals.var_pds_qwe_dn2 * locals.var_r) + locals.var_ps0_isub_dn2)), ((locals.var_vgp_dn6 + locals.var_vzadd_dn6) - ((locals.var_pds_qwe_dn6 * locals.var_r) + locals.var_ps0_isub_dn6)), ((locals.var_vgp_dn7 + locals.var_vzadd_dn7) - ((locals.var_pds_qwe_dn7 * locals.var_r) + locals.var_ps0_isub_dn7)), ((locals.var_vgp_dn10 + locals.var_vzadd_dn10) - ((locals.var_pds_qwe_dn10 * locals.var_r) + locals.var_ps0_isub_dn10)), ((locals.var_vgp_dn11 + locals.var_vzadd_dn11) - ((locals.var_pds_qwe_dn11 * locals.var_r) + locals.var_ps0_isub_dn11)), ((locals.var_vgp_dn12 + locals.var_vzadd_dn12) - ((locals.var_pds_qwe_dn12 * locals.var_r) + locals.var_ps0_isub_dn12)), ((locals.var_vgp_dn17 + locals.var_vzadd_dn17) - ((locals.var_pds_qwe_dn17 * locals.var_r) + locals.var_ps0_isub_dn17)),)
    } else {
        (locals.var_vox, locals.var_vox_dn0, locals.var_vox_dn2, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12, locals.var_vox_dn17,)
    }
};
            locals.var_vox = assign10120_body3_e9156;
            locals.var_vox_dn0 = assign10120_body3_e9156_d_n0;
            locals.var_vox_dn2 = assign10120_body3_e9156_d_n2;
            locals.var_vox_dn6 = assign10120_body3_e9156_d_n6;
            locals.var_vox_dn7 = assign10120_body3_e9156_d_n7;
            locals.var_vox_dn10 = assign10120_body3_e9156_d_n10;
            locals.var_vox_dn11 = assign10120_body3_e9156_d_n11;
            locals.var_vox_dn12 = assign10120_body3_e9156_d_n12;
            locals.var_vox_dn17 = assign10120_body3_e9156_d_n17;
            locals.var_vox_rv = 0.0;
            let (assign10120_body4_e9171, assign10120_body4_e9171_d_n0, assign10120_body4_e9171_d_n2, assign10120_body4_e9171_d_n6, assign10120_body4_e9171_d_n7, assign10120_body4_e9171_d_n10, assign10120_body4_e9171_d_n11, assign10120_body4_e9171_d_n12, assign10120_body4_e9171_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body4_e9168: f64 = (locals.var_vox / locals.var_phib);
        let assign10120_body4_e9169: f64 = (1.0 - assign10120_body4_e9168);
        (assign10120_body4_e9169, (-(locals.var_vox_dn0 / locals.var_phib)), (-(locals.var_vox_dn2 / locals.var_phib)), (-(locals.var_vox_dn6 / locals.var_phib)), (-(locals.var_vox_dn7 / locals.var_phib)), (-(locals.var_vox_dn10 / locals.var_phib)), (-(locals.var_vox_dn11 / locals.var_phib)), (-(locals.var_vox_dn12 / locals.var_phib)), (-(locals.var_vox_dn17 / locals.var_phib)),)
    } else {
        (locals.var_d0, locals.var_d0_dn0, locals.var_d0_dn2, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn10, locals.var_d0_dn11, locals.var_d0_dn12, locals.var_d0_dn17,)
    }
};
            locals.var_d0 = assign10120_body4_e9171;
            locals.var_d0_dn0 = assign10120_body4_e9171_d_n0;
            locals.var_d0_dn2 = assign10120_body4_e9171_d_n2;
            locals.var_d0_dn6 = assign10120_body4_e9171_d_n6;
            locals.var_d0_dn7 = assign10120_body4_e9171_d_n7;
            locals.var_d0_dn10 = assign10120_body4_e9171_d_n10;
            locals.var_d0_dn11 = assign10120_body4_e9171_d_n11;
            locals.var_d0_dn12 = assign10120_body4_e9171_d_n12;
            locals.var_d0_dn17 = assign10120_body4_e9171_d_n17;
            locals.var_d0_rv = 0.0;
            let (assign10120_body5_e9186, assign10120_body5_e9186_d_n0, assign10120_body5_e9186_d_n2, assign10120_body5_e9186_d_n6, assign10120_body5_e9186_d_n7, assign10120_body5_e9186_d_n10, assign10120_body5_e9186_d_n11, assign10120_body5_e9186_d_n12, assign10120_body5_e9186_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body5_e9183: f64 = (locals.var_vox / locals.var_cgs_tfox0);
        let assign10120_body5_e9184: f64 = (locals.var_eevb_wo_vox + assign10120_body5_e9183);
        (assign10120_body5_e9184, (locals.var_eevb_wo_vox_dn0 + (locals.var_vox_dn0 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn2 + (locals.var_vox_dn2 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn6 + (locals.var_vox_dn6 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn7 + (locals.var_vox_dn7 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn10 + (locals.var_vox_dn10 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn11 + (locals.var_vox_dn11 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn12 + (locals.var_vox_dn12 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn17 + (locals.var_vox_dn17 / locals.var_cgs_tfox0)),)
    } else {
        (locals.var_t2__blk195, locals.var_t2__blk195_dn0, locals.var_t2__blk195_dn2, locals.var_t2__blk195_dn6, locals.var_t2__blk195_dn7, locals.var_t2__blk195_dn10, locals.var_t2__blk195_dn11, locals.var_t2__blk195_dn12, locals.var_t2__blk195_dn17,)
    }
};
            locals.var_t2__blk195 = assign10120_body5_e9186;
            locals.var_t2__blk195_dn0 = assign10120_body5_e9186_d_n0;
            locals.var_t2__blk195_dn2 = assign10120_body5_e9186_d_n2;
            locals.var_t2__blk195_dn6 = assign10120_body5_e9186_d_n6;
            locals.var_t2__blk195_dn7 = assign10120_body5_e9186_d_n7;
            locals.var_t2__blk195_dn10 = assign10120_body5_e9186_d_n10;
            locals.var_t2__blk195_dn11 = assign10120_body5_e9186_d_n11;
            locals.var_t2__blk195_dn12 = assign10120_body5_e9186_d_n12;
            locals.var_t2__blk195_dn17 = assign10120_body5_e9186_d_n17;
            locals.var_t2__blk195_rv = 0.0;
            let (assign10120_body6_e9199, assign10120_body6_e9199_d_n0, assign10120_body6_e9199_d_n2, assign10120_body6_e9199_d_n6, assign10120_body6_e9199_d_n7, assign10120_body6_e9199_d_n10, assign10120_body6_e9199_d_n11, assign10120_body6_e9199_d_n12, assign10120_body6_e9199_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body6_e9197: f64 = (locals.var_t2__blk195 * locals.var_t2__blk195);
        (assign10120_body6_e9197, ((locals.var_t2__blk195_dn0 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn0)), ((locals.var_t2__blk195_dn2 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn2)), ((locals.var_t2__blk195_dn6 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn6)), ((locals.var_t2__blk195_dn7 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn7)), ((locals.var_t2__blk195_dn10 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn10)), ((locals.var_t2__blk195_dn11 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn11)), ((locals.var_t2__blk195_dn12 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn12)), ((locals.var_t2__blk195_dn17 * locals.var_t2__blk195) + (locals.var_t2__blk195 * locals.var_t2__blk195_dn17)),)
    } else {
        (locals.var_t0__blk193, locals.var_t0__blk193_dn0, locals.var_t0__blk193_dn2, locals.var_t0__blk193_dn6, locals.var_t0__blk193_dn7, locals.var_t0__blk193_dn10, locals.var_t0__blk193_dn11, locals.var_t0__blk193_dn12, locals.var_t0__blk193_dn17,)
    }
};
            locals.var_t0__blk193 = assign10120_body6_e9199;
            locals.var_t0__blk193_dn0 = assign10120_body6_e9199_d_n0;
            locals.var_t0__blk193_dn2 = assign10120_body6_e9199_d_n2;
            locals.var_t0__blk193_dn6 = assign10120_body6_e9199_d_n6;
            locals.var_t0__blk193_dn7 = assign10120_body6_e9199_d_n7;
            locals.var_t0__blk193_dn10 = assign10120_body6_e9199_d_n10;
            locals.var_t0__blk193_dn11 = assign10120_body6_e9199_d_n11;
            locals.var_t0__blk193_dn12 = assign10120_body6_e9199_d_n12;
            locals.var_t0__blk193_dn17 = assign10120_body6_e9199_d_n17;
            locals.var_t0__blk193_rv = 0.0;
            let (assign10120_body7_e9219, assign10120_body7_e9219_d_n0, assign10120_body7_e9219_d_n2, assign10120_body7_e9219_d_n6, assign10120_body7_e9219_d_n7, assign10120_body7_e9219_d_n10, assign10120_body7_e9219_d_n11, assign10120_body7_e9219_d_n12, assign10120_body7_e9219_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body7_e9210: f64 = (locals.var_d0 * locals.var_d0);
        let assign10120_body7_e9213: f64 = (4.0 * 0.001);
        let assign10120_body7_e9215: f64 = (assign10120_body7_e9213 * 0.001);
        let assign10120_body7_e9216: f64 = (assign10120_body7_e9210 + assign10120_body7_e9215);
        let assign10120_body7_e9217: f64 = (assign10120_body7_e9216).sqrt();
        (assign10120_body7_e9217, (((locals.var_d0_dn0 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn0)) / (2.0 * assign10120_body7_e9217)), (((locals.var_d0_dn2 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn2)) / (2.0 * assign10120_body7_e9217)), (((locals.var_d0_dn6 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn6)) / (2.0 * assign10120_body7_e9217)), (((locals.var_d0_dn7 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn7)) / (2.0 * assign10120_body7_e9217)), (((locals.var_d0_dn10 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn10)) / (2.0 * assign10120_body7_e9217)), (((locals.var_d0_dn11 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn11)) / (2.0 * assign10120_body7_e9217)), (((locals.var_d0_dn12 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn12)) / (2.0 * assign10120_body7_e9217)), (((locals.var_d0_dn17 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn17)) / (2.0 * assign10120_body7_e9217)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign10120_body7_e9219;
            locals.var_tmf1_dn0 = assign10120_body7_e9219_d_n0;
            locals.var_tmf1_dn2 = assign10120_body7_e9219_d_n2;
            locals.var_tmf1_dn6 = assign10120_body7_e9219_d_n6;
            locals.var_tmf1_dn7 = assign10120_body7_e9219_d_n7;
            locals.var_tmf1_dn10 = assign10120_body7_e9219_d_n10;
            locals.var_tmf1_dn11 = assign10120_body7_e9219_d_n11;
            locals.var_tmf1_dn12 = assign10120_body7_e9219_d_n12;
            locals.var_tmf1_dn17 = assign10120_body7_e9219_d_n17;
            locals.var_tmf1_rv = 0.0;
            let (assign10120_body8_e9238, assign10120_body8_e9238_d_n0, assign10120_body8_e9238_d_n2, assign10120_body8_e9238_d_n6, assign10120_body8_e9238_d_n7, assign10120_body8_e9238_d_n10, assign10120_body8_e9238_d_n11, assign10120_body8_e9238_d_n12, assign10120_body8_e9238_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body8_e9231: f64 = (locals.var_d0 + locals.var_tmf1);
        let assign10120_body8_e9232: f64 = (0.5 * assign10120_body8_e9231);
        let assign10120_body8_e9235: f64 = (1e-10 * 0.001);
        let assign10120_body8_e9236: f64 = (assign10120_body8_e9232 + assign10120_body8_e9235);
        (assign10120_body8_e9236, (0.5 * (locals.var_d0_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_d0_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_d0_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_d0_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_d0_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_d0_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_d0_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_d0_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_d0, locals.var_d0_dn0, locals.var_d0_dn2, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn10, locals.var_d0_dn11, locals.var_d0_dn12, locals.var_d0_dn17,)
    }
};
            locals.var_d0 = assign10120_body8_e9238;
            locals.var_d0_dn0 = assign10120_body8_e9238_d_n0;
            locals.var_d0_dn2 = assign10120_body8_e9238_d_n2;
            locals.var_d0_dn6 = assign10120_body8_e9238_d_n6;
            locals.var_d0_dn7 = assign10120_body8_e9238_d_n7;
            locals.var_d0_dn10 = assign10120_body8_e9238_d_n10;
            locals.var_d0_dn11 = assign10120_body8_e9238_d_n11;
            locals.var_d0_dn12 = assign10120_body8_e9238_d_n12;
            locals.var_d0_dn17 = assign10120_body8_e9238_d_n17;
            locals.var_d0_rv = 0.0;
            let assign10120_body9_e9241: f64 = if locals.var_d0 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard206 = assign10120_body9_e9241;
            locals.var_guard206_rv = 0.0;
            let (assign10120_body10_e9254, assign10120_body10_e9254_d_n0, assign10120_body10_e9254_d_n2, assign10120_body10_e9254_d_n6, assign10120_body10_e9254_d_n7, assign10120_body10_e9254_d_n10, assign10120_body10_e9254_d_n11, assign10120_body10_e9254_d_n12, assign10120_body10_e9254_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard206 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_d0, locals.var_d0_dn0, locals.var_d0_dn2, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn10, locals.var_d0_dn11, locals.var_d0_dn12, locals.var_d0_dn17,)
    }
};
            locals.var_d0 = assign10120_body10_e9254;
            locals.var_d0_dn0 = assign10120_body10_e9254_d_n0;
            locals.var_d0_dn2 = assign10120_body10_e9254_d_n2;
            locals.var_d0_dn6 = assign10120_body10_e9254_d_n6;
            locals.var_d0_dn7 = assign10120_body10_e9254_d_n7;
            locals.var_d0_dn10 = assign10120_body10_e9254_d_n10;
            locals.var_d0_dn11 = assign10120_body10_e9254_d_n11;
            locals.var_d0_dn12 = assign10120_body10_e9254_d_n12;
            locals.var_d0_dn17 = assign10120_body10_e9254_d_n17;
            locals.var_d0_rv = 0.0;
            let (assign10120_body11_e9272, assign10120_body11_e9272_d_n0, assign10120_body11_e9272_d_n2, assign10120_body11_e9272_d_n6, assign10120_body11_e9272_d_n7, assign10120_body11_e9272_d_n10, assign10120_body11_e9272_d_n11, assign10120_body11_e9272_d_n12, assign10120_body11_e9272_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body11_e9266: f64 = (locals.var_d0).sqrt();
        let assign10120_body11_e9268: f64 = (assign10120_body11_e9266 * locals.var_d0);
        let assign10120_body11_e9269: f64 = (1.0 - assign10120_body11_e9268);
        let assign10120_body11_e9270: f64 = (p.p143 * assign10120_body11_e9269);
        (assign10120_body11_e9270, (p.p143 * (-(((locals.var_d0_dn0 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn0)))), (p.p143 * (-(((locals.var_d0_dn2 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn2)))), (p.p143 * (-(((locals.var_d0_dn6 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn6)))), (p.p143 * (-(((locals.var_d0_dn7 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn7)))), (p.p143 * (-(((locals.var_d0_dn10 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn10)))), (p.p143 * (-(((locals.var_d0_dn11 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn11)))), (p.p143 * (-(((locals.var_d0_dn12 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn12)))), (p.p143 * (-(((locals.var_d0_dn17 / (2.0 * assign10120_body11_e9266)) * locals.var_d0) + (assign10120_body11_e9266 * locals.var_d0_dn17)))),)
    } else {
        (locals.var_t1__blk194, locals.var_t1__blk194_dn0, locals.var_t1__blk194_dn2, locals.var_t1__blk194_dn6, locals.var_t1__blk194_dn7, locals.var_t1__blk194_dn10, locals.var_t1__blk194_dn11, locals.var_t1__blk194_dn12, locals.var_t1__blk194_dn17,)
    }
};
            locals.var_t1__blk194 = assign10120_body11_e9272;
            locals.var_t1__blk194_dn0 = assign10120_body11_e9272_d_n0;
            locals.var_t1__blk194_dn2 = assign10120_body11_e9272_d_n2;
            locals.var_t1__blk194_dn6 = assign10120_body11_e9272_d_n6;
            locals.var_t1__blk194_dn7 = assign10120_body11_e9272_d_n7;
            locals.var_t1__blk194_dn10 = assign10120_body11_e9272_d_n10;
            locals.var_t1__blk194_dn11 = assign10120_body11_e9272_d_n11;
            locals.var_t1__blk194_dn12 = assign10120_body11_e9272_d_n12;
            locals.var_t1__blk194_dn17 = assign10120_body11_e9272_d_n17;
            locals.var_t1__blk194_rv = 0.0;
            let (assign10120_body12_e9286, assign10120_body12_e9286_d_n0, assign10120_body12_e9286_d_n2, assign10120_body12_e9286_d_n6, assign10120_body12_e9286_d_n7, assign10120_body12_e9286_d_n10, assign10120_body12_e9286_d_n11, assign10120_body12_e9286_d_n12, assign10120_body12_e9286_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body12_e9282: f64 = (-locals.var_t1__blk194);
        let assign10120_body12_e9284: f64 = (assign10120_body12_e9282 / locals.var_t2__blk195);
        (assign10120_body12_e9284, ((((-locals.var_t1__blk194_dn0) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn0)) / (locals.var_t2__blk195 * locals.var_t2__blk195)), ((((-locals.var_t1__blk194_dn2) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn2)) / (locals.var_t2__blk195 * locals.var_t2__blk195)), ((((-locals.var_t1__blk194_dn6) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn6)) / (locals.var_t2__blk195 * locals.var_t2__blk195)), ((((-locals.var_t1__blk194_dn7) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn7)) / (locals.var_t2__blk195 * locals.var_t2__blk195)), ((((-locals.var_t1__blk194_dn10) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn10)) / (locals.var_t2__blk195 * locals.var_t2__blk195)), ((((-locals.var_t1__blk194_dn11) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn11)) / (locals.var_t2__blk195 * locals.var_t2__blk195)), ((((-locals.var_t1__blk194_dn12) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn12)) / (locals.var_t2__blk195 * locals.var_t2__blk195)), ((((-locals.var_t1__blk194_dn17) * locals.var_t2__blk195) - (assign10120_body12_e9282 * locals.var_t2__blk195_dn17)) / (locals.var_t2__blk195 * locals.var_t2__blk195)),)
    } else {
        (locals.var_t3__blk196, locals.var_t3__blk196_dn0, locals.var_t3__blk196_dn2, locals.var_t3__blk196_dn6, locals.var_t3__blk196_dn7, locals.var_t3__blk196_dn10, locals.var_t3__blk196_dn11, locals.var_t3__blk196_dn12, locals.var_t3__blk196_dn17,)
    }
};
            locals.var_t3__blk196 = assign10120_body12_e9286;
            locals.var_t3__blk196_dn0 = assign10120_body12_e9286_d_n0;
            locals.var_t3__blk196_dn2 = assign10120_body12_e9286_d_n2;
            locals.var_t3__blk196_dn6 = assign10120_body12_e9286_d_n6;
            locals.var_t3__blk196_dn7 = assign10120_body12_e9286_d_n7;
            locals.var_t3__blk196_dn10 = assign10120_body12_e9286_d_n10;
            locals.var_t3__blk196_dn11 = assign10120_body12_e9286_d_n11;
            locals.var_t3__blk196_dn12 = assign10120_body12_e9286_d_n12;
            locals.var_t3__blk196_dn17 = assign10120_body12_e9286_d_n17;
            locals.var_t3__blk196_rv = 0.0;
            let assign10120_body13_e9289: f64 = (-34.0);
            let assign10120_body13_e9290: f64 = if locals.var_t3__blk196 < assign10120_body13_e9289 { 1.0 } else { 0.0 };
            locals.var_guard207 = assign10120_body13_e9290;
            locals.var_guard207_rv = 0.0;
            let (assign10120_body14_e9303, assign10120_body14_e9303_d_n0, assign10120_body14_e9303_d_n2, assign10120_body14_e9303_d_n6, assign10120_body14_e9303_d_n7, assign10120_body14_e9303_d_n10, assign10120_body14_e9303_d_n11, assign10120_body14_e9303_d_n12, assign10120_body14_e9303_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard207 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk198, locals.var_t5__blk198_dn0, locals.var_t5__blk198_dn2, locals.var_t5__blk198_dn6, locals.var_t5__blk198_dn7, locals.var_t5__blk198_dn10, locals.var_t5__blk198_dn11, locals.var_t5__blk198_dn12, locals.var_t5__blk198_dn17,)
    }
};
            locals.var_t5__blk198 = assign10120_body14_e9303;
            locals.var_t5__blk198_dn0 = assign10120_body14_e9303_d_n0;
            locals.var_t5__blk198_dn2 = assign10120_body14_e9303_d_n2;
            locals.var_t5__blk198_dn6 = assign10120_body14_e9303_d_n6;
            locals.var_t5__blk198_dn7 = assign10120_body14_e9303_d_n7;
            locals.var_t5__blk198_dn10 = assign10120_body14_e9303_d_n10;
            locals.var_t5__blk198_dn11 = assign10120_body14_e9303_d_n11;
            locals.var_t5__blk198_dn12 = assign10120_body14_e9303_d_n12;
            locals.var_t5__blk198_dn17 = assign10120_body14_e9303_d_n17;
            locals.var_t5__blk198_rv = 0.0;
            let (assign10120_body15_e9318, assign10120_body15_e9318_d_n0, assign10120_body15_e9318_d_n2, assign10120_body15_e9318_d_n6, assign10120_body15_e9318_d_n7, assign10120_body15_e9318_d_n10, assign10120_body15_e9318_d_n11, assign10120_body15_e9318_d_n12, assign10120_body15_e9318_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard207 == 0.0)) {
        let assign10120_body15_e9316: f64 = (locals.var_t3__blk196).exp();
        (assign10120_body15_e9316, (assign10120_body15_e9316 * locals.var_t3__blk196_dn0), (assign10120_body15_e9316 * locals.var_t3__blk196_dn2), (assign10120_body15_e9316 * locals.var_t3__blk196_dn6), (assign10120_body15_e9316 * locals.var_t3__blk196_dn7), (assign10120_body15_e9316 * locals.var_t3__blk196_dn10), (assign10120_body15_e9316 * locals.var_t3__blk196_dn11), (assign10120_body15_e9316 * locals.var_t3__blk196_dn12), (assign10120_body15_e9316 * locals.var_t3__blk196_dn17),)
    } else {
        (locals.var_t5__blk198, locals.var_t5__blk198_dn0, locals.var_t5__blk198_dn2, locals.var_t5__blk198_dn6, locals.var_t5__blk198_dn7, locals.var_t5__blk198_dn10, locals.var_t5__blk198_dn11, locals.var_t5__blk198_dn12, locals.var_t5__blk198_dn17,)
    }
};
            locals.var_t5__blk198 = assign10120_body15_e9318;
            locals.var_t5__blk198_dn0 = assign10120_body15_e9318_d_n0;
            locals.var_t5__blk198_dn2 = assign10120_body15_e9318_d_n2;
            locals.var_t5__blk198_dn6 = assign10120_body15_e9318_d_n6;
            locals.var_t5__blk198_dn7 = assign10120_body15_e9318_d_n7;
            locals.var_t5__blk198_dn10 = assign10120_body15_e9318_d_n10;
            locals.var_t5__blk198_dn11 = assign10120_body15_e9318_d_n11;
            locals.var_t5__blk198_dn12 = assign10120_body15_e9318_d_n12;
            locals.var_t5__blk198_dn17 = assign10120_body15_e9318_d_n17;
            locals.var_t5__blk198_rv = 0.0;
            let (assign10120_body16_e9329, assign10120_body16_e9329_d_n0, assign10120_body16_e9329_d_n2, assign10120_body16_e9329_d_n6, assign10120_body16_e9329_d_n7, assign10120_body16_e9329_d_n10, assign10120_body16_e9329_d_n11, assign10120_body16_e9329_d_n12, assign10120_body16_e9329_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        (locals.var_evb1_qe_wl_p_egp12, locals.var_evb1_qe_wl_p_egp12_dn0, locals.var_evb1_qe_wl_p_egp12_dn2, locals.var_evb1_qe_wl_p_egp12_dn6, locals.var_evb1_qe_wl_p_egp12_dn7, locals.var_evb1_qe_wl_p_egp12_dn10, locals.var_evb1_qe_wl_p_egp12_dn11, locals.var_evb1_qe_wl_p_egp12_dn12, locals.var_evb1_qe_wl_p_egp12_dn17,)
    } else {
        (locals.var_t6__blk199, locals.var_t6__blk199_dn0, locals.var_t6__blk199_dn2, locals.var_t6__blk199_dn6, locals.var_t6__blk199_dn7, locals.var_t6__blk199_dn10, locals.var_t6__blk199_dn11, locals.var_t6__blk199_dn12, locals.var_t6__blk199_dn17,)
    }
};
            locals.var_t6__blk199 = assign10120_body16_e9329;
            locals.var_t6__blk199_dn0 = assign10120_body16_e9329_d_n0;
            locals.var_t6__blk199_dn2 = assign10120_body16_e9329_d_n2;
            locals.var_t6__blk199_dn6 = assign10120_body16_e9329_d_n6;
            locals.var_t6__blk199_dn7 = assign10120_body16_e9329_d_n7;
            locals.var_t6__blk199_dn10 = assign10120_body16_e9329_d_n10;
            locals.var_t6__blk199_dn11 = assign10120_body16_e9329_d_n11;
            locals.var_t6__blk199_dn12 = assign10120_body16_e9329_d_n12;
            locals.var_t6__blk199_dn17 = assign10120_body16_e9329_d_n17;
            locals.var_t6__blk199_rv = 0.0;
            let (assign10120_body17_e9348, assign10120_body17_e9348_d_n0, assign10120_body17_e9348_d_n2, assign10120_body17_e9348_d_n6, assign10120_body17_e9348_d_n7, assign10120_body17_e9348_d_n10, assign10120_body17_e9348_d_n11, assign10120_body17_e9348_d_n12, assign10120_body17_e9348_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body17_e9340: f64 = (0.25 * locals.var_t6__blk199);
        let assign10120_body17_e9342: f64 = (assign10120_body17_e9340 * locals.var_t1__blk194);
        let assign10120_body17_e9344: f64 = (assign10120_body17_e9342 * locals.var_t1__blk194);
        let assign10120_body17_e9346: f64 = (assign10120_body17_e9344 * 7.38905609893065);
        (assign10120_body17_e9346, ((((((0.25 * locals.var_t6__blk199_dn0) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn0)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn0)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk199_dn2) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn2)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn2)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk199_dn6) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn6)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn6)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk199_dn7) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn7)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn7)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk199_dn10) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn10)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn10)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk199_dn11) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn11)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn11)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk199_dn12) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn12)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn12)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk199_dn17) * locals.var_t1__blk194) + (assign10120_body17_e9340 * locals.var_t1__blk194_dn17)) * locals.var_t1__blk194) + (assign10120_body17_e9342 * locals.var_t1__blk194_dn17)) * 7.38905609893065),)
    } else {
        (locals.var_t7__blk200, locals.var_t7__blk200_dn0, locals.var_t7__blk200_dn2, locals.var_t7__blk200_dn6, locals.var_t7__blk200_dn7, locals.var_t7__blk200_dn10, locals.var_t7__blk200_dn11, locals.var_t7__blk200_dn12, locals.var_t7__blk200_dn17,)
    }
};
            locals.var_t7__blk200 = assign10120_body17_e9348;
            locals.var_t7__blk200_dn0 = assign10120_body17_e9348_d_n0;
            locals.var_t7__blk200_dn2 = assign10120_body17_e9348_d_n2;
            locals.var_t7__blk200_dn6 = assign10120_body17_e9348_d_n6;
            locals.var_t7__blk200_dn7 = assign10120_body17_e9348_d_n7;
            locals.var_t7__blk200_dn10 = assign10120_body17_e9348_d_n10;
            locals.var_t7__blk200_dn11 = assign10120_body17_e9348_d_n11;
            locals.var_t7__blk200_dn12 = assign10120_body17_e9348_d_n12;
            locals.var_t7__blk200_dn17 = assign10120_body17_e9348_d_n17;
            locals.var_t7__blk200_rv = 0.0;
            let assign10120_body18_e9351: f64 = (2.0 * locals.var_t2__blk195);
            let assign10120_body18_e9353: f64 = (assign10120_body18_e9351 + locals.var_t1__blk194);
            let assign10120_body18_e9355: f64 = if assign10120_body18_e9353 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard208 = assign10120_body18_e9355;
            locals.var_guard208_rv = 0.0;
            let (assign10120_body19_e9368, assign10120_body19_e9368_d_n0, assign10120_body19_e9368_d_n2, assign10120_body19_e9368_d_n6, assign10120_body19_e9368_d_n7, assign10120_body19_e9368_d_n10, assign10120_body19_e9368_d_n11, assign10120_body19_e9368_d_n12, assign10120_body19_e9368_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard208 != 0.0)) {
        (locals.var_t7__blk200, locals.var_t7__blk200_dn0, locals.var_t7__blk200_dn2, locals.var_t7__blk200_dn6, locals.var_t7__blk200_dn7, locals.var_t7__blk200_dn10, locals.var_t7__blk200_dn11, locals.var_t7__blk200_dn12, locals.var_t7__blk200_dn17,)
    } else {
        (locals.var_ievb0, locals.var_ievb0_dn0, locals.var_ievb0_dn2, locals.var_ievb0_dn6, locals.var_ievb0_dn7, locals.var_ievb0_dn10, locals.var_ievb0_dn11, locals.var_ievb0_dn12, locals.var_ievb0_dn17,)
    }
};
            locals.var_ievb0 = assign10120_body19_e9368;
            locals.var_ievb0_dn0 = assign10120_body19_e9368_d_n0;
            locals.var_ievb0_dn2 = assign10120_body19_e9368_d_n2;
            locals.var_ievb0_dn6 = assign10120_body19_e9368_d_n6;
            locals.var_ievb0_dn7 = assign10120_body19_e9368_d_n7;
            locals.var_ievb0_dn10 = assign10120_body19_e9368_d_n10;
            locals.var_ievb0_dn11 = assign10120_body19_e9368_d_n11;
            locals.var_ievb0_dn12 = assign10120_body19_e9368_d_n12;
            locals.var_ievb0_dn17 = assign10120_body19_e9368_d_n17;
            locals.var_ievb0_rv = 0.0;
            let (assign10120_body20_e9382,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard208 == 0.0)) {
        (locals.var_evb1_qe_wl,)
    } else {
        (locals.var_t4__blk197,)
    }
};
            locals.var_t4__blk197 = assign10120_body20_e9382;
            locals.var_t4__blk197_rv = 0.0;
            let (assign10120_body21_e9400, assign10120_body21_e9400_d_n0, assign10120_body21_e9400_d_n2, assign10120_body21_e9400_d_n6, assign10120_body21_e9400_d_n7, assign10120_body21_e9400_d_n10, assign10120_body21_e9400_d_n11, assign10120_body21_e9400_d_n12, assign10120_body21_e9400_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard208 == 0.0)) {
        let assign10120_body21_e9396: f64 = (locals.var_t4__blk197 * locals.var_t0__blk193);
        let assign10120_body21_e9398: f64 = (assign10120_body21_e9396 * locals.var_t5__blk198);
        (assign10120_body21_e9398, (((locals.var_t4__blk197 * locals.var_t0__blk193_dn0) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn0)), (((locals.var_t4__blk197 * locals.var_t0__blk193_dn2) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn2)), (((locals.var_t4__blk197 * locals.var_t0__blk193_dn6) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn6)), (((locals.var_t4__blk197 * locals.var_t0__blk193_dn7) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn7)), (((locals.var_t4__blk197 * locals.var_t0__blk193_dn10) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn10)), (((locals.var_t4__blk197 * locals.var_t0__blk193_dn11) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn11)), (((locals.var_t4__blk197 * locals.var_t0__blk193_dn12) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn12)), (((locals.var_t4__blk197 * locals.var_t0__blk193_dn17) * locals.var_t5__blk198) + (assign10120_body21_e9396 * locals.var_t5__blk198_dn17)),)
    } else {
        (locals.var_t8__blk201, locals.var_t8__blk201_dn0, locals.var_t8__blk201_dn2, locals.var_t8__blk201_dn6, locals.var_t8__blk201_dn7, locals.var_t8__blk201_dn10, locals.var_t8__blk201_dn11, locals.var_t8__blk201_dn12, locals.var_t8__blk201_dn17,)
    }
};
            locals.var_t8__blk201 = assign10120_body21_e9400;
            locals.var_t8__blk201_dn0 = assign10120_body21_e9400_d_n0;
            locals.var_t8__blk201_dn2 = assign10120_body21_e9400_d_n2;
            locals.var_t8__blk201_dn6 = assign10120_body21_e9400_d_n6;
            locals.var_t8__blk201_dn7 = assign10120_body21_e9400_d_n7;
            locals.var_t8__blk201_dn10 = assign10120_body21_e9400_d_n10;
            locals.var_t8__blk201_dn11 = assign10120_body21_e9400_d_n11;
            locals.var_t8__blk201_dn12 = assign10120_body21_e9400_d_n12;
            locals.var_t8__blk201_dn17 = assign10120_body21_e9400_d_n17;
            locals.var_t8__blk201_rv = 0.0;
            let assign10120_body22_e9407: f64 = if ((locals.var_t8__blk201 < locals.var_t7__blk200) || (locals.var_t2__blk195 < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard209 = assign10120_body22_e9407;
            locals.var_guard209_rv = 0.0;
            let (assign10120_body23_e9423, assign10120_body23_e9423_d_n0, assign10120_body23_e9423_d_n2, assign10120_body23_e9423_d_n6, assign10120_body23_e9423_d_n7, assign10120_body23_e9423_d_n10, assign10120_body23_e9423_d_n11, assign10120_body23_e9423_d_n12, assign10120_body23_e9423_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard208 == 0.0)) && (locals.var_guard209 != 0.0)) {
        (locals.var_t7__blk200, locals.var_t7__blk200_dn0, locals.var_t7__blk200_dn2, locals.var_t7__blk200_dn6, locals.var_t7__blk200_dn7, locals.var_t7__blk200_dn10, locals.var_t7__blk200_dn11, locals.var_t7__blk200_dn12, locals.var_t7__blk200_dn17,)
    } else {
        (locals.var_ievb0, locals.var_ievb0_dn0, locals.var_ievb0_dn2, locals.var_ievb0_dn6, locals.var_ievb0_dn7, locals.var_ievb0_dn10, locals.var_ievb0_dn11, locals.var_ievb0_dn12, locals.var_ievb0_dn17,)
    }
};
            locals.var_ievb0 = assign10120_body23_e9423;
            locals.var_ievb0_dn0 = assign10120_body23_e9423_d_n0;
            locals.var_ievb0_dn2 = assign10120_body23_e9423_d_n2;
            locals.var_ievb0_dn6 = assign10120_body23_e9423_d_n6;
            locals.var_ievb0_dn7 = assign10120_body23_e9423_d_n7;
            locals.var_ievb0_dn10 = assign10120_body23_e9423_d_n10;
            locals.var_ievb0_dn11 = assign10120_body23_e9423_d_n11;
            locals.var_ievb0_dn12 = assign10120_body23_e9423_d_n12;
            locals.var_ievb0_dn17 = assign10120_body23_e9423_d_n17;
            locals.var_ievb0_rv = 0.0;
            let (assign10120_body24_e9440, assign10120_body24_e9440_d_n0, assign10120_body24_e9440_d_n2, assign10120_body24_e9440_d_n6, assign10120_body24_e9440_d_n7, assign10120_body24_e9440_d_n10, assign10120_body24_e9440_d_n11, assign10120_body24_e9440_d_n12, assign10120_body24_e9440_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard208 == 0.0)) && (locals.var_guard209 == 0.0)) {
        (locals.var_t8__blk201, locals.var_t8__blk201_dn0, locals.var_t8__blk201_dn2, locals.var_t8__blk201_dn6, locals.var_t8__blk201_dn7, locals.var_t8__blk201_dn10, locals.var_t8__blk201_dn11, locals.var_t8__blk201_dn12, locals.var_t8__blk201_dn17,)
    } else {
        (locals.var_ievb0, locals.var_ievb0_dn0, locals.var_ievb0_dn2, locals.var_ievb0_dn6, locals.var_ievb0_dn7, locals.var_ievb0_dn10, locals.var_ievb0_dn11, locals.var_ievb0_dn12, locals.var_ievb0_dn17,)
    }
};
            locals.var_ievb0 = assign10120_body24_e9440;
            locals.var_ievb0_dn0 = assign10120_body24_e9440_d_n0;
            locals.var_ievb0_dn2 = assign10120_body24_e9440_d_n2;
            locals.var_ievb0_dn6 = assign10120_body24_e9440_d_n6;
            locals.var_ievb0_dn7 = assign10120_body24_e9440_d_n7;
            locals.var_ievb0_dn10 = assign10120_body24_e9440_d_n10;
            locals.var_ievb0_dn11 = assign10120_body24_e9440_d_n11;
            locals.var_ievb0_dn12 = assign10120_body24_e9440_d_n12;
            locals.var_ievb0_dn17 = assign10120_body24_e9440_d_n17;
            locals.var_ievb0_rv = 0.0;
            let assign10120_body26_e9456: f64 = if locals.var_ievb0 < 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard210 = assign10120_body26_e9456;
            locals.var_guard210_rv = 0.0;
            let (assign10120_body27_e9469,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard210 != 0.0)) {
        (100.0,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10120_body27_e9469;
            locals.var_i_rv = 0.0;
            let (assign10120_body28_e9482,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) && (locals.var_guard210 != 0.0)) {
        (locals.var_lp_s0_max,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign10120_body28_e9482;
            locals.var_lp_s0_rv = 0.0;
            let (assign10120_body29_e9495,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign10120_body29_e9493: f64 = (locals.var_i + 1.0);
        (assign10120_body29_e9493,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10120_body29_e9495;
            locals.var_i_rv = 0.0;
        }

        let assign10130_e9502: f64 = if ((p.p117 <= 0.0) || (locals.var_mks_vmax <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard223 = assign10130_e9502;
        locals.var_guard223_rv = 0.0;

        let (assign10140_e9512, assign10140_e9512_d_n0, assign10140_e9512_d_n2, assign10140_e9512_d_n6, assign10140_e9512_d_n7, assign10140_e9512_d_n10, assign10140_e9512_d_n11, assign10140_e9512_d_n12, assign10140_e9512_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign10140_e9512;
        locals.var_isub_dn0 = assign10140_e9512_d_n0;
        locals.var_isub_dn2 = assign10140_e9512_d_n2;
        locals.var_isub_dn6 = assign10140_e9512_d_n6;
        locals.var_isub_dn7 = assign10140_e9512_d_n7;
        locals.var_isub_dn10 = assign10140_e9512_d_n10;
        locals.var_isub_dn11 = assign10140_e9512_d_n11;
        locals.var_isub_dn12 = assign10140_e9512_d_n12;
        locals.var_isub_dn17 = assign10140_e9512_d_n17;
        locals.var_isub_rv = 0.0;

        let assign10150_e9515: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard224 = assign10150_e9515;
        locals.var_guard224_rv = 0.0;

        let (assign10160_e9528, assign10160_e9528_d_n0, assign10160_e9528_d_n2, assign10160_e9528_d_n6, assign10160_e9528_d_n7, assign10160_e9528_d_n10, assign10160_e9528_d_n11, assign10160_e9528_d_n12, assign10160_e9528_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_t1__blk211, locals.var_t1__blk211_dn0, locals.var_t1__blk211_dn2, locals.var_t1__blk211_dn6, locals.var_t1__blk211_dn7, locals.var_t1__blk211_dn10, locals.var_t1__blk211_dn11, locals.var_t1__blk211_dn12, locals.var_t1__blk211_dn17,)
    }
};
        locals.var_t1__blk211 = assign10160_e9528;
        locals.var_t1__blk211_dn0 = assign10160_e9528_d_n0;
        locals.var_t1__blk211_dn2 = assign10160_e9528_d_n2;
        locals.var_t1__blk211_dn6 = assign10160_e9528_d_n6;
        locals.var_t1__blk211_dn7 = assign10160_e9528_d_n7;
        locals.var_t1__blk211_dn10 = assign10160_e9528_d_n10;
        locals.var_t1__blk211_dn11 = assign10160_e9528_d_n11;
        locals.var_t1__blk211_dn12 = assign10160_e9528_d_n12;
        locals.var_t1__blk211_dn17 = assign10160_e9528_d_n17;
        locals.var_t1__blk211_rv = 0.0;

        let (assign10170_e9543, assign10170_e9543_d_n0, assign10170_e9543_d_n2, assign10170_e9543_d_n6, assign10170_e9543_d_n7, assign10170_e9543_d_n10, assign10170_e9543_d_n11, assign10170_e9543_d_n12, assign10170_e9543_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10170_e9541: f64 = (locals.var_c_fox * locals.var_c_fox);
        (assign10170_e9541, ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)), ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)), ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)), ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)), ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)), ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)), ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)), ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_t7__blk218, locals.var_t7__blk218_dn0, locals.var_t7__blk218_dn2, locals.var_t7__blk218_dn6, locals.var_t7__blk218_dn7, locals.var_t7__blk218_dn10, locals.var_t7__blk218_dn11, locals.var_t7__blk218_dn12, locals.var_t7__blk218_dn17,)
    }
};
        locals.var_t7__blk218 = assign10170_e9543;
        locals.var_t7__blk218_dn0 = assign10170_e9543_d_n0;
        locals.var_t7__blk218_dn2 = assign10170_e9543_d_n2;
        locals.var_t7__blk218_dn6 = assign10170_e9543_d_n6;
        locals.var_t7__blk218_dn7 = assign10170_e9543_d_n7;
        locals.var_t7__blk218_dn10 = assign10170_e9543_d_n10;
        locals.var_t7__blk218_dn11 = assign10170_e9543_d_n11;
        locals.var_t7__blk218_dn12 = assign10170_e9543_d_n12;
        locals.var_t7__blk218_dn17 = assign10170_e9543_d_n17;
        locals.var_t7__blk218_rv = 0.0;

        let (assign10180_e9556, assign10180_e9556_d_n0, assign10180_e9556_d_n2, assign10180_e9556_d_n6, assign10180_e9556_d_n7, assign10180_e9556_d_n10, assign10180_e9556_d_n11, assign10180_e9556_d_n12, assign10180_e9556_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        (locals.var_qnsub_esi, locals.var_qnsub_esi_dn0, locals.var_qnsub_esi_dn2, locals.var_qnsub_esi_dn6, locals.var_qnsub_esi_dn7, locals.var_qnsub_esi_dn10, locals.var_qnsub_esi_dn11, locals.var_qnsub_esi_dn12, locals.var_qnsub_esi_dn17,)
    } else {
        (locals.var_t8__blk219, locals.var_t8__blk219_dn0, locals.var_t8__blk219_dn2, locals.var_t8__blk219_dn6, locals.var_t8__blk219_dn7, locals.var_t8__blk219_dn10, locals.var_t8__blk219_dn11, locals.var_t8__blk219_dn12, locals.var_t8__blk219_dn17,)
    }
};
        locals.var_t8__blk219 = assign10180_e9556;
        locals.var_t8__blk219_dn0 = assign10180_e9556_d_n0;
        locals.var_t8__blk219_dn2 = assign10180_e9556_d_n2;
        locals.var_t8__blk219_dn6 = assign10180_e9556_d_n6;
        locals.var_t8__blk219_dn7 = assign10180_e9556_d_n7;
        locals.var_t8__blk219_dn10 = assign10180_e9556_d_n10;
        locals.var_t8__blk219_dn11 = assign10180_e9556_d_n11;
        locals.var_t8__blk219_dn12 = assign10180_e9556_d_n12;
        locals.var_t8__blk219_dn17 = assign10180_e9556_d_n17;
        locals.var_t8__blk219_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10190_e9571, assign10190_e9571_d_n0, assign10190_e9571_d_n2, assign10190_e9571_d_n6, assign10190_e9571_d_n7, assign10190_e9571_d_n10, assign10190_e9571_d_n11, assign10190_e9571_d_n12, assign10190_e9571_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10190_e9569: f64 = (locals.var_t8__blk219 / locals.var_t7__blk218);
        (assign10190_e9569, (((locals.var_t8__blk219_dn0 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn0)) / (locals.var_t7__blk218 * locals.var_t7__blk218)), (((locals.var_t8__blk219_dn2 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn2)) / (locals.var_t7__blk218 * locals.var_t7__blk218)), (((locals.var_t8__blk219_dn6 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn6)) / (locals.var_t7__blk218 * locals.var_t7__blk218)), (((locals.var_t8__blk219_dn7 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn7)) / (locals.var_t7__blk218 * locals.var_t7__blk218)), (((locals.var_t8__blk219_dn10 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn10)) / (locals.var_t7__blk218 * locals.var_t7__blk218)), (((locals.var_t8__blk219_dn11 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn11)) / (locals.var_t7__blk218 * locals.var_t7__blk218)), (((locals.var_t8__blk219_dn12 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn12)) / (locals.var_t7__blk218 * locals.var_t7__blk218)), (((locals.var_t8__blk219_dn17 * locals.var_t7__blk218) - (locals.var_t8__blk219 * locals.var_t7__blk218_dn17)) / (locals.var_t7__blk218 * locals.var_t7__blk218)),)
    } else {
        (locals.var_t3__blk213, locals.var_t3__blk213_dn0, locals.var_t3__blk213_dn2, locals.var_t3__blk213_dn6, locals.var_t3__blk213_dn7, locals.var_t3__blk213_dn10, locals.var_t3__blk213_dn11, locals.var_t3__blk213_dn12, locals.var_t3__blk213_dn17,)
    }
};
        locals.var_t3__blk213 = assign10190_e9571;
        locals.var_t3__blk213_dn0 = assign10190_e9571_d_n0;
        locals.var_t3__blk213_dn2 = assign10190_e9571_d_n2;
        locals.var_t3__blk213_dn6 = assign10190_e9571_d_n6;
        locals.var_t3__blk213_dn7 = assign10190_e9571_d_n7;
        locals.var_t3__blk213_dn10 = assign10190_e9571_d_n10;
        locals.var_t3__blk213_dn11 = assign10190_e9571_d_n11;
        locals.var_t3__blk213_dn12 = assign10190_e9571_d_n12;
        locals.var_t3__blk213_dn17 = assign10190_e9571_d_n17;
        locals.var_t3__blk213_rv = 0.0;

        let (assign10200_e9586, assign10200_e9586_d_n0, assign10200_e9586_d_n2, assign10200_e9586_d_n6, assign10200_e9586_d_n7, assign10200_e9586_d_n10, assign10200_e9586_d_n11, assign10200_e9586_d_n12, assign10200_e9586_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10200_e9584: f64 = (2.0 / locals.var_t8__blk219);
        (assign10200_e9584, (-((2.0 * locals.var_t8__blk219_dn0) / (locals.var_t8__blk219 * locals.var_t8__blk219))), (-((2.0 * locals.var_t8__blk219_dn2) / (locals.var_t8__blk219 * locals.var_t8__blk219))), (-((2.0 * locals.var_t8__blk219_dn6) / (locals.var_t8__blk219 * locals.var_t8__blk219))), (-((2.0 * locals.var_t8__blk219_dn7) / (locals.var_t8__blk219 * locals.var_t8__blk219))), (-((2.0 * locals.var_t8__blk219_dn10) / (locals.var_t8__blk219 * locals.var_t8__blk219))), (-((2.0 * locals.var_t8__blk219_dn11) / (locals.var_t8__blk219 * locals.var_t8__blk219))), (-((2.0 * locals.var_t8__blk219_dn12) / (locals.var_t8__blk219 * locals.var_t8__blk219))), (-((2.0 * locals.var_t8__blk219_dn17) / (locals.var_t8__blk219 * locals.var_t8__blk219))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12, locals.var_t9_dn17,)
    }
};
        locals.var_t9 = assign10200_e9586;
        locals.var_t9_dn0 = assign10200_e9586_d_n0;
        locals.var_t9_dn2 = assign10200_e9586_d_n2;
        locals.var_t9_dn6 = assign10200_e9586_d_n6;
        locals.var_t9_dn7 = assign10200_e9586_d_n7;
        locals.var_t9_dn10 = assign10200_e9586_d_n10;
        locals.var_t9_dn11 = assign10200_e9586_d_n11;
        locals.var_t9_dn12 = assign10200_e9586_d_n12;
        locals.var_t9_dn17 = assign10200_e9586_d_n17;
        locals.var_t9_rv = 0.0;

        let (assign10210_e9601, assign10210_e9601_d_n0, assign10210_e9601_d_n2, assign10210_e9601_d_n6, assign10210_e9601_d_n7, assign10210_e9601_d_n10, assign10210_e9601_d_n11, assign10210_e9601_d_n12, assign10210_e9601_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10210_e9599: f64 = (locals.var_t9 * locals.var_t7__blk218);
        (assign10210_e9599, ((locals.var_t9_dn0 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn0)), ((locals.var_t9_dn2 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn2)), ((locals.var_t9_dn6 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn6)), ((locals.var_t9_dn7 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn7)), ((locals.var_t9_dn10 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn10)), ((locals.var_t9_dn11 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn11)), ((locals.var_t9_dn12 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn12)), ((locals.var_t9_dn17 * locals.var_t7__blk218) + (locals.var_t9 * locals.var_t7__blk218_dn17)),)
    } else {
        (locals.var_t4__blk214, locals.var_t4__blk214_dn0, locals.var_t4__blk214_dn2, locals.var_t4__blk214_dn6, locals.var_t4__blk214_dn7, locals.var_t4__blk214_dn10, locals.var_t4__blk214_dn11, locals.var_t4__blk214_dn12, locals.var_t4__blk214_dn17,)
    }
};
        locals.var_t4__blk214 = assign10210_e9601;
        locals.var_t4__blk214_dn0 = assign10210_e9601_d_n0;
        locals.var_t4__blk214_dn2 = assign10210_e9601_d_n2;
        locals.var_t4__blk214_dn6 = assign10210_e9601_d_n6;
        locals.var_t4__blk214_dn7 = assign10210_e9601_d_n7;
        locals.var_t4__blk214_dn10 = assign10210_e9601_d_n10;
        locals.var_t4__blk214_dn11 = assign10210_e9601_d_n11;
        locals.var_t4__blk214_dn12 = assign10210_e9601_d_n12;
        locals.var_t4__blk214_dn17 = assign10210_e9601_d_n17;
        locals.var_t4__blk214_rv = 0.0;

        let (assign10220_e9620, assign10220_e9620_d_n0, assign10220_e9620_d_n2, assign10220_e9620_d_n6, assign10220_e9620_d_n7, assign10220_e9620_d_n10, assign10220_e9620_d_n11, assign10220_e9620_d_n12, assign10220_e9620_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10220_e9614: f64 = (locals.var_t1__blk211 - locals.var_beta_inv);
        let assign10220_e9617: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign10220_e9618: f64 = (assign10220_e9614 - assign10220_e9617);
        (assign10220_e9618, (locals.var_t1__blk211_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk211_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk211_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk211_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk211_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk211_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk211_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk211_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk215, locals.var_t5__blk215_dn0, locals.var_t5__blk215_dn2, locals.var_t5__blk215_dn6, locals.var_t5__blk215_dn7, locals.var_t5__blk215_dn10, locals.var_t5__blk215_dn11, locals.var_t5__blk215_dn12, locals.var_t5__blk215_dn17,)
    }
};
        locals.var_t5__blk215 = assign10220_e9620;
        locals.var_t5__blk215_dn0 = assign10220_e9620_d_n0;
        locals.var_t5__blk215_dn2 = assign10220_e9620_d_n2;
        locals.var_t5__blk215_dn6 = assign10220_e9620_d_n6;
        locals.var_t5__blk215_dn7 = assign10220_e9620_d_n7;
        locals.var_t5__blk215_dn10 = assign10220_e9620_d_n10;
        locals.var_t5__blk215_dn11 = assign10220_e9620_d_n11;
        locals.var_t5__blk215_dn12 = assign10220_e9620_d_n12;
        locals.var_t5__blk215_dn17 = assign10220_e9620_d_n17;
        locals.var_t5__blk215_rv = 0.0;

        let (assign10230_e9637, assign10230_e9637_d_n0, assign10230_e9637_d_n2, assign10230_e9637_d_n6, assign10230_e9637_d_n7, assign10230_e9637_d_n10, assign10230_e9637_d_n11, assign10230_e9637_d_n12, assign10230_e9637_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10230_e9634: f64 = (locals.var_t4__blk214 * locals.var_t5__blk215);
        let assign10230_e9635: f64 = (1.0 + assign10230_e9634);
        (assign10230_e9635, ((locals.var_t4__blk214_dn0 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn0)), ((locals.var_t4__blk214_dn2 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn2)), ((locals.var_t4__blk214_dn6 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn6)), ((locals.var_t4__blk214_dn7 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn7)), ((locals.var_t4__blk214_dn10 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn10)), ((locals.var_t4__blk214_dn11 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn11)), ((locals.var_t4__blk214_dn12 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn12)), ((locals.var_t4__blk214_dn17 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn17)),)
    } else {
        (locals.var_t6w, locals.var_t6w_dn0, locals.var_t6w_dn2, locals.var_t6w_dn6, locals.var_t6w_dn7, locals.var_t6w_dn10, locals.var_t6w_dn11, locals.var_t6w_dn12, locals.var_t6w_dn17,)
    }
};
        locals.var_t6w = assign10230_e9637;
        locals.var_t6w_dn0 = assign10230_e9637_d_n0;
        locals.var_t6w_dn2 = assign10230_e9637_d_n2;
        locals.var_t6w_dn6 = assign10230_e9637_d_n6;
        locals.var_t6w_dn7 = assign10230_e9637_d_n7;
        locals.var_t6w_dn10 = assign10230_e9637_d_n10;
        locals.var_t6w_dn11 = assign10230_e9637_d_n11;
        locals.var_t6w_dn12 = assign10230_e9637_d_n12;
        locals.var_t6w_dn17 = assign10230_e9637_d_n17;
        locals.var_t6w_rv = 0.0;

        let (assign10240_e9659, assign10240_e9659_d_n0, assign10240_e9659_d_n2, assign10240_e9659_d_n6, assign10240_e9659_d_n7, assign10240_e9659_d_n10, assign10240_e9659_d_n11, assign10240_e9659_d_n12, assign10240_e9659_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10240_e9650: f64 = (locals.var_t6w * locals.var_t6w);
        let assign10240_e9653: f64 = (4.0 * 0.001);
        let assign10240_e9655: f64 = (assign10240_e9653 * 0.001);
        let assign10240_e9656: f64 = (assign10240_e9650 + assign10240_e9655);
        let assign10240_e9657: f64 = (assign10240_e9656).sqrt();
        (assign10240_e9657, (((locals.var_t6w_dn0 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn0)) / (2.0 * assign10240_e9657)), (((locals.var_t6w_dn2 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn2)) / (2.0 * assign10240_e9657)), (((locals.var_t6w_dn6 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn6)) / (2.0 * assign10240_e9657)), (((locals.var_t6w_dn7 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn7)) / (2.0 * assign10240_e9657)), (((locals.var_t6w_dn10 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn10)) / (2.0 * assign10240_e9657)), (((locals.var_t6w_dn11 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn11)) / (2.0 * assign10240_e9657)), (((locals.var_t6w_dn12 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn12)) / (2.0 * assign10240_e9657)), (((locals.var_t6w_dn17 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn17)) / (2.0 * assign10240_e9657)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign10240_e9659;
        locals.var_tmf1_dn0 = assign10240_e9659_d_n0;
        locals.var_tmf1_dn2 = assign10240_e9659_d_n2;
        locals.var_tmf1_dn6 = assign10240_e9659_d_n6;
        locals.var_tmf1_dn7 = assign10240_e9659_d_n7;
        locals.var_tmf1_dn10 = assign10240_e9659_d_n10;
        locals.var_tmf1_dn11 = assign10240_e9659_d_n11;
        locals.var_tmf1_dn12 = assign10240_e9659_d_n12;
        locals.var_tmf1_dn17 = assign10240_e9659_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign10250_e9680, assign10250_e9680_d_n0, assign10250_e9680_d_n2, assign10250_e9680_d_n6, assign10250_e9680_d_n7, assign10250_e9680_d_n10, assign10250_e9680_d_n11, assign10250_e9680_d_n12, assign10250_e9680_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10250_e9673: f64 = (locals.var_t6w + locals.var_tmf1);
        let assign10250_e9674: f64 = (0.5 * assign10250_e9673);
        let assign10250_e9677: f64 = (1e-10 * 0.001);
        let assign10250_e9678: f64 = (assign10250_e9674 + assign10250_e9677);
        (assign10250_e9678, (0.5 * (locals.var_t6w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk216, locals.var_t6__blk216_dn0, locals.var_t6__blk216_dn2, locals.var_t6__blk216_dn6, locals.var_t6__blk216_dn7, locals.var_t6__blk216_dn10, locals.var_t6__blk216_dn11, locals.var_t6__blk216_dn12, locals.var_t6__blk216_dn17,)
    }
};
        locals.var_t6__blk216 = assign10250_e9680;
        locals.var_t6__blk216_dn0 = assign10250_e9680_d_n0;
        locals.var_t6__blk216_dn2 = assign10250_e9680_d_n2;
        locals.var_t6__blk216_dn6 = assign10250_e9680_d_n6;
        locals.var_t6__blk216_dn7 = assign10250_e9680_d_n7;
        locals.var_t6__blk216_dn10 = assign10250_e9680_d_n10;
        locals.var_t6__blk216_dn11 = assign10250_e9680_d_n11;
        locals.var_t6__blk216_dn12 = assign10250_e9680_d_n12;
        locals.var_t6__blk216_dn17 = assign10250_e9680_d_n17;
        locals.var_t6__blk216_rv = 0.0;

        let assign10260_e9683: f64 = if locals.var_t6__blk216 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard225 = assign10260_e9683;
        locals.var_guard225_rv = 0.0;

        let (assign10270_e9698, assign10270_e9698_d_n0, assign10270_e9698_d_n2, assign10270_e9698_d_n6, assign10270_e9698_d_n7, assign10270_e9698_d_n10, assign10270_e9698_d_n11, assign10270_e9698_d_n12, assign10270_e9698_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) && (locals.var_guard225 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk216, locals.var_t6__blk216_dn0, locals.var_t6__blk216_dn2, locals.var_t6__blk216_dn6, locals.var_t6__blk216_dn7, locals.var_t6__blk216_dn10, locals.var_t6__blk216_dn11, locals.var_t6__blk216_dn12, locals.var_t6__blk216_dn17,)
    }
};
        locals.var_t6__blk216 = assign10270_e9698;
        locals.var_t6__blk216_dn0 = assign10270_e9698_d_n0;
        locals.var_t6__blk216_dn2 = assign10270_e9698_d_n2;
        locals.var_t6__blk216_dn6 = assign10270_e9698_d_n6;
        locals.var_t6__blk216_dn7 = assign10270_e9698_d_n7;
        locals.var_t6__blk216_dn10 = assign10270_e9698_d_n10;
        locals.var_t6__blk216_dn11 = assign10270_e9698_d_n11;
        locals.var_t6__blk216_dn12 = assign10270_e9698_d_n12;
        locals.var_t6__blk216_dn17 = assign10270_e9698_d_n17;
        locals.var_t6__blk216_rv = 0.0;

        let (assign10280_e9713, assign10280_e9713_d_n0, assign10280_e9713_d_n2, assign10280_e9713_d_n6, assign10280_e9713_d_n7, assign10280_e9713_d_n10, assign10280_e9713_d_n11, assign10280_e9713_d_n12, assign10280_e9713_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10280_e9711: f64 = (locals.var_t6__blk216 + 1e-50);
        (assign10280_e9711, locals.var_t6__blk216_dn0, locals.var_t6__blk216_dn2, locals.var_t6__blk216_dn6, locals.var_t6__blk216_dn7, locals.var_t6__blk216_dn10, locals.var_t6__blk216_dn11, locals.var_t6__blk216_dn12, locals.var_t6__blk216_dn17,)
    } else {
        (locals.var_t6__blk216, locals.var_t6__blk216_dn0, locals.var_t6__blk216_dn2, locals.var_t6__blk216_dn6, locals.var_t6__blk216_dn7, locals.var_t6__blk216_dn10, locals.var_t6__blk216_dn11, locals.var_t6__blk216_dn12, locals.var_t6__blk216_dn17,)
    }
};
        locals.var_t6__blk216 = assign10280_e9713;
        locals.var_t6__blk216_dn0 = assign10280_e9713_d_n0;
        locals.var_t6__blk216_dn2 = assign10280_e9713_d_n2;
        locals.var_t6__blk216_dn6 = assign10280_e9713_d_n6;
        locals.var_t6__blk216_dn7 = assign10280_e9713_d_n7;
        locals.var_t6__blk216_dn10 = assign10280_e9713_d_n10;
        locals.var_t6__blk216_dn11 = assign10280_e9713_d_n11;
        locals.var_t6__blk216_dn12 = assign10280_e9713_d_n12;
        locals.var_t6__blk216_dn17 = assign10280_e9713_d_n17;
        locals.var_t6__blk216_rv = 0.0;

        let (assign10290_e9727, assign10290_e9727_d_n0, assign10290_e9727_d_n2, assign10290_e9727_d_n6, assign10290_e9727_d_n7, assign10290_e9727_d_n10, assign10290_e9727_d_n11, assign10290_e9727_d_n12, assign10290_e9727_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10290_e9725: f64 = (locals.var_t6__blk216).sqrt();
        (assign10290_e9725, (locals.var_t6__blk216_dn0 / (2.0 * assign10290_e9725)), (locals.var_t6__blk216_dn2 / (2.0 * assign10290_e9725)), (locals.var_t6__blk216_dn6 / (2.0 * assign10290_e9725)), (locals.var_t6__blk216_dn7 / (2.0 * assign10290_e9725)), (locals.var_t6__blk216_dn10 / (2.0 * assign10290_e9725)), (locals.var_t6__blk216_dn11 / (2.0 * assign10290_e9725)), (locals.var_t6__blk216_dn12 / (2.0 * assign10290_e9725)), (locals.var_t6__blk216_dn17 / (2.0 * assign10290_e9725)),)
    } else {
        (locals.var_t6__blk216, locals.var_t6__blk216_dn0, locals.var_t6__blk216_dn2, locals.var_t6__blk216_dn6, locals.var_t6__blk216_dn7, locals.var_t6__blk216_dn10, locals.var_t6__blk216_dn11, locals.var_t6__blk216_dn12, locals.var_t6__blk216_dn17,)
    }
};
        locals.var_t6__blk216 = assign10290_e9727;
        locals.var_t6__blk216_dn0 = assign10290_e9727_d_n0;
        locals.var_t6__blk216_dn2 = assign10290_e9727_d_n2;
        locals.var_t6__blk216_dn6 = assign10290_e9727_d_n6;
        locals.var_t6__blk216_dn7 = assign10290_e9727_d_n7;
        locals.var_t6__blk216_dn10 = assign10290_e9727_d_n10;
        locals.var_t6__blk216_dn11 = assign10290_e9727_d_n11;
        locals.var_t6__blk216_dn12 = assign10290_e9727_d_n12;
        locals.var_t6__blk216_dn17 = assign10290_e9727_d_n17;
        locals.var_t6__blk216_rv = 0.0;

        let (assign10300_e9748, assign10300_e9748_d_n0, assign10300_e9748_d_n2, assign10300_e9748_d_n6, assign10300_e9748_d_n7, assign10300_e9748_d_n10, assign10300_e9748_d_n11, assign10300_e9748_d_n12, assign10300_e9748_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10300_e9740: f64 = (locals.var_t1__blk211 * locals.var_uc_svgs);
        let assign10300_e9744: f64 = (1.0 - locals.var_t6__blk216);
        let assign10300_e9745: f64 = (locals.var_t3__blk213 * assign10300_e9744);
        let assign10300_e9746: f64 = (assign10300_e9740 + assign10300_e9745);
        (assign10300_e9746, ((locals.var_t1__blk211_dn0 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn0 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn0)))), ((locals.var_t1__blk211_dn2 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn2 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn2)))), ((locals.var_t1__blk211_dn6 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn6 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn6)))), ((locals.var_t1__blk211_dn7 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn7 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn7)))), ((locals.var_t1__blk211_dn10 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn10 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn10)))), ((locals.var_t1__blk211_dn11 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn11 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn11)))), ((locals.var_t1__blk211_dn12 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn12 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn12)))), ((locals.var_t1__blk211_dn17 * locals.var_uc_svgs) + ((locals.var_t3__blk213_dn17 * assign10300_e9744) + (locals.var_t3__blk213 * (-locals.var_t6__blk216_dn17)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn12, locals.var_psislsat_dn17,)
    }
};
        locals.var_psislsat = assign10300_e9748;
        locals.var_psislsat_dn0 = assign10300_e9748_d_n0;
        locals.var_psislsat_dn2 = assign10300_e9748_d_n2;
        locals.var_psislsat_dn6 = assign10300_e9748_d_n6;
        locals.var_psislsat_dn7 = assign10300_e9748_d_n7;
        locals.var_psislsat_dn10 = assign10300_e9748_d_n10;
        locals.var_psislsat_dn11 = assign10300_e9748_d_n11;
        locals.var_psislsat_dn12 = assign10300_e9748_d_n12;
        locals.var_psislsat_dn17 = assign10300_e9748_d_n17;
        locals.var_psislsat_rv = 0.0;

        let (assign10310_e9771, assign10310_e9771_d_n0, assign10310_e9771_d_n2, assign10310_e9771_d_n6, assign10310_e9771_d_n7, assign10310_e9771_d_n10, assign10310_e9771_d_n11, assign10310_e9771_d_n12, assign10310_e9771_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10310_e9761: f64 = (p.p122 * locals.var_vdsz);
        let assign10310_e9763: f64 = (assign10310_e9761 + locals.var_ps0_isub);
        let assign10310_e9766: f64 = (locals.var_xgate * locals.var_zvgs);
        let assign10310_e9768: f64 = (assign10310_e9766 * locals.var_psislsat);
        let assign10310_e9769: f64 = (assign10310_e9763 - assign10310_e9768);
        (assign10310_e9769, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0_isub_dn0) - (assign10310_e9766 * locals.var_psislsat_dn0)), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0_isub_dn2) - (assign10310_e9766 * locals.var_psislsat_dn2)), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0_isub_dn6) - (assign10310_e9766 * locals.var_psislsat_dn6)), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0_isub_dn7) - (assign10310_e9766 * locals.var_psislsat_dn7)), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0_isub_dn10) - (assign10310_e9766 * locals.var_psislsat_dn10)), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0_isub_dn11) - (assign10310_e9766 * locals.var_psislsat_dn11)), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0_isub_dn12) - (assign10310_e9766 * locals.var_psislsat_dn12)), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0_isub_dn17) - (assign10310_e9766 * locals.var_psislsat_dn17)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12, locals.var_psisubsat_dn17,)
    }
};
        locals.var_psisubsat = assign10310_e9771;
        locals.var_psisubsat_dn0 = assign10310_e9771_d_n0;
        locals.var_psisubsat_dn2 = assign10310_e9771_d_n2;
        locals.var_psisubsat_dn6 = assign10310_e9771_d_n6;
        locals.var_psisubsat_dn7 = assign10310_e9771_d_n7;
        locals.var_psisubsat_dn10 = assign10310_e9771_d_n10;
        locals.var_psisubsat_dn11 = assign10310_e9771_d_n11;
        locals.var_psisubsat_dn12 = assign10310_e9771_d_n12;
        locals.var_psisubsat_dn17 = assign10310_e9771_d_n17;
        locals.var_psisubsat_rv = 0.0;

        let (assign10320_e9793, assign10320_e9793_d_n0, assign10320_e9793_d_n2, assign10320_e9793_d_n6, assign10320_e9793_d_n7, assign10320_e9793_d_n10, assign10320_e9793_d_n11, assign10320_e9793_d_n12, assign10320_e9793_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10320_e9784: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign10320_e9787: f64 = (4.0 * 0.01);
        let assign10320_e9789: f64 = (assign10320_e9787 * 0.01);
        let assign10320_e9790: f64 = (assign10320_e9784 + assign10320_e9789);
        let assign10320_e9791: f64 = (assign10320_e9790).sqrt();
        (assign10320_e9791, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign10320_e9791)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign10320_e9791)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign10320_e9791)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign10320_e9791)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign10320_e9791)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign10320_e9791)), (((locals.var_psisubsat_dn12 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn12)) / (2.0 * assign10320_e9791)), (((locals.var_psisubsat_dn17 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn17)) / (2.0 * assign10320_e9791)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign10320_e9793;
        locals.var_tmf1_dn0 = assign10320_e9793_d_n0;
        locals.var_tmf1_dn2 = assign10320_e9793_d_n2;
        locals.var_tmf1_dn6 = assign10320_e9793_d_n6;
        locals.var_tmf1_dn7 = assign10320_e9793_d_n7;
        locals.var_tmf1_dn10 = assign10320_e9793_d_n10;
        locals.var_tmf1_dn11 = assign10320_e9793_d_n11;
        locals.var_tmf1_dn12 = assign10320_e9793_d_n12;
        locals.var_tmf1_dn17 = assign10320_e9793_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign10330_e9814, assign10330_e9814_d_n0, assign10330_e9814_d_n2, assign10330_e9814_d_n6, assign10330_e9814_d_n7, assign10330_e9814_d_n10, assign10330_e9814_d_n11, assign10330_e9814_d_n12, assign10330_e9814_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) {
        let assign10330_e9807: f64 = (locals.var_psisubsat + locals.var_tmf1);
        let assign10330_e9808: f64 = (0.5 * assign10330_e9807);
        let assign10330_e9811: f64 = (1e-10 * 0.01);
        let assign10330_e9812: f64 = (assign10330_e9808 + assign10330_e9811);
        (assign10330_e9812, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_psisubsat_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_psisubsat_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12, locals.var_psisubsat_dn17,)
    }
};
        locals.var_psisubsat = assign10330_e9814;
        locals.var_psisubsat_dn0 = assign10330_e9814_d_n0;
        locals.var_psisubsat_dn2 = assign10330_e9814_d_n2;
        locals.var_psisubsat_dn6 = assign10330_e9814_d_n6;
        locals.var_psisubsat_dn7 = assign10330_e9814_d_n7;
        locals.var_psisubsat_dn10 = assign10330_e9814_d_n10;
        locals.var_psisubsat_dn11 = assign10330_e9814_d_n11;
        locals.var_psisubsat_dn12 = assign10330_e9814_d_n12;
        locals.var_psisubsat_dn17 = assign10330_e9814_d_n17;
        locals.var_psisubsat_rv = 0.0;

        let assign10340_e9817: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign10340_e9817;
        locals.var_guard226_rv = 0.0;

        let (assign10350_e9832, assign10350_e9832_d_n0, assign10350_e9832_d_n2, assign10350_e9832_d_n6, assign10350_e9832_d_n7, assign10350_e9832_d_n10, assign10350_e9832_d_n11, assign10350_e9832_d_n12, assign10350_e9832_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 != 0.0)) && (locals.var_guard226 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12, locals.var_psisubsat_dn17,)
    }
};
        locals.var_psisubsat = assign10350_e9832;
        locals.var_psisubsat_dn0 = assign10350_e9832_d_n0;
        locals.var_psisubsat_dn2 = assign10350_e9832_d_n2;
        locals.var_psisubsat_dn6 = assign10350_e9832_d_n6;
        locals.var_psisubsat_dn7 = assign10350_e9832_d_n7;
        locals.var_psisubsat_dn10 = assign10350_e9832_d_n10;
        locals.var_psisubsat_dn11 = assign10350_e9832_d_n11;
        locals.var_psisubsat_dn12 = assign10350_e9832_d_n12;
        locals.var_psisubsat_dn17 = assign10350_e9832_d_n17;
        locals.var_psisubsat_rv = 0.0;

        let (assign10360_e9848, assign10360_e9848_d_n0, assign10360_e9848_d_n2, assign10360_e9848_d_n6, assign10360_e9848_d_n7, assign10360_e9848_d_n10, assign10360_e9848_d_n11, assign10360_e9848_d_n12, assign10360_e9848_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign10360_e9846: f64 = (locals.var_vg2const * locals.var_vgpsub);
        (assign10360_e9846, ((locals.var_vg2const_dn0 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn0)), ((locals.var_vg2const_dn2 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn2)), ((locals.var_vg2const_dn6 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn6)), ((locals.var_vg2const_dn7 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn7)), ((locals.var_vg2const_dn10 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn10)), ((locals.var_vg2const_dn11 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn11)), ((locals.var_vg2const_dn12 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn12)), ((locals.var_vg2const_dn17 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn17)),)
    } else {
        (locals.var_t1__blk211, locals.var_t1__blk211_dn0, locals.var_t1__blk211_dn2, locals.var_t1__blk211_dn6, locals.var_t1__blk211_dn7, locals.var_t1__blk211_dn10, locals.var_t1__blk211_dn11, locals.var_t1__blk211_dn12, locals.var_t1__blk211_dn17,)
    }
};
        locals.var_t1__blk211 = assign10360_e9848;
        locals.var_t1__blk211_dn0 = assign10360_e9848_d_n0;
        locals.var_t1__blk211_dn2 = assign10360_e9848_d_n2;
        locals.var_t1__blk211_dn6 = assign10360_e9848_d_n6;
        locals.var_t1__blk211_dn7 = assign10360_e9848_d_n7;
        locals.var_t1__blk211_dn10 = assign10360_e9848_d_n10;
        locals.var_t1__blk211_dn11 = assign10360_e9848_d_n11;
        locals.var_t1__blk211_dn12 = assign10360_e9848_d_n12;
        locals.var_t1__blk211_dn17 = assign10360_e9848_d_n17;
        locals.var_t1__blk211_rv = 0.0;

        let (assign10370_e9866, assign10370_e9866_d_n0, assign10370_e9866_d_n2, assign10370_e9866_d_n6, assign10370_e9866_d_n7, assign10370_e9866_d_n10, assign10370_e9866_d_n11, assign10370_e9866_d_n12, assign10370_e9866_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign10370_e9863: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign10370_e9864: f64 = (locals.var_qnsub_esi / assign10370_e9863);
        (assign10370_e9864, (((locals.var_qnsub_esi_dn0 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign10370_e9863 * assign10370_e9863)), (((locals.var_qnsub_esi_dn2 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign10370_e9863 * assign10370_e9863)), (((locals.var_qnsub_esi_dn6 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign10370_e9863 * assign10370_e9863)), (((locals.var_qnsub_esi_dn7 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign10370_e9863 * assign10370_e9863)), (((locals.var_qnsub_esi_dn10 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign10370_e9863 * assign10370_e9863)), (((locals.var_qnsub_esi_dn11 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign10370_e9863 * assign10370_e9863)), (((locals.var_qnsub_esi_dn12 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign10370_e9863 * assign10370_e9863)), (((locals.var_qnsub_esi_dn17 * assign10370_e9863) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign10370_e9863 * assign10370_e9863)),)
    } else {
        (locals.var_t3__blk213, locals.var_t3__blk213_dn0, locals.var_t3__blk213_dn2, locals.var_t3__blk213_dn6, locals.var_t3__blk213_dn7, locals.var_t3__blk213_dn10, locals.var_t3__blk213_dn11, locals.var_t3__blk213_dn12, locals.var_t3__blk213_dn17,)
    }
};
        locals.var_t3__blk213 = assign10370_e9866;
        locals.var_t3__blk213_dn0 = assign10370_e9866_d_n0;
        locals.var_t3__blk213_dn2 = assign10370_e9866_d_n2;
        locals.var_t3__blk213_dn6 = assign10370_e9866_d_n6;
        locals.var_t3__blk213_dn7 = assign10370_e9866_d_n7;
        locals.var_t3__blk213_dn10 = assign10370_e9866_d_n10;
        locals.var_t3__blk213_dn11 = assign10370_e9866_d_n11;
        locals.var_t3__blk213_dn12 = assign10370_e9866_d_n12;
        locals.var_t3__blk213_dn17 = assign10370_e9866_d_n17;
        locals.var_t3__blk213_rv = 0.0;

        let (assign10380_e9886, assign10380_e9886_d_n0, assign10380_e9886_d_n2, assign10380_e9886_d_n6, assign10380_e9886_d_n7, assign10380_e9886_d_n10, assign10380_e9886_d_n11, assign10380_e9886_d_n12, assign10380_e9886_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign10380_e9880: f64 = (2.0 / locals.var_qnsub_esi);
        let assign10380_e9883: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign10380_e9884: f64 = (assign10380_e9880 * assign10380_e9883);
        (assign10380_e9884, (((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))), (((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))), (((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))), (((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))), (((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))), (((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))), (((-((2.0 * locals.var_qnsub_esi_dn12) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))), (((-((2.0 * locals.var_qnsub_esi_dn17) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10380_e9883) + (assign10380_e9880 * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))),)
    } else {
        (locals.var_t4__blk214, locals.var_t4__blk214_dn0, locals.var_t4__blk214_dn2, locals.var_t4__blk214_dn6, locals.var_t4__blk214_dn7, locals.var_t4__blk214_dn10, locals.var_t4__blk214_dn11, locals.var_t4__blk214_dn12, locals.var_t4__blk214_dn17,)
    }
};
        locals.var_t4__blk214 = assign10380_e9886;
        locals.var_t4__blk214_dn0 = assign10380_e9886_d_n0;
        locals.var_t4__blk214_dn2 = assign10380_e9886_d_n2;
        locals.var_t4__blk214_dn6 = assign10380_e9886_d_n6;
        locals.var_t4__blk214_dn7 = assign10380_e9886_d_n7;
        locals.var_t4__blk214_dn10 = assign10380_e9886_d_n10;
        locals.var_t4__blk214_dn11 = assign10380_e9886_d_n11;
        locals.var_t4__blk214_dn12 = assign10380_e9886_d_n12;
        locals.var_t4__blk214_dn17 = assign10380_e9886_d_n17;
        locals.var_t4__blk214_rv = 0.0;

        let (assign10390_e9906, assign10390_e9906_d_n0, assign10390_e9906_d_n2, assign10390_e9906_d_n6, assign10390_e9906_d_n7, assign10390_e9906_d_n10, assign10390_e9906_d_n11, assign10390_e9906_d_n12, assign10390_e9906_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign10390_e9900: f64 = (locals.var_t1__blk211 - locals.var_beta_inv);
        let assign10390_e9903: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign10390_e9904: f64 = (assign10390_e9900 - assign10390_e9903);
        (assign10390_e9904, (locals.var_t1__blk211_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk211_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk211_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk211_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk211_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk211_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk211_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk211_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk215, locals.var_t5__blk215_dn0, locals.var_t5__blk215_dn2, locals.var_t5__blk215_dn6, locals.var_t5__blk215_dn7, locals.var_t5__blk215_dn10, locals.var_t5__blk215_dn11, locals.var_t5__blk215_dn12, locals.var_t5__blk215_dn17,)
    }
};
        locals.var_t5__blk215 = assign10390_e9906;
        locals.var_t5__blk215_dn0 = assign10390_e9906_d_n0;
        locals.var_t5__blk215_dn2 = assign10390_e9906_d_n2;
        locals.var_t5__blk215_dn6 = assign10390_e9906_d_n6;
        locals.var_t5__blk215_dn7 = assign10390_e9906_d_n7;
        locals.var_t5__blk215_dn10 = assign10390_e9906_d_n10;
        locals.var_t5__blk215_dn11 = assign10390_e9906_d_n11;
        locals.var_t5__blk215_dn12 = assign10390_e9906_d_n12;
        locals.var_t5__blk215_dn17 = assign10390_e9906_d_n17;
        locals.var_t5__blk215_rv = 0.0;

        let (assign10400_e9924, assign10400_e9924_d_n0, assign10400_e9924_d_n2, assign10400_e9924_d_n6, assign10400_e9924_d_n7, assign10400_e9924_d_n10, assign10400_e9924_d_n11, assign10400_e9924_d_n12, assign10400_e9924_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign10400_e9921: f64 = (locals.var_t4__blk214 * locals.var_t5__blk215);
        let assign10400_e9922: f64 = (1.0 + assign10400_e9921);
        (assign10400_e9922, ((locals.var_t4__blk214_dn0 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn0)), ((locals.var_t4__blk214_dn2 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn2)), ((locals.var_t4__blk214_dn6 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn6)), ((locals.var_t4__blk214_dn7 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn7)), ((locals.var_t4__blk214_dn10 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn10)), ((locals.var_t4__blk214_dn11 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn11)), ((locals.var_t4__blk214_dn12 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn12)), ((locals.var_t4__blk214_dn17 * locals.var_t5__blk215) + (locals.var_t4__blk214 * locals.var_t5__blk215_dn17)),)
    } else {
        (locals.var_t6__blk216, locals.var_t6__blk216_dn0, locals.var_t6__blk216_dn2, locals.var_t6__blk216_dn6, locals.var_t6__blk216_dn7, locals.var_t6__blk216_dn10, locals.var_t6__blk216_dn11, locals.var_t6__blk216_dn12, locals.var_t6__blk216_dn17,)
    }
};
        locals.var_t6__blk216 = assign10400_e9924;
        locals.var_t6__blk216_dn0 = assign10400_e9924_d_n0;
        locals.var_t6__blk216_dn2 = assign10400_e9924_d_n2;
        locals.var_t6__blk216_dn6 = assign10400_e9924_d_n6;
        locals.var_t6__blk216_dn7 = assign10400_e9924_d_n7;
        locals.var_t6__blk216_dn10 = assign10400_e9924_d_n10;
        locals.var_t6__blk216_dn11 = assign10400_e9924_d_n11;
        locals.var_t6__blk216_dn12 = assign10400_e9924_d_n12;
        locals.var_t6__blk216_dn17 = assign10400_e9924_d_n17;
        locals.var_t6__blk216_rv = 0.0;

        let (assign10410_e9942, assign10410_e9942_d_n0, assign10410_e9942_d_n2, assign10410_e9942_d_n6, assign10410_e9942_d_n7, assign10410_e9942_d_n10, assign10410_e9942_d_n11, assign10410_e9942_d_n12, assign10410_e9942_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) {
        let assign10410_e9939: f64 = (1.0 + locals.var_t4__blk214);
        let assign10410_e9940: f64 = (2.0 * assign10410_e9939);
        (assign10410_e9940, (2.0 * locals.var_t4__blk214_dn0), (2.0 * locals.var_t4__blk214_dn2), (2.0 * locals.var_t4__blk214_dn6), (2.0 * locals.var_t4__blk214_dn7), (2.0 * locals.var_t4__blk214_dn10), (2.0 * locals.var_t4__blk214_dn11), (2.0 * locals.var_t4__blk214_dn12), (2.0 * locals.var_t4__blk214_dn17),)
    } else {
        (locals.var_t7__blk218, locals.var_t7__blk218_dn0, locals.var_t7__blk218_dn2, locals.var_t7__blk218_dn6, locals.var_t7__blk218_dn7, locals.var_t7__blk218_dn10, locals.var_t7__blk218_dn11, locals.var_t7__blk218_dn12, locals.var_t7__blk218_dn17,)
    }
};
        locals.var_t7__blk218 = assign10410_e9942;
        locals.var_t7__blk218_dn0 = assign10410_e9942_d_n0;
        locals.var_t7__blk218_dn2 = assign10410_e9942_d_n2;
        locals.var_t7__blk218_dn6 = assign10410_e9942_d_n6;
        locals.var_t7__blk218_dn7 = assign10410_e9942_d_n7;
        locals.var_t7__blk218_dn10 = assign10410_e9942_d_n10;
        locals.var_t7__blk218_dn11 = assign10410_e9942_d_n11;
        locals.var_t7__blk218_dn12 = assign10410_e9942_d_n12;
        locals.var_t7__blk218_dn17 = assign10410_e9942_d_n17;
        locals.var_t7__blk218_rv = 0.0;

        let assign10420_e9946: f64 = (1e-50 + locals.var_t7__blk218);
        let assign10420_e9951: f64 = if ((locals.var_t6__blk216 < assign10420_e9946) && (locals.var_t7__blk218 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard227 = assign10420_e9951;
        locals.var_guard227_rv = 0.0;

        let (assign10430_e9971, assign10430_e9971_d_n0, assign10430_e9971_d_n2, assign10430_e9971_d_n6, assign10430_e9971_d_n7, assign10430_e9971_d_n10, assign10430_e9971_d_n11, assign10430_e9971_d_n12, assign10430_e9971_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) && (locals.var_guard227 != 0.0)) {
        let assign10430_e9967: f64 = (1e-50 + locals.var_t7__blk218);
        let assign10430_e9969: f64 = (assign10430_e9967 - locals.var_t6__blk216);
        (assign10430_e9969, (locals.var_t7__blk218_dn0 - locals.var_t6__blk216_dn0), (locals.var_t7__blk218_dn2 - locals.var_t6__blk216_dn2), (locals.var_t7__blk218_dn6 - locals.var_t6__blk216_dn6), (locals.var_t7__blk218_dn7 - locals.var_t6__blk216_dn7), (locals.var_t7__blk218_dn10 - locals.var_t6__blk216_dn10), (locals.var_t7__blk218_dn11 - locals.var_t6__blk216_dn11), (locals.var_t7__blk218_dn12 - locals.var_t6__blk216_dn12), (locals.var_t7__blk218_dn17 - locals.var_t6__blk216_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign10430_e9971;
        locals.var_tmf1_dn0 = assign10430_e9971_d_n0;
        locals.var_tmf1_dn2 = assign10430_e9971_d_n2;
        locals.var_tmf1_dn6 = assign10430_e9971_d_n6;
        locals.var_tmf1_dn7 = assign10430_e9971_d_n7;
        locals.var_tmf1_dn10 = assign10430_e9971_d_n10;
        locals.var_tmf1_dn11 = assign10430_e9971_d_n11;
        locals.var_tmf1_dn12 = assign10430_e9971_d_n12;
        locals.var_tmf1_dn17 = assign10430_e9971_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign10440_e9989, assign10440_e9989_d_n0, assign10440_e9989_d_n2, assign10440_e9989_d_n6, assign10440_e9989_d_n7, assign10440_e9989_d_n10, assign10440_e9989_d_n11, assign10440_e9989_d_n12, assign10440_e9989_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) && (locals.var_guard227 != 0.0)) {
        let assign10440_e9987: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign10440_e9987, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign10440_e9989;
        locals.var_x2_dn0 = assign10440_e9989_d_n0;
        locals.var_x2_dn2 = assign10440_e9989_d_n2;
        locals.var_x2_dn6 = assign10440_e9989_d_n6;
        locals.var_x2_dn7 = assign10440_e9989_d_n7;
        locals.var_x2_dn10 = assign10440_e9989_d_n10;
        locals.var_x2_dn11 = assign10440_e9989_d_n11;
        locals.var_x2_dn12 = assign10440_e9989_d_n12;
        locals.var_x2_dn17 = assign10440_e9989_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign10450_e10007, assign10450_e10007_d_n0, assign10450_e10007_d_n2, assign10450_e10007_d_n6, assign10450_e10007_d_n7, assign10450_e10007_d_n10, assign10450_e10007_d_n11, assign10450_e10007_d_n12, assign10450_e10007_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard180 != 0.0)) && (locals.var_guard181 != 0.0)) && (locals.var_guard223 == 0.0)) && (locals.var_guard224 == 0.0)) && (locals.var_guard227 != 0.0)) {
        let assign10450_e10005: f64 = (locals.var_t7__blk218 * locals.var_t7__blk218);
        (assign10450_e10005, ((locals.var_t7__blk218_dn0 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn0)), ((locals.var_t7__blk218_dn2 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn2)), ((locals.var_t7__blk218_dn6 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn6)), ((locals.var_t7__blk218_dn7 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn7)), ((locals.var_t7__blk218_dn10 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn10)), ((locals.var_t7__blk218_dn11 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn11)), ((locals.var_t7__blk218_dn12 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn12)), ((locals.var_t7__blk218_dn17 * locals.var_t7__blk218) + (locals.var_t7__blk218 * locals.var_t7__blk218_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign10450_e10007;
        locals.var_xmax2_dn0 = assign10450_e10007_d_n0;
        locals.var_xmax2_dn2 = assign10450_e10007_d_n2;
        locals.var_xmax2_dn6 = assign10450_e10007_d_n6;
        locals.var_xmax2_dn7 = assign10450_e10007_d_n7;
        locals.var_xmax2_dn10 = assign10450_e10007_d_n10;
        locals.var_xmax2_dn11 = assign10450_e10007_d_n11;
        locals.var_xmax2_dn12 = assign10450_e10007_d_n12;
        locals.var_xmax2_dn17 = assign10450_e10007_d_n17;
        locals.var_xmax2_rv = 0.0;

    }
}
