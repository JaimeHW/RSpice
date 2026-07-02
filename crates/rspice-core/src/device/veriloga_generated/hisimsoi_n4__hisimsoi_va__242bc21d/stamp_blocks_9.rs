#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6830_e4542, assign6830_e4542_d_n0, assign6830_e4542_d_n2, assign6830_e4542_d_n6, assign6830_e4542_d_n7, assign6830_e4542_d_n10, assign6830_e4542_d_n11, assign6830_e4542_d_n12, assign6830_e4542_d_n17,) = {
    if (((locals.var_guard105 == 0.0) && (locals.var_guard106 == 0.0)) && (locals.var_guard107 == 0.0)) {
        let assign6830_e4528: f64 = (1.0 / 3.0);
        let assign6830_e4533: f64 = (locals.var_t3__blk103 * 0.148148111111111);
        let assign6830_e4534: f64 = (0.0402052934513951 + assign6830_e4533);
        let assign6830_e4535: f64 = (locals.var_t3__blk103 * assign6830_e4534);
        let assign6830_e4536: f64 = (assign6830_e4528 + assign6830_e4535);
        let assign6830_e4537: f64 = (locals.var_t3__blk103 * assign6830_e4536);
        let assign6830_e4538: f64 = (1.0 + assign6830_e4537);
        let assign6830_e4539: f64 = (locals.var_t3__blk103 * assign6830_e4538);
        let assign6830_e4540: f64 = (1.0 + assign6830_e4539);
        (assign6830_e4540, ((locals.var_t3__blk103_dn0 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn0 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn0 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn0 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn2 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn2 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn2 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn2 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn6 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn6 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn6 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn6 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn7 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn7 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn7 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn7 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn10 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn10 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn10 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn10 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn11 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn11 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn11 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn11 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn12 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn12 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn12 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn12 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn17 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn17 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn17 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn17 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6830_e4542;
        locals.var_dppg_dn0 = assign6830_e4542_d_n0;
        locals.var_dppg_dn2 = assign6830_e4542_d_n2;
        locals.var_dppg_dn6 = assign6830_e4542_d_n6;
        locals.var_dppg_dn7 = assign6830_e4542_d_n7;
        locals.var_dppg_dn10 = assign6830_e4542_d_n10;
        locals.var_dppg_dn11 = assign6830_e4542_d_n11;
        locals.var_dppg_dn12 = assign6830_e4542_d_n12;
        locals.var_dppg_dn17 = assign6830_e4542_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6840_e4560, assign6840_e4560_d_n0, assign6840_e4560_d_n2, assign6840_e4560_d_n6, assign6840_e4560_d_n7, assign6840_e4560_d_n10, assign6840_e4560_d_n11, assign6840_e4560_d_n12, assign6840_e4560_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6840_e4547: f64 = (locals.var_dppg - 1.0);
        let assign6840_e4550: f64 = (locals.var_dppg - 1.0);
        let assign6840_e4551: f64 = (assign6840_e4547 * assign6840_e4550);
        let assign6840_e4554: f64 = (4.0 * 0.1);
        let assign6840_e4556: f64 = (assign6840_e4554 * 0.1);
        let assign6840_e4557: f64 = (assign6840_e4551 + assign6840_e4556);
        let assign6840_e4558: f64 = (assign6840_e4557).sqrt();
        (assign6840_e4558, (((locals.var_dppg_dn0 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn0)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn2 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn2)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn6 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn6)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn7 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn7)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn10 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn10)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn11 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn11)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn12 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn12)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn17 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn17)) / (2.0 * assign6840_e4558)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6840_e4560;
        locals.var_tmf1_dn0 = assign6840_e4560_d_n0;
        locals.var_tmf1_dn2 = assign6840_e4560_d_n2;
        locals.var_tmf1_dn6 = assign6840_e4560_d_n6;
        locals.var_tmf1_dn7 = assign6840_e4560_d_n7;
        locals.var_tmf1_dn10 = assign6840_e4560_d_n10;
        locals.var_tmf1_dn11 = assign6840_e4560_d_n11;
        locals.var_tmf1_dn12 = assign6840_e4560_d_n12;
        locals.var_tmf1_dn17 = assign6840_e4560_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6850_e4575, assign6850_e4575_d_n0, assign6850_e4575_d_n2, assign6850_e4575_d_n6, assign6850_e4575_d_n7, assign6850_e4575_d_n10, assign6850_e4575_d_n11, assign6850_e4575_d_n12, assign6850_e4575_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6850_e4566: f64 = (locals.var_dppg - 1.0);
        let assign6850_e4568: f64 = (assign6850_e4566 + locals.var_tmf1);
        let assign6850_e4569: f64 = (0.5 * assign6850_e4568);
        let assign6850_e4572: f64 = (1e-10 * 0.1);
        let assign6850_e4573: f64 = (assign6850_e4569 + assign6850_e4572);
        (assign6850_e4573, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_dppg_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_dppg_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6850_e4575;
        locals.var_dppg_dn0 = assign6850_e4575_d_n0;
        locals.var_dppg_dn2 = assign6850_e4575_d_n2;
        locals.var_dppg_dn6 = assign6850_e4575_d_n6;
        locals.var_dppg_dn7 = assign6850_e4575_d_n7;
        locals.var_dppg_dn10 = assign6850_e4575_d_n10;
        locals.var_dppg_dn11 = assign6850_e4575_d_n11;
        locals.var_dppg_dn12 = assign6850_e4575_d_n12;
        locals.var_dppg_dn17 = assign6850_e4575_d_n17;
        locals.var_dppg_rv = 0.0;

        let assign6860_e4578: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign6860_e4578;
        locals.var_guard108_rv = 0.0;

        let (assign6870_e4585, assign6870_e4585_d_n0, assign6870_e4585_d_n2, assign6870_e4585_d_n6, assign6870_e4585_d_n7, assign6870_e4585_d_n10, assign6870_e4585_d_n11, assign6870_e4585_d_n12, assign6870_e4585_d_n17,) = {
    if ((locals.var_guard105 == 0.0) && (locals.var_guard108 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6870_e4585;
        locals.var_dppg_dn0 = assign6870_e4585_d_n0;
        locals.var_dppg_dn2 = assign6870_e4585_d_n2;
        locals.var_dppg_dn6 = assign6870_e4585_d_n6;
        locals.var_dppg_dn7 = assign6870_e4585_d_n7;
        locals.var_dppg_dn10 = assign6870_e4585_d_n10;
        locals.var_dppg_dn11 = assign6870_e4585_d_n11;
        locals.var_dppg_dn12 = assign6870_e4585_d_n12;
        locals.var_dppg_dn17 = assign6870_e4585_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6880_e4592, assign6880_e4592_d_n0, assign6880_e4592_d_n2, assign6880_e4592_d_n6, assign6880_e4592_d_n7, assign6880_e4592_d_n10, assign6880_e4592_d_n11, assign6880_e4592_d_n12, assign6880_e4592_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6880_e4590: f64 = (locals.var_dppg * locals.var_t0__blk102);
        (assign6880_e4590, (locals.var_dppg_dn0 * locals.var_t0__blk102), (locals.var_dppg_dn2 * locals.var_t0__blk102), (locals.var_dppg_dn6 * locals.var_t0__blk102), (locals.var_dppg_dn7 * locals.var_t0__blk102), (locals.var_dppg_dn10 * locals.var_t0__blk102), (locals.var_dppg_dn11 * locals.var_t0__blk102), (locals.var_dppg_dn12 * locals.var_t0__blk102), (locals.var_dppg_dn17 * locals.var_t0__blk102),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6880_e4592;
        locals.var_dppg_dn0 = assign6880_e4592_d_n0;
        locals.var_dppg_dn2 = assign6880_e4592_d_n2;
        locals.var_dppg_dn6 = assign6880_e4592_d_n6;
        locals.var_dppg_dn7 = assign6880_e4592_d_n7;
        locals.var_dppg_dn10 = assign6880_e4592_d_n10;
        locals.var_dppg_dn11 = assign6880_e4592_d_n11;
        locals.var_dppg_dn12 = assign6880_e4592_d_n12;
        locals.var_dppg_dn17 = assign6880_e4592_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6890_e4601, assign6890_e4601_d_n0, assign6890_e4601_d_n2, assign6890_e4601_d_n6, assign6890_e4601_d_n7, assign6890_e4601_d_n10, assign6890_e4601_d_n11, assign6890_e4601_d_n12, assign6890_e4601_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6890_e4597: f64 = (1.0 - locals.var_dppg);
        let assign6890_e4599: f64 = (assign6890_e4597 - 0.05);
        (assign6890_e4599, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn12), (-locals.var_dppg_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6890_e4601;
        locals.var_tmf1_dn0 = assign6890_e4601_d_n0;
        locals.var_tmf1_dn2 = assign6890_e4601_d_n2;
        locals.var_tmf1_dn6 = assign6890_e4601_d_n6;
        locals.var_tmf1_dn7 = assign6890_e4601_d_n7;
        locals.var_tmf1_dn10 = assign6890_e4601_d_n10;
        locals.var_tmf1_dn11 = assign6890_e4601_d_n11;
        locals.var_tmf1_dn12 = assign6890_e4601_d_n12;
        locals.var_tmf1_dn17 = assign6890_e4601_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6900_e4610, assign6900_e4610_d_n0, assign6900_e4610_d_n2, assign6900_e4610_d_n6, assign6900_e4610_d_n7, assign6900_e4610_d_n10, assign6900_e4610_d_n11, assign6900_e4610_d_n12, assign6900_e4610_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6900_e4606: f64 = 4.0;
        let assign6900_e4608: f64 = (assign6900_e4606 * 0.05);
        (assign6900_e4608, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6900_e4610;
        locals.var_tmf2_dn0 = assign6900_e4610_d_n0;
        locals.var_tmf2_dn2 = assign6900_e4610_d_n2;
        locals.var_tmf2_dn6 = assign6900_e4610_d_n6;
        locals.var_tmf2_dn7 = assign6900_e4610_d_n7;
        locals.var_tmf2_dn10 = assign6900_e4610_d_n10;
        locals.var_tmf2_dn11 = assign6900_e4610_d_n11;
        locals.var_tmf2_dn12 = assign6900_e4610_d_n12;
        locals.var_tmf2_dn17 = assign6900_e4610_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6910_e4621, assign6910_e4621_d_n0, assign6910_e4621_d_n2, assign6910_e4621_d_n6, assign6910_e4621_d_n7, assign6910_e4621_d_n10, assign6910_e4621_d_n11, assign6910_e4621_d_n12, assign6910_e4621_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let (assign6910_e4619, assign6910_e4619_d_n0, assign6910_e4619_d_n2, assign6910_e4619_d_n6, assign6910_e4619_d_n7, assign6910_e4619_d_n10, assign6910_e4619_d_n11, assign6910_e4619_d_n12, assign6910_e4619_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6910_e4618: f64 = (-locals.var_tmf2);
                (assign6910_e4618, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6910_e4619, assign6910_e4619_d_n0, assign6910_e4619_d_n2, assign6910_e4619_d_n6, assign6910_e4619_d_n7, assign6910_e4619_d_n10, assign6910_e4619_d_n11, assign6910_e4619_d_n12, assign6910_e4619_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6910_e4621;
        locals.var_tmf2_dn0 = assign6910_e4621_d_n0;
        locals.var_tmf2_dn2 = assign6910_e4621_d_n2;
        locals.var_tmf2_dn6 = assign6910_e4621_d_n6;
        locals.var_tmf2_dn7 = assign6910_e4621_d_n7;
        locals.var_tmf2_dn10 = assign6910_e4621_d_n10;
        locals.var_tmf2_dn11 = assign6910_e4621_d_n11;
        locals.var_tmf2_dn12 = assign6910_e4621_d_n12;
        locals.var_tmf2_dn17 = assign6910_e4621_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6920_e4631, assign6920_e4631_d_n0, assign6920_e4631_d_n2, assign6920_e4631_d_n6, assign6920_e4631_d_n7, assign6920_e4631_d_n10, assign6920_e4631_d_n11, assign6920_e4631_d_n12, assign6920_e4631_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6920_e4626: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6920_e4628: f64 = (assign6920_e4626 + locals.var_tmf2);
        let assign6920_e4629: f64 = (assign6920_e4628).sqrt();
        (assign6920_e4629, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6920_e4629)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6920_e4631;
        locals.var_tmf2_dn0 = assign6920_e4631_d_n0;
        locals.var_tmf2_dn2 = assign6920_e4631_d_n2;
        locals.var_tmf2_dn6 = assign6920_e4631_d_n6;
        locals.var_tmf2_dn7 = assign6920_e4631_d_n7;
        locals.var_tmf2_dn10 = assign6920_e4631_d_n10;
        locals.var_tmf2_dn11 = assign6920_e4631_d_n11;
        locals.var_tmf2_dn12 = assign6920_e4631_d_n12;
        locals.var_tmf2_dn17 = assign6920_e4631_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6930_e4642, assign6930_e4642_d_n0, assign6930_e4642_d_n2, assign6930_e4642_d_n6, assign6930_e4642_d_n7, assign6930_e4642_d_n10, assign6930_e4642_d_n11, assign6930_e4642_d_n12, assign6930_e4642_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6930_e4638: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6930_e4639: f64 = (0.5 * assign6930_e4638);
        let assign6930_e4640: f64 = (1.0 - assign6930_e4639);
        (assign6930_e4640, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6930_e4642;
        locals.var_dppg_dn0 = assign6930_e4642_d_n0;
        locals.var_dppg_dn2 = assign6930_e4642_d_n2;
        locals.var_dppg_dn6 = assign6930_e4642_d_n6;
        locals.var_dppg_dn7 = assign6930_e4642_d_n7;
        locals.var_dppg_dn10 = assign6930_e4642_d_n10;
        locals.var_dppg_dn11 = assign6930_e4642_d_n11;
        locals.var_dppg_dn12 = assign6930_e4642_d_n12;
        locals.var_dppg_dn17 = assign6930_e4642_d_n17;
        locals.var_dppg_rv = 0.0;

        let assign6940_e4645: f64 = (locals.var_vgs - locals.var_vfb);
        let assign6940_e4647: f64 = (assign6940_e4645 + locals.var_dvth);
        let assign6940_e4649: f64 = (assign6940_e4647 - locals.var_dppg);
        locals.var_vgp = assign6940_e4649;
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

        let assign6960_e4653: f64 = (locals.var_uc_nsubs / locals.var_mks_nsubb);
        let assign6960_e4654: f64 = (assign6960_e4653).ln();
        locals.var_t1 = assign6960_e4654;
        locals.var_t1_dn0 = ((locals.var_uc_nsubs_dn0 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn2 = ((locals.var_uc_nsubs_dn2 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn6 = ((locals.var_uc_nsubs_dn6 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn7 = ((locals.var_uc_nsubs_dn7 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn10 = ((locals.var_uc_nsubs_dn10 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn11 = ((locals.var_uc_nsubs_dn11 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn12 = ((locals.var_uc_nsubs_dn12 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn17 = ((locals.var_uc_nsubs_dn17 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_rv = 0.0;

        let assign6970_e4657: f64 = (locals.var_beta_inv * locals.var_t1);
        locals.var_vbi_soi = assign6970_e4657;
        locals.var_vbi_soi_dn0 = (locals.var_beta_inv * locals.var_t1_dn0);
        locals.var_vbi_soi_dn2 = (locals.var_beta_inv * locals.var_t1_dn2);
        locals.var_vbi_soi_dn6 = (locals.var_beta_inv * locals.var_t1_dn6);
        locals.var_vbi_soi_dn7 = (locals.var_beta_inv * locals.var_t1_dn7);
        locals.var_vbi_soi_dn10 = ((locals.var_beta_inv_dn10 * locals.var_t1) + (locals.var_beta_inv * locals.var_t1_dn10));
        locals.var_vbi_soi_dn11 = (locals.var_beta_inv * locals.var_t1_dn11);
        locals.var_vbi_soi_dn12 = (locals.var_beta_inv * locals.var_t1_dn12);
        locals.var_vbi_soi_dn17 = (locals.var_beta_inv * locals.var_t1_dn17);
        locals.var_vbi_soi_rv = 0.0;

        let assign6980_e4660: f64 = (locals.var_vfb - locals.var_dvth);
        let assign6980_e4662: f64 = (assign6980_e4660 + locals.var_dppg);
        locals.var_vgs_fb = assign6980_e4662;
        locals.var_vgs_fb_dn0 = ((-locals.var_dvth_dn0) + locals.var_dppg_dn0);
        locals.var_vgs_fb_dn2 = ((-locals.var_dvth_dn2) + locals.var_dppg_dn2);
        locals.var_vgs_fb_dn6 = ((-locals.var_dvth_dn6) + locals.var_dppg_dn6);
        locals.var_vgs_fb_dn7 = ((-locals.var_dvth_dn7) + locals.var_dppg_dn7);
        locals.var_vgs_fb_dn10 = ((-locals.var_dvth_dn10) + locals.var_dppg_dn10);
        locals.var_vgs_fb_dn11 = ((-locals.var_dvth_dn11) + locals.var_dppg_dn11);
        locals.var_vgs_fb_dn12 = ((-locals.var_dvth_dn12) + locals.var_dppg_dn12);
        locals.var_vgs_fb_dn17 = ((-locals.var_dvth_dn17) + locals.var_dppg_dn17);
        locals.var_vgs_fb_rv = 0.0;

        let assign6990_e4665: f64 = (locals.var_cnst0soi * locals.var_c_fox_inv);
        locals.var_fac1 = assign6990_e4665;
        locals.var_fac1_dn0 = ((locals.var_cnst0soi_dn0 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0soi_dn2 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn2));
        locals.var_fac1_dn6 = ((locals.var_cnst0soi_dn6 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0soi_dn7 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn7));
        locals.var_fac1_dn10 = ((locals.var_cnst0soi_dn10 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0soi_dn11 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn11));
        locals.var_fac1_dn12 = ((locals.var_cnst0soi_dn12 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn12));
        locals.var_fac1_dn17 = ((locals.var_cnst0soi_dn17 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn17));
        locals.var_fac1_rv = 0.0;

        let assign7000_e4668: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign7000_e4668;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn12 = ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12));
        locals.var_fac1p2_dn17 = ((locals.var_fac1_dn17 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn17));
        locals.var_fac1p2_rv = 0.0;

        let assign7010_e4671: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign7010_e4671;
        locals.var_guard109_rv = 0.0;

        let (assign7020_e4675,) = {
    if (locals.var_guard109 != 0.0) {
        (7.0,)
    } else {
        (locals.var_qdepb_dlt,)
    }
};
        locals.var_qdepb_dlt = assign7020_e4675;
        locals.var_qdepb_dlt_rv = 0.0;

        let (assign7030_e4681, assign7030_e4681_d_n0, assign7030_e4681_d_n2, assign7030_e4681_d_n6, assign7030_e4681_d_n7, assign7030_e4681_d_n10, assign7030_e4681_d_n11, assign7030_e4681_d_n12, assign7030_e4681_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7030_e4679: f64 = (locals.var_pb2 + 1.0);
        (assign7030_e4679, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    } else {
        (locals.var_vgp_ini, locals.var_vgp_ini_dn0, locals.var_vgp_ini_dn2, locals.var_vgp_ini_dn6, locals.var_vgp_ini_dn7, locals.var_vgp_ini_dn10, locals.var_vgp_ini_dn11, locals.var_vgp_ini_dn12, locals.var_vgp_ini_dn17,)
    }
};
        locals.var_vgp_ini = assign7030_e4681;
        locals.var_vgp_ini_dn0 = assign7030_e4681_d_n0;
        locals.var_vgp_ini_dn2 = assign7030_e4681_d_n2;
        locals.var_vgp_ini_dn6 = assign7030_e4681_d_n6;
        locals.var_vgp_ini_dn7 = assign7030_e4681_d_n7;
        locals.var_vgp_ini_dn10 = assign7030_e4681_d_n10;
        locals.var_vgp_ini_dn11 = assign7030_e4681_d_n11;
        locals.var_vgp_ini_dn12 = assign7030_e4681_d_n12;
        locals.var_vgp_ini_dn17 = assign7030_e4681_d_n17;
        locals.var_vgp_ini_rv = 0.0;

        let (assign7040_e4689, assign7040_e4689_d_n0, assign7040_e4689_d_n2, assign7040_e4689_d_n6, assign7040_e4689_d_n7, assign7040_e4689_d_n10, assign7040_e4689_d_n11, assign7040_e4689_d_n12, assign7040_e4689_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7040_e4685: f64 = (1.0 / locals.var_cnst1soi);
        let assign7040_e4687: f64 = (assign7040_e4685 / locals.var_cnstc_foxi);
        (assign7040_e4687, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign7040_e4689;
        locals.var_t1_dn0 = assign7040_e4689_d_n0;
        locals.var_t1_dn2 = assign7040_e4689_d_n2;
        locals.var_t1_dn6 = assign7040_e4689_d_n6;
        locals.var_t1_dn7 = assign7040_e4689_d_n7;
        locals.var_t1_dn10 = assign7040_e4689_d_n10;
        locals.var_t1_dn11 = assign7040_e4689_d_n11;
        locals.var_t1_dn12 = assign7040_e4689_d_n12;
        locals.var_t1_dn17 = assign7040_e4689_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign7050_e4701, assign7050_e4701_d_n0, assign7050_e4701_d_n2, assign7050_e4701_d_n6, assign7050_e4701_d_n7, assign7050_e4701_d_n10, assign7050_e4701_d_n11, assign7050_e4701_d_n12, assign7050_e4701_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7050_e4694: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7050_e4695: f64 = (locals.var_t1 * assign7050_e4694);
        let assign7050_e4698: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7050_e4699: f64 = (assign7050_e4695 * assign7050_e4698);
        (assign7050_e4699, ((((locals.var_t1_dn0 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign7050_e4701;
        locals.var_t2_dn0 = assign7050_e4701_d_n0;
        locals.var_t2_dn2 = assign7050_e4701_d_n2;
        locals.var_t2_dn6 = assign7050_e4701_d_n6;
        locals.var_t2_dn7 = assign7050_e4701_d_n7;
        locals.var_t2_dn10 = assign7050_e4701_d_n10;
        locals.var_t2_dn11 = assign7050_e4701_d_n11;
        locals.var_t2_dn12 = assign7050_e4701_d_n12;
        locals.var_t2_dn17 = assign7050_e4701_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign7060_e4711, assign7060_e4711_d_n0, assign7060_e4711_d_n2, assign7060_e4711_d_n6, assign7060_e4711_d_n7, assign7060_e4711_d_n10, assign7060_e4711_d_n11, assign7060_e4711_d_n12, assign7060_e4711_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7060_e4707: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7060_e4708: f64 = (2.0 / assign7060_e4707);
        let assign7060_e4709: f64 = (locals.var_beta + assign7060_e4708);
        (assign7060_e4709, (-((2.0 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7)) / (assign7060_e4707 * assign7060_e4707))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10)) / (assign7060_e4707 * assign7060_e4707)))), (-((2.0 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17)) / (assign7060_e4707 * assign7060_e4707))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign7060_e4711;
        locals.var_t3_dn0 = assign7060_e4711_d_n0;
        locals.var_t3_dn2 = assign7060_e4711_d_n2;
        locals.var_t3_dn6 = assign7060_e4711_d_n6;
        locals.var_t3_dn7 = assign7060_e4711_d_n7;
        locals.var_t3_dn10 = assign7060_e4711_d_n10;
        locals.var_t3_dn11 = assign7060_e4711_d_n11;
        locals.var_t3_dn12 = assign7060_e4711_d_n12;
        locals.var_t3_dn17 = assign7060_e4711_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign7070_e4718, assign7070_e4718_d_n0, assign7070_e4718_d_n2, assign7070_e4718_d_n6, assign7070_e4718_d_n7, assign7070_e4718_d_n10, assign7070_e4718_d_n11, assign7070_e4718_d_n12, assign7070_e4718_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7070_e4714: f64 = (locals.var_t2).ln();
        let assign7070_e4716: f64 = (assign7070_e4714 / locals.var_t3);
        (assign7070_e4716, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inic, locals.var_ps0_inic_dn0, locals.var_ps0_inic_dn2, locals.var_ps0_inic_dn6, locals.var_ps0_inic_dn7, locals.var_ps0_inic_dn10, locals.var_ps0_inic_dn11, locals.var_ps0_inic_dn12, locals.var_ps0_inic_dn17,)
    }
};
        locals.var_ps0_inic = assign7070_e4718;
        locals.var_ps0_inic_dn0 = assign7070_e4718_d_n0;
        locals.var_ps0_inic_dn2 = assign7070_e4718_d_n2;
        locals.var_ps0_inic_dn6 = assign7070_e4718_d_n6;
        locals.var_ps0_inic_dn7 = assign7070_e4718_d_n7;
        locals.var_ps0_inic_dn10 = assign7070_e4718_d_n10;
        locals.var_ps0_inic_dn11 = assign7070_e4718_d_n11;
        locals.var_ps0_inic_dn12 = assign7070_e4718_d_n12;
        locals.var_ps0_inic_dn17 = assign7070_e4718_d_n17;
        locals.var_ps0_inic_rv = 0.0;

        let (assign7080_e4725, assign7080_e4725_d_n0, assign7080_e4725_d_n2, assign7080_e4725_d_n6, assign7080_e4725_d_n7, assign7080_e4725_d_n10, assign7080_e4725_d_n11, assign7080_e4725_d_n12, assign7080_e4725_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7080_e4722: f64 = (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic);
        let assign7080_e4723: f64 = (assign7080_e4722).sqrt();
        (assign7080_e4723, (((locals.var_cnst_2esi_q_nsubs_dn0 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn0)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn2 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn2)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn6 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn6)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn7 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn7)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn10 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn10)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn11 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn11)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn12 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn12)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn17 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn17)) / (2.0 * assign7080_e4723)),)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7080_e4725;
        locals.var_wdsoi_ini0_dn0 = assign7080_e4725_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7080_e4725_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7080_e4725_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7080_e4725_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7080_e4725_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7080_e4725_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7080_e4725_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7080_e4725_d_n17;
        locals.var_wdsoi_ini0_rv = 0.0;

        let (assign7090_e4734, assign7090_e4734_d_n0, assign7090_e4734_d_n2, assign7090_e4734_d_n6, assign7090_e4734_d_n7, assign7090_e4734_d_n10, assign7090_e4734_d_n11, assign7090_e4734_d_n12, assign7090_e4734_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let (assign7090_e4732, assign7090_e4732_d_n0, assign7090_e4732_d_n2, assign7090_e4732_d_n6, assign7090_e4732_d_n7, assign7090_e4732_d_n10, assign7090_e4732_d_n11, assign7090_e4732_d_n12, assign7090_e4732_d_n17,) = {
            if (locals.var_wdsoi_ini0 > p.p237) {
                (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
            }
        };
        (assign7090_e4732, assign7090_e4732_d_n0, assign7090_e4732_d_n2, assign7090_e4732_d_n6, assign7090_e4732_d_n7, assign7090_e4732_d_n10, assign7090_e4732_d_n11, assign7090_e4732_d_n12, assign7090_e4732_d_n17,)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7090_e4734;
        locals.var_wdsoi_ini0_dn0 = assign7090_e4734_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7090_e4734_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7090_e4734_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7090_e4734_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7090_e4734_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7090_e4734_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7090_e4734_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7090_e4734_d_n17;
        locals.var_wdsoi_ini0_rv = 0.0;

        let (assign7100_e4743, assign7100_e4743_d_n0, assign7100_e4743_d_n2, assign7100_e4743_d_n6, assign7100_e4743_d_n7, assign7100_e4743_d_n10, assign7100_e4743_d_n11, assign7100_e4743_d_n12, assign7100_e4743_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7100_e4737: f64 = (-1.6021918e-19);
        let assign7100_e4739: f64 = (assign7100_e4737 * locals.var_uc_nsubs);
        let assign7100_e4741: f64 = (assign7100_e4739 * locals.var_wdsoi_ini0);
        (assign7100_e4741, (((assign7100_e4737 * locals.var_uc_nsubs_dn0) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn0)), (((assign7100_e4737 * locals.var_uc_nsubs_dn2) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn2)), (((assign7100_e4737 * locals.var_uc_nsubs_dn6) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn6)), (((assign7100_e4737 * locals.var_uc_nsubs_dn7) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn7)), (((assign7100_e4737 * locals.var_uc_nsubs_dn10) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn10)), (((assign7100_e4737 * locals.var_uc_nsubs_dn11) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn11)), (((assign7100_e4737 * locals.var_uc_nsubs_dn12) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn12)), (((assign7100_e4737 * locals.var_uc_nsubs_dn17) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn17)),)
    } else {
        (locals.var_q_wdsoi_max, locals.var_q_wdsoi_max_dn0, locals.var_q_wdsoi_max_dn2, locals.var_q_wdsoi_max_dn6, locals.var_q_wdsoi_max_dn7, locals.var_q_wdsoi_max_dn10, locals.var_q_wdsoi_max_dn11, locals.var_q_wdsoi_max_dn12, locals.var_q_wdsoi_max_dn17,)
    }
};
        locals.var_q_wdsoi_max = assign7100_e4743;
        locals.var_q_wdsoi_max_dn0 = assign7100_e4743_d_n0;
        locals.var_q_wdsoi_max_dn2 = assign7100_e4743_d_n2;
        locals.var_q_wdsoi_max_dn6 = assign7100_e4743_d_n6;
        locals.var_q_wdsoi_max_dn7 = assign7100_e4743_d_n7;
        locals.var_q_wdsoi_max_dn10 = assign7100_e4743_d_n10;
        locals.var_q_wdsoi_max_dn11 = assign7100_e4743_d_n11;
        locals.var_q_wdsoi_max_dn12 = assign7100_e4743_d_n12;
        locals.var_q_wdsoi_max_dn17 = assign7100_e4743_d_n17;
        locals.var_q_wdsoi_max_rv = 0.0;

        let (assign7110_e4747,) = {
    if (locals.var_guard109 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_t_soi,)
    }
};
        locals.var_t_soi = assign7110_e4747;
        locals.var_t_soi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7120_e4756, assign7120_e4756_d_n0, assign7120_e4756_d_n2, assign7120_e4756_d_n6, assign7120_e4756_d_n7, assign7120_e4756_d_n10, assign7120_e4756_d_n11, assign7120_e4756_d_n12, assign7120_e4756_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7120_e4750: f64 = (-1.6021918e-19);
        let assign7120_e4752: f64 = (assign7120_e4750 * locals.var_uc_nsubs);
        let assign7120_e4754: f64 = (assign7120_e4752 * locals.var_t_soi);
        (assign7120_e4754, ((assign7120_e4750 * locals.var_uc_nsubs_dn0) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn2) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn6) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn7) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn10) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn11) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn12) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn17) * locals.var_t_soi),)
    } else {
        (locals.var_q_fd_soi, locals.var_q_fd_soi_dn0, locals.var_q_fd_soi_dn2, locals.var_q_fd_soi_dn6, locals.var_q_fd_soi_dn7, locals.var_q_fd_soi_dn10, locals.var_q_fd_soi_dn11, locals.var_q_fd_soi_dn12, locals.var_q_fd_soi_dn17,)
    }
};
        locals.var_q_fd_soi = assign7120_e4756;
        locals.var_q_fd_soi_dn0 = assign7120_e4756_d_n0;
        locals.var_q_fd_soi_dn2 = assign7120_e4756_d_n2;
        locals.var_q_fd_soi_dn6 = assign7120_e4756_d_n6;
        locals.var_q_fd_soi_dn7 = assign7120_e4756_d_n7;
        locals.var_q_fd_soi_dn10 = assign7120_e4756_d_n10;
        locals.var_q_fd_soi_dn11 = assign7120_e4756_d_n11;
        locals.var_q_fd_soi_dn12 = assign7120_e4756_d_n12;
        locals.var_q_fd_soi_dn17 = assign7120_e4756_d_n17;
        locals.var_q_fd_soi_rv = 0.0;

        let (assign7130_e4760,) = {
    if (locals.var_guard109 != 0.0) {
        (1.5,)
    } else {
        (locals.var_wdsoi_ini1_dlt,)
    }
};
        locals.var_wdsoi_ini1_dlt = assign7130_e4760;
        locals.var_wdsoi_ini1_dlt_rv = 0.0;

        let (assign7140_e4766,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7140_e4764: f64 = (1.034943e-10 / locals.var_t_soi);
        (assign7140_e4764,)
    } else {
        (locals.var_c_soi__blk110,)
    }
};
        locals.var_c_soi__blk110 = assign7140_e4766;
        locals.var_c_soi__blk110_rv = 0.0;

        let (assign7150_e4772,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7150_e4770: f64 = (1.0 / locals.var_c_soi__blk110);
        (assign7150_e4770,)
    } else {
        (locals.var_c_soi_inv__blk111,)
    }
};
        locals.var_c_soi_inv__blk111 = assign7150_e4772;
        locals.var_c_soi_inv__blk111_rv = 0.0;

        let (assign7160_e4779, assign7160_e4779_d_n0, assign7160_e4779_d_n2, assign7160_e4779_d_n6, assign7160_e4779_d_n7, assign7160_e4779_d_n10, assign7160_e4779_d_n11, assign7160_e4779_d_n12, assign7160_e4779_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7160_e4775: f64 = (-locals.var_q_fd_soi);
        let assign7160_e4777: f64 = (assign7160_e4775 * 0.001);
        (assign7160_e4777, ((-locals.var_q_fd_soi_dn0) * 0.001), ((-locals.var_q_fd_soi_dn2) * 0.001), ((-locals.var_q_fd_soi_dn6) * 0.001), ((-locals.var_q_fd_soi_dn7) * 0.001), ((-locals.var_q_fd_soi_dn10) * 0.001), ((-locals.var_q_fd_soi_dn11) * 0.001), ((-locals.var_q_fd_soi_dn12) * 0.001), ((-locals.var_q_fd_soi_dn17) * 0.001),)
    } else {
        (locals.var_q_fd_dlt1, locals.var_q_fd_dlt1_dn0, locals.var_q_fd_dlt1_dn2, locals.var_q_fd_dlt1_dn6, locals.var_q_fd_dlt1_dn7, locals.var_q_fd_dlt1_dn10, locals.var_q_fd_dlt1_dn11, locals.var_q_fd_dlt1_dn12, locals.var_q_fd_dlt1_dn17,)
    }
};
        locals.var_q_fd_dlt1 = assign7160_e4779;
        locals.var_q_fd_dlt1_dn0 = assign7160_e4779_d_n0;
        locals.var_q_fd_dlt1_dn2 = assign7160_e4779_d_n2;
        locals.var_q_fd_dlt1_dn6 = assign7160_e4779_d_n6;
        locals.var_q_fd_dlt1_dn7 = assign7160_e4779_d_n7;
        locals.var_q_fd_dlt1_dn10 = assign7160_e4779_d_n10;
        locals.var_q_fd_dlt1_dn11 = assign7160_e4779_d_n11;
        locals.var_q_fd_dlt1_dn12 = assign7160_e4779_d_n12;
        locals.var_q_fd_dlt1_dn17 = assign7160_e4779_d_n17;
        locals.var_q_fd_dlt1_rv = 0.0;

        let (assign7170_e4786, assign7170_e4786_d_n0, assign7170_e4786_d_n2, assign7170_e4786_d_n6, assign7170_e4786_d_n7, assign7170_e4786_d_n10, assign7170_e4786_d_n11, assign7170_e4786_d_n12, assign7170_e4786_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7170_e4782: f64 = (-locals.var_q_fd_soi);
        let assign7170_e4784: f64 = (assign7170_e4782 * 1e-5);
        (assign7170_e4784, ((-locals.var_q_fd_soi_dn0) * 1e-5), ((-locals.var_q_fd_soi_dn2) * 1e-5), ((-locals.var_q_fd_soi_dn6) * 1e-5), ((-locals.var_q_fd_soi_dn7) * 1e-5), ((-locals.var_q_fd_soi_dn10) * 1e-5), ((-locals.var_q_fd_soi_dn11) * 1e-5), ((-locals.var_q_fd_soi_dn12) * 1e-5), ((-locals.var_q_fd_soi_dn17) * 1e-5),)
    } else {
        (locals.var_q_fd_dlt2, locals.var_q_fd_dlt2_dn0, locals.var_q_fd_dlt2_dn2, locals.var_q_fd_dlt2_dn6, locals.var_q_fd_dlt2_dn7, locals.var_q_fd_dlt2_dn10, locals.var_q_fd_dlt2_dn11, locals.var_q_fd_dlt2_dn12, locals.var_q_fd_dlt2_dn17,)
    }
};
        locals.var_q_fd_dlt2 = assign7170_e4786;
        locals.var_q_fd_dlt2_dn0 = assign7170_e4786_d_n0;
        locals.var_q_fd_dlt2_dn2 = assign7170_e4786_d_n2;
        locals.var_q_fd_dlt2_dn6 = assign7170_e4786_d_n6;
        locals.var_q_fd_dlt2_dn7 = assign7170_e4786_d_n7;
        locals.var_q_fd_dlt2_dn10 = assign7170_e4786_d_n10;
        locals.var_q_fd_dlt2_dn11 = assign7170_e4786_d_n11;
        locals.var_q_fd_dlt2_dn12 = assign7170_e4786_d_n12;
        locals.var_q_fd_dlt2_dn17 = assign7170_e4786_d_n17;
        locals.var_q_fd_dlt2_rv = 0.0;

        let (assign7180_e4794, assign7180_e4794_d_n0, assign7180_e4794_d_n2, assign7180_e4794_d_n6, assign7180_e4794_d_n7, assign7180_e4794_d_n10, assign7180_e4794_d_n11, assign7180_e4794_d_n12, assign7180_e4794_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (p.p39 != 0.0)) {
        let assign7180_e4792: f64 = (locals.var_vbsz + locals.var_vbi_soi);
        (assign7180_e4792, (locals.var_vbsz_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbsz_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbsz_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbsz_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbsz_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbsz_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbsz_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbsz_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7180_e4794;
        locals.var_vbsbiz_dn0 = assign7180_e4794_d_n0;
        locals.var_vbsbiz_dn2 = assign7180_e4794_d_n2;
        locals.var_vbsbiz_dn6 = assign7180_e4794_d_n6;
        locals.var_vbsbiz_dn7 = assign7180_e4794_d_n7;
        locals.var_vbsbiz_dn10 = assign7180_e4794_d_n10;
        locals.var_vbsbiz_dn11 = assign7180_e4794_d_n11;
        locals.var_vbsbiz_dn12 = assign7180_e4794_d_n12;
        locals.var_vbsbiz_dn17 = assign7180_e4794_d_n17;
        locals.var_vbsbiz_rv = 0.0;

        let (assign7190_e4803, assign7190_e4803_d_n0, assign7190_e4803_d_n2, assign7190_e4803_d_n6, assign7190_e4803_d_n7, assign7190_e4803_d_n10, assign7190_e4803_d_n11, assign7190_e4803_d_n12, assign7190_e4803_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (p.p39 == 0.0)) {
        let assign7190_e4801: f64 = (locals.var_vbs + locals.var_vbi_soi);
        (assign7190_e4801, (locals.var_vbs_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbs_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbs_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbs_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbs_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbs_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbs_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbs_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7190_e4803;
        locals.var_vbsbiz_dn0 = assign7190_e4803_d_n0;
        locals.var_vbsbiz_dn2 = assign7190_e4803_d_n2;
        locals.var_vbsbiz_dn6 = assign7190_e4803_d_n6;
        locals.var_vbsbiz_dn7 = assign7190_e4803_d_n7;
        locals.var_vbsbiz_dn10 = assign7190_e4803_d_n10;
        locals.var_vbsbiz_dn11 = assign7190_e4803_d_n11;
        locals.var_vbsbiz_dn12 = assign7190_e4803_d_n12;
        locals.var_vbsbiz_dn17 = assign7190_e4803_d_n17;
        locals.var_vbsbiz_rv = 0.0;

        let (assign7200_e4814, assign7200_e4814_d_n0, assign7200_e4814_d_n2, assign7200_e4814_d_n6, assign7200_e4814_d_n7, assign7200_e4814_d_n10, assign7200_e4814_d_n11, assign7200_e4814_d_n12, assign7200_e4814_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7200_e4807: f64 = (2.0 / locals.var_beta);
        let assign7200_e4810: f64 = (locals.var_mks_nsubb / locals.var_nin);
        let assign7200_e4811: f64 = (assign7200_e4810).ln();
        let assign7200_e4812: f64 = (assign7200_e4807 * assign7200_e4811);
        (assign7200_e4812, (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign7200_e4811) + (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign7200_e4810))), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)),)
    } else {
        (locals.var_pb2_bulk, locals.var_pb2_bulk_dn0, locals.var_pb2_bulk_dn2, locals.var_pb2_bulk_dn6, locals.var_pb2_bulk_dn7, locals.var_pb2_bulk_dn10, locals.var_pb2_bulk_dn11, locals.var_pb2_bulk_dn12, locals.var_pb2_bulk_dn17,)
    }
};
        locals.var_pb2_bulk = assign7200_e4814;
        locals.var_pb2_bulk_dn0 = assign7200_e4814_d_n0;
        locals.var_pb2_bulk_dn2 = assign7200_e4814_d_n2;
        locals.var_pb2_bulk_dn6 = assign7200_e4814_d_n6;
        locals.var_pb2_bulk_dn7 = assign7200_e4814_d_n7;
        locals.var_pb2_bulk_dn10 = assign7200_e4814_d_n10;
        locals.var_pb2_bulk_dn11 = assign7200_e4814_d_n11;
        locals.var_pb2_bulk_dn12 = assign7200_e4814_d_n12;
        locals.var_pb2_bulk_dn17 = assign7200_e4814_d_n17;
        locals.var_pb2_bulk_rv = 0.0;

        let (assign7210_e4824, assign7210_e4824_d_n10,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7210_e4818: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign7210_e4820: f64 = (assign7210_e4818 * locals.var_c_box_fd_inv);
        let assign7210_e4822: f64 = (assign7210_e4820 * locals.var_c_box_fd_inv);
        (assign7210_e4822, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv),)
    } else {
        (locals.var_t0__blk117, locals.var_t0__blk117_dn10,)
    }
};
        locals.var_t0__blk117 = assign7210_e4824;
        locals.var_t0__blk117_dn10 = assign7210_e4824_d_n10;
        locals.var_t0__blk117_rv = 0.0;

        let (assign7220_e4829, assign7220_e4829_d_n0, assign7220_e4829_d_n2, assign7220_e4829_d_n6, assign7220_e4829_d_n7, assign7220_e4829_d_n10, assign7220_e4829_d_n11, assign7220_e4829_d_n12, assign7220_e4829_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7220_e4827: f64 = (-locals.var_vbsbiz);
        (assign7220_e4827, (-locals.var_vbsbiz_dn0), (-locals.var_vbsbiz_dn2), (-locals.var_vbsbiz_dn6), (-locals.var_vbsbiz_dn7), (-locals.var_vbsbiz_dn10), (-locals.var_vbsbiz_dn11), (-locals.var_vbsbiz_dn12), (-locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk118, locals.var_t1__blk118_dn0, locals.var_t1__blk118_dn2, locals.var_t1__blk118_dn6, locals.var_t1__blk118_dn7, locals.var_t1__blk118_dn10, locals.var_t1__blk118_dn11, locals.var_t1__blk118_dn12, locals.var_t1__blk118_dn17,)
    }
};
        locals.var_t1__blk118 = assign7220_e4829;
        locals.var_t1__blk118_dn0 = assign7220_e4829_d_n0;
        locals.var_t1__blk118_dn2 = assign7220_e4829_d_n2;
        locals.var_t1__blk118_dn6 = assign7220_e4829_d_n6;
        locals.var_t1__blk118_dn7 = assign7220_e4829_d_n7;
        locals.var_t1__blk118_dn10 = assign7220_e4829_d_n10;
        locals.var_t1__blk118_dn11 = assign7220_e4829_d_n11;
        locals.var_t1__blk118_dn12 = assign7220_e4829_d_n12;
        locals.var_t1__blk118_dn17 = assign7220_e4829_d_n17;
        locals.var_t1__blk118_rv = 0.0;

        let (assign7230_e4855, assign7230_e4855_d_n0, assign7230_e4855_d_n2, assign7230_e4855_d_n6, assign7230_e4855_d_n7, assign7230_e4855_d_n10, assign7230_e4855_d_n11, assign7230_e4855_d_n12, assign7230_e4855_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7230_e4833: f64 = (2.0 * locals.var_t1__blk118);
        let assign7230_e4836: f64 = (locals.var_t0__blk117 * locals.var_beta);
        let assign7230_e4837: f64 = (assign7230_e4833 + assign7230_e4836);
        let assign7230_e4840: f64 = (2.0 * locals.var_t1__blk118);
        let assign7230_e4843: f64 = (locals.var_t0__blk117 * locals.var_beta);
        let assign7230_e4844: f64 = (assign7230_e4840 + assign7230_e4843);
        let assign7230_e4845: f64 = (assign7230_e4837 * assign7230_e4844);
        let assign7230_e4849: f64 = (locals.var_t1__blk118 * locals.var_t1__blk118);
        let assign7230_e4851: f64 = (assign7230_e4849 + locals.var_t0__blk117);
        let assign7230_e4852: f64 = (4.0 * assign7230_e4851);
        let assign7230_e4853: f64 = (assign7230_e4845 - assign7230_e4852);
        (assign7230_e4853, ((((2.0 * locals.var_t1__blk118_dn0) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn0))) - (4.0 * ((locals.var_t1__blk118_dn0 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn0)))), ((((2.0 * locals.var_t1__blk118_dn2) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn2))) - (4.0 * ((locals.var_t1__blk118_dn2 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn2)))), ((((2.0 * locals.var_t1__blk118_dn6) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn6))) - (4.0 * ((locals.var_t1__blk118_dn6 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn6)))), ((((2.0 * locals.var_t1__blk118_dn7) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn7))) - (4.0 * ((locals.var_t1__blk118_dn7 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn7)))), (((((2.0 * locals.var_t1__blk118_dn10) + ((locals.var_t0__blk117_dn10 * locals.var_beta) + (locals.var_t0__blk117 * locals.var_beta_dn10))) * assign7230_e4844) + (assign7230_e4837 * ((2.0 * locals.var_t1__blk118_dn10) + ((locals.var_t0__blk117_dn10 * locals.var_beta) + (locals.var_t0__blk117 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk118_dn10 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn10)) + locals.var_t0__blk117_dn10))), ((((2.0 * locals.var_t1__blk118_dn11) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn11))) - (4.0 * ((locals.var_t1__blk118_dn11 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn11)))), ((((2.0 * locals.var_t1__blk118_dn12) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn12))) - (4.0 * ((locals.var_t1__blk118_dn12 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn12)))), ((((2.0 * locals.var_t1__blk118_dn17) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn17))) - (4.0 * ((locals.var_t1__blk118_dn17 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn17)))),)
    } else {
        (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
    }
};
        locals.var_t2__blk119 = assign7230_e4855;
        locals.var_t2__blk119_dn0 = assign7230_e4855_d_n0;
        locals.var_t2__blk119_dn2 = assign7230_e4855_d_n2;
        locals.var_t2__blk119_dn6 = assign7230_e4855_d_n6;
        locals.var_t2__blk119_dn7 = assign7230_e4855_d_n7;
        locals.var_t2__blk119_dn10 = assign7230_e4855_d_n10;
        locals.var_t2__blk119_dn11 = assign7230_e4855_d_n11;
        locals.var_t2__blk119_dn12 = assign7230_e4855_d_n12;
        locals.var_t2__blk119_dn17 = assign7230_e4855_d_n17;
        locals.var_t2__blk119_rv = 0.0;

        let (assign7240_e4868, assign7240_e4868_d_n0, assign7240_e4868_d_n2, assign7240_e4868_d_n6, assign7240_e4868_d_n7, assign7240_e4868_d_n10, assign7240_e4868_d_n11, assign7240_e4868_d_n12, assign7240_e4868_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7240_e4860: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7240_e4866, assign7240_e4866_d_n0, assign7240_e4866_d_n2, assign7240_e4866_d_n6, assign7240_e4866_d_n7, assign7240_e4866_d_n10, assign7240_e4866_d_n11, assign7240_e4866_d_n12, assign7240_e4866_d_n17,) = {
            if (locals.var_t2__blk119 >= assign7240_e4860) {
                (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
            } else {
                let assign7240_e4865: f64 = (10.0 * 2.220446049250313e-16);
                (assign7240_e4865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7240_e4866, assign7240_e4866_d_n0, assign7240_e4866_d_n2, assign7240_e4866_d_n6, assign7240_e4866_d_n7, assign7240_e4866_d_n10, assign7240_e4866_d_n11, assign7240_e4866_d_n12, assign7240_e4866_d_n17,)
    } else {
        (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
    }
};
        locals.var_t2__blk119 = assign7240_e4868;
        locals.var_t2__blk119_dn0 = assign7240_e4868_d_n0;
        locals.var_t2__blk119_dn2 = assign7240_e4868_d_n2;
        locals.var_t2__blk119_dn6 = assign7240_e4868_d_n6;
        locals.var_t2__blk119_dn7 = assign7240_e4868_d_n7;
        locals.var_t2__blk119_dn10 = assign7240_e4868_d_n10;
        locals.var_t2__blk119_dn11 = assign7240_e4868_d_n11;
        locals.var_t2__blk119_dn12 = assign7240_e4868_d_n12;
        locals.var_t2__blk119_dn17 = assign7240_e4868_d_n17;
        locals.var_t2__blk119_rv = 0.0;

        let (assign7250_e4873, assign7250_e4873_d_n0, assign7250_e4873_d_n2, assign7250_e4873_d_n6, assign7250_e4873_d_n7, assign7250_e4873_d_n10, assign7250_e4873_d_n11, assign7250_e4873_d_n12, assign7250_e4873_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7250_e4871: f64 = (locals.var_t2__blk119).sqrt();
        (assign7250_e4871, (locals.var_t2__blk119_dn0 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn2 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn6 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn7 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn10 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn11 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn12 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn17 / (2.0 * assign7250_e4871)),)
    } else {
        (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
    }
};
        locals.var_t2__blk119 = assign7250_e4873;
        locals.var_t2__blk119_dn0 = assign7250_e4873_d_n0;
        locals.var_t2__blk119_dn2 = assign7250_e4873_d_n2;
        locals.var_t2__blk119_dn6 = assign7250_e4873_d_n6;
        locals.var_t2__blk119_dn7 = assign7250_e4873_d_n7;
        locals.var_t2__blk119_dn10 = assign7250_e4873_d_n10;
        locals.var_t2__blk119_dn11 = assign7250_e4873_d_n11;
        locals.var_t2__blk119_dn12 = assign7250_e4873_d_n12;
        locals.var_t2__blk119_dn17 = assign7250_e4873_d_n17;
        locals.var_t2__blk119_rv = 0.0;

        let (assign7260_e4883, assign7260_e4883_d_n0, assign7260_e4883_d_n2, assign7260_e4883_d_n6, assign7260_e4883_d_n7, assign7260_e4883_d_n10, assign7260_e4883_d_n11, assign7260_e4883_d_n12, assign7260_e4883_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7260_e4877: f64 = (2.0 * locals.var_t1__blk118);
        let assign7260_e4880: f64 = (locals.var_t0__blk117 * locals.var_beta);
        let assign7260_e4881: f64 = (assign7260_e4877 + assign7260_e4880);
        (assign7260_e4881, (2.0 * locals.var_t1__blk118_dn0), (2.0 * locals.var_t1__blk118_dn2), (2.0 * locals.var_t1__blk118_dn6), (2.0 * locals.var_t1__blk118_dn7), ((2.0 * locals.var_t1__blk118_dn10) + ((locals.var_t0__blk117_dn10 * locals.var_beta) + (locals.var_t0__blk117 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk118_dn11), (2.0 * locals.var_t1__blk118_dn12), (2.0 * locals.var_t1__blk118_dn17),)
    } else {
        (locals.var_t3__blk120, locals.var_t3__blk120_dn0, locals.var_t3__blk120_dn2, locals.var_t3__blk120_dn6, locals.var_t3__blk120_dn7, locals.var_t3__blk120_dn10, locals.var_t3__blk120_dn11, locals.var_t3__blk120_dn12, locals.var_t3__blk120_dn17,)
    }
};
        locals.var_t3__blk120 = assign7260_e4883;
        locals.var_t3__blk120_dn0 = assign7260_e4883_d_n0;
        locals.var_t3__blk120_dn2 = assign7260_e4883_d_n2;
        locals.var_t3__blk120_dn6 = assign7260_e4883_d_n6;
        locals.var_t3__blk120_dn7 = assign7260_e4883_d_n7;
        locals.var_t3__blk120_dn10 = assign7260_e4883_d_n10;
        locals.var_t3__blk120_dn11 = assign7260_e4883_d_n11;
        locals.var_t3__blk120_dn12 = assign7260_e4883_d_n12;
        locals.var_t3__blk120_dn17 = assign7260_e4883_d_n17;
        locals.var_t3__blk120_rv = 0.0;

        let (assign7270_e4891, assign7270_e4891_d_n0, assign7270_e4891_d_n2, assign7270_e4891_d_n6, assign7270_e4891_d_n7, assign7270_e4891_d_n10, assign7270_e4891_d_n11, assign7270_e4891_d_n12, assign7270_e4891_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7270_e4887: f64 = (locals.var_t3__blk120 - locals.var_t2__blk119);
        let assign7270_e4889: f64 = (assign7270_e4887 / 2.0);
        (assign7270_e4889, ((locals.var_t3__blk120_dn0 - locals.var_t2__blk119_dn0) / 2.0), ((locals.var_t3__blk120_dn2 - locals.var_t2__blk119_dn2) / 2.0), ((locals.var_t3__blk120_dn6 - locals.var_t2__blk119_dn6) / 2.0), ((locals.var_t3__blk120_dn7 - locals.var_t2__blk119_dn7) / 2.0), ((locals.var_t3__blk120_dn10 - locals.var_t2__blk119_dn10) / 2.0), ((locals.var_t3__blk120_dn11 - locals.var_t2__blk119_dn11) / 2.0), ((locals.var_t3__blk120_dn12 - locals.var_t2__blk119_dn12) / 2.0), ((locals.var_t3__blk120_dn17 - locals.var_t2__blk119_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk121, locals.var_psb_inia__blk121_dn0, locals.var_psb_inia__blk121_dn2, locals.var_psb_inia__blk121_dn6, locals.var_psb_inia__blk121_dn7, locals.var_psb_inia__blk121_dn10, locals.var_psb_inia__blk121_dn11, locals.var_psb_inia__blk121_dn12, locals.var_psb_inia__blk121_dn17,)
    }
};
        locals.var_psb_inia__blk121 = assign7270_e4891;
        locals.var_psb_inia__blk121_dn0 = assign7270_e4891_d_n0;
        locals.var_psb_inia__blk121_dn2 = assign7270_e4891_d_n2;
        locals.var_psb_inia__blk121_dn6 = assign7270_e4891_d_n6;
        locals.var_psb_inia__blk121_dn7 = assign7270_e4891_d_n7;
        locals.var_psb_inia__blk121_dn10 = assign7270_e4891_d_n10;
        locals.var_psb_inia__blk121_dn11 = assign7270_e4891_d_n11;
        locals.var_psb_inia__blk121_dn12 = assign7270_e4891_d_n12;
        locals.var_psb_inia__blk121_dn17 = assign7270_e4891_d_n17;
        locals.var_psb_inia__blk121_rv = 0.0;

        let (assign7280_e4908, assign7280_e4908_d_n0, assign7280_e4908_d_n2, assign7280_e4908_d_n6, assign7280_e4908_d_n7, assign7280_e4908_d_n10, assign7280_e4908_d_n11, assign7280_e4908_d_n12, assign7280_e4908_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7280_e4895: f64 = (locals.var_t1__blk118 * locals.var_t1__blk118);
        let assign7280_e4897: f64 = (assign7280_e4895 / locals.var_t0__blk117);
        let assign7280_e4899: f64 = (assign7280_e4897 / locals.var_cnst1bulk);
        let assign7280_e4900: f64 = (assign7280_e4899).ln();
        let assign7280_e4904: f64 = (2.0 / locals.var_t1__blk118);
        let assign7280_e4905: f64 = (locals.var_beta + assign7280_e4904);
        let assign7280_e4906: f64 = (assign7280_e4900 / assign7280_e4905);
        (assign7280_e4906, ((((((((((locals.var_t1__blk118_dn0 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn0)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn0) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn2 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn2)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn2) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn6 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn6)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn6) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn7 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn7)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn7) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((((locals.var_t1__blk118_dn10 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn10)) * locals.var_t0__blk117) - (assign7280_e4895 * locals.var_t0__blk117_dn10)) / (locals.var_t0__blk117 * locals.var_t0__blk117)) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk118_dn10) / (locals.var_t1__blk118 * locals.var_t1__blk118)))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn11 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn11)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn11) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn12 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn12)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn12) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn17 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn17)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn17) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)),)
    } else {
        (locals.var_psb_inib__blk122, locals.var_psb_inib__blk122_dn0, locals.var_psb_inib__blk122_dn2, locals.var_psb_inib__blk122_dn6, locals.var_psb_inib__blk122_dn7, locals.var_psb_inib__blk122_dn10, locals.var_psb_inib__blk122_dn11, locals.var_psb_inib__blk122_dn12, locals.var_psb_inib__blk122_dn17,)
    }
};
        locals.var_psb_inib__blk122 = assign7280_e4908;
        locals.var_psb_inib__blk122_dn0 = assign7280_e4908_d_n0;
        locals.var_psb_inib__blk122_dn2 = assign7280_e4908_d_n2;
        locals.var_psb_inib__blk122_dn6 = assign7280_e4908_d_n6;
        locals.var_psb_inib__blk122_dn7 = assign7280_e4908_d_n7;
        locals.var_psb_inib__blk122_dn10 = assign7280_e4908_d_n10;
        locals.var_psb_inib__blk122_dn11 = assign7280_e4908_d_n11;
        locals.var_psb_inib__blk122_dn12 = assign7280_e4908_d_n12;
        locals.var_psb_inib__blk122_dn17 = assign7280_e4908_d_n17;
        locals.var_psb_inib__blk122_rv = 0.0;

        let assign7290_e4911: f64 = if locals.var_psb_inia__blk121 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard123 = assign7290_e4911;
        locals.var_guard123_rv = 0.0;

        let (assign7300_e4917, assign7300_e4917_d_n0, assign7300_e4917_d_n2, assign7300_e4917_d_n6, assign7300_e4917_d_n7, assign7300_e4917_d_n10, assign7300_e4917_d_n11, assign7300_e4917_d_n12, assign7300_e4917_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 != 0.0)) {
        (locals.var_psb_inia__blk121, locals.var_psb_inia__blk121_dn0, locals.var_psb_inia__blk121_dn2, locals.var_psb_inia__blk121_dn6, locals.var_psb_inia__blk121_dn7, locals.var_psb_inia__blk121_dn10, locals.var_psb_inia__blk121_dn11, locals.var_psb_inia__blk121_dn12, locals.var_psb_inia__blk121_dn17,)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7300_e4917;
        locals.var_phi_s0_bulk_0_dn0 = assign7300_e4917_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7300_e4917_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7300_e4917_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7300_e4917_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7300_e4917_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7300_e4917_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7300_e4917_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7300_e4917_d_n17;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign7310_e4928, assign7310_e4928_d_n0, assign7310_e4928_d_n2, assign7310_e4928_d_n6, assign7310_e4928_d_n7, assign7310_e4928_d_n10, assign7310_e4928_d_n11, assign7310_e4928_d_n12, assign7310_e4928_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7310_e4924: f64 = (locals.var_psb_inib__blk122 - locals.var_psb_inia__blk121);
        let assign7310_e4926: f64 = (assign7310_e4924 - 0.0008);
        (assign7310_e4926, (locals.var_psb_inib__blk122_dn0 - locals.var_psb_inia__blk121_dn0), (locals.var_psb_inib__blk122_dn2 - locals.var_psb_inia__blk121_dn2), (locals.var_psb_inib__blk122_dn6 - locals.var_psb_inia__blk121_dn6), (locals.var_psb_inib__blk122_dn7 - locals.var_psb_inia__blk121_dn7), (locals.var_psb_inib__blk122_dn10 - locals.var_psb_inia__blk121_dn10), (locals.var_psb_inib__blk122_dn11 - locals.var_psb_inia__blk121_dn11), (locals.var_psb_inib__blk122_dn12 - locals.var_psb_inia__blk121_dn12), (locals.var_psb_inib__blk122_dn17 - locals.var_psb_inia__blk121_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7310_e4928;
        locals.var_tmf1_dn0 = assign7310_e4928_d_n0;
        locals.var_tmf1_dn2 = assign7310_e4928_d_n2;
        locals.var_tmf1_dn6 = assign7310_e4928_d_n6;
        locals.var_tmf1_dn7 = assign7310_e4928_d_n7;
        locals.var_tmf1_dn10 = assign7310_e4928_d_n10;
        locals.var_tmf1_dn11 = assign7310_e4928_d_n11;
        locals.var_tmf1_dn12 = assign7310_e4928_d_n12;
        locals.var_tmf1_dn17 = assign7310_e4928_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7320_e4939, assign7320_e4939_d_n0, assign7320_e4939_d_n2, assign7320_e4939_d_n6, assign7320_e4939_d_n7, assign7320_e4939_d_n10, assign7320_e4939_d_n11, assign7320_e4939_d_n12, assign7320_e4939_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7320_e4935: f64 = (4.0 * locals.var_psb_inib__blk122);
        let assign7320_e4937: f64 = (assign7320_e4935 * 0.0008);
        (assign7320_e4937, ((4.0 * locals.var_psb_inib__blk122_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7320_e4939;
        locals.var_tmf2_dn0 = assign7320_e4939_d_n0;
        locals.var_tmf2_dn2 = assign7320_e4939_d_n2;
        locals.var_tmf2_dn6 = assign7320_e4939_d_n6;
        locals.var_tmf2_dn7 = assign7320_e4939_d_n7;
        locals.var_tmf2_dn10 = assign7320_e4939_d_n10;
        locals.var_tmf2_dn11 = assign7320_e4939_d_n11;
        locals.var_tmf2_dn12 = assign7320_e4939_d_n12;
        locals.var_tmf2_dn17 = assign7320_e4939_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7330_e4952, assign7330_e4952_d_n0, assign7330_e4952_d_n2, assign7330_e4952_d_n6, assign7330_e4952_d_n7, assign7330_e4952_d_n10, assign7330_e4952_d_n11, assign7330_e4952_d_n12, assign7330_e4952_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let (assign7330_e4950, assign7330_e4950_d_n0, assign7330_e4950_d_n2, assign7330_e4950_d_n6, assign7330_e4950_d_n7, assign7330_e4950_d_n10, assign7330_e4950_d_n11, assign7330_e4950_d_n12, assign7330_e4950_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign7330_e4949: f64 = (-locals.var_tmf2);
                (assign7330_e4949, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign7330_e4950, assign7330_e4950_d_n0, assign7330_e4950_d_n2, assign7330_e4950_d_n6, assign7330_e4950_d_n7, assign7330_e4950_d_n10, assign7330_e4950_d_n11, assign7330_e4950_d_n12, assign7330_e4950_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7330_e4952;
        locals.var_tmf2_dn0 = assign7330_e4952_d_n0;
        locals.var_tmf2_dn2 = assign7330_e4952_d_n2;
        locals.var_tmf2_dn6 = assign7330_e4952_d_n6;
        locals.var_tmf2_dn7 = assign7330_e4952_d_n7;
        locals.var_tmf2_dn10 = assign7330_e4952_d_n10;
        locals.var_tmf2_dn11 = assign7330_e4952_d_n11;
        locals.var_tmf2_dn12 = assign7330_e4952_d_n12;
        locals.var_tmf2_dn17 = assign7330_e4952_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7340_e4964, assign7340_e4964_d_n0, assign7340_e4964_d_n2, assign7340_e4964_d_n6, assign7340_e4964_d_n7, assign7340_e4964_d_n10, assign7340_e4964_d_n11, assign7340_e4964_d_n12, assign7340_e4964_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7340_e4959: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign7340_e4961: f64 = (assign7340_e4959 + locals.var_tmf2);
        let assign7340_e4962: f64 = (assign7340_e4961).sqrt();
        (assign7340_e4962, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign7340_e4962)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7340_e4964;
        locals.var_tmf2_dn0 = assign7340_e4964_d_n0;
        locals.var_tmf2_dn2 = assign7340_e4964_d_n2;
        locals.var_tmf2_dn6 = assign7340_e4964_d_n6;
        locals.var_tmf2_dn7 = assign7340_e4964_d_n7;
        locals.var_tmf2_dn10 = assign7340_e4964_d_n10;
        locals.var_tmf2_dn11 = assign7340_e4964_d_n11;
        locals.var_tmf2_dn12 = assign7340_e4964_d_n12;
        locals.var_tmf2_dn17 = assign7340_e4964_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7350_e4977, assign7350_e4977_d_n0, assign7350_e4977_d_n2, assign7350_e4977_d_n6, assign7350_e4977_d_n7, assign7350_e4977_d_n10, assign7350_e4977_d_n11, assign7350_e4977_d_n12, assign7350_e4977_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7350_e4973: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign7350_e4974: f64 = (0.5 * assign7350_e4973);
        let assign7350_e4975: f64 = (locals.var_psb_inib__blk122 - assign7350_e4974);
        (assign7350_e4975, (locals.var_psb_inib__blk122_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk122_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk122_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk122_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk122_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk122_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk122_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk122_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7350_e4977;
        locals.var_phi_s0_bulk_0_dn0 = assign7350_e4977_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7350_e4977_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7350_e4977_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7350_e4977_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7350_e4977_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7350_e4977_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7350_e4977_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7350_e4977_d_n17;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign7360_e4981,) = {
    if (locals.var_guard109 != 0.0) {
        (0.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign7360_e4981;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        locals: &mut StampLocals,
    ) {
        let mut assign7370_loop_guard: usize = 0;
        while {
            let assign7370_cond_e4986: f64 = if ((locals.var_guard109 != 0.0) && (locals.var_lp_s0 < locals.var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign7370_cond_e4986 != 0.0
        } {
            assign7370_loop_guard += 1;
            assert!(assign7370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7370_body0_e4990, assign7370_body0_e4990_d_n10,) = {
    if (locals.var_guard109 != 0.0) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk124, locals.var_t1__blk124_dn10,)
    }
};
            locals.var_t1__blk124 = assign7370_body0_e4990;
            locals.var_t1__blk124_dn10 = assign7370_body0_e4990_d_n10;
            locals.var_t1__blk124_rv = 0.0;
            let (assign7370_body1_e4996, assign7370_body1_e4996_d_n0, assign7370_body1_e4996_d_n2, assign7370_body1_e4996_d_n6, assign7370_body1_e4996_d_n7, assign7370_body1_e4996_d_n10, assign7370_body1_e4996_d_n11, assign7370_body1_e4996_d_n12, assign7370_body1_e4996_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body1_e4994: f64 = (locals.var_beta * locals.var_phi_s0_bulk_0);
        (assign7370_body1_e4994, (locals.var_beta * locals.var_phi_s0_bulk_0_dn0), (locals.var_beta * locals.var_phi_s0_bulk_0_dn2), (locals.var_beta * locals.var_phi_s0_bulk_0_dn6), (locals.var_beta * locals.var_phi_s0_bulk_0_dn7), ((locals.var_beta_dn10 * locals.var_phi_s0_bulk_0) + (locals.var_beta * locals.var_phi_s0_bulk_0_dn10)), (locals.var_beta * locals.var_phi_s0_bulk_0_dn11), (locals.var_beta * locals.var_phi_s0_bulk_0_dn12), (locals.var_beta * locals.var_phi_s0_bulk_0_dn17),)
    } else {
        (locals.var_t2__blk125, locals.var_t2__blk125_dn0, locals.var_t2__blk125_dn2, locals.var_t2__blk125_dn6, locals.var_t2__blk125_dn7, locals.var_t2__blk125_dn10, locals.var_t2__blk125_dn11, locals.var_t2__blk125_dn12, locals.var_t2__blk125_dn17,)
    }
};
            locals.var_t2__blk125 = assign7370_body1_e4996;
            locals.var_t2__blk125_dn0 = assign7370_body1_e4996_d_n0;
            locals.var_t2__blk125_dn2 = assign7370_body1_e4996_d_n2;
            locals.var_t2__blk125_dn6 = assign7370_body1_e4996_d_n6;
            locals.var_t2__blk125_dn7 = assign7370_body1_e4996_d_n7;
            locals.var_t2__blk125_dn10 = assign7370_body1_e4996_d_n10;
            locals.var_t2__blk125_dn11 = assign7370_body1_e4996_d_n11;
            locals.var_t2__blk125_dn12 = assign7370_body1_e4996_d_n12;
            locals.var_t2__blk125_dn17 = assign7370_body1_e4996_d_n17;
            locals.var_t2__blk125_rv = 0.0;
            let (assign7370_body2_e5002, assign7370_body2_e5002_d_n0, assign7370_body2_e5002_d_n2, assign7370_body2_e5002_d_n6, assign7370_body2_e5002_d_n7, assign7370_body2_e5002_d_n10, assign7370_body2_e5002_d_n11, assign7370_body2_e5002_d_n12, assign7370_body2_e5002_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body2_e4999: f64 = (-locals.var_t2__blk125);
        let assign7370_body2_e5000: f64 = (assign7370_body2_e4999).exp();
        (assign7370_body2_e5000, (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn0)), (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn2)), (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn6)), (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn7)), (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn10)), (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn11)), (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn12)), (assign7370_body2_e5000 * (-locals.var_t2__blk125_dn17)),)
    } else {
        (locals.var_t3__blk126, locals.var_t3__blk126_dn0, locals.var_t3__blk126_dn2, locals.var_t3__blk126_dn6, locals.var_t3__blk126_dn7, locals.var_t3__blk126_dn10, locals.var_t3__blk126_dn11, locals.var_t3__blk126_dn12, locals.var_t3__blk126_dn17,)
    }
};
            locals.var_t3__blk126 = assign7370_body2_e5002;
            locals.var_t3__blk126_dn0 = assign7370_body2_e5002_d_n0;
            locals.var_t3__blk126_dn2 = assign7370_body2_e5002_d_n2;
            locals.var_t3__blk126_dn6 = assign7370_body2_e5002_d_n6;
            locals.var_t3__blk126_dn7 = assign7370_body2_e5002_d_n7;
            locals.var_t3__blk126_dn10 = assign7370_body2_e5002_d_n10;
            locals.var_t3__blk126_dn11 = assign7370_body2_e5002_d_n11;
            locals.var_t3__blk126_dn12 = assign7370_body2_e5002_d_n12;
            locals.var_t3__blk126_dn17 = assign7370_body2_e5002_d_n17;
            locals.var_t3__blk126_rv = 0.0;
            let assign7370_body3_e5005: f64 = if locals.var_phi_s0_bulk_0 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard132 = assign7370_body3_e5005;
            locals.var_guard132_rv = 0.0;
            let (assign7370_body4_e5014, assign7370_body4_e5014_d_n0, assign7370_body4_e5014_d_n2, assign7370_body4_e5014_d_n6, assign7370_body4_e5014_d_n7, assign7370_body4_e5014_d_n10, assign7370_body4_e5014_d_n11, assign7370_body4_e5014_d_n12, assign7370_body4_e5014_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard132 != 0.0)) {
        let assign7370_body4_e5011: f64 = (locals.var_beta * locals.var_phi_s0_bulk_0);
        let assign7370_body4_e5012: f64 = (assign7370_body4_e5011).exp();
        (assign7370_body4_e5012, (assign7370_body4_e5012 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn0)), (assign7370_body4_e5012 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn2)), (assign7370_body4_e5012 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn6)), (assign7370_body4_e5012 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn7)), (assign7370_body4_e5012 * ((locals.var_beta_dn10 * locals.var_phi_s0_bulk_0) + (locals.var_beta * locals.var_phi_s0_bulk_0_dn10))), (assign7370_body4_e5012 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn11)), (assign7370_body4_e5012 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn12)), (assign7370_body4_e5012 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn17)),)
    } else {
        (locals.var_t0__blk127, locals.var_t0__blk127_dn0, locals.var_t0__blk127_dn2, locals.var_t0__blk127_dn6, locals.var_t0__blk127_dn7, locals.var_t0__blk127_dn10, locals.var_t0__blk127_dn11, locals.var_t0__blk127_dn12, locals.var_t0__blk127_dn17,)
    }
};
            locals.var_t0__blk127 = assign7370_body4_e5014;
            locals.var_t0__blk127_dn0 = assign7370_body4_e5014_d_n0;
            locals.var_t0__blk127_dn2 = assign7370_body4_e5014_d_n2;
            locals.var_t0__blk127_dn6 = assign7370_body4_e5014_d_n6;
            locals.var_t0__blk127_dn7 = assign7370_body4_e5014_d_n7;
            locals.var_t0__blk127_dn10 = assign7370_body4_e5014_d_n10;
            locals.var_t0__blk127_dn11 = assign7370_body4_e5014_d_n11;
            locals.var_t0__blk127_dn12 = assign7370_body4_e5014_d_n12;
            locals.var_t0__blk127_dn17 = assign7370_body4_e5014_d_n17;
            locals.var_t0__blk127_rv = 0.0;
            let (assign7370_body5_e5034, assign7370_body5_e5034_d_n0, assign7370_body5_e5034_d_n2, assign7370_body5_e5034_d_n6, assign7370_body5_e5034_d_n7, assign7370_body5_e5034_d_n10, assign7370_body5_e5034_d_n11, assign7370_body5_e5034_d_n12, assign7370_body5_e5034_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard132 != 0.0)) {
        let assign7370_body5_e5019: f64 = (-locals.var_t1__blk124);
        let assign7370_body5_e5022: f64 = (locals.var_t3__blk126 + locals.var_t2__blk125);
        let assign7370_body5_e5024: f64 = (assign7370_body5_e5022 - 1.0);
        let assign7370_body5_e5028: f64 = (locals.var_t0__blk127 - 1.0);
        let assign7370_body5_e5029: f64 = (locals.var_cnst1bulk * assign7370_body5_e5028);
        let assign7370_body5_e5030: f64 = (assign7370_body5_e5024 + assign7370_body5_e5029);
        let assign7370_body5_e5031: f64 = (assign7370_body5_e5030).sqrt();
        let assign7370_body5_e5032: f64 = (assign7370_body5_e5019 * assign7370_body5_e5031);
        (assign7370_body5_e5032, (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn0 + locals.var_t2__blk125_dn0) + ((locals.var_cnst1bulk_dn0 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn0))) / (2.0 * assign7370_body5_e5031))), (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn2 + locals.var_t2__blk125_dn2) + ((locals.var_cnst1bulk_dn2 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn2))) / (2.0 * assign7370_body5_e5031))), (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn6 + locals.var_t2__blk125_dn6) + ((locals.var_cnst1bulk_dn6 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn6))) / (2.0 * assign7370_body5_e5031))), (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn7 + locals.var_t2__blk125_dn7) + ((locals.var_cnst1bulk_dn7 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn7))) / (2.0 * assign7370_body5_e5031))), (((-locals.var_t1__blk124_dn10) * assign7370_body5_e5031) + (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn10 + locals.var_t2__blk125_dn10) + ((locals.var_cnst1bulk_dn10 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn10))) / (2.0 * assign7370_body5_e5031)))), (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn11 + locals.var_t2__blk125_dn11) + ((locals.var_cnst1bulk_dn11 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn11))) / (2.0 * assign7370_body5_e5031))), (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn12 + locals.var_t2__blk125_dn12) + ((locals.var_cnst1bulk_dn12 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn12))) / (2.0 * assign7370_body5_e5031))), (assign7370_body5_e5019 * (((locals.var_t3__blk126_dn17 + locals.var_t2__blk125_dn17) + ((locals.var_cnst1bulk_dn17 * assign7370_body5_e5028) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn17))) / (2.0 * assign7370_body5_e5031))),)
    } else {
        (locals.var_t4__blk128, locals.var_t4__blk128_dn0, locals.var_t4__blk128_dn2, locals.var_t4__blk128_dn6, locals.var_t4__blk128_dn7, locals.var_t4__blk128_dn10, locals.var_t4__blk128_dn11, locals.var_t4__blk128_dn12, locals.var_t4__blk128_dn17,)
    }
};
            locals.var_t4__blk128 = assign7370_body5_e5034;
            locals.var_t4__blk128_dn0 = assign7370_body5_e5034_d_n0;
            locals.var_t4__blk128_dn2 = assign7370_body5_e5034_d_n2;
            locals.var_t4__blk128_dn6 = assign7370_body5_e5034_d_n6;
            locals.var_t4__blk128_dn7 = assign7370_body5_e5034_d_n7;
            locals.var_t4__blk128_dn10 = assign7370_body5_e5034_d_n10;
            locals.var_t4__blk128_dn11 = assign7370_body5_e5034_d_n11;
            locals.var_t4__blk128_dn12 = assign7370_body5_e5034_d_n12;
            locals.var_t4__blk128_dn17 = assign7370_body5_e5034_d_n17;
            locals.var_t4__blk128_rv = 0.0;
            let (assign7370_body6_e5051, assign7370_body6_e5051_d_n0, assign7370_body6_e5051_d_n2, assign7370_body6_e5051_d_n6, assign7370_body6_e5051_d_n7, assign7370_body6_e5051_d_n10, assign7370_body6_e5051_d_n11, assign7370_body6_e5051_d_n12, assign7370_body6_e5051_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard132 != 0.0)) {
        let assign7370_body6_e5040: f64 = (locals.var_c0bulk / locals.var_t4__blk128);
        let assign7370_body6_e5042: f64 = (-locals.var_t3__blk126);
        let assign7370_body6_e5044: f64 = (assign7370_body6_e5042 + 1.0);
        let assign7370_body6_e5047: f64 = (locals.var_cnst1bulk * locals.var_t0__blk127);
        let assign7370_body6_e5048: f64 = (assign7370_body6_e5044 + assign7370_body6_e5047);
        let assign7370_body6_e5049: f64 = (assign7370_body6_e5040 * assign7370_body6_e5048);
        (assign7370_body6_e5049, (((-((locals.var_c0bulk * locals.var_t4__blk128_dn0) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn2) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn6) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn7) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn10) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn11) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn12) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn17) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body6_e5048) + (assign7370_body6_e5040 * ((-locals.var_t3__blk126_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk127) + (locals.var_cnst1bulk * locals.var_t0__blk127_dn17))))),)
    } else {
        (locals.var_t5__blk129, locals.var_t5__blk129_dn0, locals.var_t5__blk129_dn2, locals.var_t5__blk129_dn6, locals.var_t5__blk129_dn7, locals.var_t5__blk129_dn10, locals.var_t5__blk129_dn11, locals.var_t5__blk129_dn12, locals.var_t5__blk129_dn17,)
    }
};
            locals.var_t5__blk129 = assign7370_body6_e5051;
            locals.var_t5__blk129_dn0 = assign7370_body6_e5051_d_n0;
            locals.var_t5__blk129_dn2 = assign7370_body6_e5051_d_n2;
            locals.var_t5__blk129_dn6 = assign7370_body6_e5051_d_n6;
            locals.var_t5__blk129_dn7 = assign7370_body6_e5051_d_n7;
            locals.var_t5__blk129_dn10 = assign7370_body6_e5051_d_n10;
            locals.var_t5__blk129_dn11 = assign7370_body6_e5051_d_n11;
            locals.var_t5__blk129_dn12 = assign7370_body6_e5051_d_n12;
            locals.var_t5__blk129_dn17 = assign7370_body6_e5051_d_n17;
            locals.var_t5__blk129_rv = 0.0;
            let assign7370_body7_e5054: f64 = (-1e-9);
            let assign7370_body7_e5055: f64 = if locals.var_phi_s0_bulk_0 < assign7370_body7_e5054 { 1.0 } else { 0.0 };
            locals.var_guard133 = assign7370_body7_e5055;
            locals.var_guard133_rv = 0.0;
            let (assign7370_body8_e5071, assign7370_body8_e5071_d_n0, assign7370_body8_e5071_d_n2, assign7370_body8_e5071_d_n6, assign7370_body8_e5071_d_n7, assign7370_body8_e5071_d_n10, assign7370_body8_e5071_d_n11, assign7370_body8_e5071_d_n12, assign7370_body8_e5071_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard132 == 0.0)) && (locals.var_guard133 != 0.0)) {
        let assign7370_body8_e5065: f64 = (locals.var_t3__blk126 + locals.var_t2__blk125);
        let assign7370_body8_e5067: f64 = (assign7370_body8_e5065 - 1.0);
        let assign7370_body8_e5068: f64 = (assign7370_body8_e5067).sqrt();
        let assign7370_body8_e5069: f64 = (locals.var_t1__blk124 * assign7370_body8_e5068);
        (assign7370_body8_e5069, (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn0 + locals.var_t2__blk125_dn0) / (2.0 * assign7370_body8_e5068))), (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn2 + locals.var_t2__blk125_dn2) / (2.0 * assign7370_body8_e5068))), (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn6 + locals.var_t2__blk125_dn6) / (2.0 * assign7370_body8_e5068))), (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn7 + locals.var_t2__blk125_dn7) / (2.0 * assign7370_body8_e5068))), ((locals.var_t1__blk124_dn10 * assign7370_body8_e5068) + (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn10 + locals.var_t2__blk125_dn10) / (2.0 * assign7370_body8_e5068)))), (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn11 + locals.var_t2__blk125_dn11) / (2.0 * assign7370_body8_e5068))), (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn12 + locals.var_t2__blk125_dn12) / (2.0 * assign7370_body8_e5068))), (locals.var_t1__blk124 * ((locals.var_t3__blk126_dn17 + locals.var_t2__blk125_dn17) / (2.0 * assign7370_body8_e5068))),)
    } else {
        (locals.var_t4__blk128, locals.var_t4__blk128_dn0, locals.var_t4__blk128_dn2, locals.var_t4__blk128_dn6, locals.var_t4__blk128_dn7, locals.var_t4__blk128_dn10, locals.var_t4__blk128_dn11, locals.var_t4__blk128_dn12, locals.var_t4__blk128_dn17,)
    }
};
            locals.var_t4__blk128 = assign7370_body8_e5071;
            locals.var_t4__blk128_dn0 = assign7370_body8_e5071_d_n0;
            locals.var_t4__blk128_dn2 = assign7370_body8_e5071_d_n2;
            locals.var_t4__blk128_dn6 = assign7370_body8_e5071_d_n6;
            locals.var_t4__blk128_dn7 = assign7370_body8_e5071_d_n7;
            locals.var_t4__blk128_dn10 = assign7370_body8_e5071_d_n10;
            locals.var_t4__blk128_dn11 = assign7370_body8_e5071_d_n11;
            locals.var_t4__blk128_dn12 = assign7370_body8_e5071_d_n12;
            locals.var_t4__blk128_dn17 = assign7370_body8_e5071_d_n17;
            locals.var_t4__blk128_rv = 0.0;
            let (assign7370_body9_e5087, assign7370_body9_e5087_d_n0, assign7370_body9_e5087_d_n2, assign7370_body9_e5087_d_n6, assign7370_body9_e5087_d_n7, assign7370_body9_e5087_d_n10, assign7370_body9_e5087_d_n11, assign7370_body9_e5087_d_n12, assign7370_body9_e5087_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard132 == 0.0)) && (locals.var_guard133 != 0.0)) {
        let assign7370_body9_e5080: f64 = (locals.var_c0bulk / locals.var_t4__blk128);
        let assign7370_body9_e5082: f64 = (-locals.var_t3__blk126);
        let assign7370_body9_e5084: f64 = (assign7370_body9_e5082 + 1.0);
        let assign7370_body9_e5085: f64 = (assign7370_body9_e5080 * assign7370_body9_e5084);
        (assign7370_body9_e5085, (((-((locals.var_c0bulk * locals.var_t4__blk128_dn0) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn2) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn6) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn7) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn10) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn11) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn12) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk128_dn17) / (locals.var_t4__blk128 * locals.var_t4__blk128))) * assign7370_body9_e5084) + (assign7370_body9_e5080 * (-locals.var_t3__blk126_dn17))),)
    } else {
        (locals.var_t5__blk129, locals.var_t5__blk129_dn0, locals.var_t5__blk129_dn2, locals.var_t5__blk129_dn6, locals.var_t5__blk129_dn7, locals.var_t5__blk129_dn10, locals.var_t5__blk129_dn11, locals.var_t5__blk129_dn12, locals.var_t5__blk129_dn17,)
    }
};
            locals.var_t5__blk129 = assign7370_body9_e5087;
            locals.var_t5__blk129_dn0 = assign7370_body9_e5087_d_n0;
            locals.var_t5__blk129_dn2 = assign7370_body9_e5087_d_n2;
            locals.var_t5__blk129_dn6 = assign7370_body9_e5087_d_n6;
            locals.var_t5__blk129_dn7 = assign7370_body9_e5087_d_n7;
            locals.var_t5__blk129_dn10 = assign7370_body9_e5087_d_n10;
            locals.var_t5__blk129_dn11 = assign7370_body9_e5087_d_n11;
            locals.var_t5__blk129_dn12 = assign7370_body9_e5087_d_n12;
            locals.var_t5__blk129_dn17 = assign7370_body9_e5087_d_n17;
            locals.var_t5__blk129_rv = 0.0;
            let (assign7370_body10_e5105, assign7370_body10_e5105_d_n0, assign7370_body10_e5105_d_n2, assign7370_body10_e5105_d_n6, assign7370_body10_e5105_d_n7, assign7370_body10_e5105_d_n10, assign7370_body10_e5105_d_n11, assign7370_body10_e5105_d_n12, assign7370_body10_e5105_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard132 == 0.0)) && (locals.var_guard133 == 0.0)) {
        let assign7370_body10_e5097: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign7370_body10_e5098: f64 = (assign7370_body10_e5097).sqrt();
        let assign7370_body10_e5099: f64 = (-assign7370_body10_e5098);
        let assign7370_body10_e5101: f64 = (assign7370_body10_e5099 * locals.var_beta);
        let assign7370_body10_e5103: f64 = (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0);
        (assign7370_body10_e5103, (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn0), (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn2), (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn6), (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign7370_body10_e5098))) * locals.var_beta) + (assign7370_body10_e5099 * locals.var_beta_dn10)) * locals.var_phi_s0_bulk_0) + (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn10)), (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn11), (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn12), (assign7370_body10_e5101 * locals.var_phi_s0_bulk_0_dn17),)
    } else {
        (locals.var_t4__blk128, locals.var_t4__blk128_dn0, locals.var_t4__blk128_dn2, locals.var_t4__blk128_dn6, locals.var_t4__blk128_dn7, locals.var_t4__blk128_dn10, locals.var_t4__blk128_dn11, locals.var_t4__blk128_dn12, locals.var_t4__blk128_dn17,)
    }
};
            locals.var_t4__blk128 = assign7370_body10_e5105;
            locals.var_t4__blk128_dn0 = assign7370_body10_e5105_d_n0;
            locals.var_t4__blk128_dn2 = assign7370_body10_e5105_d_n2;
            locals.var_t4__blk128_dn6 = assign7370_body10_e5105_d_n6;
            locals.var_t4__blk128_dn7 = assign7370_body10_e5105_d_n7;
            locals.var_t4__blk128_dn10 = assign7370_body10_e5105_d_n10;
            locals.var_t4__blk128_dn11 = assign7370_body10_e5105_d_n11;
            locals.var_t4__blk128_dn12 = assign7370_body10_e5105_d_n12;
            locals.var_t4__blk128_dn17 = assign7370_body10_e5105_d_n17;
            locals.var_t4__blk128_rv = 0.0;
            let (assign7370_body11_e5119, assign7370_body11_e5119_d_n0, assign7370_body11_e5119_d_n2, assign7370_body11_e5119_d_n6, assign7370_body11_e5119_d_n7, assign7370_body11_e5119_d_n10, assign7370_body11_e5119_d_n11, assign7370_body11_e5119_d_n12, assign7370_body11_e5119_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard132 == 0.0)) && (locals.var_guard133 == 0.0)) {
        let assign7370_body11_e5115: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign7370_body11_e5116: f64 = (assign7370_body11_e5115).sqrt();
        let assign7370_body11_e5117: f64 = (-assign7370_body11_e5116);
        (assign7370_body11_e5117, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign7370_body11_e5116))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk129, locals.var_t5__blk129_dn0, locals.var_t5__blk129_dn2, locals.var_t5__blk129_dn6, locals.var_t5__blk129_dn7, locals.var_t5__blk129_dn10, locals.var_t5__blk129_dn11, locals.var_t5__blk129_dn12, locals.var_t5__blk129_dn17,)
    }
};
            locals.var_t5__blk129 = assign7370_body11_e5119;
            locals.var_t5__blk129_dn0 = assign7370_body11_e5119_d_n0;
            locals.var_t5__blk129_dn2 = assign7370_body11_e5119_d_n2;
            locals.var_t5__blk129_dn6 = assign7370_body11_e5119_d_n6;
            locals.var_t5__blk129_dn7 = assign7370_body11_e5119_d_n7;
            locals.var_t5__blk129_dn10 = assign7370_body11_e5119_d_n10;
            locals.var_t5__blk129_dn11 = assign7370_body11_e5119_d_n11;
            locals.var_t5__blk129_dn12 = assign7370_body11_e5119_d_n12;
            locals.var_t5__blk129_dn17 = assign7370_body11_e5119_d_n17;
            locals.var_t5__blk129_rv = 0.0;
            let (assign7370_body12_e5132, assign7370_body12_e5132_d_n0, assign7370_body12_e5132_d_n2, assign7370_body12_e5132_d_n6, assign7370_body12_e5132_d_n7, assign7370_body12_e5132_d_n10, assign7370_body12_e5132_d_n11, assign7370_body12_e5132_d_n12, assign7370_body12_e5132_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body12_e5123: f64 = (locals.var_t4__blk128 * locals.var_t4__blk128);
        let assign7370_body12_e5126: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign7370_body12_e5128: f64 = (assign7370_body12_e5126 * locals.var_q_fd_dlt1);
        let assign7370_body12_e5129: f64 = (assign7370_body12_e5123 + assign7370_body12_e5128);
        let assign7370_body12_e5130: f64 = (assign7370_body12_e5129).sqrt();
        (assign7370_body12_e5130, ((((locals.var_t4__blk128_dn0 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign7370_body12_e5130)), ((((locals.var_t4__blk128_dn2 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign7370_body12_e5130)), ((((locals.var_t4__blk128_dn6 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign7370_body12_e5130)), ((((locals.var_t4__blk128_dn7 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign7370_body12_e5130)), ((((locals.var_t4__blk128_dn10 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign7370_body12_e5130)), ((((locals.var_t4__blk128_dn11 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign7370_body12_e5130)), ((((locals.var_t4__blk128_dn12 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign7370_body12_e5130)), ((((locals.var_t4__blk128_dn17 * locals.var_t4__blk128) + (locals.var_t4__blk128 * locals.var_t4__blk128_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign7370_body12_e5126 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign7370_body12_e5130)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7370_body12_e5132;
            locals.var_tmf2_dn0 = assign7370_body12_e5132_d_n0;
            locals.var_tmf2_dn2 = assign7370_body12_e5132_d_n2;
            locals.var_tmf2_dn6 = assign7370_body12_e5132_d_n6;
            locals.var_tmf2_dn7 = assign7370_body12_e5132_d_n7;
            locals.var_tmf2_dn10 = assign7370_body12_e5132_d_n10;
            locals.var_tmf2_dn11 = assign7370_body12_e5132_d_n11;
            locals.var_tmf2_dn12 = assign7370_body12_e5132_d_n12;
            locals.var_tmf2_dn17 = assign7370_body12_e5132_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7370_body13_e5142, assign7370_body13_e5142_d_n0, assign7370_body13_e5142_d_n2, assign7370_body13_e5142_d_n6, assign7370_body13_e5142_d_n7, assign7370_body13_e5142_d_n10, assign7370_body13_e5142_d_n11, assign7370_body13_e5142_d_n12, assign7370_body13_e5142_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body13_e5138: f64 = (locals.var_t4__blk128 / locals.var_tmf2);
        let assign7370_body13_e5139: f64 = (1.0 + assign7370_body13_e5138);
        let assign7370_body13_e5140: f64 = (0.5 * assign7370_body13_e5139);
        (assign7370_body13_e5140, (0.5 * (((locals.var_t4__blk128_dn0 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk128_dn2 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk128_dn6 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk128_dn7 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk128_dn10 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk128_dn11 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk128_dn12 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk128_dn17 * locals.var_tmf2) - (locals.var_t4__blk128 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk131, locals.var_t7__blk131_dn0, locals.var_t7__blk131_dn2, locals.var_t7__blk131_dn6, locals.var_t7__blk131_dn7, locals.var_t7__blk131_dn10, locals.var_t7__blk131_dn11, locals.var_t7__blk131_dn12, locals.var_t7__blk131_dn17,)
    }
};
            locals.var_t7__blk131 = assign7370_body13_e5142;
            locals.var_t7__blk131_dn0 = assign7370_body13_e5142_d_n0;
            locals.var_t7__blk131_dn2 = assign7370_body13_e5142_d_n2;
            locals.var_t7__blk131_dn6 = assign7370_body13_e5142_d_n6;
            locals.var_t7__blk131_dn7 = assign7370_body13_e5142_d_n7;
            locals.var_t7__blk131_dn10 = assign7370_body13_e5142_d_n10;
            locals.var_t7__blk131_dn11 = assign7370_body13_e5142_d_n11;
            locals.var_t7__blk131_dn12 = assign7370_body13_e5142_d_n12;
            locals.var_t7__blk131_dn17 = assign7370_body13_e5142_d_n17;
            locals.var_t7__blk131_rv = 0.0;
            let (assign7370_body14_e5154, assign7370_body14_e5154_d_n0, assign7370_body14_e5154_d_n2, assign7370_body14_e5154_d_n6, assign7370_body14_e5154_d_n7, assign7370_body14_e5154_d_n10, assign7370_body14_e5154_d_n11, assign7370_body14_e5154_d_n12, assign7370_body14_e5154_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body14_e5147: f64 = (locals.var_t4__blk128 + locals.var_tmf2);
        let assign7370_body14_e5148: f64 = (0.5 * assign7370_body14_e5147);
        let assign7370_body14_e5151: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign7370_body14_e5152: f64 = (assign7370_body14_e5148 + assign7370_body14_e5151);
        (assign7370_body14_e5152, ((0.5 * (locals.var_t4__blk128_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk128_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk128_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk128_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk128_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk128_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk128_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk128_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk130, locals.var_t6__blk130_dn0, locals.var_t6__blk130_dn2, locals.var_t6__blk130_dn6, locals.var_t6__blk130_dn7, locals.var_t6__blk130_dn10, locals.var_t6__blk130_dn11, locals.var_t6__blk130_dn12, locals.var_t6__blk130_dn17,)
    }
};
            locals.var_t6__blk130 = assign7370_body14_e5154;
            locals.var_t6__blk130_dn0 = assign7370_body14_e5154_d_n0;
            locals.var_t6__blk130_dn2 = assign7370_body14_e5154_d_n2;
            locals.var_t6__blk130_dn6 = assign7370_body14_e5154_d_n6;
            locals.var_t6__blk130_dn7 = assign7370_body14_e5154_d_n7;
            locals.var_t6__blk130_dn10 = assign7370_body14_e5154_d_n10;
            locals.var_t6__blk130_dn11 = assign7370_body14_e5154_d_n11;
            locals.var_t6__blk130_dn12 = assign7370_body14_e5154_d_n12;
            locals.var_t6__blk130_dn17 = assign7370_body14_e5154_d_n17;
            locals.var_t6__blk130_rv = 0.0;
            let assign7370_body15_e5157: f64 = if locals.var_t6__blk130 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard134 = assign7370_body15_e5157;
            locals.var_guard134_rv = 0.0;
            let (assign7370_body16_e5163, assign7370_body16_e5163_d_n0, assign7370_body16_e5163_d_n2, assign7370_body16_e5163_d_n6, assign7370_body16_e5163_d_n7, assign7370_body16_e5163_d_n10, assign7370_body16_e5163_d_n11, assign7370_body16_e5163_d_n12, assign7370_body16_e5163_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard134 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk130, locals.var_t6__blk130_dn0, locals.var_t6__blk130_dn2, locals.var_t6__blk130_dn6, locals.var_t6__blk130_dn7, locals.var_t6__blk130_dn10, locals.var_t6__blk130_dn11, locals.var_t6__blk130_dn12, locals.var_t6__blk130_dn17,)
    }
};
            locals.var_t6__blk130 = assign7370_body16_e5163;
            locals.var_t6__blk130_dn0 = assign7370_body16_e5163_d_n0;
            locals.var_t6__blk130_dn2 = assign7370_body16_e5163_d_n2;
            locals.var_t6__blk130_dn6 = assign7370_body16_e5163_d_n6;
            locals.var_t6__blk130_dn7 = assign7370_body16_e5163_d_n7;
            locals.var_t6__blk130_dn10 = assign7370_body16_e5163_d_n10;
            locals.var_t6__blk130_dn11 = assign7370_body16_e5163_d_n11;
            locals.var_t6__blk130_dn12 = assign7370_body16_e5163_d_n12;
            locals.var_t6__blk130_dn17 = assign7370_body16_e5163_d_n17;
            locals.var_t6__blk130_rv = 0.0;
            let (assign7370_body17_e5169, assign7370_body17_e5169_d_n0, assign7370_body17_e5169_d_n2, assign7370_body17_e5169_d_n6, assign7370_body17_e5169_d_n7, assign7370_body17_e5169_d_n10, assign7370_body17_e5169_d_n11, assign7370_body17_e5169_d_n12, assign7370_body17_e5169_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard134 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk131, locals.var_t7__blk131_dn0, locals.var_t7__blk131_dn2, locals.var_t7__blk131_dn6, locals.var_t7__blk131_dn7, locals.var_t7__blk131_dn10, locals.var_t7__blk131_dn11, locals.var_t7__blk131_dn12, locals.var_t7__blk131_dn17,)
    }
};
            locals.var_t7__blk131 = assign7370_body17_e5169;
            locals.var_t7__blk131_dn0 = assign7370_body17_e5169_d_n0;
            locals.var_t7__blk131_dn2 = assign7370_body17_e5169_d_n2;
            locals.var_t7__blk131_dn6 = assign7370_body17_e5169_d_n6;
            locals.var_t7__blk131_dn7 = assign7370_body17_e5169_d_n7;
            locals.var_t7__blk131_dn10 = assign7370_body17_e5169_d_n10;
            locals.var_t7__blk131_dn11 = assign7370_body17_e5169_d_n11;
            locals.var_t7__blk131_dn12 = assign7370_body17_e5169_d_n12;
            locals.var_t7__blk131_dn17 = assign7370_body17_e5169_d_n17;
            locals.var_t7__blk131_rv = 0.0;
            let (assign7370_body18_e5178, assign7370_body18_e5178_d_n0, assign7370_body18_e5178_d_n2, assign7370_body18_e5178_d_n6, assign7370_body18_e5178_d_n7, assign7370_body18_e5178_d_n10, assign7370_body18_e5178_d_n11, assign7370_body18_e5178_d_n12, assign7370_body18_e5178_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body18_e5172: f64 = (-locals.var_q_fd_soi);
        let assign7370_body18_e5174: f64 = (assign7370_body18_e5172 - locals.var_t6__blk130);
        let assign7370_body18_e5176: f64 = (assign7370_body18_e5174 - locals.var_q_fd_dlt2);
        (assign7370_body18_e5176, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk130_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk130_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk130_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk130_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk130_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk130_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk130_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk130_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign7370_body18_e5178;
            locals.var_tmf1_dn0 = assign7370_body18_e5178_d_n0;
            locals.var_tmf1_dn2 = assign7370_body18_e5178_d_n2;
            locals.var_tmf1_dn6 = assign7370_body18_e5178_d_n6;
            locals.var_tmf1_dn7 = assign7370_body18_e5178_d_n7;
            locals.var_tmf1_dn10 = assign7370_body18_e5178_d_n10;
            locals.var_tmf1_dn11 = assign7370_body18_e5178_d_n11;
            locals.var_tmf1_dn12 = assign7370_body18_e5178_d_n12;
            locals.var_tmf1_dn17 = assign7370_body18_e5178_d_n17;
            locals.var_tmf1_rv = 0.0;
            let (assign7370_body19_e5187, assign7370_body19_e5187_d_n0, assign7370_body19_e5187_d_n2, assign7370_body19_e5187_d_n6, assign7370_body19_e5187_d_n7, assign7370_body19_e5187_d_n10, assign7370_body19_e5187_d_n11, assign7370_body19_e5187_d_n12, assign7370_body19_e5187_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body19_e5182: f64 = (-locals.var_q_fd_soi);
        let assign7370_body19_e5183: f64 = (4.0 * assign7370_body19_e5182);
        let assign7370_body19_e5185: f64 = (assign7370_body19_e5183 * locals.var_q_fd_dlt2);
        (assign7370_body19_e5185, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign7370_body19_e5183 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7370_body19_e5187;
            locals.var_tmf2_dn0 = assign7370_body19_e5187_d_n0;
            locals.var_tmf2_dn2 = assign7370_body19_e5187_d_n2;
            locals.var_tmf2_dn6 = assign7370_body19_e5187_d_n6;
            locals.var_tmf2_dn7 = assign7370_body19_e5187_d_n7;
            locals.var_tmf2_dn10 = assign7370_body19_e5187_d_n10;
            locals.var_tmf2_dn11 = assign7370_body19_e5187_d_n11;
            locals.var_tmf2_dn12 = assign7370_body19_e5187_d_n12;
            locals.var_tmf2_dn17 = assign7370_body19_e5187_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7370_body20_e5197, assign7370_body20_e5197_d_n0, assign7370_body20_e5197_d_n2, assign7370_body20_e5197_d_n6, assign7370_body20_e5197_d_n7, assign7370_body20_e5197_d_n10, assign7370_body20_e5197_d_n11, assign7370_body20_e5197_d_n12, assign7370_body20_e5197_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let (assign7370_body20_e5195, assign7370_body20_e5195_d_n0, assign7370_body20_e5195_d_n2, assign7370_body20_e5195_d_n6, assign7370_body20_e5195_d_n7, assign7370_body20_e5195_d_n10, assign7370_body20_e5195_d_n11, assign7370_body20_e5195_d_n12, assign7370_body20_e5195_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign7370_body20_e5194: f64 = (-locals.var_tmf2);
                (assign7370_body20_e5194, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign7370_body20_e5195, assign7370_body20_e5195_d_n0, assign7370_body20_e5195_d_n2, assign7370_body20_e5195_d_n6, assign7370_body20_e5195_d_n7, assign7370_body20_e5195_d_n10, assign7370_body20_e5195_d_n11, assign7370_body20_e5195_d_n12, assign7370_body20_e5195_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7370_body20_e5197;
            locals.var_tmf2_dn0 = assign7370_body20_e5197_d_n0;
            locals.var_tmf2_dn2 = assign7370_body20_e5197_d_n2;
            locals.var_tmf2_dn6 = assign7370_body20_e5197_d_n6;
            locals.var_tmf2_dn7 = assign7370_body20_e5197_d_n7;
            locals.var_tmf2_dn10 = assign7370_body20_e5197_d_n10;
            locals.var_tmf2_dn11 = assign7370_body20_e5197_d_n11;
            locals.var_tmf2_dn12 = assign7370_body20_e5197_d_n12;
            locals.var_tmf2_dn17 = assign7370_body20_e5197_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7370_body21_e5206, assign7370_body21_e5206_d_n0, assign7370_body21_e5206_d_n2, assign7370_body21_e5206_d_n6, assign7370_body21_e5206_d_n7, assign7370_body21_e5206_d_n10, assign7370_body21_e5206_d_n11, assign7370_body21_e5206_d_n12, assign7370_body21_e5206_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body21_e5201: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign7370_body21_e5203: f64 = (assign7370_body21_e5201 + locals.var_tmf2);
        let assign7370_body21_e5204: f64 = (assign7370_body21_e5203).sqrt();
        (assign7370_body21_e5204, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign7370_body21_e5204)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign7370_body21_e5204)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign7370_body21_e5204)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign7370_body21_e5204)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign7370_body21_e5204)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign7370_body21_e5204)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign7370_body21_e5204)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign7370_body21_e5204)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign7370_body21_e5206;
            locals.var_tmf2_dn0 = assign7370_body21_e5206_d_n0;
            locals.var_tmf2_dn2 = assign7370_body21_e5206_d_n2;
            locals.var_tmf2_dn6 = assign7370_body21_e5206_d_n6;
            locals.var_tmf2_dn7 = assign7370_body21_e5206_d_n7;
            locals.var_tmf2_dn10 = assign7370_body21_e5206_d_n10;
            locals.var_tmf2_dn11 = assign7370_body21_e5206_d_n11;
            locals.var_tmf2_dn12 = assign7370_body21_e5206_d_n12;
            locals.var_tmf2_dn17 = assign7370_body21_e5206_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign7370_body22_e5216, assign7370_body22_e5216_d_n0, assign7370_body22_e5216_d_n2, assign7370_body22_e5216_d_n6, assign7370_body22_e5216_d_n7, assign7370_body22_e5216_d_n10, assign7370_body22_e5216_d_n11, assign7370_body22_e5216_d_n12, assign7370_body22_e5216_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body22_e5212: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign7370_body22_e5213: f64 = (1.0 + assign7370_body22_e5212);
        let assign7370_body22_e5214: f64 = (0.5 * assign7370_body22_e5213);
        (assign7370_body22_e5214, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign7370_body22_e5216;
            locals.var_t8_dn0 = assign7370_body22_e5216_d_n0;
            locals.var_t8_dn2 = assign7370_body22_e5216_d_n2;
            locals.var_t8_dn6 = assign7370_body22_e5216_d_n6;
            locals.var_t8_dn7 = assign7370_body22_e5216_d_n7;
            locals.var_t8_dn10 = assign7370_body22_e5216_d_n10;
            locals.var_t8_dn11 = assign7370_body22_e5216_d_n11;
            locals.var_t8_dn12 = assign7370_body22_e5216_d_n12;
            locals.var_t8_dn17 = assign7370_body22_e5216_d_n17;
            locals.var_t8_rv = 0.0;
            let (assign7370_body23_e5227, assign7370_body23_e5227_d_n0, assign7370_body23_e5227_d_n2, assign7370_body23_e5227_d_n6, assign7370_body23_e5227_d_n7, assign7370_body23_e5227_d_n10, assign7370_body23_e5227_d_n11, assign7370_body23_e5227_d_n12, assign7370_body23_e5227_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body23_e5219: f64 = (-locals.var_q_fd_soi);
        let assign7370_body23_e5223: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign7370_body23_e5224: f64 = (0.5 * assign7370_body23_e5223);
        let assign7370_body23_e5225: f64 = (assign7370_body23_e5219 - assign7370_body23_e5224);
        (assign7370_body23_e5225, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk130, locals.var_t6__blk130_dn0, locals.var_t6__blk130_dn2, locals.var_t6__blk130_dn6, locals.var_t6__blk130_dn7, locals.var_t6__blk130_dn10, locals.var_t6__blk130_dn11, locals.var_t6__blk130_dn12, locals.var_t6__blk130_dn17,)
    }
};
            locals.var_t6__blk130 = assign7370_body23_e5227;
            locals.var_t6__blk130_dn0 = assign7370_body23_e5227_d_n0;
            locals.var_t6__blk130_dn2 = assign7370_body23_e5227_d_n2;
            locals.var_t6__blk130_dn6 = assign7370_body23_e5227_d_n6;
            locals.var_t6__blk130_dn7 = assign7370_body23_e5227_d_n7;
            locals.var_t6__blk130_dn10 = assign7370_body23_e5227_d_n10;
            locals.var_t6__blk130_dn11 = assign7370_body23_e5227_d_n11;
            locals.var_t6__blk130_dn12 = assign7370_body23_e5227_d_n12;
            locals.var_t6__blk130_dn17 = assign7370_body23_e5227_d_n17;
            locals.var_t6__blk130_rv = 0.0;
            let (assign7370_body24_e5235, assign7370_body24_e5235_d_n0, assign7370_body24_e5235_d_n2, assign7370_body24_e5235_d_n6, assign7370_body24_e5235_d_n7, assign7370_body24_e5235_d_n10, assign7370_body24_e5235_d_n11, assign7370_body24_e5235_d_n12, assign7370_body24_e5235_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body24_e5232: f64 = (locals.var_t5__blk129 * locals.var_t8);
        let assign7370_body24_e5233: f64 = (locals.var_t7__blk131 * assign7370_body24_e5232);
        (assign7370_body24_e5233, ((locals.var_t7__blk131_dn0 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn0 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn0)))), ((locals.var_t7__blk131_dn2 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn2 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn2)))), ((locals.var_t7__blk131_dn6 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn6 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn6)))), ((locals.var_t7__blk131_dn7 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn7 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn7)))), ((locals.var_t7__blk131_dn10 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn10 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn10)))), ((locals.var_t7__blk131_dn11 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn11 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn11)))), ((locals.var_t7__blk131_dn12 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn12 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn12)))), ((locals.var_t7__blk131_dn17 * assign7370_body24_e5232) + (locals.var_t7__blk131 * ((locals.var_t5__blk129_dn17 * locals.var_t8) + (locals.var_t5__blk129 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk131, locals.var_t7__blk131_dn0, locals.var_t7__blk131_dn2, locals.var_t7__blk131_dn6, locals.var_t7__blk131_dn7, locals.var_t7__blk131_dn10, locals.var_t7__blk131_dn11, locals.var_t7__blk131_dn12, locals.var_t7__blk131_dn17,)
    }
};
            locals.var_t7__blk131 = assign7370_body24_e5235;
            locals.var_t7__blk131_dn0 = assign7370_body24_e5235_d_n0;
            locals.var_t7__blk131_dn2 = assign7370_body24_e5235_d_n2;
            locals.var_t7__blk131_dn6 = assign7370_body24_e5235_d_n6;
            locals.var_t7__blk131_dn7 = assign7370_body24_e5235_d_n7;
            locals.var_t7__blk131_dn10 = assign7370_body24_e5235_d_n10;
            locals.var_t7__blk131_dn11 = assign7370_body24_e5235_d_n11;
            locals.var_t7__blk131_dn12 = assign7370_body24_e5235_d_n12;
            locals.var_t7__blk131_dn17 = assign7370_body24_e5235_d_n17;
            locals.var_t7__blk131_rv = 0.0;
            let (assign7370_body25_e5249, assign7370_body25_e5249_d_n0, assign7370_body25_e5249_d_n2, assign7370_body25_e5249_d_n6, assign7370_body25_e5249_d_n7, assign7370_body25_e5249_d_n10, assign7370_body25_e5249_d_n11, assign7370_body25_e5249_d_n12, assign7370_body25_e5249_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body25_e5239: f64 = (locals.var_t6__blk130 * locals.var_t6__blk130);
        let assign7370_body25_e5241: f64 = (assign7370_body25_e5239 / 2.0);
        let assign7370_body25_e5243: f64 = (assign7370_body25_e5241 / 1.034943e-10);
        let assign7370_body25_e5245: f64 = (assign7370_body25_e5243 / 1.6021918e-19);
        let assign7370_body25_e5247: f64 = (assign7370_body25_e5245 / locals.var_uc_nsubs);
        (assign7370_body25_e5247, ((((((((locals.var_t6__blk130_dn0 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk130_dn2 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk130_dn6 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk130_dn7 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk130_dn10 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk130_dn11 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk130_dn12 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk130_dn17 * locals.var_t6__blk130) + (locals.var_t6__blk130 * locals.var_t6__blk130_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign7370_body25_e5245 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep0, locals.var_phi_b_dep0_dn0, locals.var_phi_b_dep0_dn2, locals.var_phi_b_dep0_dn6, locals.var_phi_b_dep0_dn7, locals.var_phi_b_dep0_dn10, locals.var_phi_b_dep0_dn11, locals.var_phi_b_dep0_dn12, locals.var_phi_b_dep0_dn17,)
    }
};
            locals.var_phi_b_dep0 = assign7370_body25_e5249;
            locals.var_phi_b_dep0_dn0 = assign7370_body25_e5249_d_n0;
            locals.var_phi_b_dep0_dn2 = assign7370_body25_e5249_d_n2;
            locals.var_phi_b_dep0_dn6 = assign7370_body25_e5249_d_n6;
            locals.var_phi_b_dep0_dn7 = assign7370_body25_e5249_d_n7;
            locals.var_phi_b_dep0_dn10 = assign7370_body25_e5249_d_n10;
            locals.var_phi_b_dep0_dn11 = assign7370_body25_e5249_d_n11;
            locals.var_phi_b_dep0_dn12 = assign7370_body25_e5249_d_n12;
            locals.var_phi_b_dep0_dn17 = assign7370_body25_e5249_d_n17;
            locals.var_phi_b_dep0_rv = 0.0;
            let (assign7370_body26_e5259, assign7370_body26_e5259_d_n0, assign7370_body26_e5259_d_n2, assign7370_body26_e5259_d_n6, assign7370_body26_e5259_d_n7, assign7370_body26_e5259_d_n10, assign7370_body26_e5259_d_n11, assign7370_body26_e5259_d_n12, assign7370_body26_e5259_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body26_e5253: f64 = (2.0 * locals.var_phi_b_dep0);
        let assign7370_body26_e5255: f64 = (assign7370_body26_e5253 * locals.var_t7__blk131);
        let assign7370_body26_e5257: f64 = (assign7370_body26_e5255 / locals.var_t6__blk130);
        (assign7370_body26_e5257, ((((((2.0 * locals.var_phi_b_dep0_dn0) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn0)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn0)) / (locals.var_t6__blk130 * locals.var_t6__blk130)), ((((((2.0 * locals.var_phi_b_dep0_dn2) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn2)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn2)) / (locals.var_t6__blk130 * locals.var_t6__blk130)), ((((((2.0 * locals.var_phi_b_dep0_dn6) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn6)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn6)) / (locals.var_t6__blk130 * locals.var_t6__blk130)), ((((((2.0 * locals.var_phi_b_dep0_dn7) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn7)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn7)) / (locals.var_t6__blk130 * locals.var_t6__blk130)), ((((((2.0 * locals.var_phi_b_dep0_dn10) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn10)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn10)) / (locals.var_t6__blk130 * locals.var_t6__blk130)), ((((((2.0 * locals.var_phi_b_dep0_dn11) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn11)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn11)) / (locals.var_t6__blk130 * locals.var_t6__blk130)), ((((((2.0 * locals.var_phi_b_dep0_dn12) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn12)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn12)) / (locals.var_t6__blk130 * locals.var_t6__blk130)), ((((((2.0 * locals.var_phi_b_dep0_dn17) * locals.var_t7__blk131) + (assign7370_body26_e5253 * locals.var_t7__blk131_dn17)) * locals.var_t6__blk130) - (assign7370_body26_e5255 * locals.var_t6__blk130_dn17)) / (locals.var_t6__blk130 * locals.var_t6__blk130)),)
    } else {
        (locals.var_phi_b_dep0_dpsb, locals.var_phi_b_dep0_dpsb_dn0, locals.var_phi_b_dep0_dpsb_dn2, locals.var_phi_b_dep0_dpsb_dn6, locals.var_phi_b_dep0_dpsb_dn7, locals.var_phi_b_dep0_dpsb_dn10, locals.var_phi_b_dep0_dpsb_dn11, locals.var_phi_b_dep0_dpsb_dn12, locals.var_phi_b_dep0_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep0_dpsb = assign7370_body26_e5259;
            locals.var_phi_b_dep0_dpsb_dn0 = assign7370_body26_e5259_d_n0;
            locals.var_phi_b_dep0_dpsb_dn2 = assign7370_body26_e5259_d_n2;
            locals.var_phi_b_dep0_dpsb_dn6 = assign7370_body26_e5259_d_n6;
            locals.var_phi_b_dep0_dpsb_dn7 = assign7370_body26_e5259_d_n7;
            locals.var_phi_b_dep0_dpsb_dn10 = assign7370_body26_e5259_d_n10;
            locals.var_phi_b_dep0_dpsb_dn11 = assign7370_body26_e5259_d_n11;
            locals.var_phi_b_dep0_dpsb_dn12 = assign7370_body26_e5259_d_n12;
            locals.var_phi_b_dep0_dpsb_dn17 = assign7370_body26_e5259_d_n17;
            locals.var_phi_b_dep0_dpsb_rv = 0.0;
            let (assign7370_body27_e5283, assign7370_body27_e5283_d_n0, assign7370_body27_e5283_d_n2, assign7370_body27_e5283_d_n6, assign7370_body27_e5283_d_n7, assign7370_body27_e5283_d_n10, assign7370_body27_e5283_d_n11, assign7370_body27_e5283_d_n12, assign7370_body27_e5283_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body27_e5263: f64 = (-locals.var_phi_s0_bulk_0);
        let assign7370_body27_e5266: f64 = (locals.var_t4__blk128 / locals.var_c_box);
        let assign7370_body27_e5267: f64 = (assign7370_body27_e5263 + assign7370_body27_e5266);
        let assign7370_body27_e5269: f64 = (assign7370_body27_e5267 - locals.var_vbsbiz);
        let assign7370_body27_e5271: f64 = (assign7370_body27_e5269 + locals.var_phi_b_dep0);
        let assign7370_body27_e5273: f64 = (-1.0);
        let assign7370_body27_e5276: f64 = (locals.var_t5__blk129 / locals.var_c_box);
        let assign7370_body27_e5277: f64 = (assign7370_body27_e5273 + assign7370_body27_e5276);
        let assign7370_body27_e5279: f64 = (assign7370_body27_e5277 + locals.var_phi_b_dep0_dpsb);
        let assign7370_body27_e5280: f64 = (assign7370_body27_e5271 / assign7370_body27_e5279);
        let assign7370_body27_e5281: f64 = (locals.var_phi_s0_bulk_0 - assign7370_body27_e5280);
        (assign7370_body27_e5281, (locals.var_phi_s0_bulk_0_dn0 - (((((((-locals.var_phi_s0_bulk_0_dn0) + (locals.var_t4__blk128_dn0 / locals.var_c_box)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep0_dn0) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn0 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn0))) / (assign7370_body27_e5279 * assign7370_body27_e5279))), (locals.var_phi_s0_bulk_0_dn2 - (((((((-locals.var_phi_s0_bulk_0_dn2) + (locals.var_t4__blk128_dn2 / locals.var_c_box)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep0_dn2) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn2 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn2))) / (assign7370_body27_e5279 * assign7370_body27_e5279))), (locals.var_phi_s0_bulk_0_dn6 - (((((((-locals.var_phi_s0_bulk_0_dn6) + (locals.var_t4__blk128_dn6 / locals.var_c_box)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep0_dn6) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn6 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn6))) / (assign7370_body27_e5279 * assign7370_body27_e5279))), (locals.var_phi_s0_bulk_0_dn7 - (((((((-locals.var_phi_s0_bulk_0_dn7) + (locals.var_t4__blk128_dn7 / locals.var_c_box)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep0_dn7) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn7 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn7))) / (assign7370_body27_e5279 * assign7370_body27_e5279))), (locals.var_phi_s0_bulk_0_dn10 - (((((((-locals.var_phi_s0_bulk_0_dn10) + (locals.var_t4__blk128_dn10 / locals.var_c_box)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep0_dn10) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn10 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn10))) / (assign7370_body27_e5279 * assign7370_body27_e5279))), (locals.var_phi_s0_bulk_0_dn11 - (((((((-locals.var_phi_s0_bulk_0_dn11) + (locals.var_t4__blk128_dn11 / locals.var_c_box)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep0_dn11) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn11 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn11))) / (assign7370_body27_e5279 * assign7370_body27_e5279))), (locals.var_phi_s0_bulk_0_dn12 - (((((((-locals.var_phi_s0_bulk_0_dn12) + (locals.var_t4__blk128_dn12 / locals.var_c_box)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep0_dn12) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn12 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn12))) / (assign7370_body27_e5279 * assign7370_body27_e5279))), (locals.var_phi_s0_bulk_0_dn17 - (((((((-locals.var_phi_s0_bulk_0_dn17) + (locals.var_t4__blk128_dn17 / locals.var_c_box)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep0_dn17) * assign7370_body27_e5279) - (assign7370_body27_e5271 * ((locals.var_t5__blk129_dn17 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn17))) / (assign7370_body27_e5279 * assign7370_body27_e5279))),)
    } else {
        (locals.var_t6__blk130, locals.var_t6__blk130_dn0, locals.var_t6__blk130_dn2, locals.var_t6__blk130_dn6, locals.var_t6__blk130_dn7, locals.var_t6__blk130_dn10, locals.var_t6__blk130_dn11, locals.var_t6__blk130_dn12, locals.var_t6__blk130_dn17,)
    }
};
            locals.var_t6__blk130 = assign7370_body27_e5283;
            locals.var_t6__blk130_dn0 = assign7370_body27_e5283_d_n0;
            locals.var_t6__blk130_dn2 = assign7370_body27_e5283_d_n2;
            locals.var_t6__blk130_dn6 = assign7370_body27_e5283_d_n6;
            locals.var_t6__blk130_dn7 = assign7370_body27_e5283_d_n7;
            locals.var_t6__blk130_dn10 = assign7370_body27_e5283_d_n10;
            locals.var_t6__blk130_dn11 = assign7370_body27_e5283_d_n11;
            locals.var_t6__blk130_dn12 = assign7370_body27_e5283_d_n12;
            locals.var_t6__blk130_dn17 = assign7370_body27_e5283_d_n17;
            locals.var_t6__blk130_rv = 0.0;
            let assign7370_body28_e5286: f64 = (locals.var_t6__blk130 - locals.var_phi_s0_bulk_0);
            let assign7370_body28_e5287: f64 = (assign7370_body28_e5286).abs();
            let assign7370_body28_e5289: f64 = if assign7370_body28_e5287 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard135 = assign7370_body28_e5289;
            locals.var_guard135_rv = 0.0;
            let (assign7370_body29_e5295,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard135 != 0.0)) {
        (locals.var_lp_s0_max,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign7370_body29_e5295;
            locals.var_lp_s0_rv = 0.0;
            let (assign7370_body30_e5299, assign7370_body30_e5299_d_n0, assign7370_body30_e5299_d_n2, assign7370_body30_e5299_d_n6, assign7370_body30_e5299_d_n7, assign7370_body30_e5299_d_n10, assign7370_body30_e5299_d_n11, assign7370_body30_e5299_d_n12, assign7370_body30_e5299_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        (locals.var_t6__blk130, locals.var_t6__blk130_dn0, locals.var_t6__blk130_dn2, locals.var_t6__blk130_dn6, locals.var_t6__blk130_dn7, locals.var_t6__blk130_dn10, locals.var_t6__blk130_dn11, locals.var_t6__blk130_dn12, locals.var_t6__blk130_dn17,)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
            locals.var_phi_s0_bulk_0 = assign7370_body30_e5299;
            locals.var_phi_s0_bulk_0_dn0 = assign7370_body30_e5299_d_n0;
            locals.var_phi_s0_bulk_0_dn2 = assign7370_body30_e5299_d_n2;
            locals.var_phi_s0_bulk_0_dn6 = assign7370_body30_e5299_d_n6;
            locals.var_phi_s0_bulk_0_dn7 = assign7370_body30_e5299_d_n7;
            locals.var_phi_s0_bulk_0_dn10 = assign7370_body30_e5299_d_n10;
            locals.var_phi_s0_bulk_0_dn11 = assign7370_body30_e5299_d_n11;
            locals.var_phi_s0_bulk_0_dn12 = assign7370_body30_e5299_d_n12;
            locals.var_phi_s0_bulk_0_dn17 = assign7370_body30_e5299_d_n17;
            locals.var_phi_s0_bulk_0_rv = 0.0;
            let (assign7370_body31_e5303, assign7370_body31_e5303_d_n0, assign7370_body31_e5303_d_n2, assign7370_body31_e5303_d_n6, assign7370_body31_e5303_d_n7, assign7370_body31_e5303_d_n10, assign7370_body31_e5303_d_n11, assign7370_body31_e5303_d_n12, assign7370_body31_e5303_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        (locals.var_t4__blk128, locals.var_t4__blk128_dn0, locals.var_t4__blk128_dn2, locals.var_t4__blk128_dn6, locals.var_t4__blk128_dn7, locals.var_t4__blk128_dn10, locals.var_t4__blk128_dn11, locals.var_t4__blk128_dn12, locals.var_t4__blk128_dn17,)
    } else {
        (locals.var_q_s0_bulk_0, locals.var_q_s0_bulk_0_dn0, locals.var_q_s0_bulk_0_dn2, locals.var_q_s0_bulk_0_dn6, locals.var_q_s0_bulk_0_dn7, locals.var_q_s0_bulk_0_dn10, locals.var_q_s0_bulk_0_dn11, locals.var_q_s0_bulk_0_dn12, locals.var_q_s0_bulk_0_dn17,)
    }
};
            locals.var_q_s0_bulk_0 = assign7370_body31_e5303;
            locals.var_q_s0_bulk_0_dn0 = assign7370_body31_e5303_d_n0;
            locals.var_q_s0_bulk_0_dn2 = assign7370_body31_e5303_d_n2;
            locals.var_q_s0_bulk_0_dn6 = assign7370_body31_e5303_d_n6;
            locals.var_q_s0_bulk_0_dn7 = assign7370_body31_e5303_d_n7;
            locals.var_q_s0_bulk_0_dn10 = assign7370_body31_e5303_d_n10;
            locals.var_q_s0_bulk_0_dn11 = assign7370_body31_e5303_d_n11;
            locals.var_q_s0_bulk_0_dn12 = assign7370_body31_e5303_d_n12;
            locals.var_q_s0_bulk_0_dn17 = assign7370_body31_e5303_d_n17;
            locals.var_q_s0_bulk_0_rv = 0.0;
            let (assign7370_body32_e5309,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7370_body32_e5307: f64 = (locals.var_lp_s0 + 1.0);
        (assign7370_body32_e5307,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign7370_body32_e5309;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_18(
        locals: &mut StampLocals,
    ) {
        let (assign7380_e5313, assign7380_e5313_d_n0, assign7380_e5313_d_n2, assign7380_e5313_d_n6, assign7380_e5313_d_n7, assign7380_e5313_d_n10, assign7380_e5313_d_n11, assign7380_e5313_d_n12, assign7380_e5313_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        (locals.var_phi_b_dep0, locals.var_phi_b_dep0_dn0, locals.var_phi_b_dep0_dn2, locals.var_phi_b_dep0_dn6, locals.var_phi_b_dep0_dn7, locals.var_phi_b_dep0_dn10, locals.var_phi_b_dep0_dn11, locals.var_phi_b_dep0_dn12, locals.var_phi_b_dep0_dn17,)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
        locals.var_phi_b_dep = assign7380_e5313;
        locals.var_phi_b_dep_dn0 = assign7380_e5313_d_n0;
        locals.var_phi_b_dep_dn2 = assign7380_e5313_d_n2;
        locals.var_phi_b_dep_dn6 = assign7380_e5313_d_n6;
        locals.var_phi_b_dep_dn7 = assign7380_e5313_d_n7;
        locals.var_phi_b_dep_dn10 = assign7380_e5313_d_n10;
        locals.var_phi_b_dep_dn11 = assign7380_e5313_d_n11;
        locals.var_phi_b_dep_dn12 = assign7380_e5313_d_n12;
        locals.var_phi_b_dep_dn17 = assign7380_e5313_d_n17;
        locals.var_phi_b_dep_rv = 0.0;

        let (assign7390_e5326, assign7390_e5326_d_n0, assign7390_e5326_d_n2, assign7390_e5326_d_n6, assign7390_e5326_d_n7, assign7390_e5326_d_n10, assign7390_e5326_d_n11, assign7390_e5326_d_n12, assign7390_e5326_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7390_e5317: f64 = (2.0 * 1.034943e-10);
        let assign7390_e5319: f64 = (assign7390_e5317 / 1.6021918e-19);
        let assign7390_e5321: f64 = (assign7390_e5319 * locals.var_phi_b_dep);
        let assign7390_e5323: f64 = (assign7390_e5321 / locals.var_uc_nsubs);
        let assign7390_e5324: f64 = (assign7390_e5323).sqrt();
        (assign7390_e5324, (((((assign7390_e5319 * locals.var_phi_b_dep_dn0) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)), (((((assign7390_e5319 * locals.var_phi_b_dep_dn2) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)), (((((assign7390_e5319 * locals.var_phi_b_dep_dn6) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)), (((((assign7390_e5319 * locals.var_phi_b_dep_dn7) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)), (((((assign7390_e5319 * locals.var_phi_b_dep_dn10) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)), (((((assign7390_e5319 * locals.var_phi_b_dep_dn11) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)), (((((assign7390_e5319 * locals.var_phi_b_dep_dn12) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)), (((((assign7390_e5319 * locals.var_phi_b_dep_dn17) * locals.var_uc_nsubs) - (assign7390_e5321 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign7390_e5324)),)
    } else {
        (locals.var_t1__blk137, locals.var_t1__blk137_dn0, locals.var_t1__blk137_dn2, locals.var_t1__blk137_dn6, locals.var_t1__blk137_dn7, locals.var_t1__blk137_dn10, locals.var_t1__blk137_dn11, locals.var_t1__blk137_dn12, locals.var_t1__blk137_dn17,)
    }
};
        locals.var_t1__blk137 = assign7390_e5326;
        locals.var_t1__blk137_dn0 = assign7390_e5326_d_n0;
        locals.var_t1__blk137_dn2 = assign7390_e5326_d_n2;
        locals.var_t1__blk137_dn6 = assign7390_e5326_d_n6;
        locals.var_t1__blk137_dn7 = assign7390_e5326_d_n7;
        locals.var_t1__blk137_dn10 = assign7390_e5326_d_n10;
        locals.var_t1__blk137_dn11 = assign7390_e5326_d_n11;
        locals.var_t1__blk137_dn12 = assign7390_e5326_d_n12;
        locals.var_t1__blk137_dn17 = assign7390_e5326_d_n17;
        locals.var_t1__blk137_rv = 0.0;

        let assign7400_e5330: f64 = (0.99 * locals.var_t_soi);
        let assign7400_e5331: f64 = if locals.var_t1__blk137 > assign7400_e5330 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7400_e5331;
        locals.var_guard142_rv = 0.0;

        let (assign7410_e5339, assign7410_e5339_d_n0, assign7410_e5339_d_n2, assign7410_e5339_d_n6, assign7410_e5339_d_n7, assign7410_e5339_d_n10, assign7410_e5339_d_n11, assign7410_e5339_d_n12, assign7410_e5339_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7410_e5337: f64 = (1.0 / locals.var_c_fox);
        (assign7410_e5337, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0__blk136, locals.var_t0__blk136_dn0, locals.var_t0__blk136_dn2, locals.var_t0__blk136_dn6, locals.var_t0__blk136_dn7, locals.var_t0__blk136_dn10, locals.var_t0__blk136_dn11, locals.var_t0__blk136_dn12, locals.var_t0__blk136_dn17,)
    }
};
        locals.var_t0__blk136 = assign7410_e5339;
        locals.var_t0__blk136_dn0 = assign7410_e5339_d_n0;
        locals.var_t0__blk136_dn2 = assign7410_e5339_d_n2;
        locals.var_t0__blk136_dn6 = assign7410_e5339_d_n6;
        locals.var_t0__blk136_dn7 = assign7410_e5339_d_n7;
        locals.var_t0__blk136_dn10 = assign7410_e5339_d_n10;
        locals.var_t0__blk136_dn11 = assign7410_e5339_d_n11;
        locals.var_t0__blk136_dn12 = assign7410_e5339_d_n12;
        locals.var_t0__blk136_dn17 = assign7410_e5339_d_n17;
        locals.var_t0__blk136_rv = 0.0;

        let (assign7420_e5347, assign7420_e5347_d_n0, assign7420_e5347_d_n2, assign7420_e5347_d_n6, assign7420_e5347_d_n7, assign7420_e5347_d_n10, assign7420_e5347_d_n11, assign7420_e5347_d_n12, assign7420_e5347_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7420_e5345: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign7420_e5345, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk137, locals.var_t1__blk137_dn0, locals.var_t1__blk137_dn2, locals.var_t1__blk137_dn6, locals.var_t1__blk137_dn7, locals.var_t1__blk137_dn10, locals.var_t1__blk137_dn11, locals.var_t1__blk137_dn12, locals.var_t1__blk137_dn17,)
    }
};
        locals.var_t1__blk137 = assign7420_e5347;
        locals.var_t1__blk137_dn0 = assign7420_e5347_d_n0;
        locals.var_t1__blk137_dn2 = assign7420_e5347_d_n2;
        locals.var_t1__blk137_dn6 = assign7420_e5347_d_n6;
        locals.var_t1__blk137_dn7 = assign7420_e5347_d_n7;
        locals.var_t1__blk137_dn10 = assign7420_e5347_d_n10;
        locals.var_t1__blk137_dn11 = assign7420_e5347_d_n11;
        locals.var_t1__blk137_dn12 = assign7420_e5347_d_n12;
        locals.var_t1__blk137_dn17 = assign7420_e5347_d_n17;
        locals.var_t1__blk137_rv = 0.0;

        let (assign7430_e5355,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7430_e5353: f64 = (1.0 / locals.var_c_box);
        (assign7430_e5353,)
    } else {
        (locals.var_t2__blk138,)
    }
};
        locals.var_t2__blk138 = assign7430_e5355;
        locals.var_t2__blk138_rv = 0.0;

        let (assign7440_e5367, assign7440_e5367_d_n0, assign7440_e5367_d_n2, assign7440_e5367_d_n6, assign7440_e5367_d_n7, assign7440_e5367_d_n10, assign7440_e5367_d_n11, assign7440_e5367_d_n12, assign7440_e5367_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7440_e5362: f64 = (locals.var_t0__blk136 + locals.var_t1__blk137);
        let assign7440_e5364: f64 = (assign7440_e5362 + locals.var_t2__blk138);
        let assign7440_e5365: f64 = (1.0 / assign7440_e5364);
        (assign7440_e5365, (-((locals.var_t0__blk136_dn0 + locals.var_t1__blk137_dn0) / (assign7440_e5364 * assign7440_e5364))), (-((locals.var_t0__blk136_dn2 + locals.var_t1__blk137_dn2) / (assign7440_e5364 * assign7440_e5364))), (-((locals.var_t0__blk136_dn6 + locals.var_t1__blk137_dn6) / (assign7440_e5364 * assign7440_e5364))), (-((locals.var_t0__blk136_dn7 + locals.var_t1__blk137_dn7) / (assign7440_e5364 * assign7440_e5364))), (-((locals.var_t0__blk136_dn10 + locals.var_t1__blk137_dn10) / (assign7440_e5364 * assign7440_e5364))), (-((locals.var_t0__blk136_dn11 + locals.var_t1__blk137_dn11) / (assign7440_e5364 * assign7440_e5364))), (-((locals.var_t0__blk136_dn12 + locals.var_t1__blk137_dn12) / (assign7440_e5364 * assign7440_e5364))), (-((locals.var_t0__blk136_dn17 + locals.var_t1__blk137_dn17) / (assign7440_e5364 * assign7440_e5364))),)
    } else {
        (locals.var_t3__blk139, locals.var_t3__blk139_dn0, locals.var_t3__blk139_dn2, locals.var_t3__blk139_dn6, locals.var_t3__blk139_dn7, locals.var_t3__blk139_dn10, locals.var_t3__blk139_dn11, locals.var_t3__blk139_dn12, locals.var_t3__blk139_dn17,)
    }
};
        locals.var_t3__blk139 = assign7440_e5367;
        locals.var_t3__blk139_dn0 = assign7440_e5367_d_n0;
        locals.var_t3__blk139_dn2 = assign7440_e5367_d_n2;
        locals.var_t3__blk139_dn6 = assign7440_e5367_d_n6;
        locals.var_t3__blk139_dn7 = assign7440_e5367_d_n7;
        locals.var_t3__blk139_dn10 = assign7440_e5367_d_n10;
        locals.var_t3__blk139_dn11 = assign7440_e5367_d_n11;
        locals.var_t3__blk139_dn12 = assign7440_e5367_d_n12;
        locals.var_t3__blk139_dn17 = assign7440_e5367_d_n17;
        locals.var_t3__blk139_rv = 0.0;

        let (assign7450_e5377, assign7450_e5377_d_n0, assign7450_e5377_d_n2, assign7450_e5377_d_n6, assign7450_e5377_d_n7, assign7450_e5377_d_n10, assign7450_e5377_d_n11, assign7450_e5377_d_n12, assign7450_e5377_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7450_e5374: f64 = (locals.var_t3__blk139 * locals.var_t0__blk136);
        let assign7450_e5375: f64 = (1.0 - assign7450_e5374);
        (assign7450_e5375, (-((locals.var_t3__blk139_dn0 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn0))), (-((locals.var_t3__blk139_dn2 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn2))), (-((locals.var_t3__blk139_dn6 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn6))), (-((locals.var_t3__blk139_dn7 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn7))), (-((locals.var_t3__blk139_dn10 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn10))), (-((locals.var_t3__blk139_dn11 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn11))), (-((locals.var_t3__blk139_dn12 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn12))), (-((locals.var_t3__blk139_dn17 * locals.var_t0__blk136) + (locals.var_t3__blk139 * locals.var_t0__blk136_dn17))),)
    } else {
        (locals.var_t4__blk140, locals.var_t4__blk140_dn0, locals.var_t4__blk140_dn2, locals.var_t4__blk140_dn6, locals.var_t4__blk140_dn7, locals.var_t4__blk140_dn10, locals.var_t4__blk140_dn11, locals.var_t4__blk140_dn12, locals.var_t4__blk140_dn17,)
    }
};
        locals.var_t4__blk140 = assign7450_e5377;
        locals.var_t4__blk140_dn0 = assign7450_e5377_d_n0;
        locals.var_t4__blk140_dn2 = assign7450_e5377_d_n2;
        locals.var_t4__blk140_dn6 = assign7450_e5377_d_n6;
        locals.var_t4__blk140_dn7 = assign7450_e5377_d_n7;
        locals.var_t4__blk140_dn10 = assign7450_e5377_d_n10;
        locals.var_t4__blk140_dn11 = assign7450_e5377_d_n11;
        locals.var_t4__blk140_dn12 = assign7450_e5377_d_n12;
        locals.var_t4__blk140_dn17 = assign7450_e5377_d_n17;
        locals.var_t4__blk140_rv = 0.0;

        let (assign7460_e5397, assign7460_e5397_d_n0, assign7460_e5397_d_n2, assign7460_e5397_d_n6, assign7460_e5397_d_n7, assign7460_e5397_d_n10, assign7460_e5397_d_n11, assign7460_e5397_d_n12, assign7460_e5397_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7460_e5384: f64 = (-locals.var_vbsbiz);
        let assign7460_e5388: f64 = (0.5 * locals.var_t1__blk137);
        let assign7460_e5389: f64 = (locals.var_t2__blk138 + assign7460_e5388);
        let assign7460_e5391: f64 = (-locals.var_q_fd_soi);
        let assign7460_e5392: f64 = (assign7460_e5389 * assign7460_e5391);
        let assign7460_e5393: f64 = (assign7460_e5384 + assign7460_e5392);
        let assign7460_e5394: f64 = (locals.var_t3__blk139 * assign7460_e5393);
        let assign7460_e5395: f64 = (locals.var_t0__blk136 * assign7460_e5394);
        (assign7460_e5395, ((locals.var_t0__blk136_dn0 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn0 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn0) + (((0.5 * locals.var_t1__blk137_dn0) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn0)))))))), ((locals.var_t0__blk136_dn2 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn2 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn2) + (((0.5 * locals.var_t1__blk137_dn2) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn2)))))))), ((locals.var_t0__blk136_dn6 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn6 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn6) + (((0.5 * locals.var_t1__blk137_dn6) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn6)))))))), ((locals.var_t0__blk136_dn7 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn7 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn7) + (((0.5 * locals.var_t1__blk137_dn7) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn7)))))))), ((locals.var_t0__blk136_dn10 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn10 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn10) + (((0.5 * locals.var_t1__blk137_dn10) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn10)))))))), ((locals.var_t0__blk136_dn11 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn11 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn11) + (((0.5 * locals.var_t1__blk137_dn11) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn11)))))))), ((locals.var_t0__blk136_dn12 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn12 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn12) + (((0.5 * locals.var_t1__blk137_dn12) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn12)))))))), ((locals.var_t0__blk136_dn17 * assign7460_e5394) + (locals.var_t0__blk136 * ((locals.var_t3__blk139_dn17 * assign7460_e5393) + (locals.var_t3__blk139 * ((-locals.var_vbsbiz_dn17) + (((0.5 * locals.var_t1__blk137_dn17) * assign7460_e5391) + (assign7460_e5389 * (-locals.var_q_fd_soi_dn17)))))))),)
    } else {
        (locals.var_t5__blk141, locals.var_t5__blk141_dn0, locals.var_t5__blk141_dn2, locals.var_t5__blk141_dn6, locals.var_t5__blk141_dn7, locals.var_t5__blk141_dn10, locals.var_t5__blk141_dn11, locals.var_t5__blk141_dn12, locals.var_t5__blk141_dn17,)
    }
};
        locals.var_t5__blk141 = assign7460_e5397;
        locals.var_t5__blk141_dn0 = assign7460_e5397_d_n0;
        locals.var_t5__blk141_dn2 = assign7460_e5397_d_n2;
        locals.var_t5__blk141_dn6 = assign7460_e5397_d_n6;
        locals.var_t5__blk141_dn7 = assign7460_e5397_d_n7;
        locals.var_t5__blk141_dn10 = assign7460_e5397_d_n10;
        locals.var_t5__blk141_dn11 = assign7460_e5397_d_n11;
        locals.var_t5__blk141_dn12 = assign7460_e5397_d_n12;
        locals.var_t5__blk141_dn17 = assign7460_e5397_d_n17;
        locals.var_t5__blk141_rv = 0.0;

        let (assign7470_e5405, assign7470_e5405_d_n0, assign7470_e5405_d_n2, assign7470_e5405_d_n6, assign7470_e5405_d_n7, assign7470_e5405_d_n10, assign7470_e5405_d_n11, assign7470_e5405_d_n12, assign7470_e5405_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7470_e5403: f64 = (locals.var_t5__blk141 / locals.var_t4__blk140);
        (assign7470_e5403, (((locals.var_t5__blk141_dn0 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn0)) / (locals.var_t4__blk140 * locals.var_t4__blk140)), (((locals.var_t5__blk141_dn2 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn2)) / (locals.var_t4__blk140 * locals.var_t4__blk140)), (((locals.var_t5__blk141_dn6 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn6)) / (locals.var_t4__blk140 * locals.var_t4__blk140)), (((locals.var_t5__blk141_dn7 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn7)) / (locals.var_t4__blk140 * locals.var_t4__blk140)), (((locals.var_t5__blk141_dn10 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn10)) / (locals.var_t4__blk140 * locals.var_t4__blk140)), (((locals.var_t5__blk141_dn11 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn11)) / (locals.var_t4__blk140 * locals.var_t4__blk140)), (((locals.var_t5__blk141_dn12 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn12)) / (locals.var_t4__blk140 * locals.var_t4__blk140)), (((locals.var_t5__blk141_dn17 * locals.var_t4__blk140) - (locals.var_t5__blk141 * locals.var_t4__blk140_dn17)) / (locals.var_t4__blk140 * locals.var_t4__blk140)),)
    } else {
        (locals.var_shift, locals.var_shift_dn0, locals.var_shift_dn2, locals.var_shift_dn6, locals.var_shift_dn7, locals.var_shift_dn10, locals.var_shift_dn11, locals.var_shift_dn12, locals.var_shift_dn17,)
    }
};
        locals.var_shift = assign7470_e5405;
        locals.var_shift_dn0 = assign7470_e5405_d_n0;
        locals.var_shift_dn2 = assign7470_e5405_d_n2;
        locals.var_shift_dn6 = assign7470_e5405_d_n6;
        locals.var_shift_dn7 = assign7470_e5405_d_n7;
        locals.var_shift_dn10 = assign7470_e5405_d_n10;
        locals.var_shift_dn11 = assign7470_e5405_d_n11;
        locals.var_shift_dn12 = assign7470_e5405_d_n12;
        locals.var_shift_dn17 = assign7470_e5405_d_n17;
        locals.var_shift_rv = 0.0;

        let (assign7480_e5413, assign7480_e5413_d_n0, assign7480_e5413_d_n2, assign7480_e5413_d_n6, assign7480_e5413_d_n7, assign7480_e5413_d_n10, assign7480_e5413_d_n11, assign7480_e5413_d_n12, assign7480_e5413_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard142 != 0.0)) {
        let assign7480_e5411: f64 = (locals.var_vgs_fb + locals.var_shift);
        (assign7480_e5411, (locals.var_vgs_fb_dn0 + locals.var_shift_dn0), (locals.var_vgs_fb_dn2 + locals.var_shift_dn2), (locals.var_vgs_fb_dn6 + locals.var_shift_dn6), (locals.var_vgs_fb_dn7 + locals.var_shift_dn7), (locals.var_vgs_fb_dn10 + locals.var_shift_dn10), (locals.var_vgs_fb_dn11 + locals.var_shift_dn11), (locals.var_vgs_fb_dn12 + locals.var_shift_dn12), (locals.var_vgs_fb_dn17 + locals.var_shift_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign7480_e5413;
        locals.var_vgs_fb_dn0 = assign7480_e5413_d_n0;
        locals.var_vgs_fb_dn2 = assign7480_e5413_d_n2;
        locals.var_vgs_fb_dn6 = assign7480_e5413_d_n6;
        locals.var_vgs_fb_dn7 = assign7480_e5413_d_n7;
        locals.var_vgs_fb_dn10 = assign7480_e5413_d_n10;
        locals.var_vgs_fb_dn11 = assign7480_e5413_d_n11;
        locals.var_vgs_fb_dn12 = assign7480_e5413_d_n12;
        locals.var_vgs_fb_dn17 = assign7480_e5413_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let (assign7490_e5421, assign7490_e5421_d_n0, assign7490_e5421_d_n2, assign7490_e5421_d_n6, assign7490_e5421_d_n7, assign7490_e5421_d_n10, assign7490_e5421_d_n11, assign7490_e5421_d_n12, assign7490_e5421_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7490_e5417: f64 = (locals.var_vbsc_dvbse * locals.var_vds);
        let assign7490_e5419: f64 = (assign7490_e5417 / 2.0);
        (assign7490_e5419, (((locals.var_vbsc_dvbse_dn0 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn0)) / 2.0), (((locals.var_vbsc_dvbse_dn2 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn2)) / 2.0), (((locals.var_vbsc_dvbse_dn6 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn6)) / 2.0), (((locals.var_vbsc_dvbse_dn7 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn7)) / 2.0), (((locals.var_vbsc_dvbse_dn10 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn10)) / 2.0), (((locals.var_vbsc_dvbse_dn11 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn11)) / 2.0), (((locals.var_vbsc_dvbse_dn12 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn12)) / 2.0), (((locals.var_vbsc_dvbse_dn17 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn17)) / 2.0),)
    } else {
        (locals.var_t1__blk143, locals.var_t1__blk143_dn0, locals.var_t1__blk143_dn2, locals.var_t1__blk143_dn6, locals.var_t1__blk143_dn7, locals.var_t1__blk143_dn10, locals.var_t1__blk143_dn11, locals.var_t1__blk143_dn12, locals.var_t1__blk143_dn17,)
    }
};
        locals.var_t1__blk143 = assign7490_e5421;
        locals.var_t1__blk143_dn0 = assign7490_e5421_d_n0;
        locals.var_t1__blk143_dn2 = assign7490_e5421_d_n2;
        locals.var_t1__blk143_dn6 = assign7490_e5421_d_n6;
        locals.var_t1__blk143_dn7 = assign7490_e5421_d_n7;
        locals.var_t1__blk143_dn10 = assign7490_e5421_d_n10;
        locals.var_t1__blk143_dn11 = assign7490_e5421_d_n11;
        locals.var_t1__blk143_dn12 = assign7490_e5421_d_n12;
        locals.var_t1__blk143_dn17 = assign7490_e5421_d_n17;
        locals.var_t1__blk143_rv = 0.0;

        let (assign7500_e5429, assign7500_e5429_d_n0, assign7500_e5429_d_n2, assign7500_e5429_d_n6, assign7500_e5429_d_n7, assign7500_e5429_d_n10, assign7500_e5429_d_n11, assign7500_e5429_d_n12, assign7500_e5429_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7500_e5425: f64 = (2.0 * locals.var_t1__blk143);
        let assign7500_e5427: f64 = (assign7500_e5425 / 0.1);
        (assign7500_e5427, ((2.0 * locals.var_t1__blk143_dn0) / 0.1), ((2.0 * locals.var_t1__blk143_dn2) / 0.1), ((2.0 * locals.var_t1__blk143_dn6) / 0.1), ((2.0 * locals.var_t1__blk143_dn7) / 0.1), ((2.0 * locals.var_t1__blk143_dn10) / 0.1), ((2.0 * locals.var_t1__blk143_dn11) / 0.1), ((2.0 * locals.var_t1__blk143_dn12) / 0.1), ((2.0 * locals.var_t1__blk143_dn17) / 0.1),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7500_e5429;
        locals.var_tmf1_dn0 = assign7500_e5429_d_n0;
        locals.var_tmf1_dn2 = assign7500_e5429_d_n2;
        locals.var_tmf1_dn6 = assign7500_e5429_d_n6;
        locals.var_tmf1_dn7 = assign7500_e5429_d_n7;
        locals.var_tmf1_dn10 = assign7500_e5429_d_n10;
        locals.var_tmf1_dn11 = assign7500_e5429_d_n11;
        locals.var_tmf1_dn12 = assign7500_e5429_d_n12;
        locals.var_tmf1_dn17 = assign7500_e5429_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7510_e5469, assign7510_e5469_d_n0, assign7510_e5469_d_n2, assign7510_e5469_d_n6, assign7510_e5469_d_n7, assign7510_e5469_d_n10, assign7510_e5469_d_n11, assign7510_e5469_d_n12, assign7510_e5469_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7510_e5435: f64 = (1.0 / 2.0);
        let assign7510_e5439: f64 = (1.0 / 6.0);
        let assign7510_e5443: f64 = (1.0 / 24.0);
        let assign7510_e5447: f64 = (1.0 / 120.0);
        let assign7510_e5451: f64 = (1.0 / 720.0);
        let assign7510_e5455: f64 = (1.0 / 5040.0);
        let assign7510_e5456: f64 = (locals.var_tmf1 * assign7510_e5455);
        let assign7510_e5457: f64 = (assign7510_e5451 + assign7510_e5456);
        let assign7510_e5458: f64 = (locals.var_tmf1 * assign7510_e5457);
        let assign7510_e5459: f64 = (assign7510_e5447 + assign7510_e5458);
        let assign7510_e5460: f64 = (locals.var_tmf1 * assign7510_e5459);
        let assign7510_e5461: f64 = (assign7510_e5443 + assign7510_e5460);
        let assign7510_e5462: f64 = (locals.var_tmf1 * assign7510_e5461);
        let assign7510_e5463: f64 = (assign7510_e5439 + assign7510_e5462);
        let assign7510_e5464: f64 = (locals.var_tmf1 * assign7510_e5463);
        let assign7510_e5465: f64 = (assign7510_e5435 + assign7510_e5464);
        let assign7510_e5466: f64 = (locals.var_tmf1 * assign7510_e5465);
        let assign7510_e5467: f64 = (1.0 + assign7510_e5466);
        (assign7510_e5467, ((locals.var_tmf1_dn0 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign7510_e5455))))))))))), ((locals.var_tmf1_dn2 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign7510_e5455))))))))))), ((locals.var_tmf1_dn6 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign7510_e5455))))))))))), ((locals.var_tmf1_dn7 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign7510_e5455))))))))))), ((locals.var_tmf1_dn10 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign7510_e5455))))))))))), ((locals.var_tmf1_dn11 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign7510_e5455))))))))))), ((locals.var_tmf1_dn12 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign7510_e5455))))))))))), ((locals.var_tmf1_dn17 * assign7510_e5465) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7510_e5463) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7510_e5461) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7510_e5459) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign7510_e5457) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign7510_e5455))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7510_e5469;
        locals.var_tmf2_dn0 = assign7510_e5469_d_n0;
        locals.var_tmf2_dn2 = assign7510_e5469_d_n2;
        locals.var_tmf2_dn6 = assign7510_e5469_d_n6;
        locals.var_tmf2_dn7 = assign7510_e5469_d_n7;
        locals.var_tmf2_dn10 = assign7510_e5469_d_n10;
        locals.var_tmf2_dn11 = assign7510_e5469_d_n11;
        locals.var_tmf2_dn12 = assign7510_e5469_d_n12;
        locals.var_tmf2_dn17 = assign7510_e5469_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7520_e5475, assign7520_e5475_d_n0, assign7520_e5475_d_n2, assign7520_e5475_d_n6, assign7520_e5475_d_n7, assign7520_e5475_d_n10, assign7520_e5475_d_n11, assign7520_e5475_d_n12, assign7520_e5475_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7520_e5473: f64 = (0.1 / locals.var_tmf2);
        (assign7520_e5473, (-((0.1 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.1 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd__blk144, locals.var_vzadd__blk144_dn0, locals.var_vzadd__blk144_dn2, locals.var_vzadd__blk144_dn6, locals.var_vzadd__blk144_dn7, locals.var_vzadd__blk144_dn10, locals.var_vzadd__blk144_dn11, locals.var_vzadd__blk144_dn12, locals.var_vzadd__blk144_dn17,)
    }
};
        locals.var_vzadd__blk144 = assign7520_e5475;
        locals.var_vzadd__blk144_dn0 = assign7520_e5475_d_n0;
        locals.var_vzadd__blk144_dn2 = assign7520_e5475_d_n2;
        locals.var_vzadd__blk144_dn6 = assign7520_e5475_d_n6;
        locals.var_vzadd__blk144_dn7 = assign7520_e5475_d_n7;
        locals.var_vzadd__blk144_dn10 = assign7520_e5475_d_n10;
        locals.var_vzadd__blk144_dn11 = assign7520_e5475_d_n11;
        locals.var_vzadd__blk144_dn12 = assign7520_e5475_d_n12;
        locals.var_vzadd__blk144_dn17 = assign7520_e5475_d_n17;
        locals.var_vzadd__blk144_rv = 0.0;

        let assign7530_e5478: f64 = if locals.var_vzadd__blk144 < 5e-12 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7530_e5478;
        locals.var_guard145_rv = 0.0;

        let (assign7540_e5484, assign7540_e5484_d_n0, assign7540_e5484_d_n2, assign7540_e5484_d_n6, assign7540_e5484_d_n7, assign7540_e5484_d_n10, assign7540_e5484_d_n11, assign7540_e5484_d_n12, assign7540_e5484_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard145 != 0.0)) {
        (5e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd__blk144, locals.var_vzadd__blk144_dn0, locals.var_vzadd__blk144_dn2, locals.var_vzadd__blk144_dn6, locals.var_vzadd__blk144_dn7, locals.var_vzadd__blk144_dn10, locals.var_vzadd__blk144_dn11, locals.var_vzadd__blk144_dn12, locals.var_vzadd__blk144_dn17,)
    }
};
        locals.var_vzadd__blk144 = assign7540_e5484;
        locals.var_vzadd__blk144_dn0 = assign7540_e5484_d_n0;
        locals.var_vzadd__blk144_dn2 = assign7540_e5484_d_n2;
        locals.var_vzadd__blk144_dn6 = assign7540_e5484_d_n6;
        locals.var_vzadd__blk144_dn7 = assign7540_e5484_d_n7;
        locals.var_vzadd__blk144_dn10 = assign7540_e5484_d_n10;
        locals.var_vzadd__blk144_dn11 = assign7540_e5484_d_n11;
        locals.var_vzadd__blk144_dn12 = assign7540_e5484_d_n12;
        locals.var_vzadd__blk144_dn17 = assign7540_e5484_d_n17;
        locals.var_vzadd__blk144_rv = 0.0;

        let (assign7550_e5488, assign7550_e5488_d_n0, assign7550_e5488_d_n2, assign7550_e5488_d_n6, assign7550_e5488_d_n7, assign7550_e5488_d_n10, assign7550_e5488_d_n11, assign7550_e5488_d_n12, assign7550_e5488_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        (locals.var_vzadd__blk144, locals.var_vzadd__blk144_dn0, locals.var_vzadd__blk144_dn2, locals.var_vzadd__blk144_dn6, locals.var_vzadd__blk144_dn7, locals.var_vzadd__blk144_dn10, locals.var_vzadd__blk144_dn11, locals.var_vzadd__blk144_dn12, locals.var_vzadd__blk144_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign7550_e5488;
        locals.var_t3_dn0 = assign7550_e5488_d_n0;
        locals.var_t3_dn2 = assign7550_e5488_d_n2;
        locals.var_t3_dn6 = assign7550_e5488_d_n6;
        locals.var_t3_dn7 = assign7550_e5488_d_n7;
        locals.var_t3_dn10 = assign7550_e5488_d_n10;
        locals.var_t3_dn11 = assign7550_e5488_d_n11;
        locals.var_t3_dn12 = assign7550_e5488_d_n12;
        locals.var_t3_dn17 = assign7550_e5488_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign7560_e5500, assign7560_e5500_d_n0, assign7560_e5500_d_n2, assign7560_e5500_d_n6, assign7560_e5500_d_n7, assign7560_e5500_d_n10, assign7560_e5500_d_n11, assign7560_e5500_d_n12, assign7560_e5500_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7560_e5492: f64 = (locals.var_vgs + locals.var_t3);
        let assign7560_e5494: f64 = (assign7560_e5492 - locals.var_vfb);
        let assign7560_e5496: f64 = (assign7560_e5494 + locals.var_dvth);
        let assign7560_e5498: f64 = (assign7560_e5496 - locals.var_dppg);
        (assign7560_e5498, ((locals.var_t3_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_t3_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), (((locals.var_vgs_dn6 + locals.var_t3_dn6) + locals.var_dvth_dn6) - locals.var_dppg_dn6), (((locals.var_vgs_dn7 + locals.var_t3_dn7) + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_t3_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), (((locals.var_vgs_dn11 + locals.var_t3_dn11) + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_t3_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_t3_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpd, locals.var_vgpd_dn0, locals.var_vgpd_dn2, locals.var_vgpd_dn6, locals.var_vgpd_dn7, locals.var_vgpd_dn10, locals.var_vgpd_dn11, locals.var_vgpd_dn12, locals.var_vgpd_dn17,)
    }
};
        locals.var_vgpd = assign7560_e5500;
        locals.var_vgpd_dn0 = assign7560_e5500_d_n0;
        locals.var_vgpd_dn2 = assign7560_e5500_d_n2;
        locals.var_vgpd_dn6 = assign7560_e5500_d_n6;
        locals.var_vgpd_dn7 = assign7560_e5500_d_n7;
        locals.var_vgpd_dn10 = assign7560_e5500_d_n10;
        locals.var_vgpd_dn11 = assign7560_e5500_d_n11;
        locals.var_vgpd_dn12 = assign7560_e5500_d_n12;
        locals.var_vgpd_dn17 = assign7560_e5500_d_n17;
        locals.var_vgpd_rv = 0.0;

        let (assign7570_e5510, assign7570_e5510_d_n0, assign7570_e5510_d_n2, assign7570_e5510_d_n6, assign7570_e5510_d_n7, assign7570_e5510_d_n10, assign7570_e5510_d_n11, assign7570_e5510_d_n12, assign7570_e5510_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7570_e5505: f64 = (locals.var_wdsoi_ini1_dlt * locals.var_pb2);
        let assign7570_e5506: f64 = (locals.var_wdsoi_ini0 / assign7570_e5505);
        let assign7570_e5508: f64 = (assign7570_e5506 * locals.var_vgpd);
        (assign7570_e5508, (((((locals.var_wdsoi_ini0_dn0 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn0))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn0)), (((((locals.var_wdsoi_ini0_dn2 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn2))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn2)), (((((locals.var_wdsoi_ini0_dn6 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn6))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn6)), (((((locals.var_wdsoi_ini0_dn7 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn7))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn7)), (((((locals.var_wdsoi_ini0_dn10 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn10))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn10)), (((((locals.var_wdsoi_ini0_dn11 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn11))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn11)), (((((locals.var_wdsoi_ini0_dn12 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn12))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn12)), (((((locals.var_wdsoi_ini0_dn17 * assign7570_e5505) - (locals.var_wdsoi_ini0 * (locals.var_wdsoi_ini1_dlt * locals.var_pb2_dn17))) / (assign7570_e5505 * assign7570_e5505)) * locals.var_vgpd) + (assign7570_e5506 * locals.var_vgpd_dn17)),)
    } else {
        (locals.var_wdsoi_ini1, locals.var_wdsoi_ini1_dn0, locals.var_wdsoi_ini1_dn2, locals.var_wdsoi_ini1_dn6, locals.var_wdsoi_ini1_dn7, locals.var_wdsoi_ini1_dn10, locals.var_wdsoi_ini1_dn11, locals.var_wdsoi_ini1_dn12, locals.var_wdsoi_ini1_dn17,)
    }
};
        locals.var_wdsoi_ini1 = assign7570_e5510;
        locals.var_wdsoi_ini1_dn0 = assign7570_e5510_d_n0;
        locals.var_wdsoi_ini1_dn2 = assign7570_e5510_d_n2;
        locals.var_wdsoi_ini1_dn6 = assign7570_e5510_d_n6;
        locals.var_wdsoi_ini1_dn7 = assign7570_e5510_d_n7;
        locals.var_wdsoi_ini1_dn10 = assign7570_e5510_d_n10;
        locals.var_wdsoi_ini1_dn11 = assign7570_e5510_d_n11;
        locals.var_wdsoi_ini1_dn12 = assign7570_e5510_d_n12;
        locals.var_wdsoi_ini1_dn17 = assign7570_e5510_d_n17;
        locals.var_wdsoi_ini1_rv = 0.0;

        let assign7580_e5515: f64 = (locals.var_t_soi * 7.0);
        let assign7580_e5516: f64 = assign7580_e5515;
        let assign7580_e5520: f64 = (locals.var_t_soi * 7.0);
        let assign7580_e5523: f64 = if ((locals.var_wdsoi_ini1 < assign7580_e5516) && (assign7580_e5520 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7580_e5523;
        locals.var_guard146_rv = 0.0;

        let (assign7590_e5535, assign7590_e5535_d_n0, assign7590_e5535_d_n2, assign7590_e5535_d_n6, assign7590_e5535_d_n7, assign7590_e5535_d_n10, assign7590_e5535_d_n11, assign7590_e5535_d_n12, assign7590_e5535_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7590_e5530: f64 = (locals.var_t_soi * 7.0);
        let assign7590_e5531: f64 = assign7590_e5530;
        let assign7590_e5533: f64 = (assign7590_e5531 - locals.var_wdsoi_ini1);
        (assign7590_e5533, (-locals.var_wdsoi_ini1_dn0), (-locals.var_wdsoi_ini1_dn2), (-locals.var_wdsoi_ini1_dn6), (-locals.var_wdsoi_ini1_dn7), (-locals.var_wdsoi_ini1_dn10), (-locals.var_wdsoi_ini1_dn11), (-locals.var_wdsoi_ini1_dn12), (-locals.var_wdsoi_ini1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7590_e5535;
        locals.var_tmf1_dn0 = assign7590_e5535_d_n0;
        locals.var_tmf1_dn2 = assign7590_e5535_d_n2;
        locals.var_tmf1_dn6 = assign7590_e5535_d_n6;
        locals.var_tmf1_dn7 = assign7590_e5535_d_n7;
        locals.var_tmf1_dn10 = assign7590_e5535_d_n10;
        locals.var_tmf1_dn11 = assign7590_e5535_d_n11;
        locals.var_tmf1_dn12 = assign7590_e5535_d_n12;
        locals.var_tmf1_dn17 = assign7590_e5535_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7600_e5543, assign7600_e5543_d_n0, assign7600_e5543_d_n2, assign7600_e5543_d_n6, assign7600_e5543_d_n7, assign7600_e5543_d_n10, assign7600_e5543_d_n11, assign7600_e5543_d_n12, assign7600_e5543_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7600_e5541: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign7600_e5541, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign7600_e5543;
        locals.var_x2_dn0 = assign7600_e5543_d_n0;
        locals.var_x2_dn2 = assign7600_e5543_d_n2;
        locals.var_x2_dn6 = assign7600_e5543_d_n6;
        locals.var_x2_dn7 = assign7600_e5543_d_n7;
        locals.var_x2_dn10 = assign7600_e5543_d_n10;
        locals.var_x2_dn11 = assign7600_e5543_d_n11;
        locals.var_x2_dn12 = assign7600_e5543_d_n12;
        locals.var_x2_dn17 = assign7600_e5543_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign7610_e5555, assign7610_e5555_d_n0, assign7610_e5555_d_n2, assign7610_e5555_d_n6, assign7610_e5555_d_n7, assign7610_e5555_d_n10, assign7610_e5555_d_n11, assign7610_e5555_d_n12, assign7610_e5555_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7610_e5549: f64 = (locals.var_t_soi * 7.0);
        let assign7610_e5552: f64 = (locals.var_t_soi * 7.0);
        let assign7610_e5553: f64 = (assign7610_e5549 * assign7610_e5552);
        (assign7610_e5553, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign7610_e5555;
        locals.var_xmax2_dn0 = assign7610_e5555_d_n0;
        locals.var_xmax2_dn2 = assign7610_e5555_d_n2;
        locals.var_xmax2_dn6 = assign7610_e5555_d_n6;
        locals.var_xmax2_dn7 = assign7610_e5555_d_n7;
        locals.var_xmax2_dn10 = assign7610_e5555_d_n10;
        locals.var_xmax2_dn11 = assign7610_e5555_d_n11;
        locals.var_xmax2_dn12 = assign7610_e5555_d_n12;
        locals.var_xmax2_dn17 = assign7610_e5555_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign7620_e5561, assign7620_e5561_d_n0, assign7620_e5561_d_n2, assign7620_e5561_d_n6, assign7620_e5561_d_n7, assign7620_e5561_d_n10, assign7620_e5561_d_n11, assign7620_e5561_d_n12, assign7620_e5561_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7620_e5561;
        locals.var_xp_dn0 = assign7620_e5561_d_n0;
        locals.var_xp_dn2 = assign7620_e5561_d_n2;
        locals.var_xp_dn6 = assign7620_e5561_d_n6;
        locals.var_xp_dn7 = assign7620_e5561_d_n7;
        locals.var_xp_dn10 = assign7620_e5561_d_n10;
        locals.var_xp_dn11 = assign7620_e5561_d_n11;
        locals.var_xp_dn12 = assign7620_e5561_d_n12;
        locals.var_xp_dn17 = assign7620_e5561_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7630_e5567, assign7630_e5567_d_n0, assign7630_e5567_d_n2, assign7630_e5567_d_n6, assign7630_e5567_d_n7, assign7630_e5567_d_n10, assign7630_e5567_d_n11, assign7630_e5567_d_n12, assign7630_e5567_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7630_e5567;
        locals.var_xmp_dn0 = assign7630_e5567_d_n0;
        locals.var_xmp_dn2 = assign7630_e5567_d_n2;
        locals.var_xmp_dn6 = assign7630_e5567_d_n6;
        locals.var_xmp_dn7 = assign7630_e5567_d_n7;
        locals.var_xmp_dn10 = assign7630_e5567_d_n10;
        locals.var_xmp_dn11 = assign7630_e5567_d_n11;
        locals.var_xmp_dn12 = assign7630_e5567_d_n12;
        locals.var_xmp_dn17 = assign7630_e5567_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign7640_e5573,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign7640_e5573;
        locals.var_m0_rv = 0.0;

        let (assign7650_e5579,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7650_e5579;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign7660_e5585, assign7660_e5585_d_n0, assign7660_e5585_d_n2, assign7660_e5585_d_n6, assign7660_e5585_d_n7, assign7660_e5585_d_n10, assign7660_e5585_d_n11, assign7660_e5585_d_n12, assign7660_e5585_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign7660_e5585;
        locals.var_arg_dn0 = assign7660_e5585_d_n0;
        locals.var_arg_dn2 = assign7660_e5585_d_n2;
        locals.var_arg_dn6 = assign7660_e5585_d_n6;
        locals.var_arg_dn7 = assign7660_e5585_d_n7;
        locals.var_arg_dn10 = assign7660_e5585_d_n10;
        locals.var_arg_dn11 = assign7660_e5585_d_n11;
        locals.var_arg_dn12 = assign7660_e5585_d_n12;
        locals.var_arg_dn17 = assign7660_e5585_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign7670_e5591, assign7670_e5591_d_n0, assign7670_e5591_d_n2, assign7670_e5591_d_n6, assign7670_e5591_d_n7, assign7670_e5591_d_n10, assign7670_e5591_d_n11, assign7670_e5591_d_n12, assign7670_e5591_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7670_e5591;
        locals.var_dnm_dn0 = assign7670_e5591_d_n0;
        locals.var_dnm_dn2 = assign7670_e5591_d_n2;
        locals.var_dnm_dn6 = assign7670_e5591_d_n6;
        locals.var_dnm_dn7 = assign7670_e5591_d_n7;
        locals.var_dnm_dn10 = assign7670_e5591_d_n10;
        locals.var_dnm_dn11 = assign7670_e5591_d_n11;
        locals.var_dnm_dn12 = assign7670_e5591_d_n12;
        locals.var_dnm_dn17 = assign7670_e5591_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign7680_e5599, assign7680_e5599_d_n0, assign7680_e5599_d_n2, assign7680_e5599_d_n6, assign7680_e5599_d_n7, assign7680_e5599_d_n10, assign7680_e5599_d_n11, assign7680_e5599_d_n12, assign7680_e5599_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7680_e5597: f64 = (locals.var_xp * locals.var_x2);
        (assign7680_e5597, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7680_e5599;
        locals.var_xp_dn0 = assign7680_e5599_d_n0;
        locals.var_xp_dn2 = assign7680_e5599_d_n2;
        locals.var_xp_dn6 = assign7680_e5599_d_n6;
        locals.var_xp_dn7 = assign7680_e5599_d_n7;
        locals.var_xp_dn10 = assign7680_e5599_d_n10;
        locals.var_xp_dn11 = assign7680_e5599_d_n11;
        locals.var_xp_dn12 = assign7680_e5599_d_n12;
        locals.var_xp_dn17 = assign7680_e5599_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7690_e5607, assign7690_e5607_d_n0, assign7690_e5607_d_n2, assign7690_e5607_d_n6, assign7690_e5607_d_n7, assign7690_e5607_d_n10, assign7690_e5607_d_n11, assign7690_e5607_d_n12, assign7690_e5607_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7690_e5605: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign7690_e5605, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7690_e5607;
        locals.var_xmp_dn0 = assign7690_e5607_d_n0;
        locals.var_xmp_dn2 = assign7690_e5607_d_n2;
        locals.var_xmp_dn6 = assign7690_e5607_d_n6;
        locals.var_xmp_dn7 = assign7690_e5607_d_n7;
        locals.var_xmp_dn10 = assign7690_e5607_d_n10;
        locals.var_xmp_dn11 = assign7690_e5607_d_n11;
        locals.var_xmp_dn12 = assign7690_e5607_d_n12;
        locals.var_xmp_dn17 = assign7690_e5607_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign7700_e5615, assign7700_e5615_d_n0, assign7700_e5615_d_n2, assign7700_e5615_d_n6, assign7700_e5615_d_n7, assign7700_e5615_d_n10, assign7700_e5615_d_n11, assign7700_e5615_d_n12, assign7700_e5615_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7700_e5613: f64 = (locals.var_xp * locals.var_x2);
        (assign7700_e5613, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7700_e5615;
        locals.var_xp_dn0 = assign7700_e5615_d_n0;
        locals.var_xp_dn2 = assign7700_e5615_d_n2;
        locals.var_xp_dn6 = assign7700_e5615_d_n6;
        locals.var_xp_dn7 = assign7700_e5615_d_n7;
        locals.var_xp_dn10 = assign7700_e5615_d_n10;
        locals.var_xp_dn11 = assign7700_e5615_d_n11;
        locals.var_xp_dn12 = assign7700_e5615_d_n12;
        locals.var_xp_dn17 = assign7700_e5615_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7710_e5623, assign7710_e5623_d_n0, assign7710_e5623_d_n2, assign7710_e5623_d_n6, assign7710_e5623_d_n7, assign7710_e5623_d_n10, assign7710_e5623_d_n11, assign7710_e5623_d_n12, assign7710_e5623_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7710_e5621: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign7710_e5621, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7710_e5623;
        locals.var_xmp_dn0 = assign7710_e5623_d_n0;
        locals.var_xmp_dn2 = assign7710_e5623_d_n2;
        locals.var_xmp_dn6 = assign7710_e5623_d_n6;
        locals.var_xmp_dn7 = assign7710_e5623_d_n7;
        locals.var_xmp_dn10 = assign7710_e5623_d_n10;
        locals.var_xmp_dn11 = assign7710_e5623_d_n11;
        locals.var_xmp_dn12 = assign7710_e5623_d_n12;
        locals.var_xmp_dn17 = assign7710_e5623_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign7720_e5631, assign7720_e5631_d_n0, assign7720_e5631_d_n2, assign7720_e5631_d_n6, assign7720_e5631_d_n7, assign7720_e5631_d_n10, assign7720_e5631_d_n11, assign7720_e5631_d_n12, assign7720_e5631_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7720_e5629: f64 = (locals.var_xp + locals.var_xmp);
        (assign7720_e5629, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign7720_e5631;
        locals.var_arg_dn0 = assign7720_e5631_d_n0;
        locals.var_arg_dn2 = assign7720_e5631_d_n2;
        locals.var_arg_dn6 = assign7720_e5631_d_n6;
        locals.var_arg_dn7 = assign7720_e5631_d_n7;
        locals.var_arg_dn10 = assign7720_e5631_d_n10;
        locals.var_arg_dn11 = assign7720_e5631_d_n11;
        locals.var_arg_dn12 = assign7720_e5631_d_n12;
        locals.var_arg_dn17 = assign7720_e5631_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign7730_e5637, assign7730_e5637_d_n0, assign7730_e5637_d_n2, assign7730_e5637_d_n6, assign7730_e5637_d_n7, assign7730_e5637_d_n10, assign7730_e5637_d_n11, assign7730_e5637_d_n12, assign7730_e5637_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7730_e5637;
        locals.var_dnm_dn0 = assign7730_e5637_d_n0;
        locals.var_dnm_dn2 = assign7730_e5637_d_n2;
        locals.var_dnm_dn6 = assign7730_e5637_d_n6;
        locals.var_dnm_dn7 = assign7730_e5637_d_n7;
        locals.var_dnm_dn10 = assign7730_e5637_d_n10;
        locals.var_dnm_dn11 = assign7730_e5637_d_n11;
        locals.var_dnm_dn12 = assign7730_e5637_d_n12;
        locals.var_dnm_dn17 = assign7730_e5637_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign7740_e5652: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7740_e5652;
        locals.var_guard147_rv = 0.0;

        let assign7750_e5655: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7750_e5655;
        locals.var_guard148_rv = 0.0;

        let (assign7760_e5665,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7760_e5665;
        locals.var_mm_rv = 0.0;

        let assign7770_e5668: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7770_e5668;
        locals.var_guard149_rv = 0.0;

        let (assign7780_e5681,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 == 0.0)) && (locals.var_guard149 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7780_e5681;
        locals.var_mm_rv = 0.0;

        let assign7790_e5684: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7790_e5684;
        locals.var_guard150_rv = 0.0;

        let (assign7800_e5700,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 == 0.0)) && (locals.var_guard149 == 0.0)) && (locals.var_guard150 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7800_e5700;
        locals.var_mm_rv = 0.0;

        let assign7810_e5703: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7810_e5703;
        locals.var_guard151_rv = 0.0;

        let (assign7820_e5722,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 == 0.0)) && (locals.var_guard149 == 0.0)) && (locals.var_guard150 == 0.0)) && (locals.var_guard151 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7820_e5722;
        locals.var_mm_rv = 0.0;

        let (assign7830_e5730,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign7830_e5730;
        locals.var_m0_rv = 0.0;

        let mut assign7840_loop_guard: usize = 0;
        while {
            let assign7840_cond_e5739: f64 = if ((((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign7840_cond_e5739 != 0.0
        } {
            assign7840_loop_guard += 1;
            assert!(assign7840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7840_body0_e5748, assign7840_body0_e5748_d_n0, assign7840_body0_e5748_d_n2, assign7840_body0_e5748_d_n6, assign7840_body0_e5748_d_n7, assign7840_body0_e5748_d_n10, assign7840_body0_e5748_d_n11, assign7840_body0_e5748_d_n12, assign7840_body0_e5748_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) {
        let assign7840_body0_e5746: f64 = (locals.var_dnm).sqrt();
        (assign7840_body0_e5746, (locals.var_dnm_dn0 / (2.0 * assign7840_body0_e5746)), (locals.var_dnm_dn2 / (2.0 * assign7840_body0_e5746)), (locals.var_dnm_dn6 / (2.0 * assign7840_body0_e5746)), (locals.var_dnm_dn7 / (2.0 * assign7840_body0_e5746)), (locals.var_dnm_dn10 / (2.0 * assign7840_body0_e5746)), (locals.var_dnm_dn11 / (2.0 * assign7840_body0_e5746)), (locals.var_dnm_dn12 / (2.0 * assign7840_body0_e5746)), (locals.var_dnm_dn17 / (2.0 * assign7840_body0_e5746)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign7840_body0_e5748;
            locals.var_dnm_dn0 = assign7840_body0_e5748_d_n0;
            locals.var_dnm_dn2 = assign7840_body0_e5748_d_n2;
            locals.var_dnm_dn6 = assign7840_body0_e5748_d_n6;
            locals.var_dnm_dn7 = assign7840_body0_e5748_d_n7;
            locals.var_dnm_dn10 = assign7840_body0_e5748_d_n10;
            locals.var_dnm_dn11 = assign7840_body0_e5748_d_n11;
            locals.var_dnm_dn12 = assign7840_body0_e5748_d_n12;
            locals.var_dnm_dn17 = assign7840_body0_e5748_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign7840_body1_e5758,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 != 0.0)) {
        let assign7840_body1_e5756: f64 = (locals.var_m0 + 1.0);
        (assign7840_body1_e5756,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign7840_body1_e5758;
            locals.var_m0_rv = 0.0;
        }

        let (assign7850_e5773, assign7850_e5773_d_n0, assign7850_e5773_d_n2, assign7850_e5773_d_n6, assign7850_e5773_d_n7, assign7850_e5773_d_n10, assign7850_e5773_d_n11, assign7850_e5773_d_n12, assign7850_e5773_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign7850_e5769: f64 = (2.0 * 2.0);
        let assign7850_e5770: f64 = (1.0 / assign7850_e5769);
        let assign7850_e5771: f64 = (locals.var_dnm).powf(assign7850_e5770);
        (assign7850_e5771, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn0)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn2)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn6)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn7)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn10)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn11)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn12)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign7850_e5770) as f64).is_finite() && ((assign7850_e5770) as f64).fract() == 0.0 { if assign7850_e5770 == 0.0 { 0.0 } else { (assign7850_e5770 * ((locals.var_dnm).powf(assign7850_e5770 - 1.0) * locals.var_dnm_dn17)) } } else { (assign7850_e5771 * (assign7850_e5770 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7850_e5773;
        locals.var_dnm_dn0 = assign7850_e5773_d_n0;
        locals.var_dnm_dn2 = assign7850_e5773_d_n2;
        locals.var_dnm_dn6 = assign7850_e5773_d_n6;
        locals.var_dnm_dn7 = assign7850_e5773_d_n7;
        locals.var_dnm_dn10 = assign7850_e5773_d_n10;
        locals.var_dnm_dn11 = assign7850_e5773_d_n11;
        locals.var_dnm_dn12 = assign7850_e5773_d_n12;
        locals.var_dnm_dn17 = assign7850_e5773_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign7860_e5781, assign7860_e5781_d_n0, assign7860_e5781_d_n2, assign7860_e5781_d_n6, assign7860_e5781_d_n7, assign7860_e5781_d_n10, assign7860_e5781_d_n11, assign7860_e5781_d_n12, assign7860_e5781_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7860_e5779: f64 = (1.0 / locals.var_dnm);
        (assign7860_e5779, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7860_e5781;
        locals.var_dnm_dn0 = assign7860_e5781_d_n0;
        locals.var_dnm_dn2 = assign7860_e5781_d_n2;
        locals.var_dnm_dn6 = assign7860_e5781_d_n6;
        locals.var_dnm_dn7 = assign7860_e5781_d_n7;
        locals.var_dnm_dn10 = assign7860_e5781_d_n10;
        locals.var_dnm_dn11 = assign7860_e5781_d_n11;
        locals.var_dnm_dn12 = assign7860_e5781_d_n12;
        locals.var_dnm_dn17 = assign7860_e5781_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign7870_e5793, assign7870_e5793_d_n0, assign7870_e5793_d_n2, assign7870_e5793_d_n6, assign7870_e5793_d_n7, assign7870_e5793_d_n10, assign7870_e5793_d_n11, assign7870_e5793_d_n12, assign7870_e5793_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7870_e5788: f64 = (locals.var_t_soi * 7.0);
        let assign7870_e5789: f64 = (locals.var_tmf1 * assign7870_e5788);
        let assign7870_e5791: f64 = (assign7870_e5789 * locals.var_dnm);
        (assign7870_e5791, (((locals.var_tmf1_dn0 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * assign7870_e5788) * locals.var_dnm) + (assign7870_e5789 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign7870_e5793;
        locals.var_tmf0_dn0 = assign7870_e5793_d_n0;
        locals.var_tmf0_dn2 = assign7870_e5793_d_n2;
        locals.var_tmf0_dn6 = assign7870_e5793_d_n6;
        locals.var_tmf0_dn7 = assign7870_e5793_d_n7;
        locals.var_tmf0_dn10 = assign7870_e5793_d_n10;
        locals.var_tmf0_dn11 = assign7870_e5793_d_n11;
        locals.var_tmf0_dn12 = assign7870_e5793_d_n12;
        locals.var_tmf0_dn17 = assign7870_e5793_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign7880_e5805, assign7880_e5805_d_n0, assign7880_e5805_d_n2, assign7880_e5805_d_n6, assign7880_e5805_d_n7, assign7880_e5805_d_n10, assign7880_e5805_d_n11, assign7880_e5805_d_n12, assign7880_e5805_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 != 0.0)) {
        let assign7880_e5800: f64 = (locals.var_t_soi * 7.0);
        let assign7880_e5801: f64 = assign7880_e5800;
        let assign7880_e5803: f64 = (assign7880_e5801 - locals.var_tmf0);
        (assign7880_e5803, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign7880_e5805;
        locals.var_wdsoi_ini2_dn0 = assign7880_e5805_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign7880_e5805_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign7880_e5805_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign7880_e5805_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign7880_e5805_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign7880_e5805_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign7880_e5805_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign7880_e5805_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let (assign7890_e5812, assign7890_e5812_d_n0, assign7890_e5812_d_n2, assign7890_e5812_d_n6, assign7890_e5812_d_n7, assign7890_e5812_d_n10, assign7890_e5812_d_n11, assign7890_e5812_d_n12, assign7890_e5812_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard146 == 0.0)) {
        (locals.var_wdsoi_ini1, locals.var_wdsoi_ini1_dn0, locals.var_wdsoi_ini1_dn2, locals.var_wdsoi_ini1_dn6, locals.var_wdsoi_ini1_dn7, locals.var_wdsoi_ini1_dn10, locals.var_wdsoi_ini1_dn11, locals.var_wdsoi_ini1_dn12, locals.var_wdsoi_ini1_dn17,)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign7890_e5812;
        locals.var_wdsoi_ini2_dn0 = assign7890_e5812_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign7890_e5812_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign7890_e5812_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign7890_e5812_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign7890_e5812_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign7890_e5812_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign7890_e5812_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign7890_e5812_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let assign7900_e5817: f64 = locals.var_t_soi;
        let assign7900_e5818: f64 = (locals.var_wdsoi_ini0 - assign7900_e5817);
        let assign7900_e5822: f64 = locals.var_t_soi;
        let assign7900_e5825: f64 = if ((locals.var_wdsoi_ini2 > assign7900_e5818) && (assign7900_e5822 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard152 = assign7900_e5825;
        locals.var_guard152_rv = 0.0;

        let (assign7910_e5837, assign7910_e5837_d_n0, assign7910_e5837_d_n2, assign7910_e5837_d_n6, assign7910_e5837_d_n7, assign7910_e5837_d_n10, assign7910_e5837_d_n11, assign7910_e5837_d_n12, assign7910_e5837_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign7910_e5831: f64 = (locals.var_wdsoi_ini2 - locals.var_wdsoi_ini0);
        let assign7910_e5834: f64 = locals.var_t_soi;
        let assign7910_e5835: f64 = (assign7910_e5831 + assign7910_e5834);
        (assign7910_e5835, (locals.var_wdsoi_ini2_dn0 - locals.var_wdsoi_ini0_dn0), (locals.var_wdsoi_ini2_dn2 - locals.var_wdsoi_ini0_dn2), (locals.var_wdsoi_ini2_dn6 - locals.var_wdsoi_ini0_dn6), (locals.var_wdsoi_ini2_dn7 - locals.var_wdsoi_ini0_dn7), (locals.var_wdsoi_ini2_dn10 - locals.var_wdsoi_ini0_dn10), (locals.var_wdsoi_ini2_dn11 - locals.var_wdsoi_ini0_dn11), (locals.var_wdsoi_ini2_dn12 - locals.var_wdsoi_ini0_dn12), (locals.var_wdsoi_ini2_dn17 - locals.var_wdsoi_ini0_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7910_e5837;
        locals.var_tmf1_dn0 = assign7910_e5837_d_n0;
        locals.var_tmf1_dn2 = assign7910_e5837_d_n2;
        locals.var_tmf1_dn6 = assign7910_e5837_d_n6;
        locals.var_tmf1_dn7 = assign7910_e5837_d_n7;
        locals.var_tmf1_dn10 = assign7910_e5837_d_n10;
        locals.var_tmf1_dn11 = assign7910_e5837_d_n11;
        locals.var_tmf1_dn12 = assign7910_e5837_d_n12;
        locals.var_tmf1_dn17 = assign7910_e5837_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7920_e5845, assign7920_e5845_d_n0, assign7920_e5845_d_n2, assign7920_e5845_d_n6, assign7920_e5845_d_n7, assign7920_e5845_d_n10, assign7920_e5845_d_n11, assign7920_e5845_d_n12, assign7920_e5845_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign7920_e5843: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign7920_e5843, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign7920_e5845;
        locals.var_x2_dn0 = assign7920_e5845_d_n0;
        locals.var_x2_dn2 = assign7920_e5845_d_n2;
        locals.var_x2_dn6 = assign7920_e5845_d_n6;
        locals.var_x2_dn7 = assign7920_e5845_d_n7;
        locals.var_x2_dn10 = assign7920_e5845_d_n10;
        locals.var_x2_dn11 = assign7920_e5845_d_n11;
        locals.var_x2_dn12 = assign7920_e5845_d_n12;
        locals.var_x2_dn17 = assign7920_e5845_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign7930_e5857, assign7930_e5857_d_n0, assign7930_e5857_d_n2, assign7930_e5857_d_n6, assign7930_e5857_d_n7, assign7930_e5857_d_n10, assign7930_e5857_d_n11, assign7930_e5857_d_n12, assign7930_e5857_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign7930_e5851: f64 = locals.var_t_soi;
        let assign7930_e5854: f64 = locals.var_t_soi;
        let assign7930_e5855: f64 = (assign7930_e5851 * assign7930_e5854);
        (assign7930_e5855, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign7930_e5857;
        locals.var_xmax2_dn0 = assign7930_e5857_d_n0;
        locals.var_xmax2_dn2 = assign7930_e5857_d_n2;
        locals.var_xmax2_dn6 = assign7930_e5857_d_n6;
        locals.var_xmax2_dn7 = assign7930_e5857_d_n7;
        locals.var_xmax2_dn10 = assign7930_e5857_d_n10;
        locals.var_xmax2_dn11 = assign7930_e5857_d_n11;
        locals.var_xmax2_dn12 = assign7930_e5857_d_n12;
        locals.var_xmax2_dn17 = assign7930_e5857_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign7940_e5863, assign7940_e5863_d_n0, assign7940_e5863_d_n2, assign7940_e5863_d_n6, assign7940_e5863_d_n7, assign7940_e5863_d_n10, assign7940_e5863_d_n11, assign7940_e5863_d_n12, assign7940_e5863_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign7940_e5863;
        locals.var_xp_dn0 = assign7940_e5863_d_n0;
        locals.var_xp_dn2 = assign7940_e5863_d_n2;
        locals.var_xp_dn6 = assign7940_e5863_d_n6;
        locals.var_xp_dn7 = assign7940_e5863_d_n7;
        locals.var_xp_dn10 = assign7940_e5863_d_n10;
        locals.var_xp_dn11 = assign7940_e5863_d_n11;
        locals.var_xp_dn12 = assign7940_e5863_d_n12;
        locals.var_xp_dn17 = assign7940_e5863_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign7950_e5869, assign7950_e5869_d_n0, assign7950_e5869_d_n2, assign7950_e5869_d_n6, assign7950_e5869_d_n7, assign7950_e5869_d_n10, assign7950_e5869_d_n11, assign7950_e5869_d_n12, assign7950_e5869_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign7950_e5869;
        locals.var_xmp_dn0 = assign7950_e5869_d_n0;
        locals.var_xmp_dn2 = assign7950_e5869_d_n2;
        locals.var_xmp_dn6 = assign7950_e5869_d_n6;
        locals.var_xmp_dn7 = assign7950_e5869_d_n7;
        locals.var_xmp_dn10 = assign7950_e5869_d_n10;
        locals.var_xmp_dn11 = assign7950_e5869_d_n11;
        locals.var_xmp_dn12 = assign7950_e5869_d_n12;
        locals.var_xmp_dn17 = assign7950_e5869_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign7960_e5875,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign7960_e5875;
        locals.var_m0_rv = 0.0;

        let (assign7970_e5881,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign7970_e5881;
        locals.var_mm_rv = 0.0;

        let (assign7980_e5887, assign7980_e5887_d_n0, assign7980_e5887_d_n2, assign7980_e5887_d_n6, assign7980_e5887_d_n7, assign7980_e5887_d_n10, assign7980_e5887_d_n11, assign7980_e5887_d_n12, assign7980_e5887_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign7980_e5887;
        locals.var_arg_dn0 = assign7980_e5887_d_n0;
        locals.var_arg_dn2 = assign7980_e5887_d_n2;
        locals.var_arg_dn6 = assign7980_e5887_d_n6;
        locals.var_arg_dn7 = assign7980_e5887_d_n7;
        locals.var_arg_dn10 = assign7980_e5887_d_n10;
        locals.var_arg_dn11 = assign7980_e5887_d_n11;
        locals.var_arg_dn12 = assign7980_e5887_d_n12;
        locals.var_arg_dn17 = assign7980_e5887_d_n17;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign7990_e5893, assign7990_e5893_d_n0, assign7990_e5893_d_n2, assign7990_e5893_d_n6, assign7990_e5893_d_n7, assign7990_e5893_d_n10, assign7990_e5893_d_n11, assign7990_e5893_d_n12, assign7990_e5893_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign7990_e5893;
        locals.var_dnm_dn0 = assign7990_e5893_d_n0;
        locals.var_dnm_dn2 = assign7990_e5893_d_n2;
        locals.var_dnm_dn6 = assign7990_e5893_d_n6;
        locals.var_dnm_dn7 = assign7990_e5893_d_n7;
        locals.var_dnm_dn10 = assign7990_e5893_d_n10;
        locals.var_dnm_dn11 = assign7990_e5893_d_n11;
        locals.var_dnm_dn12 = assign7990_e5893_d_n12;
        locals.var_dnm_dn17 = assign7990_e5893_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign8000_e5901, assign8000_e5901_d_n0, assign8000_e5901_d_n2, assign8000_e5901_d_n6, assign8000_e5901_d_n7, assign8000_e5901_d_n10, assign8000_e5901_d_n11, assign8000_e5901_d_n12, assign8000_e5901_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8000_e5899: f64 = (locals.var_xp * locals.var_x2);
        (assign8000_e5899, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8000_e5901;
        locals.var_xp_dn0 = assign8000_e5901_d_n0;
        locals.var_xp_dn2 = assign8000_e5901_d_n2;
        locals.var_xp_dn6 = assign8000_e5901_d_n6;
        locals.var_xp_dn7 = assign8000_e5901_d_n7;
        locals.var_xp_dn10 = assign8000_e5901_d_n10;
        locals.var_xp_dn11 = assign8000_e5901_d_n11;
        locals.var_xp_dn12 = assign8000_e5901_d_n12;
        locals.var_xp_dn17 = assign8000_e5901_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8010_e5909, assign8010_e5909_d_n0, assign8010_e5909_d_n2, assign8010_e5909_d_n6, assign8010_e5909_d_n7, assign8010_e5909_d_n10, assign8010_e5909_d_n11, assign8010_e5909_d_n12, assign8010_e5909_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8010_e5907: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign8010_e5907, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8010_e5909;
        locals.var_xmp_dn0 = assign8010_e5909_d_n0;
        locals.var_xmp_dn2 = assign8010_e5909_d_n2;
        locals.var_xmp_dn6 = assign8010_e5909_d_n6;
        locals.var_xmp_dn7 = assign8010_e5909_d_n7;
        locals.var_xmp_dn10 = assign8010_e5909_d_n10;
        locals.var_xmp_dn11 = assign8010_e5909_d_n11;
        locals.var_xmp_dn12 = assign8010_e5909_d_n12;
        locals.var_xmp_dn17 = assign8010_e5909_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8020_e5917, assign8020_e5917_d_n0, assign8020_e5917_d_n2, assign8020_e5917_d_n6, assign8020_e5917_d_n7, assign8020_e5917_d_n10, assign8020_e5917_d_n11, assign8020_e5917_d_n12, assign8020_e5917_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8020_e5915: f64 = (locals.var_xp * locals.var_x2);
        (assign8020_e5915, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8020_e5917;
        locals.var_xp_dn0 = assign8020_e5917_d_n0;
        locals.var_xp_dn2 = assign8020_e5917_d_n2;
        locals.var_xp_dn6 = assign8020_e5917_d_n6;
        locals.var_xp_dn7 = assign8020_e5917_d_n7;
        locals.var_xp_dn10 = assign8020_e5917_d_n10;
        locals.var_xp_dn11 = assign8020_e5917_d_n11;
        locals.var_xp_dn12 = assign8020_e5917_d_n12;
        locals.var_xp_dn17 = assign8020_e5917_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8030_e5925, assign8030_e5925_d_n0, assign8030_e5925_d_n2, assign8030_e5925_d_n6, assign8030_e5925_d_n7, assign8030_e5925_d_n10, assign8030_e5925_d_n11, assign8030_e5925_d_n12, assign8030_e5925_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8030_e5923: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign8030_e5923, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8030_e5925;
        locals.var_xmp_dn0 = assign8030_e5925_d_n0;
        locals.var_xmp_dn2 = assign8030_e5925_d_n2;
        locals.var_xmp_dn6 = assign8030_e5925_d_n6;
        locals.var_xmp_dn7 = assign8030_e5925_d_n7;
        locals.var_xmp_dn10 = assign8030_e5925_d_n10;
        locals.var_xmp_dn11 = assign8030_e5925_d_n11;
        locals.var_xmp_dn12 = assign8030_e5925_d_n12;
        locals.var_xmp_dn17 = assign8030_e5925_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8040_e5933, assign8040_e5933_d_n0, assign8040_e5933_d_n2, assign8040_e5933_d_n6, assign8040_e5933_d_n7, assign8040_e5933_d_n10, assign8040_e5933_d_n11, assign8040_e5933_d_n12, assign8040_e5933_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8040_e5931: f64 = (locals.var_xp + locals.var_xmp);
        (assign8040_e5931, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign8040_e5933;
        locals.var_arg_dn0 = assign8040_e5933_d_n0;
        locals.var_arg_dn2 = assign8040_e5933_d_n2;
        locals.var_arg_dn6 = assign8040_e5933_d_n6;
        locals.var_arg_dn7 = assign8040_e5933_d_n7;
        locals.var_arg_dn10 = assign8040_e5933_d_n10;
        locals.var_arg_dn11 = assign8040_e5933_d_n11;
        locals.var_arg_dn12 = assign8040_e5933_d_n12;
        locals.var_arg_dn17 = assign8040_e5933_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign8050_e5939, assign8050_e5939_d_n0, assign8050_e5939_d_n2, assign8050_e5939_d_n6, assign8050_e5939_d_n7, assign8050_e5939_d_n10, assign8050_e5939_d_n11, assign8050_e5939_d_n12, assign8050_e5939_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8050_e5939;
        locals.var_dnm_dn0 = assign8050_e5939_d_n0;
        locals.var_dnm_dn2 = assign8050_e5939_d_n2;
        locals.var_dnm_dn6 = assign8050_e5939_d_n6;
        locals.var_dnm_dn7 = assign8050_e5939_d_n7;
        locals.var_dnm_dn10 = assign8050_e5939_d_n10;
        locals.var_dnm_dn11 = assign8050_e5939_d_n11;
        locals.var_dnm_dn12 = assign8050_e5939_d_n12;
        locals.var_dnm_dn17 = assign8050_e5939_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign8060_e5954: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard153 = assign8060_e5954;
        locals.var_guard153_rv = 0.0;

        let assign8070_e5957: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign8070_e5957;
        locals.var_guard154_rv = 0.0;

        let (assign8080_e5967,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8080_e5967;
        locals.var_mm_rv = 0.0;

        let assign8090_e5970: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign8090_e5970;
        locals.var_guard155_rv = 0.0;

        let (assign8100_e5983,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard155 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8100_e5983;
        locals.var_mm_rv = 0.0;

        let assign8110_e5986: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign8110_e5986;
        locals.var_guard156_rv = 0.0;

        let (assign8120_e6002,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8120_e6002;
        locals.var_mm_rv = 0.0;

        let assign8130_e6005: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign8130_e6005;
        locals.var_guard157_rv = 0.0;

        let (assign8140_e6024,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 == 0.0)) && (locals.var_guard157 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8140_e6024;
        locals.var_mm_rv = 0.0;

        let (assign8150_e6032,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign8150_e6032;
        locals.var_m0_rv = 0.0;

        let mut assign8160_loop_guard: usize = 0;
        while {
            let assign8160_cond_e6041: f64 = if ((((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign8160_cond_e6041 != 0.0
        } {
            assign8160_loop_guard += 1;
            assert!(assign8160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign8160_body0_e6050, assign8160_body0_e6050_d_n0, assign8160_body0_e6050_d_n2, assign8160_body0_e6050_d_n6, assign8160_body0_e6050_d_n7, assign8160_body0_e6050_d_n10, assign8160_body0_e6050_d_n11, assign8160_body0_e6050_d_n12, assign8160_body0_e6050_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) {
        let assign8160_body0_e6048: f64 = (locals.var_dnm).sqrt();
        (assign8160_body0_e6048, (locals.var_dnm_dn0 / (2.0 * assign8160_body0_e6048)), (locals.var_dnm_dn2 / (2.0 * assign8160_body0_e6048)), (locals.var_dnm_dn6 / (2.0 * assign8160_body0_e6048)), (locals.var_dnm_dn7 / (2.0 * assign8160_body0_e6048)), (locals.var_dnm_dn10 / (2.0 * assign8160_body0_e6048)), (locals.var_dnm_dn11 / (2.0 * assign8160_body0_e6048)), (locals.var_dnm_dn12 / (2.0 * assign8160_body0_e6048)), (locals.var_dnm_dn17 / (2.0 * assign8160_body0_e6048)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign8160_body0_e6050;
            locals.var_dnm_dn0 = assign8160_body0_e6050_d_n0;
            locals.var_dnm_dn2 = assign8160_body0_e6050_d_n2;
            locals.var_dnm_dn6 = assign8160_body0_e6050_d_n6;
            locals.var_dnm_dn7 = assign8160_body0_e6050_d_n7;
            locals.var_dnm_dn10 = assign8160_body0_e6050_d_n10;
            locals.var_dnm_dn11 = assign8160_body0_e6050_d_n11;
            locals.var_dnm_dn12 = assign8160_body0_e6050_d_n12;
            locals.var_dnm_dn17 = assign8160_body0_e6050_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign8160_body1_e6060,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 != 0.0)) {
        let assign8160_body1_e6058: f64 = (locals.var_m0 + 1.0);
        (assign8160_body1_e6058,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign8160_body1_e6060;
            locals.var_m0_rv = 0.0;
        }

        let (assign8170_e6075, assign8170_e6075_d_n0, assign8170_e6075_d_n2, assign8170_e6075_d_n6, assign8170_e6075_d_n7, assign8170_e6075_d_n10, assign8170_e6075_d_n11, assign8170_e6075_d_n12, assign8170_e6075_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) && (locals.var_guard153 == 0.0)) {
        let assign8170_e6071: f64 = (2.0 * 2.0);
        let assign8170_e6072: f64 = (1.0 / assign8170_e6071);
        let assign8170_e6073: f64 = (locals.var_dnm).powf(assign8170_e6072);
        (assign8170_e6073, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn0)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn2)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn6)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn7)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn10)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn11)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn12)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign8170_e6072) as f64).is_finite() && ((assign8170_e6072) as f64).fract() == 0.0 { if assign8170_e6072 == 0.0 { 0.0 } else { (assign8170_e6072 * ((locals.var_dnm).powf(assign8170_e6072 - 1.0) * locals.var_dnm_dn17)) } } else { (assign8170_e6073 * (assign8170_e6072 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8170_e6075;
        locals.var_dnm_dn0 = assign8170_e6075_d_n0;
        locals.var_dnm_dn2 = assign8170_e6075_d_n2;
        locals.var_dnm_dn6 = assign8170_e6075_d_n6;
        locals.var_dnm_dn7 = assign8170_e6075_d_n7;
        locals.var_dnm_dn10 = assign8170_e6075_d_n10;
        locals.var_dnm_dn11 = assign8170_e6075_d_n11;
        locals.var_dnm_dn12 = assign8170_e6075_d_n12;
        locals.var_dnm_dn17 = assign8170_e6075_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign8180_e6083, assign8180_e6083_d_n0, assign8180_e6083_d_n2, assign8180_e6083_d_n6, assign8180_e6083_d_n7, assign8180_e6083_d_n10, assign8180_e6083_d_n11, assign8180_e6083_d_n12, assign8180_e6083_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8180_e6081: f64 = (1.0 / locals.var_dnm);
        (assign8180_e6081, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8180_e6083;
        locals.var_dnm_dn0 = assign8180_e6083_d_n0;
        locals.var_dnm_dn2 = assign8180_e6083_d_n2;
        locals.var_dnm_dn6 = assign8180_e6083_d_n6;
        locals.var_dnm_dn7 = assign8180_e6083_d_n7;
        locals.var_dnm_dn10 = assign8180_e6083_d_n10;
        locals.var_dnm_dn11 = assign8180_e6083_d_n11;
        locals.var_dnm_dn12 = assign8180_e6083_d_n12;
        locals.var_dnm_dn17 = assign8180_e6083_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign8190_e6095, assign8190_e6095_d_n0, assign8190_e6095_d_n2, assign8190_e6095_d_n6, assign8190_e6095_d_n7, assign8190_e6095_d_n10, assign8190_e6095_d_n11, assign8190_e6095_d_n12, assign8190_e6095_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8190_e6090: f64 = locals.var_t_soi;
        let assign8190_e6091: f64 = (locals.var_tmf1 * assign8190_e6090);
        let assign8190_e6093: f64 = (assign8190_e6091 * locals.var_dnm);
        (assign8190_e6093, (((locals.var_tmf1_dn0 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * assign8190_e6090) * locals.var_dnm) + (assign8190_e6091 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign8190_e6095;
        locals.var_tmf0_dn0 = assign8190_e6095_d_n0;
        locals.var_tmf0_dn2 = assign8190_e6095_d_n2;
        locals.var_tmf0_dn6 = assign8190_e6095_d_n6;
        locals.var_tmf0_dn7 = assign8190_e6095_d_n7;
        locals.var_tmf0_dn10 = assign8190_e6095_d_n10;
        locals.var_tmf0_dn11 = assign8190_e6095_d_n11;
        locals.var_tmf0_dn12 = assign8190_e6095_d_n12;
        locals.var_tmf0_dn17 = assign8190_e6095_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign8200_e6107, assign8200_e6107_d_n0, assign8200_e6107_d_n2, assign8200_e6107_d_n6, assign8200_e6107_d_n7, assign8200_e6107_d_n10, assign8200_e6107_d_n11, assign8200_e6107_d_n12, assign8200_e6107_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign8200_e6102: f64 = locals.var_t_soi;
        let assign8200_e6103: f64 = (locals.var_wdsoi_ini0 - assign8200_e6102);
        let assign8200_e6105: f64 = (assign8200_e6103 + locals.var_tmf0);
        (assign8200_e6105, (locals.var_wdsoi_ini0_dn0 + locals.var_tmf0_dn0), (locals.var_wdsoi_ini0_dn2 + locals.var_tmf0_dn2), (locals.var_wdsoi_ini0_dn6 + locals.var_tmf0_dn6), (locals.var_wdsoi_ini0_dn7 + locals.var_tmf0_dn7), (locals.var_wdsoi_ini0_dn10 + locals.var_tmf0_dn10), (locals.var_wdsoi_ini0_dn11 + locals.var_tmf0_dn11), (locals.var_wdsoi_ini0_dn12 + locals.var_tmf0_dn12), (locals.var_wdsoi_ini0_dn17 + locals.var_tmf0_dn17),)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign8200_e6107;
        locals.var_wdsoi_ini2_dn0 = assign8200_e6107_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign8200_e6107_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign8200_e6107_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign8200_e6107_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign8200_e6107_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign8200_e6107_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign8200_e6107_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign8200_e6107_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let (assign8210_e6114, assign8210_e6114_d_n0, assign8210_e6114_d_n2, assign8210_e6114_d_n6, assign8210_e6114_d_n7, assign8210_e6114_d_n10, assign8210_e6114_d_n11, assign8210_e6114_d_n12, assign8210_e6114_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard152 == 0.0)) {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    } else {
        (locals.var_wdsoi_ini2, locals.var_wdsoi_ini2_dn0, locals.var_wdsoi_ini2_dn2, locals.var_wdsoi_ini2_dn6, locals.var_wdsoi_ini2_dn7, locals.var_wdsoi_ini2_dn10, locals.var_wdsoi_ini2_dn11, locals.var_wdsoi_ini2_dn12, locals.var_wdsoi_ini2_dn17,)
    }
};
        locals.var_wdsoi_ini2 = assign8210_e6114;
        locals.var_wdsoi_ini2_dn0 = assign8210_e6114_d_n0;
        locals.var_wdsoi_ini2_dn2 = assign8210_e6114_d_n2;
        locals.var_wdsoi_ini2_dn6 = assign8210_e6114_d_n6;
        locals.var_wdsoi_ini2_dn7 = assign8210_e6114_d_n7;
        locals.var_wdsoi_ini2_dn10 = assign8210_e6114_d_n10;
        locals.var_wdsoi_ini2_dn11 = assign8210_e6114_d_n11;
        locals.var_wdsoi_ini2_dn12 = assign8210_e6114_d_n12;
        locals.var_wdsoi_ini2_dn17 = assign8210_e6114_d_n17;
        locals.var_wdsoi_ini2_rv = 0.0;

        let (assign8220_e6121, assign8220_e6121_d_n0, assign8220_e6121_d_n2, assign8220_e6121_d_n6, assign8220_e6121_d_n7, assign8220_e6121_d_n10, assign8220_e6121_d_n11, assign8220_e6121_d_n12, assign8220_e6121_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign8220_e6117: f64 = (-locals.var_wdsoi_ini2);
        let assign8220_e6119: f64 = (assign8220_e6117 * locals.var_q_nsub);
        (assign8220_e6119, (((-locals.var_wdsoi_ini2_dn0) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn0)), (((-locals.var_wdsoi_ini2_dn2) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn2)), (((-locals.var_wdsoi_ini2_dn6) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn6)), (((-locals.var_wdsoi_ini2_dn7) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn7)), (((-locals.var_wdsoi_ini2_dn10) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn10)), (((-locals.var_wdsoi_ini2_dn11) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn11)), (((-locals.var_wdsoi_ini2_dn12) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn12)), (((-locals.var_wdsoi_ini2_dn17) * locals.var_q_nsub) + (assign8220_e6117 * locals.var_q_nsub_dn17)),)
    } else {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    }
};
        locals.var_q_s0_dep_ini = assign8220_e6121;
        locals.var_q_s0_dep_ini_dn0 = assign8220_e6121_d_n0;
        locals.var_q_s0_dep_ini_dn2 = assign8220_e6121_d_n2;
        locals.var_q_s0_dep_ini_dn6 = assign8220_e6121_d_n6;
        locals.var_q_s0_dep_ini_dn7 = assign8220_e6121_d_n7;
        locals.var_q_s0_dep_ini_dn10 = assign8220_e6121_d_n10;
        locals.var_q_s0_dep_ini_dn11 = assign8220_e6121_d_n11;
        locals.var_q_s0_dep_ini_dn12 = assign8220_e6121_d_n12;
        locals.var_q_s0_dep_ini_dn17 = assign8220_e6121_d_n17;
        locals.var_q_s0_dep_ini_rv = 0.0;

        let (assign8230_e6134, assign8230_e6134_d_n0, assign8230_e6134_d_n2, assign8230_e6134_d_n6, assign8230_e6134_d_n7, assign8230_e6134_d_n10, assign8230_e6134_d_n11, assign8230_e6134_d_n12, assign8230_e6134_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign8230_e6124: f64 = (-locals.var_q_fd_soi);
        let assign8230_e6126: f64 = (assign8230_e6124 * locals.var_t_soi);
        let assign8230_e6128: f64 = (assign8230_e6126 / 2.0);
        let assign8230_e6130: f64 = (assign8230_e6128 / 1.034943e-10);
        let assign8230_e6132: f64 = (assign8230_e6130 + locals.var_beta_inv);
        (assign8230_e6132, ((((-locals.var_q_fd_soi_dn0) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn2) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn6) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn7) * locals.var_t_soi) / 2.0) / 1.034943e-10), (((((-locals.var_q_fd_soi_dn10) * locals.var_t_soi) / 2.0) / 1.034943e-10) + locals.var_beta_inv_dn10), ((((-locals.var_q_fd_soi_dn11) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn12) * locals.var_t_soi) / 2.0) / 1.034943e-10), ((((-locals.var_q_fd_soi_dn17) * locals.var_t_soi) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_fd_start, locals.var_fd_start_dn0, locals.var_fd_start_dn2, locals.var_fd_start_dn6, locals.var_fd_start_dn7, locals.var_fd_start_dn10, locals.var_fd_start_dn11, locals.var_fd_start_dn12, locals.var_fd_start_dn17,)
    }
};
        locals.var_fd_start = assign8230_e6134;
        locals.var_fd_start_dn0 = assign8230_e6134_d_n0;
        locals.var_fd_start_dn2 = assign8230_e6134_d_n2;
        locals.var_fd_start_dn6 = assign8230_e6134_d_n6;
        locals.var_fd_start_dn7 = assign8230_e6134_d_n7;
        locals.var_fd_start_dn10 = assign8230_e6134_d_n10;
        locals.var_fd_start_dn11 = assign8230_e6134_d_n11;
        locals.var_fd_start_dn12 = assign8230_e6134_d_n12;
        locals.var_fd_start_dn17 = assign8230_e6134_d_n17;
        locals.var_fd_start_rv = 0.0;

        let (assign8240_e6144, assign8240_e6144_d_n0, assign8240_e6144_d_n2, assign8240_e6144_d_n6, assign8240_e6144_d_n7, assign8240_e6144_d_n10, assign8240_e6144_d_n11, assign8240_e6144_d_n12, assign8240_e6144_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign8240_e6139: f64 = (locals.var_q_s0_bulk_0 * locals.var_t_soi);
        let assign8240_e6141: f64 = (assign8240_e6139 / 1.034943e-10);
        let assign8240_e6142: f64 = (locals.var_fd_start - assign8240_e6141);
        (assign8240_e6142, (locals.var_fd_start_dn0 - ((locals.var_q_s0_bulk_0_dn0 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn2 - ((locals.var_q_s0_bulk_0_dn2 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn6 - ((locals.var_q_s0_bulk_0_dn6 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn7 - ((locals.var_q_s0_bulk_0_dn7 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn10 - ((locals.var_q_s0_bulk_0_dn10 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn11 - ((locals.var_q_s0_bulk_0_dn11 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn12 - ((locals.var_q_s0_bulk_0_dn12 * locals.var_t_soi) / 1.034943e-10)), (locals.var_fd_start_dn17 - ((locals.var_q_s0_bulk_0_dn17 * locals.var_t_soi) / 1.034943e-10)),)
    } else {
        (locals.var_fd_end, locals.var_fd_end_dn0, locals.var_fd_end_dn2, locals.var_fd_end_dn6, locals.var_fd_end_dn7, locals.var_fd_end_dn10, locals.var_fd_end_dn11, locals.var_fd_end_dn12, locals.var_fd_end_dn17,)
    }
};
        locals.var_fd_end = assign8240_e6144;
        locals.var_fd_end_dn0 = assign8240_e6144_d_n0;
        locals.var_fd_end_dn2 = assign8240_e6144_d_n2;
        locals.var_fd_end_dn6 = assign8240_e6144_d_n6;
        locals.var_fd_end_dn7 = assign8240_e6144_d_n7;
        locals.var_fd_end_dn10 = assign8240_e6144_d_n10;
        locals.var_fd_end_dn11 = assign8240_e6144_d_n11;
        locals.var_fd_end_dn12 = assign8240_e6144_d_n12;
        locals.var_fd_end_dn17 = assign8240_e6144_d_n17;
        locals.var_fd_end_rv = 0.0;

        let assign8250_e6147: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign8250_e6147;
        locals.var_guard158_rv = 0.0;

        let (assign8260_e6153, assign8260_e6153_d_n0, assign8260_e6153_d_n2, assign8260_e6153_d_n6, assign8260_e6153_d_n7, assign8260_e6153_d_n10, assign8260_e6153_d_n11, assign8260_e6153_d_n12, assign8260_e6153_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign8260_e6153;
        locals.var_phi_s0_soi_dn0 = assign8260_e6153_d_n0;
        locals.var_phi_s0_soi_dn2 = assign8260_e6153_d_n2;
        locals.var_phi_s0_soi_dn6 = assign8260_e6153_d_n6;
        locals.var_phi_s0_soi_dn7 = assign8260_e6153_d_n7;
        locals.var_phi_s0_soi_dn10 = assign8260_e6153_d_n10;
        locals.var_phi_s0_soi_dn11 = assign8260_e6153_d_n11;
        locals.var_phi_s0_soi_dn12 = assign8260_e6153_d_n12;
        locals.var_phi_s0_soi_dn17 = assign8260_e6153_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign8270_e6159, assign8270_e6159_d_n0, assign8270_e6159_d_n2, assign8270_e6159_d_n6, assign8270_e6159_d_n7, assign8270_e6159_d_n10, assign8270_e6159_d_n11, assign8270_e6159_d_n12, assign8270_e6159_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 != 0.0)) {
        (locals.var_pbs0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    }
};
        locals.var_phi_b0_soi = assign8270_e6159;
        locals.var_phi_b0_soi_dn0 = assign8270_e6159_d_n0;
        locals.var_phi_b0_soi_dn2 = assign8270_e6159_d_n2;
        locals.var_phi_b0_soi_dn6 = assign8270_e6159_d_n6;
        locals.var_phi_b0_soi_dn7 = assign8270_e6159_d_n7;
        locals.var_phi_b0_soi_dn10 = assign8270_e6159_d_n10;
        locals.var_phi_b0_soi_dn11 = assign8270_e6159_d_n11;
        locals.var_phi_b0_soi_dn12 = assign8270_e6159_d_n12;
        locals.var_phi_b0_soi_dn17 = assign8270_e6159_d_n17;
        locals.var_phi_b0_soi_rv = 0.0;

        let (assign8280_e6165, assign8280_e6165_d_n0, assign8280_e6165_d_n2, assign8280_e6165_d_n6, assign8280_e6165_d_n7, assign8280_e6165_d_n10, assign8280_e6165_d_n11, assign8280_e6165_d_n12, assign8280_e6165_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 != 0.0)) {
        (locals.var_psb0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign8280_e6165;
        locals.var_phi_s0_bulk_dn0 = assign8280_e6165_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign8280_e6165_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign8280_e6165_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign8280_e6165_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign8280_e6165_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign8280_e6165_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign8280_e6165_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign8280_e6165_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign8290_e6176,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 != 0.0)) {
        let (assign8290_e6174,) = {
            if (locals.var_phi_s0_soi < locals.var_fd_end) {
                (1.0,)
            } else {
                (2.0,)
            }
        };
        (assign8290_e6174,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign8290_e6176;
        locals.var_flg_depmode_rv = 0.0;

        let (assign8300_e6195, assign8300_e6195_d_n0, assign8300_e6195_d_n2, assign8300_e6195_d_n6, assign8300_e6195_d_n7, assign8300_e6195_d_n10, assign8300_e6195_d_n11, assign8300_e6195_d_n12, assign8300_e6195_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign8300_e6185: f64 = (locals.var_beta * locals.var_vgpz);
        let assign8300_e6187: f64 = (assign8300_e6185 - 1.0);
        let assign8300_e6188: f64 = (4.0 * assign8300_e6187);
        let assign8300_e6191: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign8300_e6192: f64 = (assign8300_e6188 / assign8300_e6191);
        let assign8300_e6193: f64 = (1.0 + assign8300_e6192);
        (assign8300_e6193, ((((4.0 * (locals.var_beta * locals.var_vgpz_dn0)) * assign8300_e6191) - (assign8300_e6188 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign8300_e6191 * assign8300_e6191)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn2)) * assign8300_e6191) - (assign8300_e6188 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign8300_e6191 * assign8300_e6191)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn6)) * assign8300_e6191) - (assign8300_e6188 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign8300_e6191 * assign8300_e6191)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn7)) * assign8300_e6191) - (assign8300_e6188 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign8300_e6191 * assign8300_e6191)), ((((4.0 * ((locals.var_beta_dn10 * locals.var_vgpz) + (locals.var_beta * locals.var_vgpz_dn10))) * assign8300_e6191) - (assign8300_e6188 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign8300_e6191 * assign8300_e6191)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn11)) * assign8300_e6191) - (assign8300_e6188 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign8300_e6191 * assign8300_e6191)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn12)) * assign8300_e6191) - (assign8300_e6188 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign8300_e6191 * assign8300_e6191)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn17)) * assign8300_e6191) - (assign8300_e6188 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign8300_e6191 * assign8300_e6191)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign8300_e6195;
        locals.var_tx_dn0 = assign8300_e6195_d_n0;
        locals.var_tx_dn2 = assign8300_e6195_d_n2;
        locals.var_tx_dn6 = assign8300_e6195_d_n6;
        locals.var_tx_dn7 = assign8300_e6195_d_n7;
        locals.var_tx_dn10 = assign8300_e6195_d_n10;
        locals.var_tx_dn11 = assign8300_e6195_d_n11;
        locals.var_tx_dn12 = assign8300_e6195_d_n12;
        locals.var_tx_dn17 = assign8300_e6195_d_n17;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign8310_e6211, assign8310_e6211_d_n0, assign8310_e6211_d_n2, assign8310_e6211_d_n6, assign8310_e6211_d_n7, assign8310_e6211_d_n10, assign8310_e6211_d_n11, assign8310_e6211_d_n12, assign8310_e6211_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign8310_e6203: f64 = (10.0 * 2.220446049250313e-16);
        let (assign8310_e6209, assign8310_e6209_d_n0, assign8310_e6209_d_n2, assign8310_e6209_d_n6, assign8310_e6209_d_n7, assign8310_e6209_d_n10, assign8310_e6209_d_n11, assign8310_e6209_d_n12, assign8310_e6209_d_n17,) = {
            if (locals.var_tx >= assign8310_e6203) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign8310_e6208: f64 = (10.0 * 2.220446049250313e-16);
                (assign8310_e6208, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign8310_e6209, assign8310_e6209_d_n0, assign8310_e6209_d_n2, assign8310_e6209_d_n6, assign8310_e6209_d_n7, assign8310_e6209_d_n10, assign8310_e6209_d_n11, assign8310_e6209_d_n12, assign8310_e6209_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign8310_e6211;
        locals.var_tx_dn0 = assign8310_e6211_d_n0;
        locals.var_tx_dn2 = assign8310_e6211_d_n2;
        locals.var_tx_dn6 = assign8310_e6211_d_n6;
        locals.var_tx_dn7 = assign8310_e6211_d_n7;
        locals.var_tx_dn10 = assign8310_e6211_d_n10;
        locals.var_tx_dn11 = assign8310_e6211_d_n11;
        locals.var_tx_dn12 = assign8310_e6211_d_n12;
        locals.var_tx_dn17 = assign8310_e6211_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign8320_e6229, assign8320_e6229_d_n0, assign8320_e6229_d_n2, assign8320_e6229_d_n6, assign8320_e6229_d_n7, assign8320_e6229_d_n10, assign8320_e6229_d_n11, assign8320_e6229_d_n12, assign8320_e6229_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign8320_e6219: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign8320_e6221: f64 = (assign8320_e6219 * 0.5);
        let assign8320_e6224: f64 = (locals.var_tx).sqrt();
        let assign8320_e6225: f64 = (1.0 - assign8320_e6224);
        let assign8320_e6226: f64 = (assign8320_e6221 * assign8320_e6225);
        let assign8320_e6227: f64 = (locals.var_vgpz + assign8320_e6226);
        (assign8320_e6227, (locals.var_vgpz_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn0 / (2.0 * assign8320_e6224)))))), (locals.var_vgpz_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn2 / (2.0 * assign8320_e6224)))))), (locals.var_vgpz_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn6 / (2.0 * assign8320_e6224)))))), (locals.var_vgpz_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn7 / (2.0 * assign8320_e6224)))))), (locals.var_vgpz_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn10 / (2.0 * assign8320_e6224)))))), (locals.var_vgpz_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn11 / (2.0 * assign8320_e6224)))))), (locals.var_vgpz_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn12 / (2.0 * assign8320_e6224)))))), (locals.var_vgpz_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign8320_e6225) + (assign8320_e6221 * (-(locals.var_tx_dn17 / (2.0 * assign8320_e6224)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8320_e6229;
        locals.var_ps0_inia_dn0 = assign8320_e6229_d_n0;
        locals.var_ps0_inia_dn2 = assign8320_e6229_d_n2;
        locals.var_ps0_inia_dn6 = assign8320_e6229_d_n6;
        locals.var_ps0_inia_dn7 = assign8320_e6229_d_n7;
        locals.var_ps0_inia_dn10 = assign8320_e6229_d_n10;
        locals.var_ps0_inia_dn11 = assign8320_e6229_d_n11;
        locals.var_ps0_inia_dn12 = assign8320_e6229_d_n12;
        locals.var_ps0_inia_dn17 = assign8320_e6229_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8330_e6238, assign8330_e6238_d_n0, assign8330_e6238_d_n2, assign8330_e6238_d_n6, assign8330_e6238_d_n7, assign8330_e6238_d_n10, assign8330_e6238_d_n11, assign8330_e6238_d_n12, assign8330_e6238_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign8330_e6236: f64 = (locals.var_beta * locals.var_ps0_inia);
        (assign8330_e6236, (locals.var_beta * locals.var_ps0_inia_dn0), (locals.var_beta * locals.var_ps0_inia_dn2), (locals.var_beta * locals.var_ps0_inia_dn6), (locals.var_beta * locals.var_ps0_inia_dn7), ((locals.var_beta_dn10 * locals.var_ps0_inia) + (locals.var_beta * locals.var_ps0_inia_dn10)), (locals.var_beta * locals.var_ps0_inia_dn11), (locals.var_beta * locals.var_ps0_inia_dn12), (locals.var_beta * locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign8330_e6238;
        locals.var_chi_dn0 = assign8330_e6238_d_n0;
        locals.var_chi_dn2 = assign8330_e6238_d_n2;
        locals.var_chi_dn6 = assign8330_e6238_d_n6;
        locals.var_chi_dn7 = assign8330_e6238_d_n7;
        locals.var_chi_dn10 = assign8330_e6238_d_n10;
        locals.var_chi_dn11 = assign8330_e6238_d_n11;
        locals.var_chi_dn12 = assign8330_e6238_d_n12;
        locals.var_chi_dn17 = assign8330_e6238_d_n17;
        locals.var_chi_rv = 0.0;

        let assign8340_e6241: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign8340_e6241;
        locals.var_guard159_rv = 0.0;

        let (assign8350_e6254, assign8350_e6254_d_n0, assign8350_e6254_d_n2, assign8350_e6254_d_n6, assign8350_e6254_d_n7, assign8350_e6254_d_n10, assign8350_e6254_d_n11, assign8350_e6254_d_n12, assign8350_e6254_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8350_e6251: f64 = (locals.var_vgpz - locals.var_vbs);
        let assign8350_e6252: f64 = (locals.var_beta * assign8350_e6251);
        (assign8350_e6252, (locals.var_beta * (locals.var_vgpz_dn0 - locals.var_vbs_dn0)), (locals.var_beta * (locals.var_vgpz_dn2 - locals.var_vbs_dn2)), (locals.var_beta * (locals.var_vgpz_dn6 - locals.var_vbs_dn6)), (locals.var_beta * (locals.var_vgpz_dn7 - locals.var_vbs_dn7)), ((locals.var_beta_dn10 * assign8350_e6251) + (locals.var_beta * (locals.var_vgpz_dn10 - locals.var_vbs_dn10))), (locals.var_beta * (locals.var_vgpz_dn11 - locals.var_vbs_dn11)), (locals.var_beta * (locals.var_vgpz_dn12 - locals.var_vbs_dn12)), (locals.var_beta * (locals.var_vgpz_dn17 - locals.var_vbs_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign8350_e6254;
        locals.var_ty_dn0 = assign8350_e6254_d_n0;
        locals.var_ty_dn2 = assign8350_e6254_d_n2;
        locals.var_ty_dn6 = assign8350_e6254_d_n6;
        locals.var_ty_dn7 = assign8350_e6254_d_n7;
        locals.var_ty_dn10 = assign8350_e6254_d_n10;
        locals.var_ty_dn11 = assign8350_e6254_d_n11;
        locals.var_ty_dn12 = assign8350_e6254_d_n12;
        locals.var_ty_dn17 = assign8350_e6254_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign8360_e6271, assign8360_e6271_d_n0, assign8360_e6271_d_n2, assign8360_e6271_d_n6, assign8360_e6271_d_n7, assign8360_e6271_d_n10, assign8360_e6271_d_n11, assign8360_e6271_d_n12, assign8360_e6271_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8360_e6264: f64 = (1.414213562373095 / 108.0);
        let assign8360_e6266: f64 = (assign8360_e6264 * locals.var_beta);
        let assign8360_e6268: f64 = (assign8360_e6266 * locals.var_fac1);
        let assign8360_e6269: f64 = (1.0 / assign8360_e6268);
        (assign8360_e6269, (-((assign8360_e6266 * locals.var_fac1_dn0) / (assign8360_e6268 * assign8360_e6268))), (-((assign8360_e6266 * locals.var_fac1_dn2) / (assign8360_e6268 * assign8360_e6268))), (-((assign8360_e6266 * locals.var_fac1_dn6) / (assign8360_e6268 * assign8360_e6268))), (-((assign8360_e6266 * locals.var_fac1_dn7) / (assign8360_e6268 * assign8360_e6268))), (-((((assign8360_e6264 * locals.var_beta_dn10) * locals.var_fac1) + (assign8360_e6266 * locals.var_fac1_dn10)) / (assign8360_e6268 * assign8360_e6268))), (-((assign8360_e6266 * locals.var_fac1_dn11) / (assign8360_e6268 * assign8360_e6268))), (-((assign8360_e6266 * locals.var_fac1_dn12) / (assign8360_e6268 * assign8360_e6268))), (-((assign8360_e6266 * locals.var_fac1_dn17) / (assign8360_e6268 * assign8360_e6268))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8360_e6271;
        locals.var_t1_dn0 = assign8360_e6271_d_n0;
        locals.var_t1_dn2 = assign8360_e6271_d_n2;
        locals.var_t1_dn6 = assign8360_e6271_d_n6;
        locals.var_t1_dn7 = assign8360_e6271_d_n7;
        locals.var_t1_dn10 = assign8360_e6271_d_n10;
        locals.var_t1_dn11 = assign8360_e6271_d_n11;
        locals.var_t1_dn12 = assign8360_e6271_d_n12;
        locals.var_t1_dn17 = assign8360_e6271_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8370_e6284, assign8370_e6284_d_n0, assign8370_e6284_d_n2, assign8370_e6284_d_n6, assign8370_e6284_d_n7, assign8370_e6284_d_n10, assign8370_e6284_d_n11, assign8370_e6284_d_n12, assign8370_e6284_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8370_e6281: f64 = (3.0 * locals.var_t1);
        let assign8370_e6282: f64 = (81.0 + assign8370_e6281);
        (assign8370_e6282, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8370_e6284;
        locals.var_t2_dn0 = assign8370_e6284_d_n0;
        locals.var_t2_dn2 = assign8370_e6284_d_n2;
        locals.var_t2_dn6 = assign8370_e6284_d_n6;
        locals.var_t2_dn7 = assign8370_e6284_d_n7;
        locals.var_t2_dn10 = assign8370_e6284_d_n10;
        locals.var_t2_dn11 = assign8370_e6284_d_n11;
        locals.var_t2_dn12 = assign8370_e6284_d_n12;
        locals.var_t2_dn17 = assign8370_e6284_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8380_e6304, assign8380_e6304_d_n0, assign8380_e6304_d_n2, assign8380_e6304_d_n6, assign8380_e6304_d_n7, assign8380_e6304_d_n10, assign8380_e6304_d_n11, assign8380_e6304_d_n12, assign8380_e6304_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8380_e6292: f64 = (-2916.0);
        let assign8380_e6295: f64 = (81.0 * locals.var_t1);
        let assign8380_e6296: f64 = (assign8380_e6292 - assign8380_e6295);
        let assign8380_e6299: f64 = (27.0 * locals.var_t1);
        let assign8380_e6301: f64 = (assign8380_e6299 * locals.var_ty);
        let assign8380_e6302: f64 = (assign8380_e6296 + assign8380_e6301);
        (assign8380_e6302, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign8380_e6299 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8380_e6304;
        locals.var_t3_dn0 = assign8380_e6304_d_n0;
        locals.var_t3_dn2 = assign8380_e6304_d_n2;
        locals.var_t3_dn6 = assign8380_e6304_d_n6;
        locals.var_t3_dn7 = assign8380_e6304_d_n7;
        locals.var_t3_dn10 = assign8380_e6304_d_n10;
        locals.var_t3_dn11 = assign8380_e6304_d_n11;
        locals.var_t3_dn12 = assign8380_e6304_d_n12;
        locals.var_t3_dn17 = assign8380_e6304_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8390_e6325, assign8390_e6325_d_n0, assign8390_e6325_d_n2, assign8390_e6325_d_n6, assign8390_e6325_d_n7, assign8390_e6325_d_n10, assign8390_e6325_d_n11, assign8390_e6325_d_n12, assign8390_e6325_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8390_e6315: f64 = (54.0 + locals.var_t1);
        let assign8390_e6316: f64 = (81.0 * assign8390_e6315);
        let assign8390_e6317: f64 = (1458.0 - assign8390_e6316);
        let assign8390_e6320: f64 = (27.0 * locals.var_t1);
        let assign8390_e6322: f64 = (assign8390_e6320 * locals.var_ty);
        let assign8390_e6323: f64 = (assign8390_e6317 + assign8390_e6322);
        (assign8390_e6323, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign8390_e6320 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8390_e6325;
        locals.var_t4_dn0 = assign8390_e6325_d_n0;
        locals.var_t4_dn2 = assign8390_e6325_d_n2;
        locals.var_t4_dn6 = assign8390_e6325_d_n6;
        locals.var_t4_dn7 = assign8390_e6325_d_n7;
        locals.var_t4_dn10 = assign8390_e6325_d_n10;
        locals.var_t4_dn11 = assign8390_e6325_d_n11;
        locals.var_t4_dn12 = assign8390_e6325_d_n12;
        locals.var_t4_dn17 = assign8390_e6325_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8400_e6336, assign8400_e6336_d_n0, assign8400_e6336_d_n2, assign8400_e6336_d_n6, assign8400_e6336_d_n7, assign8400_e6336_d_n10, assign8400_e6336_d_n11, assign8400_e6336_d_n12, assign8400_e6336_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8400_e6334: f64 = (locals.var_t4 * locals.var_t4);
        (assign8400_e6334, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8400_e6336;
        locals.var_t4_dn0 = assign8400_e6336_d_n0;
        locals.var_t4_dn2 = assign8400_e6336_d_n2;
        locals.var_t4_dn6 = assign8400_e6336_d_n6;
        locals.var_t4_dn7 = assign8400_e6336_d_n7;
        locals.var_t4_dn10 = assign8400_e6336_d_n10;
        locals.var_t4_dn11 = assign8400_e6336_d_n11;
        locals.var_t4_dn12 = assign8400_e6336_d_n12;
        locals.var_t4_dn17 = assign8400_e6336_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8410_e6358, assign8410_e6358_d_n0, assign8410_e6358_d_n2, assign8410_e6358_d_n6, assign8410_e6358_d_n7, assign8410_e6358_d_n10, assign8410_e6358_d_n11, assign8410_e6358_d_n12, assign8410_e6358_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8410_e6346: f64 = (4.0 * locals.var_t2);
        let assign8410_e6348: f64 = (assign8410_e6346 * locals.var_t2);
        let assign8410_e6350: f64 = (assign8410_e6348 * locals.var_t2);
        let assign8410_e6352: f64 = (assign8410_e6350 + locals.var_t4);
        let assign8410_e6353: f64 = (assign8410_e6352).sqrt();
        let assign8410_e6354: f64 = (locals.var_t3 + assign8410_e6353);
        let assign8410_e6356: f64 = (assign8410_e6354).powf(0.3333333333333333);
        (assign8410_e6356, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn0)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn0)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign8410_e6353))) / assign8410_e6354))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn2)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn2)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign8410_e6353))) / assign8410_e6354))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn6)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn6)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign8410_e6353))) / assign8410_e6354))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn7)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn7)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign8410_e6353))) / assign8410_e6354))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn10)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn10)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign8410_e6353))) / assign8410_e6354))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn11)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn11)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign8410_e6353))) / assign8410_e6354))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn12)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn12)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign8410_e6353))) / assign8410_e6354))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign8410_e6354).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn17)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign8410_e6353))))) } } else { (assign8410_e6356 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign8410_e6346 * locals.var_t2_dn17)) * locals.var_t2) + (assign8410_e6348 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign8410_e6353))) / assign8410_e6354))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign8410_e6358;
        locals.var_t5_dn0 = assign8410_e6358_d_n0;
        locals.var_t5_dn2 = assign8410_e6358_d_n2;
        locals.var_t5_dn6 = assign8410_e6358_d_n6;
        locals.var_t5_dn7 = assign8410_e6358_d_n7;
        locals.var_t5_dn10 = assign8410_e6358_d_n10;
        locals.var_t5_dn11 = assign8410_e6358_d_n11;
        locals.var_t5_dn12 = assign8410_e6358_d_n12;
        locals.var_t5_dn17 = assign8410_e6358_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign8420_e6383, assign8420_e6383_d_n0, assign8420_e6383_d_n2, assign8420_e6383_d_n6, assign8420_e6383_d_n7, assign8420_e6383_d_n10, assign8420_e6383_d_n11, assign8420_e6383_d_n12, assign8420_e6383_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8420_e6368: f64 = (1.259921049894873 * locals.var_t2);
        let assign8420_e6371: f64 = (3.0 * locals.var_t5);
        let assign8420_e6372: f64 = (assign8420_e6368 / assign8420_e6371);
        let assign8420_e6373: f64 = (3.0 - assign8420_e6372);
        let assign8420_e6377: f64 = (3.0 * 1.259921049894873);
        let assign8420_e6378: f64 = (1.0 / assign8420_e6377);
        let assign8420_e6380: f64 = (assign8420_e6378 * locals.var_t5);
        let assign8420_e6381: f64 = (assign8420_e6373 + assign8420_e6380);
        (assign8420_e6381, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn0))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn2))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn6))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn7))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn10))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn11))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn12))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign8420_e6371) - (assign8420_e6368 * (3.0 * locals.var_t5_dn17))) / (assign8420_e6371 * assign8420_e6371))) + (assign8420_e6378 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign8420_e6383;
        locals.var_tx_dn0 = assign8420_e6383_d_n0;
        locals.var_tx_dn2 = assign8420_e6383_d_n2;
        locals.var_tx_dn6 = assign8420_e6383_d_n6;
        locals.var_tx_dn7 = assign8420_e6383_d_n7;
        locals.var_tx_dn10 = assign8420_e6383_d_n10;
        locals.var_tx_dn11 = assign8420_e6383_d_n11;
        locals.var_tx_dn12 = assign8420_e6383_d_n12;
        locals.var_tx_dn17 = assign8420_e6383_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign8430_e6396, assign8430_e6396_d_n0, assign8430_e6396_d_n2, assign8430_e6396_d_n6, assign8430_e6396_d_n7, assign8430_e6396_d_n10, assign8430_e6396_d_n11, assign8430_e6396_d_n12, assign8430_e6396_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        let assign8430_e6392: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign8430_e6394: f64 = (assign8430_e6392 + locals.var_vbs);
        (assign8430_e6394, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbs_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbs_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbs_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbs_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbs_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbs_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbs_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbs_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8430_e6396;
        locals.var_ps0_inia_dn0 = assign8430_e6396_d_n0;
        locals.var_ps0_inia_dn2 = assign8430_e6396_d_n2;
        locals.var_ps0_inia_dn6 = assign8430_e6396_d_n6;
        locals.var_ps0_inia_dn7 = assign8430_e6396_d_n7;
        locals.var_ps0_inia_dn10 = assign8430_e6396_d_n10;
        locals.var_ps0_inia_dn11 = assign8430_e6396_d_n11;
        locals.var_ps0_inia_dn12 = assign8430_e6396_d_n12;
        locals.var_ps0_inia_dn17 = assign8430_e6396_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8440_e6405, assign8440_e6405_d_n0, assign8440_e6405_d_n2, assign8440_e6405_d_n6, assign8440_e6405_d_n7, assign8440_e6405_d_n10, assign8440_e6405_d_n11, assign8440_e6405_d_n12, assign8440_e6405_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8440_e6405;
        locals.var_ps0_ini_dn0 = assign8440_e6405_d_n0;
        locals.var_ps0_ini_dn2 = assign8440_e6405_d_n2;
        locals.var_ps0_ini_dn6 = assign8440_e6405_d_n6;
        locals.var_ps0_ini_dn7 = assign8440_e6405_d_n7;
        locals.var_ps0_ini_dn10 = assign8440_e6405_d_n10;
        locals.var_ps0_ini_dn11 = assign8440_e6405_d_n11;
        locals.var_ps0_ini_dn12 = assign8440_e6405_d_n12;
        locals.var_ps0_ini_dn17 = assign8440_e6405_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign8450_e6408: f64 = (locals.var_vgs - locals.var_shift);
        let assign8450_e6410: f64 = if assign8450_e6408 <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard160 = assign8450_e6410;
        locals.var_guard160_rv = 0.0;

        let (assign8460_e6424, assign8460_e6424_d_n0, assign8460_e6424_d_n2, assign8460_e6424_d_n6, assign8460_e6424_d_n7, assign8460_e6424_d_n10, assign8460_e6424_d_n11, assign8460_e6424_d_n12, assign8460_e6424_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign8460_e6422: f64 = (1.0 / locals.var_c_fox);
        (assign8460_e6422, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign8460_e6424;
        locals.var_t0_dn0 = assign8460_e6424_d_n0;
        locals.var_t0_dn2 = assign8460_e6424_d_n2;
        locals.var_t0_dn6 = assign8460_e6424_d_n6;
        locals.var_t0_dn7 = assign8460_e6424_d_n7;
        locals.var_t0_dn10 = assign8460_e6424_d_n10;
        locals.var_t0_dn11 = assign8460_e6424_d_n11;
        locals.var_t0_dn12 = assign8460_e6424_d_n12;
        locals.var_t0_dn17 = assign8460_e6424_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign8470_e6438, assign8470_e6438_d_n0, assign8470_e6438_d_n2, assign8470_e6438_d_n6, assign8470_e6438_d_n7, assign8470_e6438_d_n10, assign8470_e6438_d_n11, assign8470_e6438_d_n12, assign8470_e6438_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign8470_e6436: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign8470_e6436, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8470_e6438;
        locals.var_t1_dn0 = assign8470_e6438_d_n0;
        locals.var_t1_dn2 = assign8470_e6438_d_n2;
        locals.var_t1_dn6 = assign8470_e6438_d_n6;
        locals.var_t1_dn7 = assign8470_e6438_d_n7;
        locals.var_t1_dn10 = assign8470_e6438_d_n10;
        locals.var_t1_dn11 = assign8470_e6438_d_n11;
        locals.var_t1_dn12 = assign8470_e6438_d_n12;
        locals.var_t1_dn17 = assign8470_e6438_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8480_e6452, assign8480_e6452_d_n0, assign8480_e6452_d_n2, assign8480_e6452_d_n6, assign8480_e6452_d_n7, assign8480_e6452_d_n10, assign8480_e6452_d_n11, assign8480_e6452_d_n12, assign8480_e6452_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign8480_e6450: f64 = (1.0 / locals.var_c_box);
        (assign8480_e6450, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8480_e6452;
        locals.var_t2_dn0 = assign8480_e6452_d_n0;
        locals.var_t2_dn2 = assign8480_e6452_d_n2;
        locals.var_t2_dn6 = assign8480_e6452_d_n6;
        locals.var_t2_dn7 = assign8480_e6452_d_n7;
        locals.var_t2_dn10 = assign8480_e6452_d_n10;
        locals.var_t2_dn11 = assign8480_e6452_d_n11;
        locals.var_t2_dn12 = assign8480_e6452_d_n12;
        locals.var_t2_dn17 = assign8480_e6452_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8490_e6470, assign8490_e6470_d_n0, assign8490_e6470_d_n2, assign8490_e6470_d_n6, assign8490_e6470_d_n7, assign8490_e6470_d_n10, assign8490_e6470_d_n11, assign8490_e6470_d_n12, assign8490_e6470_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign8490_e6465: f64 = (locals.var_t0 + locals.var_t1);
        let assign8490_e6467: f64 = (assign8490_e6465 + locals.var_t2);
        let assign8490_e6468: f64 = (1.0 / assign8490_e6467);
        (assign8490_e6468, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign8490_e6467 * assign8490_e6467))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign8490_e6467 * assign8490_e6467))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign8490_e6467 * assign8490_e6467))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign8490_e6467 * assign8490_e6467))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign8490_e6467 * assign8490_e6467))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign8490_e6467 * assign8490_e6467))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign8490_e6467 * assign8490_e6467))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign8490_e6467 * assign8490_e6467))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8490_e6470;
        locals.var_t3_dn0 = assign8490_e6470_d_n0;
        locals.var_t3_dn2 = assign8490_e6470_d_n2;
        locals.var_t3_dn6 = assign8490_e6470_d_n6;
        locals.var_t3_dn7 = assign8490_e6470_d_n7;
        locals.var_t3_dn10 = assign8490_e6470_d_n10;
        locals.var_t3_dn11 = assign8490_e6470_d_n11;
        locals.var_t3_dn12 = assign8490_e6470_d_n12;
        locals.var_t3_dn17 = assign8490_e6470_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8500_e6495, assign8500_e6495_d_n0, assign8500_e6495_d_n2, assign8500_e6495_d_n6, assign8500_e6495_d_n7, assign8500_e6495_d_n10, assign8500_e6495_d_n11, assign8500_e6495_d_n12, assign8500_e6495_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign8500_e6483: f64 = (locals.var_vgpz - locals.var_vbsbiz);
        let assign8500_e6487: f64 = (0.5 * locals.var_t1);
        let assign8500_e6488: f64 = (locals.var_t2 + assign8500_e6487);
        let assign8500_e6490: f64 = (-locals.var_q_s0_dep_ini);
        let assign8500_e6491: f64 = (assign8500_e6488 * assign8500_e6490);
        let assign8500_e6492: f64 = (assign8500_e6483 + assign8500_e6491);
        let assign8500_e6493: f64 = (locals.var_t3 * assign8500_e6492);
        (assign8500_e6493, ((locals.var_t3_dn0 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign8500_e6492) + (locals.var_t3 * ((locals.var_vgpz_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign8500_e6490) + (assign8500_e6488 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8500_e6495;
        locals.var_t4_dn0 = assign8500_e6495_d_n0;
        locals.var_t4_dn2 = assign8500_e6495_d_n2;
        locals.var_t4_dn6 = assign8500_e6495_d_n6;
        locals.var_t4_dn7 = assign8500_e6495_d_n7;
        locals.var_t4_dn10 = assign8500_e6495_d_n10;
        locals.var_t4_dn11 = assign8500_e6495_d_n11;
        locals.var_t4_dn12 = assign8500_e6495_d_n12;
        locals.var_t4_dn17 = assign8500_e6495_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8510_e6511, assign8510_e6511_d_n0, assign8510_e6511_d_n2, assign8510_e6511_d_n6, assign8510_e6511_d_n7, assign8510_e6511_d_n10, assign8510_e6511_d_n11, assign8510_e6511_d_n12, assign8510_e6511_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        let assign8510_e6508: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign8510_e6509: f64 = (locals.var_vgpz - assign8510_e6508);
        (assign8510_e6509, (locals.var_vgpz_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8510_e6511;
        locals.var_ps0_inia_dn0 = assign8510_e6511_d_n0;
        locals.var_ps0_inia_dn2 = assign8510_e6511_d_n2;
        locals.var_ps0_inia_dn6 = assign8510_e6511_d_n6;
        locals.var_ps0_inia_dn7 = assign8510_e6511_d_n7;
        locals.var_ps0_inia_dn10 = assign8510_e6511_d_n10;
        locals.var_ps0_inia_dn11 = assign8510_e6511_d_n11;
        locals.var_ps0_inia_dn12 = assign8510_e6511_d_n12;
        locals.var_ps0_inia_dn17 = assign8510_e6511_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8520_e6523, assign8520_e6523_d_n0, assign8520_e6523_d_n2, assign8520_e6523_d_n6, assign8520_e6523_d_n7, assign8520_e6523_d_n10, assign8520_e6523_d_n11, assign8520_e6523_d_n12, assign8520_e6523_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8520_e6523;
        locals.var_ps0_ini_dn0 = assign8520_e6523_d_n0;
        locals.var_ps0_ini_dn2 = assign8520_e6523_d_n2;
        locals.var_ps0_ini_dn6 = assign8520_e6523_d_n6;
        locals.var_ps0_ini_dn7 = assign8520_e6523_d_n7;
        locals.var_ps0_ini_dn10 = assign8520_e6523_d_n10;
        locals.var_ps0_ini_dn11 = assign8520_e6523_d_n11;
        locals.var_ps0_ini_dn12 = assign8520_e6523_d_n12;
        locals.var_ps0_ini_dn17 = assign8520_e6523_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign8530_e6540, assign8530_e6540_d_n0, assign8530_e6540_d_n2, assign8530_e6540_d_n6, assign8530_e6540_d_n7, assign8530_e6540_d_n10, assign8530_e6540_d_n11, assign8530_e6540_d_n12, assign8530_e6540_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8530_e6536: f64 = (1.0 / locals.var_cnst1soi);
        let assign8530_e6538: f64 = (assign8530_e6536 / locals.var_cnstc_foxi);
        (assign8530_e6538, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8530_e6536 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8530_e6540;
        locals.var_t1_dn0 = assign8530_e6540_d_n0;
        locals.var_t1_dn2 = assign8530_e6540_d_n2;
        locals.var_t1_dn6 = assign8530_e6540_d_n6;
        locals.var_t1_dn7 = assign8530_e6540_d_n7;
        locals.var_t1_dn10 = assign8530_e6540_d_n10;
        locals.var_t1_dn11 = assign8530_e6540_d_n11;
        locals.var_t1_dn12 = assign8530_e6540_d_n12;
        locals.var_t1_dn17 = assign8530_e6540_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8540_e6561, assign8540_e6561_d_n0, assign8540_e6561_d_n2, assign8540_e6561_d_n6, assign8540_e6561_d_n7, assign8540_e6561_d_n10, assign8540_e6561_d_n11, assign8540_e6561_d_n12, assign8540_e6561_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8540_e6554: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8540_e6555: f64 = (locals.var_t1 * assign8540_e6554);
        let assign8540_e6558: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8540_e6559: f64 = (assign8540_e6555 * assign8540_e6558);
        (assign8540_e6559, ((((locals.var_t1_dn0 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign8540_e6554) + (locals.var_t1 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))) * assign8540_e6558) + (assign8540_e6555 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8540_e6561;
        locals.var_t2_dn0 = assign8540_e6561_d_n0;
        locals.var_t2_dn2 = assign8540_e6561_d_n2;
        locals.var_t2_dn6 = assign8540_e6561_d_n6;
        locals.var_t2_dn7 = assign8540_e6561_d_n7;
        locals.var_t2_dn10 = assign8540_e6561_d_n10;
        locals.var_t2_dn11 = assign8540_e6561_d_n11;
        locals.var_t2_dn12 = assign8540_e6561_d_n12;
        locals.var_t2_dn17 = assign8540_e6561_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8550_e6580, assign8550_e6580_d_n0, assign8550_e6580_d_n2, assign8550_e6580_d_n6, assign8550_e6580_d_n7, assign8550_e6580_d_n10, assign8550_e6580_d_n11, assign8550_e6580_d_n12, assign8550_e6580_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8550_e6576: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8550_e6577: f64 = (2.0 / assign8550_e6576);
        let assign8550_e6578: f64 = (locals.var_beta + assign8550_e6577);
        (assign8550_e6578, (-((2.0 * (locals.var_vgpz_dn0 - locals.var_shift_dn0)) / (assign8550_e6576 * assign8550_e6576))), (-((2.0 * (locals.var_vgpz_dn2 - locals.var_shift_dn2)) / (assign8550_e6576 * assign8550_e6576))), (-((2.0 * (locals.var_vgpz_dn6 - locals.var_shift_dn6)) / (assign8550_e6576 * assign8550_e6576))), (-((2.0 * (locals.var_vgpz_dn7 - locals.var_shift_dn7)) / (assign8550_e6576 * assign8550_e6576))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgpz_dn10 - locals.var_shift_dn10)) / (assign8550_e6576 * assign8550_e6576)))), (-((2.0 * (locals.var_vgpz_dn11 - locals.var_shift_dn11)) / (assign8550_e6576 * assign8550_e6576))), (-((2.0 * (locals.var_vgpz_dn12 - locals.var_shift_dn12)) / (assign8550_e6576 * assign8550_e6576))), (-((2.0 * (locals.var_vgpz_dn17 - locals.var_shift_dn17)) / (assign8550_e6576 * assign8550_e6576))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8550_e6580;
        locals.var_t3_dn0 = assign8550_e6580_d_n0;
        locals.var_t3_dn2 = assign8550_e6580_d_n2;
        locals.var_t3_dn6 = assign8550_e6580_d_n6;
        locals.var_t3_dn7 = assign8550_e6580_d_n7;
        locals.var_t3_dn10 = assign8550_e6580_d_n10;
        locals.var_t3_dn11 = assign8550_e6580_d_n11;
        locals.var_t3_dn12 = assign8550_e6580_d_n12;
        locals.var_t3_dn17 = assign8550_e6580_d_n17;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign8560_e6596, assign8560_e6596_d_n0, assign8560_e6596_d_n2, assign8560_e6596_d_n6, assign8560_e6596_d_n7, assign8560_e6596_d_n10, assign8560_e6596_d_n11, assign8560_e6596_d_n12, assign8560_e6596_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8560_e6592: f64 = (locals.var_t2).ln();
        let assign8560_e6594: f64 = (assign8560_e6592 / locals.var_t3);
        (assign8560_e6594, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign8560_e6592 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign8560_e6596;
        locals.var_ps0_inib_dn0 = assign8560_e6596_d_n0;
        locals.var_ps0_inib_dn2 = assign8560_e6596_d_n2;
        locals.var_ps0_inib_dn6 = assign8560_e6596_d_n6;
        locals.var_ps0_inib_dn7 = assign8560_e6596_d_n7;
        locals.var_ps0_inib_dn10 = assign8560_e6596_d_n10;
        locals.var_ps0_inib_dn11 = assign8560_e6596_d_n11;
        locals.var_ps0_inib_dn12 = assign8560_e6596_d_n12;
        locals.var_ps0_inib_dn17 = assign8560_e6596_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign8570_e6613, assign8570_e6613_d_n0, assign8570_e6613_d_n2, assign8570_e6613_d_n6, assign8570_e6613_d_n7, assign8570_e6613_d_n10, assign8570_e6613_d_n11, assign8570_e6613_d_n12, assign8570_e6613_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8570_e6609: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign8570_e6611: f64 = (assign8570_e6609 - 0.0008);
        (assign8570_e6611, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign8570_e6613;
        locals.var_tmf1_dn0 = assign8570_e6613_d_n0;
        locals.var_tmf1_dn2 = assign8570_e6613_d_n2;
        locals.var_tmf1_dn6 = assign8570_e6613_d_n6;
        locals.var_tmf1_dn7 = assign8570_e6613_d_n7;
        locals.var_tmf1_dn10 = assign8570_e6613_d_n10;
        locals.var_tmf1_dn11 = assign8570_e6613_d_n11;
        locals.var_tmf1_dn12 = assign8570_e6613_d_n12;
        locals.var_tmf1_dn17 = assign8570_e6613_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign8580_e6630, assign8580_e6630_d_n0, assign8580_e6630_d_n2, assign8580_e6630_d_n6, assign8580_e6630_d_n7, assign8580_e6630_d_n10, assign8580_e6630_d_n11, assign8580_e6630_d_n12, assign8580_e6630_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8580_e6626: f64 = (4.0 * locals.var_ps0_inib);
        let assign8580_e6628: f64 = (assign8580_e6626 * 0.0008);
        (assign8580_e6628, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign8580_e6630;
        locals.var_tmf2_dn0 = assign8580_e6630_d_n0;
        locals.var_tmf2_dn2 = assign8580_e6630_d_n2;
        locals.var_tmf2_dn6 = assign8580_e6630_d_n6;
        locals.var_tmf2_dn7 = assign8580_e6630_d_n7;
        locals.var_tmf2_dn10 = assign8580_e6630_d_n10;
        locals.var_tmf2_dn11 = assign8580_e6630_d_n11;
        locals.var_tmf2_dn12 = assign8580_e6630_d_n12;
        locals.var_tmf2_dn17 = assign8580_e6630_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign8590_e6649, assign8590_e6649_d_n0, assign8590_e6649_d_n2, assign8590_e6649_d_n6, assign8590_e6649_d_n7, assign8590_e6649_d_n10, assign8590_e6649_d_n11, assign8590_e6649_d_n12, assign8590_e6649_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let (assign8590_e6647, assign8590_e6647_d_n0, assign8590_e6647_d_n2, assign8590_e6647_d_n6, assign8590_e6647_d_n7, assign8590_e6647_d_n10, assign8590_e6647_d_n11, assign8590_e6647_d_n12, assign8590_e6647_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign8590_e6646: f64 = (-locals.var_tmf2);
                (assign8590_e6646, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign8590_e6647, assign8590_e6647_d_n0, assign8590_e6647_d_n2, assign8590_e6647_d_n6, assign8590_e6647_d_n7, assign8590_e6647_d_n10, assign8590_e6647_d_n11, assign8590_e6647_d_n12, assign8590_e6647_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign8590_e6649;
        locals.var_tmf2_dn0 = assign8590_e6649_d_n0;
        locals.var_tmf2_dn2 = assign8590_e6649_d_n2;
        locals.var_tmf2_dn6 = assign8590_e6649_d_n6;
        locals.var_tmf2_dn7 = assign8590_e6649_d_n7;
        locals.var_tmf2_dn10 = assign8590_e6649_d_n10;
        locals.var_tmf2_dn11 = assign8590_e6649_d_n11;
        locals.var_tmf2_dn12 = assign8590_e6649_d_n12;
        locals.var_tmf2_dn17 = assign8590_e6649_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign8600_e6667, assign8600_e6667_d_n0, assign8600_e6667_d_n2, assign8600_e6667_d_n6, assign8600_e6667_d_n7, assign8600_e6667_d_n10, assign8600_e6667_d_n11, assign8600_e6667_d_n12, assign8600_e6667_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8600_e6662: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign8600_e6664: f64 = (assign8600_e6662 + locals.var_tmf2);
        let assign8600_e6665: f64 = (assign8600_e6664).sqrt();
        (assign8600_e6665, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign8600_e6665)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign8600_e6665)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign8600_e6665)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign8600_e6665)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign8600_e6665)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign8600_e6665)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign8600_e6665)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign8600_e6665)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign8600_e6667;
        locals.var_tmf2_dn0 = assign8600_e6667_d_n0;
        locals.var_tmf2_dn2 = assign8600_e6667_d_n2;
        locals.var_tmf2_dn6 = assign8600_e6667_d_n6;
        locals.var_tmf2_dn7 = assign8600_e6667_d_n7;
        locals.var_tmf2_dn10 = assign8600_e6667_d_n10;
        locals.var_tmf2_dn11 = assign8600_e6667_d_n11;
        locals.var_tmf2_dn12 = assign8600_e6667_d_n12;
        locals.var_tmf2_dn17 = assign8600_e6667_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign8610_e6686, assign8610_e6686_d_n0, assign8610_e6686_d_n2, assign8610_e6686_d_n6, assign8610_e6686_d_n7, assign8610_e6686_d_n10, assign8610_e6686_d_n11, assign8610_e6686_d_n12, assign8610_e6686_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) && (locals.var_guard160 == 0.0)) {
        let assign8610_e6682: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign8610_e6683: f64 = (0.5 * assign8610_e6682);
        let assign8610_e6684: f64 = (locals.var_ps0_inib - assign8610_e6683);
        (assign8610_e6684, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8610_e6686;
        locals.var_ps0_ini_dn0 = assign8610_e6686_d_n0;
        locals.var_ps0_ini_dn2 = assign8610_e6686_d_n2;
        locals.var_ps0_ini_dn6 = assign8610_e6686_d_n6;
        locals.var_ps0_ini_dn7 = assign8610_e6686_d_n7;
        locals.var_ps0_ini_dn10 = assign8610_e6686_d_n10;
        locals.var_ps0_ini_dn11 = assign8610_e6686_d_n11;
        locals.var_ps0_ini_dn12 = assign8610_e6686_d_n12;
        locals.var_ps0_ini_dn17 = assign8610_e6686_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign8620_e6707, assign8620_e6707_d_n0, assign8620_e6707_d_n2, assign8620_e6707_d_n6, assign8620_e6707_d_n7, assign8620_e6707_d_n10, assign8620_e6707_d_n11, assign8620_e6707_d_n12, assign8620_e6707_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let (assign8620_e6705, assign8620_e6705_d_n0, assign8620_e6705_d_n2, assign8620_e6705_d_n6, assign8620_e6705_d_n7, assign8620_e6705_d_n10, assign8620_e6705_d_n11, assign8620_e6705_d_n12, assign8620_e6705_d_n17,) = {
            if (locals.var_ps0_ini > 0.0) {
                let assign8620_e6696: f64 = (2.0 * 1.034943e-10);
                let assign8620_e6698: f64 = (assign8620_e6696 / 1.6021918e-19);
                let assign8620_e6700: f64 = (assign8620_e6698 * locals.var_ps0_ini);
                let assign8620_e6702: f64 = (assign8620_e6700 / locals.var_uc_nsubs);
                let assign8620_e6703: f64 = (assign8620_e6702).sqrt();
                (assign8620_e6703, (((((assign8620_e6698 * locals.var_ps0_ini_dn0) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)), (((((assign8620_e6698 * locals.var_ps0_ini_dn2) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)), (((((assign8620_e6698 * locals.var_ps0_ini_dn6) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)), (((((assign8620_e6698 * locals.var_ps0_ini_dn7) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)), (((((assign8620_e6698 * locals.var_ps0_ini_dn10) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)), (((((assign8620_e6698 * locals.var_ps0_ini_dn11) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)), (((((assign8620_e6698 * locals.var_ps0_ini_dn12) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)), (((((assign8620_e6698 * locals.var_ps0_ini_dn17) * locals.var_uc_nsubs) - (assign8620_e6700 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign8620_e6703)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign8620_e6705, assign8620_e6705_d_n0, assign8620_e6705_d_n2, assign8620_e6705_d_n6, assign8620_e6705_d_n7, assign8620_e6705_d_n10, assign8620_e6705_d_n11, assign8620_e6705_d_n12, assign8620_e6705_d_n17,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign8620_e6707;
        locals.var_wdsoi_dn0 = assign8620_e6707_d_n0;
        locals.var_wdsoi_dn2 = assign8620_e6707_d_n2;
        locals.var_wdsoi_dn6 = assign8620_e6707_d_n6;
        locals.var_wdsoi_dn7 = assign8620_e6707_d_n7;
        locals.var_wdsoi_dn10 = assign8620_e6707_d_n10;
        locals.var_wdsoi_dn11 = assign8620_e6707_d_n11;
        locals.var_wdsoi_dn12 = assign8620_e6707_d_n12;
        locals.var_wdsoi_dn17 = assign8620_e6707_d_n17;
        locals.var_wdsoi_rv = 0.0;

        let assign8630_e6710: f64 = if locals.var_wdsoi < locals.var_t_soi { 1.0 } else { 0.0 };
        locals.var_guard161 = assign8630_e6710;
        locals.var_guard161_rv = 0.0;

        let (assign8640_e6719,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard161 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign8640_e6719;
        locals.var_flg_depmode_rv = 0.0;

        let (assign8650_e6729,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard161 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign8650_e6729;
        locals.var_flg_depmode_rv = 0.0;

        let assign8660_e6732: f64 = (locals.var_vgs - locals.var_shift);
        let assign8660_e6734: f64 = if assign8660_e6732 <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard162 = assign8660_e6734;
        locals.var_guard162_rv = 0.0;

        let (assign8670_e6745, assign8670_e6745_d_n0, assign8670_e6745_d_n2, assign8670_e6745_d_n6, assign8670_e6745_d_n7, assign8670_e6745_d_n10, assign8670_e6745_d_n11, assign8670_e6745_d_n12, assign8670_e6745_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 != 0.0)) {
        let assign8670_e6743: f64 = (1.0 / locals.var_c_fox);
        (assign8670_e6743, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign8670_e6745;
        locals.var_t0_dn0 = assign8670_e6745_d_n0;
        locals.var_t0_dn2 = assign8670_e6745_d_n2;
        locals.var_t0_dn6 = assign8670_e6745_d_n6;
        locals.var_t0_dn7 = assign8670_e6745_d_n7;
        locals.var_t0_dn10 = assign8670_e6745_d_n10;
        locals.var_t0_dn11 = assign8670_e6745_d_n11;
        locals.var_t0_dn12 = assign8670_e6745_d_n12;
        locals.var_t0_dn17 = assign8670_e6745_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign8680_e6756, assign8680_e6756_d_n0, assign8680_e6756_d_n2, assign8680_e6756_d_n6, assign8680_e6756_d_n7, assign8680_e6756_d_n10, assign8680_e6756_d_n11, assign8680_e6756_d_n12, assign8680_e6756_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 != 0.0)) {
        let assign8680_e6754: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign8680_e6754, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8680_e6756;
        locals.var_t1_dn0 = assign8680_e6756_d_n0;
        locals.var_t1_dn2 = assign8680_e6756_d_n2;
        locals.var_t1_dn6 = assign8680_e6756_d_n6;
        locals.var_t1_dn7 = assign8680_e6756_d_n7;
        locals.var_t1_dn10 = assign8680_e6756_d_n10;
        locals.var_t1_dn11 = assign8680_e6756_d_n11;
        locals.var_t1_dn12 = assign8680_e6756_d_n12;
        locals.var_t1_dn17 = assign8680_e6756_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8690_e6767, assign8690_e6767_d_n0, assign8690_e6767_d_n2, assign8690_e6767_d_n6, assign8690_e6767_d_n7, assign8690_e6767_d_n10, assign8690_e6767_d_n11, assign8690_e6767_d_n12, assign8690_e6767_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 != 0.0)) {
        let assign8690_e6765: f64 = (1.0 / locals.var_c_box);
        (assign8690_e6765, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8690_e6767;
        locals.var_t2_dn0 = assign8690_e6767_d_n0;
        locals.var_t2_dn2 = assign8690_e6767_d_n2;
        locals.var_t2_dn6 = assign8690_e6767_d_n6;
        locals.var_t2_dn7 = assign8690_e6767_d_n7;
        locals.var_t2_dn10 = assign8690_e6767_d_n10;
        locals.var_t2_dn11 = assign8690_e6767_d_n11;
        locals.var_t2_dn12 = assign8690_e6767_d_n12;
        locals.var_t2_dn17 = assign8690_e6767_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8700_e6782, assign8700_e6782_d_n0, assign8700_e6782_d_n2, assign8700_e6782_d_n6, assign8700_e6782_d_n7, assign8700_e6782_d_n10, assign8700_e6782_d_n11, assign8700_e6782_d_n12, assign8700_e6782_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 != 0.0)) {
        let assign8700_e6777: f64 = (locals.var_t0 + locals.var_t1);
        let assign8700_e6779: f64 = (assign8700_e6777 + locals.var_t2);
        let assign8700_e6780: f64 = (1.0 / assign8700_e6779);
        (assign8700_e6780, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign8700_e6779 * assign8700_e6779))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign8700_e6779 * assign8700_e6779))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign8700_e6779 * assign8700_e6779))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign8700_e6779 * assign8700_e6779))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign8700_e6779 * assign8700_e6779))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign8700_e6779 * assign8700_e6779))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign8700_e6779 * assign8700_e6779))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign8700_e6779 * assign8700_e6779))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8700_e6782;
        locals.var_t3_dn0 = assign8700_e6782_d_n0;
        locals.var_t3_dn2 = assign8700_e6782_d_n2;
        locals.var_t3_dn6 = assign8700_e6782_d_n6;
        locals.var_t3_dn7 = assign8700_e6782_d_n7;
        locals.var_t3_dn10 = assign8700_e6782_d_n10;
        locals.var_t3_dn11 = assign8700_e6782_d_n11;
        locals.var_t3_dn12 = assign8700_e6782_d_n12;
        locals.var_t3_dn17 = assign8700_e6782_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8710_e6804, assign8710_e6804_d_n0, assign8710_e6804_d_n2, assign8710_e6804_d_n6, assign8710_e6804_d_n7, assign8710_e6804_d_n10, assign8710_e6804_d_n11, assign8710_e6804_d_n12, assign8710_e6804_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 != 0.0)) {
        let assign8710_e6792: f64 = (locals.var_vgpz - locals.var_vbsbiz);
        let assign8710_e6796: f64 = (0.5 * locals.var_t1);
        let assign8710_e6797: f64 = (locals.var_t2 + assign8710_e6796);
        let assign8710_e6799: f64 = (-locals.var_q_s0_dep_ini);
        let assign8710_e6800: f64 = (assign8710_e6797 * assign8710_e6799);
        let assign8710_e6801: f64 = (assign8710_e6792 + assign8710_e6800);
        let assign8710_e6802: f64 = (locals.var_t3 * assign8710_e6801);
        (assign8710_e6802, ((locals.var_t3_dn0 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign8710_e6801) + (locals.var_t3 * ((locals.var_vgpz_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign8710_e6799) + (assign8710_e6797 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8710_e6804;
        locals.var_t4_dn0 = assign8710_e6804_d_n0;
        locals.var_t4_dn2 = assign8710_e6804_d_n2;
        locals.var_t4_dn6 = assign8710_e6804_d_n6;
        locals.var_t4_dn7 = assign8710_e6804_d_n7;
        locals.var_t4_dn10 = assign8710_e6804_d_n10;
        locals.var_t4_dn11 = assign8710_e6804_d_n11;
        locals.var_t4_dn12 = assign8710_e6804_d_n12;
        locals.var_t4_dn17 = assign8710_e6804_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8720_e6817, assign8720_e6817_d_n0, assign8720_e6817_d_n2, assign8720_e6817_d_n6, assign8720_e6817_d_n7, assign8720_e6817_d_n10, assign8720_e6817_d_n11, assign8720_e6817_d_n12, assign8720_e6817_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 != 0.0)) {
        let assign8720_e6814: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign8720_e6815: f64 = (locals.var_vgpz - assign8720_e6814);
        (assign8720_e6815, (locals.var_vgpz_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8720_e6817;
        locals.var_ps0_inia_dn0 = assign8720_e6817_d_n0;
        locals.var_ps0_inia_dn2 = assign8720_e6817_d_n2;
        locals.var_ps0_inia_dn6 = assign8720_e6817_d_n6;
        locals.var_ps0_inia_dn7 = assign8720_e6817_d_n7;
        locals.var_ps0_inia_dn10 = assign8720_e6817_d_n10;
        locals.var_ps0_inia_dn11 = assign8720_e6817_d_n11;
        locals.var_ps0_inia_dn12 = assign8720_e6817_d_n12;
        locals.var_ps0_inia_dn17 = assign8720_e6817_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8730_e6826, assign8730_e6826_d_n0, assign8730_e6826_d_n2, assign8730_e6826_d_n6, assign8730_e6826_d_n7, assign8730_e6826_d_n10, assign8730_e6826_d_n11, assign8730_e6826_d_n12, assign8730_e6826_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8730_e6826;
        locals.var_ps0_ini_dn0 = assign8730_e6826_d_n0;
        locals.var_ps0_ini_dn2 = assign8730_e6826_d_n2;
        locals.var_ps0_ini_dn6 = assign8730_e6826_d_n6;
        locals.var_ps0_ini_dn7 = assign8730_e6826_d_n7;
        locals.var_ps0_ini_dn10 = assign8730_e6826_d_n10;
        locals.var_ps0_ini_dn11 = assign8730_e6826_d_n11;
        locals.var_ps0_ini_dn12 = assign8730_e6826_d_n12;
        locals.var_ps0_ini_dn17 = assign8730_e6826_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign8740_e6838, assign8740_e6838_d_n0, assign8740_e6838_d_n2, assign8740_e6838_d_n6, assign8740_e6838_d_n7, assign8740_e6838_d_n10, assign8740_e6838_d_n11, assign8740_e6838_d_n12, assign8740_e6838_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign8740_e6836: f64 = (1.0 / locals.var_c_fox);
        (assign8740_e6836, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign8740_e6838;
        locals.var_t0_dn0 = assign8740_e6838_d_n0;
        locals.var_t0_dn2 = assign8740_e6838_d_n2;
        locals.var_t0_dn6 = assign8740_e6838_d_n6;
        locals.var_t0_dn7 = assign8740_e6838_d_n7;
        locals.var_t0_dn10 = assign8740_e6838_d_n10;
        locals.var_t0_dn11 = assign8740_e6838_d_n11;
        locals.var_t0_dn12 = assign8740_e6838_d_n12;
        locals.var_t0_dn17 = assign8740_e6838_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign8750_e6850, assign8750_e6850_d_n0, assign8750_e6850_d_n2, assign8750_e6850_d_n6, assign8750_e6850_d_n7, assign8750_e6850_d_n10, assign8750_e6850_d_n11, assign8750_e6850_d_n12, assign8750_e6850_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign8750_e6848: f64 = (locals.var_t_soi / 1.034943e-10);
        (assign8750_e6848, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8750_e6850;
        locals.var_t1_dn0 = assign8750_e6850_d_n0;
        locals.var_t1_dn2 = assign8750_e6850_d_n2;
        locals.var_t1_dn6 = assign8750_e6850_d_n6;
        locals.var_t1_dn7 = assign8750_e6850_d_n7;
        locals.var_t1_dn10 = assign8750_e6850_d_n10;
        locals.var_t1_dn11 = assign8750_e6850_d_n11;
        locals.var_t1_dn12 = assign8750_e6850_d_n12;
        locals.var_t1_dn17 = assign8750_e6850_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign8760_e6862, assign8760_e6862_d_n0, assign8760_e6862_d_n2, assign8760_e6862_d_n6, assign8760_e6862_d_n7, assign8760_e6862_d_n10, assign8760_e6862_d_n11, assign8760_e6862_d_n12, assign8760_e6862_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign8760_e6860: f64 = (1.0 / locals.var_c_box);
        (assign8760_e6860, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8760_e6862;
        locals.var_t2_dn0 = assign8760_e6862_d_n0;
        locals.var_t2_dn2 = assign8760_e6862_d_n2;
        locals.var_t2_dn6 = assign8760_e6862_d_n6;
        locals.var_t2_dn7 = assign8760_e6862_d_n7;
        locals.var_t2_dn10 = assign8760_e6862_d_n10;
        locals.var_t2_dn11 = assign8760_e6862_d_n11;
        locals.var_t2_dn12 = assign8760_e6862_d_n12;
        locals.var_t2_dn17 = assign8760_e6862_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8770_e6878, assign8770_e6878_d_n0, assign8770_e6878_d_n2, assign8770_e6878_d_n6, assign8770_e6878_d_n7, assign8770_e6878_d_n10, assign8770_e6878_d_n11, assign8770_e6878_d_n12, assign8770_e6878_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign8770_e6873: f64 = (locals.var_t0 + locals.var_t1);
        let assign8770_e6875: f64 = (assign8770_e6873 + locals.var_t2);
        let assign8770_e6876: f64 = (1.0 / assign8770_e6875);
        (assign8770_e6876, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign8770_e6875 * assign8770_e6875))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign8770_e6875 * assign8770_e6875))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign8770_e6875 * assign8770_e6875))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign8770_e6875 * assign8770_e6875))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign8770_e6875 * assign8770_e6875))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign8770_e6875 * assign8770_e6875))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign8770_e6875 * assign8770_e6875))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign8770_e6875 * assign8770_e6875))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8770_e6878;
        locals.var_t3_dn0 = assign8770_e6878_d_n0;
        locals.var_t3_dn2 = assign8770_e6878_d_n2;
        locals.var_t3_dn6 = assign8770_e6878_d_n6;
        locals.var_t3_dn7 = assign8770_e6878_d_n7;
        locals.var_t3_dn10 = assign8770_e6878_d_n10;
        locals.var_t3_dn11 = assign8770_e6878_d_n11;
        locals.var_t3_dn12 = assign8770_e6878_d_n12;
        locals.var_t3_dn17 = assign8770_e6878_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8780_e6901, assign8780_e6901_d_n0, assign8780_e6901_d_n2, assign8780_e6901_d_n6, assign8780_e6901_d_n7, assign8780_e6901_d_n10, assign8780_e6901_d_n11, assign8780_e6901_d_n12, assign8780_e6901_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign8780_e6889: f64 = (locals.var_vgpz - locals.var_vbsbiz);
        let assign8780_e6893: f64 = (0.5 * locals.var_t1);
        let assign8780_e6894: f64 = (locals.var_t2 + assign8780_e6893);
        let assign8780_e6896: f64 = (-locals.var_q_s0_dep_ini);
        let assign8780_e6897: f64 = (assign8780_e6894 * assign8780_e6896);
        let assign8780_e6898: f64 = (assign8780_e6889 + assign8780_e6897);
        let assign8780_e6899: f64 = (locals.var_t3 * assign8780_e6898);
        (assign8780_e6899, ((locals.var_t3_dn0 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign8780_e6898) + (locals.var_t3 * ((locals.var_vgpz_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign8780_e6896) + (assign8780_e6894 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign8780_e6901;
        locals.var_t4_dn0 = assign8780_e6901_d_n0;
        locals.var_t4_dn2 = assign8780_e6901_d_n2;
        locals.var_t4_dn6 = assign8780_e6901_d_n6;
        locals.var_t4_dn7 = assign8780_e6901_d_n7;
        locals.var_t4_dn10 = assign8780_e6901_d_n10;
        locals.var_t4_dn11 = assign8780_e6901_d_n11;
        locals.var_t4_dn12 = assign8780_e6901_d_n12;
        locals.var_t4_dn17 = assign8780_e6901_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign8790_e6915, assign8790_e6915_d_n0, assign8790_e6915_d_n2, assign8790_e6915_d_n6, assign8790_e6915_d_n7, assign8790_e6915_d_n10, assign8790_e6915_d_n11, assign8790_e6915_d_n12, assign8790_e6915_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign8790_e6912: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign8790_e6913: f64 = (locals.var_vgpz - assign8790_e6912);
        (assign8790_e6913, (locals.var_vgpz_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgpz_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign8790_e6915;
        locals.var_ps0_inia_dn0 = assign8790_e6915_d_n0;
        locals.var_ps0_inia_dn2 = assign8790_e6915_d_n2;
        locals.var_ps0_inia_dn6 = assign8790_e6915_d_n6;
        locals.var_ps0_inia_dn7 = assign8790_e6915_d_n7;
        locals.var_ps0_inia_dn10 = assign8790_e6915_d_n10;
        locals.var_ps0_inia_dn11 = assign8790_e6915_d_n11;
        locals.var_ps0_inia_dn12 = assign8790_e6915_d_n12;
        locals.var_ps0_inia_dn17 = assign8790_e6915_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign8800_e6925, assign8800_e6925_d_n0, assign8800_e6925_d_n2, assign8800_e6925_d_n6, assign8800_e6925_d_n7, assign8800_e6925_d_n10, assign8800_e6925_d_n11, assign8800_e6925_d_n12, assign8800_e6925_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign8800_e6925;
        locals.var_ps0_ini_dn0 = assign8800_e6925_d_n0;
        locals.var_ps0_ini_dn2 = assign8800_e6925_d_n2;
        locals.var_ps0_ini_dn6 = assign8800_e6925_d_n6;
        locals.var_ps0_ini_dn7 = assign8800_e6925_d_n7;
        locals.var_ps0_ini_dn10 = assign8800_e6925_d_n10;
        locals.var_ps0_ini_dn11 = assign8800_e6925_d_n11;
        locals.var_ps0_ini_dn12 = assign8800_e6925_d_n12;
        locals.var_ps0_ini_dn17 = assign8800_e6925_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign8810_e6928: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8810_e6930: f64 = if assign8810_e6928 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign8810_e6930;
        locals.var_guard163_rv = 0.0;

        let (assign8820_e6946, assign8820_e6946_d_n0, assign8820_e6946_d_n2, assign8820_e6946_d_n6, assign8820_e6946_d_n7, assign8820_e6946_d_n10, assign8820_e6946_d_n11, assign8820_e6946_d_n12, assign8820_e6946_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8820_e6942: f64 = (1.0 / locals.var_cnst1soi);
        let assign8820_e6944: f64 = (assign8820_e6942 / locals.var_cnstc_foxi);
        (assign8820_e6944, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign8820_e6942 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign8820_e6946;
        locals.var_t1_dn0 = assign8820_e6946_d_n0;
        locals.var_t1_dn2 = assign8820_e6946_d_n2;
        locals.var_t1_dn6 = assign8820_e6946_d_n6;
        locals.var_t1_dn7 = assign8820_e6946_d_n7;
        locals.var_t1_dn10 = assign8820_e6946_d_n10;
        locals.var_t1_dn11 = assign8820_e6946_d_n11;
        locals.var_t1_dn12 = assign8820_e6946_d_n12;
        locals.var_t1_dn17 = assign8820_e6946_d_n17;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign8830_e6966, assign8830_e6966_d_n0, assign8830_e6966_d_n2, assign8830_e6966_d_n6, assign8830_e6966_d_n7, assign8830_e6966_d_n10, assign8830_e6966_d_n11, assign8830_e6966_d_n12, assign8830_e6966_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8830_e6959: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8830_e6960: f64 = (locals.var_t1 * assign8830_e6959);
        let assign8830_e6963: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8830_e6964: f64 = (assign8830_e6960 * assign8830_e6963);
        (assign8830_e6964, ((((locals.var_t1_dn0 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign8830_e6959) + (locals.var_t1 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))) * assign8830_e6963) + (assign8830_e6960 * (locals.var_vgpz_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign8830_e6966;
        locals.var_t2_dn0 = assign8830_e6966_d_n0;
        locals.var_t2_dn2 = assign8830_e6966_d_n2;
        locals.var_t2_dn6 = assign8830_e6966_d_n6;
        locals.var_t2_dn7 = assign8830_e6966_d_n7;
        locals.var_t2_dn10 = assign8830_e6966_d_n10;
        locals.var_t2_dn11 = assign8830_e6966_d_n11;
        locals.var_t2_dn12 = assign8830_e6966_d_n12;
        locals.var_t2_dn17 = assign8830_e6966_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign8840_e6984, assign8840_e6984_d_n0, assign8840_e6984_d_n2, assign8840_e6984_d_n6, assign8840_e6984_d_n7, assign8840_e6984_d_n10, assign8840_e6984_d_n11, assign8840_e6984_d_n12, assign8840_e6984_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8840_e6980: f64 = (locals.var_vgpz - locals.var_shift);
        let assign8840_e6981: f64 = (2.0 / assign8840_e6980);
        let assign8840_e6982: f64 = (locals.var_beta + assign8840_e6981);
        (assign8840_e6982, (-((2.0 * (locals.var_vgpz_dn0 - locals.var_shift_dn0)) / (assign8840_e6980 * assign8840_e6980))), (-((2.0 * (locals.var_vgpz_dn2 - locals.var_shift_dn2)) / (assign8840_e6980 * assign8840_e6980))), (-((2.0 * (locals.var_vgpz_dn6 - locals.var_shift_dn6)) / (assign8840_e6980 * assign8840_e6980))), (-((2.0 * (locals.var_vgpz_dn7 - locals.var_shift_dn7)) / (assign8840_e6980 * assign8840_e6980))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgpz_dn10 - locals.var_shift_dn10)) / (assign8840_e6980 * assign8840_e6980)))), (-((2.0 * (locals.var_vgpz_dn11 - locals.var_shift_dn11)) / (assign8840_e6980 * assign8840_e6980))), (-((2.0 * (locals.var_vgpz_dn12 - locals.var_shift_dn12)) / (assign8840_e6980 * assign8840_e6980))), (-((2.0 * (locals.var_vgpz_dn17 - locals.var_shift_dn17)) / (assign8840_e6980 * assign8840_e6980))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign8840_e6984;
        locals.var_t3_dn0 = assign8840_e6984_d_n0;
        locals.var_t3_dn2 = assign8840_e6984_d_n2;
        locals.var_t3_dn6 = assign8840_e6984_d_n6;
        locals.var_t3_dn7 = assign8840_e6984_d_n7;
        locals.var_t3_dn10 = assign8840_e6984_d_n10;
        locals.var_t3_dn11 = assign8840_e6984_d_n11;
        locals.var_t3_dn12 = assign8840_e6984_d_n12;
        locals.var_t3_dn17 = assign8840_e6984_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign8850_e6999, assign8850_e6999_d_n0, assign8850_e6999_d_n2, assign8850_e6999_d_n6, assign8850_e6999_d_n7, assign8850_e6999_d_n10, assign8850_e6999_d_n11, assign8850_e6999_d_n12, assign8850_e6999_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) {
        let assign8850_e6995: f64 = (locals.var_t2).ln();
        let assign8850_e6997: f64 = (assign8850_e6995 / locals.var_t3);
        (assign8850_e6997, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign8850_e6995 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign8850_e6999;
        locals.var_ps0_inib_dn0 = assign8850_e6999_d_n0;
        locals.var_ps0_inib_dn2 = assign8850_e6999_d_n2;
        locals.var_ps0_inib_dn6 = assign8850_e6999_d_n6;
        locals.var_ps0_inib_dn7 = assign8850_e6999_d_n7;
        locals.var_ps0_inib_dn10 = assign8850_e6999_d_n10;
        locals.var_ps0_inib_dn11 = assign8850_e6999_d_n11;
        locals.var_ps0_inib_dn12 = assign8850_e6999_d_n12;
        locals.var_ps0_inib_dn17 = assign8850_e6999_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let assign8860_e7003: f64 = (locals.var_ps0_inib * 0.98);
        let assign8860_e7005: f64 = (assign8860_e7003 - 0.4);
        let assign8860_e7010: f64 = if ((locals.var_ps0_inia > assign8860_e7005) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard164 = assign8860_e7010;
        locals.var_guard164_rv = 0.0;

        let (assign8870_e7030, assign8870_e7030_d_n0, assign8870_e7030_d_n2, assign8870_e7030_d_n6, assign8870_e7030_d_n7, assign8870_e7030_d_n10, assign8870_e7030_d_n11, assign8870_e7030_d_n12, assign8870_e7030_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8870_e7025: f64 = (locals.var_ps0_inib * 0.98);
        let assign8870_e7026: f64 = (locals.var_ps0_inia - assign8870_e7025);
        let assign8870_e7028: f64 = (assign8870_e7026 + 0.4);
        (assign8870_e7028, (locals.var_ps0_inia_dn0 - (locals.var_ps0_inib_dn0 * 0.98)), (locals.var_ps0_inia_dn2 - (locals.var_ps0_inib_dn2 * 0.98)), (locals.var_ps0_inia_dn6 - (locals.var_ps0_inib_dn6 * 0.98)), (locals.var_ps0_inia_dn7 - (locals.var_ps0_inib_dn7 * 0.98)), (locals.var_ps0_inia_dn10 - (locals.var_ps0_inib_dn10 * 0.98)), (locals.var_ps0_inia_dn11 - (locals.var_ps0_inib_dn11 * 0.98)), (locals.var_ps0_inia_dn12 - (locals.var_ps0_inib_dn12 * 0.98)), (locals.var_ps0_inia_dn17 - (locals.var_ps0_inib_dn17 * 0.98)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign8870_e7030;
        locals.var_tmf1_dn0 = assign8870_e7030_d_n0;
        locals.var_tmf1_dn2 = assign8870_e7030_d_n2;
        locals.var_tmf1_dn6 = assign8870_e7030_d_n6;
        locals.var_tmf1_dn7 = assign8870_e7030_d_n7;
        locals.var_tmf1_dn10 = assign8870_e7030_d_n10;
        locals.var_tmf1_dn11 = assign8870_e7030_d_n11;
        locals.var_tmf1_dn12 = assign8870_e7030_d_n12;
        locals.var_tmf1_dn17 = assign8870_e7030_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign8880_e7046, assign8880_e7046_d_n0, assign8880_e7046_d_n2, assign8880_e7046_d_n6, assign8880_e7046_d_n7, assign8880_e7046_d_n10, assign8880_e7046_d_n11, assign8880_e7046_d_n12, assign8880_e7046_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8880_e7044: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign8880_e7044, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign8880_e7046;
        locals.var_x2_dn0 = assign8880_e7046_d_n0;
        locals.var_x2_dn2 = assign8880_e7046_d_n2;
        locals.var_x2_dn6 = assign8880_e7046_d_n6;
        locals.var_x2_dn7 = assign8880_e7046_d_n7;
        locals.var_x2_dn10 = assign8880_e7046_d_n10;
        locals.var_x2_dn11 = assign8880_e7046_d_n11;
        locals.var_x2_dn12 = assign8880_e7046_d_n12;
        locals.var_x2_dn17 = assign8880_e7046_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign8890_e7062, assign8890_e7062_d_n0, assign8890_e7062_d_n2, assign8890_e7062_d_n6, assign8890_e7062_d_n7, assign8890_e7062_d_n10, assign8890_e7062_d_n11, assign8890_e7062_d_n12, assign8890_e7062_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8890_e7060: f64 = (0.4 * 0.4);
        (assign8890_e7060, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign8890_e7062;
        locals.var_xmax2_dn0 = assign8890_e7062_d_n0;
        locals.var_xmax2_dn2 = assign8890_e7062_d_n2;
        locals.var_xmax2_dn6 = assign8890_e7062_d_n6;
        locals.var_xmax2_dn7 = assign8890_e7062_d_n7;
        locals.var_xmax2_dn10 = assign8890_e7062_d_n10;
        locals.var_xmax2_dn11 = assign8890_e7062_d_n11;
        locals.var_xmax2_dn12 = assign8890_e7062_d_n12;
        locals.var_xmax2_dn17 = assign8890_e7062_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign8900_e7076, assign8900_e7076_d_n0, assign8900_e7076_d_n2, assign8900_e7076_d_n6, assign8900_e7076_d_n7, assign8900_e7076_d_n10, assign8900_e7076_d_n11, assign8900_e7076_d_n12, assign8900_e7076_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8900_e7076;
        locals.var_xp_dn0 = assign8900_e7076_d_n0;
        locals.var_xp_dn2 = assign8900_e7076_d_n2;
        locals.var_xp_dn6 = assign8900_e7076_d_n6;
        locals.var_xp_dn7 = assign8900_e7076_d_n7;
        locals.var_xp_dn10 = assign8900_e7076_d_n10;
        locals.var_xp_dn11 = assign8900_e7076_d_n11;
        locals.var_xp_dn12 = assign8900_e7076_d_n12;
        locals.var_xp_dn17 = assign8900_e7076_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8910_e7090, assign8910_e7090_d_n0, assign8910_e7090_d_n2, assign8910_e7090_d_n6, assign8910_e7090_d_n7, assign8910_e7090_d_n10, assign8910_e7090_d_n11, assign8910_e7090_d_n12, assign8910_e7090_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8910_e7090;
        locals.var_xmp_dn0 = assign8910_e7090_d_n0;
        locals.var_xmp_dn2 = assign8910_e7090_d_n2;
        locals.var_xmp_dn6 = assign8910_e7090_d_n6;
        locals.var_xmp_dn7 = assign8910_e7090_d_n7;
        locals.var_xmp_dn10 = assign8910_e7090_d_n10;
        locals.var_xmp_dn11 = assign8910_e7090_d_n11;
        locals.var_xmp_dn12 = assign8910_e7090_d_n12;
        locals.var_xmp_dn17 = assign8910_e7090_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8920_e7104,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign8920_e7104;
        locals.var_m0_rv = 0.0;

        let (assign8930_e7118,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign8930_e7118;
        locals.var_mm_rv = 0.0;

        let (assign8940_e7132, assign8940_e7132_d_n0, assign8940_e7132_d_n2, assign8940_e7132_d_n6, assign8940_e7132_d_n7, assign8940_e7132_d_n10, assign8940_e7132_d_n11, assign8940_e7132_d_n12, assign8940_e7132_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign8940_e7132;
        locals.var_arg_dn0 = assign8940_e7132_d_n0;
        locals.var_arg_dn2 = assign8940_e7132_d_n2;
        locals.var_arg_dn6 = assign8940_e7132_d_n6;
        locals.var_arg_dn7 = assign8940_e7132_d_n7;
        locals.var_arg_dn10 = assign8940_e7132_d_n10;
        locals.var_arg_dn11 = assign8940_e7132_d_n11;
        locals.var_arg_dn12 = assign8940_e7132_d_n12;
        locals.var_arg_dn17 = assign8940_e7132_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign8950_e7146, assign8950_e7146_d_n0, assign8950_e7146_d_n2, assign8950_e7146_d_n6, assign8950_e7146_d_n7, assign8950_e7146_d_n10, assign8950_e7146_d_n11, assign8950_e7146_d_n12, assign8950_e7146_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign8950_e7146;
        locals.var_dnm_dn0 = assign8950_e7146_d_n0;
        locals.var_dnm_dn2 = assign8950_e7146_d_n2;
        locals.var_dnm_dn6 = assign8950_e7146_d_n6;
        locals.var_dnm_dn7 = assign8950_e7146_d_n7;
        locals.var_dnm_dn10 = assign8950_e7146_d_n10;
        locals.var_dnm_dn11 = assign8950_e7146_d_n11;
        locals.var_dnm_dn12 = assign8950_e7146_d_n12;
        locals.var_dnm_dn17 = assign8950_e7146_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign8960_e7162, assign8960_e7162_d_n0, assign8960_e7162_d_n2, assign8960_e7162_d_n6, assign8960_e7162_d_n7, assign8960_e7162_d_n10, assign8960_e7162_d_n11, assign8960_e7162_d_n12, assign8960_e7162_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8960_e7160: f64 = (locals.var_xp * locals.var_x2);
        (assign8960_e7160, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8960_e7162;
        locals.var_xp_dn0 = assign8960_e7162_d_n0;
        locals.var_xp_dn2 = assign8960_e7162_d_n2;
        locals.var_xp_dn6 = assign8960_e7162_d_n6;
        locals.var_xp_dn7 = assign8960_e7162_d_n7;
        locals.var_xp_dn10 = assign8960_e7162_d_n10;
        locals.var_xp_dn11 = assign8960_e7162_d_n11;
        locals.var_xp_dn12 = assign8960_e7162_d_n12;
        locals.var_xp_dn17 = assign8960_e7162_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8970_e7178, assign8970_e7178_d_n0, assign8970_e7178_d_n2, assign8970_e7178_d_n6, assign8970_e7178_d_n7, assign8970_e7178_d_n10, assign8970_e7178_d_n11, assign8970_e7178_d_n12, assign8970_e7178_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8970_e7176: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign8970_e7176, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8970_e7178;
        locals.var_xmp_dn0 = assign8970_e7178_d_n0;
        locals.var_xmp_dn2 = assign8970_e7178_d_n2;
        locals.var_xmp_dn6 = assign8970_e7178_d_n6;
        locals.var_xmp_dn7 = assign8970_e7178_d_n7;
        locals.var_xmp_dn10 = assign8970_e7178_d_n10;
        locals.var_xmp_dn11 = assign8970_e7178_d_n11;
        locals.var_xmp_dn12 = assign8970_e7178_d_n12;
        locals.var_xmp_dn17 = assign8970_e7178_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign8980_e7194, assign8980_e7194_d_n0, assign8980_e7194_d_n2, assign8980_e7194_d_n6, assign8980_e7194_d_n7, assign8980_e7194_d_n10, assign8980_e7194_d_n11, assign8980_e7194_d_n12, assign8980_e7194_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8980_e7192: f64 = (locals.var_xp * locals.var_x2);
        (assign8980_e7192, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign8980_e7194;
        locals.var_xp_dn0 = assign8980_e7194_d_n0;
        locals.var_xp_dn2 = assign8980_e7194_d_n2;
        locals.var_xp_dn6 = assign8980_e7194_d_n6;
        locals.var_xp_dn7 = assign8980_e7194_d_n7;
        locals.var_xp_dn10 = assign8980_e7194_d_n10;
        locals.var_xp_dn11 = assign8980_e7194_d_n11;
        locals.var_xp_dn12 = assign8980_e7194_d_n12;
        locals.var_xp_dn17 = assign8980_e7194_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign8990_e7210, assign8990_e7210_d_n0, assign8990_e7210_d_n2, assign8990_e7210_d_n6, assign8990_e7210_d_n7, assign8990_e7210_d_n10, assign8990_e7210_d_n11, assign8990_e7210_d_n12, assign8990_e7210_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign8990_e7208: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign8990_e7208, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign8990_e7210;
        locals.var_xmp_dn0 = assign8990_e7210_d_n0;
        locals.var_xmp_dn2 = assign8990_e7210_d_n2;
        locals.var_xmp_dn6 = assign8990_e7210_d_n6;
        locals.var_xmp_dn7 = assign8990_e7210_d_n7;
        locals.var_xmp_dn10 = assign8990_e7210_d_n10;
        locals.var_xmp_dn11 = assign8990_e7210_d_n11;
        locals.var_xmp_dn12 = assign8990_e7210_d_n12;
        locals.var_xmp_dn17 = assign8990_e7210_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign9000_e7226, assign9000_e7226_d_n0, assign9000_e7226_d_n2, assign9000_e7226_d_n6, assign9000_e7226_d_n7, assign9000_e7226_d_n10, assign9000_e7226_d_n11, assign9000_e7226_d_n12, assign9000_e7226_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign9000_e7224: f64 = (locals.var_xp + locals.var_xmp);
        (assign9000_e7224, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign9000_e7226;
        locals.var_arg_dn0 = assign9000_e7226_d_n0;
        locals.var_arg_dn2 = assign9000_e7226_d_n2;
        locals.var_arg_dn6 = assign9000_e7226_d_n6;
        locals.var_arg_dn7 = assign9000_e7226_d_n7;
        locals.var_arg_dn10 = assign9000_e7226_d_n10;
        locals.var_arg_dn11 = assign9000_e7226_d_n11;
        locals.var_arg_dn12 = assign9000_e7226_d_n12;
        locals.var_arg_dn17 = assign9000_e7226_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign9010_e7240, assign9010_e7240_d_n0, assign9010_e7240_d_n2, assign9010_e7240_d_n6, assign9010_e7240_d_n7, assign9010_e7240_d_n10, assign9010_e7240_d_n11, assign9010_e7240_d_n12, assign9010_e7240_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign9010_e7240;
        locals.var_dnm_dn0 = assign9010_e7240_d_n0;
        locals.var_dnm_dn2 = assign9010_e7240_d_n2;
        locals.var_dnm_dn6 = assign9010_e7240_d_n6;
        locals.var_dnm_dn7 = assign9010_e7240_d_n7;
        locals.var_dnm_dn10 = assign9010_e7240_d_n10;
        locals.var_dnm_dn11 = assign9010_e7240_d_n11;
        locals.var_dnm_dn12 = assign9010_e7240_d_n12;
        locals.var_dnm_dn17 = assign9010_e7240_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign9020_e7255: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard165 = assign9020_e7255;
        locals.var_guard165_rv = 0.0;

        let assign9030_e7258: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign9030_e7258;
        locals.var_guard166_rv = 0.0;

        let (assign9040_e7276,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9040_e7276;
        locals.var_mm_rv = 0.0;

        let assign9050_e7279: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign9050_e7279;
        locals.var_guard167_rv = 0.0;

        let (assign9060_e7300,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9060_e7300;
        locals.var_mm_rv = 0.0;

        let assign9070_e7303: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign9070_e7303;
        locals.var_guard168_rv = 0.0;

        let (assign9080_e7327,) = {
    if (((((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard168 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9080_e7327;
        locals.var_mm_rv = 0.0;

        let assign9090_e7330: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign9090_e7330;
        locals.var_guard169_rv = 0.0;

        let (assign9100_e7357,) = {
    if ((((((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard168 == 0.0)) && (locals.var_guard169 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign9100_e7357;
        locals.var_mm_rv = 0.0;

        let (assign9110_e7373,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign9110_e7373;
        locals.var_m0_rv = 0.0;

        let mut assign9120_loop_guard: usize = 0;
        while {
            let assign9120_cond_e7390: f64 = if (((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign9120_cond_e7390 != 0.0
        } {
            assign9120_loop_guard += 1;
            assert!(assign9120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9120_body0_e7407, assign9120_body0_e7407_d_n0, assign9120_body0_e7407_d_n2, assign9120_body0_e7407_d_n6, assign9120_body0_e7407_d_n7, assign9120_body0_e7407_d_n10, assign9120_body0_e7407_d_n11, assign9120_body0_e7407_d_n12, assign9120_body0_e7407_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) {
        let assign9120_body0_e7405: f64 = (locals.var_dnm).sqrt();
        (assign9120_body0_e7405, (locals.var_dnm_dn0 / (2.0 * assign9120_body0_e7405)), (locals.var_dnm_dn2 / (2.0 * assign9120_body0_e7405)), (locals.var_dnm_dn6 / (2.0 * assign9120_body0_e7405)), (locals.var_dnm_dn7 / (2.0 * assign9120_body0_e7405)), (locals.var_dnm_dn10 / (2.0 * assign9120_body0_e7405)), (locals.var_dnm_dn11 / (2.0 * assign9120_body0_e7405)), (locals.var_dnm_dn12 / (2.0 * assign9120_body0_e7405)), (locals.var_dnm_dn17 / (2.0 * assign9120_body0_e7405)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign9120_body0_e7407;
            locals.var_dnm_dn0 = assign9120_body0_e7407_d_n0;
            locals.var_dnm_dn2 = assign9120_body0_e7407_d_n2;
            locals.var_dnm_dn6 = assign9120_body0_e7407_d_n6;
            locals.var_dnm_dn7 = assign9120_body0_e7407_d_n7;
            locals.var_dnm_dn10 = assign9120_body0_e7407_d_n10;
            locals.var_dnm_dn11 = assign9120_body0_e7407_d_n11;
            locals.var_dnm_dn12 = assign9120_body0_e7407_d_n12;
            locals.var_dnm_dn17 = assign9120_body0_e7407_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign9120_body1_e7425,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 != 0.0)) {
        let assign9120_body1_e7423: f64 = (locals.var_m0 + 1.0);
        (assign9120_body1_e7423,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign9120_body1_e7425;
            locals.var_m0_rv = 0.0;
        }

        let (assign9130_e7448, assign9130_e7448_d_n0, assign9130_e7448_d_n2, assign9130_e7448_d_n6, assign9130_e7448_d_n7, assign9130_e7448_d_n10, assign9130_e7448_d_n11, assign9130_e7448_d_n12, assign9130_e7448_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) && (locals.var_guard165 == 0.0)) {
        let assign9130_e7444: f64 = (2.0 * 2.0);
        let assign9130_e7445: f64 = (1.0 / assign9130_e7444);
        let assign9130_e7446: f64 = (locals.var_dnm).powf(assign9130_e7445);
        (assign9130_e7446, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn0)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn2)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn6)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn7)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn10)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn11)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn12)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign9130_e7445) as f64).is_finite() && ((assign9130_e7445) as f64).fract() == 0.0 { if assign9130_e7445 == 0.0 { 0.0 } else { (assign9130_e7445 * ((locals.var_dnm).powf(assign9130_e7445 - 1.0) * locals.var_dnm_dn17)) } } else { (assign9130_e7446 * (assign9130_e7445 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign9130_e7448;
        locals.var_dnm_dn0 = assign9130_e7448_d_n0;
        locals.var_dnm_dn2 = assign9130_e7448_d_n2;
        locals.var_dnm_dn6 = assign9130_e7448_d_n6;
        locals.var_dnm_dn7 = assign9130_e7448_d_n7;
        locals.var_dnm_dn10 = assign9130_e7448_d_n10;
        locals.var_dnm_dn11 = assign9130_e7448_d_n11;
        locals.var_dnm_dn12 = assign9130_e7448_d_n12;
        locals.var_dnm_dn17 = assign9130_e7448_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign9140_e7464, assign9140_e7464_d_n0, assign9140_e7464_d_n2, assign9140_e7464_d_n6, assign9140_e7464_d_n7, assign9140_e7464_d_n10, assign9140_e7464_d_n11, assign9140_e7464_d_n12, assign9140_e7464_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign9140_e7462: f64 = (1.0 / locals.var_dnm);
        (assign9140_e7462, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign9140_e7464;
        locals.var_dnm_dn0 = assign9140_e7464_d_n0;
        locals.var_dnm_dn2 = assign9140_e7464_d_n2;
        locals.var_dnm_dn6 = assign9140_e7464_d_n6;
        locals.var_dnm_dn7 = assign9140_e7464_d_n7;
        locals.var_dnm_dn10 = assign9140_e7464_d_n10;
        locals.var_dnm_dn11 = assign9140_e7464_d_n11;
        locals.var_dnm_dn12 = assign9140_e7464_d_n12;
        locals.var_dnm_dn17 = assign9140_e7464_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign9150_e7482, assign9150_e7482_d_n0, assign9150_e7482_d_n2, assign9150_e7482_d_n6, assign9150_e7482_d_n7, assign9150_e7482_d_n10, assign9150_e7482_d_n11, assign9150_e7482_d_n12, assign9150_e7482_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign9150_e7478: f64 = (locals.var_tmf1 * 0.4);
        let assign9150_e7480: f64 = (assign9150_e7478 * locals.var_dnm);
        (assign9150_e7480, (((locals.var_tmf1_dn0 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 0.4) * locals.var_dnm) + (assign9150_e7478 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign9150_e7482;
        locals.var_tmf0_dn0 = assign9150_e7482_d_n0;
        locals.var_tmf0_dn2 = assign9150_e7482_d_n2;
        locals.var_tmf0_dn6 = assign9150_e7482_d_n6;
        locals.var_tmf0_dn7 = assign9150_e7482_d_n7;
        locals.var_tmf0_dn10 = assign9150_e7482_d_n10;
        locals.var_tmf0_dn11 = assign9150_e7482_d_n11;
        locals.var_tmf0_dn12 = assign9150_e7482_d_n12;
        locals.var_tmf0_dn17 = assign9150_e7482_d_n17;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        locals: &mut StampLocals,
    ) {
        let (assign9160_e7502, assign9160_e7502_d_n0, assign9160_e7502_d_n2, assign9160_e7502_d_n6, assign9160_e7502_d_n7, assign9160_e7502_d_n10, assign9160_e7502_d_n11, assign9160_e7502_d_n12, assign9160_e7502_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 != 0.0)) {
        let assign9160_e7496: f64 = (locals.var_ps0_inib * 0.98);
        let assign9160_e7498: f64 = (assign9160_e7496 - 0.4);
        let assign9160_e7500: f64 = (assign9160_e7498 + locals.var_tmf0);
        (assign9160_e7500, ((locals.var_ps0_inib_dn0 * 0.98) + locals.var_tmf0_dn0), ((locals.var_ps0_inib_dn2 * 0.98) + locals.var_tmf0_dn2), ((locals.var_ps0_inib_dn6 * 0.98) + locals.var_tmf0_dn6), ((locals.var_ps0_inib_dn7 * 0.98) + locals.var_tmf0_dn7), ((locals.var_ps0_inib_dn10 * 0.98) + locals.var_tmf0_dn10), ((locals.var_ps0_inib_dn11 * 0.98) + locals.var_tmf0_dn11), ((locals.var_ps0_inib_dn12 * 0.98) + locals.var_tmf0_dn12), ((locals.var_ps0_inib_dn17 * 0.98) + locals.var_tmf0_dn17),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign9160_e7502;
        locals.var_ps0_ini_dn0 = assign9160_e7502_d_n0;
        locals.var_ps0_ini_dn2 = assign9160_e7502_d_n2;
        locals.var_ps0_ini_dn6 = assign9160_e7502_d_n6;
        locals.var_ps0_ini_dn7 = assign9160_e7502_d_n7;
        locals.var_ps0_ini_dn10 = assign9160_e7502_d_n10;
        locals.var_ps0_ini_dn11 = assign9160_e7502_d_n11;
        locals.var_ps0_ini_dn12 = assign9160_e7502_d_n12;
        locals.var_ps0_ini_dn17 = assign9160_e7502_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign9170_e7517, assign9170_e7517_d_n0, assign9170_e7517_d_n2, assign9170_e7517_d_n6, assign9170_e7517_d_n7, assign9170_e7517_d_n10, assign9170_e7517_d_n11, assign9170_e7517_d_n12, assign9170_e7517_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard162 == 0.0)) && (locals.var_guard163 != 0.0)) && (locals.var_guard164 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign9170_e7517;
        locals.var_ps0_ini_dn0 = assign9170_e7517_d_n0;
        locals.var_ps0_ini_dn2 = assign9170_e7517_d_n2;
        locals.var_ps0_ini_dn6 = assign9170_e7517_d_n6;
        locals.var_ps0_ini_dn7 = assign9170_e7517_d_n7;
        locals.var_ps0_ini_dn10 = assign9170_e7517_d_n10;
        locals.var_ps0_ini_dn11 = assign9170_e7517_d_n11;
        locals.var_ps0_ini_dn12 = assign9170_e7517_d_n12;
        locals.var_ps0_ini_dn17 = assign9170_e7517_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign9180_e7524, assign9180_e7524_d_n0, assign9180_e7524_d_n2, assign9180_e7524_d_n6, assign9180_e7524_d_n7, assign9180_e7524_d_n10, assign9180_e7524_d_n11, assign9180_e7524_d_n12, assign9180_e7524_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign9180_e7524;
        locals.var_phi_s0_soi_dn0 = assign9180_e7524_d_n0;
        locals.var_phi_s0_soi_dn2 = assign9180_e7524_d_n2;
        locals.var_phi_s0_soi_dn6 = assign9180_e7524_d_n6;
        locals.var_phi_s0_soi_dn7 = assign9180_e7524_d_n7;
        locals.var_phi_s0_soi_dn10 = assign9180_e7524_d_n10;
        locals.var_phi_s0_soi_dn11 = assign9180_e7524_d_n11;
        locals.var_phi_s0_soi_dn12 = assign9180_e7524_d_n12;
        locals.var_phi_s0_soi_dn17 = assign9180_e7524_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign9190_e7531, assign9190_e7531_d_n0, assign9190_e7531_d_n2, assign9190_e7531_d_n6, assign9190_e7531_d_n7, assign9190_e7531_d_n10, assign9190_e7531_d_n11, assign9190_e7531_d_n12, assign9190_e7531_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign9190_e7531;
        locals.var_psl_lim_dn0 = assign9190_e7531_d_n0;
        locals.var_psl_lim_dn2 = assign9190_e7531_d_n2;
        locals.var_psl_lim_dn6 = assign9190_e7531_d_n6;
        locals.var_psl_lim_dn7 = assign9190_e7531_d_n7;
        locals.var_psl_lim_dn10 = assign9190_e7531_d_n10;
        locals.var_psl_lim_dn11 = assign9190_e7531_d_n11;
        locals.var_psl_lim_dn12 = assign9190_e7531_d_n12;
        locals.var_psl_lim_dn17 = assign9190_e7531_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let (assign9200_e7546, assign9200_e7546_d_n0, assign9200_e7546_d_n2, assign9200_e7546_d_n6, assign9200_e7546_d_n7, assign9200_e7546_d_n10, assign9200_e7546_d_n11, assign9200_e7546_d_n12, assign9200_e7546_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9200_e7539: f64 = (0.5 * locals.var_q_fd_soi);
        let assign9200_e7541: f64 = (assign9200_e7539 * locals.var_c_soi_inv__blk111);
        let assign9200_e7542: f64 = (locals.var_phi_s0_soi + assign9200_e7541);
        let assign9200_e7544: f64 = (assign9200_e7542 - locals.var_vbsbiz);
        (assign9200_e7544, ((locals.var_phi_s0_soi_dn0 + ((0.5 * locals.var_q_fd_soi_dn0) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn0), ((locals.var_phi_s0_soi_dn2 + ((0.5 * locals.var_q_fd_soi_dn2) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn2), ((locals.var_phi_s0_soi_dn6 + ((0.5 * locals.var_q_fd_soi_dn6) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn6), ((locals.var_phi_s0_soi_dn7 + ((0.5 * locals.var_q_fd_soi_dn7) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn7), ((locals.var_phi_s0_soi_dn10 + ((0.5 * locals.var_q_fd_soi_dn10) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn10), ((locals.var_phi_s0_soi_dn11 + ((0.5 * locals.var_q_fd_soi_dn11) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn11), ((locals.var_phi_s0_soi_dn12 + ((0.5 * locals.var_q_fd_soi_dn12) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn12), ((locals.var_phi_s0_soi_dn17 + ((0.5 * locals.var_q_fd_soi_dn17) * locals.var_c_soi_inv__blk111)) - locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign9200_e7546;
        locals.var_t1_dn0 = assign9200_e7546_d_n0;
        locals.var_t1_dn2 = assign9200_e7546_d_n2;
        locals.var_t1_dn6 = assign9200_e7546_d_n6;
        locals.var_t1_dn7 = assign9200_e7546_d_n7;
        locals.var_t1_dn10 = assign9200_e7546_d_n10;
        locals.var_t1_dn11 = assign9200_e7546_d_n11;
        locals.var_t1_dn12 = assign9200_e7546_d_n12;
        locals.var_t1_dn17 = assign9200_e7546_d_n17;
        locals.var_t1_rv = 0.0;

        let assign9210_e7549: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign9210_e7549;
        locals.var_guard170_rv = 0.0;

        let (assign9220_e7562, assign9220_e7562_d_n0, assign9220_e7562_d_n2, assign9220_e7562_d_n6, assign9220_e7562_d_n7, assign9220_e7562_d_n10, assign9220_e7562_d_n11, assign9220_e7562_d_n12, assign9220_e7562_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9220_e7559: f64 = (locals.var_c_box_inv + locals.var_c_soi_inv__blk111);
        let assign9220_e7560: f64 = (locals.var_cnst0bulk * assign9220_e7559);
        (assign9220_e7560, 0.0, 0.0, 0.0, 0.0, (locals.var_cnst0bulk_dn10 * assign9220_e7559), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9220_e7562;
        locals.var_t2_dn0 = assign9220_e7562_d_n0;
        locals.var_t2_dn2 = assign9220_e7562_d_n2;
        locals.var_t2_dn6 = assign9220_e7562_d_n6;
        locals.var_t2_dn7 = assign9220_e7562_d_n7;
        locals.var_t2_dn10 = assign9220_e7562_d_n10;
        locals.var_t2_dn11 = assign9220_e7562_d_n11;
        locals.var_t2_dn12 = assign9220_e7562_d_n12;
        locals.var_t2_dn17 = assign9220_e7562_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9230_e7573, assign9230_e7573_d_n0, assign9230_e7573_d_n2, assign9230_e7573_d_n6, assign9230_e7573_d_n7, assign9230_e7573_d_n10, assign9230_e7573_d_n11, assign9230_e7573_d_n12, assign9230_e7573_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9230_e7571: f64 = (locals.var_t2 * locals.var_t2);
        (assign9230_e7571, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9230_e7573;
        locals.var_t2_dn0 = assign9230_e7573_d_n0;
        locals.var_t2_dn2 = assign9230_e7573_d_n2;
        locals.var_t2_dn6 = assign9230_e7573_d_n6;
        locals.var_t2_dn7 = assign9230_e7573_d_n7;
        locals.var_t2_dn10 = assign9230_e7573_d_n10;
        locals.var_t2_dn11 = assign9230_e7573_d_n11;
        locals.var_t2_dn12 = assign9230_e7573_d_n12;
        locals.var_t2_dn17 = assign9230_e7573_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9240_e7587, assign9240_e7587_d_n0, assign9240_e7587_d_n2, assign9240_e7587_d_n6, assign9240_e7587_d_n7, assign9240_e7587_d_n10, assign9240_e7587_d_n11, assign9240_e7587_d_n12, assign9240_e7587_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9240_e7581: f64 = (-1.6);
        let assign9240_e7583: f64 = (assign9240_e7581 * locals.var_t1);
        let assign9240_e7585: f64 = (assign9240_e7583 + 0.6);
        (assign9240_e7585, (assign9240_e7581 * locals.var_t1_dn0), (assign9240_e7581 * locals.var_t1_dn2), (assign9240_e7581 * locals.var_t1_dn6), (assign9240_e7581 * locals.var_t1_dn7), (assign9240_e7581 * locals.var_t1_dn10), (assign9240_e7581 * locals.var_t1_dn11), (assign9240_e7581 * locals.var_t1_dn12), (assign9240_e7581 * locals.var_t1_dn17),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign9240_e7587;
        locals.var_t5_dn0 = assign9240_e7587_d_n0;
        locals.var_t5_dn2 = assign9240_e7587_d_n2;
        locals.var_t5_dn6 = assign9240_e7587_d_n6;
        locals.var_t5_dn7 = assign9240_e7587_d_n7;
        locals.var_t5_dn10 = assign9240_e7587_d_n10;
        locals.var_t5_dn11 = assign9240_e7587_d_n11;
        locals.var_t5_dn12 = assign9240_e7587_d_n12;
        locals.var_t5_dn17 = assign9240_e7587_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign9250_e7596, assign9250_e7596_d_n0, assign9250_e7596_d_n2, assign9250_e7596_d_n6, assign9250_e7596_d_n7, assign9250_e7596_d_n10, assign9250_e7596_d_n11, assign9250_e7596_d_n12, assign9250_e7596_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign9250_e7596;
        locals.var_t4_dn0 = assign9250_e7596_d_n0;
        locals.var_t4_dn2 = assign9250_e7596_d_n2;
        locals.var_t4_dn6 = assign9250_e7596_d_n6;
        locals.var_t4_dn7 = assign9250_e7596_d_n7;
        locals.var_t4_dn10 = assign9250_e7596_d_n10;
        locals.var_t4_dn11 = assign9250_e7596_d_n11;
        locals.var_t4_dn12 = assign9250_e7596_d_n12;
        locals.var_t4_dn17 = assign9250_e7596_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign9260_e7611, assign9260_e7611_d_n0, assign9260_e7611_d_n2, assign9260_e7611_d_n6, assign9260_e7611_d_n7, assign9260_e7611_d_n10, assign9260_e7611_d_n11, assign9260_e7611_d_n12, assign9260_e7611_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9260_e7605: f64 = (locals.var_t5 - locals.var_t4);
        let assign9260_e7608: f64 = (locals.var_t5 * 0.001);
        let assign9260_e7609: f64 = (assign9260_e7605 - assign9260_e7608);
        (assign9260_e7609, ((locals.var_t5_dn0 - locals.var_t4_dn0) - (locals.var_t5_dn0 * 0.001)), ((locals.var_t5_dn2 - locals.var_t4_dn2) - (locals.var_t5_dn2 * 0.001)), ((locals.var_t5_dn6 - locals.var_t4_dn6) - (locals.var_t5_dn6 * 0.001)), ((locals.var_t5_dn7 - locals.var_t4_dn7) - (locals.var_t5_dn7 * 0.001)), ((locals.var_t5_dn10 - locals.var_t4_dn10) - (locals.var_t5_dn10 * 0.001)), ((locals.var_t5_dn11 - locals.var_t4_dn11) - (locals.var_t5_dn11 * 0.001)), ((locals.var_t5_dn12 - locals.var_t4_dn12) - (locals.var_t5_dn12 * 0.001)), ((locals.var_t5_dn17 - locals.var_t4_dn17) - (locals.var_t5_dn17 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign9260_e7611;
        locals.var_tmf1_dn0 = assign9260_e7611_d_n0;
        locals.var_tmf1_dn2 = assign9260_e7611_d_n2;
        locals.var_tmf1_dn6 = assign9260_e7611_d_n6;
        locals.var_tmf1_dn7 = assign9260_e7611_d_n7;
        locals.var_tmf1_dn10 = assign9260_e7611_d_n10;
        locals.var_tmf1_dn11 = assign9260_e7611_d_n11;
        locals.var_tmf1_dn12 = assign9260_e7611_d_n12;
        locals.var_tmf1_dn17 = assign9260_e7611_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign9270_e7626, assign9270_e7626_d_n0, assign9270_e7626_d_n2, assign9270_e7626_d_n6, assign9270_e7626_d_n7, assign9270_e7626_d_n10, assign9270_e7626_d_n11, assign9270_e7626_d_n12, assign9270_e7626_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9270_e7620: f64 = (4.0 * locals.var_t5);
        let assign9270_e7623: f64 = (locals.var_t5 * 0.001);
        let assign9270_e7624: f64 = (assign9270_e7620 * assign9270_e7623);
        (assign9270_e7624, (((4.0 * locals.var_t5_dn0) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn0 * 0.001))), (((4.0 * locals.var_t5_dn2) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn2 * 0.001))), (((4.0 * locals.var_t5_dn6) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn6 * 0.001))), (((4.0 * locals.var_t5_dn7) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn7 * 0.001))), (((4.0 * locals.var_t5_dn10) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn10 * 0.001))), (((4.0 * locals.var_t5_dn11) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn11 * 0.001))), (((4.0 * locals.var_t5_dn12) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn12 * 0.001))), (((4.0 * locals.var_t5_dn17) * assign9270_e7623) + (assign9270_e7620 * (locals.var_t5_dn17 * 0.001))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9270_e7626;
        locals.var_tmf2_dn0 = assign9270_e7626_d_n0;
        locals.var_tmf2_dn2 = assign9270_e7626_d_n2;
        locals.var_tmf2_dn6 = assign9270_e7626_d_n6;
        locals.var_tmf2_dn7 = assign9270_e7626_d_n7;
        locals.var_tmf2_dn10 = assign9270_e7626_d_n10;
        locals.var_tmf2_dn11 = assign9270_e7626_d_n11;
        locals.var_tmf2_dn12 = assign9270_e7626_d_n12;
        locals.var_tmf2_dn17 = assign9270_e7626_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9280_e7641, assign9280_e7641_d_n0, assign9280_e7641_d_n2, assign9280_e7641_d_n6, assign9280_e7641_d_n7, assign9280_e7641_d_n10, assign9280_e7641_d_n11, assign9280_e7641_d_n12, assign9280_e7641_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let (assign9280_e7639, assign9280_e7639_d_n0, assign9280_e7639_d_n2, assign9280_e7639_d_n6, assign9280_e7639_d_n7, assign9280_e7639_d_n10, assign9280_e7639_d_n11, assign9280_e7639_d_n12, assign9280_e7639_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign9280_e7638: f64 = (-locals.var_tmf2);
                (assign9280_e7638, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign9280_e7639, assign9280_e7639_d_n0, assign9280_e7639_d_n2, assign9280_e7639_d_n6, assign9280_e7639_d_n7, assign9280_e7639_d_n10, assign9280_e7639_d_n11, assign9280_e7639_d_n12, assign9280_e7639_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9280_e7641;
        locals.var_tmf2_dn0 = assign9280_e7641_d_n0;
        locals.var_tmf2_dn2 = assign9280_e7641_d_n2;
        locals.var_tmf2_dn6 = assign9280_e7641_d_n6;
        locals.var_tmf2_dn7 = assign9280_e7641_d_n7;
        locals.var_tmf2_dn10 = assign9280_e7641_d_n10;
        locals.var_tmf2_dn11 = assign9280_e7641_d_n11;
        locals.var_tmf2_dn12 = assign9280_e7641_d_n12;
        locals.var_tmf2_dn17 = assign9280_e7641_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9290_e7655, assign9290_e7655_d_n0, assign9290_e7655_d_n2, assign9290_e7655_d_n6, assign9290_e7655_d_n7, assign9290_e7655_d_n10, assign9290_e7655_d_n11, assign9290_e7655_d_n12, assign9290_e7655_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9290_e7650: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9290_e7652: f64 = (assign9290_e7650 + locals.var_tmf2);
        let assign9290_e7653: f64 = (assign9290_e7652).sqrt();
        (assign9290_e7653, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9290_e7653)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9290_e7653)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign9290_e7653)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign9290_e7653)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign9290_e7653)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign9290_e7653)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign9290_e7653)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign9290_e7653)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9290_e7655;
        locals.var_tmf2_dn0 = assign9290_e7655_d_n0;
        locals.var_tmf2_dn2 = assign9290_e7655_d_n2;
        locals.var_tmf2_dn6 = assign9290_e7655_d_n6;
        locals.var_tmf2_dn7 = assign9290_e7655_d_n7;
        locals.var_tmf2_dn10 = assign9290_e7655_d_n10;
        locals.var_tmf2_dn11 = assign9290_e7655_d_n11;
        locals.var_tmf2_dn12 = assign9290_e7655_d_n12;
        locals.var_tmf2_dn17 = assign9290_e7655_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9300_e7670, assign9300_e7670_d_n0, assign9300_e7670_d_n2, assign9300_e7670_d_n6, assign9300_e7670_d_n7, assign9300_e7670_d_n10, assign9300_e7670_d_n11, assign9300_e7670_d_n12, assign9300_e7670_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9300_e7666: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9300_e7667: f64 = (0.5 * assign9300_e7666);
        let assign9300_e7668: f64 = (locals.var_t5 - assign9300_e7667);
        (assign9300_e7668, (locals.var_t5_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t5_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t5_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t5_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t5_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t5_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t5_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_t5_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign9300_e7670;
        locals.var_t4_dn0 = assign9300_e7670_d_n0;
        locals.var_t4_dn2 = assign9300_e7670_d_n2;
        locals.var_t4_dn6 = assign9300_e7670_d_n6;
        locals.var_t4_dn7 = assign9300_e7670_d_n7;
        locals.var_t4_dn10 = assign9300_e7670_d_n10;
        locals.var_t4_dn11 = assign9300_e7670_d_n11;
        locals.var_t4_dn12 = assign9300_e7670_d_n12;
        locals.var_t4_dn17 = assign9300_e7670_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign9310_e7683, assign9310_e7683_d_n0, assign9310_e7683_d_n2, assign9310_e7683_d_n6, assign9310_e7683_d_n7, assign9310_e7683_d_n10, assign9310_e7683_d_n11, assign9310_e7683_d_n12, assign9310_e7683_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9310_e7679: f64 = (locals.var_t2 * locals.var_t4);
        let assign9310_e7681: f64 = (assign9310_e7679 * locals.var_beta2);
        (assign9310_e7681, (((locals.var_t2_dn0 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn0)) * locals.var_beta2), (((locals.var_t2_dn2 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn2)) * locals.var_beta2), (((locals.var_t2_dn6 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn6)) * locals.var_beta2), (((locals.var_t2_dn7 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn7)) * locals.var_beta2), ((((locals.var_t2_dn10 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn10)) * locals.var_beta2) + (assign9310_e7679 * locals.var_beta2_dn10)), (((locals.var_t2_dn11 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn11)) * locals.var_beta2), (((locals.var_t2_dn12 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn12)) * locals.var_beta2), (((locals.var_t2_dn17 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn17)) * locals.var_beta2),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign9310_e7683;
        locals.var_t3_dn0 = assign9310_e7683_d_n0;
        locals.var_t3_dn2 = assign9310_e7683_d_n2;
        locals.var_t3_dn6 = assign9310_e7683_d_n6;
        locals.var_t3_dn7 = assign9310_e7683_d_n7;
        locals.var_t3_dn10 = assign9310_e7683_d_n10;
        locals.var_t3_dn11 = assign9310_e7683_d_n11;
        locals.var_t3_dn12 = assign9310_e7683_d_n12;
        locals.var_t3_dn17 = assign9310_e7683_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign9320_e7701, assign9320_e7701_d_n0, assign9320_e7701_d_n2, assign9320_e7701_d_n6, assign9320_e7701_d_n7, assign9320_e7701_d_n10, assign9320_e7701_d_n11, assign9320_e7701_d_n12, assign9320_e7701_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 != 0.0)) {
        let assign9320_e7693: f64 = (locals.var_t3).sqrt();
        let assign9320_e7694: f64 = (1.0 - assign9320_e7693);
        let assign9320_e7695: f64 = (locals.var_t1 * assign9320_e7694);
        let assign9320_e7698: f64 = (1.0 - locals.var_t3);
        let assign9320_e7699: f64 = (assign9320_e7695 / assign9320_e7698);
        (assign9320_e7699, (((((locals.var_t1_dn0 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn0 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn0))) / (assign9320_e7698 * assign9320_e7698)), (((((locals.var_t1_dn2 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn2 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn2))) / (assign9320_e7698 * assign9320_e7698)), (((((locals.var_t1_dn6 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn6 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn6))) / (assign9320_e7698 * assign9320_e7698)), (((((locals.var_t1_dn7 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn7 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn7))) / (assign9320_e7698 * assign9320_e7698)), (((((locals.var_t1_dn10 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn10 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn10))) / (assign9320_e7698 * assign9320_e7698)), (((((locals.var_t1_dn11 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn11 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn11))) / (assign9320_e7698 * assign9320_e7698)), (((((locals.var_t1_dn12 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn12 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn12))) / (assign9320_e7698 * assign9320_e7698)), (((((locals.var_t1_dn17 * assign9320_e7694) + (locals.var_t1 * (-(locals.var_t3_dn17 / (2.0 * assign9320_e7693))))) * assign9320_e7698) - (assign9320_e7695 * (-locals.var_t3_dn17))) / (assign9320_e7698 * assign9320_e7698)),)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9320_e7701;
        locals.var_phi_s0_bulk_dn0 = assign9320_e7701_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9320_e7701_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9320_e7701_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9320_e7701_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9320_e7701_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9320_e7701_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9320_e7701_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9320_e7701_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9330_e7717, assign9330_e7717_d_n0, assign9330_e7717_d_n2, assign9330_e7717_d_n6, assign9330_e7717_d_n7, assign9330_e7717_d_n10, assign9330_e7717_d_n11, assign9330_e7717_d_n12, assign9330_e7717_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9330_e7711: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign9330_e7713: f64 = (assign9330_e7711 * locals.var_c_box_fd_inv);
        let assign9330_e7715: f64 = (assign9330_e7713 * locals.var_c_box_fd_inv);
        (assign9330_e7715, 0.0, 0.0, 0.0, 0.0, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign9330_e7717;
        locals.var_t0_dn0 = assign9330_e7717_d_n0;
        locals.var_t0_dn2 = assign9330_e7717_d_n2;
        locals.var_t0_dn6 = assign9330_e7717_d_n6;
        locals.var_t0_dn7 = assign9330_e7717_d_n7;
        locals.var_t0_dn10 = assign9330_e7717_d_n10;
        locals.var_t0_dn11 = assign9330_e7717_d_n11;
        locals.var_t0_dn12 = assign9330_e7717_d_n12;
        locals.var_t0_dn17 = assign9330_e7717_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign9340_e7738, assign9340_e7738_d_n0, assign9340_e7738_d_n2, assign9340_e7738_d_n6, assign9340_e7738_d_n7, assign9340_e7738_d_n10, assign9340_e7738_d_n11, assign9340_e7738_d_n12, assign9340_e7738_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9340_e7727: f64 = (locals.var_vbsbiz - locals.var_phi_s0_soi);
        let assign9340_e7730: f64 = (locals.var_q_fd_soi / 2.0);
        let assign9340_e7732: f64 = (assign9340_e7730 * locals.var_t_soi);
        let assign9340_e7734: f64 = (assign9340_e7732 / 1.034943e-10);
        let assign9340_e7735: f64 = (assign9340_e7727 - assign9340_e7734);
        let assign9340_e7736: f64 = (-assign9340_e7735);
        (assign9340_e7736, (-((locals.var_vbsbiz_dn0 - locals.var_phi_s0_soi_dn0) - (((locals.var_q_fd_soi_dn0 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn2 - locals.var_phi_s0_soi_dn2) - (((locals.var_q_fd_soi_dn2 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn6 - locals.var_phi_s0_soi_dn6) - (((locals.var_q_fd_soi_dn6 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn7 - locals.var_phi_s0_soi_dn7) - (((locals.var_q_fd_soi_dn7 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn10 - locals.var_phi_s0_soi_dn10) - (((locals.var_q_fd_soi_dn10 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn11 - locals.var_phi_s0_soi_dn11) - (((locals.var_q_fd_soi_dn11 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn12 - locals.var_phi_s0_soi_dn12) - (((locals.var_q_fd_soi_dn12 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn17 - locals.var_phi_s0_soi_dn17) - (((locals.var_q_fd_soi_dn17 / 2.0) * locals.var_t_soi) / 1.034943e-10))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign9340_e7738;
        locals.var_t1_dn0 = assign9340_e7738_d_n0;
        locals.var_t1_dn2 = assign9340_e7738_d_n2;
        locals.var_t1_dn6 = assign9340_e7738_d_n6;
        locals.var_t1_dn7 = assign9340_e7738_d_n7;
        locals.var_t1_dn10 = assign9340_e7738_d_n10;
        locals.var_t1_dn11 = assign9340_e7738_d_n11;
        locals.var_t1_dn12 = assign9340_e7738_d_n12;
        locals.var_t1_dn17 = assign9340_e7738_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign9350_e7770, assign9350_e7770_d_n0, assign9350_e7770_d_n2, assign9350_e7770_d_n6, assign9350_e7770_d_n7, assign9350_e7770_d_n10, assign9350_e7770_d_n11, assign9350_e7770_d_n12, assign9350_e7770_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9350_e7748: f64 = (2.0 * locals.var_t1);
        let assign9350_e7751: f64 = (locals.var_t0 * locals.var_beta);
        let assign9350_e7752: f64 = (assign9350_e7748 + assign9350_e7751);
        let assign9350_e7755: f64 = (2.0 * locals.var_t1);
        let assign9350_e7758: f64 = (locals.var_t0 * locals.var_beta);
        let assign9350_e7759: f64 = (assign9350_e7755 + assign9350_e7758);
        let assign9350_e7760: f64 = (assign9350_e7752 * assign9350_e7759);
        let assign9350_e7764: f64 = (locals.var_t1 * locals.var_t1);
        let assign9350_e7766: f64 = (assign9350_e7764 + locals.var_t0);
        let assign9350_e7767: f64 = (4.0 * assign9350_e7766);
        let assign9350_e7768: f64 = (assign9350_e7760 - assign9350_e7767);
        (assign9350_e7768, (((((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + locals.var_t0_dn0))), (((((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + locals.var_t0_dn2))), (((((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + locals.var_t0_dn6))), (((((2.0 * locals.var_t1_dn7) + (locals.var_t0_dn7 * locals.var_beta)) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn7) + (locals.var_t0_dn7 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + locals.var_t0_dn7))), (((((2.0 * locals.var_t1_dn10) + ((locals.var_t0_dn10 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn10))) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn10) + ((locals.var_t0_dn10 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + locals.var_t0_dn10))), (((((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + locals.var_t0_dn11))), (((((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + locals.var_t0_dn12))), (((((2.0 * locals.var_t1_dn17) + (locals.var_t0_dn17 * locals.var_beta)) * assign9350_e7759) + (assign9350_e7752 * ((2.0 * locals.var_t1_dn17) + (locals.var_t0_dn17 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) + locals.var_t0_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9350_e7770;
        locals.var_t2_dn0 = assign9350_e7770_d_n0;
        locals.var_t2_dn2 = assign9350_e7770_d_n2;
        locals.var_t2_dn6 = assign9350_e7770_d_n6;
        locals.var_t2_dn7 = assign9350_e7770_d_n7;
        locals.var_t2_dn10 = assign9350_e7770_d_n10;
        locals.var_t2_dn11 = assign9350_e7770_d_n11;
        locals.var_t2_dn12 = assign9350_e7770_d_n12;
        locals.var_t2_dn17 = assign9350_e7770_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9360_e7789, assign9360_e7789_d_n0, assign9360_e7789_d_n2, assign9360_e7789_d_n6, assign9360_e7789_d_n7, assign9360_e7789_d_n10, assign9360_e7789_d_n11, assign9360_e7789_d_n12, assign9360_e7789_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9360_e7781: f64 = (10.0 * 2.220446049250313e-16);
        let (assign9360_e7787, assign9360_e7787_d_n0, assign9360_e7787_d_n2, assign9360_e7787_d_n6, assign9360_e7787_d_n7, assign9360_e7787_d_n10, assign9360_e7787_d_n11, assign9360_e7787_d_n12, assign9360_e7787_d_n17,) = {
            if (locals.var_t2 >= assign9360_e7781) {
                (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
            } else {
                let assign9360_e7786: f64 = (10.0 * 2.220446049250313e-16);
                (assign9360_e7786, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign9360_e7787, assign9360_e7787_d_n0, assign9360_e7787_d_n2, assign9360_e7787_d_n6, assign9360_e7787_d_n7, assign9360_e7787_d_n10, assign9360_e7787_d_n11, assign9360_e7787_d_n12, assign9360_e7787_d_n17,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9360_e7789;
        locals.var_t2_dn0 = assign9360_e7789_d_n0;
        locals.var_t2_dn2 = assign9360_e7789_d_n2;
        locals.var_t2_dn6 = assign9360_e7789_d_n6;
        locals.var_t2_dn7 = assign9360_e7789_d_n7;
        locals.var_t2_dn10 = assign9360_e7789_d_n10;
        locals.var_t2_dn11 = assign9360_e7789_d_n11;
        locals.var_t2_dn12 = assign9360_e7789_d_n12;
        locals.var_t2_dn17 = assign9360_e7789_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9370_e7800, assign9370_e7800_d_n0, assign9370_e7800_d_n2, assign9370_e7800_d_n6, assign9370_e7800_d_n7, assign9370_e7800_d_n10, assign9370_e7800_d_n11, assign9370_e7800_d_n12, assign9370_e7800_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9370_e7798: f64 = (locals.var_t2).sqrt();
        (assign9370_e7798, (locals.var_t2_dn0 / (2.0 * assign9370_e7798)), (locals.var_t2_dn2 / (2.0 * assign9370_e7798)), (locals.var_t2_dn6 / (2.0 * assign9370_e7798)), (locals.var_t2_dn7 / (2.0 * assign9370_e7798)), (locals.var_t2_dn10 / (2.0 * assign9370_e7798)), (locals.var_t2_dn11 / (2.0 * assign9370_e7798)), (locals.var_t2_dn12 / (2.0 * assign9370_e7798)), (locals.var_t2_dn17 / (2.0 * assign9370_e7798)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9370_e7800;
        locals.var_t2_dn0 = assign9370_e7800_d_n0;
        locals.var_t2_dn2 = assign9370_e7800_d_n2;
        locals.var_t2_dn6 = assign9370_e7800_d_n6;
        locals.var_t2_dn7 = assign9370_e7800_d_n7;
        locals.var_t2_dn10 = assign9370_e7800_d_n10;
        locals.var_t2_dn11 = assign9370_e7800_d_n11;
        locals.var_t2_dn12 = assign9370_e7800_d_n12;
        locals.var_t2_dn17 = assign9370_e7800_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9380_e7816, assign9380_e7816_d_n0, assign9380_e7816_d_n2, assign9380_e7816_d_n6, assign9380_e7816_d_n7, assign9380_e7816_d_n10, assign9380_e7816_d_n11, assign9380_e7816_d_n12, assign9380_e7816_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9380_e7810: f64 = (2.0 * locals.var_t1);
        let assign9380_e7813: f64 = (locals.var_t0 * locals.var_beta);
        let assign9380_e7814: f64 = (assign9380_e7810 + assign9380_e7813);
        (assign9380_e7814, ((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)), ((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)), ((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)), ((2.0 * locals.var_t1_dn7) + (locals.var_t0_dn7 * locals.var_beta)), ((2.0 * locals.var_t1_dn10) + ((locals.var_t0_dn10 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn10))), ((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)), ((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)), ((2.0 * locals.var_t1_dn17) + (locals.var_t0_dn17 * locals.var_beta)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign9380_e7816;
        locals.var_t3_dn0 = assign9380_e7816_d_n0;
        locals.var_t3_dn2 = assign9380_e7816_d_n2;
        locals.var_t3_dn6 = assign9380_e7816_d_n6;
        locals.var_t3_dn7 = assign9380_e7816_d_n7;
        locals.var_t3_dn10 = assign9380_e7816_d_n10;
        locals.var_t3_dn11 = assign9380_e7816_d_n11;
        locals.var_t3_dn12 = assign9380_e7816_d_n12;
        locals.var_t3_dn17 = assign9380_e7816_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign9390_e7830, assign9390_e7830_d_n0, assign9390_e7830_d_n2, assign9390_e7830_d_n6, assign9390_e7830_d_n7, assign9390_e7830_d_n10, assign9390_e7830_d_n11, assign9390_e7830_d_n12, assign9390_e7830_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9390_e7826: f64 = (locals.var_t3 - locals.var_t2);
        let assign9390_e7828: f64 = (assign9390_e7826 / 2.0);
        (assign9390_e7828, ((locals.var_t3_dn0 - locals.var_t2_dn0) / 2.0), ((locals.var_t3_dn2 - locals.var_t2_dn2) / 2.0), ((locals.var_t3_dn6 - locals.var_t2_dn6) / 2.0), ((locals.var_t3_dn7 - locals.var_t2_dn7) / 2.0), ((locals.var_t3_dn10 - locals.var_t2_dn10) / 2.0), ((locals.var_t3_dn11 - locals.var_t2_dn11) / 2.0), ((locals.var_t3_dn12 - locals.var_t2_dn12) / 2.0), ((locals.var_t3_dn17 - locals.var_t2_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia, locals.var_psb_inia_dn0, locals.var_psb_inia_dn2, locals.var_psb_inia_dn6, locals.var_psb_inia_dn7, locals.var_psb_inia_dn10, locals.var_psb_inia_dn11, locals.var_psb_inia_dn12, locals.var_psb_inia_dn17,)
    }
};
        locals.var_psb_inia = assign9390_e7830;
        locals.var_psb_inia_dn0 = assign9390_e7830_d_n0;
        locals.var_psb_inia_dn2 = assign9390_e7830_d_n2;
        locals.var_psb_inia_dn6 = assign9390_e7830_d_n6;
        locals.var_psb_inia_dn7 = assign9390_e7830_d_n7;
        locals.var_psb_inia_dn10 = assign9390_e7830_d_n10;
        locals.var_psb_inia_dn11 = assign9390_e7830_d_n11;
        locals.var_psb_inia_dn12 = assign9390_e7830_d_n12;
        locals.var_psb_inia_dn17 = assign9390_e7830_d_n17;
        locals.var_psb_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        locals: &mut StampLocals,
    ) {
        let (assign9400_e7853, assign9400_e7853_d_n0, assign9400_e7853_d_n2, assign9400_e7853_d_n6, assign9400_e7853_d_n7, assign9400_e7853_d_n10, assign9400_e7853_d_n11, assign9400_e7853_d_n12, assign9400_e7853_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign9400_e7840: f64 = (locals.var_t1 * locals.var_t1);
        let assign9400_e7842: f64 = (assign9400_e7840 / locals.var_t0);
        let assign9400_e7844: f64 = (assign9400_e7842 / locals.var_cnst1bulk);
        let assign9400_e7845: f64 = (assign9400_e7844).ln();
        let assign9400_e7849: f64 = (2.0 / locals.var_t1);
        let assign9400_e7850: f64 = (locals.var_beta + assign9400_e7849);
        let assign9400_e7851: f64 = (assign9400_e7845 / assign9400_e7850);
        (assign9400_e7851, ((((((((((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))))) / (assign9400_e7850 * assign9400_e7850)), ((((((((((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))))) / (assign9400_e7850 * assign9400_e7850)), ((((((((((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))))) / (assign9400_e7850 * assign9400_e7850)), ((((((((((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))))) / (assign9400_e7850 * assign9400_e7850)), ((((((((((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))))) / (assign9400_e7850 * assign9400_e7850)), ((((((((((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))))) / (assign9400_e7850 * assign9400_e7850)), ((((((((((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (-((2.0 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))))) / (assign9400_e7850 * assign9400_e7850)), ((((((((((((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) * locals.var_t0) - (assign9400_e7840 * locals.var_t0_dn17)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign9400_e7842 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign9400_e7844) * assign9400_e7850) - (assign9400_e7845 * (-((2.0 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))))) / (assign9400_e7850 * assign9400_e7850)),)
    } else {
        (locals.var_psb_inib, locals.var_psb_inib_dn0, locals.var_psb_inib_dn2, locals.var_psb_inib_dn6, locals.var_psb_inib_dn7, locals.var_psb_inib_dn10, locals.var_psb_inib_dn11, locals.var_psb_inib_dn12, locals.var_psb_inib_dn17,)
    }
};
        locals.var_psb_inib = assign9400_e7853;
        locals.var_psb_inib_dn0 = assign9400_e7853_d_n0;
        locals.var_psb_inib_dn2 = assign9400_e7853_d_n2;
        locals.var_psb_inib_dn6 = assign9400_e7853_d_n6;
        locals.var_psb_inib_dn7 = assign9400_e7853_d_n7;
        locals.var_psb_inib_dn10 = assign9400_e7853_d_n10;
        locals.var_psb_inib_dn11 = assign9400_e7853_d_n11;
        locals.var_psb_inib_dn12 = assign9400_e7853_d_n12;
        locals.var_psb_inib_dn17 = assign9400_e7853_d_n17;
        locals.var_psb_inib_rv = 0.0;

        let assign9410_e7856: f64 = if locals.var_psb_inia < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard171 = assign9410_e7856;
        locals.var_guard171_rv = 0.0;

        let (assign9420_e7868, assign9420_e7868_d_n0, assign9420_e7868_d_n2, assign9420_e7868_d_n6, assign9420_e7868_d_n7, assign9420_e7868_d_n10, assign9420_e7868_d_n11, assign9420_e7868_d_n12, assign9420_e7868_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 != 0.0)) {
        (locals.var_psb_inia, locals.var_psb_inia_dn0, locals.var_psb_inia_dn2, locals.var_psb_inia_dn6, locals.var_psb_inia_dn7, locals.var_psb_inia_dn10, locals.var_psb_inia_dn11, locals.var_psb_inia_dn12, locals.var_psb_inia_dn17,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9420_e7868;
        locals.var_phi_s0_bulk_dn0 = assign9420_e7868_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9420_e7868_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9420_e7868_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9420_e7868_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9420_e7868_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9420_e7868_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9420_e7868_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9420_e7868_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9430_e7885, assign9430_e7885_d_n0, assign9430_e7885_d_n2, assign9430_e7885_d_n6, assign9430_e7885_d_n7, assign9430_e7885_d_n10, assign9430_e7885_d_n11, assign9430_e7885_d_n12, assign9430_e7885_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 == 0.0)) {
        let assign9430_e7881: f64 = (locals.var_psb_inib - locals.var_psb_inia);
        let assign9430_e7883: f64 = (assign9430_e7881 - 0.0008);
        (assign9430_e7883, (locals.var_psb_inib_dn0 - locals.var_psb_inia_dn0), (locals.var_psb_inib_dn2 - locals.var_psb_inia_dn2), (locals.var_psb_inib_dn6 - locals.var_psb_inia_dn6), (locals.var_psb_inib_dn7 - locals.var_psb_inia_dn7), (locals.var_psb_inib_dn10 - locals.var_psb_inia_dn10), (locals.var_psb_inib_dn11 - locals.var_psb_inia_dn11), (locals.var_psb_inib_dn12 - locals.var_psb_inia_dn12), (locals.var_psb_inib_dn17 - locals.var_psb_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign9430_e7885;
        locals.var_tmf1_dn0 = assign9430_e7885_d_n0;
        locals.var_tmf1_dn2 = assign9430_e7885_d_n2;
        locals.var_tmf1_dn6 = assign9430_e7885_d_n6;
        locals.var_tmf1_dn7 = assign9430_e7885_d_n7;
        locals.var_tmf1_dn10 = assign9430_e7885_d_n10;
        locals.var_tmf1_dn11 = assign9430_e7885_d_n11;
        locals.var_tmf1_dn12 = assign9430_e7885_d_n12;
        locals.var_tmf1_dn17 = assign9430_e7885_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign9440_e7902, assign9440_e7902_d_n0, assign9440_e7902_d_n2, assign9440_e7902_d_n6, assign9440_e7902_d_n7, assign9440_e7902_d_n10, assign9440_e7902_d_n11, assign9440_e7902_d_n12, assign9440_e7902_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 == 0.0)) {
        let assign9440_e7898: f64 = (4.0 * locals.var_psb_inib);
        let assign9440_e7900: f64 = (assign9440_e7898 * 0.0008);
        (assign9440_e7900, ((4.0 * locals.var_psb_inib_dn0) * 0.0008), ((4.0 * locals.var_psb_inib_dn2) * 0.0008), ((4.0 * locals.var_psb_inib_dn6) * 0.0008), ((4.0 * locals.var_psb_inib_dn7) * 0.0008), ((4.0 * locals.var_psb_inib_dn10) * 0.0008), ((4.0 * locals.var_psb_inib_dn11) * 0.0008), ((4.0 * locals.var_psb_inib_dn12) * 0.0008), ((4.0 * locals.var_psb_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9440_e7902;
        locals.var_tmf2_dn0 = assign9440_e7902_d_n0;
        locals.var_tmf2_dn2 = assign9440_e7902_d_n2;
        locals.var_tmf2_dn6 = assign9440_e7902_d_n6;
        locals.var_tmf2_dn7 = assign9440_e7902_d_n7;
        locals.var_tmf2_dn10 = assign9440_e7902_d_n10;
        locals.var_tmf2_dn11 = assign9440_e7902_d_n11;
        locals.var_tmf2_dn12 = assign9440_e7902_d_n12;
        locals.var_tmf2_dn17 = assign9440_e7902_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9450_e7921, assign9450_e7921_d_n0, assign9450_e7921_d_n2, assign9450_e7921_d_n6, assign9450_e7921_d_n7, assign9450_e7921_d_n10, assign9450_e7921_d_n11, assign9450_e7921_d_n12, assign9450_e7921_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 == 0.0)) {
        let (assign9450_e7919, assign9450_e7919_d_n0, assign9450_e7919_d_n2, assign9450_e7919_d_n6, assign9450_e7919_d_n7, assign9450_e7919_d_n10, assign9450_e7919_d_n11, assign9450_e7919_d_n12, assign9450_e7919_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign9450_e7918: f64 = (-locals.var_tmf2);
                (assign9450_e7918, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign9450_e7919, assign9450_e7919_d_n0, assign9450_e7919_d_n2, assign9450_e7919_d_n6, assign9450_e7919_d_n7, assign9450_e7919_d_n10, assign9450_e7919_d_n11, assign9450_e7919_d_n12, assign9450_e7919_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9450_e7921;
        locals.var_tmf2_dn0 = assign9450_e7921_d_n0;
        locals.var_tmf2_dn2 = assign9450_e7921_d_n2;
        locals.var_tmf2_dn6 = assign9450_e7921_d_n6;
        locals.var_tmf2_dn7 = assign9450_e7921_d_n7;
        locals.var_tmf2_dn10 = assign9450_e7921_d_n10;
        locals.var_tmf2_dn11 = assign9450_e7921_d_n11;
        locals.var_tmf2_dn12 = assign9450_e7921_d_n12;
        locals.var_tmf2_dn17 = assign9450_e7921_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9460_e7939, assign9460_e7939_d_n0, assign9460_e7939_d_n2, assign9460_e7939_d_n6, assign9460_e7939_d_n7, assign9460_e7939_d_n10, assign9460_e7939_d_n11, assign9460_e7939_d_n12, assign9460_e7939_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 == 0.0)) {
        let assign9460_e7934: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9460_e7936: f64 = (assign9460_e7934 + locals.var_tmf2);
        let assign9460_e7937: f64 = (assign9460_e7936).sqrt();
        (assign9460_e7937, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9460_e7937)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9460_e7937)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign9460_e7937)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign9460_e7937)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign9460_e7937)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign9460_e7937)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign9460_e7937)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign9460_e7937)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign9460_e7939;
        locals.var_tmf2_dn0 = assign9460_e7939_d_n0;
        locals.var_tmf2_dn2 = assign9460_e7939_d_n2;
        locals.var_tmf2_dn6 = assign9460_e7939_d_n6;
        locals.var_tmf2_dn7 = assign9460_e7939_d_n7;
        locals.var_tmf2_dn10 = assign9460_e7939_d_n10;
        locals.var_tmf2_dn11 = assign9460_e7939_d_n11;
        locals.var_tmf2_dn12 = assign9460_e7939_d_n12;
        locals.var_tmf2_dn17 = assign9460_e7939_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign9470_e7958, assign9470_e7958_d_n0, assign9470_e7958_d_n2, assign9470_e7958_d_n6, assign9470_e7958_d_n7, assign9470_e7958_d_n10, assign9470_e7958_d_n11, assign9470_e7958_d_n12, assign9470_e7958_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard170 == 0.0)) && (locals.var_guard171 == 0.0)) {
        let assign9470_e7954: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9470_e7955: f64 = (0.5 * assign9470_e7954);
        let assign9470_e7956: f64 = (locals.var_psb_inib - assign9470_e7955);
        (assign9470_e7956, (locals.var_psb_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9470_e7958;
        locals.var_phi_s0_bulk_dn0 = assign9470_e7958_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9470_e7958_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9470_e7958_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9470_e7958_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9470_e7958_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9470_e7958_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9470_e7958_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9470_e7958_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9480_e7965,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign9480_e7965;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        locals: &mut StampLocals,
    ) {
        let mut assign9490_loop_guard: usize = 0;
        while {
            let assign9490_cond_e7973: f64 = if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_lp_s0 < locals.var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign9490_cond_e7973 != 0.0
        } {
            assign9490_loop_guard += 1;
            assert!(assign9490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9490_body0_e7980, assign9490_body0_e7980_d_n0, assign9490_body0_e7980_d_n2, assign9490_body0_e7980_d_n6, assign9490_body0_e7980_d_n7, assign9490_body0_e7980_d_n10, assign9490_body0_e7980_d_n11, assign9490_body0_e7980_d_n12, assign9490_body0_e7980_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        (locals.var_cnst0bulk, 0.0, 0.0, 0.0, 0.0, locals.var_cnst0bulk_dn10, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign9490_body0_e7980;
            locals.var_t1_dn0 = assign9490_body0_e7980_d_n0;
            locals.var_t1_dn2 = assign9490_body0_e7980_d_n2;
            locals.var_t1_dn6 = assign9490_body0_e7980_d_n6;
            locals.var_t1_dn7 = assign9490_body0_e7980_d_n7;
            locals.var_t1_dn10 = assign9490_body0_e7980_d_n10;
            locals.var_t1_dn11 = assign9490_body0_e7980_d_n11;
            locals.var_t1_dn12 = assign9490_body0_e7980_d_n12;
            locals.var_t1_dn17 = assign9490_body0_e7980_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign9490_body1_e7989, assign9490_body1_e7989_d_n0, assign9490_body1_e7989_d_n2, assign9490_body1_e7989_d_n6, assign9490_body1_e7989_d_n7, assign9490_body1_e7989_d_n10, assign9490_body1_e7989_d_n11, assign9490_body1_e7989_d_n12, assign9490_body1_e7989_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body1_e7987: f64 = (locals.var_beta * locals.var_phi_s0_bulk);
        (assign9490_body1_e7987, (locals.var_beta * locals.var_phi_s0_bulk_dn0), (locals.var_beta * locals.var_phi_s0_bulk_dn2), (locals.var_beta * locals.var_phi_s0_bulk_dn6), (locals.var_beta * locals.var_phi_s0_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_s0_bulk) + (locals.var_beta * locals.var_phi_s0_bulk_dn10)), (locals.var_beta * locals.var_phi_s0_bulk_dn11), (locals.var_beta * locals.var_phi_s0_bulk_dn12), (locals.var_beta * locals.var_phi_s0_bulk_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign9490_body1_e7989;
            locals.var_t2_dn0 = assign9490_body1_e7989_d_n0;
            locals.var_t2_dn2 = assign9490_body1_e7989_d_n2;
            locals.var_t2_dn6 = assign9490_body1_e7989_d_n6;
            locals.var_t2_dn7 = assign9490_body1_e7989_d_n7;
            locals.var_t2_dn10 = assign9490_body1_e7989_d_n10;
            locals.var_t2_dn11 = assign9490_body1_e7989_d_n11;
            locals.var_t2_dn12 = assign9490_body1_e7989_d_n12;
            locals.var_t2_dn17 = assign9490_body1_e7989_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign9490_body2_e7998, assign9490_body2_e7998_d_n0, assign9490_body2_e7998_d_n2, assign9490_body2_e7998_d_n6, assign9490_body2_e7998_d_n7, assign9490_body2_e7998_d_n10, assign9490_body2_e7998_d_n11, assign9490_body2_e7998_d_n12, assign9490_body2_e7998_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body2_e7995: f64 = (-locals.var_t2);
        let assign9490_body2_e7996: f64 = (assign9490_body2_e7995).exp();
        (assign9490_body2_e7996, (assign9490_body2_e7996 * (-locals.var_t2_dn0)), (assign9490_body2_e7996 * (-locals.var_t2_dn2)), (assign9490_body2_e7996 * (-locals.var_t2_dn6)), (assign9490_body2_e7996 * (-locals.var_t2_dn7)), (assign9490_body2_e7996 * (-locals.var_t2_dn10)), (assign9490_body2_e7996 * (-locals.var_t2_dn11)), (assign9490_body2_e7996 * (-locals.var_t2_dn12)), (assign9490_body2_e7996 * (-locals.var_t2_dn17)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign9490_body2_e7998;
            locals.var_t3_dn0 = assign9490_body2_e7998_d_n0;
            locals.var_t3_dn2 = assign9490_body2_e7998_d_n2;
            locals.var_t3_dn6 = assign9490_body2_e7998_d_n6;
            locals.var_t3_dn7 = assign9490_body2_e7998_d_n7;
            locals.var_t3_dn10 = assign9490_body2_e7998_d_n10;
            locals.var_t3_dn11 = assign9490_body2_e7998_d_n11;
            locals.var_t3_dn12 = assign9490_body2_e7998_d_n12;
            locals.var_t3_dn17 = assign9490_body2_e7998_d_n17;
            locals.var_t3_rv = 0.0;
            let assign9490_body3_e8001: f64 = if locals.var_phi_s0_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard172 = assign9490_body3_e8001;
            locals.var_guard172_rv = 0.0;
            let (assign9490_body4_e8013, assign9490_body4_e8013_d_n0, assign9490_body4_e8013_d_n2, assign9490_body4_e8013_d_n6, assign9490_body4_e8013_d_n7, assign9490_body4_e8013_d_n10, assign9490_body4_e8013_d_n11, assign9490_body4_e8013_d_n12, assign9490_body4_e8013_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign9490_body4_e8010: f64 = (locals.var_beta * locals.var_phi_s0_bulk);
        let assign9490_body4_e8011: f64 = (assign9490_body4_e8010).exp();
        (assign9490_body4_e8011, (assign9490_body4_e8011 * (locals.var_beta * locals.var_phi_s0_bulk_dn0)), (assign9490_body4_e8011 * (locals.var_beta * locals.var_phi_s0_bulk_dn2)), (assign9490_body4_e8011 * (locals.var_beta * locals.var_phi_s0_bulk_dn6)), (assign9490_body4_e8011 * (locals.var_beta * locals.var_phi_s0_bulk_dn7)), (assign9490_body4_e8011 * ((locals.var_beta_dn10 * locals.var_phi_s0_bulk) + (locals.var_beta * locals.var_phi_s0_bulk_dn10))), (assign9490_body4_e8011 * (locals.var_beta * locals.var_phi_s0_bulk_dn11)), (assign9490_body4_e8011 * (locals.var_beta * locals.var_phi_s0_bulk_dn12)), (assign9490_body4_e8011 * (locals.var_beta * locals.var_phi_s0_bulk_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign9490_body4_e8013;
            locals.var_t0_dn0 = assign9490_body4_e8013_d_n0;
            locals.var_t0_dn2 = assign9490_body4_e8013_d_n2;
            locals.var_t0_dn6 = assign9490_body4_e8013_d_n6;
            locals.var_t0_dn7 = assign9490_body4_e8013_d_n7;
            locals.var_t0_dn10 = assign9490_body4_e8013_d_n10;
            locals.var_t0_dn11 = assign9490_body4_e8013_d_n11;
            locals.var_t0_dn12 = assign9490_body4_e8013_d_n12;
            locals.var_t0_dn17 = assign9490_body4_e8013_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign9490_body5_e8036, assign9490_body5_e8036_d_n0, assign9490_body5_e8036_d_n2, assign9490_body5_e8036_d_n6, assign9490_body5_e8036_d_n7, assign9490_body5_e8036_d_n10, assign9490_body5_e8036_d_n11, assign9490_body5_e8036_d_n12, assign9490_body5_e8036_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign9490_body5_e8021: f64 = (-locals.var_t1);
        let assign9490_body5_e8024: f64 = (locals.var_t3 + locals.var_t2);
        let assign9490_body5_e8026: f64 = (assign9490_body5_e8024 - 1.0);
        let assign9490_body5_e8030: f64 = (locals.var_t0 - 1.0);
        let assign9490_body5_e8031: f64 = (locals.var_cnst1bulk * assign9490_body5_e8030);
        let assign9490_body5_e8032: f64 = (assign9490_body5_e8026 + assign9490_body5_e8031);
        let assign9490_body5_e8033: f64 = (assign9490_body5_e8032).sqrt();
        let assign9490_body5_e8034: f64 = (assign9490_body5_e8021 * assign9490_body5_e8033);
        (assign9490_body5_e8034, (((-locals.var_t1_dn0) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn0 + locals.var_t2_dn0) + ((locals.var_cnst1bulk_dn0 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn0))) / (2.0 * assign9490_body5_e8033)))), (((-locals.var_t1_dn2) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn2 + locals.var_t2_dn2) + ((locals.var_cnst1bulk_dn2 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn2))) / (2.0 * assign9490_body5_e8033)))), (((-locals.var_t1_dn6) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn6 + locals.var_t2_dn6) + ((locals.var_cnst1bulk_dn6 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn6))) / (2.0 * assign9490_body5_e8033)))), (((-locals.var_t1_dn7) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn7 + locals.var_t2_dn7) + ((locals.var_cnst1bulk_dn7 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn7))) / (2.0 * assign9490_body5_e8033)))), (((-locals.var_t1_dn10) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn10 + locals.var_t2_dn10) + ((locals.var_cnst1bulk_dn10 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn10))) / (2.0 * assign9490_body5_e8033)))), (((-locals.var_t1_dn11) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn11 + locals.var_t2_dn11) + ((locals.var_cnst1bulk_dn11 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn11))) / (2.0 * assign9490_body5_e8033)))), (((-locals.var_t1_dn12) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn12 + locals.var_t2_dn12) + ((locals.var_cnst1bulk_dn12 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn12))) / (2.0 * assign9490_body5_e8033)))), (((-locals.var_t1_dn17) * assign9490_body5_e8033) + (assign9490_body5_e8021 * (((locals.var_t3_dn17 + locals.var_t2_dn17) + ((locals.var_cnst1bulk_dn17 * assign9490_body5_e8030) + (locals.var_cnst1bulk * locals.var_t0_dn17))) / (2.0 * assign9490_body5_e8033)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
            locals.var_t4 = assign9490_body5_e8036;
            locals.var_t4_dn0 = assign9490_body5_e8036_d_n0;
            locals.var_t4_dn2 = assign9490_body5_e8036_d_n2;
            locals.var_t4_dn6 = assign9490_body5_e8036_d_n6;
            locals.var_t4_dn7 = assign9490_body5_e8036_d_n7;
            locals.var_t4_dn10 = assign9490_body5_e8036_d_n10;
            locals.var_t4_dn11 = assign9490_body5_e8036_d_n11;
            locals.var_t4_dn12 = assign9490_body5_e8036_d_n12;
            locals.var_t4_dn17 = assign9490_body5_e8036_d_n17;
            locals.var_t4_rv = 0.0;
            let (assign9490_body6_e8056, assign9490_body6_e8056_d_n0, assign9490_body6_e8056_d_n2, assign9490_body6_e8056_d_n6, assign9490_body6_e8056_d_n7, assign9490_body6_e8056_d_n10, assign9490_body6_e8056_d_n11, assign9490_body6_e8056_d_n12, assign9490_body6_e8056_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard172 != 0.0)) {
        let assign9490_body6_e8045: f64 = (locals.var_c0bulk / locals.var_t4);
        let assign9490_body6_e8047: f64 = (-locals.var_t3);
        let assign9490_body6_e8049: f64 = (assign9490_body6_e8047 + 1.0);
        let assign9490_body6_e8052: f64 = (locals.var_cnst1bulk * locals.var_t0);
        let assign9490_body6_e8053: f64 = (assign9490_body6_e8049 + assign9490_body6_e8052);
        let assign9490_body6_e8054: f64 = (assign9490_body6_e8045 * assign9490_body6_e8053);
        (assign9490_body6_e8054, (((-((locals.var_c0bulk * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn0))))), (((-((locals.var_c0bulk * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn2))))), (((-((locals.var_c0bulk * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn6))))), (((-((locals.var_c0bulk * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn7))))), (((-((locals.var_c0bulk * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn10))))), (((-((locals.var_c0bulk * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn11))))), (((-((locals.var_c0bulk * locals.var_t4_dn12) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn12))))), (((-((locals.var_c0bulk * locals.var_t4_dn17) / (locals.var_t4 * locals.var_t4))) * assign9490_body6_e8053) + (assign9490_body6_e8045 * ((-locals.var_t3_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn17))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
            locals.var_t5 = assign9490_body6_e8056;
            locals.var_t5_dn0 = assign9490_body6_e8056_d_n0;
            locals.var_t5_dn2 = assign9490_body6_e8056_d_n2;
            locals.var_t5_dn6 = assign9490_body6_e8056_d_n6;
            locals.var_t5_dn7 = assign9490_body6_e8056_d_n7;
            locals.var_t5_dn10 = assign9490_body6_e8056_d_n10;
            locals.var_t5_dn11 = assign9490_body6_e8056_d_n11;
            locals.var_t5_dn12 = assign9490_body6_e8056_d_n12;
            locals.var_t5_dn17 = assign9490_body6_e8056_d_n17;
            locals.var_t5_rv = 0.0;
            let assign9490_body7_e8059: f64 = (-1e-9);
            let assign9490_body7_e8060: f64 = if locals.var_phi_s0_bulk < assign9490_body7_e8059 { 1.0 } else { 0.0 };
            locals.var_guard173 = assign9490_body7_e8060;
            locals.var_guard173_rv = 0.0;
            let (assign9490_body8_e8079, assign9490_body8_e8079_d_n0, assign9490_body8_e8079_d_n2, assign9490_body8_e8079_d_n6, assign9490_body8_e8079_d_n7, assign9490_body8_e8079_d_n10, assign9490_body8_e8079_d_n11, assign9490_body8_e8079_d_n12, assign9490_body8_e8079_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard172 == 0.0)) && (locals.var_guard173 != 0.0)) {
        let assign9490_body8_e8073: f64 = (locals.var_t3 + locals.var_t2);
        let assign9490_body8_e8075: f64 = (assign9490_body8_e8073 - 1.0);
        let assign9490_body8_e8076: f64 = (assign9490_body8_e8075).sqrt();
        let assign9490_body8_e8077: f64 = (locals.var_t1 * assign9490_body8_e8076);
        (assign9490_body8_e8077, ((locals.var_t1_dn0 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn0 + locals.var_t2_dn0) / (2.0 * assign9490_body8_e8076)))), ((locals.var_t1_dn2 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn2 + locals.var_t2_dn2) / (2.0 * assign9490_body8_e8076)))), ((locals.var_t1_dn6 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn6 + locals.var_t2_dn6) / (2.0 * assign9490_body8_e8076)))), ((locals.var_t1_dn7 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn7 + locals.var_t2_dn7) / (2.0 * assign9490_body8_e8076)))), ((locals.var_t1_dn10 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn10 + locals.var_t2_dn10) / (2.0 * assign9490_body8_e8076)))), ((locals.var_t1_dn11 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn11 + locals.var_t2_dn11) / (2.0 * assign9490_body8_e8076)))), ((locals.var_t1_dn12 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn12 + locals.var_t2_dn12) / (2.0 * assign9490_body8_e8076)))), ((locals.var_t1_dn17 * assign9490_body8_e8076) + (locals.var_t1 * ((locals.var_t3_dn17 + locals.var_t2_dn17) / (2.0 * assign9490_body8_e8076)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
            locals.var_t4 = assign9490_body8_e8079;
            locals.var_t4_dn0 = assign9490_body8_e8079_d_n0;
            locals.var_t4_dn2 = assign9490_body8_e8079_d_n2;
            locals.var_t4_dn6 = assign9490_body8_e8079_d_n6;
            locals.var_t4_dn7 = assign9490_body8_e8079_d_n7;
            locals.var_t4_dn10 = assign9490_body8_e8079_d_n10;
            locals.var_t4_dn11 = assign9490_body8_e8079_d_n11;
            locals.var_t4_dn12 = assign9490_body8_e8079_d_n12;
            locals.var_t4_dn17 = assign9490_body8_e8079_d_n17;
            locals.var_t4_rv = 0.0;
            let (assign9490_body9_e8098, assign9490_body9_e8098_d_n0, assign9490_body9_e8098_d_n2, assign9490_body9_e8098_d_n6, assign9490_body9_e8098_d_n7, assign9490_body9_e8098_d_n10, assign9490_body9_e8098_d_n11, assign9490_body9_e8098_d_n12, assign9490_body9_e8098_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard172 == 0.0)) && (locals.var_guard173 != 0.0)) {
        let assign9490_body9_e8091: f64 = (locals.var_c0bulk / locals.var_t4);
        let assign9490_body9_e8093: f64 = (-locals.var_t3);
        let assign9490_body9_e8095: f64 = (assign9490_body9_e8093 + 1.0);
        let assign9490_body9_e8096: f64 = (assign9490_body9_e8091 * assign9490_body9_e8095);
        (assign9490_body9_e8096, (((-((locals.var_c0bulk * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn0))), (((-((locals.var_c0bulk * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn2))), (((-((locals.var_c0bulk * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn6))), (((-((locals.var_c0bulk * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn7))), (((-((locals.var_c0bulk * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn10))), (((-((locals.var_c0bulk * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn11))), (((-((locals.var_c0bulk * locals.var_t4_dn12) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn12))), (((-((locals.var_c0bulk * locals.var_t4_dn17) / (locals.var_t4 * locals.var_t4))) * assign9490_body9_e8095) + (assign9490_body9_e8091 * (-locals.var_t3_dn17))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
            locals.var_t5 = assign9490_body9_e8098;
            locals.var_t5_dn0 = assign9490_body9_e8098_d_n0;
            locals.var_t5_dn2 = assign9490_body9_e8098_d_n2;
            locals.var_t5_dn6 = assign9490_body9_e8098_d_n6;
            locals.var_t5_dn7 = assign9490_body9_e8098_d_n7;
            locals.var_t5_dn10 = assign9490_body9_e8098_d_n10;
            locals.var_t5_dn11 = assign9490_body9_e8098_d_n11;
            locals.var_t5_dn12 = assign9490_body9_e8098_d_n12;
            locals.var_t5_dn17 = assign9490_body9_e8098_d_n17;
            locals.var_t5_rv = 0.0;
            let (assign9490_body10_e8119, assign9490_body10_e8119_d_n0, assign9490_body10_e8119_d_n2, assign9490_body10_e8119_d_n6, assign9490_body10_e8119_d_n7, assign9490_body10_e8119_d_n10, assign9490_body10_e8119_d_n11, assign9490_body10_e8119_d_n12, assign9490_body10_e8119_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard172 == 0.0)) && (locals.var_guard173 == 0.0)) {
        let assign9490_body10_e8111: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign9490_body10_e8112: f64 = (assign9490_body10_e8111).sqrt();
        let assign9490_body10_e8113: f64 = (-assign9490_body10_e8112);
        let assign9490_body10_e8115: f64 = (assign9490_body10_e8113 * locals.var_beta);
        let assign9490_body10_e8117: f64 = (assign9490_body10_e8115 * locals.var_phi_s0_bulk);
        (assign9490_body10_e8117, (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn0), (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn2), (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn6), (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign9490_body10_e8112))) * locals.var_beta) + (assign9490_body10_e8113 * locals.var_beta_dn10)) * locals.var_phi_s0_bulk) + (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn10)), (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn11), (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn12), (assign9490_body10_e8115 * locals.var_phi_s0_bulk_dn17),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
            locals.var_t4 = assign9490_body10_e8119;
            locals.var_t4_dn0 = assign9490_body10_e8119_d_n0;
            locals.var_t4_dn2 = assign9490_body10_e8119_d_n2;
            locals.var_t4_dn6 = assign9490_body10_e8119_d_n6;
            locals.var_t4_dn7 = assign9490_body10_e8119_d_n7;
            locals.var_t4_dn10 = assign9490_body10_e8119_d_n10;
            locals.var_t4_dn11 = assign9490_body10_e8119_d_n11;
            locals.var_t4_dn12 = assign9490_body10_e8119_d_n12;
            locals.var_t4_dn17 = assign9490_body10_e8119_d_n17;
            locals.var_t4_rv = 0.0;
            let (assign9490_body11_e8136, assign9490_body11_e8136_d_n0, assign9490_body11_e8136_d_n2, assign9490_body11_e8136_d_n6, assign9490_body11_e8136_d_n7, assign9490_body11_e8136_d_n10, assign9490_body11_e8136_d_n11, assign9490_body11_e8136_d_n12, assign9490_body11_e8136_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard172 == 0.0)) && (locals.var_guard173 == 0.0)) {
        let assign9490_body11_e8132: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign9490_body11_e8133: f64 = (assign9490_body11_e8132).sqrt();
        let assign9490_body11_e8134: f64 = (-assign9490_body11_e8133);
        (assign9490_body11_e8134, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign9490_body11_e8133))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
            locals.var_t5 = assign9490_body11_e8136;
            locals.var_t5_dn0 = assign9490_body11_e8136_d_n0;
            locals.var_t5_dn2 = assign9490_body11_e8136_d_n2;
            locals.var_t5_dn6 = assign9490_body11_e8136_d_n6;
            locals.var_t5_dn7 = assign9490_body11_e8136_d_n7;
            locals.var_t5_dn10 = assign9490_body11_e8136_d_n10;
            locals.var_t5_dn11 = assign9490_body11_e8136_d_n11;
            locals.var_t5_dn12 = assign9490_body11_e8136_d_n12;
            locals.var_t5_dn17 = assign9490_body11_e8136_d_n17;
            locals.var_t5_rv = 0.0;
            let (assign9490_body12_e8152, assign9490_body12_e8152_d_n0, assign9490_body12_e8152_d_n2, assign9490_body12_e8152_d_n6, assign9490_body12_e8152_d_n7, assign9490_body12_e8152_d_n10, assign9490_body12_e8152_d_n11, assign9490_body12_e8152_d_n12, assign9490_body12_e8152_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body12_e8143: f64 = (locals.var_t4 * locals.var_t4);
        let assign9490_body12_e8146: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign9490_body12_e8148: f64 = (assign9490_body12_e8146 * locals.var_q_fd_dlt1);
        let assign9490_body12_e8149: f64 = (assign9490_body12_e8143 + assign9490_body12_e8148);
        let assign9490_body12_e8150: f64 = (assign9490_body12_e8149).sqrt();
        (assign9490_body12_e8150, ((((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign9490_body12_e8150)), ((((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign9490_body12_e8150)), ((((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign9490_body12_e8150)), ((((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign9490_body12_e8150)), ((((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign9490_body12_e8150)), ((((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign9490_body12_e8150)), ((((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign9490_body12_e8150)), ((((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign9490_body12_e8146 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign9490_body12_e8150)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9490_body12_e8152;
            locals.var_tmf2_dn0 = assign9490_body12_e8152_d_n0;
            locals.var_tmf2_dn2 = assign9490_body12_e8152_d_n2;
            locals.var_tmf2_dn6 = assign9490_body12_e8152_d_n6;
            locals.var_tmf2_dn7 = assign9490_body12_e8152_d_n7;
            locals.var_tmf2_dn10 = assign9490_body12_e8152_d_n10;
            locals.var_tmf2_dn11 = assign9490_body12_e8152_d_n11;
            locals.var_tmf2_dn12 = assign9490_body12_e8152_d_n12;
            locals.var_tmf2_dn17 = assign9490_body12_e8152_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9490_body13_e8165, assign9490_body13_e8165_d_n0, assign9490_body13_e8165_d_n2, assign9490_body13_e8165_d_n6, assign9490_body13_e8165_d_n7, assign9490_body13_e8165_d_n10, assign9490_body13_e8165_d_n11, assign9490_body13_e8165_d_n12, assign9490_body13_e8165_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body13_e8161: f64 = (locals.var_t4 / locals.var_tmf2);
        let assign9490_body13_e8162: f64 = (1.0 + assign9490_body13_e8161);
        let assign9490_body13_e8163: f64 = (0.5 * assign9490_body13_e8162);
        (assign9490_body13_e8163, (0.5 * (((locals.var_t4_dn0 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn2 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn6 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn7 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn10 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn11 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn12 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn17 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9490_body13_e8165;
            locals.var_t7_dn0 = assign9490_body13_e8165_d_n0;
            locals.var_t7_dn2 = assign9490_body13_e8165_d_n2;
            locals.var_t7_dn6 = assign9490_body13_e8165_d_n6;
            locals.var_t7_dn7 = assign9490_body13_e8165_d_n7;
            locals.var_t7_dn10 = assign9490_body13_e8165_d_n10;
            locals.var_t7_dn11 = assign9490_body13_e8165_d_n11;
            locals.var_t7_dn12 = assign9490_body13_e8165_d_n12;
            locals.var_t7_dn17 = assign9490_body13_e8165_d_n17;
            locals.var_t7_rv = 0.0;
            let (assign9490_body14_e8180, assign9490_body14_e8180_d_n0, assign9490_body14_e8180_d_n2, assign9490_body14_e8180_d_n6, assign9490_body14_e8180_d_n7, assign9490_body14_e8180_d_n10, assign9490_body14_e8180_d_n11, assign9490_body14_e8180_d_n12, assign9490_body14_e8180_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body14_e8173: f64 = (locals.var_t4 + locals.var_tmf2);
        let assign9490_body14_e8174: f64 = (0.5 * assign9490_body14_e8173);
        let assign9490_body14_e8177: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign9490_body14_e8178: f64 = (assign9490_body14_e8174 + assign9490_body14_e8177);
        (assign9490_body14_e8178, ((0.5 * (locals.var_t4_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9490_body14_e8180;
            locals.var_t6_dn0 = assign9490_body14_e8180_d_n0;
            locals.var_t6_dn2 = assign9490_body14_e8180_d_n2;
            locals.var_t6_dn6 = assign9490_body14_e8180_d_n6;
            locals.var_t6_dn7 = assign9490_body14_e8180_d_n7;
            locals.var_t6_dn10 = assign9490_body14_e8180_d_n10;
            locals.var_t6_dn11 = assign9490_body14_e8180_d_n11;
            locals.var_t6_dn12 = assign9490_body14_e8180_d_n12;
            locals.var_t6_dn17 = assign9490_body14_e8180_d_n17;
            locals.var_t6_rv = 0.0;
            let assign9490_body15_e8183: f64 = if locals.var_t6 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard174 = assign9490_body15_e8183;
            locals.var_guard174_rv = 0.0;
            let (assign9490_body16_e8192, assign9490_body16_e8192_d_n0, assign9490_body16_e8192_d_n2, assign9490_body16_e8192_d_n6, assign9490_body16_e8192_d_n7, assign9490_body16_e8192_d_n10, assign9490_body16_e8192_d_n11, assign9490_body16_e8192_d_n12, assign9490_body16_e8192_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard174 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9490_body16_e8192;
            locals.var_t6_dn0 = assign9490_body16_e8192_d_n0;
            locals.var_t6_dn2 = assign9490_body16_e8192_d_n2;
            locals.var_t6_dn6 = assign9490_body16_e8192_d_n6;
            locals.var_t6_dn7 = assign9490_body16_e8192_d_n7;
            locals.var_t6_dn10 = assign9490_body16_e8192_d_n10;
            locals.var_t6_dn11 = assign9490_body16_e8192_d_n11;
            locals.var_t6_dn12 = assign9490_body16_e8192_d_n12;
            locals.var_t6_dn17 = assign9490_body16_e8192_d_n17;
            locals.var_t6_rv = 0.0;
            let (assign9490_body17_e8201, assign9490_body17_e8201_d_n0, assign9490_body17_e8201_d_n2, assign9490_body17_e8201_d_n6, assign9490_body17_e8201_d_n7, assign9490_body17_e8201_d_n10, assign9490_body17_e8201_d_n11, assign9490_body17_e8201_d_n12, assign9490_body17_e8201_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard174 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9490_body17_e8201;
            locals.var_t7_dn0 = assign9490_body17_e8201_d_n0;
            locals.var_t7_dn2 = assign9490_body17_e8201_d_n2;
            locals.var_t7_dn6 = assign9490_body17_e8201_d_n6;
            locals.var_t7_dn7 = assign9490_body17_e8201_d_n7;
            locals.var_t7_dn10 = assign9490_body17_e8201_d_n10;
            locals.var_t7_dn11 = assign9490_body17_e8201_d_n11;
            locals.var_t7_dn12 = assign9490_body17_e8201_d_n12;
            locals.var_t7_dn17 = assign9490_body17_e8201_d_n17;
            locals.var_t7_rv = 0.0;
            let (assign9490_body18_e8213, assign9490_body18_e8213_d_n0, assign9490_body18_e8213_d_n2, assign9490_body18_e8213_d_n6, assign9490_body18_e8213_d_n7, assign9490_body18_e8213_d_n10, assign9490_body18_e8213_d_n11, assign9490_body18_e8213_d_n12, assign9490_body18_e8213_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body18_e8207: f64 = (-locals.var_q_fd_soi);
        let assign9490_body18_e8209: f64 = (assign9490_body18_e8207 - locals.var_t6);
        let assign9490_body18_e8211: f64 = (assign9490_body18_e8209 - locals.var_q_fd_dlt2);
        (assign9490_body18_e8211, (((-locals.var_q_fd_soi_dn0) - locals.var_t6_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign9490_body18_e8213;
            locals.var_tmf1_dn0 = assign9490_body18_e8213_d_n0;
            locals.var_tmf1_dn2 = assign9490_body18_e8213_d_n2;
            locals.var_tmf1_dn6 = assign9490_body18_e8213_d_n6;
            locals.var_tmf1_dn7 = assign9490_body18_e8213_d_n7;
            locals.var_tmf1_dn10 = assign9490_body18_e8213_d_n10;
            locals.var_tmf1_dn11 = assign9490_body18_e8213_d_n11;
            locals.var_tmf1_dn12 = assign9490_body18_e8213_d_n12;
            locals.var_tmf1_dn17 = assign9490_body18_e8213_d_n17;
            locals.var_tmf1_rv = 0.0;
            let (assign9490_body19_e8225, assign9490_body19_e8225_d_n0, assign9490_body19_e8225_d_n2, assign9490_body19_e8225_d_n6, assign9490_body19_e8225_d_n7, assign9490_body19_e8225_d_n10, assign9490_body19_e8225_d_n11, assign9490_body19_e8225_d_n12, assign9490_body19_e8225_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body19_e8220: f64 = (-locals.var_q_fd_soi);
        let assign9490_body19_e8221: f64 = (4.0 * assign9490_body19_e8220);
        let assign9490_body19_e8223: f64 = (assign9490_body19_e8221 * locals.var_q_fd_dlt2);
        (assign9490_body19_e8223, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign9490_body19_e8221 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9490_body19_e8225;
            locals.var_tmf2_dn0 = assign9490_body19_e8225_d_n0;
            locals.var_tmf2_dn2 = assign9490_body19_e8225_d_n2;
            locals.var_tmf2_dn6 = assign9490_body19_e8225_d_n6;
            locals.var_tmf2_dn7 = assign9490_body19_e8225_d_n7;
            locals.var_tmf2_dn10 = assign9490_body19_e8225_d_n10;
            locals.var_tmf2_dn11 = assign9490_body19_e8225_d_n11;
            locals.var_tmf2_dn12 = assign9490_body19_e8225_d_n12;
            locals.var_tmf2_dn17 = assign9490_body19_e8225_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9490_body20_e8238, assign9490_body20_e8238_d_n0, assign9490_body20_e8238_d_n2, assign9490_body20_e8238_d_n6, assign9490_body20_e8238_d_n7, assign9490_body20_e8238_d_n10, assign9490_body20_e8238_d_n11, assign9490_body20_e8238_d_n12, assign9490_body20_e8238_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let (assign9490_body20_e8236, assign9490_body20_e8236_d_n0, assign9490_body20_e8236_d_n2, assign9490_body20_e8236_d_n6, assign9490_body20_e8236_d_n7, assign9490_body20_e8236_d_n10, assign9490_body20_e8236_d_n11, assign9490_body20_e8236_d_n12, assign9490_body20_e8236_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign9490_body20_e8235: f64 = (-locals.var_tmf2);
                (assign9490_body20_e8235, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign9490_body20_e8236, assign9490_body20_e8236_d_n0, assign9490_body20_e8236_d_n2, assign9490_body20_e8236_d_n6, assign9490_body20_e8236_d_n7, assign9490_body20_e8236_d_n10, assign9490_body20_e8236_d_n11, assign9490_body20_e8236_d_n12, assign9490_body20_e8236_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9490_body20_e8238;
            locals.var_tmf2_dn0 = assign9490_body20_e8238_d_n0;
            locals.var_tmf2_dn2 = assign9490_body20_e8238_d_n2;
            locals.var_tmf2_dn6 = assign9490_body20_e8238_d_n6;
            locals.var_tmf2_dn7 = assign9490_body20_e8238_d_n7;
            locals.var_tmf2_dn10 = assign9490_body20_e8238_d_n10;
            locals.var_tmf2_dn11 = assign9490_body20_e8238_d_n11;
            locals.var_tmf2_dn12 = assign9490_body20_e8238_d_n12;
            locals.var_tmf2_dn17 = assign9490_body20_e8238_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9490_body21_e8250, assign9490_body21_e8250_d_n0, assign9490_body21_e8250_d_n2, assign9490_body21_e8250_d_n6, assign9490_body21_e8250_d_n7, assign9490_body21_e8250_d_n10, assign9490_body21_e8250_d_n11, assign9490_body21_e8250_d_n12, assign9490_body21_e8250_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body21_e8245: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign9490_body21_e8247: f64 = (assign9490_body21_e8245 + locals.var_tmf2);
        let assign9490_body21_e8248: f64 = (assign9490_body21_e8247).sqrt();
        (assign9490_body21_e8248, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign9490_body21_e8248)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign9490_body21_e8248)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign9490_body21_e8248)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign9490_body21_e8248)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign9490_body21_e8248)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign9490_body21_e8248)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign9490_body21_e8248)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign9490_body21_e8248)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign9490_body21_e8250;
            locals.var_tmf2_dn0 = assign9490_body21_e8250_d_n0;
            locals.var_tmf2_dn2 = assign9490_body21_e8250_d_n2;
            locals.var_tmf2_dn6 = assign9490_body21_e8250_d_n6;
            locals.var_tmf2_dn7 = assign9490_body21_e8250_d_n7;
            locals.var_tmf2_dn10 = assign9490_body21_e8250_d_n10;
            locals.var_tmf2_dn11 = assign9490_body21_e8250_d_n11;
            locals.var_tmf2_dn12 = assign9490_body21_e8250_d_n12;
            locals.var_tmf2_dn17 = assign9490_body21_e8250_d_n17;
            locals.var_tmf2_rv = 0.0;
            let (assign9490_body22_e8263, assign9490_body22_e8263_d_n0, assign9490_body22_e8263_d_n2, assign9490_body22_e8263_d_n6, assign9490_body22_e8263_d_n7, assign9490_body22_e8263_d_n10, assign9490_body22_e8263_d_n11, assign9490_body22_e8263_d_n12, assign9490_body22_e8263_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body22_e8259: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign9490_body22_e8260: f64 = (1.0 + assign9490_body22_e8259);
        let assign9490_body22_e8261: f64 = (0.5 * assign9490_body22_e8260);
        (assign9490_body22_e8261, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign9490_body22_e8263;
            locals.var_t8_dn0 = assign9490_body22_e8263_d_n0;
            locals.var_t8_dn2 = assign9490_body22_e8263_d_n2;
            locals.var_t8_dn6 = assign9490_body22_e8263_d_n6;
            locals.var_t8_dn7 = assign9490_body22_e8263_d_n7;
            locals.var_t8_dn10 = assign9490_body22_e8263_d_n10;
            locals.var_t8_dn11 = assign9490_body22_e8263_d_n11;
            locals.var_t8_dn12 = assign9490_body22_e8263_d_n12;
            locals.var_t8_dn17 = assign9490_body22_e8263_d_n17;
            locals.var_t8_rv = 0.0;
            let (assign9490_body23_e8277, assign9490_body23_e8277_d_n0, assign9490_body23_e8277_d_n2, assign9490_body23_e8277_d_n6, assign9490_body23_e8277_d_n7, assign9490_body23_e8277_d_n10, assign9490_body23_e8277_d_n11, assign9490_body23_e8277_d_n12, assign9490_body23_e8277_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body23_e8269: f64 = (-locals.var_q_fd_soi);
        let assign9490_body23_e8273: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign9490_body23_e8274: f64 = (0.5 * assign9490_body23_e8273);
        let assign9490_body23_e8275: f64 = (assign9490_body23_e8269 - assign9490_body23_e8274);
        (assign9490_body23_e8275, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9490_body23_e8277;
            locals.var_t6_dn0 = assign9490_body23_e8277_d_n0;
            locals.var_t6_dn2 = assign9490_body23_e8277_d_n2;
            locals.var_t6_dn6 = assign9490_body23_e8277_d_n6;
            locals.var_t6_dn7 = assign9490_body23_e8277_d_n7;
            locals.var_t6_dn10 = assign9490_body23_e8277_d_n10;
            locals.var_t6_dn11 = assign9490_body23_e8277_d_n11;
            locals.var_t6_dn12 = assign9490_body23_e8277_d_n12;
            locals.var_t6_dn17 = assign9490_body23_e8277_d_n17;
            locals.var_t6_rv = 0.0;
            let (assign9490_body24_e8288, assign9490_body24_e8288_d_n0, assign9490_body24_e8288_d_n2, assign9490_body24_e8288_d_n6, assign9490_body24_e8288_d_n7, assign9490_body24_e8288_d_n10, assign9490_body24_e8288_d_n11, assign9490_body24_e8288_d_n12, assign9490_body24_e8288_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body24_e8285: f64 = (locals.var_t5 * locals.var_t8);
        let assign9490_body24_e8286: f64 = (locals.var_t7 * assign9490_body24_e8285);
        (assign9490_body24_e8286, ((locals.var_t7_dn0 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn0 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn0)))), ((locals.var_t7_dn2 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn2 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn2)))), ((locals.var_t7_dn6 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn6 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn6)))), ((locals.var_t7_dn7 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn7 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn7)))), ((locals.var_t7_dn10 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn10 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn10)))), ((locals.var_t7_dn11 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn11 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn11)))), ((locals.var_t7_dn12 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn12 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn12)))), ((locals.var_t7_dn17 * assign9490_body24_e8285) + (locals.var_t7 * ((locals.var_t5_dn17 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9490_body24_e8288;
            locals.var_t7_dn0 = assign9490_body24_e8288_d_n0;
            locals.var_t7_dn2 = assign9490_body24_e8288_d_n2;
            locals.var_t7_dn6 = assign9490_body24_e8288_d_n6;
            locals.var_t7_dn7 = assign9490_body24_e8288_d_n7;
            locals.var_t7_dn10 = assign9490_body24_e8288_d_n10;
            locals.var_t7_dn11 = assign9490_body24_e8288_d_n11;
            locals.var_t7_dn12 = assign9490_body24_e8288_d_n12;
            locals.var_t7_dn17 = assign9490_body24_e8288_d_n17;
            locals.var_t7_rv = 0.0;
            let (assign9490_body25_e8305, assign9490_body25_e8305_d_n0, assign9490_body25_e8305_d_n2, assign9490_body25_e8305_d_n6, assign9490_body25_e8305_d_n7, assign9490_body25_e8305_d_n10, assign9490_body25_e8305_d_n11, assign9490_body25_e8305_d_n12, assign9490_body25_e8305_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body25_e8295: f64 = (locals.var_t6 * locals.var_t6);
        let assign9490_body25_e8297: f64 = (assign9490_body25_e8295 / 2.0);
        let assign9490_body25_e8299: f64 = (assign9490_body25_e8297 / 1.034943e-10);
        let assign9490_body25_e8301: f64 = (assign9490_body25_e8299 / 1.6021918e-19);
        let assign9490_body25_e8303: f64 = (assign9490_body25_e8301 / locals.var_uc_nsubs);
        (assign9490_body25_e8303, ((((((((locals.var_t6_dn0 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn2 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn12 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6_dn17 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign9490_body25_e8301 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign9490_body25_e8305;
            locals.var_phi_b_dep_dn0 = assign9490_body25_e8305_d_n0;
            locals.var_phi_b_dep_dn2 = assign9490_body25_e8305_d_n2;
            locals.var_phi_b_dep_dn6 = assign9490_body25_e8305_d_n6;
            locals.var_phi_b_dep_dn7 = assign9490_body25_e8305_d_n7;
            locals.var_phi_b_dep_dn10 = assign9490_body25_e8305_d_n10;
            locals.var_phi_b_dep_dn11 = assign9490_body25_e8305_d_n11;
            locals.var_phi_b_dep_dn12 = assign9490_body25_e8305_d_n12;
            locals.var_phi_b_dep_dn17 = assign9490_body25_e8305_d_n17;
            locals.var_phi_b_dep_rv = 0.0;
            let (assign9490_body26_e8318, assign9490_body26_e8318_d_n0, assign9490_body26_e8318_d_n2, assign9490_body26_e8318_d_n6, assign9490_body26_e8318_d_n7, assign9490_body26_e8318_d_n10, assign9490_body26_e8318_d_n11, assign9490_body26_e8318_d_n12, assign9490_body26_e8318_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body26_e8312: f64 = (2.0 * locals.var_phi_b_dep);
        let assign9490_body26_e8314: f64 = (assign9490_body26_e8312 * locals.var_t7);
        let assign9490_body26_e8316: f64 = (assign9490_body26_e8314 / locals.var_t6);
        (assign9490_body26_e8316, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn0)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn2)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn6)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn7)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn10)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn11)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn12)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn12)) / (locals.var_t6 * locals.var_t6)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7) + (assign9490_body26_e8312 * locals.var_t7_dn17)) * locals.var_t6) - (assign9490_body26_e8314 * locals.var_t6_dn17)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign9490_body26_e8318;
            locals.var_phi_b_dep_dpsb_dn0 = assign9490_body26_e8318_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign9490_body26_e8318_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign9490_body26_e8318_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign9490_body26_e8318_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign9490_body26_e8318_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign9490_body26_e8318_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign9490_body26_e8318_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign9490_body26_e8318_d_n17;
            locals.var_phi_b_dep_dpsb_rv = 0.0;
            let (assign9490_body27_e8362, assign9490_body27_e8362_d_n0, assign9490_body27_e8362_d_n2, assign9490_body27_e8362_d_n6, assign9490_body27_e8362_d_n7, assign9490_body27_e8362_d_n10, assign9490_body27_e8362_d_n11, assign9490_body27_e8362_d_n12, assign9490_body27_e8362_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body27_e8326: f64 = (locals.var_phi_s0_soi - locals.var_phi_s0_bulk);
        let assign9490_body27_e8329: f64 = (locals.var_t4 / locals.var_c_box);
        let assign9490_body27_e8330: f64 = (assign9490_body27_e8326 + assign9490_body27_e8329);
        let assign9490_body27_e8334: f64 = (locals.var_q_fd_soi / 2.0);
        let assign9490_body27_e8335: f64 = (locals.var_t4 + assign9490_body27_e8334);
        let assign9490_body27_e8337: f64 = (assign9490_body27_e8335 * locals.var_t_soi);
        let assign9490_body27_e8339: f64 = (assign9490_body27_e8337 / 1.034943e-10);
        let assign9490_body27_e8340: f64 = (assign9490_body27_e8330 + assign9490_body27_e8339);
        let assign9490_body27_e8342: f64 = (assign9490_body27_e8340 - locals.var_vbsbiz);
        let assign9490_body27_e8344: f64 = (assign9490_body27_e8342 + locals.var_phi_b_dep);
        let assign9490_body27_e8346: f64 = (-1.0);
        let assign9490_body27_e8349: f64 = (locals.var_t5 / locals.var_c_box);
        let assign9490_body27_e8350: f64 = (assign9490_body27_e8346 + assign9490_body27_e8349);
        let assign9490_body27_e8353: f64 = (locals.var_t5 * locals.var_t_soi);
        let assign9490_body27_e8355: f64 = (assign9490_body27_e8353 / 1.034943e-10);
        let assign9490_body27_e8356: f64 = (assign9490_body27_e8350 + assign9490_body27_e8355);
        let assign9490_body27_e8358: f64 = (assign9490_body27_e8356 + locals.var_phi_b_dep_dpsb);
        let assign9490_body27_e8359: f64 = (assign9490_body27_e8344 / assign9490_body27_e8358);
        let assign9490_body27_e8360: f64 = (locals.var_phi_s0_bulk - assign9490_body27_e8359);
        (assign9490_body27_e8360, (locals.var_phi_s0_bulk_dn0 - ((((((((locals.var_phi_s0_soi_dn0 - locals.var_phi_s0_bulk_dn0) + (locals.var_t4_dn0 / locals.var_c_box)) + (((locals.var_t4_dn0 + (locals.var_q_fd_soi_dn0 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn0 / locals.var_c_box) + ((locals.var_t5_dn0 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn0))) / (assign9490_body27_e8358 * assign9490_body27_e8358))), (locals.var_phi_s0_bulk_dn2 - ((((((((locals.var_phi_s0_soi_dn2 - locals.var_phi_s0_bulk_dn2) + (locals.var_t4_dn2 / locals.var_c_box)) + (((locals.var_t4_dn2 + (locals.var_q_fd_soi_dn2 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn2 / locals.var_c_box) + ((locals.var_t5_dn2 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn2))) / (assign9490_body27_e8358 * assign9490_body27_e8358))), (locals.var_phi_s0_bulk_dn6 - ((((((((locals.var_phi_s0_soi_dn6 - locals.var_phi_s0_bulk_dn6) + (locals.var_t4_dn6 / locals.var_c_box)) + (((locals.var_t4_dn6 + (locals.var_q_fd_soi_dn6 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn6 / locals.var_c_box) + ((locals.var_t5_dn6 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn6))) / (assign9490_body27_e8358 * assign9490_body27_e8358))), (locals.var_phi_s0_bulk_dn7 - ((((((((locals.var_phi_s0_soi_dn7 - locals.var_phi_s0_bulk_dn7) + (locals.var_t4_dn7 / locals.var_c_box)) + (((locals.var_t4_dn7 + (locals.var_q_fd_soi_dn7 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn7 / locals.var_c_box) + ((locals.var_t5_dn7 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn7))) / (assign9490_body27_e8358 * assign9490_body27_e8358))), (locals.var_phi_s0_bulk_dn10 - ((((((((locals.var_phi_s0_soi_dn10 - locals.var_phi_s0_bulk_dn10) + (locals.var_t4_dn10 / locals.var_c_box)) + (((locals.var_t4_dn10 + (locals.var_q_fd_soi_dn10 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn10 / locals.var_c_box) + ((locals.var_t5_dn10 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn10))) / (assign9490_body27_e8358 * assign9490_body27_e8358))), (locals.var_phi_s0_bulk_dn11 - ((((((((locals.var_phi_s0_soi_dn11 - locals.var_phi_s0_bulk_dn11) + (locals.var_t4_dn11 / locals.var_c_box)) + (((locals.var_t4_dn11 + (locals.var_q_fd_soi_dn11 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn11 / locals.var_c_box) + ((locals.var_t5_dn11 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn11))) / (assign9490_body27_e8358 * assign9490_body27_e8358))), (locals.var_phi_s0_bulk_dn12 - ((((((((locals.var_phi_s0_soi_dn12 - locals.var_phi_s0_bulk_dn12) + (locals.var_t4_dn12 / locals.var_c_box)) + (((locals.var_t4_dn12 + (locals.var_q_fd_soi_dn12 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn12 / locals.var_c_box) + ((locals.var_t5_dn12 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn12))) / (assign9490_body27_e8358 * assign9490_body27_e8358))), (locals.var_phi_s0_bulk_dn17 - ((((((((locals.var_phi_s0_soi_dn17 - locals.var_phi_s0_bulk_dn17) + (locals.var_t4_dn17 / locals.var_c_box)) + (((locals.var_t4_dn17 + (locals.var_q_fd_soi_dn17 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign9490_body27_e8358) - (assign9490_body27_e8344 * (((locals.var_t5_dn17 / locals.var_c_box) + ((locals.var_t5_dn17 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn17))) / (assign9490_body27_e8358 * assign9490_body27_e8358))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign9490_body27_e8362;
            locals.var_t6_dn0 = assign9490_body27_e8362_d_n0;
            locals.var_t6_dn2 = assign9490_body27_e8362_d_n2;
            locals.var_t6_dn6 = assign9490_body27_e8362_d_n6;
            locals.var_t6_dn7 = assign9490_body27_e8362_d_n7;
            locals.var_t6_dn10 = assign9490_body27_e8362_d_n10;
            locals.var_t6_dn11 = assign9490_body27_e8362_d_n11;
            locals.var_t6_dn12 = assign9490_body27_e8362_d_n12;
            locals.var_t6_dn17 = assign9490_body27_e8362_d_n17;
            locals.var_t6_rv = 0.0;
            let (assign9490_body28_e8369, assign9490_body28_e8369_d_n0, assign9490_body28_e8369_d_n2, assign9490_body28_e8369_d_n6, assign9490_body28_e8369_d_n7, assign9490_body28_e8369_d_n10, assign9490_body28_e8369_d_n11, assign9490_body28_e8369_d_n12, assign9490_body28_e8369_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        (locals.var_lp_s0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
            locals.var_t7 = assign9490_body28_e8369;
            locals.var_t7_dn0 = assign9490_body28_e8369_d_n0;
            locals.var_t7_dn2 = assign9490_body28_e8369_d_n2;
            locals.var_t7_dn6 = assign9490_body28_e8369_d_n6;
            locals.var_t7_dn7 = assign9490_body28_e8369_d_n7;
            locals.var_t7_dn10 = assign9490_body28_e8369_d_n10;
            locals.var_t7_dn11 = assign9490_body28_e8369_d_n11;
            locals.var_t7_dn12 = assign9490_body28_e8369_d_n12;
            locals.var_t7_dn17 = assign9490_body28_e8369_d_n17;
            locals.var_t7_rv = 0.0;
            let assign9490_body29_e8372: f64 = (locals.var_t6 - locals.var_phi_s0_bulk);
            let assign9490_body29_e8373: f64 = (assign9490_body29_e8372).abs();
            let assign9490_body29_e8375: f64 = if assign9490_body29_e8373 < 0.001 { 1.0 } else { 0.0 };
            locals.var_guard175 = assign9490_body29_e8375;
            locals.var_guard175_rv = 0.0;
            let (assign9490_body30_e8384,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard175 != 0.0)) {
        (locals.var_lp_s0_max,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign9490_body30_e8384;
            locals.var_lp_s0_rv = 0.0;
            let (assign9490_body31_e8391, assign9490_body31_e8391_d_n0, assign9490_body31_e8391_d_n2, assign9490_body31_e8391_d_n6, assign9490_body31_e8391_d_n7, assign9490_body31_e8391_d_n10, assign9490_body31_e8391_d_n11, assign9490_body31_e8391_d_n12, assign9490_body31_e8391_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
            locals.var_phi_s0_bulk = assign9490_body31_e8391;
            locals.var_phi_s0_bulk_dn0 = assign9490_body31_e8391_d_n0;
            locals.var_phi_s0_bulk_dn2 = assign9490_body31_e8391_d_n2;
            locals.var_phi_s0_bulk_dn6 = assign9490_body31_e8391_d_n6;
            locals.var_phi_s0_bulk_dn7 = assign9490_body31_e8391_d_n7;
            locals.var_phi_s0_bulk_dn10 = assign9490_body31_e8391_d_n10;
            locals.var_phi_s0_bulk_dn11 = assign9490_body31_e8391_d_n11;
            locals.var_phi_s0_bulk_dn12 = assign9490_body31_e8391_d_n12;
            locals.var_phi_s0_bulk_dn17 = assign9490_body31_e8391_d_n17;
            locals.var_phi_s0_bulk_rv = 0.0;
            let (assign9490_body32_e8398, assign9490_body32_e8398_d_n0, assign9490_body32_e8398_d_n2, assign9490_body32_e8398_d_n6, assign9490_body32_e8398_d_n7, assign9490_body32_e8398_d_n10, assign9490_body32_e8398_d_n11, assign9490_body32_e8398_d_n12, assign9490_body32_e8398_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_q_s0_bulk, locals.var_q_s0_bulk_dn0, locals.var_q_s0_bulk_dn2, locals.var_q_s0_bulk_dn6, locals.var_q_s0_bulk_dn7, locals.var_q_s0_bulk_dn10, locals.var_q_s0_bulk_dn11, locals.var_q_s0_bulk_dn12, locals.var_q_s0_bulk_dn17,)
    }
};
            locals.var_q_s0_bulk = assign9490_body32_e8398;
            locals.var_q_s0_bulk_dn0 = assign9490_body32_e8398_d_n0;
            locals.var_q_s0_bulk_dn2 = assign9490_body32_e8398_d_n2;
            locals.var_q_s0_bulk_dn6 = assign9490_body32_e8398_d_n6;
            locals.var_q_s0_bulk_dn7 = assign9490_body32_e8398_d_n7;
            locals.var_q_s0_bulk_dn10 = assign9490_body32_e8398_d_n10;
            locals.var_q_s0_bulk_dn11 = assign9490_body32_e8398_d_n11;
            locals.var_q_s0_bulk_dn12 = assign9490_body32_e8398_d_n12;
            locals.var_q_s0_bulk_dn17 = assign9490_body32_e8398_d_n17;
            locals.var_q_s0_bulk_rv = 0.0;
            let (assign9490_body33_e8407,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9490_body33_e8405: f64 = (locals.var_lp_s0 + 1.0);
        (assign9490_body33_e8405,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign9490_body33_e8407;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9500_e8416, assign9500_e8416_d_n0, assign9500_e8416_d_n2, assign9500_e8416_d_n6, assign9500_e8416_d_n7, assign9500_e8416_d_n10, assign9500_e8416_d_n11, assign9500_e8416_d_n12, assign9500_e8416_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9500_e8414: f64 = (locals.var_vbsbiz + locals.var_phi_s0_bulk);
        (assign9500_e8414, (locals.var_vbsbiz_dn0 + locals.var_phi_s0_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_s0_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_s0_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_s0_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_s0_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_s0_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_s0_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_s0_bulk_dn17),)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign9500_e8416;
        locals.var_phi_s0_bulk_dn0 = assign9500_e8416_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign9500_e8416_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign9500_e8416_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign9500_e8416_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign9500_e8416_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign9500_e8416_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign9500_e8416_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign9500_e8416_d_n17;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign9510_e8431, assign9510_e8431_d_n0, assign9510_e8431_d_n2, assign9510_e8431_d_n6, assign9510_e8431_d_n7, assign9510_e8431_d_n10, assign9510_e8431_d_n11, assign9510_e8431_d_n12, assign9510_e8431_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard158 == 0.0)) {
        let assign9510_e8425: f64 = (0.5 * locals.var_q_fd_soi);
        let assign9510_e8427: f64 = (assign9510_e8425 + locals.var_q_s0_bulk);
        let assign9510_e8428: f64 = (locals.var_c_soi_inv__blk111 * assign9510_e8427);
        let assign9510_e8429: f64 = (locals.var_phi_s0_soi + assign9510_e8428);
        (assign9510_e8429, (locals.var_phi_s0_soi_dn0 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_s0_bulk_dn0))), (locals.var_phi_s0_soi_dn2 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_s0_bulk_dn2))), (locals.var_phi_s0_soi_dn6 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_s0_bulk_dn6))), (locals.var_phi_s0_soi_dn7 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_s0_bulk_dn7))), (locals.var_phi_s0_soi_dn10 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_s0_bulk_dn10))), (locals.var_phi_s0_soi_dn11 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_s0_bulk_dn11))), (locals.var_phi_s0_soi_dn12 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_s0_bulk_dn12))), (locals.var_phi_s0_soi_dn17 + (locals.var_c_soi_inv__blk111 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_s0_bulk_dn17))),)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    }
};
        locals.var_phi_b0_soi = assign9510_e8431;
        locals.var_phi_b0_soi_dn0 = assign9510_e8431_d_n0;
        locals.var_phi_b0_soi_dn2 = assign9510_e8431_d_n2;
        locals.var_phi_b0_soi_dn6 = assign9510_e8431_d_n6;
        locals.var_phi_b0_soi_dn7 = assign9510_e8431_d_n7;
        locals.var_phi_b0_soi_dn10 = assign9510_e8431_d_n10;
        locals.var_phi_b0_soi_dn11 = assign9510_e8431_d_n11;
        locals.var_phi_b0_soi_dn12 = assign9510_e8431_d_n12;
        locals.var_phi_b0_soi_dn17 = assign9510_e8431_d_n17;
        locals.var_phi_b0_soi_rv = 0.0;

        let assign9520_e8438: f64 = (locals.var_vgs_fb + 0.2);
        let assign9520_e8440: f64 = if ((p.p25 == 1.0) && (locals.var_vgs > assign9520_e8438)) { 1.0 } else { 0.0 };
        locals.var_guard176 = assign9520_e8440;
        locals.var_guard176_rv = 0.0;

        let (assign9530_e8446,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        (locals.var_vfbsub0,)
    } else {
        (locals.var_vfbsub1,)
    }
};
        locals.var_vfbsub1 = assign9530_e8446;
        locals.var_vfbsub1_rv = 0.0;

        let (assign9540_e8458, assign9540_e8458_d_n0, assign9540_e8458_d_n2, assign9540_e8458_d_n6, assign9540_e8458_d_n7, assign9540_e8458_d_n10, assign9540_e8458_d_n11, assign9540_e8458_d_n12, assign9540_e8458_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9540_e8452: f64 = (locals.var_vgsz - locals.var_vfbsub1);
        let assign9540_e8454: f64 = (assign9540_e8452 + locals.var_dvth);
        let assign9540_e8456: f64 = (assign9540_e8454 - locals.var_dppg);
        (assign9540_e8456, ((locals.var_vgsz_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_vgsz_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    }
};
        locals.var_vgpsub = assign9540_e8458;
        locals.var_vgpsub_dn0 = assign9540_e8458_d_n0;
        locals.var_vgpsub_dn2 = assign9540_e8458_d_n2;
        locals.var_vgpsub_dn6 = assign9540_e8458_d_n6;
        locals.var_vgpsub_dn7 = assign9540_e8458_d_n7;
        locals.var_vgpsub_dn10 = assign9540_e8458_d_n10;
        locals.var_vgpsub_dn11 = assign9540_e8458_d_n11;
        locals.var_vgpsub_dn12 = assign9540_e8458_d_n12;
        locals.var_vgpsub_dn17 = assign9540_e8458_d_n17;
        locals.var_vgpsub_rv = 0.0;

        let (assign9550_e8464,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        (p.p137,)
    } else {
        (locals.var_sti2_dlt,)
    }
};
        locals.var_sti2_dlt = assign9550_e8464;
        locals.var_sti2_dlt_rv = 0.0;

        let (assign9560_e8470, assign9560_e8470_d_n0, assign9560_e8470_d_n2, assign9560_e8470_d_n6, assign9560_e8470_d_n7, assign9560_e8470_d_n10, assign9560_e8470_d_n11, assign9560_e8470_d_n12, assign9560_e8470_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn12, locals.var_vgssti_dn17,)
    }
};
        locals.var_vgssti = assign9560_e8470;
        locals.var_vgssti_dn0 = assign9560_e8470_d_n0;
        locals.var_vgssti_dn2 = assign9560_e8470_d_n2;
        locals.var_vgssti_dn6 = assign9560_e8470_d_n6;
        locals.var_vgssti_dn7 = assign9560_e8470_d_n7;
        locals.var_vgssti_dn10 = assign9560_e8470_d_n10;
        locals.var_vgssti_dn11 = assign9560_e8470_d_n11;
        locals.var_vgssti_dn12 = assign9560_e8470_d_n12;
        locals.var_vgssti_dn17 = assign9560_e8470_d_n17;
        locals.var_vgssti_rv = 0.0;

        let (assign9570_e8485, assign9570_e8485_d_n0, assign9570_e8485_d_n2, assign9570_e8485_d_n6, assign9570_e8485_d_n7, assign9570_e8485_d_n10, assign9570_e8485_d_n11, assign9570_e8485_d_n12, assign9570_e8485_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9570_e8476: f64 = (2.0 * 1.6021918e-19);
        let assign9570_e8478: f64 = (assign9570_e8476 * locals.var_uc_nsubs);
        let assign9570_e8480: f64 = (assign9570_e8478 * 1.034943e-10);
        let assign9570_e8482: f64 = (assign9570_e8480 / locals.var_beta);
        let assign9570_e8483: f64 = (assign9570_e8482).sqrt();
        (assign9570_e8483, ((((assign9570_e8476 * locals.var_uc_nsubs_dn0) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9570_e8483)), ((((assign9570_e8476 * locals.var_uc_nsubs_dn2) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9570_e8483)), ((((assign9570_e8476 * locals.var_uc_nsubs_dn6) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9570_e8483)), ((((assign9570_e8476 * locals.var_uc_nsubs_dn7) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9570_e8483)), ((((((assign9570_e8476 * locals.var_uc_nsubs_dn10) * 1.034943e-10) * locals.var_beta) - (assign9570_e8480 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign9570_e8483)), ((((assign9570_e8476 * locals.var_uc_nsubs_dn11) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9570_e8483)), ((((assign9570_e8476 * locals.var_uc_nsubs_dn12) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9570_e8483)), ((((assign9570_e8476 * locals.var_uc_nsubs_dn17) * 1.034943e-10) / locals.var_beta) / (2.0 * assign9570_e8483)),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn10, locals.var_costi0_dn11, locals.var_costi0_dn12, locals.var_costi0_dn17,)
    }
};
        locals.var_costi0 = assign9570_e8485;
        locals.var_costi0_dn0 = assign9570_e8485_d_n0;
        locals.var_costi0_dn2 = assign9570_e8485_d_n2;
        locals.var_costi0_dn6 = assign9570_e8485_d_n6;
        locals.var_costi0_dn7 = assign9570_e8485_d_n7;
        locals.var_costi0_dn10 = assign9570_e8485_d_n10;
        locals.var_costi0_dn11 = assign9570_e8485_d_n11;
        locals.var_costi0_dn12 = assign9570_e8485_d_n12;
        locals.var_costi0_dn17 = assign9570_e8485_d_n17;
        locals.var_costi0_rv = 0.0;

        let (assign9580_e8497, assign9580_e8497_d_n0, assign9580_e8497_d_n2, assign9580_e8497_d_n6, assign9580_e8497_d_n7, assign9580_e8497_d_n10, assign9580_e8497_d_n11, assign9580_e8497_d_n12, assign9580_e8497_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9580_e8491: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_nsubs;
        let assign9580_e8493: f64 = (assign9580_e8491 * __rspice_inv_cse_0);
        let assign9580_e8495: f64 = (assign9580_e8493 * __rspice_inv_cse_0);
        (assign9580_e8495, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn12 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn12)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_nin_dn17 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn17)) * locals.var_uc_nsubs) - (assign9580_e8491 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) * locals.var_uc_nsubs) - (assign9580_e8493 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn10, locals.var_costi1_dn11, locals.var_costi1_dn12, locals.var_costi1_dn17,)
    }
};
        locals.var_costi1 = assign9580_e8497;
        locals.var_costi1_dn0 = assign9580_e8497_d_n0;
        locals.var_costi1_dn2 = assign9580_e8497_d_n2;
        locals.var_costi1_dn6 = assign9580_e8497_d_n6;
        locals.var_costi1_dn7 = assign9580_e8497_d_n7;
        locals.var_costi1_dn10 = assign9580_e8497_d_n10;
        locals.var_costi1_dn11 = assign9580_e8497_d_n11;
        locals.var_costi1_dn12 = assign9580_e8497_d_n12;
        locals.var_costi1_dn17 = assign9580_e8497_d_n17;
        locals.var_costi1_rv = 0.0;

        let (assign9590_e8509, assign9590_e8509_d_n0, assign9590_e8509_d_n2, assign9590_e8509_d_n6, assign9590_e8509_d_n7, assign9590_e8509_d_n10, assign9590_e8509_d_n11, assign9590_e8509_d_n12, assign9590_e8509_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9590_e8503: f64 = (locals.var_costi0 * locals.var_costi0);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_c_fox;
        let assign9590_e8505: f64 = (assign9590_e8503 * __rspice_inv_cse_1);
        let assign9590_e8507: f64 = (assign9590_e8505 * __rspice_inv_cse_1);
        (assign9590_e8507, ((((((((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn12 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn12)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox)), ((((((((locals.var_costi0_dn17 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn17)) * locals.var_c_fox) - (assign9590_e8503 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox)) * locals.var_c_fox) - (assign9590_e8505 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn12, locals.var_costi3_dn17,)
    }
};
        locals.var_costi3 = assign9590_e8509;
        locals.var_costi3_dn0 = assign9590_e8509_d_n0;
        locals.var_costi3_dn2 = assign9590_e8509_d_n2;
        locals.var_costi3_dn6 = assign9590_e8509_d_n6;
        locals.var_costi3_dn7 = assign9590_e8509_d_n7;
        locals.var_costi3_dn10 = assign9590_e8509_d_n10;
        locals.var_costi3_dn11 = assign9590_e8509_d_n11;
        locals.var_costi3_dn12 = assign9590_e8509_d_n12;
        locals.var_costi3_dn17 = assign9590_e8509_d_n17;
        locals.var_costi3_rv = 0.0;

        let (assign9600_e8519, assign9600_e8519_d_n0, assign9600_e8519_d_n2, assign9600_e8519_d_n6, assign9600_e8519_d_n7, assign9600_e8519_d_n10, assign9600_e8519_d_n11, assign9600_e8519_d_n12, assign9600_e8519_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9600_e8515: f64 = (locals.var_costi3 * locals.var_beta);
        let assign9600_e8517: f64 = (assign9600_e8515 / 2.0);
        (assign9600_e8517, ((locals.var_costi3_dn0 * locals.var_beta) / 2.0), ((locals.var_costi3_dn2 * locals.var_beta) / 2.0), ((locals.var_costi3_dn6 * locals.var_beta) / 2.0), ((locals.var_costi3_dn7 * locals.var_beta) / 2.0), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) / 2.0), ((locals.var_costi3_dn11 * locals.var_beta) / 2.0), ((locals.var_costi3_dn12 * locals.var_beta) / 2.0), ((locals.var_costi3_dn17 * locals.var_beta) / 2.0),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn12, locals.var_costi4_dn17,)
    }
};
        locals.var_costi4 = assign9600_e8519;
        locals.var_costi4_dn0 = assign9600_e8519_d_n0;
        locals.var_costi4_dn2 = assign9600_e8519_d_n2;
        locals.var_costi4_dn6 = assign9600_e8519_d_n6;
        locals.var_costi4_dn7 = assign9600_e8519_d_n7;
        locals.var_costi4_dn10 = assign9600_e8519_d_n10;
        locals.var_costi4_dn11 = assign9600_e8519_d_n11;
        locals.var_costi4_dn12 = assign9600_e8519_d_n12;
        locals.var_costi4_dn17 = assign9600_e8519_d_n17;
        locals.var_costi4_rv = 0.0;

        let (assign9610_e8529, assign9610_e8529_d_n0, assign9610_e8529_d_n2, assign9610_e8529_d_n6, assign9610_e8529_d_n7, assign9610_e8529_d_n10, assign9610_e8529_d_n11, assign9610_e8529_d_n12, assign9610_e8529_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9610_e8525: f64 = (locals.var_costi4 * locals.var_beta);
        let assign9610_e8527: f64 = (assign9610_e8525 * 2.0);
        (assign9610_e8527, ((locals.var_costi4_dn0 * locals.var_beta) * 2.0), ((locals.var_costi4_dn2 * locals.var_beta) * 2.0), ((locals.var_costi4_dn6 * locals.var_beta) * 2.0), ((locals.var_costi4_dn7 * locals.var_beta) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), ((locals.var_costi4_dn11 * locals.var_beta) * 2.0), ((locals.var_costi4_dn12 * locals.var_beta) * 2.0), ((locals.var_costi4_dn17 * locals.var_beta) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn12, locals.var_costi5_dn17,)
    }
};
        locals.var_costi5 = assign9610_e8529;
        locals.var_costi5_dn0 = assign9610_e8529_d_n0;
        locals.var_costi5_dn2 = assign9610_e8529_d_n2;
        locals.var_costi5_dn6 = assign9610_e8529_d_n6;
        locals.var_costi5_dn7 = assign9610_e8529_d_n7;
        locals.var_costi5_dn10 = assign9610_e8529_d_n10;
        locals.var_costi5_dn11 = assign9610_e8529_d_n11;
        locals.var_costi5_dn12 = assign9610_e8529_d_n12;
        locals.var_costi5_dn17 = assign9610_e8529_d_n17;
        locals.var_costi5_rv = 0.0;

        let (assign9620_e8546, assign9620_e8546_d_n0, assign9620_e8546_d_n2, assign9620_e8546_d_n6, assign9620_e8546_d_n7, assign9620_e8546_d_n10, assign9620_e8546_d_n11, assign9620_e8546_d_n12, assign9620_e8546_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9620_e8537: f64 = (locals.var_beta * locals.var_vgssti);
        let assign9620_e8539: f64 = (assign9620_e8537 - 1.0);
        let assign9620_e8540: f64 = (4.0 * assign9620_e8539);
        let assign9620_e8542: f64 = (assign9620_e8540 / locals.var_costi5);
        let assign9620_e8543: f64 = (1.0 + assign9620_e8542);
        let assign9620_e8544: f64 = (assign9620_e8543).sqrt();
        (assign9620_e8544, (((((4.0 * (locals.var_beta * locals.var_vgssti_dn0)) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn0)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn2)) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn2)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn6)) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn6)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn7)) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn7)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)), (((((4.0 * ((locals.var_beta_dn10 * locals.var_vgssti) + (locals.var_beta * locals.var_vgssti_dn10))) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn10)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn11)) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn11)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn12)) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn12)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)), (((((4.0 * (locals.var_beta * locals.var_vgssti_dn17)) * locals.var_costi5) - (assign9620_e8540 * locals.var_costi5_dn17)) / (locals.var_costi5 * locals.var_costi5)) / (2.0 * assign9620_e8544)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn12, locals.var_costi6_dn17,)
    }
};
        locals.var_costi6 = assign9620_e8546;
        locals.var_costi6_dn0 = assign9620_e8546_d_n0;
        locals.var_costi6_dn2 = assign9620_e8546_d_n2;
        locals.var_costi6_dn6 = assign9620_e8546_d_n6;
        locals.var_costi6_dn7 = assign9620_e8546_d_n7;
        locals.var_costi6_dn10 = assign9620_e8546_d_n10;
        locals.var_costi6_dn11 = assign9620_e8546_d_n11;
        locals.var_costi6_dn12 = assign9620_e8546_d_n12;
        locals.var_costi6_dn17 = assign9620_e8546_d_n17;
        locals.var_costi6_rv = 0.0;

        let (assign9630_e8558, assign9630_e8558_d_n0, assign9630_e8558_d_n2, assign9630_e8558_d_n6, assign9630_e8558_d_n7, assign9630_e8558_d_n10, assign9630_e8558_d_n11, assign9630_e8558_d_n12, assign9630_e8558_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9630_e8554: f64 = (1.0 - locals.var_costi6);
        let assign9630_e8555: f64 = (locals.var_costi4 * assign9630_e8554);
        let assign9630_e8556: f64 = (locals.var_vgssti + assign9630_e8555);
        (assign9630_e8556, (locals.var_vgssti_dn0 + ((locals.var_costi4_dn0 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn0)))), (locals.var_vgssti_dn2 + ((locals.var_costi4_dn2 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn2)))), (locals.var_vgssti_dn6 + ((locals.var_costi4_dn6 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn6)))), (locals.var_vgssti_dn7 + ((locals.var_costi4_dn7 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn7)))), (locals.var_vgssti_dn10 + ((locals.var_costi4_dn10 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn10)))), (locals.var_vgssti_dn11 + ((locals.var_costi4_dn11 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn11)))), (locals.var_vgssti_dn12 + ((locals.var_costi4_dn12 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn12)))), (locals.var_vgssti_dn17 + ((locals.var_costi4_dn17 * assign9630_e8554) + (locals.var_costi4 * (-locals.var_costi6_dn17)))),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn12, locals.var_psasti_dn17,)
    }
};
        locals.var_psasti = assign9630_e8558;
        locals.var_psasti_dn0 = assign9630_e8558_d_n0;
        locals.var_psasti_dn2 = assign9630_e8558_d_n2;
        locals.var_psasti_dn6 = assign9630_e8558_d_n6;
        locals.var_psasti_dn7 = assign9630_e8558_d_n7;
        locals.var_psasti_dn10 = assign9630_e8558_d_n10;
        locals.var_psasti_dn11 = assign9630_e8558_d_n11;
        locals.var_psasti_dn12 = assign9630_e8558_d_n12;
        locals.var_psasti_dn17 = assign9630_e8558_d_n17;
        locals.var_psasti_rv = 0.0;

        let (assign9640_e8568, assign9640_e8568_d_n0, assign9640_e8568_d_n2, assign9640_e8568_d_n6, assign9640_e8568_d_n7, assign9640_e8568_d_n10, assign9640_e8568_d_n11, assign9640_e8568_d_n12, assign9640_e8568_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9640_e8564: f64 = (1.0 / locals.var_costi1);
        let assign9640_e8566: f64 = (assign9640_e8564 / locals.var_costi3);
        (assign9640_e8566, ((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn12 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn12)) / (locals.var_costi3 * locals.var_costi3)), ((((-(locals.var_costi1_dn17 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign9640_e8564 * locals.var_costi3_dn17)) / (locals.var_costi3 * locals.var_costi3)),)
    } else {
        (locals.var_asti, locals.var_asti_dn0, locals.var_asti_dn2, locals.var_asti_dn6, locals.var_asti_dn7, locals.var_asti_dn10, locals.var_asti_dn11, locals.var_asti_dn12, locals.var_asti_dn17,)
    }
};
        locals.var_asti = assign9640_e8568;
        locals.var_asti_dn0 = assign9640_e8568_d_n0;
        locals.var_asti_dn2 = assign9640_e8568_d_n2;
        locals.var_asti_dn6 = assign9640_e8568_d_n6;
        locals.var_asti_dn7 = assign9640_e8568_d_n7;
        locals.var_asti_dn10 = assign9640_e8568_d_n10;
        locals.var_asti_dn11 = assign9640_e8568_d_n11;
        locals.var_asti_dn12 = assign9640_e8568_d_n12;
        locals.var_asti_dn17 = assign9640_e8568_d_n17;
        locals.var_asti_rv = 0.0;

        let (assign9650_e8585, assign9650_e8585_d_n0, assign9650_e8585_d_n2, assign9650_e8585_d_n6, assign9650_e8585_d_n7, assign9650_e8585_d_n10, assign9650_e8585_d_n11, assign9650_e8585_d_n12, assign9650_e8585_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9650_e8575: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign9650_e8576: f64 = (locals.var_asti * assign9650_e8575);
        let assign9650_e8577: f64 = (assign9650_e8576).ln();
        let assign9650_e8581: f64 = (2.0 / locals.var_vgssti);
        let assign9650_e8582: f64 = (locals.var_beta + assign9650_e8581);
        let assign9650_e8583: f64 = (assign9650_e8577 / assign9650_e8582);
        (assign9650_e8583, ((((((locals.var_asti_dn0 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (-((2.0 * locals.var_vgssti_dn0) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9650_e8582 * assign9650_e8582)), ((((((locals.var_asti_dn2 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (-((2.0 * locals.var_vgssti_dn2) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9650_e8582 * assign9650_e8582)), ((((((locals.var_asti_dn6 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (-((2.0 * locals.var_vgssti_dn6) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9650_e8582 * assign9650_e8582)), ((((((locals.var_asti_dn7 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (-((2.0 * locals.var_vgssti_dn7) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9650_e8582 * assign9650_e8582)), ((((((locals.var_asti_dn10 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (locals.var_vgssti * locals.var_vgssti)))))) / (assign9650_e8582 * assign9650_e8582)), ((((((locals.var_asti_dn11 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (-((2.0 * locals.var_vgssti_dn11) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9650_e8582 * assign9650_e8582)), ((((((locals.var_asti_dn12 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn12 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn12)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (-((2.0 * locals.var_vgssti_dn12) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9650_e8582 * assign9650_e8582)), ((((((locals.var_asti_dn17 * assign9650_e8575) + (locals.var_asti * ((locals.var_vgssti_dn17 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn17)))) / assign9650_e8576) * assign9650_e8582) - (assign9650_e8577 * (-((2.0 * locals.var_vgssti_dn17) / (locals.var_vgssti * locals.var_vgssti))))) / (assign9650_e8582 * assign9650_e8582)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn12, locals.var_psbsti_dn17,)
    }
};
        locals.var_psbsti = assign9650_e8585;
        locals.var_psbsti_dn0 = assign9650_e8585_d_n0;
        locals.var_psbsti_dn2 = assign9650_e8585_d_n2;
        locals.var_psbsti_dn6 = assign9650_e8585_d_n6;
        locals.var_psbsti_dn7 = assign9650_e8585_d_n7;
        locals.var_psbsti_dn10 = assign9650_e8585_d_n10;
        locals.var_psbsti_dn11 = assign9650_e8585_d_n11;
        locals.var_psbsti_dn12 = assign9650_e8585_d_n12;
        locals.var_psbsti_dn17 = assign9650_e8585_d_n17;
        locals.var_psbsti_rv = 0.0;

        let (assign9660_e8595, assign9660_e8595_d_n0, assign9660_e8595_d_n2, assign9660_e8595_d_n6, assign9660_e8595_d_n7, assign9660_e8595_d_n10, assign9660_e8595_d_n11, assign9660_e8595_d_n12, assign9660_e8595_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9660_e8591: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign9660_e8593: f64 = (assign9660_e8591 - locals.var_sti2_dlt);
        (assign9660_e8593, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn12 - locals.var_psasti_dn12), (locals.var_psbsti_dn17 - locals.var_psasti_dn17),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn12, locals.var_psab_dn17,)
    }
};
        locals.var_psab = assign9660_e8595;
        locals.var_psab_dn0 = assign9660_e8595_d_n0;
        locals.var_psab_dn2 = assign9660_e8595_d_n2;
        locals.var_psab_dn6 = assign9660_e8595_d_n6;
        locals.var_psab_dn7 = assign9660_e8595_d_n7;
        locals.var_psab_dn10 = assign9660_e8595_d_n10;
        locals.var_psab_dn11 = assign9660_e8595_d_n11;
        locals.var_psab_dn12 = assign9660_e8595_d_n12;
        locals.var_psab_dn17 = assign9660_e8595_d_n17;
        locals.var_psab_rv = 0.0;

        let (assign9670_e8616, assign9670_e8616_d_n0, assign9670_e8616_d_n2, assign9670_e8616_d_n6, assign9670_e8616_d_n7, assign9670_e8616_d_n10, assign9670_e8616_d_n11, assign9670_e8616_d_n12, assign9670_e8616_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9670_e8604: f64 = (locals.var_psab * locals.var_psab);
        let assign9670_e8607: f64 = (4.0 * locals.var_sti2_dlt);
        let assign9670_e8609: f64 = (assign9670_e8607 * locals.var_psbsti);
        let assign9670_e8610: f64 = (assign9670_e8604 + assign9670_e8609);
        let assign9670_e8611: f64 = (assign9670_e8610).sqrt();
        let assign9670_e8612: f64 = (locals.var_psab + assign9670_e8611);
        let assign9670_e8613: f64 = (0.5 * assign9670_e8612);
        let assign9670_e8614: f64 = (locals.var_psbsti - assign9670_e8613);
        (assign9670_e8614, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign9670_e8607 * locals.var_psbsti_dn0)) / (2.0 * assign9670_e8611))))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign9670_e8607 * locals.var_psbsti_dn2)) / (2.0 * assign9670_e8611))))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign9670_e8607 * locals.var_psbsti_dn6)) / (2.0 * assign9670_e8611))))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign9670_e8607 * locals.var_psbsti_dn7)) / (2.0 * assign9670_e8611))))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign9670_e8607 * locals.var_psbsti_dn10)) / (2.0 * assign9670_e8611))))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign9670_e8607 * locals.var_psbsti_dn11)) / (2.0 * assign9670_e8611))))), (locals.var_psbsti_dn12 - (0.5 * (locals.var_psab_dn12 + ((((locals.var_psab_dn12 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn12)) + (assign9670_e8607 * locals.var_psbsti_dn12)) / (2.0 * assign9670_e8611))))), (locals.var_psbsti_dn17 - (0.5 * (locals.var_psab_dn17 + ((((locals.var_psab_dn17 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn17)) + (assign9670_e8607 * locals.var_psbsti_dn17)) / (2.0 * assign9670_e8611))))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    }
};
        locals.var_psti = assign9670_e8616;
        locals.var_psti_dn0 = assign9670_e8616_d_n0;
        locals.var_psti_dn2 = assign9670_e8616_d_n2;
        locals.var_psti_dn6 = assign9670_e8616_d_n6;
        locals.var_psti_dn7 = assign9670_e8616_d_n7;
        locals.var_psti_dn10 = assign9670_e8616_d_n10;
        locals.var_psti_dn11 = assign9670_e8616_d_n11;
        locals.var_psti_dn12 = assign9670_e8616_d_n12;
        locals.var_psti_dn17 = assign9670_e8616_d_n17;
        locals.var_psti_rv = 0.0;

        let (assign9680_e8625, assign9680_e8625_d_n0, assign9680_e8625_d_n2, assign9680_e8625_d_n6, assign9680_e8625_d_n7, assign9680_e8625_d_n10, assign9680_e8625_d_n11, assign9680_e8625_d_n12, assign9680_e8625_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9680_e8622: f64 = (locals.var_beta * locals.var_psti);
        let assign9680_e8623: f64 = (assign9680_e8622).exp();
        (assign9680_e8623, (assign9680_e8623 * (locals.var_beta * locals.var_psti_dn0)), (assign9680_e8623 * (locals.var_beta * locals.var_psti_dn2)), (assign9680_e8623 * (locals.var_beta * locals.var_psti_dn6)), (assign9680_e8623 * (locals.var_beta * locals.var_psti_dn7)), (assign9680_e8623 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))), (assign9680_e8623 * (locals.var_beta * locals.var_psti_dn11)), (assign9680_e8623 * (locals.var_beta * locals.var_psti_dn12)), (assign9680_e8623 * (locals.var_beta * locals.var_psti_dn17)),)
    } else {
        (locals.var_expsti, locals.var_expsti_dn0, locals.var_expsti_dn2, locals.var_expsti_dn6, locals.var_expsti_dn7, locals.var_expsti_dn10, locals.var_expsti_dn11, locals.var_expsti_dn12, locals.var_expsti_dn17,)
    }
};
        locals.var_expsti = assign9680_e8625;
        locals.var_expsti_dn0 = assign9680_e8625_d_n0;
        locals.var_expsti_dn2 = assign9680_e8625_d_n2;
        locals.var_expsti_dn6 = assign9680_e8625_d_n6;
        locals.var_expsti_dn7 = assign9680_e8625_d_n7;
        locals.var_expsti_dn10 = assign9680_e8625_d_n10;
        locals.var_expsti_dn11 = assign9680_e8625_d_n11;
        locals.var_expsti_dn12 = assign9680_e8625_d_n12;
        locals.var_expsti_dn17 = assign9680_e8625_d_n17;
        locals.var_expsti_rv = 0.0;

        let (assign9690_e8639, assign9690_e8639_d_n0, assign9690_e8639_d_n2, assign9690_e8639_d_n6, assign9690_e8639_d_n7, assign9690_e8639_d_n10, assign9690_e8639_d_n11, assign9690_e8639_d_n12, assign9690_e8639_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9690_e8631: f64 = (locals.var_beta * locals.var_psti);
        let assign9690_e8633: f64 = (assign9690_e8631 - 1.0);
        let assign9690_e8636: f64 = (locals.var_costi1 * locals.var_expsti);
        let assign9690_e8637: f64 = (assign9690_e8633 + assign9690_e8636);
        (assign9690_e8637, ((locals.var_beta * locals.var_psti_dn0) + ((locals.var_costi1_dn0 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn0))), ((locals.var_beta * locals.var_psti_dn2) + ((locals.var_costi1_dn2 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn2))), ((locals.var_beta * locals.var_psti_dn6) + ((locals.var_costi1_dn6 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn6))), ((locals.var_beta * locals.var_psti_dn7) + ((locals.var_costi1_dn7 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn7))), (((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)) + ((locals.var_costi1_dn10 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn10))), ((locals.var_beta * locals.var_psti_dn11) + ((locals.var_costi1_dn11 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn11))), ((locals.var_beta * locals.var_psti_dn12) + ((locals.var_costi1_dn12 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn12))), ((locals.var_beta * locals.var_psti_dn17) + ((locals.var_costi1_dn17 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn17))),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12, locals.var_sq1sti_dn17,)
    }
};
        locals.var_sq1sti = assign9690_e8639;
        locals.var_sq1sti_dn0 = assign9690_e8639_d_n0;
        locals.var_sq1sti_dn2 = assign9690_e8639_d_n2;
        locals.var_sq1sti_dn6 = assign9690_e8639_d_n6;
        locals.var_sq1sti_dn7 = assign9690_e8639_d_n7;
        locals.var_sq1sti_dn10 = assign9690_e8639_d_n10;
        locals.var_sq1sti_dn11 = assign9690_e8639_d_n11;
        locals.var_sq1sti_dn12 = assign9690_e8639_d_n12;
        locals.var_sq1sti_dn17 = assign9690_e8639_d_n17;
        locals.var_sq1sti_rv = 0.0;

        let (assign9700_e8649, assign9700_e8649_d_n0, assign9700_e8649_d_n2, assign9700_e8649_d_n6, assign9700_e8649_d_n7, assign9700_e8649_d_n10, assign9700_e8649_d_n11, assign9700_e8649_d_n12, assign9700_e8649_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) {
        let assign9700_e8645: f64 = (locals.var_beta * locals.var_psti);
        let assign9700_e8647: f64 = (assign9700_e8645 - 1.0);
        (assign9700_e8647, (locals.var_beta * locals.var_psti_dn0), (locals.var_beta * locals.var_psti_dn2), (locals.var_beta * locals.var_psti_dn6), (locals.var_beta * locals.var_psti_dn7), ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)), (locals.var_beta * locals.var_psti_dn11), (locals.var_beta * locals.var_psti_dn12), (locals.var_beta * locals.var_psti_dn17),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12, locals.var_sq2sti_dn17,)
    }
};
        locals.var_sq2sti = assign9700_e8649;
        locals.var_sq2sti_dn0 = assign9700_e8649_d_n0;
        locals.var_sq2sti_dn2 = assign9700_e8649_d_n2;
        locals.var_sq2sti_dn6 = assign9700_e8649_d_n6;
        locals.var_sq2sti_dn7 = assign9700_e8649_d_n7;
        locals.var_sq2sti_dn10 = assign9700_e8649_d_n10;
        locals.var_sq2sti_dn11 = assign9700_e8649_d_n11;
        locals.var_sq2sti_dn12 = assign9700_e8649_d_n12;
        locals.var_sq2sti_dn17 = assign9700_e8649_d_n17;
        locals.var_sq2sti_rv = 0.0;

        let assign9710_e8656: f64 = if ((locals.var_sq1sti > 0.0) && (locals.var_sq2sti > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard177 = assign9710_e8656;
        locals.var_guard177_rv = 0.0;

        let (assign9720_e8673, assign9720_e8673_d_n0, assign9720_e8673_d_n2, assign9720_e8673_d_n6, assign9720_e8673_d_n7, assign9720_e8673_d_n10, assign9720_e8673_d_n11, assign9720_e8673_d_n12, assign9720_e8673_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9720_e8664: f64 = (locals.var_beta * locals.var_psti);
        let assign9720_e8666: f64 = (assign9720_e8664 - 1.0);
        let assign9720_e8669: f64 = (locals.var_costi1 * locals.var_expsti);
        let assign9720_e8670: f64 = (assign9720_e8666 + assign9720_e8669);
        let assign9720_e8671: f64 = (assign9720_e8670).sqrt();
        (assign9720_e8671, (((locals.var_beta * locals.var_psti_dn0) + ((locals.var_costi1_dn0 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn0))) / (2.0 * assign9720_e8671)), (((locals.var_beta * locals.var_psti_dn2) + ((locals.var_costi1_dn2 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn2))) / (2.0 * assign9720_e8671)), (((locals.var_beta * locals.var_psti_dn6) + ((locals.var_costi1_dn6 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn6))) / (2.0 * assign9720_e8671)), (((locals.var_beta * locals.var_psti_dn7) + ((locals.var_costi1_dn7 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn7))) / (2.0 * assign9720_e8671)), ((((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)) + ((locals.var_costi1_dn10 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn10))) / (2.0 * assign9720_e8671)), (((locals.var_beta * locals.var_psti_dn11) + ((locals.var_costi1_dn11 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn11))) / (2.0 * assign9720_e8671)), (((locals.var_beta * locals.var_psti_dn12) + ((locals.var_costi1_dn12 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn12))) / (2.0 * assign9720_e8671)), (((locals.var_beta * locals.var_psti_dn17) + ((locals.var_costi1_dn17 * locals.var_expsti) + (locals.var_costi1 * locals.var_expsti_dn17))) / (2.0 * assign9720_e8671)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12, locals.var_sq1sti_dn17,)
    }
};
        locals.var_sq1sti = assign9720_e8673;
        locals.var_sq1sti_dn0 = assign9720_e8673_d_n0;
        locals.var_sq1sti_dn2 = assign9720_e8673_d_n2;
        locals.var_sq1sti_dn6 = assign9720_e8673_d_n6;
        locals.var_sq1sti_dn7 = assign9720_e8673_d_n7;
        locals.var_sq1sti_dn10 = assign9720_e8673_d_n10;
        locals.var_sq1sti_dn11 = assign9720_e8673_d_n11;
        locals.var_sq1sti_dn12 = assign9720_e8673_d_n12;
        locals.var_sq1sti_dn17 = assign9720_e8673_d_n17;
        locals.var_sq1sti_rv = 0.0;

        let (assign9730_e8686, assign9730_e8686_d_n0, assign9730_e8686_d_n2, assign9730_e8686_d_n6, assign9730_e8686_d_n7, assign9730_e8686_d_n10, assign9730_e8686_d_n11, assign9730_e8686_d_n12, assign9730_e8686_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9730_e8681: f64 = (locals.var_beta * locals.var_psti);
        let assign9730_e8683: f64 = (assign9730_e8681 - 1.0);
        let assign9730_e8684: f64 = (assign9730_e8683).sqrt();
        (assign9730_e8684, ((locals.var_beta * locals.var_psti_dn0) / (2.0 * assign9730_e8684)), ((locals.var_beta * locals.var_psti_dn2) / (2.0 * assign9730_e8684)), ((locals.var_beta * locals.var_psti_dn6) / (2.0 * assign9730_e8684)), ((locals.var_beta * locals.var_psti_dn7) / (2.0 * assign9730_e8684)), (((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10)) / (2.0 * assign9730_e8684)), ((locals.var_beta * locals.var_psti_dn11) / (2.0 * assign9730_e8684)), ((locals.var_beta * locals.var_psti_dn12) / (2.0 * assign9730_e8684)), ((locals.var_beta * locals.var_psti_dn17) / (2.0 * assign9730_e8684)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12, locals.var_sq2sti_dn17,)
    }
};
        locals.var_sq2sti = assign9730_e8686;
        locals.var_sq2sti_dn0 = assign9730_e8686_d_n0;
        locals.var_sq2sti_dn2 = assign9730_e8686_d_n2;
        locals.var_sq2sti_dn6 = assign9730_e8686_d_n6;
        locals.var_sq2sti_dn7 = assign9730_e8686_d_n7;
        locals.var_sq2sti_dn10 = assign9730_e8686_d_n10;
        locals.var_sq2sti_dn11 = assign9730_e8686_d_n11;
        locals.var_sq2sti_dn12 = assign9730_e8686_d_n12;
        locals.var_sq2sti_dn17 = assign9730_e8686_d_n17;
        locals.var_sq2sti_rv = 0.0;

        let (assign9740_e8698, assign9740_e8698_d_n0, assign9740_e8698_d_n2, assign9740_e8698_d_n6, assign9740_e8698_d_n7, assign9740_e8698_d_n10, assign9740_e8698_d_n11, assign9740_e8698_d_n12, assign9740_e8698_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9740_e8695: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign9740_e8696: f64 = (locals.var_costi0 * assign9740_e8695);
        (assign9740_e8696, ((locals.var_costi0_dn0 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn6 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn10 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn12 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn12 - locals.var_sq2sti_dn12))), ((locals.var_costi0_dn17 * assign9740_e8695) + (locals.var_costi0 * (locals.var_sq1sti_dn17 - locals.var_sq2sti_dn17))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn12, locals.var_qn0sti_dn17,)
    }
};
        locals.var_qn0sti = assign9740_e8698;
        locals.var_qn0sti_dn0 = assign9740_e8698_d_n0;
        locals.var_qn0sti_dn2 = assign9740_e8698_d_n2;
        locals.var_qn0sti_dn6 = assign9740_e8698_d_n6;
        locals.var_qn0sti_dn7 = assign9740_e8698_d_n7;
        locals.var_qn0sti_dn10 = assign9740_e8698_d_n10;
        locals.var_qn0sti_dn11 = assign9740_e8698_d_n11;
        locals.var_qn0sti_dn12 = assign9740_e8698_d_n12;
        locals.var_qn0sti_dn17 = assign9740_e8698_d_n17;
        locals.var_qn0sti_rv = 0.0;

        let (assign9750_e8710, assign9750_e8710_d_n10,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9750_e8706: f64 = (2.0 * locals.var_weff);
        let assign9750_e8708: f64 = (assign9750_e8706 / locals.var_beta);
        (assign9750_e8708, (-((assign9750_e8706 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn10,)
    }
};
        locals.var_costi7 = assign9750_e8710;
        locals.var_costi7_dn10 = assign9750_e8710_d_n10;
        locals.var_costi7_rv = 0.0;

        let (assign9760_e8720, assign9760_e8720_d_n0, assign9760_e8720_d_n2, assign9760_e8720_d_n6, assign9760_e8720_d_n7, assign9760_e8720_d_n10, assign9760_e8720_d_n11, assign9760_e8720_d_n12, assign9760_e8720_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9760_e8718: f64 = (300.0 * 0.0001);
        (assign9760_e8718, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn12, locals.var_mu_dn17,)
    }
};
        locals.var_mu = assign9760_e8720;
        locals.var_mu_dn0 = assign9760_e8720_d_n0;
        locals.var_mu_dn2 = assign9760_e8720_d_n2;
        locals.var_mu_dn6 = assign9760_e8720_d_n6;
        locals.var_mu_dn7 = assign9760_e8720_d_n7;
        locals.var_mu_dn10 = assign9760_e8720_d_n10;
        locals.var_mu_dn11 = assign9760_e8720_d_n11;
        locals.var_mu_dn12 = assign9760_e8720_d_n12;
        locals.var_mu_dn17 = assign9760_e8720_d_n17;
        locals.var_mu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9770_e8728, assign9770_e8728_d_n0, assign9770_e8728_d_n2, assign9770_e8728_d_n6, assign9770_e8728_d_n7, assign9770_e8728_d_n10, assign9770_e8728_d_n11, assign9770_e8728_d_n12, assign9770_e8728_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign9770_e8728;
        locals.var_lred_dn0 = assign9770_e8728_d_n0;
        locals.var_lred_dn2 = assign9770_e8728_d_n2;
        locals.var_lred_dn6 = assign9770_e8728_d_n6;
        locals.var_lred_dn7 = assign9770_e8728_d_n7;
        locals.var_lred_dn10 = assign9770_e8728_d_n10;
        locals.var_lred_dn11 = assign9770_e8728_d_n11;
        locals.var_lred_dn12 = assign9770_e8728_d_n12;
        locals.var_lred_dn17 = assign9770_e8728_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign9780_e8737, assign9780_e8737_d_n0, assign9780_e8737_d_n2, assign9780_e8737_d_n6, assign9780_e8737_d_n7, assign9780_e8737_d_n10, assign9780_e8737_d_n11, assign9780_e8737_d_n12, assign9780_e8737_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9780_e8735: f64 = 0.0;
        (assign9780_e8735, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign9780_e8737;
        locals.var_t1_dn0 = assign9780_e8737_d_n0;
        locals.var_t1_dn2 = assign9780_e8737_d_n2;
        locals.var_t1_dn6 = assign9780_e8737_d_n6;
        locals.var_t1_dn7 = assign9780_e8737_d_n7;
        locals.var_t1_dn10 = assign9780_e8737_d_n10;
        locals.var_t1_dn11 = assign9780_e8737_d_n11;
        locals.var_t1_dn12 = assign9780_e8737_d_n12;
        locals.var_t1_dn17 = assign9780_e8737_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign9790_e8749, assign9790_e8749_d_n0, assign9790_e8749_d_n2, assign9790_e8749_d_n6, assign9790_e8749_d_n7, assign9790_e8749_d_n10, assign9790_e8749_d_n11, assign9790_e8749_d_n12, assign9790_e8749_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9790_e8746: f64 = (locals.var_leff - locals.var_lred);
        let assign9790_e8747: f64 = (1.0 / assign9790_e8746);
        (assign9790_e8747, (-((-locals.var_lred_dn0) / (assign9790_e8746 * assign9790_e8746))), (-((-locals.var_lred_dn2) / (assign9790_e8746 * assign9790_e8746))), (-((-locals.var_lred_dn6) / (assign9790_e8746 * assign9790_e8746))), (-((-locals.var_lred_dn7) / (assign9790_e8746 * assign9790_e8746))), (-((-locals.var_lred_dn10) / (assign9790_e8746 * assign9790_e8746))), (-((-locals.var_lred_dn11) / (assign9790_e8746 * assign9790_e8746))), (-((-locals.var_lred_dn12) / (assign9790_e8746 * assign9790_e8746))), (-((-locals.var_lred_dn17) / (assign9790_e8746 * assign9790_e8746))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign9790_e8749;
        locals.var_t2_dn0 = assign9790_e8749_d_n0;
        locals.var_t2_dn2 = assign9790_e8749_d_n2;
        locals.var_t2_dn6 = assign9790_e8749_d_n6;
        locals.var_t2_dn7 = assign9790_e8749_d_n7;
        locals.var_t2_dn10 = assign9790_e8749_d_n10;
        locals.var_t2_dn11 = assign9790_e8749_d_n11;
        locals.var_t2_dn12 = assign9790_e8749_d_n12;
        locals.var_t2_dn17 = assign9790_e8749_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign9800_e8765, assign9800_e8765_d_n0, assign9800_e8765_d_n2, assign9800_e8765_d_n6, assign9800_e8765_d_n7, assign9800_e8765_d_n10, assign9800_e8765_d_n11, assign9800_e8765_d_n12, assign9800_e8765_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9800_e8757: f64 = (locals.var_costi7 * locals.var_mu);
        let assign9800_e8759: f64 = (assign9800_e8757 * locals.var_qn0sti);
        let assign9800_e8761: f64 = (assign9800_e8759 * locals.var_t1);
        let assign9800_e8763: f64 = (assign9800_e8761 * locals.var_t2);
        (assign9800_e8763, (((((((locals.var_costi7 * locals.var_mu_dn0) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn0)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn0)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn0)), (((((((locals.var_costi7 * locals.var_mu_dn2) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn2)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn2)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn2)), (((((((locals.var_costi7 * locals.var_mu_dn6) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn6)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn6)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn6)), (((((((locals.var_costi7 * locals.var_mu_dn7) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn7)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn7)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn7)), ((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn10)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn10)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn10)), (((((((locals.var_costi7 * locals.var_mu_dn11) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn11)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn11)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn11)), (((((((locals.var_costi7 * locals.var_mu_dn12) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn12)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn12)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn12)), (((((((locals.var_costi7 * locals.var_mu_dn17) * locals.var_qn0sti) + (assign9800_e8757 * locals.var_qn0sti_dn17)) * locals.var_t1) + (assign9800_e8759 * locals.var_t1_dn17)) * locals.var_t2) + (assign9800_e8761 * locals.var_t2_dn17)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12, locals.var_idssti_dn17,)
    }
};
        locals.var_idssti = assign9800_e8765;
        locals.var_idssti_dn0 = assign9800_e8765_d_n0;
        locals.var_idssti_dn2 = assign9800_e8765_d_n2;
        locals.var_idssti_dn6 = assign9800_e8765_d_n6;
        locals.var_idssti_dn7 = assign9800_e8765_d_n7;
        locals.var_idssti_dn10 = assign9800_e8765_d_n10;
        locals.var_idssti_dn11 = assign9800_e8765_d_n11;
        locals.var_idssti_dn12 = assign9800_e8765_d_n12;
        locals.var_idssti_dn17 = assign9800_e8765_d_n17;
        locals.var_idssti_rv = 0.0;

        let (assign9810_e8773, assign9810_e8773_d_n0, assign9810_e8773_d_n2, assign9810_e8773_d_n6, assign9810_e8773_d_n7, assign9810_e8773_d_n10, assign9810_e8773_d_n11, assign9810_e8773_d_n12, assign9810_e8773_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12, locals.var_idssti_dn17,)
    } else {
        (locals.var_ids_isub, locals.var_ids_isub_dn0, locals.var_ids_isub_dn2, locals.var_ids_isub_dn6, locals.var_ids_isub_dn7, locals.var_ids_isub_dn10, locals.var_ids_isub_dn11, locals.var_ids_isub_dn12, locals.var_ids_isub_dn17,)
    }
};
        locals.var_ids_isub = assign9810_e8773;
        locals.var_ids_isub_dn0 = assign9810_e8773_d_n0;
        locals.var_ids_isub_dn2 = assign9810_e8773_d_n2;
        locals.var_ids_isub_dn6 = assign9810_e8773_d_n6;
        locals.var_ids_isub_dn7 = assign9810_e8773_d_n7;
        locals.var_ids_isub_dn10 = assign9810_e8773_d_n10;
        locals.var_ids_isub_dn11 = assign9810_e8773_d_n11;
        locals.var_ids_isub_dn12 = assign9810_e8773_d_n12;
        locals.var_ids_isub_dn17 = assign9810_e8773_d_n17;
        locals.var_ids_isub_rv = 0.0;

        let (assign9820_e8781, assign9820_e8781_d_n0, assign9820_e8781_d_n2, assign9820_e8781_d_n6, assign9820_e8781_d_n7, assign9820_e8781_d_n10, assign9820_e8781_d_n11, assign9820_e8781_d_n12, assign9820_e8781_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    } else {
        (locals.var_ps0_isub, locals.var_ps0_isub_dn0, locals.var_ps0_isub_dn2, locals.var_ps0_isub_dn6, locals.var_ps0_isub_dn7, locals.var_ps0_isub_dn10, locals.var_ps0_isub_dn11, locals.var_ps0_isub_dn12, locals.var_ps0_isub_dn17,)
    }
};
        locals.var_ps0_isub = assign9820_e8781;
        locals.var_ps0_isub_dn0 = assign9820_e8781_d_n0;
        locals.var_ps0_isub_dn2 = assign9820_e8781_d_n2;
        locals.var_ps0_isub_dn6 = assign9820_e8781_d_n6;
        locals.var_ps0_isub_dn7 = assign9820_e8781_d_n7;
        locals.var_ps0_isub_dn10 = assign9820_e8781_d_n10;
        locals.var_ps0_isub_dn11 = assign9820_e8781_d_n11;
        locals.var_ps0_isub_dn12 = assign9820_e8781_d_n12;
        locals.var_ps0_isub_dn17 = assign9820_e8781_d_n17;
        locals.var_ps0_isub_rv = 0.0;

        let (assign9830_e8801, assign9830_e8801_d_n0, assign9830_e8801_d_n2, assign9830_e8801_d_n6, assign9830_e8801_d_n7, assign9830_e8801_d_n10, assign9830_e8801_d_n11, assign9830_e8801_d_n12, assign9830_e8801_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9830_e8791: f64 = (locals.var_beta * locals.var_vgpz);
        let assign9830_e8793: f64 = (assign9830_e8791 - 1.0);
        let assign9830_e8794: f64 = (4.0 * assign9830_e8793);
        let assign9830_e8797: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign9830_e8798: f64 = (assign9830_e8794 / assign9830_e8797);
        let assign9830_e8799: f64 = (1.0 + assign9830_e8798);
        (assign9830_e8799, ((((4.0 * (locals.var_beta * locals.var_vgpz_dn0)) * assign9830_e8797) - (assign9830_e8794 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign9830_e8797 * assign9830_e8797)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn2)) * assign9830_e8797) - (assign9830_e8794 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign9830_e8797 * assign9830_e8797)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn6)) * assign9830_e8797) - (assign9830_e8794 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign9830_e8797 * assign9830_e8797)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn7)) * assign9830_e8797) - (assign9830_e8794 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign9830_e8797 * assign9830_e8797)), ((((4.0 * ((locals.var_beta_dn10 * locals.var_vgpz) + (locals.var_beta * locals.var_vgpz_dn10))) * assign9830_e8797) - (assign9830_e8794 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign9830_e8797 * assign9830_e8797)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn11)) * assign9830_e8797) - (assign9830_e8794 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign9830_e8797 * assign9830_e8797)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn12)) * assign9830_e8797) - (assign9830_e8794 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign9830_e8797 * assign9830_e8797)), ((((4.0 * (locals.var_beta * locals.var_vgpz_dn17)) * assign9830_e8797) - (assign9830_e8794 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign9830_e8797 * assign9830_e8797)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign9830_e8801;
        locals.var_tx_dn0 = assign9830_e8801_d_n0;
        locals.var_tx_dn2 = assign9830_e8801_d_n2;
        locals.var_tx_dn6 = assign9830_e8801_d_n6;
        locals.var_tx_dn7 = assign9830_e8801_d_n7;
        locals.var_tx_dn10 = assign9830_e8801_d_n10;
        locals.var_tx_dn11 = assign9830_e8801_d_n11;
        locals.var_tx_dn12 = assign9830_e8801_d_n12;
        locals.var_tx_dn17 = assign9830_e8801_d_n17;
        locals.var_tx_rv = 0.0;

        let assign9840_e8805: f64 = (10.0 * 2.220446049250313e-16);
        let assign9840_e8806: f64 = if locals.var_tx < assign9840_e8805 { 1.0 } else { 0.0 };
        locals.var_guard178 = assign9840_e8806;
        locals.var_guard178_rv = 0.0;

        let (assign9850_e8818, assign9850_e8818_d_n0, assign9850_e8818_d_n2, assign9850_e8818_d_n6, assign9850_e8818_d_n7, assign9850_e8818_d_n10, assign9850_e8818_d_n11, assign9850_e8818_d_n12, assign9850_e8818_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) {
        let assign9850_e8816: f64 = (10.0 * 2.220446049250313e-16);
        (assign9850_e8816, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign9850_e8818;
        locals.var_tx_dn0 = assign9850_e8818_d_n0;
        locals.var_tx_dn2 = assign9850_e8818_d_n2;
        locals.var_tx_dn6 = assign9850_e8818_d_n6;
        locals.var_tx_dn7 = assign9850_e8818_d_n7;
        locals.var_tx_dn10 = assign9850_e8818_d_n10;
        locals.var_tx_dn11 = assign9850_e8818_d_n11;
        locals.var_tx_dn12 = assign9850_e8818_d_n12;
        locals.var_tx_dn17 = assign9850_e8818_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign9860_e8837, assign9860_e8837_d_n0, assign9860_e8837_d_n2, assign9860_e8837_d_n6, assign9860_e8837_d_n7, assign9860_e8837_d_n10, assign9860_e8837_d_n11, assign9860_e8837_d_n12, assign9860_e8837_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9860_e8827: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign9860_e8829: f64 = (assign9860_e8827 * 0.5);
        let assign9860_e8832: f64 = (locals.var_tx).sqrt();
        let assign9860_e8833: f64 = (1.0 - assign9860_e8832);
        let assign9860_e8834: f64 = (assign9860_e8829 * assign9860_e8833);
        let assign9860_e8835: f64 = (locals.var_vgpz + assign9860_e8834);
        (assign9860_e8835, (locals.var_vgpz_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn0 / (2.0 * assign9860_e8832)))))), (locals.var_vgpz_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn2 / (2.0 * assign9860_e8832)))))), (locals.var_vgpz_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn6 / (2.0 * assign9860_e8832)))))), (locals.var_vgpz_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn7 / (2.0 * assign9860_e8832)))))), (locals.var_vgpz_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn10 / (2.0 * assign9860_e8832)))))), (locals.var_vgpz_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn11 / (2.0 * assign9860_e8832)))))), (locals.var_vgpz_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn12 / (2.0 * assign9860_e8832)))))), (locals.var_vgpz_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign9860_e8833) + (assign9860_e8829 * (-(locals.var_tx_dn17 / (2.0 * assign9860_e8832)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign9860_e8837;
        locals.var_ps0_inia_dn0 = assign9860_e8837_d_n0;
        locals.var_ps0_inia_dn2 = assign9860_e8837_d_n2;
        locals.var_ps0_inia_dn6 = assign9860_e8837_d_n6;
        locals.var_ps0_inia_dn7 = assign9860_e8837_d_n7;
        locals.var_ps0_inia_dn10 = assign9860_e8837_d_n10;
        locals.var_ps0_inia_dn11 = assign9860_e8837_d_n11;
        locals.var_ps0_inia_dn12 = assign9860_e8837_d_n12;
        locals.var_ps0_inia_dn17 = assign9860_e8837_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign9870_e8845, assign9870_e8845_d_n0, assign9870_e8845_d_n2, assign9870_e8845_d_n6, assign9870_e8845_d_n7, assign9870_e8845_d_n10, assign9870_e8845_d_n11, assign9870_e8845_d_n12, assign9870_e8845_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign9870_e8845;
        locals.var_psl_lim_dn0 = assign9870_e8845_d_n0;
        locals.var_psl_lim_dn2 = assign9870_e8845_d_n2;
        locals.var_psl_lim_dn6 = assign9870_e8845_d_n6;
        locals.var_psl_lim_dn7 = assign9870_e8845_d_n7;
        locals.var_psl_lim_dn10 = assign9870_e8845_d_n10;
        locals.var_psl_lim_dn11 = assign9870_e8845_d_n11;
        locals.var_psl_lim_dn12 = assign9870_e8845_d_n12;
        locals.var_psl_lim_dn17 = assign9870_e8845_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let (assign9880_e8855, assign9880_e8855_d_n0, assign9880_e8855_d_n2, assign9880_e8855_d_n6, assign9880_e8855_d_n7, assign9880_e8855_d_n10, assign9880_e8855_d_n11, assign9880_e8855_d_n12, assign9880_e8855_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9880_e8853: f64 = (locals.var_ps0_inia - locals.var_ps0_isub);
        (assign9880_e8853, (locals.var_ps0_inia_dn0 - locals.var_ps0_isub_dn0), (locals.var_ps0_inia_dn2 - locals.var_ps0_isub_dn2), (locals.var_ps0_inia_dn6 - locals.var_ps0_isub_dn6), (locals.var_ps0_inia_dn7 - locals.var_ps0_isub_dn7), (locals.var_ps0_inia_dn10 - locals.var_ps0_isub_dn10), (locals.var_ps0_inia_dn11 - locals.var_ps0_isub_dn11), (locals.var_ps0_inia_dn12 - locals.var_ps0_isub_dn12), (locals.var_ps0_inia_dn17 - locals.var_ps0_isub_dn17),)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign9880_e8855;
        locals.var_pds_max_dn0 = assign9880_e8855_d_n0;
        locals.var_pds_max_dn2 = assign9880_e8855_d_n2;
        locals.var_pds_max_dn6 = assign9880_e8855_d_n6;
        locals.var_pds_max_dn7 = assign9880_e8855_d_n7;
        locals.var_pds_max_dn10 = assign9880_e8855_d_n10;
        locals.var_pds_max_dn11 = assign9880_e8855_d_n11;
        locals.var_pds_max_dn12 = assign9880_e8855_d_n12;
        locals.var_pds_max_dn17 = assign9880_e8855_d_n17;
        locals.var_pds_max_rv = 0.0;

        let assign9890_e8858: f64 = if locals.var_pds_max < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard179 = assign9890_e8858;
        locals.var_guard179_rv = 0.0;

        let (assign9900_e8868, assign9900_e8868_d_n0, assign9900_e8868_d_n2, assign9900_e8868_d_n6, assign9900_e8868_d_n7, assign9900_e8868_d_n10, assign9900_e8868_d_n11, assign9900_e8868_d_n12, assign9900_e8868_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard179 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign9900_e8868;
        locals.var_pds_max_dn0 = assign9900_e8868_d_n0;
        locals.var_pds_max_dn2 = assign9900_e8868_d_n2;
        locals.var_pds_max_dn6 = assign9900_e8868_d_n6;
        locals.var_pds_max_dn7 = assign9900_e8868_d_n7;
        locals.var_pds_max_dn10 = assign9900_e8868_d_n10;
        locals.var_pds_max_dn11 = assign9900_e8868_d_n11;
        locals.var_pds_max_dn12 = assign9900_e8868_d_n12;
        locals.var_pds_max_dn17 = assign9900_e8868_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign9910_e8880, assign9910_e8880_d_n0, assign9910_e8880_d_n2, assign9910_e8880_d_n6, assign9910_e8880_d_n7, assign9910_e8880_d_n10, assign9910_e8880_d_n11, assign9910_e8880_d_n12, assign9910_e8880_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9910_e8876: f64 = (1.0 + 0.3);
        let assign9910_e8878: f64 = (assign9910_e8876 * locals.var_pds_max);
        (assign9910_e8878, (assign9910_e8876 * locals.var_pds_max_dn0), (assign9910_e8876 * locals.var_pds_max_dn2), (assign9910_e8876 * locals.var_pds_max_dn6), (assign9910_e8876 * locals.var_pds_max_dn7), (assign9910_e8876 * locals.var_pds_max_dn10), (assign9910_e8876 * locals.var_pds_max_dn11), (assign9910_e8876 * locals.var_pds_max_dn12), (assign9910_e8876 * locals.var_pds_max_dn17),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign9910_e8880;
        locals.var_t5_dn0 = assign9910_e8880_d_n0;
        locals.var_t5_dn2 = assign9910_e8880_d_n2;
        locals.var_t5_dn6 = assign9910_e8880_d_n6;
        locals.var_t5_dn7 = assign9910_e8880_d_n7;
        locals.var_t5_dn10 = assign9910_e8880_d_n10;
        locals.var_t5_dn11 = assign9910_e8880_d_n11;
        locals.var_t5_dn12 = assign9910_e8880_d_n12;
        locals.var_t5_dn17 = assign9910_e8880_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign9920_e8892, assign9920_e8892_d_n0, assign9920_e8892_d_n2, assign9920_e8892_d_n6, assign9920_e8892_d_n7, assign9920_e8892_d_n10, assign9920_e8892_d_n11, assign9920_e8892_d_n12, assign9920_e8892_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9920_e8888: f64 = (locals.var_t5 - locals.var_vdsz);
        let assign9920_e8890: f64 = (assign9920_e8888 - 0.03);
        (assign9920_e8890, (locals.var_t5_dn0 - locals.var_vdsz_dn0), (locals.var_t5_dn2 - locals.var_vdsz_dn2), (locals.var_t5_dn6 - locals.var_vdsz_dn6), (locals.var_t5_dn7 - locals.var_vdsz_dn7), (locals.var_t5_dn10 - locals.var_vdsz_dn10), (locals.var_t5_dn11 - locals.var_vdsz_dn11), (locals.var_t5_dn12 - locals.var_vdsz_dn12), (locals.var_t5_dn17 - locals.var_vdsz_dn17),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign9920_e8892;
        locals.var_t6_dn0 = assign9920_e8892_d_n0;
        locals.var_t6_dn2 = assign9920_e8892_d_n2;
        locals.var_t6_dn6 = assign9920_e8892_d_n6;
        locals.var_t6_dn7 = assign9920_e8892_d_n7;
        locals.var_t6_dn10 = assign9920_e8892_d_n10;
        locals.var_t6_dn11 = assign9920_e8892_d_n11;
        locals.var_t6_dn12 = assign9920_e8892_d_n12;
        locals.var_t6_dn17 = assign9920_e8892_d_n17;
        locals.var_t6_rv = 0.0;

        let (assign9930_e8909, assign9930_e8909_d_n0, assign9930_e8909_d_n2, assign9930_e8909_d_n6, assign9930_e8909_d_n7, assign9930_e8909_d_n10, assign9930_e8909_d_n11, assign9930_e8909_d_n12, assign9930_e8909_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9930_e8900: f64 = (locals.var_t6 * locals.var_t6);
        let assign9930_e8903: f64 = (4.0 * locals.var_t5);
        let assign9930_e8905: f64 = (assign9930_e8903 * 0.03);
        let assign9930_e8906: f64 = (assign9930_e8900 + assign9930_e8905);
        let assign9930_e8907: f64 = (assign9930_e8906).sqrt();
        (assign9930_e8907, ((((locals.var_t6_dn0 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn0)) + ((4.0 * locals.var_t5_dn0) * 0.03)) / (2.0 * assign9930_e8907)), ((((locals.var_t6_dn2 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn2)) + ((4.0 * locals.var_t5_dn2) * 0.03)) / (2.0 * assign9930_e8907)), ((((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) + ((4.0 * locals.var_t5_dn6) * 0.03)) / (2.0 * assign9930_e8907)), ((((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) + ((4.0 * locals.var_t5_dn7) * 0.03)) / (2.0 * assign9930_e8907)), ((((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) + ((4.0 * locals.var_t5_dn10) * 0.03)) / (2.0 * assign9930_e8907)), ((((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) + ((4.0 * locals.var_t5_dn11) * 0.03)) / (2.0 * assign9930_e8907)), ((((locals.var_t6_dn12 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn12)) + ((4.0 * locals.var_t5_dn12) * 0.03)) / (2.0 * assign9930_e8907)), ((((locals.var_t6_dn17 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn17)) + ((4.0 * locals.var_t5_dn17) * 0.03)) / (2.0 * assign9930_e8907)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
        locals.var_t7 = assign9930_e8909;
        locals.var_t7_dn0 = assign9930_e8909_d_n0;
        locals.var_t7_dn2 = assign9930_e8909_d_n2;
        locals.var_t7_dn6 = assign9930_e8909_d_n6;
        locals.var_t7_dn7 = assign9930_e8909_d_n7;
        locals.var_t7_dn10 = assign9930_e8909_d_n10;
        locals.var_t7_dn11 = assign9930_e8909_d_n11;
        locals.var_t7_dn12 = assign9930_e8909_d_n12;
        locals.var_t7_dn17 = assign9930_e8909_d_n17;
        locals.var_t7_rv = 0.0;

        let (assign9940_e8923, assign9940_e8923_d_n0, assign9940_e8923_d_n2, assign9940_e8923_d_n6, assign9940_e8923_d_n7, assign9940_e8923_d_n10, assign9940_e8923_d_n11, assign9940_e8923_d_n12, assign9940_e8923_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9940_e8919: f64 = (locals.var_t6 + locals.var_t7);
        let assign9940_e8920: f64 = (0.5 * assign9940_e8919);
        let assign9940_e8921: f64 = (locals.var_t5 - assign9940_e8920);
        (assign9940_e8921, (locals.var_t5_dn0 - (0.5 * (locals.var_t6_dn0 + locals.var_t7_dn0))), (locals.var_t5_dn2 - (0.5 * (locals.var_t6_dn2 + locals.var_t7_dn2))), (locals.var_t5_dn6 - (0.5 * (locals.var_t6_dn6 + locals.var_t7_dn6))), (locals.var_t5_dn7 - (0.5 * (locals.var_t6_dn7 + locals.var_t7_dn7))), (locals.var_t5_dn10 - (0.5 * (locals.var_t6_dn10 + locals.var_t7_dn10))), (locals.var_t5_dn11 - (0.5 * (locals.var_t6_dn11 + locals.var_t7_dn11))), (locals.var_t5_dn12 - (0.5 * (locals.var_t6_dn12 + locals.var_t7_dn12))), (locals.var_t5_dn17 - (0.5 * (locals.var_t6_dn17 + locals.var_t7_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign9940_e8923;
        locals.var_pds_ini_dn0 = assign9940_e8923_d_n0;
        locals.var_pds_ini_dn2 = assign9940_e8923_d_n2;
        locals.var_pds_ini_dn6 = assign9940_e8923_d_n6;
        locals.var_pds_ini_dn7 = assign9940_e8923_d_n7;
        locals.var_pds_ini_dn10 = assign9940_e8923_d_n10;
        locals.var_pds_ini_dn11 = assign9940_e8923_d_n11;
        locals.var_pds_ini_dn12 = assign9940_e8923_d_n12;
        locals.var_pds_ini_dn17 = assign9940_e8923_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign9950_e8926: f64 = if locals.var_pds_ini > locals.var_pds_max { 1.0 } else { 0.0 };
        locals.var_guard180 = assign9950_e8926;
        locals.var_guard180_rv = 0.0;

        let (assign9960_e8936, assign9960_e8936_d_n0, assign9960_e8936_d_n2, assign9960_e8936_d_n6, assign9960_e8936_d_n7, assign9960_e8936_d_n10, assign9960_e8936_d_n11, assign9960_e8936_d_n12, assign9960_e8936_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard180 != 0.0)) {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign9960_e8936;
        locals.var_pds_ini_dn0 = assign9960_e8936_d_n0;
        locals.var_pds_ini_dn2 = assign9960_e8936_d_n2;
        locals.var_pds_ini_dn6 = assign9960_e8936_d_n6;
        locals.var_pds_ini_dn7 = assign9960_e8936_d_n7;
        locals.var_pds_ini_dn10 = assign9960_e8936_d_n10;
        locals.var_pds_ini_dn11 = assign9960_e8936_d_n11;
        locals.var_pds_ini_dn12 = assign9960_e8936_d_n12;
        locals.var_pds_ini_dn17 = assign9960_e8936_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign9970_e8944, assign9970_e8944_d_n0, assign9970_e8944_d_n2, assign9970_e8944_d_n6, assign9970_e8944_d_n7, assign9970_e8944_d_n10, assign9970_e8944_d_n11, assign9970_e8944_d_n12, assign9970_e8944_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds_qwe, locals.var_pds_qwe_dn0, locals.var_pds_qwe_dn2, locals.var_pds_qwe_dn6, locals.var_pds_qwe_dn7, locals.var_pds_qwe_dn10, locals.var_pds_qwe_dn11, locals.var_pds_qwe_dn12, locals.var_pds_qwe_dn17,)
    }
};
        locals.var_pds_qwe = assign9970_e8944;
        locals.var_pds_qwe_dn0 = assign9970_e8944_d_n0;
        locals.var_pds_qwe_dn2 = assign9970_e8944_d_n2;
        locals.var_pds_qwe_dn6 = assign9970_e8944_d_n6;
        locals.var_pds_qwe_dn7 = assign9970_e8944_d_n7;
        locals.var_pds_qwe_dn10 = assign9970_e8944_d_n10;
        locals.var_pds_qwe_dn11 = assign9970_e8944_d_n11;
        locals.var_pds_qwe_dn12 = assign9970_e8944_d_n12;
        locals.var_pds_qwe_dn17 = assign9970_e8944_d_n17;
        locals.var_pds_qwe_rv = 0.0;

        let (assign9980_e8954,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9980_e8952: f64 = (locals.var_tfox0 * 100.0);
        (assign9980_e8952,)
    } else {
        (locals.var_cgs_tfox0,)
    }
};
        locals.var_cgs_tfox0 = assign9980_e8954;
        locals.var_cgs_tfox0_rv = 0.0;

        let (assign9990_e8964,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign9990_e8962: f64 = (locals.var_weff_nf * 100.0);
        (assign9990_e8962,)
    } else {
        (locals.var_cgs_weff_nf,)
    }
};
        locals.var_cgs_weff_nf = assign9990_e8964;
        locals.var_cgs_weff_nf_rv = 0.0;

        let (assign10000_e8974,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) {
        let assign10000_e8972: f64 = (locals.var_leff * 100.0);
        (assign10000_e8972,)
    } else {
        (locals.var_cgs_leff,)
    }
};
        locals.var_cgs_leff = assign10000_e8974;
        locals.var_cgs_leff_rv = 0.0;

        let assign10010_e8977: f64 = if p.p36 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign10010_e8977;
        locals.var_guard201_rv = 0.0;

        let (assign10030_e8998,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        (4.12,)
    } else {
        (locals.var_phib,)
    }
};
        locals.var_phib = assign10030_e8998;
        locals.var_phib_rv = 0.0;

        let (assign10040_e9015,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10040_e9009: f64 = (p.p142 * 1.6021918e-19);
        let assign10040_e9011: f64 = (assign10040_e9009 * locals.var_cgs_weff_nf);
        let assign10040_e9013: f64 = (assign10040_e9011 * locals.var_cgs_leff);
        (assign10040_e9013,)
    } else {
        (locals.var_evb1_qe_wl,)
    }
};
        locals.var_evb1_qe_wl = assign10040_e9015;
        locals.var_evb1_qe_wl_rv = 0.0;

        let (assign10050_e9028, assign10050_e9028_d_n0, assign10050_e9028_d_n2, assign10050_e9028_d_n6, assign10050_e9028_d_n7, assign10050_e9028_d_n10, assign10050_e9028_d_n11, assign10050_e9028_d_n12, assign10050_e9028_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10050_e9026: f64 = (locals.var_evb1_qe_wl / locals.var_egp12);
        (assign10050_e9026, (-((locals.var_evb1_qe_wl * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))), (-((locals.var_evb1_qe_wl * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_evb1_qe_wl_p_egp12, locals.var_evb1_qe_wl_p_egp12_dn0, locals.var_evb1_qe_wl_p_egp12_dn2, locals.var_evb1_qe_wl_p_egp12_dn6, locals.var_evb1_qe_wl_p_egp12_dn7, locals.var_evb1_qe_wl_p_egp12_dn10, locals.var_evb1_qe_wl_p_egp12_dn11, locals.var_evb1_qe_wl_p_egp12_dn12, locals.var_evb1_qe_wl_p_egp12_dn17,)
    }
};
        locals.var_evb1_qe_wl_p_egp12 = assign10050_e9028;
        locals.var_evb1_qe_wl_p_egp12_dn0 = assign10050_e9028_d_n0;
        locals.var_evb1_qe_wl_p_egp12_dn2 = assign10050_e9028_d_n2;
        locals.var_evb1_qe_wl_p_egp12_dn6 = assign10050_e9028_d_n6;
        locals.var_evb1_qe_wl_p_egp12_dn7 = assign10050_e9028_d_n7;
        locals.var_evb1_qe_wl_p_egp12_dn10 = assign10050_e9028_d_n10;
        locals.var_evb1_qe_wl_p_egp12_dn11 = assign10050_e9028_d_n11;
        locals.var_evb1_qe_wl_p_egp12_dn12 = assign10050_e9028_d_n12;
        locals.var_evb1_qe_wl_p_egp12_dn17 = assign10050_e9028_d_n17;
        locals.var_evb1_qe_wl_p_egp12_rv = 0.0;

        let (assign10060_e9052, assign10060_e9052_d_n0, assign10060_e9052_d_n2, assign10060_e9052_d_n6, assign10060_e9052_d_n7, assign10060_e9052_d_n10, assign10060_e9052_d_n11, assign10060_e9052_d_n12, assign10060_e9052_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10060_e9039: f64 = (p.p145 * locals.var_vbspz);
        let assign10060_e9041: f64 = (assign10060_e9039 + locals.var_dvthsc);
        let assign10060_e9043: f64 = (assign10060_e9041 + locals.var_dvthlp);
        let assign10060_e9045: f64 = (assign10060_e9043 + locals.var_eg);
        let assign10060_e9047: f64 = (assign10060_e9045 + p.p144);
        let assign10060_e9048: f64 = (-assign10060_e9047);
        let assign10060_e9050: f64 = (assign10060_e9048 / locals.var_cgs_tfox0);
        (assign10060_e9050, ((-((((p.p145 * locals.var_vbspz_dn0) + locals.var_dvthsc_dn0) + locals.var_dvthlp_dn0) + locals.var_eg_dn0)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn2) + locals.var_dvthsc_dn2) + locals.var_dvthlp_dn2) + locals.var_eg_dn2)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn6) + locals.var_dvthsc_dn6) + locals.var_dvthlp_dn6) + locals.var_eg_dn6)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn7) + locals.var_dvthsc_dn7) + locals.var_dvthlp_dn7) + locals.var_eg_dn7)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn10) + locals.var_dvthsc_dn10) + locals.var_dvthlp_dn10) + locals.var_eg_dn10)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn11) + locals.var_dvthsc_dn11) + locals.var_dvthlp_dn11) + locals.var_eg_dn11)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn12) + locals.var_dvthsc_dn12) + locals.var_dvthlp_dn12) + locals.var_eg_dn12)) / locals.var_cgs_tfox0), ((-((((p.p145 * locals.var_vbspz_dn17) + locals.var_dvthsc_dn17) + locals.var_dvthlp_dn17) + locals.var_eg_dn17)) / locals.var_cgs_tfox0),)
    } else {
        (locals.var_eevb_wo_vox, locals.var_eevb_wo_vox_dn0, locals.var_eevb_wo_vox_dn2, locals.var_eevb_wo_vox_dn6, locals.var_eevb_wo_vox_dn7, locals.var_eevb_wo_vox_dn10, locals.var_eevb_wo_vox_dn11, locals.var_eevb_wo_vox_dn12, locals.var_eevb_wo_vox_dn17,)
    }
};
        locals.var_eevb_wo_vox = assign10060_e9052;
        locals.var_eevb_wo_vox_dn0 = assign10060_e9052_d_n0;
        locals.var_eevb_wo_vox_dn2 = assign10060_e9052_d_n2;
        locals.var_eevb_wo_vox_dn6 = assign10060_e9052_d_n6;
        locals.var_eevb_wo_vox_dn7 = assign10060_e9052_d_n7;
        locals.var_eevb_wo_vox_dn10 = assign10060_e9052_d_n10;
        locals.var_eevb_wo_vox_dn11 = assign10060_e9052_d_n11;
        locals.var_eevb_wo_vox_dn12 = assign10060_e9052_d_n12;
        locals.var_eevb_wo_vox_dn17 = assign10060_e9052_d_n17;
        locals.var_eevb_wo_vox_rv = 0.0;

        let (assign10070_e9063,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign10070_e9063;
        locals.var_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign10080_loop_guard: usize = 0;
        while {
            let assign10080_cond_e9075: f64 = (100.0 - 1.0);
            let assign10080_cond_e9077: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_i <= assign10080_cond_e9075)) { 1.0 } else { 0.0 };
            assign10080_cond_e9077 != 0.0
        } {
            assign10080_loop_guard += 1;
            assert!(assign10080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10080_body0_e9088,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        (locals.var_i,)
    } else {
        (locals.var_reali,)
    }
};
            locals.var_reali = assign10080_body0_e9088;
            locals.var_reali_rv = 0.0;
            let (assign10080_body1_e9099,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        (100.0,)
    } else {
        (locals.var_realn,)
    }
};
            locals.var_realn = assign10080_body1_e9099;
            locals.var_realn_rv = 0.0;
            let (assign10080_body2_e9112,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body2_e9110: f64 = (locals.var_reali / locals.var_realn);
        (assign10080_body2_e9110,)
    } else {
        (locals.var_r,)
    }
};
            locals.var_r = assign10080_body2_e9112;
            locals.var_r_rv = 0.0;
            let (assign10080_body3_e9131, assign10080_body3_e9131_d_n0, assign10080_body3_e9131_d_n2, assign10080_body3_e9131_d_n6, assign10080_body3_e9131_d_n7, assign10080_body3_e9131_d_n10, assign10080_body3_e9131_d_n11, assign10080_body3_e9131_d_n12, assign10080_body3_e9131_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body3_e9123: f64 = (locals.var_vgp + locals.var_vzadd);
        let assign10080_body3_e9126: f64 = (locals.var_pds_qwe * locals.var_r);
        let assign10080_body3_e9128: f64 = (assign10080_body3_e9126 + locals.var_ps0_isub);
        let assign10080_body3_e9129: f64 = (assign10080_body3_e9123 - assign10080_body3_e9128);
        (assign10080_body3_e9129, ((locals.var_vgp_dn0 + locals.var_vzadd_dn0) - ((locals.var_pds_qwe_dn0 * locals.var_r) + locals.var_ps0_isub_dn0)), ((locals.var_vgp_dn2 + locals.var_vzadd_dn2) - ((locals.var_pds_qwe_dn2 * locals.var_r) + locals.var_ps0_isub_dn2)), ((locals.var_vgp_dn6 + locals.var_vzadd_dn6) - ((locals.var_pds_qwe_dn6 * locals.var_r) + locals.var_ps0_isub_dn6)), ((locals.var_vgp_dn7 + locals.var_vzadd_dn7) - ((locals.var_pds_qwe_dn7 * locals.var_r) + locals.var_ps0_isub_dn7)), ((locals.var_vgp_dn10 + locals.var_vzadd_dn10) - ((locals.var_pds_qwe_dn10 * locals.var_r) + locals.var_ps0_isub_dn10)), ((locals.var_vgp_dn11 + locals.var_vzadd_dn11) - ((locals.var_pds_qwe_dn11 * locals.var_r) + locals.var_ps0_isub_dn11)), ((locals.var_vgp_dn12 + locals.var_vzadd_dn12) - ((locals.var_pds_qwe_dn12 * locals.var_r) + locals.var_ps0_isub_dn12)), ((locals.var_vgp_dn17 + locals.var_vzadd_dn17) - ((locals.var_pds_qwe_dn17 * locals.var_r) + locals.var_ps0_isub_dn17)),)
    } else {
        (locals.var_vox, locals.var_vox_dn0, locals.var_vox_dn2, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12, locals.var_vox_dn17,)
    }
};
            locals.var_vox = assign10080_body3_e9131;
            locals.var_vox_dn0 = assign10080_body3_e9131_d_n0;
            locals.var_vox_dn2 = assign10080_body3_e9131_d_n2;
            locals.var_vox_dn6 = assign10080_body3_e9131_d_n6;
            locals.var_vox_dn7 = assign10080_body3_e9131_d_n7;
            locals.var_vox_dn10 = assign10080_body3_e9131_d_n10;
            locals.var_vox_dn11 = assign10080_body3_e9131_d_n11;
            locals.var_vox_dn12 = assign10080_body3_e9131_d_n12;
            locals.var_vox_dn17 = assign10080_body3_e9131_d_n17;
            locals.var_vox_rv = 0.0;
            let (assign10080_body4_e9146, assign10080_body4_e9146_d_n0, assign10080_body4_e9146_d_n2, assign10080_body4_e9146_d_n6, assign10080_body4_e9146_d_n7, assign10080_body4_e9146_d_n10, assign10080_body4_e9146_d_n11, assign10080_body4_e9146_d_n12, assign10080_body4_e9146_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body4_e9143: f64 = (locals.var_vox / locals.var_phib);
        let assign10080_body4_e9144: f64 = (1.0 - assign10080_body4_e9143);
        (assign10080_body4_e9144, (-(locals.var_vox_dn0 / locals.var_phib)), (-(locals.var_vox_dn2 / locals.var_phib)), (-(locals.var_vox_dn6 / locals.var_phib)), (-(locals.var_vox_dn7 / locals.var_phib)), (-(locals.var_vox_dn10 / locals.var_phib)), (-(locals.var_vox_dn11 / locals.var_phib)), (-(locals.var_vox_dn12 / locals.var_phib)), (-(locals.var_vox_dn17 / locals.var_phib)),)
    } else {
        (locals.var_d0, locals.var_d0_dn0, locals.var_d0_dn2, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn10, locals.var_d0_dn11, locals.var_d0_dn12, locals.var_d0_dn17,)
    }
};
            locals.var_d0 = assign10080_body4_e9146;
            locals.var_d0_dn0 = assign10080_body4_e9146_d_n0;
            locals.var_d0_dn2 = assign10080_body4_e9146_d_n2;
            locals.var_d0_dn6 = assign10080_body4_e9146_d_n6;
            locals.var_d0_dn7 = assign10080_body4_e9146_d_n7;
            locals.var_d0_dn10 = assign10080_body4_e9146_d_n10;
            locals.var_d0_dn11 = assign10080_body4_e9146_d_n11;
            locals.var_d0_dn12 = assign10080_body4_e9146_d_n12;
            locals.var_d0_dn17 = assign10080_body4_e9146_d_n17;
            locals.var_d0_rv = 0.0;
            let (assign10080_body5_e9161, assign10080_body5_e9161_d_n0, assign10080_body5_e9161_d_n2, assign10080_body5_e9161_d_n6, assign10080_body5_e9161_d_n7, assign10080_body5_e9161_d_n10, assign10080_body5_e9161_d_n11, assign10080_body5_e9161_d_n12, assign10080_body5_e9161_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body5_e9158: f64 = (locals.var_vox / locals.var_cgs_tfox0);
        let assign10080_body5_e9159: f64 = (locals.var_eevb_wo_vox + assign10080_body5_e9158);
        (assign10080_body5_e9159, (locals.var_eevb_wo_vox_dn0 + (locals.var_vox_dn0 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn2 + (locals.var_vox_dn2 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn6 + (locals.var_vox_dn6 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn7 + (locals.var_vox_dn7 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn10 + (locals.var_vox_dn10 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn11 + (locals.var_vox_dn11 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn12 + (locals.var_vox_dn12 / locals.var_cgs_tfox0)), (locals.var_eevb_wo_vox_dn17 + (locals.var_vox_dn17 / locals.var_cgs_tfox0)),)
    } else {
        (locals.var_t2__blk191, locals.var_t2__blk191_dn0, locals.var_t2__blk191_dn2, locals.var_t2__blk191_dn6, locals.var_t2__blk191_dn7, locals.var_t2__blk191_dn10, locals.var_t2__blk191_dn11, locals.var_t2__blk191_dn12, locals.var_t2__blk191_dn17,)
    }
};
            locals.var_t2__blk191 = assign10080_body5_e9161;
            locals.var_t2__blk191_dn0 = assign10080_body5_e9161_d_n0;
            locals.var_t2__blk191_dn2 = assign10080_body5_e9161_d_n2;
            locals.var_t2__blk191_dn6 = assign10080_body5_e9161_d_n6;
            locals.var_t2__blk191_dn7 = assign10080_body5_e9161_d_n7;
            locals.var_t2__blk191_dn10 = assign10080_body5_e9161_d_n10;
            locals.var_t2__blk191_dn11 = assign10080_body5_e9161_d_n11;
            locals.var_t2__blk191_dn12 = assign10080_body5_e9161_d_n12;
            locals.var_t2__blk191_dn17 = assign10080_body5_e9161_d_n17;
            locals.var_t2__blk191_rv = 0.0;
            let (assign10080_body6_e9174, assign10080_body6_e9174_d_n0, assign10080_body6_e9174_d_n2, assign10080_body6_e9174_d_n6, assign10080_body6_e9174_d_n7, assign10080_body6_e9174_d_n10, assign10080_body6_e9174_d_n11, assign10080_body6_e9174_d_n12, assign10080_body6_e9174_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body6_e9172: f64 = (locals.var_t2__blk191 * locals.var_t2__blk191);
        (assign10080_body6_e9172, ((locals.var_t2__blk191_dn0 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn0)), ((locals.var_t2__blk191_dn2 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn2)), ((locals.var_t2__blk191_dn6 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn6)), ((locals.var_t2__blk191_dn7 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn7)), ((locals.var_t2__blk191_dn10 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn10)), ((locals.var_t2__blk191_dn11 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn11)), ((locals.var_t2__blk191_dn12 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn12)), ((locals.var_t2__blk191_dn17 * locals.var_t2__blk191) + (locals.var_t2__blk191 * locals.var_t2__blk191_dn17)),)
    } else {
        (locals.var_t0__blk189, locals.var_t0__blk189_dn0, locals.var_t0__blk189_dn2, locals.var_t0__blk189_dn6, locals.var_t0__blk189_dn7, locals.var_t0__blk189_dn10, locals.var_t0__blk189_dn11, locals.var_t0__blk189_dn12, locals.var_t0__blk189_dn17,)
    }
};
            locals.var_t0__blk189 = assign10080_body6_e9174;
            locals.var_t0__blk189_dn0 = assign10080_body6_e9174_d_n0;
            locals.var_t0__blk189_dn2 = assign10080_body6_e9174_d_n2;
            locals.var_t0__blk189_dn6 = assign10080_body6_e9174_d_n6;
            locals.var_t0__blk189_dn7 = assign10080_body6_e9174_d_n7;
            locals.var_t0__blk189_dn10 = assign10080_body6_e9174_d_n10;
            locals.var_t0__blk189_dn11 = assign10080_body6_e9174_d_n11;
            locals.var_t0__blk189_dn12 = assign10080_body6_e9174_d_n12;
            locals.var_t0__blk189_dn17 = assign10080_body6_e9174_d_n17;
            locals.var_t0__blk189_rv = 0.0;
            let (assign10080_body7_e9194, assign10080_body7_e9194_d_n0, assign10080_body7_e9194_d_n2, assign10080_body7_e9194_d_n6, assign10080_body7_e9194_d_n7, assign10080_body7_e9194_d_n10, assign10080_body7_e9194_d_n11, assign10080_body7_e9194_d_n12, assign10080_body7_e9194_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body7_e9185: f64 = (locals.var_d0 * locals.var_d0);
        let assign10080_body7_e9188: f64 = (4.0 * 0.001);
        let assign10080_body7_e9190: f64 = (assign10080_body7_e9188 * 0.001);
        let assign10080_body7_e9191: f64 = (assign10080_body7_e9185 + assign10080_body7_e9190);
        let assign10080_body7_e9192: f64 = (assign10080_body7_e9191).sqrt();
        (assign10080_body7_e9192, (((locals.var_d0_dn0 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn0)) / (2.0 * assign10080_body7_e9192)), (((locals.var_d0_dn2 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn2)) / (2.0 * assign10080_body7_e9192)), (((locals.var_d0_dn6 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn6)) / (2.0 * assign10080_body7_e9192)), (((locals.var_d0_dn7 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn7)) / (2.0 * assign10080_body7_e9192)), (((locals.var_d0_dn10 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn10)) / (2.0 * assign10080_body7_e9192)), (((locals.var_d0_dn11 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn11)) / (2.0 * assign10080_body7_e9192)), (((locals.var_d0_dn12 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn12)) / (2.0 * assign10080_body7_e9192)), (((locals.var_d0_dn17 * locals.var_d0) + (locals.var_d0 * locals.var_d0_dn17)) / (2.0 * assign10080_body7_e9192)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign10080_body7_e9194;
            locals.var_tmf1_dn0 = assign10080_body7_e9194_d_n0;
            locals.var_tmf1_dn2 = assign10080_body7_e9194_d_n2;
            locals.var_tmf1_dn6 = assign10080_body7_e9194_d_n6;
            locals.var_tmf1_dn7 = assign10080_body7_e9194_d_n7;
            locals.var_tmf1_dn10 = assign10080_body7_e9194_d_n10;
            locals.var_tmf1_dn11 = assign10080_body7_e9194_d_n11;
            locals.var_tmf1_dn12 = assign10080_body7_e9194_d_n12;
            locals.var_tmf1_dn17 = assign10080_body7_e9194_d_n17;
            locals.var_tmf1_rv = 0.0;
            let (assign10080_body8_e9213, assign10080_body8_e9213_d_n0, assign10080_body8_e9213_d_n2, assign10080_body8_e9213_d_n6, assign10080_body8_e9213_d_n7, assign10080_body8_e9213_d_n10, assign10080_body8_e9213_d_n11, assign10080_body8_e9213_d_n12, assign10080_body8_e9213_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body8_e9206: f64 = (locals.var_d0 + locals.var_tmf1);
        let assign10080_body8_e9207: f64 = (0.5 * assign10080_body8_e9206);
        let assign10080_body8_e9210: f64 = (1e-10 * 0.001);
        let assign10080_body8_e9211: f64 = (assign10080_body8_e9207 + assign10080_body8_e9210);
        (assign10080_body8_e9211, (0.5 * (locals.var_d0_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_d0_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_d0_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_d0_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_d0_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_d0_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_d0_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_d0_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_d0, locals.var_d0_dn0, locals.var_d0_dn2, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn10, locals.var_d0_dn11, locals.var_d0_dn12, locals.var_d0_dn17,)
    }
};
            locals.var_d0 = assign10080_body8_e9213;
            locals.var_d0_dn0 = assign10080_body8_e9213_d_n0;
            locals.var_d0_dn2 = assign10080_body8_e9213_d_n2;
            locals.var_d0_dn6 = assign10080_body8_e9213_d_n6;
            locals.var_d0_dn7 = assign10080_body8_e9213_d_n7;
            locals.var_d0_dn10 = assign10080_body8_e9213_d_n10;
            locals.var_d0_dn11 = assign10080_body8_e9213_d_n11;
            locals.var_d0_dn12 = assign10080_body8_e9213_d_n12;
            locals.var_d0_dn17 = assign10080_body8_e9213_d_n17;
            locals.var_d0_rv = 0.0;
            let assign10080_body9_e9216: f64 = if locals.var_d0 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard202 = assign10080_body9_e9216;
            locals.var_guard202_rv = 0.0;
            let (assign10080_body10_e9229, assign10080_body10_e9229_d_n0, assign10080_body10_e9229_d_n2, assign10080_body10_e9229_d_n6, assign10080_body10_e9229_d_n7, assign10080_body10_e9229_d_n10, assign10080_body10_e9229_d_n11, assign10080_body10_e9229_d_n12, assign10080_body10_e9229_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard202 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_d0, locals.var_d0_dn0, locals.var_d0_dn2, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn10, locals.var_d0_dn11, locals.var_d0_dn12, locals.var_d0_dn17,)
    }
};
            locals.var_d0 = assign10080_body10_e9229;
            locals.var_d0_dn0 = assign10080_body10_e9229_d_n0;
            locals.var_d0_dn2 = assign10080_body10_e9229_d_n2;
            locals.var_d0_dn6 = assign10080_body10_e9229_d_n6;
            locals.var_d0_dn7 = assign10080_body10_e9229_d_n7;
            locals.var_d0_dn10 = assign10080_body10_e9229_d_n10;
            locals.var_d0_dn11 = assign10080_body10_e9229_d_n11;
            locals.var_d0_dn12 = assign10080_body10_e9229_d_n12;
            locals.var_d0_dn17 = assign10080_body10_e9229_d_n17;
            locals.var_d0_rv = 0.0;
            let (assign10080_body11_e9247, assign10080_body11_e9247_d_n0, assign10080_body11_e9247_d_n2, assign10080_body11_e9247_d_n6, assign10080_body11_e9247_d_n7, assign10080_body11_e9247_d_n10, assign10080_body11_e9247_d_n11, assign10080_body11_e9247_d_n12, assign10080_body11_e9247_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body11_e9241: f64 = (locals.var_d0).sqrt();
        let assign10080_body11_e9243: f64 = (assign10080_body11_e9241 * locals.var_d0);
        let assign10080_body11_e9244: f64 = (1.0 - assign10080_body11_e9243);
        let assign10080_body11_e9245: f64 = (p.p143 * assign10080_body11_e9244);
        (assign10080_body11_e9245, (p.p143 * (-(((locals.var_d0_dn0 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn0)))), (p.p143 * (-(((locals.var_d0_dn2 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn2)))), (p.p143 * (-(((locals.var_d0_dn6 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn6)))), (p.p143 * (-(((locals.var_d0_dn7 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn7)))), (p.p143 * (-(((locals.var_d0_dn10 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn10)))), (p.p143 * (-(((locals.var_d0_dn11 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn11)))), (p.p143 * (-(((locals.var_d0_dn12 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn12)))), (p.p143 * (-(((locals.var_d0_dn17 / (2.0 * assign10080_body11_e9241)) * locals.var_d0) + (assign10080_body11_e9241 * locals.var_d0_dn17)))),)
    } else {
        (locals.var_t1__blk190, locals.var_t1__blk190_dn0, locals.var_t1__blk190_dn2, locals.var_t1__blk190_dn6, locals.var_t1__blk190_dn7, locals.var_t1__blk190_dn10, locals.var_t1__blk190_dn11, locals.var_t1__blk190_dn12, locals.var_t1__blk190_dn17,)
    }
};
            locals.var_t1__blk190 = assign10080_body11_e9247;
            locals.var_t1__blk190_dn0 = assign10080_body11_e9247_d_n0;
            locals.var_t1__blk190_dn2 = assign10080_body11_e9247_d_n2;
            locals.var_t1__blk190_dn6 = assign10080_body11_e9247_d_n6;
            locals.var_t1__blk190_dn7 = assign10080_body11_e9247_d_n7;
            locals.var_t1__blk190_dn10 = assign10080_body11_e9247_d_n10;
            locals.var_t1__blk190_dn11 = assign10080_body11_e9247_d_n11;
            locals.var_t1__blk190_dn12 = assign10080_body11_e9247_d_n12;
            locals.var_t1__blk190_dn17 = assign10080_body11_e9247_d_n17;
            locals.var_t1__blk190_rv = 0.0;
            let (assign10080_body12_e9261, assign10080_body12_e9261_d_n0, assign10080_body12_e9261_d_n2, assign10080_body12_e9261_d_n6, assign10080_body12_e9261_d_n7, assign10080_body12_e9261_d_n10, assign10080_body12_e9261_d_n11, assign10080_body12_e9261_d_n12, assign10080_body12_e9261_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body12_e9257: f64 = (-locals.var_t1__blk190);
        let assign10080_body12_e9259: f64 = (assign10080_body12_e9257 / locals.var_t2__blk191);
        (assign10080_body12_e9259, ((((-locals.var_t1__blk190_dn0) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn0)) / (locals.var_t2__blk191 * locals.var_t2__blk191)), ((((-locals.var_t1__blk190_dn2) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn2)) / (locals.var_t2__blk191 * locals.var_t2__blk191)), ((((-locals.var_t1__blk190_dn6) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn6)) / (locals.var_t2__blk191 * locals.var_t2__blk191)), ((((-locals.var_t1__blk190_dn7) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn7)) / (locals.var_t2__blk191 * locals.var_t2__blk191)), ((((-locals.var_t1__blk190_dn10) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn10)) / (locals.var_t2__blk191 * locals.var_t2__blk191)), ((((-locals.var_t1__blk190_dn11) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn11)) / (locals.var_t2__blk191 * locals.var_t2__blk191)), ((((-locals.var_t1__blk190_dn12) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn12)) / (locals.var_t2__blk191 * locals.var_t2__blk191)), ((((-locals.var_t1__blk190_dn17) * locals.var_t2__blk191) - (assign10080_body12_e9257 * locals.var_t2__blk191_dn17)) / (locals.var_t2__blk191 * locals.var_t2__blk191)),)
    } else {
        (locals.var_t3__blk192, locals.var_t3__blk192_dn0, locals.var_t3__blk192_dn2, locals.var_t3__blk192_dn6, locals.var_t3__blk192_dn7, locals.var_t3__blk192_dn10, locals.var_t3__blk192_dn11, locals.var_t3__blk192_dn12, locals.var_t3__blk192_dn17,)
    }
};
            locals.var_t3__blk192 = assign10080_body12_e9261;
            locals.var_t3__blk192_dn0 = assign10080_body12_e9261_d_n0;
            locals.var_t3__blk192_dn2 = assign10080_body12_e9261_d_n2;
            locals.var_t3__blk192_dn6 = assign10080_body12_e9261_d_n6;
            locals.var_t3__blk192_dn7 = assign10080_body12_e9261_d_n7;
            locals.var_t3__blk192_dn10 = assign10080_body12_e9261_d_n10;
            locals.var_t3__blk192_dn11 = assign10080_body12_e9261_d_n11;
            locals.var_t3__blk192_dn12 = assign10080_body12_e9261_d_n12;
            locals.var_t3__blk192_dn17 = assign10080_body12_e9261_d_n17;
            locals.var_t3__blk192_rv = 0.0;
            let assign10080_body13_e9264: f64 = (-34.0);
            let assign10080_body13_e9265: f64 = if locals.var_t3__blk192 < assign10080_body13_e9264 { 1.0 } else { 0.0 };
            locals.var_guard203 = assign10080_body13_e9265;
            locals.var_guard203_rv = 0.0;
            let (assign10080_body14_e9278, assign10080_body14_e9278_d_n0, assign10080_body14_e9278_d_n2, assign10080_body14_e9278_d_n6, assign10080_body14_e9278_d_n7, assign10080_body14_e9278_d_n10, assign10080_body14_e9278_d_n11, assign10080_body14_e9278_d_n12, assign10080_body14_e9278_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard203 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk194, locals.var_t5__blk194_dn0, locals.var_t5__blk194_dn2, locals.var_t5__blk194_dn6, locals.var_t5__blk194_dn7, locals.var_t5__blk194_dn10, locals.var_t5__blk194_dn11, locals.var_t5__blk194_dn12, locals.var_t5__blk194_dn17,)
    }
};
            locals.var_t5__blk194 = assign10080_body14_e9278;
            locals.var_t5__blk194_dn0 = assign10080_body14_e9278_d_n0;
            locals.var_t5__blk194_dn2 = assign10080_body14_e9278_d_n2;
            locals.var_t5__blk194_dn6 = assign10080_body14_e9278_d_n6;
            locals.var_t5__blk194_dn7 = assign10080_body14_e9278_d_n7;
            locals.var_t5__blk194_dn10 = assign10080_body14_e9278_d_n10;
            locals.var_t5__blk194_dn11 = assign10080_body14_e9278_d_n11;
            locals.var_t5__blk194_dn12 = assign10080_body14_e9278_d_n12;
            locals.var_t5__blk194_dn17 = assign10080_body14_e9278_d_n17;
            locals.var_t5__blk194_rv = 0.0;
            let (assign10080_body15_e9293, assign10080_body15_e9293_d_n0, assign10080_body15_e9293_d_n2, assign10080_body15_e9293_d_n6, assign10080_body15_e9293_d_n7, assign10080_body15_e9293_d_n10, assign10080_body15_e9293_d_n11, assign10080_body15_e9293_d_n12, assign10080_body15_e9293_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard203 == 0.0)) {
        let assign10080_body15_e9291: f64 = (locals.var_t3__blk192).exp();
        (assign10080_body15_e9291, (assign10080_body15_e9291 * locals.var_t3__blk192_dn0), (assign10080_body15_e9291 * locals.var_t3__blk192_dn2), (assign10080_body15_e9291 * locals.var_t3__blk192_dn6), (assign10080_body15_e9291 * locals.var_t3__blk192_dn7), (assign10080_body15_e9291 * locals.var_t3__blk192_dn10), (assign10080_body15_e9291 * locals.var_t3__blk192_dn11), (assign10080_body15_e9291 * locals.var_t3__blk192_dn12), (assign10080_body15_e9291 * locals.var_t3__blk192_dn17),)
    } else {
        (locals.var_t5__blk194, locals.var_t5__blk194_dn0, locals.var_t5__blk194_dn2, locals.var_t5__blk194_dn6, locals.var_t5__blk194_dn7, locals.var_t5__blk194_dn10, locals.var_t5__blk194_dn11, locals.var_t5__blk194_dn12, locals.var_t5__blk194_dn17,)
    }
};
            locals.var_t5__blk194 = assign10080_body15_e9293;
            locals.var_t5__blk194_dn0 = assign10080_body15_e9293_d_n0;
            locals.var_t5__blk194_dn2 = assign10080_body15_e9293_d_n2;
            locals.var_t5__blk194_dn6 = assign10080_body15_e9293_d_n6;
            locals.var_t5__blk194_dn7 = assign10080_body15_e9293_d_n7;
            locals.var_t5__blk194_dn10 = assign10080_body15_e9293_d_n10;
            locals.var_t5__blk194_dn11 = assign10080_body15_e9293_d_n11;
            locals.var_t5__blk194_dn12 = assign10080_body15_e9293_d_n12;
            locals.var_t5__blk194_dn17 = assign10080_body15_e9293_d_n17;
            locals.var_t5__blk194_rv = 0.0;
            let (assign10080_body16_e9304, assign10080_body16_e9304_d_n0, assign10080_body16_e9304_d_n2, assign10080_body16_e9304_d_n6, assign10080_body16_e9304_d_n7, assign10080_body16_e9304_d_n10, assign10080_body16_e9304_d_n11, assign10080_body16_e9304_d_n12, assign10080_body16_e9304_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        (locals.var_evb1_qe_wl_p_egp12, locals.var_evb1_qe_wl_p_egp12_dn0, locals.var_evb1_qe_wl_p_egp12_dn2, locals.var_evb1_qe_wl_p_egp12_dn6, locals.var_evb1_qe_wl_p_egp12_dn7, locals.var_evb1_qe_wl_p_egp12_dn10, locals.var_evb1_qe_wl_p_egp12_dn11, locals.var_evb1_qe_wl_p_egp12_dn12, locals.var_evb1_qe_wl_p_egp12_dn17,)
    } else {
        (locals.var_t6__blk195, locals.var_t6__blk195_dn0, locals.var_t6__blk195_dn2, locals.var_t6__blk195_dn6, locals.var_t6__blk195_dn7, locals.var_t6__blk195_dn10, locals.var_t6__blk195_dn11, locals.var_t6__blk195_dn12, locals.var_t6__blk195_dn17,)
    }
};
            locals.var_t6__blk195 = assign10080_body16_e9304;
            locals.var_t6__blk195_dn0 = assign10080_body16_e9304_d_n0;
            locals.var_t6__blk195_dn2 = assign10080_body16_e9304_d_n2;
            locals.var_t6__blk195_dn6 = assign10080_body16_e9304_d_n6;
            locals.var_t6__blk195_dn7 = assign10080_body16_e9304_d_n7;
            locals.var_t6__blk195_dn10 = assign10080_body16_e9304_d_n10;
            locals.var_t6__blk195_dn11 = assign10080_body16_e9304_d_n11;
            locals.var_t6__blk195_dn12 = assign10080_body16_e9304_d_n12;
            locals.var_t6__blk195_dn17 = assign10080_body16_e9304_d_n17;
            locals.var_t6__blk195_rv = 0.0;
            let (assign10080_body17_e9323, assign10080_body17_e9323_d_n0, assign10080_body17_e9323_d_n2, assign10080_body17_e9323_d_n6, assign10080_body17_e9323_d_n7, assign10080_body17_e9323_d_n10, assign10080_body17_e9323_d_n11, assign10080_body17_e9323_d_n12, assign10080_body17_e9323_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body17_e9315: f64 = (0.25 * locals.var_t6__blk195);
        let assign10080_body17_e9317: f64 = (assign10080_body17_e9315 * locals.var_t1__blk190);
        let assign10080_body17_e9319: f64 = (assign10080_body17_e9317 * locals.var_t1__blk190);
        let assign10080_body17_e9321: f64 = (assign10080_body17_e9319 * 7.38905609893065);
        (assign10080_body17_e9321, ((((((0.25 * locals.var_t6__blk195_dn0) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn0)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn0)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk195_dn2) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn2)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn2)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk195_dn6) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn6)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn6)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk195_dn7) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn7)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn7)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk195_dn10) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn10)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn10)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk195_dn11) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn11)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn11)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk195_dn12) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn12)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn12)) * 7.38905609893065), ((((((0.25 * locals.var_t6__blk195_dn17) * locals.var_t1__blk190) + (assign10080_body17_e9315 * locals.var_t1__blk190_dn17)) * locals.var_t1__blk190) + (assign10080_body17_e9317 * locals.var_t1__blk190_dn17)) * 7.38905609893065),)
    } else {
        (locals.var_t7__blk196, locals.var_t7__blk196_dn0, locals.var_t7__blk196_dn2, locals.var_t7__blk196_dn6, locals.var_t7__blk196_dn7, locals.var_t7__blk196_dn10, locals.var_t7__blk196_dn11, locals.var_t7__blk196_dn12, locals.var_t7__blk196_dn17,)
    }
};
            locals.var_t7__blk196 = assign10080_body17_e9323;
            locals.var_t7__blk196_dn0 = assign10080_body17_e9323_d_n0;
            locals.var_t7__blk196_dn2 = assign10080_body17_e9323_d_n2;
            locals.var_t7__blk196_dn6 = assign10080_body17_e9323_d_n6;
            locals.var_t7__blk196_dn7 = assign10080_body17_e9323_d_n7;
            locals.var_t7__blk196_dn10 = assign10080_body17_e9323_d_n10;
            locals.var_t7__blk196_dn11 = assign10080_body17_e9323_d_n11;
            locals.var_t7__blk196_dn12 = assign10080_body17_e9323_d_n12;
            locals.var_t7__blk196_dn17 = assign10080_body17_e9323_d_n17;
            locals.var_t7__blk196_rv = 0.0;
            let assign10080_body18_e9326: f64 = (2.0 * locals.var_t2__blk191);
            let assign10080_body18_e9328: f64 = (assign10080_body18_e9326 + locals.var_t1__blk190);
            let assign10080_body18_e9330: f64 = if assign10080_body18_e9328 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard204 = assign10080_body18_e9330;
            locals.var_guard204_rv = 0.0;
            let (assign10080_body19_e9343, assign10080_body19_e9343_d_n0, assign10080_body19_e9343_d_n2, assign10080_body19_e9343_d_n6, assign10080_body19_e9343_d_n7, assign10080_body19_e9343_d_n10, assign10080_body19_e9343_d_n11, assign10080_body19_e9343_d_n12, assign10080_body19_e9343_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard204 != 0.0)) {
        (locals.var_t7__blk196, locals.var_t7__blk196_dn0, locals.var_t7__blk196_dn2, locals.var_t7__blk196_dn6, locals.var_t7__blk196_dn7, locals.var_t7__blk196_dn10, locals.var_t7__blk196_dn11, locals.var_t7__blk196_dn12, locals.var_t7__blk196_dn17,)
    } else {
        (locals.var_ievb0, locals.var_ievb0_dn0, locals.var_ievb0_dn2, locals.var_ievb0_dn6, locals.var_ievb0_dn7, locals.var_ievb0_dn10, locals.var_ievb0_dn11, locals.var_ievb0_dn12, locals.var_ievb0_dn17,)
    }
};
            locals.var_ievb0 = assign10080_body19_e9343;
            locals.var_ievb0_dn0 = assign10080_body19_e9343_d_n0;
            locals.var_ievb0_dn2 = assign10080_body19_e9343_d_n2;
            locals.var_ievb0_dn6 = assign10080_body19_e9343_d_n6;
            locals.var_ievb0_dn7 = assign10080_body19_e9343_d_n7;
            locals.var_ievb0_dn10 = assign10080_body19_e9343_d_n10;
            locals.var_ievb0_dn11 = assign10080_body19_e9343_d_n11;
            locals.var_ievb0_dn12 = assign10080_body19_e9343_d_n12;
            locals.var_ievb0_dn17 = assign10080_body19_e9343_d_n17;
            locals.var_ievb0_rv = 0.0;
            let (assign10080_body20_e9357,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard204 == 0.0)) {
        (locals.var_evb1_qe_wl,)
    } else {
        (locals.var_t4__blk193,)
    }
};
            locals.var_t4__blk193 = assign10080_body20_e9357;
            locals.var_t4__blk193_rv = 0.0;
            let (assign10080_body21_e9375, assign10080_body21_e9375_d_n0, assign10080_body21_e9375_d_n2, assign10080_body21_e9375_d_n6, assign10080_body21_e9375_d_n7, assign10080_body21_e9375_d_n10, assign10080_body21_e9375_d_n11, assign10080_body21_e9375_d_n12, assign10080_body21_e9375_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard204 == 0.0)) {
        let assign10080_body21_e9371: f64 = (locals.var_t4__blk193 * locals.var_t0__blk189);
        let assign10080_body21_e9373: f64 = (assign10080_body21_e9371 * locals.var_t5__blk194);
        (assign10080_body21_e9373, (((locals.var_t4__blk193 * locals.var_t0__blk189_dn0) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn0)), (((locals.var_t4__blk193 * locals.var_t0__blk189_dn2) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn2)), (((locals.var_t4__blk193 * locals.var_t0__blk189_dn6) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn6)), (((locals.var_t4__blk193 * locals.var_t0__blk189_dn7) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn7)), (((locals.var_t4__blk193 * locals.var_t0__blk189_dn10) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn10)), (((locals.var_t4__blk193 * locals.var_t0__blk189_dn11) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn11)), (((locals.var_t4__blk193 * locals.var_t0__blk189_dn12) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn12)), (((locals.var_t4__blk193 * locals.var_t0__blk189_dn17) * locals.var_t5__blk194) + (assign10080_body21_e9371 * locals.var_t5__blk194_dn17)),)
    } else {
        (locals.var_t8__blk197, locals.var_t8__blk197_dn0, locals.var_t8__blk197_dn2, locals.var_t8__blk197_dn6, locals.var_t8__blk197_dn7, locals.var_t8__blk197_dn10, locals.var_t8__blk197_dn11, locals.var_t8__blk197_dn12, locals.var_t8__blk197_dn17,)
    }
};
            locals.var_t8__blk197 = assign10080_body21_e9375;
            locals.var_t8__blk197_dn0 = assign10080_body21_e9375_d_n0;
            locals.var_t8__blk197_dn2 = assign10080_body21_e9375_d_n2;
            locals.var_t8__blk197_dn6 = assign10080_body21_e9375_d_n6;
            locals.var_t8__blk197_dn7 = assign10080_body21_e9375_d_n7;
            locals.var_t8__blk197_dn10 = assign10080_body21_e9375_d_n10;
            locals.var_t8__blk197_dn11 = assign10080_body21_e9375_d_n11;
            locals.var_t8__blk197_dn12 = assign10080_body21_e9375_d_n12;
            locals.var_t8__blk197_dn17 = assign10080_body21_e9375_d_n17;
            locals.var_t8__blk197_rv = 0.0;
            let assign10080_body22_e9382: f64 = if ((locals.var_t8__blk197 < locals.var_t7__blk196) || (locals.var_t2__blk191 < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard205 = assign10080_body22_e9382;
            locals.var_guard205_rv = 0.0;
            let (assign10080_body23_e9398, assign10080_body23_e9398_d_n0, assign10080_body23_e9398_d_n2, assign10080_body23_e9398_d_n6, assign10080_body23_e9398_d_n7, assign10080_body23_e9398_d_n10, assign10080_body23_e9398_d_n11, assign10080_body23_e9398_d_n12, assign10080_body23_e9398_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard204 == 0.0)) && (locals.var_guard205 != 0.0)) {
        (locals.var_t7__blk196, locals.var_t7__blk196_dn0, locals.var_t7__blk196_dn2, locals.var_t7__blk196_dn6, locals.var_t7__blk196_dn7, locals.var_t7__blk196_dn10, locals.var_t7__blk196_dn11, locals.var_t7__blk196_dn12, locals.var_t7__blk196_dn17,)
    } else {
        (locals.var_ievb0, locals.var_ievb0_dn0, locals.var_ievb0_dn2, locals.var_ievb0_dn6, locals.var_ievb0_dn7, locals.var_ievb0_dn10, locals.var_ievb0_dn11, locals.var_ievb0_dn12, locals.var_ievb0_dn17,)
    }
};
            locals.var_ievb0 = assign10080_body23_e9398;
            locals.var_ievb0_dn0 = assign10080_body23_e9398_d_n0;
            locals.var_ievb0_dn2 = assign10080_body23_e9398_d_n2;
            locals.var_ievb0_dn6 = assign10080_body23_e9398_d_n6;
            locals.var_ievb0_dn7 = assign10080_body23_e9398_d_n7;
            locals.var_ievb0_dn10 = assign10080_body23_e9398_d_n10;
            locals.var_ievb0_dn11 = assign10080_body23_e9398_d_n11;
            locals.var_ievb0_dn12 = assign10080_body23_e9398_d_n12;
            locals.var_ievb0_dn17 = assign10080_body23_e9398_d_n17;
            locals.var_ievb0_rv = 0.0;
            let (assign10080_body24_e9415, assign10080_body24_e9415_d_n0, assign10080_body24_e9415_d_n2, assign10080_body24_e9415_d_n6, assign10080_body24_e9415_d_n7, assign10080_body24_e9415_d_n10, assign10080_body24_e9415_d_n11, assign10080_body24_e9415_d_n12, assign10080_body24_e9415_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard204 == 0.0)) && (locals.var_guard205 == 0.0)) {
        (locals.var_t8__blk197, locals.var_t8__blk197_dn0, locals.var_t8__blk197_dn2, locals.var_t8__blk197_dn6, locals.var_t8__blk197_dn7, locals.var_t8__blk197_dn10, locals.var_t8__blk197_dn11, locals.var_t8__blk197_dn12, locals.var_t8__blk197_dn17,)
    } else {
        (locals.var_ievb0, locals.var_ievb0_dn0, locals.var_ievb0_dn2, locals.var_ievb0_dn6, locals.var_ievb0_dn7, locals.var_ievb0_dn10, locals.var_ievb0_dn11, locals.var_ievb0_dn12, locals.var_ievb0_dn17,)
    }
};
            locals.var_ievb0 = assign10080_body24_e9415;
            locals.var_ievb0_dn0 = assign10080_body24_e9415_d_n0;
            locals.var_ievb0_dn2 = assign10080_body24_e9415_d_n2;
            locals.var_ievb0_dn6 = assign10080_body24_e9415_d_n6;
            locals.var_ievb0_dn7 = assign10080_body24_e9415_d_n7;
            locals.var_ievb0_dn10 = assign10080_body24_e9415_d_n10;
            locals.var_ievb0_dn11 = assign10080_body24_e9415_d_n11;
            locals.var_ievb0_dn12 = assign10080_body24_e9415_d_n12;
            locals.var_ievb0_dn17 = assign10080_body24_e9415_d_n17;
            locals.var_ievb0_rv = 0.0;
            let assign10080_body26_e9431: f64 = if locals.var_ievb0 < 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard206 = assign10080_body26_e9431;
            locals.var_guard206_rv = 0.0;
            let (assign10080_body27_e9444,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard206 != 0.0)) {
        (100.0,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10080_body27_e9444;
            locals.var_i_rv = 0.0;
            let (assign10080_body28_e9457,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard206 != 0.0)) {
        (locals.var_lp_s0_max,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign10080_body28_e9457;
            locals.var_lp_s0_rv = 0.0;
            let (assign10080_body29_e9470,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard201 == 0.0)) {
        let assign10080_body29_e9468: f64 = (locals.var_i + 1.0);
        (assign10080_body29_e9468,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10080_body29_e9470;
            locals.var_i_rv = 0.0;
        }

        let assign10090_e9477: f64 = if ((p.p117 <= 0.0) || (locals.var_mks_vmax <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard219 = assign10090_e9477;
        locals.var_guard219_rv = 0.0;

        let (assign10100_e9487, assign10100_e9487_d_n0, assign10100_e9487_d_n2, assign10100_e9487_d_n6, assign10100_e9487_d_n7, assign10100_e9487_d_n10, assign10100_e9487_d_n11, assign10100_e9487_d_n12, assign10100_e9487_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign10100_e9487;
        locals.var_isub_dn0 = assign10100_e9487_d_n0;
        locals.var_isub_dn2 = assign10100_e9487_d_n2;
        locals.var_isub_dn6 = assign10100_e9487_d_n6;
        locals.var_isub_dn7 = assign10100_e9487_d_n7;
        locals.var_isub_dn10 = assign10100_e9487_d_n10;
        locals.var_isub_dn11 = assign10100_e9487_d_n11;
        locals.var_isub_dn12 = assign10100_e9487_d_n12;
        locals.var_isub_dn17 = assign10100_e9487_d_n17;
        locals.var_isub_rv = 0.0;

        let assign10110_e9490: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard220 = assign10110_e9490;
        locals.var_guard220_rv = 0.0;

        let (assign10120_e9503, assign10120_e9503_d_n0, assign10120_e9503_d_n2, assign10120_e9503_d_n6, assign10120_e9503_d_n7, assign10120_e9503_d_n10, assign10120_e9503_d_n11, assign10120_e9503_d_n12, assign10120_e9503_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_t1__blk207, locals.var_t1__blk207_dn0, locals.var_t1__blk207_dn2, locals.var_t1__blk207_dn6, locals.var_t1__blk207_dn7, locals.var_t1__blk207_dn10, locals.var_t1__blk207_dn11, locals.var_t1__blk207_dn12, locals.var_t1__blk207_dn17,)
    }
};
        locals.var_t1__blk207 = assign10120_e9503;
        locals.var_t1__blk207_dn0 = assign10120_e9503_d_n0;
        locals.var_t1__blk207_dn2 = assign10120_e9503_d_n2;
        locals.var_t1__blk207_dn6 = assign10120_e9503_d_n6;
        locals.var_t1__blk207_dn7 = assign10120_e9503_d_n7;
        locals.var_t1__blk207_dn10 = assign10120_e9503_d_n10;
        locals.var_t1__blk207_dn11 = assign10120_e9503_d_n11;
        locals.var_t1__blk207_dn12 = assign10120_e9503_d_n12;
        locals.var_t1__blk207_dn17 = assign10120_e9503_d_n17;
        locals.var_t1__blk207_rv = 0.0;

        let (assign10130_e9518, assign10130_e9518_d_n0, assign10130_e9518_d_n2, assign10130_e9518_d_n6, assign10130_e9518_d_n7, assign10130_e9518_d_n10, assign10130_e9518_d_n11, assign10130_e9518_d_n12, assign10130_e9518_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10130_e9516: f64 = (locals.var_c_fox * locals.var_c_fox);
        (assign10130_e9516, ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)), ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)), ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)), ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)), ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)), ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)), ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)), ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_t7__blk214, locals.var_t7__blk214_dn0, locals.var_t7__blk214_dn2, locals.var_t7__blk214_dn6, locals.var_t7__blk214_dn7, locals.var_t7__blk214_dn10, locals.var_t7__blk214_dn11, locals.var_t7__blk214_dn12, locals.var_t7__blk214_dn17,)
    }
};
        locals.var_t7__blk214 = assign10130_e9518;
        locals.var_t7__blk214_dn0 = assign10130_e9518_d_n0;
        locals.var_t7__blk214_dn2 = assign10130_e9518_d_n2;
        locals.var_t7__blk214_dn6 = assign10130_e9518_d_n6;
        locals.var_t7__blk214_dn7 = assign10130_e9518_d_n7;
        locals.var_t7__blk214_dn10 = assign10130_e9518_d_n10;
        locals.var_t7__blk214_dn11 = assign10130_e9518_d_n11;
        locals.var_t7__blk214_dn12 = assign10130_e9518_d_n12;
        locals.var_t7__blk214_dn17 = assign10130_e9518_d_n17;
        locals.var_t7__blk214_rv = 0.0;

        let (assign10140_e9531, assign10140_e9531_d_n0, assign10140_e9531_d_n2, assign10140_e9531_d_n6, assign10140_e9531_d_n7, assign10140_e9531_d_n10, assign10140_e9531_d_n11, assign10140_e9531_d_n12, assign10140_e9531_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        (locals.var_qnsub_esi, locals.var_qnsub_esi_dn0, locals.var_qnsub_esi_dn2, locals.var_qnsub_esi_dn6, locals.var_qnsub_esi_dn7, locals.var_qnsub_esi_dn10, locals.var_qnsub_esi_dn11, locals.var_qnsub_esi_dn12, locals.var_qnsub_esi_dn17,)
    } else {
        (locals.var_t8__blk215, locals.var_t8__blk215_dn0, locals.var_t8__blk215_dn2, locals.var_t8__blk215_dn6, locals.var_t8__blk215_dn7, locals.var_t8__blk215_dn10, locals.var_t8__blk215_dn11, locals.var_t8__blk215_dn12, locals.var_t8__blk215_dn17,)
    }
};
        locals.var_t8__blk215 = assign10140_e9531;
        locals.var_t8__blk215_dn0 = assign10140_e9531_d_n0;
        locals.var_t8__blk215_dn2 = assign10140_e9531_d_n2;
        locals.var_t8__blk215_dn6 = assign10140_e9531_d_n6;
        locals.var_t8__blk215_dn7 = assign10140_e9531_d_n7;
        locals.var_t8__blk215_dn10 = assign10140_e9531_d_n10;
        locals.var_t8__blk215_dn11 = assign10140_e9531_d_n11;
        locals.var_t8__blk215_dn12 = assign10140_e9531_d_n12;
        locals.var_t8__blk215_dn17 = assign10140_e9531_d_n17;
        locals.var_t8__blk215_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10150_e9546, assign10150_e9546_d_n0, assign10150_e9546_d_n2, assign10150_e9546_d_n6, assign10150_e9546_d_n7, assign10150_e9546_d_n10, assign10150_e9546_d_n11, assign10150_e9546_d_n12, assign10150_e9546_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10150_e9544: f64 = (locals.var_t8__blk215 / locals.var_t7__blk214);
        (assign10150_e9544, (((locals.var_t8__blk215_dn0 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn0)) / (locals.var_t7__blk214 * locals.var_t7__blk214)), (((locals.var_t8__blk215_dn2 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn2)) / (locals.var_t7__blk214 * locals.var_t7__blk214)), (((locals.var_t8__blk215_dn6 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn6)) / (locals.var_t7__blk214 * locals.var_t7__blk214)), (((locals.var_t8__blk215_dn7 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn7)) / (locals.var_t7__blk214 * locals.var_t7__blk214)), (((locals.var_t8__blk215_dn10 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn10)) / (locals.var_t7__blk214 * locals.var_t7__blk214)), (((locals.var_t8__blk215_dn11 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn11)) / (locals.var_t7__blk214 * locals.var_t7__blk214)), (((locals.var_t8__blk215_dn12 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn12)) / (locals.var_t7__blk214 * locals.var_t7__blk214)), (((locals.var_t8__blk215_dn17 * locals.var_t7__blk214) - (locals.var_t8__blk215 * locals.var_t7__blk214_dn17)) / (locals.var_t7__blk214 * locals.var_t7__blk214)),)
    } else {
        (locals.var_t3__blk209, locals.var_t3__blk209_dn0, locals.var_t3__blk209_dn2, locals.var_t3__blk209_dn6, locals.var_t3__blk209_dn7, locals.var_t3__blk209_dn10, locals.var_t3__blk209_dn11, locals.var_t3__blk209_dn12, locals.var_t3__blk209_dn17,)
    }
};
        locals.var_t3__blk209 = assign10150_e9546;
        locals.var_t3__blk209_dn0 = assign10150_e9546_d_n0;
        locals.var_t3__blk209_dn2 = assign10150_e9546_d_n2;
        locals.var_t3__blk209_dn6 = assign10150_e9546_d_n6;
        locals.var_t3__blk209_dn7 = assign10150_e9546_d_n7;
        locals.var_t3__blk209_dn10 = assign10150_e9546_d_n10;
        locals.var_t3__blk209_dn11 = assign10150_e9546_d_n11;
        locals.var_t3__blk209_dn12 = assign10150_e9546_d_n12;
        locals.var_t3__blk209_dn17 = assign10150_e9546_d_n17;
        locals.var_t3__blk209_rv = 0.0;

        let (assign10160_e9561, assign10160_e9561_d_n0, assign10160_e9561_d_n2, assign10160_e9561_d_n6, assign10160_e9561_d_n7, assign10160_e9561_d_n10, assign10160_e9561_d_n11, assign10160_e9561_d_n12, assign10160_e9561_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10160_e9559: f64 = (2.0 / locals.var_t8__blk215);
        (assign10160_e9559, (-((2.0 * locals.var_t8__blk215_dn0) / (locals.var_t8__blk215 * locals.var_t8__blk215))), (-((2.0 * locals.var_t8__blk215_dn2) / (locals.var_t8__blk215 * locals.var_t8__blk215))), (-((2.0 * locals.var_t8__blk215_dn6) / (locals.var_t8__blk215 * locals.var_t8__blk215))), (-((2.0 * locals.var_t8__blk215_dn7) / (locals.var_t8__blk215 * locals.var_t8__blk215))), (-((2.0 * locals.var_t8__blk215_dn10) / (locals.var_t8__blk215 * locals.var_t8__blk215))), (-((2.0 * locals.var_t8__blk215_dn11) / (locals.var_t8__blk215 * locals.var_t8__blk215))), (-((2.0 * locals.var_t8__blk215_dn12) / (locals.var_t8__blk215 * locals.var_t8__blk215))), (-((2.0 * locals.var_t8__blk215_dn17) / (locals.var_t8__blk215 * locals.var_t8__blk215))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12, locals.var_t9_dn17,)
    }
};
        locals.var_t9 = assign10160_e9561;
        locals.var_t9_dn0 = assign10160_e9561_d_n0;
        locals.var_t9_dn2 = assign10160_e9561_d_n2;
        locals.var_t9_dn6 = assign10160_e9561_d_n6;
        locals.var_t9_dn7 = assign10160_e9561_d_n7;
        locals.var_t9_dn10 = assign10160_e9561_d_n10;
        locals.var_t9_dn11 = assign10160_e9561_d_n11;
        locals.var_t9_dn12 = assign10160_e9561_d_n12;
        locals.var_t9_dn17 = assign10160_e9561_d_n17;
        locals.var_t9_rv = 0.0;

        let (assign10170_e9576, assign10170_e9576_d_n0, assign10170_e9576_d_n2, assign10170_e9576_d_n6, assign10170_e9576_d_n7, assign10170_e9576_d_n10, assign10170_e9576_d_n11, assign10170_e9576_d_n12, assign10170_e9576_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10170_e9574: f64 = (locals.var_t9 * locals.var_t7__blk214);
        (assign10170_e9574, ((locals.var_t9_dn0 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn0)), ((locals.var_t9_dn2 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn2)), ((locals.var_t9_dn6 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn6)), ((locals.var_t9_dn7 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn7)), ((locals.var_t9_dn10 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn10)), ((locals.var_t9_dn11 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn11)), ((locals.var_t9_dn12 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn12)), ((locals.var_t9_dn17 * locals.var_t7__blk214) + (locals.var_t9 * locals.var_t7__blk214_dn17)),)
    } else {
        (locals.var_t4__blk210, locals.var_t4__blk210_dn0, locals.var_t4__blk210_dn2, locals.var_t4__blk210_dn6, locals.var_t4__blk210_dn7, locals.var_t4__blk210_dn10, locals.var_t4__blk210_dn11, locals.var_t4__blk210_dn12, locals.var_t4__blk210_dn17,)
    }
};
        locals.var_t4__blk210 = assign10170_e9576;
        locals.var_t4__blk210_dn0 = assign10170_e9576_d_n0;
        locals.var_t4__blk210_dn2 = assign10170_e9576_d_n2;
        locals.var_t4__blk210_dn6 = assign10170_e9576_d_n6;
        locals.var_t4__blk210_dn7 = assign10170_e9576_d_n7;
        locals.var_t4__blk210_dn10 = assign10170_e9576_d_n10;
        locals.var_t4__blk210_dn11 = assign10170_e9576_d_n11;
        locals.var_t4__blk210_dn12 = assign10170_e9576_d_n12;
        locals.var_t4__blk210_dn17 = assign10170_e9576_d_n17;
        locals.var_t4__blk210_rv = 0.0;

        let (assign10180_e9595, assign10180_e9595_d_n0, assign10180_e9595_d_n2, assign10180_e9595_d_n6, assign10180_e9595_d_n7, assign10180_e9595_d_n10, assign10180_e9595_d_n11, assign10180_e9595_d_n12, assign10180_e9595_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10180_e9589: f64 = (locals.var_t1__blk207 - locals.var_beta_inv);
        let assign10180_e9592: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign10180_e9593: f64 = (assign10180_e9589 - assign10180_e9592);
        (assign10180_e9593, (locals.var_t1__blk207_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk207_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk207_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk207_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk207_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk207_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk207_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk207_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk211, locals.var_t5__blk211_dn0, locals.var_t5__blk211_dn2, locals.var_t5__blk211_dn6, locals.var_t5__blk211_dn7, locals.var_t5__blk211_dn10, locals.var_t5__blk211_dn11, locals.var_t5__blk211_dn12, locals.var_t5__blk211_dn17,)
    }
};
        locals.var_t5__blk211 = assign10180_e9595;
        locals.var_t5__blk211_dn0 = assign10180_e9595_d_n0;
        locals.var_t5__blk211_dn2 = assign10180_e9595_d_n2;
        locals.var_t5__blk211_dn6 = assign10180_e9595_d_n6;
        locals.var_t5__blk211_dn7 = assign10180_e9595_d_n7;
        locals.var_t5__blk211_dn10 = assign10180_e9595_d_n10;
        locals.var_t5__blk211_dn11 = assign10180_e9595_d_n11;
        locals.var_t5__blk211_dn12 = assign10180_e9595_d_n12;
        locals.var_t5__blk211_dn17 = assign10180_e9595_d_n17;
        locals.var_t5__blk211_rv = 0.0;

        let (assign10190_e9612, assign10190_e9612_d_n0, assign10190_e9612_d_n2, assign10190_e9612_d_n6, assign10190_e9612_d_n7, assign10190_e9612_d_n10, assign10190_e9612_d_n11, assign10190_e9612_d_n12, assign10190_e9612_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10190_e9609: f64 = (locals.var_t4__blk210 * locals.var_t5__blk211);
        let assign10190_e9610: f64 = (1.0 + assign10190_e9609);
        (assign10190_e9610, ((locals.var_t4__blk210_dn0 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn0)), ((locals.var_t4__blk210_dn2 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn2)), ((locals.var_t4__blk210_dn6 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn6)), ((locals.var_t4__blk210_dn7 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn7)), ((locals.var_t4__blk210_dn10 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn10)), ((locals.var_t4__blk210_dn11 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn11)), ((locals.var_t4__blk210_dn12 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn12)), ((locals.var_t4__blk210_dn17 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn17)),)
    } else {
        (locals.var_t6w, locals.var_t6w_dn0, locals.var_t6w_dn2, locals.var_t6w_dn6, locals.var_t6w_dn7, locals.var_t6w_dn10, locals.var_t6w_dn11, locals.var_t6w_dn12, locals.var_t6w_dn17,)
    }
};
        locals.var_t6w = assign10190_e9612;
        locals.var_t6w_dn0 = assign10190_e9612_d_n0;
        locals.var_t6w_dn2 = assign10190_e9612_d_n2;
        locals.var_t6w_dn6 = assign10190_e9612_d_n6;
        locals.var_t6w_dn7 = assign10190_e9612_d_n7;
        locals.var_t6w_dn10 = assign10190_e9612_d_n10;
        locals.var_t6w_dn11 = assign10190_e9612_d_n11;
        locals.var_t6w_dn12 = assign10190_e9612_d_n12;
        locals.var_t6w_dn17 = assign10190_e9612_d_n17;
        locals.var_t6w_rv = 0.0;

        let (assign10200_e9634, assign10200_e9634_d_n0, assign10200_e9634_d_n2, assign10200_e9634_d_n6, assign10200_e9634_d_n7, assign10200_e9634_d_n10, assign10200_e9634_d_n11, assign10200_e9634_d_n12, assign10200_e9634_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10200_e9625: f64 = (locals.var_t6w * locals.var_t6w);
        let assign10200_e9628: f64 = (4.0 * 0.001);
        let assign10200_e9630: f64 = (assign10200_e9628 * 0.001);
        let assign10200_e9631: f64 = (assign10200_e9625 + assign10200_e9630);
        let assign10200_e9632: f64 = (assign10200_e9631).sqrt();
        (assign10200_e9632, (((locals.var_t6w_dn0 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn0)) / (2.0 * assign10200_e9632)), (((locals.var_t6w_dn2 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn2)) / (2.0 * assign10200_e9632)), (((locals.var_t6w_dn6 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn6)) / (2.0 * assign10200_e9632)), (((locals.var_t6w_dn7 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn7)) / (2.0 * assign10200_e9632)), (((locals.var_t6w_dn10 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn10)) / (2.0 * assign10200_e9632)), (((locals.var_t6w_dn11 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn11)) / (2.0 * assign10200_e9632)), (((locals.var_t6w_dn12 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn12)) / (2.0 * assign10200_e9632)), (((locals.var_t6w_dn17 * locals.var_t6w) + (locals.var_t6w * locals.var_t6w_dn17)) / (2.0 * assign10200_e9632)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign10200_e9634;
        locals.var_tmf1_dn0 = assign10200_e9634_d_n0;
        locals.var_tmf1_dn2 = assign10200_e9634_d_n2;
        locals.var_tmf1_dn6 = assign10200_e9634_d_n6;
        locals.var_tmf1_dn7 = assign10200_e9634_d_n7;
        locals.var_tmf1_dn10 = assign10200_e9634_d_n10;
        locals.var_tmf1_dn11 = assign10200_e9634_d_n11;
        locals.var_tmf1_dn12 = assign10200_e9634_d_n12;
        locals.var_tmf1_dn17 = assign10200_e9634_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign10210_e9655, assign10210_e9655_d_n0, assign10210_e9655_d_n2, assign10210_e9655_d_n6, assign10210_e9655_d_n7, assign10210_e9655_d_n10, assign10210_e9655_d_n11, assign10210_e9655_d_n12, assign10210_e9655_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10210_e9648: f64 = (locals.var_t6w + locals.var_tmf1);
        let assign10210_e9649: f64 = (0.5 * assign10210_e9648);
        let assign10210_e9652: f64 = (1e-10 * 0.001);
        let assign10210_e9653: f64 = (assign10210_e9649 + assign10210_e9652);
        (assign10210_e9653, (0.5 * (locals.var_t6w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk212, locals.var_t6__blk212_dn0, locals.var_t6__blk212_dn2, locals.var_t6__blk212_dn6, locals.var_t6__blk212_dn7, locals.var_t6__blk212_dn10, locals.var_t6__blk212_dn11, locals.var_t6__blk212_dn12, locals.var_t6__blk212_dn17,)
    }
};
        locals.var_t6__blk212 = assign10210_e9655;
        locals.var_t6__blk212_dn0 = assign10210_e9655_d_n0;
        locals.var_t6__blk212_dn2 = assign10210_e9655_d_n2;
        locals.var_t6__blk212_dn6 = assign10210_e9655_d_n6;
        locals.var_t6__blk212_dn7 = assign10210_e9655_d_n7;
        locals.var_t6__blk212_dn10 = assign10210_e9655_d_n10;
        locals.var_t6__blk212_dn11 = assign10210_e9655_d_n11;
        locals.var_t6__blk212_dn12 = assign10210_e9655_d_n12;
        locals.var_t6__blk212_dn17 = assign10210_e9655_d_n17;
        locals.var_t6__blk212_rv = 0.0;

        let assign10220_e9658: f64 = if locals.var_t6__blk212 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard221 = assign10220_e9658;
        locals.var_guard221_rv = 0.0;

        let (assign10230_e9673, assign10230_e9673_d_n0, assign10230_e9673_d_n2, assign10230_e9673_d_n6, assign10230_e9673_d_n7, assign10230_e9673_d_n10, assign10230_e9673_d_n11, assign10230_e9673_d_n12, assign10230_e9673_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard221 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk212, locals.var_t6__blk212_dn0, locals.var_t6__blk212_dn2, locals.var_t6__blk212_dn6, locals.var_t6__blk212_dn7, locals.var_t6__blk212_dn10, locals.var_t6__blk212_dn11, locals.var_t6__blk212_dn12, locals.var_t6__blk212_dn17,)
    }
};
        locals.var_t6__blk212 = assign10230_e9673;
        locals.var_t6__blk212_dn0 = assign10230_e9673_d_n0;
        locals.var_t6__blk212_dn2 = assign10230_e9673_d_n2;
        locals.var_t6__blk212_dn6 = assign10230_e9673_d_n6;
        locals.var_t6__blk212_dn7 = assign10230_e9673_d_n7;
        locals.var_t6__blk212_dn10 = assign10230_e9673_d_n10;
        locals.var_t6__blk212_dn11 = assign10230_e9673_d_n11;
        locals.var_t6__blk212_dn12 = assign10230_e9673_d_n12;
        locals.var_t6__blk212_dn17 = assign10230_e9673_d_n17;
        locals.var_t6__blk212_rv = 0.0;

        let (assign10240_e9688, assign10240_e9688_d_n0, assign10240_e9688_d_n2, assign10240_e9688_d_n6, assign10240_e9688_d_n7, assign10240_e9688_d_n10, assign10240_e9688_d_n11, assign10240_e9688_d_n12, assign10240_e9688_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10240_e9686: f64 = (locals.var_t6__blk212 + 1e-50);
        (assign10240_e9686, locals.var_t6__blk212_dn0, locals.var_t6__blk212_dn2, locals.var_t6__blk212_dn6, locals.var_t6__blk212_dn7, locals.var_t6__blk212_dn10, locals.var_t6__blk212_dn11, locals.var_t6__blk212_dn12, locals.var_t6__blk212_dn17,)
    } else {
        (locals.var_t6__blk212, locals.var_t6__blk212_dn0, locals.var_t6__blk212_dn2, locals.var_t6__blk212_dn6, locals.var_t6__blk212_dn7, locals.var_t6__blk212_dn10, locals.var_t6__blk212_dn11, locals.var_t6__blk212_dn12, locals.var_t6__blk212_dn17,)
    }
};
        locals.var_t6__blk212 = assign10240_e9688;
        locals.var_t6__blk212_dn0 = assign10240_e9688_d_n0;
        locals.var_t6__blk212_dn2 = assign10240_e9688_d_n2;
        locals.var_t6__blk212_dn6 = assign10240_e9688_d_n6;
        locals.var_t6__blk212_dn7 = assign10240_e9688_d_n7;
        locals.var_t6__blk212_dn10 = assign10240_e9688_d_n10;
        locals.var_t6__blk212_dn11 = assign10240_e9688_d_n11;
        locals.var_t6__blk212_dn12 = assign10240_e9688_d_n12;
        locals.var_t6__blk212_dn17 = assign10240_e9688_d_n17;
        locals.var_t6__blk212_rv = 0.0;

        let (assign10250_e9702, assign10250_e9702_d_n0, assign10250_e9702_d_n2, assign10250_e9702_d_n6, assign10250_e9702_d_n7, assign10250_e9702_d_n10, assign10250_e9702_d_n11, assign10250_e9702_d_n12, assign10250_e9702_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10250_e9700: f64 = (locals.var_t6__blk212).sqrt();
        (assign10250_e9700, (locals.var_t6__blk212_dn0 / (2.0 * assign10250_e9700)), (locals.var_t6__blk212_dn2 / (2.0 * assign10250_e9700)), (locals.var_t6__blk212_dn6 / (2.0 * assign10250_e9700)), (locals.var_t6__blk212_dn7 / (2.0 * assign10250_e9700)), (locals.var_t6__blk212_dn10 / (2.0 * assign10250_e9700)), (locals.var_t6__blk212_dn11 / (2.0 * assign10250_e9700)), (locals.var_t6__blk212_dn12 / (2.0 * assign10250_e9700)), (locals.var_t6__blk212_dn17 / (2.0 * assign10250_e9700)),)
    } else {
        (locals.var_t6__blk212, locals.var_t6__blk212_dn0, locals.var_t6__blk212_dn2, locals.var_t6__blk212_dn6, locals.var_t6__blk212_dn7, locals.var_t6__blk212_dn10, locals.var_t6__blk212_dn11, locals.var_t6__blk212_dn12, locals.var_t6__blk212_dn17,)
    }
};
        locals.var_t6__blk212 = assign10250_e9702;
        locals.var_t6__blk212_dn0 = assign10250_e9702_d_n0;
        locals.var_t6__blk212_dn2 = assign10250_e9702_d_n2;
        locals.var_t6__blk212_dn6 = assign10250_e9702_d_n6;
        locals.var_t6__blk212_dn7 = assign10250_e9702_d_n7;
        locals.var_t6__blk212_dn10 = assign10250_e9702_d_n10;
        locals.var_t6__blk212_dn11 = assign10250_e9702_d_n11;
        locals.var_t6__blk212_dn12 = assign10250_e9702_d_n12;
        locals.var_t6__blk212_dn17 = assign10250_e9702_d_n17;
        locals.var_t6__blk212_rv = 0.0;

        let (assign10260_e9723, assign10260_e9723_d_n0, assign10260_e9723_d_n2, assign10260_e9723_d_n6, assign10260_e9723_d_n7, assign10260_e9723_d_n10, assign10260_e9723_d_n11, assign10260_e9723_d_n12, assign10260_e9723_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10260_e9715: f64 = (locals.var_t1__blk207 * locals.var_uc_svgs);
        let assign10260_e9719: f64 = (1.0 - locals.var_t6__blk212);
        let assign10260_e9720: f64 = (locals.var_t3__blk209 * assign10260_e9719);
        let assign10260_e9721: f64 = (assign10260_e9715 + assign10260_e9720);
        (assign10260_e9721, ((locals.var_t1__blk207_dn0 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn0 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn0)))), ((locals.var_t1__blk207_dn2 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn2 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn2)))), ((locals.var_t1__blk207_dn6 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn6 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn6)))), ((locals.var_t1__blk207_dn7 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn7 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn7)))), ((locals.var_t1__blk207_dn10 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn10 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn10)))), ((locals.var_t1__blk207_dn11 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn11 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn11)))), ((locals.var_t1__blk207_dn12 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn12 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn12)))), ((locals.var_t1__blk207_dn17 * locals.var_uc_svgs) + ((locals.var_t3__blk209_dn17 * assign10260_e9719) + (locals.var_t3__blk209 * (-locals.var_t6__blk212_dn17)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn12, locals.var_psislsat_dn17,)
    }
};
        locals.var_psislsat = assign10260_e9723;
        locals.var_psislsat_dn0 = assign10260_e9723_d_n0;
        locals.var_psislsat_dn2 = assign10260_e9723_d_n2;
        locals.var_psislsat_dn6 = assign10260_e9723_d_n6;
        locals.var_psislsat_dn7 = assign10260_e9723_d_n7;
        locals.var_psislsat_dn10 = assign10260_e9723_d_n10;
        locals.var_psislsat_dn11 = assign10260_e9723_d_n11;
        locals.var_psislsat_dn12 = assign10260_e9723_d_n12;
        locals.var_psislsat_dn17 = assign10260_e9723_d_n17;
        locals.var_psislsat_rv = 0.0;

        let (assign10270_e9746, assign10270_e9746_d_n0, assign10270_e9746_d_n2, assign10270_e9746_d_n6, assign10270_e9746_d_n7, assign10270_e9746_d_n10, assign10270_e9746_d_n11, assign10270_e9746_d_n12, assign10270_e9746_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10270_e9736: f64 = (p.p122 * locals.var_vdsz);
        let assign10270_e9738: f64 = (assign10270_e9736 + locals.var_ps0_isub);
        let assign10270_e9741: f64 = (locals.var_xgate * locals.var_zvgs);
        let assign10270_e9743: f64 = (assign10270_e9741 * locals.var_psislsat);
        let assign10270_e9744: f64 = (assign10270_e9738 - assign10270_e9743);
        (assign10270_e9744, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0_isub_dn0) - (assign10270_e9741 * locals.var_psislsat_dn0)), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0_isub_dn2) - (assign10270_e9741 * locals.var_psislsat_dn2)), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0_isub_dn6) - (assign10270_e9741 * locals.var_psislsat_dn6)), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0_isub_dn7) - (assign10270_e9741 * locals.var_psislsat_dn7)), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0_isub_dn10) - (assign10270_e9741 * locals.var_psislsat_dn10)), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0_isub_dn11) - (assign10270_e9741 * locals.var_psislsat_dn11)), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0_isub_dn12) - (assign10270_e9741 * locals.var_psislsat_dn12)), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0_isub_dn17) - (assign10270_e9741 * locals.var_psislsat_dn17)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12, locals.var_psisubsat_dn17,)
    }
};
        locals.var_psisubsat = assign10270_e9746;
        locals.var_psisubsat_dn0 = assign10270_e9746_d_n0;
        locals.var_psisubsat_dn2 = assign10270_e9746_d_n2;
        locals.var_psisubsat_dn6 = assign10270_e9746_d_n6;
        locals.var_psisubsat_dn7 = assign10270_e9746_d_n7;
        locals.var_psisubsat_dn10 = assign10270_e9746_d_n10;
        locals.var_psisubsat_dn11 = assign10270_e9746_d_n11;
        locals.var_psisubsat_dn12 = assign10270_e9746_d_n12;
        locals.var_psisubsat_dn17 = assign10270_e9746_d_n17;
        locals.var_psisubsat_rv = 0.0;

        let (assign10280_e9768, assign10280_e9768_d_n0, assign10280_e9768_d_n2, assign10280_e9768_d_n6, assign10280_e9768_d_n7, assign10280_e9768_d_n10, assign10280_e9768_d_n11, assign10280_e9768_d_n12, assign10280_e9768_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10280_e9759: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign10280_e9762: f64 = (4.0 * 0.01);
        let assign10280_e9764: f64 = (assign10280_e9762 * 0.01);
        let assign10280_e9765: f64 = (assign10280_e9759 + assign10280_e9764);
        let assign10280_e9766: f64 = (assign10280_e9765).sqrt();
        (assign10280_e9766, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign10280_e9766)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign10280_e9766)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign10280_e9766)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign10280_e9766)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign10280_e9766)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign10280_e9766)), (((locals.var_psisubsat_dn12 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn12)) / (2.0 * assign10280_e9766)), (((locals.var_psisubsat_dn17 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn17)) / (2.0 * assign10280_e9766)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign10280_e9768;
        locals.var_tmf1_dn0 = assign10280_e9768_d_n0;
        locals.var_tmf1_dn2 = assign10280_e9768_d_n2;
        locals.var_tmf1_dn6 = assign10280_e9768_d_n6;
        locals.var_tmf1_dn7 = assign10280_e9768_d_n7;
        locals.var_tmf1_dn10 = assign10280_e9768_d_n10;
        locals.var_tmf1_dn11 = assign10280_e9768_d_n11;
        locals.var_tmf1_dn12 = assign10280_e9768_d_n12;
        locals.var_tmf1_dn17 = assign10280_e9768_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign10290_e9789, assign10290_e9789_d_n0, assign10290_e9789_d_n2, assign10290_e9789_d_n6, assign10290_e9789_d_n7, assign10290_e9789_d_n10, assign10290_e9789_d_n11, assign10290_e9789_d_n12, assign10290_e9789_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) {
        let assign10290_e9782: f64 = (locals.var_psisubsat + locals.var_tmf1);
        let assign10290_e9783: f64 = (0.5 * assign10290_e9782);
        let assign10290_e9786: f64 = (1e-10 * 0.01);
        let assign10290_e9787: f64 = (assign10290_e9783 + assign10290_e9786);
        (assign10290_e9787, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_psisubsat_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_psisubsat_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12, locals.var_psisubsat_dn17,)
    }
};
        locals.var_psisubsat = assign10290_e9789;
        locals.var_psisubsat_dn0 = assign10290_e9789_d_n0;
        locals.var_psisubsat_dn2 = assign10290_e9789_d_n2;
        locals.var_psisubsat_dn6 = assign10290_e9789_d_n6;
        locals.var_psisubsat_dn7 = assign10290_e9789_d_n7;
        locals.var_psisubsat_dn10 = assign10290_e9789_d_n10;
        locals.var_psisubsat_dn11 = assign10290_e9789_d_n11;
        locals.var_psisubsat_dn12 = assign10290_e9789_d_n12;
        locals.var_psisubsat_dn17 = assign10290_e9789_d_n17;
        locals.var_psisubsat_rv = 0.0;

        let assign10300_e9792: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign10300_e9792;
        locals.var_guard222_rv = 0.0;

        let (assign10310_e9807, assign10310_e9807_d_n0, assign10310_e9807_d_n2, assign10310_e9807_d_n6, assign10310_e9807_d_n7, assign10310_e9807_d_n10, assign10310_e9807_d_n11, assign10310_e9807_d_n12, assign10310_e9807_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12, locals.var_psisubsat_dn17,)
    }
};
        locals.var_psisubsat = assign10310_e9807;
        locals.var_psisubsat_dn0 = assign10310_e9807_d_n0;
        locals.var_psisubsat_dn2 = assign10310_e9807_d_n2;
        locals.var_psisubsat_dn6 = assign10310_e9807_d_n6;
        locals.var_psisubsat_dn7 = assign10310_e9807_d_n7;
        locals.var_psisubsat_dn10 = assign10310_e9807_d_n10;
        locals.var_psisubsat_dn11 = assign10310_e9807_d_n11;
        locals.var_psisubsat_dn12 = assign10310_e9807_d_n12;
        locals.var_psisubsat_dn17 = assign10310_e9807_d_n17;
        locals.var_psisubsat_rv = 0.0;

        let (assign10320_e9823, assign10320_e9823_d_n0, assign10320_e9823_d_n2, assign10320_e9823_d_n6, assign10320_e9823_d_n7, assign10320_e9823_d_n10, assign10320_e9823_d_n11, assign10320_e9823_d_n12, assign10320_e9823_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign10320_e9821: f64 = (locals.var_vg2const * locals.var_vgpsub);
        (assign10320_e9821, ((locals.var_vg2const_dn0 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn0)), ((locals.var_vg2const_dn2 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn2)), ((locals.var_vg2const_dn6 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn6)), ((locals.var_vg2const_dn7 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn7)), ((locals.var_vg2const_dn10 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn10)), ((locals.var_vg2const_dn11 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn11)), ((locals.var_vg2const_dn12 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn12)), ((locals.var_vg2const_dn17 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn17)),)
    } else {
        (locals.var_t1__blk207, locals.var_t1__blk207_dn0, locals.var_t1__blk207_dn2, locals.var_t1__blk207_dn6, locals.var_t1__blk207_dn7, locals.var_t1__blk207_dn10, locals.var_t1__blk207_dn11, locals.var_t1__blk207_dn12, locals.var_t1__blk207_dn17,)
    }
};
        locals.var_t1__blk207 = assign10320_e9823;
        locals.var_t1__blk207_dn0 = assign10320_e9823_d_n0;
        locals.var_t1__blk207_dn2 = assign10320_e9823_d_n2;
        locals.var_t1__blk207_dn6 = assign10320_e9823_d_n6;
        locals.var_t1__blk207_dn7 = assign10320_e9823_d_n7;
        locals.var_t1__blk207_dn10 = assign10320_e9823_d_n10;
        locals.var_t1__blk207_dn11 = assign10320_e9823_d_n11;
        locals.var_t1__blk207_dn12 = assign10320_e9823_d_n12;
        locals.var_t1__blk207_dn17 = assign10320_e9823_d_n17;
        locals.var_t1__blk207_rv = 0.0;

        let (assign10330_e9841, assign10330_e9841_d_n0, assign10330_e9841_d_n2, assign10330_e9841_d_n6, assign10330_e9841_d_n7, assign10330_e9841_d_n10, assign10330_e9841_d_n11, assign10330_e9841_d_n12, assign10330_e9841_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign10330_e9838: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign10330_e9839: f64 = (locals.var_qnsub_esi / assign10330_e9838);
        (assign10330_e9839, (((locals.var_qnsub_esi_dn0 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign10330_e9838 * assign10330_e9838)), (((locals.var_qnsub_esi_dn2 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign10330_e9838 * assign10330_e9838)), (((locals.var_qnsub_esi_dn6 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign10330_e9838 * assign10330_e9838)), (((locals.var_qnsub_esi_dn7 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign10330_e9838 * assign10330_e9838)), (((locals.var_qnsub_esi_dn10 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign10330_e9838 * assign10330_e9838)), (((locals.var_qnsub_esi_dn11 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign10330_e9838 * assign10330_e9838)), (((locals.var_qnsub_esi_dn12 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign10330_e9838 * assign10330_e9838)), (((locals.var_qnsub_esi_dn17 * assign10330_e9838) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign10330_e9838 * assign10330_e9838)),)
    } else {
        (locals.var_t3__blk209, locals.var_t3__blk209_dn0, locals.var_t3__blk209_dn2, locals.var_t3__blk209_dn6, locals.var_t3__blk209_dn7, locals.var_t3__blk209_dn10, locals.var_t3__blk209_dn11, locals.var_t3__blk209_dn12, locals.var_t3__blk209_dn17,)
    }
};
        locals.var_t3__blk209 = assign10330_e9841;
        locals.var_t3__blk209_dn0 = assign10330_e9841_d_n0;
        locals.var_t3__blk209_dn2 = assign10330_e9841_d_n2;
        locals.var_t3__blk209_dn6 = assign10330_e9841_d_n6;
        locals.var_t3__blk209_dn7 = assign10330_e9841_d_n7;
        locals.var_t3__blk209_dn10 = assign10330_e9841_d_n10;
        locals.var_t3__blk209_dn11 = assign10330_e9841_d_n11;
        locals.var_t3__blk209_dn12 = assign10330_e9841_d_n12;
        locals.var_t3__blk209_dn17 = assign10330_e9841_d_n17;
        locals.var_t3__blk209_rv = 0.0;

        let (assign10340_e9861, assign10340_e9861_d_n0, assign10340_e9861_d_n2, assign10340_e9861_d_n6, assign10340_e9861_d_n7, assign10340_e9861_d_n10, assign10340_e9861_d_n11, assign10340_e9861_d_n12, assign10340_e9861_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign10340_e9855: f64 = (2.0 / locals.var_qnsub_esi);
        let assign10340_e9858: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign10340_e9859: f64 = (assign10340_e9855 * assign10340_e9858);
        (assign10340_e9859, (((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))), (((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))), (((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))), (((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))), (((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))), (((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))), (((-((2.0 * locals.var_qnsub_esi_dn12) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))), (((-((2.0 * locals.var_qnsub_esi_dn17) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign10340_e9858) + (assign10340_e9855 * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))),)
    } else {
        (locals.var_t4__blk210, locals.var_t4__blk210_dn0, locals.var_t4__blk210_dn2, locals.var_t4__blk210_dn6, locals.var_t4__blk210_dn7, locals.var_t4__blk210_dn10, locals.var_t4__blk210_dn11, locals.var_t4__blk210_dn12, locals.var_t4__blk210_dn17,)
    }
};
        locals.var_t4__blk210 = assign10340_e9861;
        locals.var_t4__blk210_dn0 = assign10340_e9861_d_n0;
        locals.var_t4__blk210_dn2 = assign10340_e9861_d_n2;
        locals.var_t4__blk210_dn6 = assign10340_e9861_d_n6;
        locals.var_t4__blk210_dn7 = assign10340_e9861_d_n7;
        locals.var_t4__blk210_dn10 = assign10340_e9861_d_n10;
        locals.var_t4__blk210_dn11 = assign10340_e9861_d_n11;
        locals.var_t4__blk210_dn12 = assign10340_e9861_d_n12;
        locals.var_t4__blk210_dn17 = assign10340_e9861_d_n17;
        locals.var_t4__blk210_rv = 0.0;

        let (assign10350_e9881, assign10350_e9881_d_n0, assign10350_e9881_d_n2, assign10350_e9881_d_n6, assign10350_e9881_d_n7, assign10350_e9881_d_n10, assign10350_e9881_d_n11, assign10350_e9881_d_n12, assign10350_e9881_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign10350_e9875: f64 = (locals.var_t1__blk207 - locals.var_beta_inv);
        let assign10350_e9878: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign10350_e9879: f64 = (assign10350_e9875 - assign10350_e9878);
        (assign10350_e9879, (locals.var_t1__blk207_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk207_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk207_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk207_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk207_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk207_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk207_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk207_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk211, locals.var_t5__blk211_dn0, locals.var_t5__blk211_dn2, locals.var_t5__blk211_dn6, locals.var_t5__blk211_dn7, locals.var_t5__blk211_dn10, locals.var_t5__blk211_dn11, locals.var_t5__blk211_dn12, locals.var_t5__blk211_dn17,)
    }
};
        locals.var_t5__blk211 = assign10350_e9881;
        locals.var_t5__blk211_dn0 = assign10350_e9881_d_n0;
        locals.var_t5__blk211_dn2 = assign10350_e9881_d_n2;
        locals.var_t5__blk211_dn6 = assign10350_e9881_d_n6;
        locals.var_t5__blk211_dn7 = assign10350_e9881_d_n7;
        locals.var_t5__blk211_dn10 = assign10350_e9881_d_n10;
        locals.var_t5__blk211_dn11 = assign10350_e9881_d_n11;
        locals.var_t5__blk211_dn12 = assign10350_e9881_d_n12;
        locals.var_t5__blk211_dn17 = assign10350_e9881_d_n17;
        locals.var_t5__blk211_rv = 0.0;

        let (assign10360_e9899, assign10360_e9899_d_n0, assign10360_e9899_d_n2, assign10360_e9899_d_n6, assign10360_e9899_d_n7, assign10360_e9899_d_n10, assign10360_e9899_d_n11, assign10360_e9899_d_n12, assign10360_e9899_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign10360_e9896: f64 = (locals.var_t4__blk210 * locals.var_t5__blk211);
        let assign10360_e9897: f64 = (1.0 + assign10360_e9896);
        (assign10360_e9897, ((locals.var_t4__blk210_dn0 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn0)), ((locals.var_t4__blk210_dn2 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn2)), ((locals.var_t4__blk210_dn6 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn6)), ((locals.var_t4__blk210_dn7 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn7)), ((locals.var_t4__blk210_dn10 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn10)), ((locals.var_t4__blk210_dn11 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn11)), ((locals.var_t4__blk210_dn12 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn12)), ((locals.var_t4__blk210_dn17 * locals.var_t5__blk211) + (locals.var_t4__blk210 * locals.var_t5__blk211_dn17)),)
    } else {
        (locals.var_t6__blk212, locals.var_t6__blk212_dn0, locals.var_t6__blk212_dn2, locals.var_t6__blk212_dn6, locals.var_t6__blk212_dn7, locals.var_t6__blk212_dn10, locals.var_t6__blk212_dn11, locals.var_t6__blk212_dn12, locals.var_t6__blk212_dn17,)
    }
};
        locals.var_t6__blk212 = assign10360_e9899;
        locals.var_t6__blk212_dn0 = assign10360_e9899_d_n0;
        locals.var_t6__blk212_dn2 = assign10360_e9899_d_n2;
        locals.var_t6__blk212_dn6 = assign10360_e9899_d_n6;
        locals.var_t6__blk212_dn7 = assign10360_e9899_d_n7;
        locals.var_t6__blk212_dn10 = assign10360_e9899_d_n10;
        locals.var_t6__blk212_dn11 = assign10360_e9899_d_n11;
        locals.var_t6__blk212_dn12 = assign10360_e9899_d_n12;
        locals.var_t6__blk212_dn17 = assign10360_e9899_d_n17;
        locals.var_t6__blk212_rv = 0.0;

        let (assign10370_e9917, assign10370_e9917_d_n0, assign10370_e9917_d_n2, assign10370_e9917_d_n6, assign10370_e9917_d_n7, assign10370_e9917_d_n10, assign10370_e9917_d_n11, assign10370_e9917_d_n12, assign10370_e9917_d_n17,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) {
        let assign10370_e9914: f64 = (1.0 + locals.var_t4__blk210);
        let assign10370_e9915: f64 = (2.0 * assign10370_e9914);
        (assign10370_e9915, (2.0 * locals.var_t4__blk210_dn0), (2.0 * locals.var_t4__blk210_dn2), (2.0 * locals.var_t4__blk210_dn6), (2.0 * locals.var_t4__blk210_dn7), (2.0 * locals.var_t4__blk210_dn10), (2.0 * locals.var_t4__blk210_dn11), (2.0 * locals.var_t4__blk210_dn12), (2.0 * locals.var_t4__blk210_dn17),)
    } else {
        (locals.var_t7__blk214, locals.var_t7__blk214_dn0, locals.var_t7__blk214_dn2, locals.var_t7__blk214_dn6, locals.var_t7__blk214_dn7, locals.var_t7__blk214_dn10, locals.var_t7__blk214_dn11, locals.var_t7__blk214_dn12, locals.var_t7__blk214_dn17,)
    }
};
        locals.var_t7__blk214 = assign10370_e9917;
        locals.var_t7__blk214_dn0 = assign10370_e9917_d_n0;
        locals.var_t7__blk214_dn2 = assign10370_e9917_d_n2;
        locals.var_t7__blk214_dn6 = assign10370_e9917_d_n6;
        locals.var_t7__blk214_dn7 = assign10370_e9917_d_n7;
        locals.var_t7__blk214_dn10 = assign10370_e9917_d_n10;
        locals.var_t7__blk214_dn11 = assign10370_e9917_d_n11;
        locals.var_t7__blk214_dn12 = assign10370_e9917_d_n12;
        locals.var_t7__blk214_dn17 = assign10370_e9917_d_n17;
        locals.var_t7__blk214_rv = 0.0;

        let assign10380_e9921: f64 = (1e-50 + locals.var_t7__blk214);
        let assign10380_e9926: f64 = if ((locals.var_t6__blk212 < assign10380_e9921) && (locals.var_t7__blk214 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard223 = assign10380_e9926;
        locals.var_guard223_rv = 0.0;

        let (assign10390_e9946, assign10390_e9946_d_n0, assign10390_e9946_d_n2, assign10390_e9946_d_n6, assign10390_e9946_d_n7, assign10390_e9946_d_n10, assign10390_e9946_d_n11, assign10390_e9946_d_n12, assign10390_e9946_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) && (locals.var_guard223 != 0.0)) {
        let assign10390_e9942: f64 = (1e-50 + locals.var_t7__blk214);
        let assign10390_e9944: f64 = (assign10390_e9942 - locals.var_t6__blk212);
        (assign10390_e9944, (locals.var_t7__blk214_dn0 - locals.var_t6__blk212_dn0), (locals.var_t7__blk214_dn2 - locals.var_t6__blk212_dn2), (locals.var_t7__blk214_dn6 - locals.var_t6__blk212_dn6), (locals.var_t7__blk214_dn7 - locals.var_t6__blk212_dn7), (locals.var_t7__blk214_dn10 - locals.var_t6__blk212_dn10), (locals.var_t7__blk214_dn11 - locals.var_t6__blk212_dn11), (locals.var_t7__blk214_dn12 - locals.var_t6__blk212_dn12), (locals.var_t7__blk214_dn17 - locals.var_t6__blk212_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign10390_e9946;
        locals.var_tmf1_dn0 = assign10390_e9946_d_n0;
        locals.var_tmf1_dn2 = assign10390_e9946_d_n2;
        locals.var_tmf1_dn6 = assign10390_e9946_d_n6;
        locals.var_tmf1_dn7 = assign10390_e9946_d_n7;
        locals.var_tmf1_dn10 = assign10390_e9946_d_n10;
        locals.var_tmf1_dn11 = assign10390_e9946_d_n11;
        locals.var_tmf1_dn12 = assign10390_e9946_d_n12;
        locals.var_tmf1_dn17 = assign10390_e9946_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign10400_e9964, assign10400_e9964_d_n0, assign10400_e9964_d_n2, assign10400_e9964_d_n6, assign10400_e9964_d_n7, assign10400_e9964_d_n10, assign10400_e9964_d_n11, assign10400_e9964_d_n12, assign10400_e9964_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) && (locals.var_guard223 != 0.0)) {
        let assign10400_e9962: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign10400_e9962, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign10400_e9964;
        locals.var_x2_dn0 = assign10400_e9964_d_n0;
        locals.var_x2_dn2 = assign10400_e9964_d_n2;
        locals.var_x2_dn6 = assign10400_e9964_d_n6;
        locals.var_x2_dn7 = assign10400_e9964_d_n7;
        locals.var_x2_dn10 = assign10400_e9964_d_n10;
        locals.var_x2_dn11 = assign10400_e9964_d_n11;
        locals.var_x2_dn12 = assign10400_e9964_d_n12;
        locals.var_x2_dn17 = assign10400_e9964_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign10410_e9982, assign10410_e9982_d_n0, assign10410_e9982_d_n2, assign10410_e9982_d_n6, assign10410_e9982_d_n7, assign10410_e9982_d_n10, assign10410_e9982_d_n11, assign10410_e9982_d_n12, assign10410_e9982_d_n17,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard176 != 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard219 == 0.0)) && (locals.var_guard220 == 0.0)) && (locals.var_guard223 != 0.0)) {
        let assign10410_e9980: f64 = (locals.var_t7__blk214 * locals.var_t7__blk214);
        (assign10410_e9980, ((locals.var_t7__blk214_dn0 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn0)), ((locals.var_t7__blk214_dn2 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn2)), ((locals.var_t7__blk214_dn6 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn6)), ((locals.var_t7__blk214_dn7 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn7)), ((locals.var_t7__blk214_dn10 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn10)), ((locals.var_t7__blk214_dn11 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn11)), ((locals.var_t7__blk214_dn12 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn12)), ((locals.var_t7__blk214_dn17 * locals.var_t7__blk214) + (locals.var_t7__blk214 * locals.var_t7__blk214_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign10410_e9982;
        locals.var_xmax2_dn0 = assign10410_e9982_d_n0;
        locals.var_xmax2_dn2 = assign10410_e9982_d_n2;
        locals.var_xmax2_dn6 = assign10410_e9982_d_n6;
        locals.var_xmax2_dn7 = assign10410_e9982_d_n7;
        locals.var_xmax2_dn10 = assign10410_e9982_d_n10;
        locals.var_xmax2_dn11 = assign10410_e9982_d_n11;
        locals.var_xmax2_dn12 = assign10410_e9982_d_n12;
        locals.var_xmax2_dn17 = assign10410_e9982_d_n17;
        locals.var_xmax2_rv = 0.0;

    }
}
