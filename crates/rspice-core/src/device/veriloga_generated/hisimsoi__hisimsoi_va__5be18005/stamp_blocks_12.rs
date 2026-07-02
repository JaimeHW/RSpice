#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18040_e25586, assign18040_e25586_d_n0, assign18040_e25586_d_n2, assign18040_e25586_d_n6, assign18040_e25586_d_n7, assign18040_e25586_d_n10, assign18040_e25586_d_n11, assign18040_e25586_d_n12, assign18040_e25586_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18040_e25584: f64 = (1.0 / locals.var_t1__blk537);
        (assign18040_e25584, (-(locals.var_t1__blk537_dn0 / (locals.var_t1__blk537 * locals.var_t1__blk537))), (-(locals.var_t1__blk537_dn2 / (locals.var_t1__blk537 * locals.var_t1__blk537))), (-(locals.var_t1__blk537_dn6 / (locals.var_t1__blk537 * locals.var_t1__blk537))), (-(locals.var_t1__blk537_dn7 / (locals.var_t1__blk537 * locals.var_t1__blk537))), (-(locals.var_t1__blk537_dn10 / (locals.var_t1__blk537 * locals.var_t1__blk537))), (-(locals.var_t1__blk537_dn11 / (locals.var_t1__blk537 * locals.var_t1__blk537))), (-(locals.var_t1__blk537_dn12 / (locals.var_t1__blk537 * locals.var_t1__blk537))), (-(locals.var_t1__blk537_dn17 / (locals.var_t1__blk537 * locals.var_t1__blk537))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn12, locals.var_muun_dn17,)
    }
};
        locals.var_muun = assign18040_e25586;
        locals.var_muun_dn0 = assign18040_e25586_d_n0;
        locals.var_muun_dn2 = assign18040_e25586_d_n2;
        locals.var_muun_dn6 = assign18040_e25586_d_n6;
        locals.var_muun_dn7 = assign18040_e25586_d_n7;
        locals.var_muun_dn10 = assign18040_e25586_d_n10;
        locals.var_muun_dn11 = assign18040_e25586_d_n11;
        locals.var_muun_dn12 = assign18040_e25586_d_n12;
        locals.var_muun_dn17 = assign18040_e25586_d_n17;
        locals.var_muun_rv = 0.0;

        let (assign18050_e25592, assign18050_e25592_d_n0, assign18050_e25592_d_n2, assign18050_e25592_d_n6, assign18050_e25592_d_n7, assign18050_e25592_d_n10, assign18050_e25592_d_n11, assign18050_e25592_d_n12, assign18050_e25592_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18050_e25590: f64 = (locals.var_muun * 0.0001);
        (assign18050_e25590, (locals.var_muun_dn0 * 0.0001), (locals.var_muun_dn2 * 0.0001), (locals.var_muun_dn6 * 0.0001), (locals.var_muun_dn7 * 0.0001), (locals.var_muun_dn10 * 0.0001), (locals.var_muun_dn11 * 0.0001), (locals.var_muun_dn12 * 0.0001), (locals.var_muun_dn17 * 0.0001),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn12, locals.var_muun_dn17,)
    }
};
        locals.var_muun = assign18050_e25592;
        locals.var_muun_dn0 = assign18050_e25592_d_n0;
        locals.var_muun_dn2 = assign18050_e25592_d_n2;
        locals.var_muun_dn6 = assign18050_e25592_d_n6;
        locals.var_muun_dn7 = assign18050_e25592_d_n7;
        locals.var_muun_dn10 = assign18050_e25592_d_n10;
        locals.var_muun_dn11 = assign18050_e25592_d_n11;
        locals.var_muun_dn12 = assign18050_e25592_d_n12;
        locals.var_muun_dn17 = assign18050_e25592_d_n17;
        locals.var_muun_rv = 0.0;

        let (assign18060_e25600, assign18060_e25600_d_n0, assign18060_e25600_d_n2, assign18060_e25600_d_n6, assign18060_e25600_d_n7, assign18060_e25600_d_n10, assign18060_e25600_d_n11, assign18060_e25600_d_n12, assign18060_e25600_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18060_e25596: f64 = (locals.var_beta * locals.var_qn0);
        let assign18060_e25598: f64 = (assign18060_e25596 * locals.var_lch);
        (assign18060_e25598, (((locals.var_beta * locals.var_qn0_dn0) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn0)), (((locals.var_beta * locals.var_qn0_dn2) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn2)), (((locals.var_beta * locals.var_qn0_dn6) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn6)), (((locals.var_beta * locals.var_qn0_dn7) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn7)), ((((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn10)), (((locals.var_beta * locals.var_qn0_dn11) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn11)), (((locals.var_beta * locals.var_qn0_dn12) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn12)), (((locals.var_beta * locals.var_qn0_dn17) * locals.var_lch) + (assign18060_e25596 * locals.var_lch_dn17)),)
    } else {
        (locals.var_t2__blk550, locals.var_t2__blk550_dn0, locals.var_t2__blk550_dn2, locals.var_t2__blk550_dn6, locals.var_t2__blk550_dn7, locals.var_t2__blk550_dn10, locals.var_t2__blk550_dn11, locals.var_t2__blk550_dn12, locals.var_t2__blk550_dn17,)
    }
};
        locals.var_t2__blk550 = assign18060_e25600;
        locals.var_t2__blk550_dn0 = assign18060_e25600_d_n0;
        locals.var_t2__blk550_dn2 = assign18060_e25600_d_n2;
        locals.var_t2__blk550_dn6 = assign18060_e25600_d_n6;
        locals.var_t2__blk550_dn7 = assign18060_e25600_d_n7;
        locals.var_t2__blk550_dn10 = assign18060_e25600_d_n10;
        locals.var_t2__blk550_dn11 = assign18060_e25600_d_n11;
        locals.var_t2__blk550_dn12 = assign18060_e25600_d_n12;
        locals.var_t2__blk550_dn17 = assign18060_e25600_d_n17;
        locals.var_t2__blk550_rv = 0.0;

        let (assign18070_e25613, assign18070_e25613_d_n0, assign18070_e25613_d_n2, assign18070_e25613_d_n6, assign18070_e25613_d_n7, assign18070_e25613_d_n10, assign18070_e25613_d_n11, assign18070_e25613_d_n12, assign18070_e25613_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18070_e25604: f64 = (locals.var_t2__blk550 * locals.var_t2__blk550);
        let assign18070_e25607: f64 = (4.0 * 1e-50);
        let assign18070_e25609: f64 = (assign18070_e25607 * 1e-50);
        let assign18070_e25610: f64 = (assign18070_e25604 + assign18070_e25609);
        let assign18070_e25611: f64 = (assign18070_e25610).sqrt();
        (assign18070_e25611, (((locals.var_t2__blk550_dn0 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn0)) / (2.0 * assign18070_e25611)), (((locals.var_t2__blk550_dn2 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn2)) / (2.0 * assign18070_e25611)), (((locals.var_t2__blk550_dn6 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn6)) / (2.0 * assign18070_e25611)), (((locals.var_t2__blk550_dn7 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn7)) / (2.0 * assign18070_e25611)), (((locals.var_t2__blk550_dn10 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn10)) / (2.0 * assign18070_e25611)), (((locals.var_t2__blk550_dn11 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn11)) / (2.0 * assign18070_e25611)), (((locals.var_t2__blk550_dn12 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn12)) / (2.0 * assign18070_e25611)), (((locals.var_t2__blk550_dn17 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn17)) / (2.0 * assign18070_e25611)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18070_e25613;
        locals.var_tmf1_dn0 = assign18070_e25613_d_n0;
        locals.var_tmf1_dn2 = assign18070_e25613_d_n2;
        locals.var_tmf1_dn6 = assign18070_e25613_d_n6;
        locals.var_tmf1_dn7 = assign18070_e25613_d_n7;
        locals.var_tmf1_dn10 = assign18070_e25613_d_n10;
        locals.var_tmf1_dn11 = assign18070_e25613_d_n11;
        locals.var_tmf1_dn12 = assign18070_e25613_d_n12;
        locals.var_tmf1_dn17 = assign18070_e25613_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18080_e25625, assign18080_e25625_d_n0, assign18080_e25625_d_n2, assign18080_e25625_d_n6, assign18080_e25625_d_n7, assign18080_e25625_d_n10, assign18080_e25625_d_n11, assign18080_e25625_d_n12, assign18080_e25625_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18080_e25618: f64 = (locals.var_t2__blk550 + locals.var_tmf1);
        let assign18080_e25619: f64 = (0.5 * assign18080_e25618);
        let assign18080_e25622: f64 = (1e-10 * 1e-50);
        let assign18080_e25623: f64 = (assign18080_e25619 + assign18080_e25622);
        (assign18080_e25623, (0.5 * (locals.var_t2__blk550_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t2__blk550_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t2__blk550_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t2__blk550_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t2__blk550_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t2__blk550_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t2__blk550_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t2__blk550_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk550, locals.var_t2__blk550_dn0, locals.var_t2__blk550_dn2, locals.var_t2__blk550_dn6, locals.var_t2__blk550_dn7, locals.var_t2__blk550_dn10, locals.var_t2__blk550_dn11, locals.var_t2__blk550_dn12, locals.var_t2__blk550_dn17,)
    }
};
        locals.var_t2__blk550 = assign18080_e25625;
        locals.var_t2__blk550_dn0 = assign18080_e25625_d_n0;
        locals.var_t2__blk550_dn2 = assign18080_e25625_d_n2;
        locals.var_t2__blk550_dn6 = assign18080_e25625_d_n6;
        locals.var_t2__blk550_dn7 = assign18080_e25625_d_n7;
        locals.var_t2__blk550_dn10 = assign18080_e25625_d_n10;
        locals.var_t2__blk550_dn11 = assign18080_e25625_d_n11;
        locals.var_t2__blk550_dn12 = assign18080_e25625_d_n12;
        locals.var_t2__blk550_dn17 = assign18080_e25625_d_n17;
        locals.var_t2__blk550_rv = 0.0;

        let assign18090_e25628: f64 = if locals.var_t2__blk550 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign18090_e25628;
        locals.var_guard558_rv = 0.0;

        let (assign18100_e25634, assign18100_e25634_d_n0, assign18100_e25634_d_n2, assign18100_e25634_d_n6, assign18100_e25634_d_n7, assign18100_e25634_d_n10, assign18100_e25634_d_n11, assign18100_e25634_d_n12, assign18100_e25634_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard558 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk550, locals.var_t2__blk550_dn0, locals.var_t2__blk550_dn2, locals.var_t2__blk550_dn6, locals.var_t2__blk550_dn7, locals.var_t2__blk550_dn10, locals.var_t2__blk550_dn11, locals.var_t2__blk550_dn12, locals.var_t2__blk550_dn17,)
    }
};
        locals.var_t2__blk550 = assign18100_e25634;
        locals.var_t2__blk550_dn0 = assign18100_e25634_d_n0;
        locals.var_t2__blk550_dn2 = assign18100_e25634_d_n2;
        locals.var_t2__blk550_dn6 = assign18100_e25634_d_n6;
        locals.var_t2__blk550_dn7 = assign18100_e25634_d_n7;
        locals.var_t2__blk550_dn10 = assign18100_e25634_d_n10;
        locals.var_t2__blk550_dn11 = assign18100_e25634_d_n11;
        locals.var_t2__blk550_dn12 = assign18100_e25634_d_n12;
        locals.var_t2__blk550_dn17 = assign18100_e25634_d_n17;
        locals.var_t2__blk550_rv = 0.0;

        let (assign18110_e25640, assign18110_e25640_d_n0, assign18110_e25640_d_n2, assign18110_e25640_d_n6, assign18110_e25640_d_n7, assign18110_e25640_d_n10, assign18110_e25640_d_n11, assign18110_e25640_d_n12, assign18110_e25640_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18110_e25638: f64 = (1.0 / locals.var_t2__blk550);
        (assign18110_e25638, (-(locals.var_t2__blk550_dn0 / (locals.var_t2__blk550 * locals.var_t2__blk550))), (-(locals.var_t2__blk550_dn2 / (locals.var_t2__blk550 * locals.var_t2__blk550))), (-(locals.var_t2__blk550_dn6 / (locals.var_t2__blk550 * locals.var_t2__blk550))), (-(locals.var_t2__blk550_dn7 / (locals.var_t2__blk550 * locals.var_t2__blk550))), (-(locals.var_t2__blk550_dn10 / (locals.var_t2__blk550 * locals.var_t2__blk550))), (-(locals.var_t2__blk550_dn11 / (locals.var_t2__blk550 * locals.var_t2__blk550))), (-(locals.var_t2__blk550_dn12 / (locals.var_t2__blk550 * locals.var_t2__blk550))), (-(locals.var_t2__blk550_dn17 / (locals.var_t2__blk550 * locals.var_t2__blk550))),)
    } else {
        (locals.var_t1__blk551, locals.var_t1__blk551_dn0, locals.var_t1__blk551_dn2, locals.var_t1__blk551_dn6, locals.var_t1__blk551_dn7, locals.var_t1__blk551_dn10, locals.var_t1__blk551_dn11, locals.var_t1__blk551_dn12, locals.var_t1__blk551_dn17,)
    }
};
        locals.var_t1__blk551 = assign18110_e25640;
        locals.var_t1__blk551_dn0 = assign18110_e25640_d_n0;
        locals.var_t1__blk551_dn2 = assign18110_e25640_d_n2;
        locals.var_t1__blk551_dn6 = assign18110_e25640_d_n6;
        locals.var_t1__blk551_dn7 = assign18110_e25640_d_n7;
        locals.var_t1__blk551_dn10 = assign18110_e25640_d_n10;
        locals.var_t1__blk551_dn11 = assign18110_e25640_d_n11;
        locals.var_t1__blk551_dn12 = assign18110_e25640_d_n12;
        locals.var_t1__blk551_dn17 = assign18110_e25640_d_n17;
        locals.var_t1__blk551_rv = 0.0;

        let (assign18120_e25646, assign18120_e25646_d_n0, assign18120_e25646_d_n2, assign18120_e25646_d_n6, assign18120_e25646_d_n7, assign18120_e25646_d_n10, assign18120_e25646_d_n11, assign18120_e25646_d_n12, assign18120_e25646_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18120_e25644: f64 = (locals.var_idd * locals.var_t1__blk551);
        (assign18120_e25644, ((locals.var_idd_dn0 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn0)), ((locals.var_idd_dn2 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn2)), ((locals.var_idd_dn6 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn6)), ((locals.var_idd_dn7 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn7)), ((locals.var_idd_dn10 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn10)), ((locals.var_idd_dn11 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn11)), ((locals.var_idd_dn12 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn12)), ((locals.var_idd_dn17 * locals.var_t1__blk551) + (locals.var_idd * locals.var_t1__blk551_dn17)),)
    } else {
        (locals.var_ty__blk552, locals.var_ty__blk552_dn0, locals.var_ty__blk552_dn2, locals.var_ty__blk552_dn6, locals.var_ty__blk552_dn7, locals.var_ty__blk552_dn10, locals.var_ty__blk552_dn11, locals.var_ty__blk552_dn12, locals.var_ty__blk552_dn17,)
    }
};
        locals.var_ty__blk552 = assign18120_e25646;
        locals.var_ty__blk552_dn0 = assign18120_e25646_d_n0;
        locals.var_ty__blk552_dn2 = assign18120_e25646_d_n2;
        locals.var_ty__blk552_dn6 = assign18120_e25646_d_n6;
        locals.var_ty__blk552_dn7 = assign18120_e25646_d_n7;
        locals.var_ty__blk552_dn10 = assign18120_e25646_d_n10;
        locals.var_ty__blk552_dn11 = assign18120_e25646_d_n11;
        locals.var_ty__blk552_dn12 = assign18120_e25646_d_n12;
        locals.var_ty__blk552_dn17 = assign18120_e25646_d_n17;
        locals.var_ty__blk552_rv = 0.0;

        let (assign18130_e25654, assign18130_e25654_d_n0, assign18130_e25654_d_n2, assign18130_e25654_d_n6, assign18130_e25654_d_n7, assign18130_e25654_d_n10, assign18130_e25654_d_n11, assign18130_e25654_d_n12, assign18130_e25654_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18130_e25650: f64 = (0.2 * locals.var_vmaxe);
        let assign18130_e25652: f64 = (assign18130_e25650 / locals.var_muun);
        (assign18130_e25652, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn12) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn12)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn17) * locals.var_muun) - (assign18130_e25650 * locals.var_muun_dn17)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2__blk550, locals.var_t2__blk550_dn0, locals.var_t2__blk550_dn2, locals.var_t2__blk550_dn6, locals.var_t2__blk550_dn7, locals.var_t2__blk550_dn10, locals.var_t2__blk550_dn11, locals.var_t2__blk550_dn12, locals.var_t2__blk550_dn17,)
    }
};
        locals.var_t2__blk550 = assign18130_e25654;
        locals.var_t2__blk550_dn0 = assign18130_e25654_d_n0;
        locals.var_t2__blk550_dn2 = assign18130_e25654_d_n2;
        locals.var_t2__blk550_dn6 = assign18130_e25654_d_n6;
        locals.var_t2__blk550_dn7 = assign18130_e25654_d_n7;
        locals.var_t2__blk550_dn10 = assign18130_e25654_d_n10;
        locals.var_t2__blk550_dn11 = assign18130_e25654_d_n11;
        locals.var_t2__blk550_dn12 = assign18130_e25654_d_n12;
        locals.var_t2__blk550_dn17 = assign18130_e25654_d_n17;
        locals.var_t2__blk550_rv = 0.0;

        let (assign18140_e25665, assign18140_e25665_d_n0, assign18140_e25665_d_n2, assign18140_e25665_d_n6, assign18140_e25665_d_n7, assign18140_e25665_d_n10, assign18140_e25665_d_n11, assign18140_e25665_d_n12, assign18140_e25665_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18140_e25658: f64 = (locals.var_ty__blk552 * locals.var_ty__blk552);
        let assign18140_e25661: f64 = (locals.var_t2__blk550 * locals.var_t2__blk550);
        let assign18140_e25662: f64 = (assign18140_e25658 + assign18140_e25661);
        let assign18140_e25663: f64 = (assign18140_e25662).sqrt();
        (assign18140_e25663, ((((locals.var_ty__blk552_dn0 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn0)) + ((locals.var_t2__blk550_dn0 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn0))) / (2.0 * assign18140_e25663)), ((((locals.var_ty__blk552_dn2 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn2)) + ((locals.var_t2__blk550_dn2 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn2))) / (2.0 * assign18140_e25663)), ((((locals.var_ty__blk552_dn6 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn6)) + ((locals.var_t2__blk550_dn6 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn6))) / (2.0 * assign18140_e25663)), ((((locals.var_ty__blk552_dn7 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn7)) + ((locals.var_t2__blk550_dn7 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn7))) / (2.0 * assign18140_e25663)), ((((locals.var_ty__blk552_dn10 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn10)) + ((locals.var_t2__blk550_dn10 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn10))) / (2.0 * assign18140_e25663)), ((((locals.var_ty__blk552_dn11 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn11)) + ((locals.var_t2__blk550_dn11 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn11))) / (2.0 * assign18140_e25663)), ((((locals.var_ty__blk552_dn12 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn12)) + ((locals.var_t2__blk550_dn12 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn12))) / (2.0 * assign18140_e25663)), ((((locals.var_ty__blk552_dn17 * locals.var_ty__blk552) + (locals.var_ty__blk552 * locals.var_ty__blk552_dn17)) + ((locals.var_t2__blk550_dn17 * locals.var_t2__blk550) + (locals.var_t2__blk550 * locals.var_t2__blk550_dn17))) / (2.0 * assign18140_e25663)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn12, locals.var_ey_dn17,)
    }
};
        locals.var_ey = assign18140_e25665;
        locals.var_ey_dn0 = assign18140_e25665_d_n0;
        locals.var_ey_dn2 = assign18140_e25665_d_n2;
        locals.var_ey_dn6 = assign18140_e25665_d_n6;
        locals.var_ey_dn7 = assign18140_e25665_d_n7;
        locals.var_ey_dn10 = assign18140_e25665_d_n10;
        locals.var_ey_dn11 = assign18140_e25665_d_n11;
        locals.var_ey_dn12 = assign18140_e25665_d_n12;
        locals.var_ey_dn17 = assign18140_e25665_d_n17;
        locals.var_ey_rv = 0.0;

        let (assign18150_e25671, assign18150_e25671_d_n0, assign18150_e25671_d_n2, assign18150_e25671_d_n6, assign18150_e25671_d_n7, assign18150_e25671_d_n10, assign18150_e25671_d_n11, assign18150_e25671_d_n12, assign18150_e25671_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18150_e25669: f64 = (locals.var_muun * locals.var_ey);
        (assign18150_e25669, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn12 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn12)), ((locals.var_muun_dn17 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn17)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn12, locals.var_em_dn17,)
    }
};
        locals.var_em = assign18150_e25671;
        locals.var_em_dn0 = assign18150_e25671_d_n0;
        locals.var_em_dn2 = assign18150_e25671_d_n2;
        locals.var_em_dn6 = assign18150_e25671_d_n6;
        locals.var_em_dn7 = assign18150_e25671_d_n7;
        locals.var_em_dn10 = assign18150_e25671_d_n10;
        locals.var_em_dn11 = assign18150_e25671_d_n11;
        locals.var_em_dn12 = assign18150_e25671_d_n12;
        locals.var_em_dn17 = assign18150_e25671_d_n17;
        locals.var_em_rv = 0.0;

        let (assign18160_e25677, assign18160_e25677_d_n0, assign18160_e25677_d_n2, assign18160_e25677_d_n6, assign18160_e25677_d_n7, assign18160_e25677_d_n10, assign18160_e25677_d_n11, assign18160_e25677_d_n12, assign18160_e25677_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18160_e25675: f64 = (locals.var_em / locals.var_vmaxe);
        (assign18160_e25675, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn12 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn12)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn17 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn17)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1__blk551, locals.var_t1__blk551_dn0, locals.var_t1__blk551_dn2, locals.var_t1__blk551_dn6, locals.var_t1__blk551_dn7, locals.var_t1__blk551_dn10, locals.var_t1__blk551_dn11, locals.var_t1__blk551_dn12, locals.var_t1__blk551_dn17,)
    }
};
        locals.var_t1__blk551 = assign18160_e25677;
        locals.var_t1__blk551_dn0 = assign18160_e25677_d_n0;
        locals.var_t1__blk551_dn2 = assign18160_e25677_d_n2;
        locals.var_t1__blk551_dn6 = assign18160_e25677_d_n6;
        locals.var_t1__blk551_dn7 = assign18160_e25677_d_n7;
        locals.var_t1__blk551_dn10 = assign18160_e25677_d_n10;
        locals.var_t1__blk551_dn11 = assign18160_e25677_d_n11;
        locals.var_t1__blk551_dn12 = assign18160_e25677_d_n12;
        locals.var_t1__blk551_dn17 = assign18160_e25677_d_n17;
        locals.var_t1__blk551_rv = 0.0;

        let assign18170_e25681: f64 = (10.0 * 2.220446049250313e-16);
        let assign18170_e25682: f64 = (1.0 - assign18170_e25681);
        let assign18170_e25689: f64 = (10.0 * 2.220446049250313e-16);
        let assign18170_e25690: f64 = (1.0 + assign18170_e25689);
        let assign18170_e25692: f64 = if ((assign18170_e25682 <= p.p113) && (p.p113 <= assign18170_e25690)) { 1.0 } else { 0.0 };
        locals.var_guard559 = assign18170_e25692;
        locals.var_guard559_rv = 0.0;

        let (assign18180_e25698, assign18180_e25698_d_n0, assign18180_e25698_d_n2, assign18180_e25698_d_n6, assign18180_e25698_d_n7, assign18180_e25698_d_n10, assign18180_e25698_d_n11, assign18180_e25698_d_n12, assign18180_e25698_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard559 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk554, locals.var_t3__blk554_dn0, locals.var_t3__blk554_dn2, locals.var_t3__blk554_dn6, locals.var_t3__blk554_dn7, locals.var_t3__blk554_dn10, locals.var_t3__blk554_dn11, locals.var_t3__blk554_dn12, locals.var_t3__blk554_dn17,)
    }
};
        locals.var_t3__blk554 = assign18180_e25698;
        locals.var_t3__blk554_dn0 = assign18180_e25698_d_n0;
        locals.var_t3__blk554_dn2 = assign18180_e25698_d_n2;
        locals.var_t3__blk554_dn6 = assign18180_e25698_d_n6;
        locals.var_t3__blk554_dn7 = assign18180_e25698_d_n7;
        locals.var_t3__blk554_dn10 = assign18180_e25698_d_n10;
        locals.var_t3__blk554_dn11 = assign18180_e25698_d_n11;
        locals.var_t3__blk554_dn12 = assign18180_e25698_d_n12;
        locals.var_t3__blk554_dn17 = assign18180_e25698_d_n17;
        locals.var_t3__blk554_rv = 0.0;

        let assign18190_e25702: f64 = (10.0 * 2.220446049250313e-16);
        let assign18190_e25703: f64 = (2.0 - assign18190_e25702);
        let assign18190_e25710: f64 = (10.0 * 2.220446049250313e-16);
        let assign18190_e25711: f64 = (2.0 + assign18190_e25710);
        let assign18190_e25713: f64 = if ((assign18190_e25703 <= p.p113) && (p.p113 <= assign18190_e25711)) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign18190_e25713;
        locals.var_guard560_rv = 0.0;

        let (assign18200_e25722, assign18200_e25722_d_n0, assign18200_e25722_d_n2, assign18200_e25722_d_n6, assign18200_e25722_d_n7, assign18200_e25722_d_n10, assign18200_e25722_d_n11, assign18200_e25722_d_n12, assign18200_e25722_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard559 == 0.0)) && (locals.var_guard560 != 0.0)) {
        (locals.var_t1__blk551, locals.var_t1__blk551_dn0, locals.var_t1__blk551_dn2, locals.var_t1__blk551_dn6, locals.var_t1__blk551_dn7, locals.var_t1__blk551_dn10, locals.var_t1__blk551_dn11, locals.var_t1__blk551_dn12, locals.var_t1__blk551_dn17,)
    } else {
        (locals.var_t3__blk554, locals.var_t3__blk554_dn0, locals.var_t3__blk554_dn2, locals.var_t3__blk554_dn6, locals.var_t3__blk554_dn7, locals.var_t3__blk554_dn10, locals.var_t3__blk554_dn11, locals.var_t3__blk554_dn12, locals.var_t3__blk554_dn17,)
    }
};
        locals.var_t3__blk554 = assign18200_e25722;
        locals.var_t3__blk554_dn0 = assign18200_e25722_d_n0;
        locals.var_t3__blk554_dn2 = assign18200_e25722_d_n2;
        locals.var_t3__blk554_dn6 = assign18200_e25722_d_n6;
        locals.var_t3__blk554_dn7 = assign18200_e25722_d_n7;
        locals.var_t3__blk554_dn10 = assign18200_e25722_d_n10;
        locals.var_t3__blk554_dn11 = assign18200_e25722_d_n11;
        locals.var_t3__blk554_dn12 = assign18200_e25722_d_n12;
        locals.var_t3__blk554_dn17 = assign18200_e25722_d_n17;
        locals.var_t3__blk554_rv = 0.0;

        let (assign18210_e25736, assign18210_e25736_d_n0, assign18210_e25736_d_n2, assign18210_e25736_d_n6, assign18210_e25736_d_n7, assign18210_e25736_d_n10, assign18210_e25736_d_n11, assign18210_e25736_d_n12, assign18210_e25736_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard559 == 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign18210_e25733: f64 = (p.p113 - 1.0);
        let assign18210_e25734: f64 = (locals.var_t1__blk551).powf(assign18210_e25733);
        (assign18210_e25734, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn0)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn0 / locals.var_t1__blk551))) }, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn2)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn2 / locals.var_t1__blk551))) }, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn6)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn6 / locals.var_t1__blk551))) }, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn7)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn7 / locals.var_t1__blk551))) }, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn10)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn10 / locals.var_t1__blk551))) }, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn11)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn11 / locals.var_t1__blk551))) }, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn12)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn12 / locals.var_t1__blk551))) }, if 0.0 == 0.0 && ((assign18210_e25733) as f64).is_finite() && ((assign18210_e25733) as f64).fract() == 0.0 { if assign18210_e25733 == 0.0 { 0.0 } else { (assign18210_e25733 * ((locals.var_t1__blk551).powf(assign18210_e25733 - 1.0) * locals.var_t1__blk551_dn17)) } } else { (assign18210_e25734 * (assign18210_e25733 * (locals.var_t1__blk551_dn17 / locals.var_t1__blk551))) },)
    } else {
        (locals.var_t3__blk554, locals.var_t3__blk554_dn0, locals.var_t3__blk554_dn2, locals.var_t3__blk554_dn6, locals.var_t3__blk554_dn7, locals.var_t3__blk554_dn10, locals.var_t3__blk554_dn11, locals.var_t3__blk554_dn12, locals.var_t3__blk554_dn17,)
    }
};
        locals.var_t3__blk554 = assign18210_e25736;
        locals.var_t3__blk554_dn0 = assign18210_e25736_d_n0;
        locals.var_t3__blk554_dn2 = assign18210_e25736_d_n2;
        locals.var_t3__blk554_dn6 = assign18210_e25736_d_n6;
        locals.var_t3__blk554_dn7 = assign18210_e25736_d_n7;
        locals.var_t3__blk554_dn10 = assign18210_e25736_d_n10;
        locals.var_t3__blk554_dn11 = assign18210_e25736_d_n11;
        locals.var_t3__blk554_dn12 = assign18210_e25736_d_n12;
        locals.var_t3__blk554_dn17 = assign18210_e25736_d_n17;
        locals.var_t3__blk554_rv = 0.0;

        let (assign18220_e25742, assign18220_e25742_d_n0, assign18220_e25742_d_n2, assign18220_e25742_d_n6, assign18220_e25742_d_n7, assign18220_e25742_d_n10, assign18220_e25742_d_n11, assign18220_e25742_d_n12, assign18220_e25742_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18220_e25740: f64 = (locals.var_t1__blk551 * locals.var_t3__blk554);
        (assign18220_e25740, ((locals.var_t1__blk551_dn0 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn0)), ((locals.var_t1__blk551_dn2 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn2)), ((locals.var_t1__blk551_dn6 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn6)), ((locals.var_t1__blk551_dn7 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn7)), ((locals.var_t1__blk551_dn10 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn10)), ((locals.var_t1__blk551_dn11 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn11)), ((locals.var_t1__blk551_dn12 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn12)), ((locals.var_t1__blk551_dn17 * locals.var_t3__blk554) + (locals.var_t1__blk551 * locals.var_t3__blk554_dn17)),)
    } else {
        (locals.var_t2__blk550, locals.var_t2__blk550_dn0, locals.var_t2__blk550_dn2, locals.var_t2__blk550_dn6, locals.var_t2__blk550_dn7, locals.var_t2__blk550_dn10, locals.var_t2__blk550_dn11, locals.var_t2__blk550_dn12, locals.var_t2__blk550_dn17,)
    }
};
        locals.var_t2__blk550 = assign18220_e25742;
        locals.var_t2__blk550_dn0 = assign18220_e25742_d_n0;
        locals.var_t2__blk550_dn2 = assign18220_e25742_d_n2;
        locals.var_t2__blk550_dn6 = assign18220_e25742_d_n6;
        locals.var_t2__blk550_dn7 = assign18220_e25742_d_n7;
        locals.var_t2__blk550_dn10 = assign18220_e25742_d_n10;
        locals.var_t2__blk550_dn11 = assign18220_e25742_d_n11;
        locals.var_t2__blk550_dn12 = assign18220_e25742_d_n12;
        locals.var_t2__blk550_dn17 = assign18220_e25742_d_n17;
        locals.var_t2__blk550_rv = 0.0;

        let (assign18230_e25748, assign18230_e25748_d_n0, assign18230_e25748_d_n2, assign18230_e25748_d_n6, assign18230_e25748_d_n7, assign18230_e25748_d_n10, assign18230_e25748_d_n11, assign18230_e25748_d_n12, assign18230_e25748_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18230_e25746: f64 = (1.0 + locals.var_t2__blk550);
        (assign18230_e25746, locals.var_t2__blk550_dn0, locals.var_t2__blk550_dn2, locals.var_t2__blk550_dn6, locals.var_t2__blk550_dn7, locals.var_t2__blk550_dn10, locals.var_t2__blk550_dn11, locals.var_t2__blk550_dn12, locals.var_t2__blk550_dn17,)
    } else {
        (locals.var_t4__blk555, locals.var_t4__blk555_dn0, locals.var_t4__blk555_dn2, locals.var_t4__blk555_dn6, locals.var_t4__blk555_dn7, locals.var_t4__blk555_dn10, locals.var_t4__blk555_dn11, locals.var_t4__blk555_dn12, locals.var_t4__blk555_dn17,)
    }
};
        locals.var_t4__blk555 = assign18230_e25748;
        locals.var_t4__blk555_dn0 = assign18230_e25748_d_n0;
        locals.var_t4__blk555_dn2 = assign18230_e25748_d_n2;
        locals.var_t4__blk555_dn6 = assign18230_e25748_d_n6;
        locals.var_t4__blk555_dn7 = assign18230_e25748_d_n7;
        locals.var_t4__blk555_dn10 = assign18230_e25748_d_n10;
        locals.var_t4__blk555_dn11 = assign18230_e25748_d_n11;
        locals.var_t4__blk555_dn12 = assign18230_e25748_d_n12;
        locals.var_t4__blk555_dn17 = assign18230_e25748_d_n17;
        locals.var_t4__blk555_rv = 0.0;

        let assign18240_e25752: f64 = (10.0 * 2.220446049250313e-16);
        let assign18240_e25753: f64 = (1.0 - assign18240_e25752);
        let assign18240_e25760: f64 = (10.0 * 2.220446049250313e-16);
        let assign18240_e25761: f64 = (1.0 + assign18240_e25760);
        let assign18240_e25763: f64 = if ((assign18240_e25753 <= p.p113) && (p.p113 <= assign18240_e25761)) { 1.0 } else { 0.0 };
        locals.var_guard561 = assign18240_e25763;
        locals.var_guard561_rv = 0.0;

        let (assign18250_e25771, assign18250_e25771_d_n0, assign18250_e25771_d_n2, assign18250_e25771_d_n6, assign18250_e25771_d_n7, assign18250_e25771_d_n10, assign18250_e25771_d_n11, assign18250_e25771_d_n12, assign18250_e25771_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard561 != 0.0)) {
        let assign18250_e25769: f64 = (1.0 / locals.var_t4__blk555);
        (assign18250_e25769, (-(locals.var_t4__blk555_dn0 / (locals.var_t4__blk555 * locals.var_t4__blk555))), (-(locals.var_t4__blk555_dn2 / (locals.var_t4__blk555 * locals.var_t4__blk555))), (-(locals.var_t4__blk555_dn6 / (locals.var_t4__blk555 * locals.var_t4__blk555))), (-(locals.var_t4__blk555_dn7 / (locals.var_t4__blk555 * locals.var_t4__blk555))), (-(locals.var_t4__blk555_dn10 / (locals.var_t4__blk555 * locals.var_t4__blk555))), (-(locals.var_t4__blk555_dn11 / (locals.var_t4__blk555 * locals.var_t4__blk555))), (-(locals.var_t4__blk555_dn12 / (locals.var_t4__blk555 * locals.var_t4__blk555))), (-(locals.var_t4__blk555_dn17 / (locals.var_t4__blk555 * locals.var_t4__blk555))),)
    } else {
        (locals.var_t5__blk556, locals.var_t5__blk556_dn0, locals.var_t5__blk556_dn2, locals.var_t5__blk556_dn6, locals.var_t5__blk556_dn7, locals.var_t5__blk556_dn10, locals.var_t5__blk556_dn11, locals.var_t5__blk556_dn12, locals.var_t5__blk556_dn17,)
    }
};
        locals.var_t5__blk556 = assign18250_e25771;
        locals.var_t5__blk556_dn0 = assign18250_e25771_d_n0;
        locals.var_t5__blk556_dn2 = assign18250_e25771_d_n2;
        locals.var_t5__blk556_dn6 = assign18250_e25771_d_n6;
        locals.var_t5__blk556_dn7 = assign18250_e25771_d_n7;
        locals.var_t5__blk556_dn10 = assign18250_e25771_d_n10;
        locals.var_t5__blk556_dn11 = assign18250_e25771_d_n11;
        locals.var_t5__blk556_dn12 = assign18250_e25771_d_n12;
        locals.var_t5__blk556_dn17 = assign18250_e25771_d_n17;
        locals.var_t5__blk556_rv = 0.0;

        let assign18260_e25775: f64 = (10.0 * 2.220446049250313e-16);
        let assign18260_e25776: f64 = (2.0 - assign18260_e25775);
        let assign18260_e25783: f64 = (10.0 * 2.220446049250313e-16);
        let assign18260_e25784: f64 = (2.0 + assign18260_e25783);
        let assign18260_e25786: f64 = if ((assign18260_e25776 <= p.p113) && (p.p113 <= assign18260_e25784)) { 1.0 } else { 0.0 };
        locals.var_guard562 = assign18260_e25786;
        locals.var_guard562_rv = 0.0;

        let (assign18270_e25798, assign18270_e25798_d_n0, assign18270_e25798_d_n2, assign18270_e25798_d_n6, assign18270_e25798_d_n7, assign18270_e25798_d_n10, assign18270_e25798_d_n11, assign18270_e25798_d_n12, assign18270_e25798_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard561 == 0.0)) && (locals.var_guard562 != 0.0)) {
        let assign18270_e25795: f64 = (locals.var_t4__blk555).sqrt();
        let assign18270_e25796: f64 = (1.0 / assign18270_e25795);
        (assign18270_e25796, (-((locals.var_t4__blk555_dn0 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))), (-((locals.var_t4__blk555_dn2 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))), (-((locals.var_t4__blk555_dn6 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))), (-((locals.var_t4__blk555_dn7 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))), (-((locals.var_t4__blk555_dn10 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))), (-((locals.var_t4__blk555_dn11 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))), (-((locals.var_t4__blk555_dn12 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))), (-((locals.var_t4__blk555_dn17 / (2.0 * assign18270_e25795)) / (assign18270_e25795 * assign18270_e25795))),)
    } else {
        (locals.var_t5__blk556, locals.var_t5__blk556_dn0, locals.var_t5__blk556_dn2, locals.var_t5__blk556_dn6, locals.var_t5__blk556_dn7, locals.var_t5__blk556_dn10, locals.var_t5__blk556_dn11, locals.var_t5__blk556_dn12, locals.var_t5__blk556_dn17,)
    }
};
        locals.var_t5__blk556 = assign18270_e25798;
        locals.var_t5__blk556_dn0 = assign18270_e25798_d_n0;
        locals.var_t5__blk556_dn2 = assign18270_e25798_d_n2;
        locals.var_t5__blk556_dn6 = assign18270_e25798_d_n6;
        locals.var_t5__blk556_dn7 = assign18270_e25798_d_n7;
        locals.var_t5__blk556_dn10 = assign18270_e25798_d_n10;
        locals.var_t5__blk556_dn11 = assign18270_e25798_d_n11;
        locals.var_t5__blk556_dn12 = assign18270_e25798_d_n12;
        locals.var_t5__blk556_dn17 = assign18270_e25798_d_n17;
        locals.var_t5__blk556_rv = 0.0;

        let (assign18280_e25815, assign18280_e25815_d_n0, assign18280_e25815_d_n2, assign18280_e25815_d_n6, assign18280_e25815_d_n7, assign18280_e25815_d_n10, assign18280_e25815_d_n11, assign18280_e25815_d_n12, assign18280_e25815_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard561 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign18280_e25808: f64 = (-1.0);
        let assign18280_e25810: f64 = (assign18280_e25808 / p.p113);
        let assign18280_e25812: f64 = (assign18280_e25810 - 1.0);
        let assign18280_e25813: f64 = (locals.var_t4__blk555).powf(assign18280_e25812);
        (assign18280_e25813, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn0)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn0 / locals.var_t4__blk555))) }, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn2)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn2 / locals.var_t4__blk555))) }, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn6)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn6 / locals.var_t4__blk555))) }, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn7)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn7 / locals.var_t4__blk555))) }, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn10)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn10 / locals.var_t4__blk555))) }, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn11)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn11 / locals.var_t4__blk555))) }, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn12)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn12 / locals.var_t4__blk555))) }, if 0.0 == 0.0 && ((assign18280_e25812) as f64).is_finite() && ((assign18280_e25812) as f64).fract() == 0.0 { if assign18280_e25812 == 0.0 { 0.0 } else { (assign18280_e25812 * ((locals.var_t4__blk555).powf(assign18280_e25812 - 1.0) * locals.var_t4__blk555_dn17)) } } else { (assign18280_e25813 * (assign18280_e25812 * (locals.var_t4__blk555_dn17 / locals.var_t4__blk555))) },)
    } else {
        (locals.var_t6__blk557, locals.var_t6__blk557_dn0, locals.var_t6__blk557_dn2, locals.var_t6__blk557_dn6, locals.var_t6__blk557_dn7, locals.var_t6__blk557_dn10, locals.var_t6__blk557_dn11, locals.var_t6__blk557_dn12, locals.var_t6__blk557_dn17,)
    }
};
        locals.var_t6__blk557 = assign18280_e25815;
        locals.var_t6__blk557_dn0 = assign18280_e25815_d_n0;
        locals.var_t6__blk557_dn2 = assign18280_e25815_d_n2;
        locals.var_t6__blk557_dn6 = assign18280_e25815_d_n6;
        locals.var_t6__blk557_dn7 = assign18280_e25815_d_n7;
        locals.var_t6__blk557_dn10 = assign18280_e25815_d_n10;
        locals.var_t6__blk557_dn11 = assign18280_e25815_d_n11;
        locals.var_t6__blk557_dn12 = assign18280_e25815_d_n12;
        locals.var_t6__blk557_dn17 = assign18280_e25815_d_n17;
        locals.var_t6__blk557_rv = 0.0;

        let (assign18290_e25827, assign18290_e25827_d_n0, assign18290_e25827_d_n2, assign18290_e25827_d_n6, assign18290_e25827_d_n7, assign18290_e25827_d_n10, assign18290_e25827_d_n11, assign18290_e25827_d_n12, assign18290_e25827_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard561 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign18290_e25825: f64 = (locals.var_t4__blk555 * locals.var_t6__blk557);
        (assign18290_e25825, ((locals.var_t4__blk555_dn0 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn0)), ((locals.var_t4__blk555_dn2 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn2)), ((locals.var_t4__blk555_dn6 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn6)), ((locals.var_t4__blk555_dn7 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn7)), ((locals.var_t4__blk555_dn10 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn10)), ((locals.var_t4__blk555_dn11 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn11)), ((locals.var_t4__blk555_dn12 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn12)), ((locals.var_t4__blk555_dn17 * locals.var_t6__blk557) + (locals.var_t4__blk555 * locals.var_t6__blk557_dn17)),)
    } else {
        (locals.var_t5__blk556, locals.var_t5__blk556_dn0, locals.var_t5__blk556_dn2, locals.var_t5__blk556_dn6, locals.var_t5__blk556_dn7, locals.var_t5__blk556_dn10, locals.var_t5__blk556_dn11, locals.var_t5__blk556_dn12, locals.var_t5__blk556_dn17,)
    }
};
        locals.var_t5__blk556 = assign18290_e25827;
        locals.var_t5__blk556_dn0 = assign18290_e25827_d_n0;
        locals.var_t5__blk556_dn2 = assign18290_e25827_d_n2;
        locals.var_t5__blk556_dn6 = assign18290_e25827_d_n6;
        locals.var_t5__blk556_dn7 = assign18290_e25827_d_n7;
        locals.var_t5__blk556_dn10 = assign18290_e25827_d_n10;
        locals.var_t5__blk556_dn11 = assign18290_e25827_d_n11;
        locals.var_t5__blk556_dn12 = assign18290_e25827_d_n12;
        locals.var_t5__blk556_dn17 = assign18290_e25827_d_n17;
        locals.var_t5__blk556_rv = 0.0;

        let (assign18300_e25833, assign18300_e25833_d_n0, assign18300_e25833_d_n2, assign18300_e25833_d_n6, assign18300_e25833_d_n7, assign18300_e25833_d_n10, assign18300_e25833_d_n11, assign18300_e25833_d_n12, assign18300_e25833_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18300_e25831: f64 = (locals.var_muun * locals.var_t5__blk556);
        (assign18300_e25831, ((locals.var_muun_dn0 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn0)), ((locals.var_muun_dn2 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn2)), ((locals.var_muun_dn6 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn6)), ((locals.var_muun_dn7 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn7)), ((locals.var_muun_dn10 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn10)), ((locals.var_muun_dn11 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn11)), ((locals.var_muun_dn12 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn12)), ((locals.var_muun_dn17 * locals.var_t5__blk556) + (locals.var_muun * locals.var_t5__blk556_dn17)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn12, locals.var_mu_dn17,)
    }
};
        locals.var_mu = assign18300_e25833;
        locals.var_mu_dn0 = assign18300_e25833_d_n0;
        locals.var_mu_dn2 = assign18300_e25833_d_n2;
        locals.var_mu_dn6 = assign18300_e25833_d_n6;
        locals.var_mu_dn7 = assign18300_e25833_d_n7;
        locals.var_mu_dn10 = assign18300_e25833_d_n10;
        locals.var_mu_dn11 = assign18300_e25833_d_n11;
        locals.var_mu_dn12 = assign18300_e25833_d_n12;
        locals.var_mu_dn17 = assign18300_e25833_d_n17;
        locals.var_mu_rv = 0.0;

        let (assign18310_e25843, assign18310_e25843_d_n0, assign18310_e25843_d_n2, assign18310_e25843_d_n6, assign18310_e25843_d_n7, assign18310_e25843_d_n10, assign18310_e25843_d_n11, assign18310_e25843_d_n12, assign18310_e25843_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18310_e25837: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign18310_e25840: f64 = (locals.var_leff - locals.var_lred);
        let assign18310_e25841: f64 = (assign18310_e25837 / assign18310_e25840);
        (assign18310_e25841, (-((assign18310_e25837 * (-locals.var_lred_dn0)) / (assign18310_e25840 * assign18310_e25840))), (-((assign18310_e25837 * (-locals.var_lred_dn2)) / (assign18310_e25840 * assign18310_e25840))), (-((assign18310_e25837 * (-locals.var_lred_dn6)) / (assign18310_e25840 * assign18310_e25840))), (-((assign18310_e25837 * (-locals.var_lred_dn7)) / (assign18310_e25840 * assign18310_e25840))), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * assign18310_e25840) - (assign18310_e25837 * (-locals.var_lred_dn10))) / (assign18310_e25840 * assign18310_e25840)), (-((assign18310_e25837 * (-locals.var_lred_dn11)) / (assign18310_e25840 * assign18310_e25840))), (-((assign18310_e25837 * (-locals.var_lred_dn12)) / (assign18310_e25840 * assign18310_e25840))), (-((assign18310_e25837 * (-locals.var_lred_dn17)) / (assign18310_e25840 * assign18310_e25840))),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn12, locals.var_betawl_dn17,)
    }
};
        locals.var_betawl = assign18310_e25843;
        locals.var_betawl_dn0 = assign18310_e25843_d_n0;
        locals.var_betawl_dn2 = assign18310_e25843_d_n2;
        locals.var_betawl_dn6 = assign18310_e25843_d_n6;
        locals.var_betawl_dn7 = assign18310_e25843_d_n7;
        locals.var_betawl_dn10 = assign18310_e25843_d_n10;
        locals.var_betawl_dn11 = assign18310_e25843_d_n11;
        locals.var_betawl_dn12 = assign18310_e25843_d_n12;
        locals.var_betawl_dn17 = assign18310_e25843_d_n17;
        locals.var_betawl_rv = 0.0;

        let (assign18320_e25851, assign18320_e25851_d_n0, assign18320_e25851_d_n2, assign18320_e25851_d_n6, assign18320_e25851_d_n7, assign18320_e25851_d_n10, assign18320_e25851_d_n11, assign18320_e25851_d_n12, assign18320_e25851_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18320_e25847: f64 = (locals.var_betawl * locals.var_idd);
        let assign18320_e25849: f64 = (assign18320_e25847 * locals.var_mu);
        (assign18320_e25849, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn12)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn17)) * locals.var_mu) + (assign18320_e25847 * locals.var_mu_dn17)),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn12, locals.var_ids0_dn17,)
    }
};
        locals.var_ids0 = assign18320_e25851;
        locals.var_ids0_dn0 = assign18320_e25851_d_n0;
        locals.var_ids0_dn2 = assign18320_e25851_d_n2;
        locals.var_ids0_dn6 = assign18320_e25851_d_n6;
        locals.var_ids0_dn7 = assign18320_e25851_d_n7;
        locals.var_ids0_dn10 = assign18320_e25851_d_n10;
        locals.var_ids0_dn11 = assign18320_e25851_d_n11;
        locals.var_ids0_dn12 = assign18320_e25851_d_n12;
        locals.var_ids0_dn17 = assign18320_e25851_d_n17;
        locals.var_ids0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18330_e25855, assign18330_e25855_d_n0, assign18330_e25855_d_n2, assign18330_e25855_d_n6, assign18330_e25855_d_n7, assign18330_e25855_d_n10, assign18330_e25855_d_n11, assign18330_e25855_d_n12, assign18330_e25855_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18330_e25855;
        locals.var_idspt_dn0 = assign18330_e25855_d_n0;
        locals.var_idspt_dn2 = assign18330_e25855_d_n2;
        locals.var_idspt_dn6 = assign18330_e25855_d_n6;
        locals.var_idspt_dn7 = assign18330_e25855_d_n7;
        locals.var_idspt_dn10 = assign18330_e25855_d_n10;
        locals.var_idspt_dn11 = assign18330_e25855_d_n11;
        locals.var_idspt_dn12 = assign18330_e25855_d_n12;
        locals.var_idspt_dn17 = assign18330_e25855_d_n17;
        locals.var_idspt_rv = 0.0;

        let assign18340_e25862: f64 = if ((p.p281 > 0.0) && (p.p244 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard572 = assign18340_e25862;
        locals.var_guard572_rv = 0.0;

        let (assign18350_e25872, assign18350_e25872_d_n0, assign18350_e25872_d_n2, assign18350_e25872_d_n6, assign18350_e25872_d_n7, assign18350_e25872_d_n10, assign18350_e25872_d_n11, assign18350_e25872_d_n12, assign18350_e25872_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18350_e25869: f64 = (locals.var_vds - locals.var_pds);
        let assign18350_e25870: f64 = (0.5 * assign18350_e25869);
        (assign18350_e25870, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn12 - locals.var_pds_dn12)), (0.5 * (locals.var_vds_dn17 - locals.var_pds_dn17)),)
    } else {
        (locals.var_t1__blk563, locals.var_t1__blk563_dn0, locals.var_t1__blk563_dn2, locals.var_t1__blk563_dn6, locals.var_t1__blk563_dn7, locals.var_t1__blk563_dn10, locals.var_t1__blk563_dn11, locals.var_t1__blk563_dn12, locals.var_t1__blk563_dn17,)
    }
};
        locals.var_t1__blk563 = assign18350_e25872;
        locals.var_t1__blk563_dn0 = assign18350_e25872_d_n0;
        locals.var_t1__blk563_dn2 = assign18350_e25872_d_n2;
        locals.var_t1__blk563_dn6 = assign18350_e25872_d_n6;
        locals.var_t1__blk563_dn7 = assign18350_e25872_d_n7;
        locals.var_t1__blk563_dn10 = assign18350_e25872_d_n10;
        locals.var_t1__blk563_dn11 = assign18350_e25872_d_n11;
        locals.var_t1__blk563_dn12 = assign18350_e25872_d_n12;
        locals.var_t1__blk563_dn17 = assign18350_e25872_d_n17;
        locals.var_t1__blk563_rv = 0.0;

        let (assign18360_e25882, assign18360_e25882_d_n0, assign18360_e25882_d_n2, assign18360_e25882_d_n6, assign18360_e25882_d_n7, assign18360_e25882_d_n10, assign18360_e25882_d_n11, assign18360_e25882_d_n12, assign18360_e25882_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18360_e25878: f64 = (2.0 * locals.var_t1__blk563);
        let assign18360_e25880: f64 = (assign18360_e25878 / 0.01);
        (assign18360_e25880, ((2.0 * locals.var_t1__blk563_dn0) / 0.01), ((2.0 * locals.var_t1__blk563_dn2) / 0.01), ((2.0 * locals.var_t1__blk563_dn6) / 0.01), ((2.0 * locals.var_t1__blk563_dn7) / 0.01), ((2.0 * locals.var_t1__blk563_dn10) / 0.01), ((2.0 * locals.var_t1__blk563_dn11) / 0.01), ((2.0 * locals.var_t1__blk563_dn12) / 0.01), ((2.0 * locals.var_t1__blk563_dn17) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18360_e25882;
        locals.var_tmf1_dn0 = assign18360_e25882_d_n0;
        locals.var_tmf1_dn2 = assign18360_e25882_d_n2;
        locals.var_tmf1_dn6 = assign18360_e25882_d_n6;
        locals.var_tmf1_dn7 = assign18360_e25882_d_n7;
        locals.var_tmf1_dn10 = assign18360_e25882_d_n10;
        locals.var_tmf1_dn11 = assign18360_e25882_d_n11;
        locals.var_tmf1_dn12 = assign18360_e25882_d_n12;
        locals.var_tmf1_dn17 = assign18360_e25882_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18370_e25924, assign18370_e25924_d_n0, assign18370_e25924_d_n2, assign18370_e25924_d_n6, assign18370_e25924_d_n7, assign18370_e25924_d_n10, assign18370_e25924_d_n11, assign18370_e25924_d_n12, assign18370_e25924_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18370_e25890: f64 = (1.0 / 2.0);
        let assign18370_e25894: f64 = (1.0 / 6.0);
        let assign18370_e25898: f64 = (1.0 / 24.0);
        let assign18370_e25902: f64 = (1.0 / 120.0);
        let assign18370_e25906: f64 = (1.0 / 720.0);
        let assign18370_e25910: f64 = (1.0 / 5040.0);
        let assign18370_e25911: f64 = (locals.var_tmf1 * assign18370_e25910);
        let assign18370_e25912: f64 = (assign18370_e25906 + assign18370_e25911);
        let assign18370_e25913: f64 = (locals.var_tmf1 * assign18370_e25912);
        let assign18370_e25914: f64 = (assign18370_e25902 + assign18370_e25913);
        let assign18370_e25915: f64 = (locals.var_tmf1 * assign18370_e25914);
        let assign18370_e25916: f64 = (assign18370_e25898 + assign18370_e25915);
        let assign18370_e25917: f64 = (locals.var_tmf1 * assign18370_e25916);
        let assign18370_e25918: f64 = (assign18370_e25894 + assign18370_e25917);
        let assign18370_e25919: f64 = (locals.var_tmf1 * assign18370_e25918);
        let assign18370_e25920: f64 = (assign18370_e25890 + assign18370_e25919);
        let assign18370_e25921: f64 = (locals.var_tmf1 * assign18370_e25920);
        let assign18370_e25922: f64 = (1.0 + assign18370_e25921);
        (assign18370_e25922, ((locals.var_tmf1_dn0 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign18370_e25910))))))))))), ((locals.var_tmf1_dn2 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign18370_e25910))))))))))), ((locals.var_tmf1_dn6 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign18370_e25910))))))))))), ((locals.var_tmf1_dn7 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign18370_e25910))))))))))), ((locals.var_tmf1_dn10 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign18370_e25910))))))))))), ((locals.var_tmf1_dn11 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign18370_e25910))))))))))), ((locals.var_tmf1_dn12 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign18370_e25910))))))))))), ((locals.var_tmf1_dn17 * assign18370_e25920) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18370_e25918) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18370_e25916) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18370_e25914) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18370_e25912) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign18370_e25910))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign18370_e25924;
        locals.var_tmf2_dn0 = assign18370_e25924_d_n0;
        locals.var_tmf2_dn2 = assign18370_e25924_d_n2;
        locals.var_tmf2_dn6 = assign18370_e25924_d_n6;
        locals.var_tmf2_dn7 = assign18370_e25924_d_n7;
        locals.var_tmf2_dn10 = assign18370_e25924_d_n10;
        locals.var_tmf2_dn11 = assign18370_e25924_d_n11;
        locals.var_tmf2_dn12 = assign18370_e25924_d_n12;
        locals.var_tmf2_dn17 = assign18370_e25924_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign18380_e25932, assign18380_e25932_d_n0, assign18380_e25932_d_n2, assign18380_e25932_d_n6, assign18380_e25932_d_n7, assign18380_e25932_d_n10, assign18380_e25932_d_n11, assign18380_e25932_d_n12, assign18380_e25932_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18380_e25930: f64 = (0.01 / locals.var_tmf2);
        (assign18380_e25930, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6__blk569, locals.var_t6__blk569_dn0, locals.var_t6__blk569_dn2, locals.var_t6__blk569_dn6, locals.var_t6__blk569_dn7, locals.var_t6__blk569_dn10, locals.var_t6__blk569_dn11, locals.var_t6__blk569_dn12, locals.var_t6__blk569_dn17,)
    }
};
        locals.var_t6__blk569 = assign18380_e25932;
        locals.var_t6__blk569_dn0 = assign18380_e25932_d_n0;
        locals.var_t6__blk569_dn2 = assign18380_e25932_d_n2;
        locals.var_t6__blk569_dn6 = assign18380_e25932_d_n6;
        locals.var_t6__blk569_dn7 = assign18380_e25932_d_n7;
        locals.var_t6__blk569_dn10 = assign18380_e25932_d_n10;
        locals.var_t6__blk569_dn11 = assign18380_e25932_d_n11;
        locals.var_t6__blk569_dn12 = assign18380_e25932_d_n12;
        locals.var_t6__blk569_dn17 = assign18380_e25932_d_n17;
        locals.var_t6__blk569_rv = 0.0;

        let (assign18390_e25942, assign18390_e25942_d_n0, assign18390_e25942_d_n2, assign18390_e25942_d_n6, assign18390_e25942_d_n7, assign18390_e25942_d_n10, assign18390_e25942_d_n11, assign18390_e25942_d_n12, assign18390_e25942_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18390_e25939: f64 = (locals.var_ps0 + locals.var_t6__blk569);
        let assign18390_e25940: f64 = (1.1 - assign18390_e25939);
        (assign18390_e25940, (-(locals.var_ps0_dn0 + locals.var_t6__blk569_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6__blk569_dn2)), (-(locals.var_ps0_dn6 + locals.var_t6__blk569_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6__blk569_dn7)), (-(locals.var_ps0_dn10 + locals.var_t6__blk569_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6__blk569_dn11)), (-(locals.var_ps0_dn12 + locals.var_t6__blk569_dn12)), (-(locals.var_ps0_dn17 + locals.var_t6__blk569_dn17)),)
    } else {
        (locals.var_t1__blk563, locals.var_t1__blk563_dn0, locals.var_t1__blk563_dn2, locals.var_t1__blk563_dn6, locals.var_t1__blk563_dn7, locals.var_t1__blk563_dn10, locals.var_t1__blk563_dn11, locals.var_t1__blk563_dn12, locals.var_t1__blk563_dn17,)
    }
};
        locals.var_t1__blk563 = assign18390_e25942;
        locals.var_t1__blk563_dn0 = assign18390_e25942_d_n0;
        locals.var_t1__blk563_dn2 = assign18390_e25942_d_n2;
        locals.var_t1__blk563_dn6 = assign18390_e25942_d_n6;
        locals.var_t1__blk563_dn7 = assign18390_e25942_d_n7;
        locals.var_t1__blk563_dn10 = assign18390_e25942_d_n10;
        locals.var_t1__blk563_dn11 = assign18390_e25942_d_n11;
        locals.var_t1__blk563_dn12 = assign18390_e25942_d_n12;
        locals.var_t1__blk563_dn17 = assign18390_e25942_d_n17;
        locals.var_t1__blk563_rv = 0.0;

        let (assign18400_e25957, assign18400_e25957_d_n0, assign18400_e25957_d_n2, assign18400_e25957_d_n6, assign18400_e25957_d_n7, assign18400_e25957_d_n10, assign18400_e25957_d_n11, assign18400_e25957_d_n12, assign18400_e25957_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18400_e25948: f64 = (locals.var_t1__blk563 * locals.var_t1__blk563);
        let assign18400_e25951: f64 = (4.0 * 0.05);
        let assign18400_e25953: f64 = (assign18400_e25951 * 0.05);
        let assign18400_e25954: f64 = (assign18400_e25948 + assign18400_e25953);
        let assign18400_e25955: f64 = (assign18400_e25954).sqrt();
        (assign18400_e25955, (((locals.var_t1__blk563_dn0 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn0)) / (2.0 * assign18400_e25955)), (((locals.var_t1__blk563_dn2 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn2)) / (2.0 * assign18400_e25955)), (((locals.var_t1__blk563_dn6 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn6)) / (2.0 * assign18400_e25955)), (((locals.var_t1__blk563_dn7 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn7)) / (2.0 * assign18400_e25955)), (((locals.var_t1__blk563_dn10 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn10)) / (2.0 * assign18400_e25955)), (((locals.var_t1__blk563_dn11 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn11)) / (2.0 * assign18400_e25955)), (((locals.var_t1__blk563_dn12 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn12)) / (2.0 * assign18400_e25955)), (((locals.var_t1__blk563_dn17 * locals.var_t1__blk563) + (locals.var_t1__blk563 * locals.var_t1__blk563_dn17)) / (2.0 * assign18400_e25955)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18400_e25957;
        locals.var_tmf1_dn0 = assign18400_e25957_d_n0;
        locals.var_tmf1_dn2 = assign18400_e25957_d_n2;
        locals.var_tmf1_dn6 = assign18400_e25957_d_n6;
        locals.var_tmf1_dn7 = assign18400_e25957_d_n7;
        locals.var_tmf1_dn10 = assign18400_e25957_d_n10;
        locals.var_tmf1_dn11 = assign18400_e25957_d_n11;
        locals.var_tmf1_dn12 = assign18400_e25957_d_n12;
        locals.var_tmf1_dn17 = assign18400_e25957_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18410_e25971, assign18410_e25971_d_n0, assign18410_e25971_d_n2, assign18410_e25971_d_n6, assign18410_e25971_d_n7, assign18410_e25971_d_n10, assign18410_e25971_d_n11, assign18410_e25971_d_n12, assign18410_e25971_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18410_e25964: f64 = (locals.var_t1__blk563 + locals.var_tmf1);
        let assign18410_e25965: f64 = (0.5 * assign18410_e25964);
        let assign18410_e25968: f64 = (1e-10 * 0.05);
        let assign18410_e25969: f64 = (assign18410_e25965 + assign18410_e25968);
        (assign18410_e25969, (0.5 * (locals.var_t1__blk563_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1__blk563_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1__blk563_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1__blk563_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1__blk563_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1__blk563_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1__blk563_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1__blk563_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk571, locals.var_t2__blk571_dn0, locals.var_t2__blk571_dn2, locals.var_t2__blk571_dn6, locals.var_t2__blk571_dn7, locals.var_t2__blk571_dn10, locals.var_t2__blk571_dn11, locals.var_t2__blk571_dn12, locals.var_t2__blk571_dn17,)
    }
};
        locals.var_t2__blk571 = assign18410_e25971;
        locals.var_t2__blk571_dn0 = assign18410_e25971_d_n0;
        locals.var_t2__blk571_dn2 = assign18410_e25971_d_n2;
        locals.var_t2__blk571_dn6 = assign18410_e25971_d_n6;
        locals.var_t2__blk571_dn7 = assign18410_e25971_d_n7;
        locals.var_t2__blk571_dn10 = assign18410_e25971_d_n10;
        locals.var_t2__blk571_dn11 = assign18410_e25971_d_n11;
        locals.var_t2__blk571_dn12 = assign18410_e25971_d_n12;
        locals.var_t2__blk571_dn17 = assign18410_e25971_d_n17;
        locals.var_t2__blk571_rv = 0.0;

        let assign18420_e25974: f64 = if locals.var_t2__blk571 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign18420_e25974;
        locals.var_guard573_rv = 0.0;

        let (assign18430_e25982, assign18430_e25982_d_n0, assign18430_e25982_d_n2, assign18430_e25982_d_n6, assign18430_e25982_d_n7, assign18430_e25982_d_n10, assign18430_e25982_d_n11, assign18430_e25982_d_n12, assign18430_e25982_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk571, locals.var_t2__blk571_dn0, locals.var_t2__blk571_dn2, locals.var_t2__blk571_dn6, locals.var_t2__blk571_dn7, locals.var_t2__blk571_dn10, locals.var_t2__blk571_dn11, locals.var_t2__blk571_dn12, locals.var_t2__blk571_dn17,)
    }
};
        locals.var_t2__blk571 = assign18430_e25982;
        locals.var_t2__blk571_dn0 = assign18430_e25982_d_n0;
        locals.var_t2__blk571_dn2 = assign18430_e25982_d_n2;
        locals.var_t2__blk571_dn6 = assign18430_e25982_d_n6;
        locals.var_t2__blk571_dn7 = assign18430_e25982_d_n7;
        locals.var_t2__blk571_dn10 = assign18430_e25982_d_n10;
        locals.var_t2__blk571_dn11 = assign18430_e25982_d_n11;
        locals.var_t2__blk571_dn12 = assign18430_e25982_d_n12;
        locals.var_t2__blk571_dn17 = assign18430_e25982_d_n17;
        locals.var_t2__blk571_rv = 0.0;

        let (assign18440_e25990, assign18440_e25990_d_n0, assign18440_e25990_d_n2, assign18440_e25990_d_n6, assign18440_e25990_d_n7, assign18440_e25990_d_n10, assign18440_e25990_d_n11, assign18440_e25990_d_n12, assign18440_e25990_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18440_e25988: f64 = (locals.var_beta * locals.var_ptl0);
        (assign18440_e25988, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_ptl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk564, locals.var_t0__blk564_dn0, locals.var_t0__blk564_dn2, locals.var_t0__blk564_dn6, locals.var_t0__blk564_dn7, locals.var_t0__blk564_dn10, locals.var_t0__blk564_dn11, locals.var_t0__blk564_dn12, locals.var_t0__blk564_dn17,)
    }
};
        locals.var_t0__blk564 = assign18440_e25990;
        locals.var_t0__blk564_dn0 = assign18440_e25990_d_n0;
        locals.var_t0__blk564_dn2 = assign18440_e25990_d_n2;
        locals.var_t0__blk564_dn6 = assign18440_e25990_d_n6;
        locals.var_t0__blk564_dn7 = assign18440_e25990_d_n7;
        locals.var_t0__blk564_dn10 = assign18440_e25990_d_n10;
        locals.var_t0__blk564_dn11 = assign18440_e25990_d_n11;
        locals.var_t0__blk564_dn12 = assign18440_e25990_d_n12;
        locals.var_t0__blk564_dn17 = assign18440_e25990_d_n17;
        locals.var_t0__blk564_rv = 0.0;

        let (assign18450_e25998, assign18450_e25998_d_n0, assign18450_e25998_d_n2, assign18450_e25998_d_n6, assign18450_e25998_d_n7, assign18450_e25998_d_n10, assign18450_e25998_d_n11, assign18450_e25998_d_n12, assign18450_e25998_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18450_e25996: f64 = (locals.var_c_fox * locals.var_t0__blk564);
        (assign18450_e25996, ((locals.var_c_fox_dn0 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn0)), ((locals.var_c_fox_dn2 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn2)), ((locals.var_c_fox_dn6 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn6)), ((locals.var_c_fox_dn7 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn7)), ((locals.var_c_fox_dn10 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn10)), ((locals.var_c_fox_dn11 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn11)), ((locals.var_c_fox_dn12 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn12)), ((locals.var_c_fox_dn17 * locals.var_t0__blk564) + (locals.var_c_fox * locals.var_t0__blk564_dn17)),)
    } else {
        (locals.var_t3__blk565, locals.var_t3__blk565_dn0, locals.var_t3__blk565_dn2, locals.var_t3__blk565_dn6, locals.var_t3__blk565_dn7, locals.var_t3__blk565_dn10, locals.var_t3__blk565_dn11, locals.var_t3__blk565_dn12, locals.var_t3__blk565_dn17,)
    }
};
        locals.var_t3__blk565 = assign18450_e25998;
        locals.var_t3__blk565_dn0 = assign18450_e25998_d_n0;
        locals.var_t3__blk565_dn2 = assign18450_e25998_d_n2;
        locals.var_t3__blk565_dn6 = assign18450_e25998_d_n6;
        locals.var_t3__blk565_dn7 = assign18450_e25998_d_n7;
        locals.var_t3__blk565_dn10 = assign18450_e25998_d_n10;
        locals.var_t3__blk565_dn11 = assign18450_e25998_d_n11;
        locals.var_t3__blk565_dn12 = assign18450_e25998_d_n12;
        locals.var_t3__blk565_dn17 = assign18450_e25998_d_n17;
        locals.var_t3__blk565_rv = 0.0;

        let (assign18460_e26006, assign18460_e26006_d_n0, assign18460_e26006_d_n2, assign18460_e26006_d_n6, assign18460_e26006_d_n7, assign18460_e26006_d_n10, assign18460_e26006_d_n11, assign18460_e26006_d_n12, assign18460_e26006_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18460_e26004: f64 = (locals.var_t2__blk571).powf(p.p245);
        (assign18460_e26004, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn0)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn0 / locals.var_t2__blk571))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn2)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn2 / locals.var_t2__blk571))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn6)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn6 / locals.var_t2__blk571))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn7)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn7 / locals.var_t2__blk571))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn10)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn10 / locals.var_t2__blk571))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn11)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn11 / locals.var_t2__blk571))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn12)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn12 / locals.var_t2__blk571))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk571).powf(p.p245 - 1.0) * locals.var_t2__blk571_dn17)) } } else { (assign18460_e26004 * (p.p245 * (locals.var_t2__blk571_dn17 / locals.var_t2__blk571))) },)
    } else {
        (locals.var_t0__blk564, locals.var_t0__blk564_dn0, locals.var_t0__blk564_dn2, locals.var_t0__blk564_dn6, locals.var_t0__blk564_dn7, locals.var_t0__blk564_dn10, locals.var_t0__blk564_dn11, locals.var_t0__blk564_dn12, locals.var_t0__blk564_dn17,)
    }
};
        locals.var_t0__blk564 = assign18460_e26006;
        locals.var_t0__blk564_dn0 = assign18460_e26006_d_n0;
        locals.var_t0__blk564_dn2 = assign18460_e26006_d_n2;
        locals.var_t0__blk564_dn6 = assign18460_e26006_d_n6;
        locals.var_t0__blk564_dn7 = assign18460_e26006_d_n7;
        locals.var_t0__blk564_dn10 = assign18460_e26006_d_n10;
        locals.var_t0__blk564_dn11 = assign18460_e26006_d_n11;
        locals.var_t0__blk564_dn12 = assign18460_e26006_d_n12;
        locals.var_t0__blk564_dn17 = assign18460_e26006_d_n17;
        locals.var_t0__blk564_rv = 0.0;

        let (assign18470_e26014, assign18470_e26014_d_n0, assign18470_e26014_d_n2, assign18470_e26014_d_n6, assign18470_e26014_d_n7, assign18470_e26014_d_n10, assign18470_e26014_d_n11, assign18470_e26014_d_n12, assign18470_e26014_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18470_e26012: f64 = (locals.var_t3__blk565 * locals.var_t0__blk564);
        (assign18470_e26012, ((locals.var_t3__blk565_dn0 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn0)), ((locals.var_t3__blk565_dn2 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn2)), ((locals.var_t3__blk565_dn6 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn6)), ((locals.var_t3__blk565_dn7 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn7)), ((locals.var_t3__blk565_dn10 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn10)), ((locals.var_t3__blk565_dn11 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn11)), ((locals.var_t3__blk565_dn12 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn12)), ((locals.var_t3__blk565_dn17 * locals.var_t0__blk564) + (locals.var_t3__blk565 * locals.var_t0__blk564_dn17)),)
    } else {
        (locals.var_t9__blk566, locals.var_t9__blk566_dn0, locals.var_t9__blk566_dn2, locals.var_t9__blk566_dn6, locals.var_t9__blk566_dn7, locals.var_t9__blk566_dn10, locals.var_t9__blk566_dn11, locals.var_t9__blk566_dn12, locals.var_t9__blk566_dn17,)
    }
};
        locals.var_t9__blk566 = assign18470_e26014;
        locals.var_t9__blk566_dn0 = assign18470_e26014_d_n0;
        locals.var_t9__blk566_dn2 = assign18470_e26014_d_n2;
        locals.var_t9__blk566_dn6 = assign18470_e26014_d_n6;
        locals.var_t9__blk566_dn7 = assign18470_e26014_d_n7;
        locals.var_t9__blk566_dn10 = assign18470_e26014_d_n10;
        locals.var_t9__blk566_dn11 = assign18470_e26014_d_n11;
        locals.var_t9__blk566_dn12 = assign18470_e26014_d_n12;
        locals.var_t9__blk566_dn17 = assign18470_e26014_d_n17;
        locals.var_t9__blk566_rv = 0.0;

        let (assign18480_e26024, assign18480_e26024_d_n0, assign18480_e26024_d_n2, assign18480_e26024_d_n6, assign18480_e26024_d_n7, assign18480_e26024_d_n10, assign18480_e26024_d_n11, assign18480_e26024_d_n12, assign18480_e26024_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18480_e26021: f64 = (locals.var_vdsz * p.p246);
        let assign18480_e26022: f64 = (1.0 + assign18480_e26021);
        (assign18480_e26022, (locals.var_vdsz_dn0 * p.p246), (locals.var_vdsz_dn2 * p.p246), (locals.var_vdsz_dn6 * p.p246), (locals.var_vdsz_dn7 * p.p246), (locals.var_vdsz_dn10 * p.p246), (locals.var_vdsz_dn11 * p.p246), (locals.var_vdsz_dn12 * p.p246), (locals.var_vdsz_dn17 * p.p246),)
    } else {
        (locals.var_t4__blk567, locals.var_t4__blk567_dn0, locals.var_t4__blk567_dn2, locals.var_t4__blk567_dn6, locals.var_t4__blk567_dn7, locals.var_t4__blk567_dn10, locals.var_t4__blk567_dn11, locals.var_t4__blk567_dn12, locals.var_t4__blk567_dn17,)
    }
};
        locals.var_t4__blk567 = assign18480_e26024;
        locals.var_t4__blk567_dn0 = assign18480_e26024_d_n0;
        locals.var_t4__blk567_dn2 = assign18480_e26024_d_n2;
        locals.var_t4__blk567_dn6 = assign18480_e26024_d_n6;
        locals.var_t4__blk567_dn7 = assign18480_e26024_d_n7;
        locals.var_t4__blk567_dn10 = assign18480_e26024_d_n10;
        locals.var_t4__blk567_dn11 = assign18480_e26024_d_n11;
        locals.var_t4__blk567_dn12 = assign18480_e26024_d_n12;
        locals.var_t4__blk567_dn17 = assign18480_e26024_d_n17;
        locals.var_t4__blk567_rv = 0.0;

        let (assign18490_e26030, assign18490_e26030_d_n0, assign18490_e26030_d_n2, assign18490_e26030_d_n6, assign18490_e26030_d_n7, assign18490_e26030_d_n10, assign18490_e26030_d_n11, assign18490_e26030_d_n12, assign18490_e26030_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk564, locals.var_t0__blk564_dn0, locals.var_t0__blk564_dn2, locals.var_t0__blk564_dn6, locals.var_t0__blk564_dn7, locals.var_t0__blk564_dn10, locals.var_t0__blk564_dn11, locals.var_t0__blk564_dn12, locals.var_t0__blk564_dn17,)
    }
};
        locals.var_t0__blk564 = assign18490_e26030;
        locals.var_t0__blk564_dn0 = assign18490_e26030_d_n0;
        locals.var_t0__blk564_dn2 = assign18490_e26030_d_n2;
        locals.var_t0__blk564_dn6 = assign18490_e26030_d_n6;
        locals.var_t0__blk564_dn7 = assign18490_e26030_d_n7;
        locals.var_t0__blk564_dn10 = assign18490_e26030_d_n10;
        locals.var_t0__blk564_dn11 = assign18490_e26030_d_n11;
        locals.var_t0__blk564_dn12 = assign18490_e26030_d_n12;
        locals.var_t0__blk564_dn17 = assign18490_e26030_d_n17;
        locals.var_t0__blk564_rv = 0.0;

        let assign18500_e26037: f64 = if ((locals.var_subversion < 3.0) || (p.p43 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard574 = assign18500_e26037;
        locals.var_guard574_rv = 0.0;

        let (assign18510_e26049, assign18510_e26049_d_n0, assign18510_e26049_d_n2, assign18510_e26049_d_n6, assign18510_e26049_d_n7, assign18510_e26049_d_n10, assign18510_e26049_d_n11, assign18510_e26049_d_n12, assign18510_e26049_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) && (locals.var_guard574 != 0.0)) {
        let assign18510_e26045: f64 = (locals.var_ps0 + locals.var_t6__blk569);
        let assign18510_e26047: f64 = (assign18510_e26045 - locals.var_vbsz);
        (assign18510_e26047, ((locals.var_ps0_dn0 + locals.var_t6__blk569_dn0) - locals.var_vbsz_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk569_dn2) - locals.var_vbsz_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk569_dn6) - locals.var_vbsz_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk569_dn7) - locals.var_vbsz_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk569_dn10) - locals.var_vbsz_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk569_dn11) - locals.var_vbsz_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk569_dn12) - locals.var_vbsz_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk569_dn17) - locals.var_vbsz_dn17),)
    } else {
        (locals.var_t5__blk568, locals.var_t5__blk568_dn0, locals.var_t5__blk568_dn2, locals.var_t5__blk568_dn6, locals.var_t5__blk568_dn7, locals.var_t5__blk568_dn10, locals.var_t5__blk568_dn11, locals.var_t5__blk568_dn12, locals.var_t5__blk568_dn17,)
    }
};
        locals.var_t5__blk568 = assign18510_e26049;
        locals.var_t5__blk568_dn0 = assign18510_e26049_d_n0;
        locals.var_t5__blk568_dn2 = assign18510_e26049_d_n2;
        locals.var_t5__blk568_dn6 = assign18510_e26049_d_n6;
        locals.var_t5__blk568_dn7 = assign18510_e26049_d_n7;
        locals.var_t5__blk568_dn10 = assign18510_e26049_d_n10;
        locals.var_t5__blk568_dn11 = assign18510_e26049_d_n11;
        locals.var_t5__blk568_dn12 = assign18510_e26049_d_n12;
        locals.var_t5__blk568_dn17 = assign18510_e26049_d_n17;
        locals.var_t5__blk568_rv = 0.0;

        let (assign18520_e26062, assign18520_e26062_d_n0, assign18520_e26062_d_n2, assign18520_e26062_d_n6, assign18520_e26062_d_n7, assign18520_e26062_d_n10, assign18520_e26062_d_n11, assign18520_e26062_d_n12, assign18520_e26062_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) && (locals.var_guard574 == 0.0)) {
        let assign18520_e26058: f64 = (locals.var_ps0 + locals.var_t6__blk569);
        let assign18520_e26060: f64 = (assign18520_e26058 - locals.var_phi_b0_soi);
        (assign18520_e26060, ((locals.var_ps0_dn0 + locals.var_t6__blk569_dn0) - locals.var_phi_b0_soi_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk569_dn2) - locals.var_phi_b0_soi_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk569_dn6) - locals.var_phi_b0_soi_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk569_dn7) - locals.var_phi_b0_soi_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk569_dn10) - locals.var_phi_b0_soi_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk569_dn11) - locals.var_phi_b0_soi_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk569_dn12) - locals.var_phi_b0_soi_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk569_dn17) - locals.var_phi_b0_soi_dn17),)
    } else {
        (locals.var_t5__blk568, locals.var_t5__blk568_dn0, locals.var_t5__blk568_dn2, locals.var_t5__blk568_dn6, locals.var_t5__blk568_dn7, locals.var_t5__blk568_dn10, locals.var_t5__blk568_dn11, locals.var_t5__blk568_dn12, locals.var_t5__blk568_dn17,)
    }
};
        locals.var_t5__blk568 = assign18520_e26062;
        locals.var_t5__blk568_dn0 = assign18520_e26062_d_n0;
        locals.var_t5__blk568_dn2 = assign18520_e26062_d_n2;
        locals.var_t5__blk568_dn6 = assign18520_e26062_d_n6;
        locals.var_t5__blk568_dn7 = assign18520_e26062_d_n7;
        locals.var_t5__blk568_dn10 = assign18520_e26062_d_n10;
        locals.var_t5__blk568_dn11 = assign18520_e26062_d_n11;
        locals.var_t5__blk568_dn12 = assign18520_e26062_d_n12;
        locals.var_t5__blk568_dn17 = assign18520_e26062_d_n17;
        locals.var_t5__blk568_rv = 0.0;

        let (assign18530_e26074, assign18530_e26074_d_n0, assign18530_e26074_d_n2, assign18530_e26074_d_n6, assign18530_e26074_d_n7, assign18530_e26074_d_n10, assign18530_e26074_d_n11, assign18530_e26074_d_n12, assign18530_e26074_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18530_e26069: f64 = (locals.var_vdsz * locals.var_t0__blk564);
        let assign18530_e26071: f64 = (assign18530_e26069 * locals.var_t5__blk568);
        let assign18530_e26072: f64 = (locals.var_t4__blk567 + assign18530_e26071);
        (assign18530_e26072, (locals.var_t4__blk567_dn0 + ((((locals.var_vdsz_dn0 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn0)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn0))), (locals.var_t4__blk567_dn2 + ((((locals.var_vdsz_dn2 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn2)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn2))), (locals.var_t4__blk567_dn6 + ((((locals.var_vdsz_dn6 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn6)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn6))), (locals.var_t4__blk567_dn7 + ((((locals.var_vdsz_dn7 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn7)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn7))), (locals.var_t4__blk567_dn10 + ((((locals.var_vdsz_dn10 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn10)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn10))), (locals.var_t4__blk567_dn11 + ((((locals.var_vdsz_dn11 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn11)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn11))), (locals.var_t4__blk567_dn12 + ((((locals.var_vdsz_dn12 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn12)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn12))), (locals.var_t4__blk567_dn17 + ((((locals.var_vdsz_dn17 * locals.var_t0__blk564) + (locals.var_vdsz * locals.var_t0__blk564_dn17)) * locals.var_t5__blk568) + (assign18530_e26069 * locals.var_t5__blk568_dn17))),)
    } else {
        (locals.var_t4__blk567, locals.var_t4__blk567_dn0, locals.var_t4__blk567_dn2, locals.var_t4__blk567_dn6, locals.var_t4__blk567_dn7, locals.var_t4__blk567_dn10, locals.var_t4__blk567_dn11, locals.var_t4__blk567_dn12, locals.var_t4__blk567_dn17,)
    }
};
        locals.var_t4__blk567 = assign18530_e26074;
        locals.var_t4__blk567_dn0 = assign18530_e26074_d_n0;
        locals.var_t4__blk567_dn2 = assign18530_e26074_d_n2;
        locals.var_t4__blk567_dn6 = assign18530_e26074_d_n6;
        locals.var_t4__blk567_dn7 = assign18530_e26074_d_n7;
        locals.var_t4__blk567_dn10 = assign18530_e26074_d_n10;
        locals.var_t4__blk567_dn11 = assign18530_e26074_d_n11;
        locals.var_t4__blk567_dn12 = assign18530_e26074_d_n12;
        locals.var_t4__blk567_dn17 = assign18530_e26074_d_n17;
        locals.var_t4__blk567_rv = 0.0;

        let (assign18540_e26082, assign18540_e26082_d_n0, assign18540_e26082_d_n2, assign18540_e26082_d_n6, assign18540_e26082_d_n7, assign18540_e26082_d_n10, assign18540_e26082_d_n11, assign18540_e26082_d_n12, assign18540_e26082_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18540_e26080: f64 = (locals.var_t9__blk566 * locals.var_t4__blk567);
        (assign18540_e26080, ((locals.var_t9__blk566_dn0 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn0)), ((locals.var_t9__blk566_dn2 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn2)), ((locals.var_t9__blk566_dn6 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn6)), ((locals.var_t9__blk566_dn7 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn7)), ((locals.var_t9__blk566_dn10 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn10)), ((locals.var_t9__blk566_dn11 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn11)), ((locals.var_t9__blk566_dn12 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn12)), ((locals.var_t9__blk566_dn17 * locals.var_t4__blk567) + (locals.var_t9__blk566 * locals.var_t4__blk567_dn17)),)
    } else {
        (locals.var_t6__blk569, locals.var_t6__blk569_dn0, locals.var_t6__blk569_dn2, locals.var_t6__blk569_dn6, locals.var_t6__blk569_dn7, locals.var_t6__blk569_dn10, locals.var_t6__blk569_dn11, locals.var_t6__blk569_dn12, locals.var_t6__blk569_dn17,)
    }
};
        locals.var_t6__blk569 = assign18540_e26082;
        locals.var_t6__blk569_dn0 = assign18540_e26082_d_n0;
        locals.var_t6__blk569_dn2 = assign18540_e26082_d_n2;
        locals.var_t6__blk569_dn6 = assign18540_e26082_d_n6;
        locals.var_t6__blk569_dn7 = assign18540_e26082_d_n7;
        locals.var_t6__blk569_dn10 = assign18540_e26082_d_n10;
        locals.var_t6__blk569_dn11 = assign18540_e26082_d_n11;
        locals.var_t6__blk569_dn12 = assign18540_e26082_d_n12;
        locals.var_t6__blk569_dn17 = assign18540_e26082_d_n17;
        locals.var_t6__blk569_rv = 0.0;

        let (assign18550_e26088, assign18550_e26088_d_n0, assign18550_e26088_d_n2, assign18550_e26088_d_n6, assign18550_e26088_d_n7, assign18550_e26088_d_n10, assign18550_e26088_d_n11, assign18550_e26088_d_n12, assign18550_e26088_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 != 0.0)) {
        (locals.var_t6__blk569, locals.var_t6__blk569_dn0, locals.var_t6__blk569_dn2, locals.var_t6__blk569_dn6, locals.var_t6__blk569_dn7, locals.var_t6__blk569_dn10, locals.var_t6__blk569_dn11, locals.var_t6__blk569_dn12, locals.var_t6__blk569_dn17,)
    } else {
        (locals.var_t9__blk566, locals.var_t9__blk566_dn0, locals.var_t9__blk566_dn2, locals.var_t9__blk566_dn6, locals.var_t9__blk566_dn7, locals.var_t9__blk566_dn10, locals.var_t9__blk566_dn11, locals.var_t9__blk566_dn12, locals.var_t9__blk566_dn17,)
    }
};
        locals.var_t9__blk566 = assign18550_e26088;
        locals.var_t9__blk566_dn0 = assign18550_e26088_d_n0;
        locals.var_t9__blk566_dn2 = assign18550_e26088_d_n2;
        locals.var_t9__blk566_dn6 = assign18550_e26088_d_n6;
        locals.var_t9__blk566_dn7 = assign18550_e26088_d_n7;
        locals.var_t9__blk566_dn10 = assign18550_e26088_d_n10;
        locals.var_t9__blk566_dn11 = assign18550_e26088_d_n11;
        locals.var_t9__blk566_dn12 = assign18550_e26088_d_n12;
        locals.var_t9__blk566_dn17 = assign18550_e26088_d_n17;
        locals.var_t9__blk566_rv = 0.0;

        let (assign18560_e26095, assign18560_e26095_d_n0, assign18560_e26095_d_n2, assign18560_e26095_d_n6, assign18560_e26095_d_n7, assign18560_e26095_d_n10, assign18560_e26095_d_n11, assign18560_e26095_d_n12, assign18560_e26095_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard572 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9__blk566, locals.var_t9__blk566_dn0, locals.var_t9__blk566_dn2, locals.var_t9__blk566_dn6, locals.var_t9__blk566_dn7, locals.var_t9__blk566_dn10, locals.var_t9__blk566_dn11, locals.var_t9__blk566_dn12, locals.var_t9__blk566_dn17,)
    }
};
        locals.var_t9__blk566 = assign18560_e26095;
        locals.var_t9__blk566_dn0 = assign18560_e26095_d_n0;
        locals.var_t9__blk566_dn2 = assign18560_e26095_d_n2;
        locals.var_t9__blk566_dn6 = assign18560_e26095_d_n6;
        locals.var_t9__blk566_dn7 = assign18560_e26095_d_n7;
        locals.var_t9__blk566_dn10 = assign18560_e26095_d_n10;
        locals.var_t9__blk566_dn11 = assign18560_e26095_d_n11;
        locals.var_t9__blk566_dn12 = assign18560_e26095_d_n12;
        locals.var_t9__blk566_dn17 = assign18560_e26095_d_n17;
        locals.var_t9__blk566_rv = 0.0;

        let assign18570_e26098: f64 = if p.p248 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign18570_e26098;
        locals.var_guard575_rv = 0.0;

        let (assign18580_e26106, assign18580_e26106_d_n0, assign18580_e26106_d_n2, assign18580_e26106_d_n6, assign18580_e26106_d_n7, assign18580_e26106_d_n10, assign18580_e26106_d_n11, assign18580_e26106_d_n12, assign18580_e26106_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard575 != 0.0)) {
        let assign18580_e26104: f64 = (locals.var_beta * locals.var_gdl0);
        (assign18580_e26104, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_gdl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk563, locals.var_t1__blk563_dn0, locals.var_t1__blk563_dn2, locals.var_t1__blk563_dn6, locals.var_t1__blk563_dn7, locals.var_t1__blk563_dn10, locals.var_t1__blk563_dn11, locals.var_t1__blk563_dn12, locals.var_t1__blk563_dn17,)
    }
};
        locals.var_t1__blk563 = assign18580_e26106;
        locals.var_t1__blk563_dn0 = assign18580_e26106_d_n0;
        locals.var_t1__blk563_dn2 = assign18580_e26106_d_n2;
        locals.var_t1__blk563_dn6 = assign18580_e26106_d_n6;
        locals.var_t1__blk563_dn7 = assign18580_e26106_d_n7;
        locals.var_t1__blk563_dn10 = assign18580_e26106_d_n10;
        locals.var_t1__blk563_dn11 = assign18580_e26106_d_n11;
        locals.var_t1__blk563_dn12 = assign18580_e26106_d_n12;
        locals.var_t1__blk563_dn17 = assign18580_e26106_d_n17;
        locals.var_t1__blk563_rv = 0.0;

        let (assign18590_e26114, assign18590_e26114_d_n0, assign18590_e26114_d_n2, assign18590_e26114_d_n6, assign18590_e26114_d_n7, assign18590_e26114_d_n10, assign18590_e26114_d_n11, assign18590_e26114_d_n12, assign18590_e26114_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard575 != 0.0)) {
        let assign18590_e26112: f64 = (locals.var_c_fox * locals.var_t1__blk563);
        (assign18590_e26112, ((locals.var_c_fox_dn0 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn0)), ((locals.var_c_fox_dn2 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn2)), ((locals.var_c_fox_dn6 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn6)), ((locals.var_c_fox_dn7 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn7)), ((locals.var_c_fox_dn10 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn10)), ((locals.var_c_fox_dn11 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn11)), ((locals.var_c_fox_dn12 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn12)), ((locals.var_c_fox_dn17 * locals.var_t1__blk563) + (locals.var_c_fox * locals.var_t1__blk563_dn17)),)
    } else {
        (locals.var_t2__blk571, locals.var_t2__blk571_dn0, locals.var_t2__blk571_dn2, locals.var_t2__blk571_dn6, locals.var_t2__blk571_dn7, locals.var_t2__blk571_dn10, locals.var_t2__blk571_dn11, locals.var_t2__blk571_dn12, locals.var_t2__blk571_dn17,)
    }
};
        locals.var_t2__blk571 = assign18590_e26114;
        locals.var_t2__blk571_dn0 = assign18590_e26114_d_n0;
        locals.var_t2__blk571_dn2 = assign18590_e26114_d_n2;
        locals.var_t2__blk571_dn6 = assign18590_e26114_d_n6;
        locals.var_t2__blk571_dn7 = assign18590_e26114_d_n7;
        locals.var_t2__blk571_dn10 = assign18590_e26114_d_n10;
        locals.var_t2__blk571_dn11 = assign18590_e26114_d_n11;
        locals.var_t2__blk571_dn12 = assign18590_e26114_d_n12;
        locals.var_t2__blk571_dn17 = assign18590_e26114_d_n17;
        locals.var_t2__blk571_rv = 0.0;

        let (assign18600_e26122, assign18600_e26122_d_n0, assign18600_e26122_d_n2, assign18600_e26122_d_n6, assign18600_e26122_d_n7, assign18600_e26122_d_n10, assign18600_e26122_d_n11, assign18600_e26122_d_n12, assign18600_e26122_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard575 != 0.0)) {
        let assign18600_e26120: f64 = (locals.var_t2__blk571 * locals.var_vdsz);
        (assign18600_e26120, ((locals.var_t2__blk571_dn0 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn0)), ((locals.var_t2__blk571_dn2 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn2)), ((locals.var_t2__blk571_dn6 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn6)), ((locals.var_t2__blk571_dn7 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn7)), ((locals.var_t2__blk571_dn10 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn10)), ((locals.var_t2__blk571_dn11 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn11)), ((locals.var_t2__blk571_dn12 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn12)), ((locals.var_t2__blk571_dn17 * locals.var_vdsz) + (locals.var_t2__blk571 * locals.var_vdsz_dn17)),)
    } else {
        (locals.var_t8__blk570, locals.var_t8__blk570_dn0, locals.var_t8__blk570_dn2, locals.var_t8__blk570_dn6, locals.var_t8__blk570_dn7, locals.var_t8__blk570_dn10, locals.var_t8__blk570_dn11, locals.var_t8__blk570_dn12, locals.var_t8__blk570_dn17,)
    }
};
        locals.var_t8__blk570 = assign18600_e26122;
        locals.var_t8__blk570_dn0 = assign18600_e26122_d_n0;
        locals.var_t8__blk570_dn2 = assign18600_e26122_d_n2;
        locals.var_t8__blk570_dn6 = assign18600_e26122_d_n6;
        locals.var_t8__blk570_dn7 = assign18600_e26122_d_n7;
        locals.var_t8__blk570_dn10 = assign18600_e26122_d_n10;
        locals.var_t8__blk570_dn11 = assign18600_e26122_d_n11;
        locals.var_t8__blk570_dn12 = assign18600_e26122_d_n12;
        locals.var_t8__blk570_dn17 = assign18600_e26122_d_n17;
        locals.var_t8__blk570_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18610_e26129, assign18610_e26129_d_n0, assign18610_e26129_d_n2, assign18610_e26129_d_n6, assign18610_e26129_d_n7, assign18610_e26129_d_n10, assign18610_e26129_d_n11, assign18610_e26129_d_n12, assign18610_e26129_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard575 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8__blk570, locals.var_t8__blk570_dn0, locals.var_t8__blk570_dn2, locals.var_t8__blk570_dn6, locals.var_t8__blk570_dn7, locals.var_t8__blk570_dn10, locals.var_t8__blk570_dn11, locals.var_t8__blk570_dn12, locals.var_t8__blk570_dn17,)
    }
};
        locals.var_t8__blk570 = assign18610_e26129;
        locals.var_t8__blk570_dn0 = assign18610_e26129_d_n0;
        locals.var_t8__blk570_dn2 = assign18610_e26129_d_n2;
        locals.var_t8__blk570_dn6 = assign18610_e26129_d_n6;
        locals.var_t8__blk570_dn7 = assign18610_e26129_d_n7;
        locals.var_t8__blk570_dn10 = assign18610_e26129_d_n10;
        locals.var_t8__blk570_dn11 = assign18610_e26129_d_n11;
        locals.var_t8__blk570_dn12 = assign18610_e26129_d_n12;
        locals.var_t8__blk570_dn17 = assign18610_e26129_d_n17;
        locals.var_t8__blk570_rv = 0.0;

        let assign18620_e26132: f64 = (locals.var_t9__blk566 + locals.var_t8__blk570);
        let assign18620_e26134: f64 = if assign18620_e26132 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign18620_e26134;
        locals.var_guard576_rv = 0.0;

        let (assign18630_e26144, assign18630_e26144_d_n0, assign18630_e26144_d_n2, assign18630_e26144_d_n6, assign18630_e26144_d_n7, assign18630_e26144_d_n10, assign18630_e26144_d_n11, assign18630_e26144_d_n12, assign18630_e26144_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard576 != 0.0)) {
        let assign18630_e26141: f64 = (locals.var_t9__blk566 + locals.var_t8__blk570);
        let assign18630_e26142: f64 = (locals.var_pds * assign18630_e26141);
        (assign18630_e26142, ((locals.var_pds_dn0 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn0 + locals.var_t8__blk570_dn0))), ((locals.var_pds_dn2 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn2 + locals.var_t8__blk570_dn2))), ((locals.var_pds_dn6 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn6 + locals.var_t8__blk570_dn6))), ((locals.var_pds_dn7 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn7 + locals.var_t8__blk570_dn7))), ((locals.var_pds_dn10 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn10 + locals.var_t8__blk570_dn10))), ((locals.var_pds_dn11 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn11 + locals.var_t8__blk570_dn11))), ((locals.var_pds_dn12 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn12 + locals.var_t8__blk570_dn12))), ((locals.var_pds_dn17 * assign18630_e26141) + (locals.var_pds * (locals.var_t9__blk566_dn17 + locals.var_t8__blk570_dn17))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn12, locals.var_idd1_dn17,)
    }
};
        locals.var_idd1 = assign18630_e26144;
        locals.var_idd1_dn0 = assign18630_e26144_d_n0;
        locals.var_idd1_dn2 = assign18630_e26144_d_n2;
        locals.var_idd1_dn6 = assign18630_e26144_d_n6;
        locals.var_idd1_dn7 = assign18630_e26144_d_n7;
        locals.var_idd1_dn10 = assign18630_e26144_d_n10;
        locals.var_idd1_dn11 = assign18630_e26144_d_n11;
        locals.var_idd1_dn12 = assign18630_e26144_d_n12;
        locals.var_idd1_dn17 = assign18630_e26144_d_n17;
        locals.var_idd1_rv = 0.0;

        let (assign18640_e26154, assign18640_e26154_d_n0, assign18640_e26154_d_n2, assign18640_e26154_d_n6, assign18640_e26154_d_n7, assign18640_e26154_d_n10, assign18640_e26154_d_n11, assign18640_e26154_d_n12, assign18640_e26154_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard576 != 0.0)) {
        let assign18640_e26150: f64 = (locals.var_betawl * locals.var_idd1);
        let assign18640_e26152: f64 = (assign18640_e26150 * locals.var_mu);
        (assign18640_e26152, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn12)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn17)) * locals.var_mu) + (assign18640_e26150 * locals.var_mu_dn17)),)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18640_e26154;
        locals.var_idspt_dn0 = assign18640_e26154_d_n0;
        locals.var_idspt_dn2 = assign18640_e26154_d_n2;
        locals.var_idspt_dn6 = assign18640_e26154_d_n6;
        locals.var_idspt_dn7 = assign18640_e26154_d_n7;
        locals.var_idspt_dn10 = assign18640_e26154_d_n10;
        locals.var_idspt_dn11 = assign18640_e26154_d_n11;
        locals.var_idspt_dn12 = assign18640_e26154_d_n12;
        locals.var_idspt_dn17 = assign18640_e26154_d_n17;
        locals.var_idspt_rv = 0.0;

        let (assign18650_e26160, assign18650_e26160_d_n0, assign18650_e26160_d_n2, assign18650_e26160_d_n6, assign18650_e26160_d_n7, assign18650_e26160_d_n10, assign18650_e26160_d_n11, assign18650_e26160_d_n12, assign18650_e26160_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18650_e26158: f64 = (locals.var_ids0 + locals.var_idspt);
        (assign18650_e26158, (locals.var_ids0_dn0 + locals.var_idspt_dn0), (locals.var_ids0_dn2 + locals.var_idspt_dn2), (locals.var_ids0_dn6 + locals.var_idspt_dn6), (locals.var_ids0_dn7 + locals.var_idspt_dn7), (locals.var_ids0_dn10 + locals.var_idspt_dn10), (locals.var_ids0_dn11 + locals.var_idspt_dn11), (locals.var_ids0_dn12 + locals.var_idspt_dn12), (locals.var_ids0_dn17 + locals.var_idspt_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign18650_e26160;
        locals.var_ids_dn0 = assign18650_e26160_d_n0;
        locals.var_ids_dn2 = assign18650_e26160_d_n2;
        locals.var_ids_dn6 = assign18650_e26160_d_n6;
        locals.var_ids_dn7 = assign18650_e26160_d_n7;
        locals.var_ids_dn10 = assign18650_e26160_d_n10;
        locals.var_ids_dn11 = assign18650_e26160_d_n11;
        locals.var_ids_dn12 = assign18650_e26160_d_n12;
        locals.var_ids_dn17 = assign18650_e26160_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign18660_e26164, assign18660_e26164_d_n0, assign18660_e26164_d_n2, assign18660_e26164_d_n6, assign18660_e26164_d_n7, assign18660_e26164_d_n10, assign18660_e26164_d_n11, assign18660_e26164_d_n12, assign18660_e26164_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn12, locals.var_idspt0_dn17,)
    }
};
        locals.var_idspt0 = assign18660_e26164;
        locals.var_idspt0_dn0 = assign18660_e26164_d_n0;
        locals.var_idspt0_dn2 = assign18660_e26164_d_n2;
        locals.var_idspt0_dn6 = assign18660_e26164_d_n6;
        locals.var_idspt0_dn7 = assign18660_e26164_d_n7;
        locals.var_idspt0_dn10 = assign18660_e26164_d_n10;
        locals.var_idspt0_dn11 = assign18660_e26164_d_n11;
        locals.var_idspt0_dn12 = assign18660_e26164_d_n12;
        locals.var_idspt0_dn17 = assign18660_e26164_d_n17;
        locals.var_idspt0_rv = 0.0;

        let assign18670_e26167: f64 = if p.p33 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign18670_e26167;
        locals.var_guard586_rv = 0.0;

        let (assign18680_e26173, assign18680_e26173_d_n0, assign18680_e26173_d_n2, assign18680_e26173_d_n6, assign18680_e26173_d_n7, assign18680_e26173_d_n10, assign18680_e26173_d_n11, assign18680_e26173_d_n12, assign18680_e26173_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn12, locals.var_wdpl_dn17,)
    } else {
        (locals.var_t2__blk579, locals.var_t2__blk579_dn0, locals.var_t2__blk579_dn2, locals.var_t2__blk579_dn6, locals.var_t2__blk579_dn7, locals.var_t2__blk579_dn10, locals.var_t2__blk579_dn11, locals.var_t2__blk579_dn12, locals.var_t2__blk579_dn17,)
    }
};
        locals.var_t2__blk579 = assign18680_e26173;
        locals.var_t2__blk579_dn0 = assign18680_e26173_d_n0;
        locals.var_t2__blk579_dn2 = assign18680_e26173_d_n2;
        locals.var_t2__blk579_dn6 = assign18680_e26173_d_n6;
        locals.var_t2__blk579_dn7 = assign18680_e26173_d_n7;
        locals.var_t2__blk579_dn10 = assign18680_e26173_d_n10;
        locals.var_t2__blk579_dn11 = assign18680_e26173_d_n11;
        locals.var_t2__blk579_dn12 = assign18680_e26173_d_n12;
        locals.var_t2__blk579_dn17 = assign18680_e26173_d_n17;
        locals.var_t2__blk579_rv = 0.0;

        let (assign18690_e26181, assign18690_e26181_d_n0, assign18690_e26181_d_n2, assign18690_e26181_d_n6, assign18690_e26181_d_n7, assign18690_e26181_d_n10, assign18690_e26181_d_n11, assign18690_e26181_d_n12, assign18690_e26181_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18690_e26179: f64 = (locals.var_lgatesm - p.p71);
        (assign18690_e26179, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk580, locals.var_t3__blk580_dn0, locals.var_t3__blk580_dn2, locals.var_t3__blk580_dn6, locals.var_t3__blk580_dn7, locals.var_t3__blk580_dn10, locals.var_t3__blk580_dn11, locals.var_t3__blk580_dn12, locals.var_t3__blk580_dn17,)
    }
};
        locals.var_t3__blk580 = assign18690_e26181;
        locals.var_t3__blk580_dn0 = assign18690_e26181_d_n0;
        locals.var_t3__blk580_dn2 = assign18690_e26181_d_n2;
        locals.var_t3__blk580_dn6 = assign18690_e26181_d_n6;
        locals.var_t3__blk580_dn7 = assign18690_e26181_d_n7;
        locals.var_t3__blk580_dn10 = assign18690_e26181_d_n10;
        locals.var_t3__blk580_dn11 = assign18690_e26181_d_n11;
        locals.var_t3__blk580_dn12 = assign18690_e26181_d_n12;
        locals.var_t3__blk580_dn17 = assign18690_e26181_d_n17;
        locals.var_t3__blk580_rv = 0.0;

        let (assign18700_e26191, assign18700_e26191_d_n0, assign18700_e26191_d_n2, assign18700_e26191_d_n6, assign18700_e26191_d_n7, assign18700_e26191_d_n10, assign18700_e26191_d_n11, assign18700_e26191_d_n12, assign18700_e26191_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18700_e26188: f64 = (locals.var_t3__blk580 * locals.var_t3__blk580);
        let assign18700_e26189: f64 = (1.0 / assign18700_e26188);
        (assign18700_e26189, (-(((locals.var_t3__blk580_dn0 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn0)) / (assign18700_e26188 * assign18700_e26188))), (-(((locals.var_t3__blk580_dn2 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn2)) / (assign18700_e26188 * assign18700_e26188))), (-(((locals.var_t3__blk580_dn6 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn6)) / (assign18700_e26188 * assign18700_e26188))), (-(((locals.var_t3__blk580_dn7 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn7)) / (assign18700_e26188 * assign18700_e26188))), (-(((locals.var_t3__blk580_dn10 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn10)) / (assign18700_e26188 * assign18700_e26188))), (-(((locals.var_t3__blk580_dn11 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn11)) / (assign18700_e26188 * assign18700_e26188))), (-(((locals.var_t3__blk580_dn12 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn12)) / (assign18700_e26188 * assign18700_e26188))), (-(((locals.var_t3__blk580_dn17 * locals.var_t3__blk580) + (locals.var_t3__blk580 * locals.var_t3__blk580_dn17)) / (assign18700_e26188 * assign18700_e26188))),)
    } else {
        (locals.var_t4__blk581, locals.var_t4__blk581_dn0, locals.var_t4__blk581_dn2, locals.var_t4__blk581_dn6, locals.var_t4__blk581_dn7, locals.var_t4__blk581_dn10, locals.var_t4__blk581_dn11, locals.var_t4__blk581_dn12, locals.var_t4__blk581_dn17,)
    }
};
        locals.var_t4__blk581 = assign18700_e26191;
        locals.var_t4__blk581_dn0 = assign18700_e26191_d_n0;
        locals.var_t4__blk581_dn2 = assign18700_e26191_d_n2;
        locals.var_t4__blk581_dn6 = assign18700_e26191_d_n6;
        locals.var_t4__blk581_dn7 = assign18700_e26191_d_n7;
        locals.var_t4__blk581_dn10 = assign18700_e26191_d_n10;
        locals.var_t4__blk581_dn11 = assign18700_e26191_d_n11;
        locals.var_t4__blk581_dn12 = assign18700_e26191_d_n12;
        locals.var_t4__blk581_dn17 = assign18700_e26191_d_n17;
        locals.var_t4__blk581_rv = 0.0;

        let (assign18710_e26209, assign18710_e26209_d_n0, assign18710_e26209_d_n2, assign18710_e26209_d_n6, assign18710_e26209_d_n7, assign18710_e26209_d_n10, assign18710_e26209_d_n11, assign18710_e26209_d_n12, assign18710_e26209_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18710_e26198: f64 = (p.p69 - locals.var_pb20b);
        let assign18710_e26199: f64 = (2.0 * assign18710_e26198);
        let assign18710_e26202: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        let assign18710_e26203: f64 = (assign18710_e26199 * assign18710_e26202);
        let assign18710_e26205: f64 = (assign18710_e26203 * locals.var_t2__blk579);
        let assign18710_e26207: f64 = (assign18710_e26205 * locals.var_t4__blk581);
        (assign18710_e26207, (((((((2.0 * (-locals.var_pb20b_dn0)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn0))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn0)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn2))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn2)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn2)), (((((((2.0 * (-locals.var_pb20b_dn6)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn6))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn6)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn7))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn7)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn7)), (((((((2.0 * (-locals.var_pb20b_dn10)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn10))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn10)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn11))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn11)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn11)), (((((((2.0 * (-locals.var_pb20b_dn12)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn12))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn12)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn12)), (((((((2.0 * (-locals.var_pb20b_dn17)) * assign18710_e26202) + (assign18710_e26199 * (1.034943e-10 * locals.var_c_fox_inv_dn17))) * locals.var_t2__blk579) + (assign18710_e26203 * locals.var_t2__blk579_dn17)) * locals.var_t4__blk581) + (assign18710_e26205 * locals.var_t4__blk581_dn17)),)
    } else {
        (locals.var_t5__blk582, locals.var_t5__blk582_dn0, locals.var_t5__blk582_dn2, locals.var_t5__blk582_dn6, locals.var_t5__blk582_dn7, locals.var_t5__blk582_dn10, locals.var_t5__blk582_dn11, locals.var_t5__blk582_dn12, locals.var_t5__blk582_dn17,)
    }
};
        locals.var_t5__blk582 = assign18710_e26209;
        locals.var_t5__blk582_dn0 = assign18710_e26209_d_n0;
        locals.var_t5__blk582_dn2 = assign18710_e26209_d_n2;
        locals.var_t5__blk582_dn6 = assign18710_e26209_d_n6;
        locals.var_t5__blk582_dn7 = assign18710_e26209_d_n7;
        locals.var_t5__blk582_dn10 = assign18710_e26209_d_n10;
        locals.var_t5__blk582_dn11 = assign18710_e26209_d_n11;
        locals.var_t5__blk582_dn12 = assign18710_e26209_d_n12;
        locals.var_t5__blk582_dn17 = assign18710_e26209_d_n17;
        locals.var_t5__blk582_rv = 0.0;

        let (assign18720_e26217, assign18720_e26217_d_n0, assign18720_e26217_d_n2, assign18720_e26217_d_n6, assign18720_e26217_d_n7, assign18720_e26217_d_n10, assign18720_e26217_d_n11, assign18720_e26217_d_n12, assign18720_e26217_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18720_e26215: f64 = (locals.var_t5__blk582 * locals.var_sqrt_pbsum);
        (assign18720_e26215, ((locals.var_t5__blk582_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5__blk582_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5__blk582_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5__blk582_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5__blk582_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5__blk582_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5__blk582_dn12 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn12)), ((locals.var_t5__blk582_dn17 * locals.var_sqrt_pbsum) + (locals.var_t5__blk582 * locals.var_sqrt_pbsum_dn17)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn12, locals.var_dvth0_dn17,)
    }
};
        locals.var_dvth0 = assign18720_e26217;
        locals.var_dvth0_dn0 = assign18720_e26217_d_n0;
        locals.var_dvth0_dn2 = assign18720_e26217_d_n2;
        locals.var_dvth0_dn6 = assign18720_e26217_d_n6;
        locals.var_dvth0_dn7 = assign18720_e26217_d_n7;
        locals.var_dvth0_dn10 = assign18720_e26217_d_n10;
        locals.var_dvth0_dn11 = assign18720_e26217_d_n11;
        locals.var_dvth0_dn12 = assign18720_e26217_d_n12;
        locals.var_dvth0_dn17 = assign18720_e26217_d_n17;
        locals.var_dvth0_rv = 0.0;

        let (assign18730_e26227, assign18730_e26227_d_n0, assign18730_e26227_d_n2, assign18730_e26227_d_n6, assign18730_e26227_d_n7, assign18730_e26227_d_n10, assign18730_e26227_d_n11, assign18730_e26227_d_n12, assign18730_e26227_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18730_e26224: f64 = (p.p155 * locals.var_vdsz);
        let assign18730_e26225: f64 = (p.p154 + assign18730_e26224);
        (assign18730_e26225, (p.p155 * locals.var_vdsz_dn0), (p.p155 * locals.var_vdsz_dn2), (p.p155 * locals.var_vdsz_dn6), (p.p155 * locals.var_vdsz_dn7), (p.p155 * locals.var_vdsz_dn10), (p.p155 * locals.var_vdsz_dn11), (p.p155 * locals.var_vdsz_dn12), (p.p155 * locals.var_vdsz_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18730_e26227;
        locals.var_t1w_dn0 = assign18730_e26227_d_n0;
        locals.var_t1w_dn2 = assign18730_e26227_d_n2;
        locals.var_t1w_dn6 = assign18730_e26227_d_n6;
        locals.var_t1w_dn7 = assign18730_e26227_d_n7;
        locals.var_t1w_dn10 = assign18730_e26227_d_n10;
        locals.var_t1w_dn11 = assign18730_e26227_d_n11;
        locals.var_t1w_dn12 = assign18730_e26227_d_n12;
        locals.var_t1w_dn17 = assign18730_e26227_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign18740_e26235, assign18740_e26235_d_n0, assign18740_e26235_d_n2, assign18740_e26235_d_n6, assign18740_e26235_d_n7, assign18740_e26235_d_n10, assign18740_e26235_d_n11, assign18740_e26235_d_n12, assign18740_e26235_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18740_e26233: f64 = (locals.var_dvth0 * locals.var_t1w);
        (assign18740_e26233, ((locals.var_dvth0_dn0 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn2)), ((locals.var_dvth0_dn6 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn7)), ((locals.var_dvth0_dn10 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn11)), ((locals.var_dvth0_dn12 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn12)), ((locals.var_dvth0_dn17 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn17)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn12, locals.var_dvthscsti_dn17,)
    }
};
        locals.var_dvthscsti = assign18740_e26235;
        locals.var_dvthscsti_dn0 = assign18740_e26235_d_n0;
        locals.var_dvthscsti_dn2 = assign18740_e26235_d_n2;
        locals.var_dvthscsti_dn6 = assign18740_e26235_d_n6;
        locals.var_dvthscsti_dn7 = assign18740_e26235_d_n7;
        locals.var_dvthscsti_dn10 = assign18740_e26235_d_n10;
        locals.var_dvthscsti_dn11 = assign18740_e26235_d_n11;
        locals.var_dvthscsti_dn12 = assign18740_e26235_d_n12;
        locals.var_dvthscsti_dn17 = assign18740_e26235_d_n17;
        locals.var_dvthscsti_rv = 0.0;

        let (assign18750_e26245, assign18750_e26245_d_n0, assign18750_e26245_d_n2, assign18750_e26245_d_n6, assign18750_e26245_d_n7, assign18750_e26245_d_n10, assign18750_e26245_d_n11, assign18750_e26245_d_n12, assign18750_e26245_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18750_e26242: f64 = (p.p157 * locals.var_vds);
        let assign18750_e26243: f64 = (p.p156 - assign18750_e26242);
        (assign18750_e26243, (-(p.p157 * locals.var_vds_dn0)), (-(p.p157 * locals.var_vds_dn2)), (-(p.p157 * locals.var_vds_dn6)), (-(p.p157 * locals.var_vds_dn7)), (-(p.p157 * locals.var_vds_dn10)), (-(p.p157 * locals.var_vds_dn11)), (-(p.p157 * locals.var_vds_dn12)), (-(p.p157 * locals.var_vds_dn17)),)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign18750_e26245;
        locals.var_t1__blk577_dn0 = assign18750_e26245_d_n0;
        locals.var_t1__blk577_dn2 = assign18750_e26245_d_n2;
        locals.var_t1__blk577_dn6 = assign18750_e26245_d_n6;
        locals.var_t1__blk577_dn7 = assign18750_e26245_d_n7;
        locals.var_t1__blk577_dn10 = assign18750_e26245_d_n10;
        locals.var_t1__blk577_dn11 = assign18750_e26245_d_n11;
        locals.var_t1__blk577_dn12 = assign18750_e26245_d_n12;
        locals.var_t1__blk577_dn17 = assign18750_e26245_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let (assign18760_e26257, assign18760_e26257_d_n0, assign18760_e26257_d_n2, assign18760_e26257_d_n6, assign18760_e26257_d_n7, assign18760_e26257_d_n10, assign18760_e26257_d_n11, assign18760_e26257_d_n12, assign18760_e26257_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18760_e26251: f64 = (locals.var_vgsz - locals.var_vfb);
        let assign18760_e26253: f64 = (assign18760_e26251 + locals.var_t1__blk577);
        let assign18760_e26255: f64 = (assign18760_e26253 + locals.var_dvthscsti);
        (assign18760_e26255, ((locals.var_vgsz_dn0 + locals.var_t1__blk577_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz_dn2 + locals.var_t1__blk577_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz_dn6 + locals.var_t1__blk577_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz_dn7 + locals.var_t1__blk577_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz_dn10 + locals.var_t1__blk577_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz_dn11 + locals.var_t1__blk577_dn11) + locals.var_dvthscsti_dn11), ((locals.var_vgsz_dn12 + locals.var_t1__blk577_dn12) + locals.var_dvthscsti_dn12), ((locals.var_vgsz_dn17 + locals.var_t1__blk577_dn17) + locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn12, locals.var_vgssti_dn17,)
    }
};
        locals.var_vgssti = assign18760_e26257;
        locals.var_vgssti_dn0 = assign18760_e26257_d_n0;
        locals.var_vgssti_dn2 = assign18760_e26257_d_n2;
        locals.var_vgssti_dn6 = assign18760_e26257_d_n6;
        locals.var_vgssti_dn7 = assign18760_e26257_d_n7;
        locals.var_vgssti_dn10 = assign18760_e26257_d_n10;
        locals.var_vgssti_dn11 = assign18760_e26257_d_n11;
        locals.var_vgssti_dn12 = assign18760_e26257_d_n12;
        locals.var_vgssti_dn17 = assign18760_e26257_d_n17;
        locals.var_vgssti_rv = 0.0;

        let (assign18770_e26267, assign18770_e26267_d_n0, assign18770_e26267_d_n2, assign18770_e26267_d_n6, assign18770_e26267_d_n7, assign18770_e26267_d_n10, assign18770_e26267_d_n11, assign18770_e26267_d_n12, assign18770_e26267_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18770_e26263: f64 = (locals.var_costi0_p2 * locals.var_c_fox_inv);
        let assign18770_e26265: f64 = (assign18770_e26263 * locals.var_c_fox_inv);
        (assign18770_e26265, ((((locals.var_costi0_p2_dn0 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn2)), ((((locals.var_costi0_p2_dn6 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn7)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn7)), ((((locals.var_costi0_p2_dn10 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn11)), ((((locals.var_costi0_p2_dn12 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn12)), ((((locals.var_costi0_p2_dn17 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn17)) * locals.var_c_fox_inv) + (assign18770_e26263 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn12, locals.var_costi3_dn17,)
    }
};
        locals.var_costi3 = assign18770_e26267;
        locals.var_costi3_dn0 = assign18770_e26267_d_n0;
        locals.var_costi3_dn2 = assign18770_e26267_d_n2;
        locals.var_costi3_dn6 = assign18770_e26267_d_n6;
        locals.var_costi3_dn7 = assign18770_e26267_d_n7;
        locals.var_costi3_dn10 = assign18770_e26267_d_n10;
        locals.var_costi3_dn11 = assign18770_e26267_d_n11;
        locals.var_costi3_dn12 = assign18770_e26267_d_n12;
        locals.var_costi3_dn17 = assign18770_e26267_d_n17;
        locals.var_costi3_rv = 0.0;

        let (assign18780_e26277, assign18780_e26277_d_n0, assign18780_e26277_d_n2, assign18780_e26277_d_n6, assign18780_e26277_d_n7, assign18780_e26277_d_n10, assign18780_e26277_d_n11, assign18780_e26277_d_n12, assign18780_e26277_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18780_e26273: f64 = (locals.var_costi3 * locals.var_beta);
        let assign18780_e26275: f64 = (assign18780_e26273 * 0.5);
        (assign18780_e26275, ((locals.var_costi3_dn0 * locals.var_beta) * 0.5), ((locals.var_costi3_dn2 * locals.var_beta) * 0.5), ((locals.var_costi3_dn6 * locals.var_beta) * 0.5), ((locals.var_costi3_dn7 * locals.var_beta) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), ((locals.var_costi3_dn11 * locals.var_beta) * 0.5), ((locals.var_costi3_dn12 * locals.var_beta) * 0.5), ((locals.var_costi3_dn17 * locals.var_beta) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn12, locals.var_costi4_dn17,)
    }
};
        locals.var_costi4 = assign18780_e26277;
        locals.var_costi4_dn0 = assign18780_e26277_d_n0;
        locals.var_costi4_dn2 = assign18780_e26277_d_n2;
        locals.var_costi4_dn6 = assign18780_e26277_d_n6;
        locals.var_costi4_dn7 = assign18780_e26277_d_n7;
        locals.var_costi4_dn10 = assign18780_e26277_d_n10;
        locals.var_costi4_dn11 = assign18780_e26277_d_n11;
        locals.var_costi4_dn12 = assign18780_e26277_d_n12;
        locals.var_costi4_dn17 = assign18780_e26277_d_n17;
        locals.var_costi4_rv = 0.0;

        let (assign18790_e26287, assign18790_e26287_d_n0, assign18790_e26287_d_n2, assign18790_e26287_d_n6, assign18790_e26287_d_n7, assign18790_e26287_d_n10, assign18790_e26287_d_n11, assign18790_e26287_d_n12, assign18790_e26287_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18790_e26283: f64 = (locals.var_costi4 * locals.var_beta);
        let assign18790_e26285: f64 = (assign18790_e26283 * 2.0);
        (assign18790_e26285, ((locals.var_costi4_dn0 * locals.var_beta) * 2.0), ((locals.var_costi4_dn2 * locals.var_beta) * 2.0), ((locals.var_costi4_dn6 * locals.var_beta) * 2.0), ((locals.var_costi4_dn7 * locals.var_beta) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), ((locals.var_costi4_dn11 * locals.var_beta) * 2.0), ((locals.var_costi4_dn12 * locals.var_beta) * 2.0), ((locals.var_costi4_dn17 * locals.var_beta) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn12, locals.var_costi5_dn17,)
    }
};
        locals.var_costi5 = assign18790_e26287;
        locals.var_costi5_dn0 = assign18790_e26287_d_n0;
        locals.var_costi5_dn2 = assign18790_e26287_d_n2;
        locals.var_costi5_dn6 = assign18790_e26287_d_n6;
        locals.var_costi5_dn7 = assign18790_e26287_d_n7;
        locals.var_costi5_dn10 = assign18790_e26287_d_n10;
        locals.var_costi5_dn11 = assign18790_e26287_d_n11;
        locals.var_costi5_dn12 = assign18790_e26287_d_n12;
        locals.var_costi5_dn17 = assign18790_e26287_d_n17;
        locals.var_costi5_rv = 0.0;

        let (assign18800_e26307, assign18800_e26307_d_n0, assign18800_e26307_d_n2, assign18800_e26307_d_n6, assign18800_e26307_d_n7, assign18800_e26307_d_n10, assign18800_e26307_d_n11, assign18800_e26307_d_n12, assign18800_e26307_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18800_e26295: f64 = (locals.var_beta * 0.25);
        let assign18800_e26296: f64 = (locals.var_costi3 * assign18800_e26295);
        let assign18800_e26297: f64 = (locals.var_beta_inv - assign18800_e26296);
        let assign18800_e26299: f64 = (assign18800_e26297 + locals.var_vfb);
        let assign18800_e26301: f64 = (assign18800_e26299 - p.p156);
        let assign18800_e26303: f64 = (assign18800_e26301 - locals.var_dvthscsti);
        let assign18800_e26305: f64 = (assign18800_e26303 + 1e-50);
        (assign18800_e26305, ((-(locals.var_costi3_dn0 * assign18800_e26295)) - locals.var_dvthscsti_dn0), ((-(locals.var_costi3_dn2 * assign18800_e26295)) - locals.var_dvthscsti_dn2), ((-(locals.var_costi3_dn6 * assign18800_e26295)) - locals.var_dvthscsti_dn6), ((-(locals.var_costi3_dn7 * assign18800_e26295)) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * assign18800_e26295) + (locals.var_costi3 * (locals.var_beta_dn10 * 0.25)))) - locals.var_dvthscsti_dn10), ((-(locals.var_costi3_dn11 * assign18800_e26295)) - locals.var_dvthscsti_dn11), ((-(locals.var_costi3_dn12 * assign18800_e26295)) - locals.var_dvthscsti_dn12), ((-(locals.var_costi3_dn17 * assign18800_e26295)) - locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_t10__blk583, locals.var_t10__blk583_dn0, locals.var_t10__blk583_dn2, locals.var_t10__blk583_dn6, locals.var_t10__blk583_dn7, locals.var_t10__blk583_dn10, locals.var_t10__blk583_dn11, locals.var_t10__blk583_dn12, locals.var_t10__blk583_dn17,)
    }
};
        locals.var_t10__blk583 = assign18800_e26307;
        locals.var_t10__blk583_dn0 = assign18800_e26307_d_n0;
        locals.var_t10__blk583_dn2 = assign18800_e26307_d_n2;
        locals.var_t10__blk583_dn6 = assign18800_e26307_d_n6;
        locals.var_t10__blk583_dn7 = assign18800_e26307_d_n7;
        locals.var_t10__blk583_dn10 = assign18800_e26307_d_n10;
        locals.var_t10__blk583_dn11 = assign18800_e26307_d_n11;
        locals.var_t10__blk583_dn12 = assign18800_e26307_d_n12;
        locals.var_t10__blk583_dn17 = assign18800_e26307_d_n17;
        locals.var_t10__blk583_rv = 0.0;

        let (assign18810_e26317, assign18810_e26317_d_n0, assign18810_e26317_d_n2, assign18810_e26317_d_n6, assign18810_e26317_d_n7, assign18810_e26317_d_n10, assign18810_e26317_d_n11, assign18810_e26317_d_n12, assign18810_e26317_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18810_e26313: f64 = (locals.var_vgsz - locals.var_t10__blk583);
        let assign18810_e26315: f64 = (assign18810_e26313 - 0.005);
        (assign18810_e26315, (locals.var_vgsz_dn0 - locals.var_t10__blk583_dn0), (locals.var_vgsz_dn2 - locals.var_t10__blk583_dn2), (locals.var_vgsz_dn6 - locals.var_t10__blk583_dn6), (locals.var_vgsz_dn7 - locals.var_t10__blk583_dn7), (locals.var_vgsz_dn10 - locals.var_t10__blk583_dn10), (locals.var_vgsz_dn11 - locals.var_t10__blk583_dn11), (locals.var_vgsz_dn12 - locals.var_t10__blk583_dn12), (locals.var_vgsz_dn17 - locals.var_t10__blk583_dn17),)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign18810_e26317;
        locals.var_t1__blk577_dn0 = assign18810_e26317_d_n0;
        locals.var_t1__blk577_dn2 = assign18810_e26317_d_n2;
        locals.var_t1__blk577_dn6 = assign18810_e26317_d_n6;
        locals.var_t1__blk577_dn7 = assign18810_e26317_d_n7;
        locals.var_t1__blk577_dn10 = assign18810_e26317_d_n10;
        locals.var_t1__blk577_dn11 = assign18810_e26317_d_n11;
        locals.var_t1__blk577_dn12 = assign18810_e26317_d_n12;
        locals.var_t1__blk577_dn17 = assign18810_e26317_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let (assign18820_e26329, assign18820_e26329_d_n0, assign18820_e26329_d_n2, assign18820_e26329_d_n6, assign18820_e26329_d_n7, assign18820_e26329_d_n10, assign18820_e26329_d_n11, assign18820_e26329_d_n12, assign18820_e26329_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let (assign18820_e26327,) = {
            if (locals.var_t10__blk583 >= 0.0) {
                (1.0,)
            } else {
                let assign18820_e26326: f64 = (-1.0);
                (assign18820_e26326,)
            }
        };
        (assign18820_e26327, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18820_e26329;
        locals.var_t0_dn0 = assign18820_e26329_d_n0;
        locals.var_t0_dn2 = assign18820_e26329_d_n2;
        locals.var_t0_dn6 = assign18820_e26329_d_n6;
        locals.var_t0_dn7 = assign18820_e26329_d_n7;
        locals.var_t0_dn10 = assign18820_e26329_d_n10;
        locals.var_t0_dn11 = assign18820_e26329_d_n11;
        locals.var_t0_dn12 = assign18820_e26329_d_n12;
        locals.var_t0_dn17 = assign18820_e26329_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18830_e26346, assign18830_e26346_d_n0, assign18830_e26346_d_n2, assign18830_e26346_d_n6, assign18830_e26346_d_n7, assign18830_e26346_d_n10, assign18830_e26346_d_n11, assign18830_e26346_d_n12, assign18830_e26346_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18830_e26335: f64 = (locals.var_t1__blk577 * locals.var_t1__blk577);
        let assign18830_e26338: f64 = (locals.var_t0 * 4.0);
        let assign18830_e26340: f64 = (assign18830_e26338 * locals.var_t10__blk583);
        let assign18830_e26342: f64 = (assign18830_e26340 * 0.005);
        let assign18830_e26343: f64 = (assign18830_e26335 + assign18830_e26342);
        let assign18830_e26344: f64 = (assign18830_e26343).sqrt();
        (assign18830_e26344, ((((locals.var_t1__blk577_dn0 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn0)) * 0.005)) / (2.0 * assign18830_e26344)), ((((locals.var_t1__blk577_dn2 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn2)) * 0.005)) / (2.0 * assign18830_e26344)), ((((locals.var_t1__blk577_dn6 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn6)) * 0.005)) / (2.0 * assign18830_e26344)), ((((locals.var_t1__blk577_dn7 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn7)) * 0.005)) / (2.0 * assign18830_e26344)), ((((locals.var_t1__blk577_dn10 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn10)) * 0.005)) / (2.0 * assign18830_e26344)), ((((locals.var_t1__blk577_dn11 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn11)) * 0.005)) / (2.0 * assign18830_e26344)), ((((locals.var_t1__blk577_dn12 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn12)) + ((((locals.var_t0_dn12 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn12)) * 0.005)) / (2.0 * assign18830_e26344)), ((((locals.var_t1__blk577_dn17 * locals.var_t1__blk577) + (locals.var_t1__blk577 * locals.var_t1__blk577_dn17)) + ((((locals.var_t0_dn17 * 4.0) * locals.var_t10__blk583) + (assign18830_e26338 * locals.var_t10__blk583_dn17)) * 0.005)) / (2.0 * assign18830_e26344)),)
    } else {
        (locals.var_t2__blk579, locals.var_t2__blk579_dn0, locals.var_t2__blk579_dn2, locals.var_t2__blk579_dn6, locals.var_t2__blk579_dn7, locals.var_t2__blk579_dn10, locals.var_t2__blk579_dn11, locals.var_t2__blk579_dn12, locals.var_t2__blk579_dn17,)
    }
};
        locals.var_t2__blk579 = assign18830_e26346;
        locals.var_t2__blk579_dn0 = assign18830_e26346_d_n0;
        locals.var_t2__blk579_dn2 = assign18830_e26346_d_n2;
        locals.var_t2__blk579_dn6 = assign18830_e26346_d_n6;
        locals.var_t2__blk579_dn7 = assign18830_e26346_d_n7;
        locals.var_t2__blk579_dn10 = assign18830_e26346_d_n10;
        locals.var_t2__blk579_dn11 = assign18830_e26346_d_n11;
        locals.var_t2__blk579_dn12 = assign18830_e26346_d_n12;
        locals.var_t2__blk579_dn17 = assign18830_e26346_d_n17;
        locals.var_t2__blk579_rv = 0.0;

        let (assign18840_e26366, assign18840_e26366_d_n0, assign18840_e26366_d_n2, assign18840_e26366_d_n6, assign18840_e26366_d_n7, assign18840_e26366_d_n10, assign18840_e26366_d_n11, assign18840_e26366_d_n12, assign18840_e26366_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18840_e26354: f64 = (locals.var_t1__blk577 + locals.var_t2__blk579);
        let assign18840_e26355: f64 = (0.5 * assign18840_e26354);
        let assign18840_e26356: f64 = (locals.var_t10__blk583 + assign18840_e26355);
        let assign18840_e26358: f64 = (assign18840_e26356 - locals.var_vfb);
        let assign18840_e26360: f64 = (assign18840_e26358 + p.p156);
        let assign18840_e26362: f64 = (assign18840_e26360 + locals.var_dvthscsti);
        let assign18840_e26364: f64 = (assign18840_e26362 - locals.var_vbspz);
        (assign18840_e26364, (((locals.var_t10__blk583_dn0 + (0.5 * (locals.var_t1__blk577_dn0 + locals.var_t2__blk579_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbspz_dn0), (((locals.var_t10__blk583_dn2 + (0.5 * (locals.var_t1__blk577_dn2 + locals.var_t2__blk579_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbspz_dn2), (((locals.var_t10__blk583_dn6 + (0.5 * (locals.var_t1__blk577_dn6 + locals.var_t2__blk579_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbspz_dn6), (((locals.var_t10__blk583_dn7 + (0.5 * (locals.var_t1__blk577_dn7 + locals.var_t2__blk579_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbspz_dn7), (((locals.var_t10__blk583_dn10 + (0.5 * (locals.var_t1__blk577_dn10 + locals.var_t2__blk579_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbspz_dn10), (((locals.var_t10__blk583_dn11 + (0.5 * (locals.var_t1__blk577_dn11 + locals.var_t2__blk579_dn11))) + locals.var_dvthscsti_dn11) - locals.var_vbspz_dn11), (((locals.var_t10__blk583_dn12 + (0.5 * (locals.var_t1__blk577_dn12 + locals.var_t2__blk579_dn12))) + locals.var_dvthscsti_dn12) - locals.var_vbspz_dn12), (((locals.var_t10__blk583_dn17 + (0.5 * (locals.var_t1__blk577_dn17 + locals.var_t2__blk579_dn17))) + locals.var_dvthscsti_dn17) - locals.var_vbspz_dn17),)
    } else {
        (locals.var_t3__blk580, locals.var_t3__blk580_dn0, locals.var_t3__blk580_dn2, locals.var_t3__blk580_dn6, locals.var_t3__blk580_dn7, locals.var_t3__blk580_dn10, locals.var_t3__blk580_dn11, locals.var_t3__blk580_dn12, locals.var_t3__blk580_dn17,)
    }
};
        locals.var_t3__blk580 = assign18840_e26366;
        locals.var_t3__blk580_dn0 = assign18840_e26366_d_n0;
        locals.var_t3__blk580_dn2 = assign18840_e26366_d_n2;
        locals.var_t3__blk580_dn6 = assign18840_e26366_d_n6;
        locals.var_t3__blk580_dn7 = assign18840_e26366_d_n7;
        locals.var_t3__blk580_dn10 = assign18840_e26366_d_n10;
        locals.var_t3__blk580_dn11 = assign18840_e26366_d_n11;
        locals.var_t3__blk580_dn12 = assign18840_e26366_d_n12;
        locals.var_t3__blk580_dn17 = assign18840_e26366_d_n17;
        locals.var_t3__blk580_rv = 0.0;

        let (assign18850_e26376, assign18850_e26376_d_n0, assign18850_e26376_d_n2, assign18850_e26376_d_n6, assign18850_e26376_d_n7, assign18850_e26376_d_n10, assign18850_e26376_d_n11, assign18850_e26376_d_n12, assign18850_e26376_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18850_e26372: f64 = (locals.var_beta * locals.var_t3__blk580);
        let assign18850_e26374: f64 = (assign18850_e26372 - 1.0);
        (assign18850_e26374, (locals.var_beta * locals.var_t3__blk580_dn0), (locals.var_beta * locals.var_t3__blk580_dn2), (locals.var_beta * locals.var_t3__blk580_dn6), (locals.var_beta * locals.var_t3__blk580_dn7), ((locals.var_beta_dn10 * locals.var_t3__blk580) + (locals.var_beta * locals.var_t3__blk580_dn10)), (locals.var_beta * locals.var_t3__blk580_dn11), (locals.var_beta * locals.var_t3__blk580_dn12), (locals.var_beta * locals.var_t3__blk580_dn17),)
    } else {
        (locals.var_t4__blk581, locals.var_t4__blk581_dn0, locals.var_t4__blk581_dn2, locals.var_t4__blk581_dn6, locals.var_t4__blk581_dn7, locals.var_t4__blk581_dn10, locals.var_t4__blk581_dn11, locals.var_t4__blk581_dn12, locals.var_t4__blk581_dn17,)
    }
};
        locals.var_t4__blk581 = assign18850_e26376;
        locals.var_t4__blk581_dn0 = assign18850_e26376_d_n0;
        locals.var_t4__blk581_dn2 = assign18850_e26376_d_n2;
        locals.var_t4__blk581_dn6 = assign18850_e26376_d_n6;
        locals.var_t4__blk581_dn7 = assign18850_e26376_d_n7;
        locals.var_t4__blk581_dn10 = assign18850_e26376_d_n10;
        locals.var_t4__blk581_dn11 = assign18850_e26376_d_n11;
        locals.var_t4__blk581_dn12 = assign18850_e26376_d_n12;
        locals.var_t4__blk581_dn17 = assign18850_e26376_d_n17;
        locals.var_t4__blk581_rv = 0.0;

        let (assign18860_e26384, assign18860_e26384_d_n0, assign18860_e26384_d_n2, assign18860_e26384_d_n6, assign18860_e26384_d_n7, assign18860_e26384_d_n10, assign18860_e26384_d_n11, assign18860_e26384_d_n12, assign18860_e26384_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18860_e26382: f64 = (4.0 / locals.var_costi5);
        (assign18860_e26382, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn12) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn17) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5__blk582, locals.var_t5__blk582_dn0, locals.var_t5__blk582_dn2, locals.var_t5__blk582_dn6, locals.var_t5__blk582_dn7, locals.var_t5__blk582_dn10, locals.var_t5__blk582_dn11, locals.var_t5__blk582_dn12, locals.var_t5__blk582_dn17,)
    }
};
        locals.var_t5__blk582 = assign18860_e26384;
        locals.var_t5__blk582_dn0 = assign18860_e26384_d_n0;
        locals.var_t5__blk582_dn2 = assign18860_e26384_d_n2;
        locals.var_t5__blk582_dn6 = assign18860_e26384_d_n6;
        locals.var_t5__blk582_dn7 = assign18860_e26384_d_n7;
        locals.var_t5__blk582_dn10 = assign18860_e26384_d_n10;
        locals.var_t5__blk582_dn11 = assign18860_e26384_d_n11;
        locals.var_t5__blk582_dn12 = assign18860_e26384_d_n12;
        locals.var_t5__blk582_dn17 = assign18860_e26384_d_n17;
        locals.var_t5__blk582_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign18870_e26394, assign18870_e26394_d_n0, assign18870_e26394_d_n2, assign18870_e26394_d_n6, assign18870_e26394_d_n7, assign18870_e26394_d_n10, assign18870_e26394_d_n11, assign18870_e26394_d_n12, assign18870_e26394_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18870_e26391: f64 = (locals.var_t4__blk581 * locals.var_t5__blk582);
        let assign18870_e26392: f64 = (1.0 + assign18870_e26391);
        (assign18870_e26392, ((locals.var_t4__blk581_dn0 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn0)), ((locals.var_t4__blk581_dn2 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn2)), ((locals.var_t4__blk581_dn6 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn6)), ((locals.var_t4__blk581_dn7 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn7)), ((locals.var_t4__blk581_dn10 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn10)), ((locals.var_t4__blk581_dn11 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn11)), ((locals.var_t4__blk581_dn12 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn12)), ((locals.var_t4__blk581_dn17 * locals.var_t5__blk582) + (locals.var_t4__blk581 * locals.var_t5__blk582_dn17)),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18870_e26394;
        locals.var_t1w_dn0 = assign18870_e26394_d_n0;
        locals.var_t1w_dn2 = assign18870_e26394_d_n2;
        locals.var_t1w_dn6 = assign18870_e26394_d_n6;
        locals.var_t1w_dn7 = assign18870_e26394_d_n7;
        locals.var_t1w_dn10 = assign18870_e26394_d_n10;
        locals.var_t1w_dn11 = assign18870_e26394_d_n11;
        locals.var_t1w_dn12 = assign18870_e26394_d_n12;
        locals.var_t1w_dn17 = assign18870_e26394_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign18880_e26409, assign18880_e26409_d_n0, assign18880_e26409_d_n2, assign18880_e26409_d_n6, assign18880_e26409_d_n7, assign18880_e26409_d_n10, assign18880_e26409_d_n11, assign18880_e26409_d_n12, assign18880_e26409_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18880_e26400: f64 = (locals.var_t1w * locals.var_t1w);
        let assign18880_e26403: f64 = (4.0 * 0.01);
        let assign18880_e26405: f64 = (assign18880_e26403 * 0.01);
        let assign18880_e26406: f64 = (assign18880_e26400 + assign18880_e26405);
        let assign18880_e26407: f64 = (assign18880_e26406).sqrt();
        (assign18880_e26407, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign18880_e26407)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign18880_e26407)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign18880_e26407)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign18880_e26407)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign18880_e26407)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign18880_e26407)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign18880_e26407)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign18880_e26407)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18880_e26409;
        locals.var_tmf1_dn0 = assign18880_e26409_d_n0;
        locals.var_tmf1_dn2 = assign18880_e26409_d_n2;
        locals.var_tmf1_dn6 = assign18880_e26409_d_n6;
        locals.var_tmf1_dn7 = assign18880_e26409_d_n7;
        locals.var_tmf1_dn10 = assign18880_e26409_d_n10;
        locals.var_tmf1_dn11 = assign18880_e26409_d_n11;
        locals.var_tmf1_dn12 = assign18880_e26409_d_n12;
        locals.var_tmf1_dn17 = assign18880_e26409_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18890_e26423, assign18890_e26423_d_n0, assign18890_e26423_d_n2, assign18890_e26423_d_n6, assign18890_e26423_d_n7, assign18890_e26423_d_n10, assign18890_e26423_d_n11, assign18890_e26423_d_n12, assign18890_e26423_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18890_e26416: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign18890_e26417: f64 = (0.5 * assign18890_e26416);
        let assign18890_e26420: f64 = (1e-10 * 0.01);
        let assign18890_e26421: f64 = (assign18890_e26417 + assign18890_e26420);
        (assign18890_e26421, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign18890_e26423;
        locals.var_t1__blk577_dn0 = assign18890_e26423_d_n0;
        locals.var_t1__blk577_dn2 = assign18890_e26423_d_n2;
        locals.var_t1__blk577_dn6 = assign18890_e26423_d_n6;
        locals.var_t1__blk577_dn7 = assign18890_e26423_d_n7;
        locals.var_t1__blk577_dn10 = assign18890_e26423_d_n10;
        locals.var_t1__blk577_dn11 = assign18890_e26423_d_n11;
        locals.var_t1__blk577_dn12 = assign18890_e26423_d_n12;
        locals.var_t1__blk577_dn17 = assign18890_e26423_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let assign18900_e26426: f64 = if locals.var_t1__blk577 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign18900_e26426;
        locals.var_guard587_rv = 0.0;

        let (assign18910_e26434, assign18910_e26434_d_n0, assign18910_e26434_d_n2, assign18910_e26434_d_n6, assign18910_e26434_d_n7, assign18910_e26434_d_n10, assign18910_e26434_d_n11, assign18910_e26434_d_n12, assign18910_e26434_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign18910_e26434;
        locals.var_t1__blk577_dn0 = assign18910_e26434_d_n0;
        locals.var_t1__blk577_dn2 = assign18910_e26434_d_n2;
        locals.var_t1__blk577_dn6 = assign18910_e26434_d_n6;
        locals.var_t1__blk577_dn7 = assign18910_e26434_d_n7;
        locals.var_t1__blk577_dn10 = assign18910_e26434_d_n10;
        locals.var_t1__blk577_dn11 = assign18910_e26434_d_n11;
        locals.var_t1__blk577_dn12 = assign18910_e26434_d_n12;
        locals.var_t1__blk577_dn17 = assign18910_e26434_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let (assign18920_e26443, assign18920_e26443_d_n0, assign18920_e26443_d_n2, assign18920_e26443_d_n6, assign18920_e26443_d_n7, assign18920_e26443_d_n10, assign18920_e26443_d_n11, assign18920_e26443_d_n12, assign18920_e26443_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18920_e26440: f64 = (locals.var_t1__blk577 + 1e-50);
        let assign18920_e26441: f64 = (assign18920_e26440).sqrt();
        (assign18920_e26441, (locals.var_t1__blk577_dn0 / (2.0 * assign18920_e26441)), (locals.var_t1__blk577_dn2 / (2.0 * assign18920_e26441)), (locals.var_t1__blk577_dn6 / (2.0 * assign18920_e26441)), (locals.var_t1__blk577_dn7 / (2.0 * assign18920_e26441)), (locals.var_t1__blk577_dn10 / (2.0 * assign18920_e26441)), (locals.var_t1__blk577_dn11 / (2.0 * assign18920_e26441)), (locals.var_t1__blk577_dn12 / (2.0 * assign18920_e26441)), (locals.var_t1__blk577_dn17 / (2.0 * assign18920_e26441)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn12, locals.var_costi6_dn17,)
    }
};
        locals.var_costi6 = assign18920_e26443;
        locals.var_costi6_dn0 = assign18920_e26443_d_n0;
        locals.var_costi6_dn2 = assign18920_e26443_d_n2;
        locals.var_costi6_dn6 = assign18920_e26443_d_n6;
        locals.var_costi6_dn7 = assign18920_e26443_d_n7;
        locals.var_costi6_dn10 = assign18920_e26443_d_n10;
        locals.var_costi6_dn11 = assign18920_e26443_d_n11;
        locals.var_costi6_dn12 = assign18920_e26443_d_n12;
        locals.var_costi6_dn17 = assign18920_e26443_d_n17;
        locals.var_costi6_rv = 0.0;

        let (assign18930_e26455, assign18930_e26455_d_n0, assign18930_e26455_d_n2, assign18930_e26455_d_n6, assign18930_e26455_d_n7, assign18930_e26455_d_n10, assign18930_e26455_d_n11, assign18930_e26455_d_n12, assign18930_e26455_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18930_e26451: f64 = (1.0 - locals.var_costi6);
        let assign18930_e26452: f64 = (locals.var_costi4 * assign18930_e26451);
        let assign18930_e26453: f64 = (locals.var_vgssti + assign18930_e26452);
        (assign18930_e26453, (locals.var_vgssti_dn0 + ((locals.var_costi4_dn0 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn0)))), (locals.var_vgssti_dn2 + ((locals.var_costi4_dn2 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn2)))), (locals.var_vgssti_dn6 + ((locals.var_costi4_dn6 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn6)))), (locals.var_vgssti_dn7 + ((locals.var_costi4_dn7 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn7)))), (locals.var_vgssti_dn10 + ((locals.var_costi4_dn10 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn10)))), (locals.var_vgssti_dn11 + ((locals.var_costi4_dn11 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn11)))), (locals.var_vgssti_dn12 + ((locals.var_costi4_dn12 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn12)))), (locals.var_vgssti_dn17 + ((locals.var_costi4_dn17 * assign18930_e26451) + (locals.var_costi4 * (-locals.var_costi6_dn17)))),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn12, locals.var_psasti_dn17,)
    }
};
        locals.var_psasti = assign18930_e26455;
        locals.var_psasti_dn0 = assign18930_e26455_d_n0;
        locals.var_psasti_dn2 = assign18930_e26455_d_n2;
        locals.var_psasti_dn6 = assign18930_e26455_d_n6;
        locals.var_psasti_dn7 = assign18930_e26455_d_n7;
        locals.var_psasti_dn10 = assign18930_e26455_d_n10;
        locals.var_psasti_dn11 = assign18930_e26455_d_n11;
        locals.var_psasti_dn12 = assign18930_e26455_d_n12;
        locals.var_psasti_dn17 = assign18930_e26455_d_n17;
        locals.var_psasti_rv = 0.0;

        let (assign18940_e26469, assign18940_e26469_d_n0, assign18940_e26469_d_n2, assign18940_e26469_d_n6, assign18940_e26469_d_n7, assign18940_e26469_d_n10, assign18940_e26469_d_n11, assign18940_e26469_d_n12, assign18940_e26469_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18940_e26464: f64 = (locals.var_vgssti + 1e-50);
        let assign18940_e26465: f64 = (2.0 / assign18940_e26464);
        let assign18940_e26466: f64 = (locals.var_beta + assign18940_e26465);
        let assign18940_e26467: f64 = (1.0 / assign18940_e26466);
        (assign18940_e26467, (-((-((2.0 * locals.var_vgssti_dn0) / (assign18940_e26464 * assign18940_e26464))) / (assign18940_e26466 * assign18940_e26466))), (-((-((2.0 * locals.var_vgssti_dn2) / (assign18940_e26464 * assign18940_e26464))) / (assign18940_e26466 * assign18940_e26466))), (-((-((2.0 * locals.var_vgssti_dn6) / (assign18940_e26464 * assign18940_e26464))) / (assign18940_e26466 * assign18940_e26466))), (-((-((2.0 * locals.var_vgssti_dn7) / (assign18940_e26464 * assign18940_e26464))) / (assign18940_e26466 * assign18940_e26466))), (-((locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (assign18940_e26464 * assign18940_e26464)))) / (assign18940_e26466 * assign18940_e26466))), (-((-((2.0 * locals.var_vgssti_dn11) / (assign18940_e26464 * assign18940_e26464))) / (assign18940_e26466 * assign18940_e26466))), (-((-((2.0 * locals.var_vgssti_dn12) / (assign18940_e26464 * assign18940_e26464))) / (assign18940_e26466 * assign18940_e26466))), (-((-((2.0 * locals.var_vgssti_dn17) / (assign18940_e26464 * assign18940_e26464))) / (assign18940_e26466 * assign18940_e26466))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18940_e26469;
        locals.var_t0_dn0 = assign18940_e26469_d_n0;
        locals.var_t0_dn2 = assign18940_e26469_d_n2;
        locals.var_t0_dn6 = assign18940_e26469_d_n6;
        locals.var_t0_dn7 = assign18940_e26469_d_n7;
        locals.var_t0_dn10 = assign18940_e26469_d_n10;
        locals.var_t0_dn11 = assign18940_e26469_d_n11;
        locals.var_t0_dn12 = assign18940_e26469_d_n12;
        locals.var_t0_dn17 = assign18940_e26469_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18950_e26486, assign18950_e26486_d_n0, assign18950_e26486_d_n2, assign18950_e26486_d_n6, assign18950_e26486_d_n7, assign18950_e26486_d_n10, assign18950_e26486_d_n11, assign18950_e26486_d_n12, assign18950_e26486_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18950_e26475: f64 = (1.0 / locals.var_costi1);
        let assign18950_e26477: f64 = (assign18950_e26475 / locals.var_costi3);
        let assign18950_e26480: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign18950_e26481: f64 = (assign18950_e26477 * assign18950_e26480);
        let assign18950_e26482: f64 = (assign18950_e26481).ln();
        let assign18950_e26484: f64 = (assign18950_e26482 * locals.var_t0);
        (assign18950_e26484, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn7)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn11)), (((((((((-(locals.var_costi1_dn12 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn12)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn12 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn12)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn12)), (((((((((-(locals.var_costi1_dn17 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18950_e26475 * locals.var_costi3_dn17)) / (locals.var_costi3 * locals.var_costi3)) * assign18950_e26480) + (assign18950_e26477 * ((locals.var_vgssti_dn17 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn17)))) / assign18950_e26481) * locals.var_t0) + (assign18950_e26482 * locals.var_t0_dn17)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn12, locals.var_psbsti_dn17,)
    }
};
        locals.var_psbsti = assign18950_e26486;
        locals.var_psbsti_dn0 = assign18950_e26486_d_n0;
        locals.var_psbsti_dn2 = assign18950_e26486_d_n2;
        locals.var_psbsti_dn6 = assign18950_e26486_d_n6;
        locals.var_psbsti_dn7 = assign18950_e26486_d_n7;
        locals.var_psbsti_dn10 = assign18950_e26486_d_n10;
        locals.var_psbsti_dn11 = assign18950_e26486_d_n11;
        locals.var_psbsti_dn12 = assign18950_e26486_d_n12;
        locals.var_psbsti_dn17 = assign18950_e26486_d_n17;
        locals.var_psbsti_rv = 0.0;

        let (assign18960_e26496, assign18960_e26496_d_n0, assign18960_e26496_d_n2, assign18960_e26496_d_n6, assign18960_e26496_d_n7, assign18960_e26496_d_n10, assign18960_e26496_d_n11, assign18960_e26496_d_n12, assign18960_e26496_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18960_e26493: f64 = (locals.var_vgssti + 1e-50);
        let assign18960_e26494: f64 = (locals.var_psbsti / assign18960_e26493);
        (assign18960_e26494, (((locals.var_psbsti_dn0 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn0)) / (assign18960_e26493 * assign18960_e26493)), (((locals.var_psbsti_dn2 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn2)) / (assign18960_e26493 * assign18960_e26493)), (((locals.var_psbsti_dn6 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn6)) / (assign18960_e26493 * assign18960_e26493)), (((locals.var_psbsti_dn7 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn7)) / (assign18960_e26493 * assign18960_e26493)), (((locals.var_psbsti_dn10 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn10)) / (assign18960_e26493 * assign18960_e26493)), (((locals.var_psbsti_dn11 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn11)) / (assign18960_e26493 * assign18960_e26493)), (((locals.var_psbsti_dn12 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn12)) / (assign18960_e26493 * assign18960_e26493)), (((locals.var_psbsti_dn17 * assign18960_e26493) - (locals.var_psbsti * locals.var_vgssti_dn17)) / (assign18960_e26493 * assign18960_e26493)),)
    } else {
        (locals.var_t3__blk580, locals.var_t3__blk580_dn0, locals.var_t3__blk580_dn2, locals.var_t3__blk580_dn6, locals.var_t3__blk580_dn7, locals.var_t3__blk580_dn10, locals.var_t3__blk580_dn11, locals.var_t3__blk580_dn12, locals.var_t3__blk580_dn17,)
    }
};
        locals.var_t3__blk580 = assign18960_e26496;
        locals.var_t3__blk580_dn0 = assign18960_e26496_d_n0;
        locals.var_t3__blk580_dn2 = assign18960_e26496_d_n2;
        locals.var_t3__blk580_dn6 = assign18960_e26496_d_n6;
        locals.var_t3__blk580_dn7 = assign18960_e26496_d_n7;
        locals.var_t3__blk580_dn10 = assign18960_e26496_d_n10;
        locals.var_t3__blk580_dn11 = assign18960_e26496_d_n11;
        locals.var_t3__blk580_dn12 = assign18960_e26496_d_n12;
        locals.var_t3__blk580_dn17 = assign18960_e26496_d_n17;
        locals.var_t3__blk580_rv = 0.0;

        let (assign18970_e26506, assign18970_e26506_d_n0, assign18970_e26506_d_n2, assign18970_e26506_d_n6, assign18970_e26506_d_n7, assign18970_e26506_d_n10, assign18970_e26506_d_n11, assign18970_e26506_d_n12, assign18970_e26506_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18970_e26502: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign18970_e26504: f64 = (assign18970_e26502 - 0.002);
        (assign18970_e26504, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn12 - locals.var_psasti_dn12), (locals.var_psbsti_dn17 - locals.var_psasti_dn17),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn12, locals.var_psab_dn17,)
    }
};
        locals.var_psab = assign18970_e26506;
        locals.var_psab_dn0 = assign18970_e26506_d_n0;
        locals.var_psab_dn2 = assign18970_e26506_d_n2;
        locals.var_psab_dn6 = assign18970_e26506_d_n6;
        locals.var_psab_dn7 = assign18970_e26506_d_n7;
        locals.var_psab_dn10 = assign18970_e26506_d_n10;
        locals.var_psab_dn11 = assign18970_e26506_d_n11;
        locals.var_psab_dn12 = assign18970_e26506_d_n12;
        locals.var_psab_dn17 = assign18970_e26506_d_n17;
        locals.var_psab_rv = 0.0;

        let (assign18980_e26521, assign18980_e26521_d_n0, assign18980_e26521_d_n2, assign18980_e26521_d_n6, assign18980_e26521_d_n7, assign18980_e26521_d_n10, assign18980_e26521_d_n11, assign18980_e26521_d_n12, assign18980_e26521_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18980_e26512: f64 = (locals.var_psab * locals.var_psab);
        let assign18980_e26515: f64 = (4.0 * 0.002);
        let assign18980_e26517: f64 = (assign18980_e26515 * locals.var_psbsti);
        let assign18980_e26518: f64 = (assign18980_e26512 + assign18980_e26517);
        let assign18980_e26519: f64 = (assign18980_e26518).sqrt();
        (assign18980_e26519, ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign18980_e26515 * locals.var_psbsti_dn0)) / (2.0 * assign18980_e26519)), ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign18980_e26515 * locals.var_psbsti_dn2)) / (2.0 * assign18980_e26519)), ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign18980_e26515 * locals.var_psbsti_dn6)) / (2.0 * assign18980_e26519)), ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign18980_e26515 * locals.var_psbsti_dn7)) / (2.0 * assign18980_e26519)), ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign18980_e26515 * locals.var_psbsti_dn10)) / (2.0 * assign18980_e26519)), ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign18980_e26515 * locals.var_psbsti_dn11)) / (2.0 * assign18980_e26519)), ((((locals.var_psab_dn12 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn12)) + (assign18980_e26515 * locals.var_psbsti_dn12)) / (2.0 * assign18980_e26519)), ((((locals.var_psab_dn17 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn17)) + (assign18980_e26515 * locals.var_psbsti_dn17)) / (2.0 * assign18980_e26519)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18980_e26521;
        locals.var_t0_dn0 = assign18980_e26521_d_n0;
        locals.var_t0_dn2 = assign18980_e26521_d_n2;
        locals.var_t0_dn6 = assign18980_e26521_d_n6;
        locals.var_t0_dn7 = assign18980_e26521_d_n7;
        locals.var_t0_dn10 = assign18980_e26521_d_n10;
        locals.var_t0_dn11 = assign18980_e26521_d_n11;
        locals.var_t0_dn12 = assign18980_e26521_d_n12;
        locals.var_t0_dn17 = assign18980_e26521_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18990_e26533, assign18990_e26533_d_n0, assign18990_e26533_d_n2, assign18990_e26533_d_n6, assign18990_e26533_d_n7, assign18990_e26533_d_n10, assign18990_e26533_d_n11, assign18990_e26533_d_n12, assign18990_e26533_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign18990_e26529: f64 = (locals.var_psab + locals.var_t0);
        let assign18990_e26530: f64 = (0.5 * assign18990_e26529);
        let assign18990_e26531: f64 = (locals.var_psbsti - assign18990_e26530);
        (assign18990_e26531, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + locals.var_t0_dn7))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + locals.var_t0_dn11))), (locals.var_psbsti_dn12 - (0.5 * (locals.var_psab_dn12 + locals.var_t0_dn12))), (locals.var_psbsti_dn17 - (0.5 * (locals.var_psab_dn17 + locals.var_t0_dn17))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    }
};
        locals.var_psti = assign18990_e26533;
        locals.var_psti_dn0 = assign18990_e26533_d_n0;
        locals.var_psti_dn2 = assign18990_e26533_d_n2;
        locals.var_psti_dn6 = assign18990_e26533_d_n6;
        locals.var_psti_dn7 = assign18990_e26533_d_n7;
        locals.var_psti_dn10 = assign18990_e26533_d_n10;
        locals.var_psti_dn11 = assign18990_e26533_d_n11;
        locals.var_psti_dn12 = assign18990_e26533_d_n12;
        locals.var_psti_dn17 = assign18990_e26533_d_n17;
        locals.var_psti_rv = 0.0;

        let (assign19000_e26541, assign19000_e26541_d_n0, assign19000_e26541_d_n2, assign19000_e26541_d_n6, assign19000_e26541_d_n7, assign19000_e26541_d_n10, assign19000_e26541_d_n11, assign19000_e26541_d_n12, assign19000_e26541_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19000_e26539: f64 = (1.0 / locals.var_t0);
        (assign19000_e26539, (-(locals.var_t0_dn0 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn2 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn12 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn17 / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign19000_e26541;
        locals.var_t1__blk577_dn0 = assign19000_e26541_d_n0;
        locals.var_t1__blk577_dn2 = assign19000_e26541_d_n2;
        locals.var_t1__blk577_dn6 = assign19000_e26541_d_n6;
        locals.var_t1__blk577_dn7 = assign19000_e26541_d_n7;
        locals.var_t1__blk577_dn10 = assign19000_e26541_d_n10;
        locals.var_t1__blk577_dn11 = assign19000_e26541_d_n11;
        locals.var_t1__blk577_dn12 = assign19000_e26541_d_n12;
        locals.var_t1__blk577_dn17 = assign19000_e26541_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let (assign19010_e26552, assign19010_e26552_d_n0, assign19010_e26552_d_n2, assign19010_e26552_d_n6, assign19010_e26552_d_n7, assign19010_e26552_d_n10, assign19010_e26552_d_n11, assign19010_e26552_d_n12, assign19010_e26552_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19010_e26548: f64 = (locals.var_beta * locals.var_psti);
        let assign19010_e26549: f64 = (assign19010_e26548).exp();
        let assign19010_e26550: f64 = (locals.var_costi1 * assign19010_e26549);
        (assign19010_e26550, ((locals.var_costi1_dn0 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * (locals.var_beta * locals.var_psti_dn0)))), ((locals.var_costi1_dn2 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * (locals.var_beta * locals.var_psti_dn2)))), ((locals.var_costi1_dn6 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * (locals.var_beta * locals.var_psti_dn6)))), ((locals.var_costi1_dn7 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * (locals.var_beta * locals.var_psti_dn7)))), ((locals.var_costi1_dn10 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))))), ((locals.var_costi1_dn11 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * (locals.var_beta * locals.var_psti_dn11)))), ((locals.var_costi1_dn12 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * (locals.var_beta * locals.var_psti_dn12)))), ((locals.var_costi1_dn17 * assign19010_e26549) + (locals.var_costi1 * (assign19010_e26549 * (locals.var_beta * locals.var_psti_dn17)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign19010_e26552;
        locals.var_t0_dn0 = assign19010_e26552_d_n0;
        locals.var_t0_dn2 = assign19010_e26552_d_n2;
        locals.var_t0_dn6 = assign19010_e26552_d_n6;
        locals.var_t0_dn7 = assign19010_e26552_d_n7;
        locals.var_t0_dn10 = assign19010_e26552_d_n10;
        locals.var_t0_dn11 = assign19010_e26552_d_n11;
        locals.var_t0_dn12 = assign19010_e26552_d_n12;
        locals.var_t0_dn17 = assign19010_e26552_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign19020_e26566, assign19020_e26566_d_n0, assign19020_e26566_d_n2, assign19020_e26566_d_n6, assign19020_e26566_d_n7, assign19020_e26566_d_n10, assign19020_e26566_d_n11, assign19020_e26566_d_n12, assign19020_e26566_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19020_e26559: f64 = (locals.var_psti - locals.var_vbspz);
        let assign19020_e26560: f64 = (locals.var_beta * assign19020_e26559);
        let assign19020_e26562: f64 = (assign19020_e26560 - 1.0);
        let assign19020_e26564: f64 = (assign19020_e26562 + locals.var_t0);
        (assign19020_e26564, ((locals.var_beta * (locals.var_psti_dn0 - locals.var_vbspz_dn0)) + locals.var_t0_dn0), ((locals.var_beta * (locals.var_psti_dn2 - locals.var_vbspz_dn2)) + locals.var_t0_dn2), ((locals.var_beta * (locals.var_psti_dn6 - locals.var_vbspz_dn6)) + locals.var_t0_dn6), ((locals.var_beta * (locals.var_psti_dn7 - locals.var_vbspz_dn7)) + locals.var_t0_dn7), (((locals.var_beta_dn10 * assign19020_e26559) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbspz_dn10))) + locals.var_t0_dn10), ((locals.var_beta * (locals.var_psti_dn11 - locals.var_vbspz_dn11)) + locals.var_t0_dn11), ((locals.var_beta * (locals.var_psti_dn12 - locals.var_vbspz_dn12)) + locals.var_t0_dn12), ((locals.var_beta * (locals.var_psti_dn17 - locals.var_vbspz_dn17)) + locals.var_t0_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19020_e26566;
        locals.var_t1w_dn0 = assign19020_e26566_d_n0;
        locals.var_t1w_dn2 = assign19020_e26566_d_n2;
        locals.var_t1w_dn6 = assign19020_e26566_d_n6;
        locals.var_t1w_dn7 = assign19020_e26566_d_n7;
        locals.var_t1w_dn10 = assign19020_e26566_d_n10;
        locals.var_t1w_dn11 = assign19020_e26566_d_n11;
        locals.var_t1w_dn12 = assign19020_e26566_d_n12;
        locals.var_t1w_dn17 = assign19020_e26566_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19030_e26581, assign19030_e26581_d_n0, assign19030_e26581_d_n2, assign19030_e26581_d_n6, assign19030_e26581_d_n7, assign19030_e26581_d_n10, assign19030_e26581_d_n11, assign19030_e26581_d_n12, assign19030_e26581_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19030_e26572: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19030_e26575: f64 = (4.0 * 0.01);
        let assign19030_e26577: f64 = (assign19030_e26575 * 0.01);
        let assign19030_e26578: f64 = (assign19030_e26572 + assign19030_e26577);
        let assign19030_e26579: f64 = (assign19030_e26578).sqrt();
        (assign19030_e26579, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19030_e26579)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19030_e26579)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19030_e26579)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19030_e26579)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19030_e26579)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19030_e26579)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19030_e26579)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19030_e26579)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19030_e26581;
        locals.var_tmf1_dn0 = assign19030_e26581_d_n0;
        locals.var_tmf1_dn2 = assign19030_e26581_d_n2;
        locals.var_tmf1_dn6 = assign19030_e26581_d_n6;
        locals.var_tmf1_dn7 = assign19030_e26581_d_n7;
        locals.var_tmf1_dn10 = assign19030_e26581_d_n10;
        locals.var_tmf1_dn11 = assign19030_e26581_d_n11;
        locals.var_tmf1_dn12 = assign19030_e26581_d_n12;
        locals.var_tmf1_dn17 = assign19030_e26581_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19040_e26595, assign19040_e26595_d_n0, assign19040_e26595_d_n2, assign19040_e26595_d_n6, assign19040_e26595_d_n7, assign19040_e26595_d_n10, assign19040_e26595_d_n11, assign19040_e26595_d_n12, assign19040_e26595_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19040_e26588: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19040_e26589: f64 = (0.5 * assign19040_e26588);
        let assign19040_e26592: f64 = (1e-10 * 0.01);
        let assign19040_e26593: f64 = (assign19040_e26589 + assign19040_e26592);
        (assign19040_e26593, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign19040_e26595;
        locals.var_t1__blk577_dn0 = assign19040_e26595_d_n0;
        locals.var_t1__blk577_dn2 = assign19040_e26595_d_n2;
        locals.var_t1__blk577_dn6 = assign19040_e26595_d_n6;
        locals.var_t1__blk577_dn7 = assign19040_e26595_d_n7;
        locals.var_t1__blk577_dn10 = assign19040_e26595_d_n10;
        locals.var_t1__blk577_dn11 = assign19040_e26595_d_n11;
        locals.var_t1__blk577_dn12 = assign19040_e26595_d_n12;
        locals.var_t1__blk577_dn17 = assign19040_e26595_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let assign19050_e26598: f64 = if locals.var_t1__blk577 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign19050_e26598;
        locals.var_guard588_rv = 0.0;

        let (assign19060_e26606, assign19060_e26606_d_n0, assign19060_e26606_d_n2, assign19060_e26606_d_n6, assign19060_e26606_d_n7, assign19060_e26606_d_n10, assign19060_e26606_d_n11, assign19060_e26606_d_n12, assign19060_e26606_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard588 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign19060_e26606;
        locals.var_t1__blk577_dn0 = assign19060_e26606_d_n0;
        locals.var_t1__blk577_dn2 = assign19060_e26606_d_n2;
        locals.var_t1__blk577_dn6 = assign19060_e26606_d_n6;
        locals.var_t1__blk577_dn7 = assign19060_e26606_d_n7;
        locals.var_t1__blk577_dn10 = assign19060_e26606_d_n10;
        locals.var_t1__blk577_dn11 = assign19060_e26606_d_n11;
        locals.var_t1__blk577_dn12 = assign19060_e26606_d_n12;
        locals.var_t1__blk577_dn17 = assign19060_e26606_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let (assign19070_e26617, assign19070_e26617_d_n0, assign19070_e26617_d_n2, assign19070_e26617_d_n6, assign19070_e26617_d_n7, assign19070_e26617_d_n10, assign19070_e26617_d_n11, assign19070_e26617_d_n12, assign19070_e26617_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19070_e26613: f64 = (10.0 * 2.220446049250313e-16);
        let assign19070_e26614: f64 = (locals.var_t1__blk577 + assign19070_e26613);
        let assign19070_e26615: f64 = (assign19070_e26614).sqrt();
        (assign19070_e26615, (locals.var_t1__blk577_dn0 / (2.0 * assign19070_e26615)), (locals.var_t1__blk577_dn2 / (2.0 * assign19070_e26615)), (locals.var_t1__blk577_dn6 / (2.0 * assign19070_e26615)), (locals.var_t1__blk577_dn7 / (2.0 * assign19070_e26615)), (locals.var_t1__blk577_dn10 / (2.0 * assign19070_e26615)), (locals.var_t1__blk577_dn11 / (2.0 * assign19070_e26615)), (locals.var_t1__blk577_dn12 / (2.0 * assign19070_e26615)), (locals.var_t1__blk577_dn17 / (2.0 * assign19070_e26615)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12, locals.var_sq1sti_dn17,)
    }
};
        locals.var_sq1sti = assign19070_e26617;
        locals.var_sq1sti_dn0 = assign19070_e26617_d_n0;
        locals.var_sq1sti_dn2 = assign19070_e26617_d_n2;
        locals.var_sq1sti_dn6 = assign19070_e26617_d_n6;
        locals.var_sq1sti_dn7 = assign19070_e26617_d_n7;
        locals.var_sq1sti_dn10 = assign19070_e26617_d_n10;
        locals.var_sq1sti_dn11 = assign19070_e26617_d_n11;
        locals.var_sq1sti_dn12 = assign19070_e26617_d_n12;
        locals.var_sq1sti_dn17 = assign19070_e26617_d_n17;
        locals.var_sq1sti_rv = 0.0;

        let (assign19080_e26629, assign19080_e26629_d_n0, assign19080_e26629_d_n2, assign19080_e26629_d_n6, assign19080_e26629_d_n7, assign19080_e26629_d_n10, assign19080_e26629_d_n11, assign19080_e26629_d_n12, assign19080_e26629_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19080_e26624: f64 = (locals.var_psti - locals.var_vbspz);
        let assign19080_e26625: f64 = (locals.var_beta * assign19080_e26624);
        let assign19080_e26627: f64 = (assign19080_e26625 - 1.0);
        (assign19080_e26627, (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbspz_dn0)), (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbspz_dn2)), (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbspz_dn6)), (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbspz_dn7)), ((locals.var_beta_dn10 * assign19080_e26624) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbspz_dn10))), (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbspz_dn11)), (locals.var_beta * (locals.var_psti_dn12 - locals.var_vbspz_dn12)), (locals.var_beta * (locals.var_psti_dn17 - locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19080_e26629;
        locals.var_t1w_dn0 = assign19080_e26629_d_n0;
        locals.var_t1w_dn2 = assign19080_e26629_d_n2;
        locals.var_t1w_dn6 = assign19080_e26629_d_n6;
        locals.var_t1w_dn7 = assign19080_e26629_d_n7;
        locals.var_t1w_dn10 = assign19080_e26629_d_n10;
        locals.var_t1w_dn11 = assign19080_e26629_d_n11;
        locals.var_t1w_dn12 = assign19080_e26629_d_n12;
        locals.var_t1w_dn17 = assign19080_e26629_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19090_e26644, assign19090_e26644_d_n0, assign19090_e26644_d_n2, assign19090_e26644_d_n6, assign19090_e26644_d_n7, assign19090_e26644_d_n10, assign19090_e26644_d_n11, assign19090_e26644_d_n12, assign19090_e26644_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19090_e26635: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19090_e26638: f64 = (4.0 * 0.01);
        let assign19090_e26640: f64 = (assign19090_e26638 * 0.01);
        let assign19090_e26641: f64 = (assign19090_e26635 + assign19090_e26640);
        let assign19090_e26642: f64 = (assign19090_e26641).sqrt();
        (assign19090_e26642, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19090_e26642)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19090_e26642)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19090_e26642)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19090_e26642)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19090_e26642)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19090_e26642)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19090_e26642)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19090_e26642)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19090_e26644;
        locals.var_tmf1_dn0 = assign19090_e26644_d_n0;
        locals.var_tmf1_dn2 = assign19090_e26644_d_n2;
        locals.var_tmf1_dn6 = assign19090_e26644_d_n6;
        locals.var_tmf1_dn7 = assign19090_e26644_d_n7;
        locals.var_tmf1_dn10 = assign19090_e26644_d_n10;
        locals.var_tmf1_dn11 = assign19090_e26644_d_n11;
        locals.var_tmf1_dn12 = assign19090_e26644_d_n12;
        locals.var_tmf1_dn17 = assign19090_e26644_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19100_e26658, assign19100_e26658_d_n0, assign19100_e26658_d_n2, assign19100_e26658_d_n6, assign19100_e26658_d_n7, assign19100_e26658_d_n10, assign19100_e26658_d_n11, assign19100_e26658_d_n12, assign19100_e26658_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19100_e26651: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19100_e26652: f64 = (0.5 * assign19100_e26651);
        let assign19100_e26655: f64 = (1e-10 * 0.01);
        let assign19100_e26656: f64 = (assign19100_e26652 + assign19100_e26655);
        (assign19100_e26656, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign19100_e26658;
        locals.var_t1__blk577_dn0 = assign19100_e26658_d_n0;
        locals.var_t1__blk577_dn2 = assign19100_e26658_d_n2;
        locals.var_t1__blk577_dn6 = assign19100_e26658_d_n6;
        locals.var_t1__blk577_dn7 = assign19100_e26658_d_n7;
        locals.var_t1__blk577_dn10 = assign19100_e26658_d_n10;
        locals.var_t1__blk577_dn11 = assign19100_e26658_d_n11;
        locals.var_t1__blk577_dn12 = assign19100_e26658_d_n12;
        locals.var_t1__blk577_dn17 = assign19100_e26658_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let assign19110_e26661: f64 = if locals.var_t1__blk577 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign19110_e26661;
        locals.var_guard589_rv = 0.0;

        let (assign19120_e26669, assign19120_e26669_d_n0, assign19120_e26669_d_n2, assign19120_e26669_d_n6, assign19120_e26669_d_n7, assign19120_e26669_d_n10, assign19120_e26669_d_n11, assign19120_e26669_d_n12, assign19120_e26669_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard589 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign19120_e26669;
        locals.var_t1__blk577_dn0 = assign19120_e26669_d_n0;
        locals.var_t1__blk577_dn2 = assign19120_e26669_d_n2;
        locals.var_t1__blk577_dn6 = assign19120_e26669_d_n6;
        locals.var_t1__blk577_dn7 = assign19120_e26669_d_n7;
        locals.var_t1__blk577_dn10 = assign19120_e26669_d_n10;
        locals.var_t1__blk577_dn11 = assign19120_e26669_d_n11;
        locals.var_t1__blk577_dn12 = assign19120_e26669_d_n12;
        locals.var_t1__blk577_dn17 = assign19120_e26669_d_n17;
        locals.var_t1__blk577_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign19130_e26680, assign19130_e26680_d_n0, assign19130_e26680_d_n2, assign19130_e26680_d_n6, assign19130_e26680_d_n7, assign19130_e26680_d_n10, assign19130_e26680_d_n11, assign19130_e26680_d_n12, assign19130_e26680_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19130_e26676: f64 = (10.0 * 2.220446049250313e-16);
        let assign19130_e26677: f64 = (locals.var_t1__blk577 + assign19130_e26676);
        let assign19130_e26678: f64 = (assign19130_e26677).sqrt();
        (assign19130_e26678, (locals.var_t1__blk577_dn0 / (2.0 * assign19130_e26678)), (locals.var_t1__blk577_dn2 / (2.0 * assign19130_e26678)), (locals.var_t1__blk577_dn6 / (2.0 * assign19130_e26678)), (locals.var_t1__blk577_dn7 / (2.0 * assign19130_e26678)), (locals.var_t1__blk577_dn10 / (2.0 * assign19130_e26678)), (locals.var_t1__blk577_dn11 / (2.0 * assign19130_e26678)), (locals.var_t1__blk577_dn12 / (2.0 * assign19130_e26678)), (locals.var_t1__blk577_dn17 / (2.0 * assign19130_e26678)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12, locals.var_sq2sti_dn17,)
    }
};
        locals.var_sq2sti = assign19130_e26680;
        locals.var_sq2sti_dn0 = assign19130_e26680_d_n0;
        locals.var_sq2sti_dn2 = assign19130_e26680_d_n2;
        locals.var_sq2sti_dn6 = assign19130_e26680_d_n6;
        locals.var_sq2sti_dn7 = assign19130_e26680_d_n7;
        locals.var_sq2sti_dn10 = assign19130_e26680_d_n10;
        locals.var_sq2sti_dn11 = assign19130_e26680_d_n11;
        locals.var_sq2sti_dn12 = assign19130_e26680_d_n12;
        locals.var_sq2sti_dn17 = assign19130_e26680_d_n17;
        locals.var_sq2sti_rv = 0.0;

        let (assign19140_e26690, assign19140_e26690_d_n0, assign19140_e26690_d_n2, assign19140_e26690_d_n6, assign19140_e26690_d_n7, assign19140_e26690_d_n10, assign19140_e26690_d_n11, assign19140_e26690_d_n12, assign19140_e26690_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19140_e26687: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign19140_e26688: f64 = (locals.var_costi0 * assign19140_e26687);
        (assign19140_e26688, ((locals.var_costi0_dn0 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn6 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn10 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn12 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn12 - locals.var_sq2sti_dn12))), ((locals.var_costi0_dn17 * assign19140_e26687) + (locals.var_costi0 * (locals.var_sq1sti_dn17 - locals.var_sq2sti_dn17))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn12, locals.var_qn0sti_dn17,)
    }
};
        locals.var_qn0sti = assign19140_e26690;
        locals.var_qn0sti_dn0 = assign19140_e26690_d_n0;
        locals.var_qn0sti_dn2 = assign19140_e26690_d_n2;
        locals.var_qn0sti_dn6 = assign19140_e26690_d_n6;
        locals.var_qn0sti_dn7 = assign19140_e26690_d_n7;
        locals.var_qn0sti_dn10 = assign19140_e26690_d_n10;
        locals.var_qn0sti_dn11 = assign19140_e26690_d_n11;
        locals.var_qn0sti_dn12 = assign19140_e26690_d_n12;
        locals.var_qn0sti_dn17 = assign19140_e26690_d_n17;
        locals.var_qn0sti_rv = 0.0;

        let (assign19150_e26698, assign19150_e26698_d_n0, assign19150_e26698_d_n2, assign19150_e26698_d_n6, assign19150_e26698_d_n7, assign19150_e26698_d_n10, assign19150_e26698_d_n11, assign19150_e26698_d_n12, assign19150_e26698_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19150_e26696: f64 = (locals.var_psasti - locals.var_psti);
        (assign19150_e26696, (locals.var_psasti_dn0 - locals.var_psti_dn0), (locals.var_psasti_dn2 - locals.var_psti_dn2), (locals.var_psasti_dn6 - locals.var_psti_dn6), (locals.var_psasti_dn7 - locals.var_psti_dn7), (locals.var_psasti_dn10 - locals.var_psti_dn10), (locals.var_psasti_dn11 - locals.var_psti_dn11), (locals.var_psasti_dn12 - locals.var_psti_dn12), (locals.var_psasti_dn17 - locals.var_psti_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19150_e26698;
        locals.var_t1w_dn0 = assign19150_e26698_d_n0;
        locals.var_t1w_dn2 = assign19150_e26698_d_n2;
        locals.var_t1w_dn6 = assign19150_e26698_d_n6;
        locals.var_t1w_dn7 = assign19150_e26698_d_n7;
        locals.var_t1w_dn10 = assign19150_e26698_d_n10;
        locals.var_t1w_dn11 = assign19150_e26698_d_n11;
        locals.var_t1w_dn12 = assign19150_e26698_d_n12;
        locals.var_t1w_dn17 = assign19150_e26698_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19160_e26713, assign19160_e26713_d_n0, assign19160_e26713_d_n2, assign19160_e26713_d_n6, assign19160_e26713_d_n7, assign19160_e26713_d_n10, assign19160_e26713_d_n11, assign19160_e26713_d_n12, assign19160_e26713_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19160_e26704: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19160_e26707: f64 = (4.0 * 0.1);
        let assign19160_e26709: f64 = (assign19160_e26707 * 0.1);
        let assign19160_e26710: f64 = (assign19160_e26704 + assign19160_e26709);
        let assign19160_e26711: f64 = (assign19160_e26710).sqrt();
        (assign19160_e26711, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19160_e26711)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19160_e26711)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19160_e26711)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19160_e26711)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19160_e26711)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19160_e26711)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19160_e26711)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19160_e26711)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19160_e26713;
        locals.var_tmf1_dn0 = assign19160_e26713_d_n0;
        locals.var_tmf1_dn2 = assign19160_e26713_d_n2;
        locals.var_tmf1_dn6 = assign19160_e26713_d_n6;
        locals.var_tmf1_dn7 = assign19160_e26713_d_n7;
        locals.var_tmf1_dn10 = assign19160_e26713_d_n10;
        locals.var_tmf1_dn11 = assign19160_e26713_d_n11;
        locals.var_tmf1_dn12 = assign19160_e26713_d_n12;
        locals.var_tmf1_dn17 = assign19160_e26713_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19170_e26727, assign19170_e26727_d_n0, assign19170_e26727_d_n2, assign19170_e26727_d_n6, assign19170_e26727_d_n7, assign19170_e26727_d_n10, assign19170_e26727_d_n11, assign19170_e26727_d_n12, assign19170_e26727_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19170_e26720: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19170_e26721: f64 = (0.5 * assign19170_e26720);
        let assign19170_e26724: f64 = (1e-10 * 0.1);
        let assign19170_e26725: f64 = (assign19170_e26721 + assign19170_e26724);
        (assign19170_e26725, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign19170_e26727;
        locals.var_t1__blk577_dn0 = assign19170_e26727_d_n0;
        locals.var_t1__blk577_dn2 = assign19170_e26727_d_n2;
        locals.var_t1__blk577_dn6 = assign19170_e26727_d_n6;
        locals.var_t1__blk577_dn7 = assign19170_e26727_d_n7;
        locals.var_t1__blk577_dn10 = assign19170_e26727_d_n10;
        locals.var_t1__blk577_dn11 = assign19170_e26727_d_n11;
        locals.var_t1__blk577_dn12 = assign19170_e26727_d_n12;
        locals.var_t1__blk577_dn17 = assign19170_e26727_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let assign19180_e26730: f64 = if locals.var_t1__blk577 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign19180_e26730;
        locals.var_guard590_rv = 0.0;

        let (assign19190_e26738, assign19190_e26738_d_n0, assign19190_e26738_d_n2, assign19190_e26738_d_n6, assign19190_e26738_d_n7, assign19190_e26738_d_n10, assign19190_e26738_d_n11, assign19190_e26738_d_n12, assign19190_e26738_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard590 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk577, locals.var_t1__blk577_dn0, locals.var_t1__blk577_dn2, locals.var_t1__blk577_dn6, locals.var_t1__blk577_dn7, locals.var_t1__blk577_dn10, locals.var_t1__blk577_dn11, locals.var_t1__blk577_dn12, locals.var_t1__blk577_dn17,)
    }
};
        locals.var_t1__blk577 = assign19190_e26738;
        locals.var_t1__blk577_dn0 = assign19190_e26738_d_n0;
        locals.var_t1__blk577_dn2 = assign19190_e26738_d_n2;
        locals.var_t1__blk577_dn6 = assign19190_e26738_d_n6;
        locals.var_t1__blk577_dn7 = assign19190_e26738_d_n7;
        locals.var_t1__blk577_dn10 = assign19190_e26738_d_n10;
        locals.var_t1__blk577_dn11 = assign19190_e26738_d_n11;
        locals.var_t1__blk577_dn12 = assign19190_e26738_d_n12;
        locals.var_t1__blk577_dn17 = assign19190_e26738_d_n17;
        locals.var_t1__blk577_rv = 0.0;

        let (assign19200_e26750, assign19200_e26750_d_n0, assign19200_e26750_d_n2, assign19200_e26750_d_n6, assign19200_e26750_d_n7, assign19200_e26750_d_n10, assign19200_e26750_d_n11, assign19200_e26750_d_n12, assign19200_e26750_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19200_e26746: f64 = (10.0 * 2.220446049250313e-16);
        let assign19200_e26747: f64 = (locals.var_t1__blk577 + assign19200_e26746);
        let assign19200_e26748: f64 = (locals.var_vds / assign19200_e26747);
        (assign19200_e26748, (((locals.var_vds_dn0 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn0)) / (assign19200_e26747 * assign19200_e26747)), (((locals.var_vds_dn2 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn2)) / (assign19200_e26747 * assign19200_e26747)), (((locals.var_vds_dn6 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn6)) / (assign19200_e26747 * assign19200_e26747)), (((locals.var_vds_dn7 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn7)) / (assign19200_e26747 * assign19200_e26747)), (((locals.var_vds_dn10 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn10)) / (assign19200_e26747 * assign19200_e26747)), (((locals.var_vds_dn11 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn11)) / (assign19200_e26747 * assign19200_e26747)), (((locals.var_vds_dn12 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn12)) / (assign19200_e26747 * assign19200_e26747)), (((locals.var_vds_dn17 * assign19200_e26747) - (locals.var_vds * locals.var_t1__blk577_dn17)) / (assign19200_e26747 * assign19200_e26747)),)
    } else {
        (locals.var_tx__blk584, locals.var_tx__blk584_dn0, locals.var_tx__blk584_dn2, locals.var_tx__blk584_dn6, locals.var_tx__blk584_dn7, locals.var_tx__blk584_dn10, locals.var_tx__blk584_dn11, locals.var_tx__blk584_dn12, locals.var_tx__blk584_dn17,)
    }
};
        locals.var_tx__blk584 = assign19200_e26750;
        locals.var_tx__blk584_dn0 = assign19200_e26750_d_n0;
        locals.var_tx__blk584_dn2 = assign19200_e26750_d_n2;
        locals.var_tx__blk584_dn6 = assign19200_e26750_d_n6;
        locals.var_tx__blk584_dn7 = assign19200_e26750_d_n7;
        locals.var_tx__blk584_dn10 = assign19200_e26750_d_n10;
        locals.var_tx__blk584_dn11 = assign19200_e26750_d_n11;
        locals.var_tx__blk584_dn12 = assign19200_e26750_d_n12;
        locals.var_tx__blk584_dn17 = assign19200_e26750_d_n17;
        locals.var_tx__blk584_rv = 0.0;

        let (assign19210_e26758, assign19210_e26758_d_n0, assign19210_e26758_d_n2, assign19210_e26758_d_n6, assign19210_e26758_d_n7, assign19210_e26758_d_n10, assign19210_e26758_d_n11, assign19210_e26758_d_n12, assign19210_e26758_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19210_e26756: f64 = (locals.var_tx__blk584 * locals.var_tx__blk584);
        (assign19210_e26756, ((locals.var_tx__blk584_dn0 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn0)), ((locals.var_tx__blk584_dn2 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn2)), ((locals.var_tx__blk584_dn6 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn6)), ((locals.var_tx__blk584_dn7 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn7)), ((locals.var_tx__blk584_dn10 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn10)), ((locals.var_tx__blk584_dn11 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn11)), ((locals.var_tx__blk584_dn12 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn12)), ((locals.var_tx__blk584_dn17 * locals.var_tx__blk584) + (locals.var_tx__blk584 * locals.var_tx__blk584_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign19210_e26758;
        locals.var_x2_dn0 = assign19210_e26758_d_n0;
        locals.var_x2_dn2 = assign19210_e26758_d_n2;
        locals.var_x2_dn6 = assign19210_e26758_d_n6;
        locals.var_x2_dn7 = assign19210_e26758_d_n7;
        locals.var_x2_dn10 = assign19210_e26758_d_n10;
        locals.var_x2_dn11 = assign19210_e26758_d_n11;
        locals.var_x2_dn12 = assign19210_e26758_d_n12;
        locals.var_x2_dn17 = assign19210_e26758_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign19220_e26766, assign19220_e26766_d_n0, assign19220_e26766_d_n2, assign19220_e26766_d_n6, assign19220_e26766_d_n7, assign19220_e26766_d_n10, assign19220_e26766_d_n11, assign19220_e26766_d_n12, assign19220_e26766_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19220_e26764: f64 = 1.0;
        (assign19220_e26764, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign19220_e26766;
        locals.var_xmax2_dn0 = assign19220_e26766_d_n0;
        locals.var_xmax2_dn2 = assign19220_e26766_d_n2;
        locals.var_xmax2_dn6 = assign19220_e26766_d_n6;
        locals.var_xmax2_dn7 = assign19220_e26766_d_n7;
        locals.var_xmax2_dn10 = assign19220_e26766_d_n10;
        locals.var_xmax2_dn11 = assign19220_e26766_d_n11;
        locals.var_xmax2_dn12 = assign19220_e26766_d_n12;
        locals.var_xmax2_dn17 = assign19220_e26766_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign19230_e26772, assign19230_e26772_d_n0, assign19230_e26772_d_n2, assign19230_e26772_d_n6, assign19230_e26772_d_n7, assign19230_e26772_d_n10, assign19230_e26772_d_n11, assign19230_e26772_d_n12, assign19230_e26772_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19230_e26772;
        locals.var_xp_dn0 = assign19230_e26772_d_n0;
        locals.var_xp_dn2 = assign19230_e26772_d_n2;
        locals.var_xp_dn6 = assign19230_e26772_d_n6;
        locals.var_xp_dn7 = assign19230_e26772_d_n7;
        locals.var_xp_dn10 = assign19230_e26772_d_n10;
        locals.var_xp_dn11 = assign19230_e26772_d_n11;
        locals.var_xp_dn12 = assign19230_e26772_d_n12;
        locals.var_xp_dn17 = assign19230_e26772_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19240_e26778, assign19240_e26778_d_n0, assign19240_e26778_d_n2, assign19240_e26778_d_n6, assign19240_e26778_d_n7, assign19240_e26778_d_n10, assign19240_e26778_d_n11, assign19240_e26778_d_n12, assign19240_e26778_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19240_e26778;
        locals.var_xmp_dn0 = assign19240_e26778_d_n0;
        locals.var_xmp_dn2 = assign19240_e26778_d_n2;
        locals.var_xmp_dn6 = assign19240_e26778_d_n6;
        locals.var_xmp_dn7 = assign19240_e26778_d_n7;
        locals.var_xmp_dn10 = assign19240_e26778_d_n10;
        locals.var_xmp_dn11 = assign19240_e26778_d_n11;
        locals.var_xmp_dn12 = assign19240_e26778_d_n12;
        locals.var_xmp_dn17 = assign19240_e26778_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19250_e26784,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign19250_e26784;
        locals.var_m0_rv = 0.0;

        let (assign19260_e26790,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19260_e26790;
        locals.var_mm_rv = 0.0;

        let (assign19270_e26796, assign19270_e26796_d_n0, assign19270_e26796_d_n2, assign19270_e26796_d_n6, assign19270_e26796_d_n7, assign19270_e26796_d_n10, assign19270_e26796_d_n11, assign19270_e26796_d_n12, assign19270_e26796_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign19270_e26796;
        locals.var_arg_dn0 = assign19270_e26796_d_n0;
        locals.var_arg_dn2 = assign19270_e26796_d_n2;
        locals.var_arg_dn6 = assign19270_e26796_d_n6;
        locals.var_arg_dn7 = assign19270_e26796_d_n7;
        locals.var_arg_dn10 = assign19270_e26796_d_n10;
        locals.var_arg_dn11 = assign19270_e26796_d_n11;
        locals.var_arg_dn12 = assign19270_e26796_d_n12;
        locals.var_arg_dn17 = assign19270_e26796_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign19280_e26802, assign19280_e26802_d_n0, assign19280_e26802_d_n2, assign19280_e26802_d_n6, assign19280_e26802_d_n7, assign19280_e26802_d_n10, assign19280_e26802_d_n11, assign19280_e26802_d_n12, assign19280_e26802_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19280_e26802;
        locals.var_dnm_dn0 = assign19280_e26802_d_n0;
        locals.var_dnm_dn2 = assign19280_e26802_d_n2;
        locals.var_dnm_dn6 = assign19280_e26802_d_n6;
        locals.var_dnm_dn7 = assign19280_e26802_d_n7;
        locals.var_dnm_dn10 = assign19280_e26802_d_n10;
        locals.var_dnm_dn11 = assign19280_e26802_d_n11;
        locals.var_dnm_dn12 = assign19280_e26802_d_n12;
        locals.var_dnm_dn17 = assign19280_e26802_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19290_e26810, assign19290_e26810_d_n0, assign19290_e26810_d_n2, assign19290_e26810_d_n6, assign19290_e26810_d_n7, assign19290_e26810_d_n10, assign19290_e26810_d_n11, assign19290_e26810_d_n12, assign19290_e26810_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19290_e26808: f64 = (locals.var_xp * locals.var_x2);
        (assign19290_e26808, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19290_e26810;
        locals.var_xp_dn0 = assign19290_e26810_d_n0;
        locals.var_xp_dn2 = assign19290_e26810_d_n2;
        locals.var_xp_dn6 = assign19290_e26810_d_n6;
        locals.var_xp_dn7 = assign19290_e26810_d_n7;
        locals.var_xp_dn10 = assign19290_e26810_d_n10;
        locals.var_xp_dn11 = assign19290_e26810_d_n11;
        locals.var_xp_dn12 = assign19290_e26810_d_n12;
        locals.var_xp_dn17 = assign19290_e26810_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19300_e26818, assign19300_e26818_d_n0, assign19300_e26818_d_n2, assign19300_e26818_d_n6, assign19300_e26818_d_n7, assign19300_e26818_d_n10, assign19300_e26818_d_n11, assign19300_e26818_d_n12, assign19300_e26818_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19300_e26816: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19300_e26816, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19300_e26818;
        locals.var_xmp_dn0 = assign19300_e26818_d_n0;
        locals.var_xmp_dn2 = assign19300_e26818_d_n2;
        locals.var_xmp_dn6 = assign19300_e26818_d_n6;
        locals.var_xmp_dn7 = assign19300_e26818_d_n7;
        locals.var_xmp_dn10 = assign19300_e26818_d_n10;
        locals.var_xmp_dn11 = assign19300_e26818_d_n11;
        locals.var_xmp_dn12 = assign19300_e26818_d_n12;
        locals.var_xmp_dn17 = assign19300_e26818_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19310_e26826, assign19310_e26826_d_n0, assign19310_e26826_d_n2, assign19310_e26826_d_n6, assign19310_e26826_d_n7, assign19310_e26826_d_n10, assign19310_e26826_d_n11, assign19310_e26826_d_n12, assign19310_e26826_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19310_e26824: f64 = (locals.var_xp * locals.var_x2);
        (assign19310_e26824, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19310_e26826;
        locals.var_xp_dn0 = assign19310_e26826_d_n0;
        locals.var_xp_dn2 = assign19310_e26826_d_n2;
        locals.var_xp_dn6 = assign19310_e26826_d_n6;
        locals.var_xp_dn7 = assign19310_e26826_d_n7;
        locals.var_xp_dn10 = assign19310_e26826_d_n10;
        locals.var_xp_dn11 = assign19310_e26826_d_n11;
        locals.var_xp_dn12 = assign19310_e26826_d_n12;
        locals.var_xp_dn17 = assign19310_e26826_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19320_e26834, assign19320_e26834_d_n0, assign19320_e26834_d_n2, assign19320_e26834_d_n6, assign19320_e26834_d_n7, assign19320_e26834_d_n10, assign19320_e26834_d_n11, assign19320_e26834_d_n12, assign19320_e26834_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19320_e26832: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19320_e26832, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19320_e26834;
        locals.var_xmp_dn0 = assign19320_e26834_d_n0;
        locals.var_xmp_dn2 = assign19320_e26834_d_n2;
        locals.var_xmp_dn6 = assign19320_e26834_d_n6;
        locals.var_xmp_dn7 = assign19320_e26834_d_n7;
        locals.var_xmp_dn10 = assign19320_e26834_d_n10;
        locals.var_xmp_dn11 = assign19320_e26834_d_n11;
        locals.var_xmp_dn12 = assign19320_e26834_d_n12;
        locals.var_xmp_dn17 = assign19320_e26834_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19330_e26842, assign19330_e26842_d_n0, assign19330_e26842_d_n2, assign19330_e26842_d_n6, assign19330_e26842_d_n7, assign19330_e26842_d_n10, assign19330_e26842_d_n11, assign19330_e26842_d_n12, assign19330_e26842_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19330_e26840: f64 = (locals.var_xp * locals.var_x2);
        (assign19330_e26840, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19330_e26842;
        locals.var_xp_dn0 = assign19330_e26842_d_n0;
        locals.var_xp_dn2 = assign19330_e26842_d_n2;
        locals.var_xp_dn6 = assign19330_e26842_d_n6;
        locals.var_xp_dn7 = assign19330_e26842_d_n7;
        locals.var_xp_dn10 = assign19330_e26842_d_n10;
        locals.var_xp_dn11 = assign19330_e26842_d_n11;
        locals.var_xp_dn12 = assign19330_e26842_d_n12;
        locals.var_xp_dn17 = assign19330_e26842_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19340_e26850, assign19340_e26850_d_n0, assign19340_e26850_d_n2, assign19340_e26850_d_n6, assign19340_e26850_d_n7, assign19340_e26850_d_n10, assign19340_e26850_d_n11, assign19340_e26850_d_n12, assign19340_e26850_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19340_e26848: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19340_e26848, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19340_e26850;
        locals.var_xmp_dn0 = assign19340_e26850_d_n0;
        locals.var_xmp_dn2 = assign19340_e26850_d_n2;
        locals.var_xmp_dn6 = assign19340_e26850_d_n6;
        locals.var_xmp_dn7 = assign19340_e26850_d_n7;
        locals.var_xmp_dn10 = assign19340_e26850_d_n10;
        locals.var_xmp_dn11 = assign19340_e26850_d_n11;
        locals.var_xmp_dn12 = assign19340_e26850_d_n12;
        locals.var_xmp_dn17 = assign19340_e26850_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19350_e26858, assign19350_e26858_d_n0, assign19350_e26858_d_n2, assign19350_e26858_d_n6, assign19350_e26858_d_n7, assign19350_e26858_d_n10, assign19350_e26858_d_n11, assign19350_e26858_d_n12, assign19350_e26858_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19350_e26856: f64 = (locals.var_xp * locals.var_x2);
        (assign19350_e26856, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19350_e26858;
        locals.var_xp_dn0 = assign19350_e26858_d_n0;
        locals.var_xp_dn2 = assign19350_e26858_d_n2;
        locals.var_xp_dn6 = assign19350_e26858_d_n6;
        locals.var_xp_dn7 = assign19350_e26858_d_n7;
        locals.var_xp_dn10 = assign19350_e26858_d_n10;
        locals.var_xp_dn11 = assign19350_e26858_d_n11;
        locals.var_xp_dn12 = assign19350_e26858_d_n12;
        locals.var_xp_dn17 = assign19350_e26858_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19360_e26866, assign19360_e26866_d_n0, assign19360_e26866_d_n2, assign19360_e26866_d_n6, assign19360_e26866_d_n7, assign19360_e26866_d_n10, assign19360_e26866_d_n11, assign19360_e26866_d_n12, assign19360_e26866_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19360_e26864: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19360_e26864, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19360_e26866;
        locals.var_xmp_dn0 = assign19360_e26866_d_n0;
        locals.var_xmp_dn2 = assign19360_e26866_d_n2;
        locals.var_xmp_dn6 = assign19360_e26866_d_n6;
        locals.var_xmp_dn7 = assign19360_e26866_d_n7;
        locals.var_xmp_dn10 = assign19360_e26866_d_n10;
        locals.var_xmp_dn11 = assign19360_e26866_d_n11;
        locals.var_xmp_dn12 = assign19360_e26866_d_n12;
        locals.var_xmp_dn17 = assign19360_e26866_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19370_e26874, assign19370_e26874_d_n0, assign19370_e26874_d_n2, assign19370_e26874_d_n6, assign19370_e26874_d_n7, assign19370_e26874_d_n10, assign19370_e26874_d_n11, assign19370_e26874_d_n12, assign19370_e26874_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19370_e26872: f64 = (locals.var_xp + locals.var_xmp);
        (assign19370_e26872, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign19370_e26874;
        locals.var_arg_dn0 = assign19370_e26874_d_n0;
        locals.var_arg_dn2 = assign19370_e26874_d_n2;
        locals.var_arg_dn6 = assign19370_e26874_d_n6;
        locals.var_arg_dn7 = assign19370_e26874_d_n7;
        locals.var_arg_dn10 = assign19370_e26874_d_n10;
        locals.var_arg_dn11 = assign19370_e26874_d_n11;
        locals.var_arg_dn12 = assign19370_e26874_d_n12;
        locals.var_arg_dn17 = assign19370_e26874_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign19380_e26880, assign19380_e26880_d_n0, assign19380_e26880_d_n2, assign19380_e26880_d_n6, assign19380_e26880_d_n7, assign19380_e26880_d_n10, assign19380_e26880_d_n11, assign19380_e26880_d_n12, assign19380_e26880_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19380_e26880;
        locals.var_dnm_dn0 = assign19380_e26880_d_n0;
        locals.var_dnm_dn2 = assign19380_e26880_d_n2;
        locals.var_dnm_dn6 = assign19380_e26880_d_n6;
        locals.var_dnm_dn7 = assign19380_e26880_d_n7;
        locals.var_dnm_dn10 = assign19380_e26880_d_n10;
        locals.var_dnm_dn11 = assign19380_e26880_d_n11;
        locals.var_dnm_dn12 = assign19380_e26880_d_n12;
        locals.var_dnm_dn17 = assign19380_e26880_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign19390_e26895: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard591 = assign19390_e26895;
        locals.var_guard591_rv = 0.0;

        let assign19400_e26898: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign19400_e26898;
        locals.var_guard592_rv = 0.0;

        let (assign19410_e26908,) = {
    if ((((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) && (locals.var_guard592 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19410_e26908;
        locals.var_mm_rv = 0.0;

        let assign19420_e26911: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign19420_e26911;
        locals.var_guard593_rv = 0.0;

        let (assign19430_e26924,) = {
    if (((((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) && (locals.var_guard592 == 0.0)) && (locals.var_guard593 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19430_e26924;
        locals.var_mm_rv = 0.0;

        let assign19440_e26927: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign19440_e26927;
        locals.var_guard594_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19450_e26943,) = {
    if ((((((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) && (locals.var_guard592 == 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19450_e26943;
        locals.var_mm_rv = 0.0;

        let assign19460_e26946: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign19460_e26946;
        locals.var_guard595_rv = 0.0;

        let (assign19470_e26965,) = {
    if (((((((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) && (locals.var_guard592 == 0.0)) && (locals.var_guard593 == 0.0)) && (locals.var_guard594 == 0.0)) && (locals.var_guard595 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19470_e26965;
        locals.var_mm_rv = 0.0;

        let (assign19480_e26973,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign19480_e26973;
        locals.var_m0_rv = 0.0;

        let mut assign19490_loop_guard: usize = 0;
        while {
            let assign19490_cond_e26982: f64 = if ((((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign19490_cond_e26982 != 0.0
        } {
            assign19490_loop_guard += 1;
            assert!(assign19490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign19490_body0_e26991, assign19490_body0_e26991_d_n0, assign19490_body0_e26991_d_n2, assign19490_body0_e26991_d_n6, assign19490_body0_e26991_d_n7, assign19490_body0_e26991_d_n10, assign19490_body0_e26991_d_n11, assign19490_body0_e26991_d_n12, assign19490_body0_e26991_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign19490_body0_e26989: f64 = (locals.var_dnm).sqrt();
        (assign19490_body0_e26989, (locals.var_dnm_dn0 / (2.0 * assign19490_body0_e26989)), (locals.var_dnm_dn2 / (2.0 * assign19490_body0_e26989)), (locals.var_dnm_dn6 / (2.0 * assign19490_body0_e26989)), (locals.var_dnm_dn7 / (2.0 * assign19490_body0_e26989)), (locals.var_dnm_dn10 / (2.0 * assign19490_body0_e26989)), (locals.var_dnm_dn11 / (2.0 * assign19490_body0_e26989)), (locals.var_dnm_dn12 / (2.0 * assign19490_body0_e26989)), (locals.var_dnm_dn17 / (2.0 * assign19490_body0_e26989)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign19490_body0_e26991;
            locals.var_dnm_dn0 = assign19490_body0_e26991_d_n0;
            locals.var_dnm_dn2 = assign19490_body0_e26991_d_n2;
            locals.var_dnm_dn6 = assign19490_body0_e26991_d_n6;
            locals.var_dnm_dn7 = assign19490_body0_e26991_d_n7;
            locals.var_dnm_dn10 = assign19490_body0_e26991_d_n10;
            locals.var_dnm_dn11 = assign19490_body0_e26991_d_n11;
            locals.var_dnm_dn12 = assign19490_body0_e26991_d_n12;
            locals.var_dnm_dn17 = assign19490_body0_e26991_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign19490_body1_e27001,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign19490_body1_e26999: f64 = (locals.var_m0 + 1.0);
        (assign19490_body1_e26999,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign19490_body1_e27001;
            locals.var_m0_rv = 0.0;
        }

        let (assign19500_e27016, assign19500_e27016_d_n0, assign19500_e27016_d_n2, assign19500_e27016_d_n6, assign19500_e27016_d_n7, assign19500_e27016_d_n10, assign19500_e27016_d_n11, assign19500_e27016_d_n12, assign19500_e27016_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) && (locals.var_guard591 == 0.0)) {
        let assign19500_e27012: f64 = (2.0 * 4.0);
        let assign19500_e27013: f64 = (1.0 / assign19500_e27012);
        let assign19500_e27014: f64 = (locals.var_dnm).powf(assign19500_e27013);
        (assign19500_e27014, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn0)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn2)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn6)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn7)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn10)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn11)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn12)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19500_e27013) as f64).is_finite() && ((assign19500_e27013) as f64).fract() == 0.0 { if assign19500_e27013 == 0.0 { 0.0 } else { (assign19500_e27013 * ((locals.var_dnm).powf(assign19500_e27013 - 1.0) * locals.var_dnm_dn17)) } } else { (assign19500_e27014 * (assign19500_e27013 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19500_e27016;
        locals.var_dnm_dn0 = assign19500_e27016_d_n0;
        locals.var_dnm_dn2 = assign19500_e27016_d_n2;
        locals.var_dnm_dn6 = assign19500_e27016_d_n6;
        locals.var_dnm_dn7 = assign19500_e27016_d_n7;
        locals.var_dnm_dn10 = assign19500_e27016_d_n10;
        locals.var_dnm_dn11 = assign19500_e27016_d_n11;
        locals.var_dnm_dn12 = assign19500_e27016_d_n12;
        locals.var_dnm_dn17 = assign19500_e27016_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19510_e27024, assign19510_e27024_d_n0, assign19510_e27024_d_n2, assign19510_e27024_d_n6, assign19510_e27024_d_n7, assign19510_e27024_d_n10, assign19510_e27024_d_n11, assign19510_e27024_d_n12, assign19510_e27024_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19510_e27022: f64 = (1.0 / locals.var_dnm);
        (assign19510_e27022, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19510_e27024;
        locals.var_dnm_dn0 = assign19510_e27024_d_n0;
        locals.var_dnm_dn2 = assign19510_e27024_d_n2;
        locals.var_dnm_dn6 = assign19510_e27024_d_n6;
        locals.var_dnm_dn7 = assign19510_e27024_d_n7;
        locals.var_dnm_dn10 = assign19510_e27024_d_n10;
        locals.var_dnm_dn11 = assign19510_e27024_d_n11;
        locals.var_dnm_dn12 = assign19510_e27024_d_n12;
        locals.var_dnm_dn17 = assign19510_e27024_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19520_e27034, assign19520_e27034_d_n0, assign19520_e27034_d_n2, assign19520_e27034_d_n6, assign19520_e27034_d_n7, assign19520_e27034_d_n10, assign19520_e27034_d_n11, assign19520_e27034_d_n12, assign19520_e27034_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19520_e27030: f64 = locals.var_tx__blk584;
        let assign19520_e27032: f64 = (assign19520_e27030 * locals.var_dnm);
        (assign19520_e27032, ((locals.var_tx__blk584_dn0 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn0)), ((locals.var_tx__blk584_dn2 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn2)), ((locals.var_tx__blk584_dn6 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn6)), ((locals.var_tx__blk584_dn7 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn7)), ((locals.var_tx__blk584_dn10 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn10)), ((locals.var_tx__blk584_dn11 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn11)), ((locals.var_tx__blk584_dn12 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn12)), ((locals.var_tx__blk584_dn17 * locals.var_dnm) + (assign19520_e27030 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_ty__blk585, locals.var_ty__blk585_dn0, locals.var_ty__blk585_dn2, locals.var_ty__blk585_dn6, locals.var_ty__blk585_dn7, locals.var_ty__blk585_dn10, locals.var_ty__blk585_dn11, locals.var_ty__blk585_dn12, locals.var_ty__blk585_dn17,)
    }
};
        locals.var_ty__blk585 = assign19520_e27034;
        locals.var_ty__blk585_dn0 = assign19520_e27034_d_n0;
        locals.var_ty__blk585_dn2 = assign19520_e27034_d_n2;
        locals.var_ty__blk585_dn6 = assign19520_e27034_d_n6;
        locals.var_ty__blk585_dn7 = assign19520_e27034_d_n7;
        locals.var_ty__blk585_dn10 = assign19520_e27034_d_n10;
        locals.var_ty__blk585_dn11 = assign19520_e27034_d_n11;
        locals.var_ty__blk585_dn12 = assign19520_e27034_d_n12;
        locals.var_ty__blk585_dn17 = assign19520_e27034_d_n17;
        locals.var_ty__blk585_rv = 0.0;

        let (assign19530_e27046, assign19530_e27046_d_n10,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19530_e27040: f64 = (2.0 * locals.var_uc_wsti);
        let assign19530_e27042: f64 = (assign19530_e27040 * p.p9);
        let assign19530_e27044: f64 = (assign19530_e27042 * locals.var_beta_inv);
        (assign19530_e27044, (assign19530_e27042 * locals.var_beta_inv_dn10),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn10,)
    }
};
        locals.var_costi7 = assign19530_e27046;
        locals.var_costi7_dn10 = assign19530_e27046_d_n10;
        locals.var_costi7_rv = 0.0;

        let (assign19540_e27060, assign19540_e27060_d_n0, assign19540_e27060_d_n2, assign19540_e27060_d_n6, assign19540_e27060_d_n7, assign19540_e27060_d_n10, assign19540_e27060_d_n11, assign19540_e27060_d_n12, assign19540_e27060_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19540_e27052: f64 = (locals.var_costi7 * locals.var_mu);
        let assign19540_e27054: f64 = (assign19540_e27052 * locals.var_qn0sti);
        let assign19540_e27056: f64 = (assign19540_e27054 * locals.var_ty__blk585);
        let assign19540_e27058: f64 = (assign19540_e27056 / locals.var_lch);
        (assign19540_e27058, ((((((((locals.var_costi7 * locals.var_mu_dn0) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn0)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn0)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn2) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn2)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn2)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn6) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn6)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn6)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn7) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn7)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn7)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn10)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn10)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn11) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn11)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn11)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn12) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn12)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn12)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn17) * locals.var_qn0sti) + (assign19540_e27052 * locals.var_qn0sti_dn17)) * locals.var_ty__blk585) + (assign19540_e27054 * locals.var_ty__blk585_dn17)) * locals.var_lch) - (assign19540_e27056 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12, locals.var_idssti_dn17,)
    }
};
        locals.var_idssti = assign19540_e27060;
        locals.var_idssti_dn0 = assign19540_e27060_d_n0;
        locals.var_idssti_dn2 = assign19540_e27060_d_n2;
        locals.var_idssti_dn6 = assign19540_e27060_d_n6;
        locals.var_idssti_dn7 = assign19540_e27060_d_n7;
        locals.var_idssti_dn10 = assign19540_e27060_d_n10;
        locals.var_idssti_dn11 = assign19540_e27060_d_n11;
        locals.var_idssti_dn12 = assign19540_e27060_d_n12;
        locals.var_idssti_dn17 = assign19540_e27060_d_n17;
        locals.var_idssti_rv = 0.0;

        let (assign19550_e27068, assign19550_e27068_d_n0, assign19550_e27068_d_n2, assign19550_e27068_d_n6, assign19550_e27068_d_n7, assign19550_e27068_d_n10, assign19550_e27068_d_n11, assign19550_e27068_d_n12, assign19550_e27068_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard586 != 0.0)) {
        let assign19550_e27066: f64 = (locals.var_ids + locals.var_idssti);
        (assign19550_e27066, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn7 + locals.var_idssti_dn7), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn11 + locals.var_idssti_dn11), (locals.var_ids_dn12 + locals.var_idssti_dn12), (locals.var_ids_dn17 + locals.var_idssti_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign19550_e27068;
        locals.var_ids_dn0 = assign19550_e27068_d_n0;
        locals.var_ids_dn2 = assign19550_e27068_d_n2;
        locals.var_ids_dn6 = assign19550_e27068_d_n6;
        locals.var_ids_dn7 = assign19550_e27068_d_n7;
        locals.var_ids_dn10 = assign19550_e27068_d_n10;
        locals.var_ids_dn11 = assign19550_e27068_d_n11;
        locals.var_ids_dn12 = assign19550_e27068_d_n12;
        locals.var_ids_dn17 = assign19550_e27068_d_n17;
        locals.var_ids_rv = 0.0;

        let assign19560_e27075: f64 = if ((p.p30 != 0.0) && (p.p32 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard596 = assign19560_e27075;
        locals.var_guard596_rv = 0.0;

        let (assign19570_e27083, assign19570_e27083_d_n0, assign19570_e27083_d_n2, assign19570_e27083_d_n6, assign19570_e27083_d_n7, assign19570_e27083_d_n10, assign19570_e27083_d_n11, assign19570_e27083_d_n12, assign19570_e27083_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19570_e27081: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign19570_e27081, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn7 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn7)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn11 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn11)), ((locals.var_vgvt_dn12 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn12)), ((locals.var_vgvt_dn17 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn17)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19570_e27083;
        locals.var_kusai00_dn0 = assign19570_e27083_d_n0;
        locals.var_kusai00_dn2 = assign19570_e27083_d_n2;
        locals.var_kusai00_dn6 = assign19570_e27083_d_n6;
        locals.var_kusai00_dn7 = assign19570_e27083_d_n7;
        locals.var_kusai00_dn10 = assign19570_e27083_d_n10;
        locals.var_kusai00_dn11 = assign19570_e27083_d_n11;
        locals.var_kusai00_dn12 = assign19570_e27083_d_n12;
        locals.var_kusai00_dn17 = assign19570_e27083_d_n17;
        locals.var_kusai00_rv = 0.0;

        let (assign19580_e27095, assign19580_e27095_d_n0, assign19580_e27095_d_n2, assign19580_e27095_d_n6, assign19580_e27095_d_n7, assign19580_e27095_d_n10, assign19580_e27095_d_n11, assign19580_e27095_d_n12, assign19580_e27095_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19580_e27089: f64 = (2.0 * locals.var_beta_inv);
        let assign19580_e27091: f64 = (assign19580_e27089 * locals.var_c_fox_inv);
        let assign19580_e27093: f64 = (assign19580_e27091 * locals.var_idd);
        (assign19580_e27093, (((assign19580_e27089 * locals.var_c_fox_inv_dn0) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn0)), (((assign19580_e27089 * locals.var_c_fox_inv_dn2) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn2)), (((assign19580_e27089 * locals.var_c_fox_inv_dn6) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn6)), (((assign19580_e27089 * locals.var_c_fox_inv_dn7) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn7)), (((((2.0 * locals.var_beta_inv_dn10) * locals.var_c_fox_inv) + (assign19580_e27089 * locals.var_c_fox_inv_dn10)) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn10)), (((assign19580_e27089 * locals.var_c_fox_inv_dn11) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn11)), (((assign19580_e27089 * locals.var_c_fox_inv_dn12) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn12)), (((assign19580_e27089 * locals.var_c_fox_inv_dn17) * locals.var_idd) + (assign19580_e27091 * locals.var_idd_dn17)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn6, locals.var_kusaidd_dn7, locals.var_kusaidd_dn10, locals.var_kusaidd_dn11, locals.var_kusaidd_dn12, locals.var_kusaidd_dn17,)
    }
};
        locals.var_kusaidd = assign19580_e27095;
        locals.var_kusaidd_dn0 = assign19580_e27095_d_n0;
        locals.var_kusaidd_dn2 = assign19580_e27095_d_n2;
        locals.var_kusaidd_dn6 = assign19580_e27095_d_n6;
        locals.var_kusaidd_dn7 = assign19580_e27095_d_n7;
        locals.var_kusaidd_dn10 = assign19580_e27095_d_n10;
        locals.var_kusaidd_dn11 = assign19580_e27095_d_n11;
        locals.var_kusaidd_dn12 = assign19580_e27095_d_n12;
        locals.var_kusaidd_dn17 = assign19580_e27095_d_n17;
        locals.var_kusaidd_rv = 0.0;

        let (assign19590_e27103, assign19590_e27103_d_n0, assign19590_e27103_d_n2, assign19590_e27103_d_n6, assign19590_e27103_d_n7, assign19590_e27103_d_n10, assign19590_e27103_d_n11, assign19590_e27103_d_n12, assign19590_e27103_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19590_e27101: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign19590_e27101, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn7 - locals.var_kusaidd_dn7), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn11 - locals.var_kusaidd_dn11), (locals.var_kusai00_dn12 - locals.var_kusaidd_dn12), (locals.var_kusai00_dn17 - locals.var_kusaidd_dn17),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19590_e27103;
        locals.var_kusail_dn0 = assign19590_e27103_d_n0;
        locals.var_kusail_dn2 = assign19590_e27103_d_n2;
        locals.var_kusail_dn6 = assign19590_e27103_d_n6;
        locals.var_kusail_dn7 = assign19590_e27103_d_n7;
        locals.var_kusail_dn10 = assign19590_e27103_d_n10;
        locals.var_kusail_dn11 = assign19590_e27103_d_n11;
        locals.var_kusail_dn12 = assign19590_e27103_d_n12;
        locals.var_kusail_dn17 = assign19590_e27103_d_n17;
        locals.var_kusail_rv = 0.0;

        let (assign19600_e27118, assign19600_e27118_d_n0, assign19600_e27118_d_n2, assign19600_e27118_d_n6, assign19600_e27118_d_n7, assign19600_e27118_d_n10, assign19600_e27118_d_n11, assign19600_e27118_d_n12, assign19600_e27118_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19600_e27109: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign19600_e27112: f64 = (4.0 * 0.001);
        let assign19600_e27114: f64 = (assign19600_e27112 * 0.001);
        let assign19600_e27115: f64 = (assign19600_e27109 + assign19600_e27114);
        let assign19600_e27116: f64 = (assign19600_e27115).sqrt();
        (assign19600_e27116, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign19600_e27116)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign19600_e27116)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign19600_e27116)), (((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)) / (2.0 * assign19600_e27116)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign19600_e27116)), (((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)) / (2.0 * assign19600_e27116)), (((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)) / (2.0 * assign19600_e27116)), (((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)) / (2.0 * assign19600_e27116)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19600_e27118;
        locals.var_tmf1_dn0 = assign19600_e27118_d_n0;
        locals.var_tmf1_dn2 = assign19600_e27118_d_n2;
        locals.var_tmf1_dn6 = assign19600_e27118_d_n6;
        locals.var_tmf1_dn7 = assign19600_e27118_d_n7;
        locals.var_tmf1_dn10 = assign19600_e27118_d_n10;
        locals.var_tmf1_dn11 = assign19600_e27118_d_n11;
        locals.var_tmf1_dn12 = assign19600_e27118_d_n12;
        locals.var_tmf1_dn17 = assign19600_e27118_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19610_e27132, assign19610_e27132_d_n0, assign19610_e27132_d_n2, assign19610_e27132_d_n6, assign19610_e27132_d_n7, assign19610_e27132_d_n10, assign19610_e27132_d_n11, assign19610_e27132_d_n12, assign19610_e27132_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19610_e27125: f64 = (locals.var_kusai00 + locals.var_tmf1);
        let assign19610_e27126: f64 = (0.5 * assign19610_e27125);
        let assign19610_e27129: f64 = (1e-10 * 0.001);
        let assign19610_e27130: f64 = (assign19610_e27126 + assign19610_e27129);
        (assign19610_e27130, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_kusai00_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_kusai00_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_kusai00_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_kusai00_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19610_e27132;
        locals.var_kusai00_dn0 = assign19610_e27132_d_n0;
        locals.var_kusai00_dn2 = assign19610_e27132_d_n2;
        locals.var_kusai00_dn6 = assign19610_e27132_d_n6;
        locals.var_kusai00_dn7 = assign19610_e27132_d_n7;
        locals.var_kusai00_dn10 = assign19610_e27132_d_n10;
        locals.var_kusai00_dn11 = assign19610_e27132_d_n11;
        locals.var_kusai00_dn12 = assign19610_e27132_d_n12;
        locals.var_kusai00_dn17 = assign19610_e27132_d_n17;
        locals.var_kusai00_rv = 0.0;

        let assign19620_e27135: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign19620_e27135;
        locals.var_guard597_rv = 0.0;

        let (assign19630_e27143, assign19630_e27143_d_n0, assign19630_e27143_d_n2, assign19630_e27143_d_n6, assign19630_e27143_d_n7, assign19630_e27143_d_n10, assign19630_e27143_d_n11, assign19630_e27143_d_n12, assign19630_e27143_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19630_e27143;
        locals.var_kusai00_dn0 = assign19630_e27143_d_n0;
        locals.var_kusai00_dn2 = assign19630_e27143_d_n2;
        locals.var_kusai00_dn6 = assign19630_e27143_d_n6;
        locals.var_kusai00_dn7 = assign19630_e27143_d_n7;
        locals.var_kusai00_dn10 = assign19630_e27143_d_n10;
        locals.var_kusai00_dn11 = assign19630_e27143_d_n11;
        locals.var_kusai00_dn12 = assign19630_e27143_d_n12;
        locals.var_kusai00_dn17 = assign19630_e27143_d_n17;
        locals.var_kusai00_rv = 0.0;

        let (assign19640_e27158, assign19640_e27158_d_n0, assign19640_e27158_d_n2, assign19640_e27158_d_n6, assign19640_e27158_d_n7, assign19640_e27158_d_n10, assign19640_e27158_d_n11, assign19640_e27158_d_n12, assign19640_e27158_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19640_e27149: f64 = (locals.var_kusail * locals.var_kusail);
        let assign19640_e27152: f64 = (4.0 * 0.001);
        let assign19640_e27154: f64 = (assign19640_e27152 * 0.001);
        let assign19640_e27155: f64 = (assign19640_e27149 + assign19640_e27154);
        let assign19640_e27156: f64 = (assign19640_e27155).sqrt();
        (assign19640_e27156, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign19640_e27156)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign19640_e27156)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign19640_e27156)), (((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)) / (2.0 * assign19640_e27156)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign19640_e27156)), (((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)) / (2.0 * assign19640_e27156)), (((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)) / (2.0 * assign19640_e27156)), (((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)) / (2.0 * assign19640_e27156)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19640_e27158;
        locals.var_tmf1_dn0 = assign19640_e27158_d_n0;
        locals.var_tmf1_dn2 = assign19640_e27158_d_n2;
        locals.var_tmf1_dn6 = assign19640_e27158_d_n6;
        locals.var_tmf1_dn7 = assign19640_e27158_d_n7;
        locals.var_tmf1_dn10 = assign19640_e27158_d_n10;
        locals.var_tmf1_dn11 = assign19640_e27158_d_n11;
        locals.var_tmf1_dn12 = assign19640_e27158_d_n12;
        locals.var_tmf1_dn17 = assign19640_e27158_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19650_e27172, assign19650_e27172_d_n0, assign19650_e27172_d_n2, assign19650_e27172_d_n6, assign19650_e27172_d_n7, assign19650_e27172_d_n10, assign19650_e27172_d_n11, assign19650_e27172_d_n12, assign19650_e27172_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19650_e27165: f64 = (locals.var_kusail + locals.var_tmf1);
        let assign19650_e27166: f64 = (0.5 * assign19650_e27165);
        let assign19650_e27169: f64 = (1e-10 * 0.001);
        let assign19650_e27170: f64 = (assign19650_e27166 + assign19650_e27169);
        (assign19650_e27170, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_kusail_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_kusail_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_kusail_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_kusail_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19650_e27172;
        locals.var_kusail_dn0 = assign19650_e27172_d_n0;
        locals.var_kusail_dn2 = assign19650_e27172_d_n2;
        locals.var_kusail_dn6 = assign19650_e27172_d_n6;
        locals.var_kusail_dn7 = assign19650_e27172_d_n7;
        locals.var_kusail_dn10 = assign19650_e27172_d_n10;
        locals.var_kusail_dn11 = assign19650_e27172_d_n11;
        locals.var_kusail_dn12 = assign19650_e27172_d_n12;
        locals.var_kusail_dn17 = assign19650_e27172_d_n17;
        locals.var_kusail_rv = 0.0;

        let assign19660_e27175: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard598 = assign19660_e27175;
        locals.var_guard598_rv = 0.0;

        let (assign19670_e27183, assign19670_e27183_d_n0, assign19670_e27183_d_n2, assign19670_e27183_d_n6, assign19670_e27183_d_n7, assign19670_e27183_d_n10, assign19670_e27183_d_n11, assign19670_e27183_d_n12, assign19670_e27183_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard598 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19670_e27183;
        locals.var_kusail_dn0 = assign19670_e27183_d_n0;
        locals.var_kusail_dn2 = assign19670_e27183_d_n2;
        locals.var_kusail_dn6 = assign19670_e27183_d_n6;
        locals.var_kusail_dn7 = assign19670_e27183_d_n7;
        locals.var_kusail_dn10 = assign19670_e27183_d_n10;
        locals.var_kusail_dn11 = assign19670_e27183_d_n11;
        locals.var_kusail_dn12 = assign19670_e27183_d_n12;
        locals.var_kusail_dn17 = assign19670_e27183_d_n17;
        locals.var_kusail_rv = 0.0;

        let (assign19680_e27191, assign19680_e27191_d_n0, assign19680_e27191_d_n2, assign19680_e27191_d_n6, assign19680_e27191_d_n7, assign19680_e27191_d_n10, assign19680_e27191_d_n11, assign19680_e27191_d_n12, assign19680_e27191_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign19680_e27189: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign19680_e27189, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn7 - locals.var_kusail_dn7), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn11 - locals.var_kusail_dn11), (locals.var_kusai00_dn12 - locals.var_kusail_dn12), (locals.var_kusai00_dn17 - locals.var_kusail_dn17),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn6, locals.var_kusai00l_dn7, locals.var_kusai00l_dn10, locals.var_kusai00l_dn11, locals.var_kusai00l_dn12, locals.var_kusai00l_dn17,)
    }
};
        locals.var_kusai00l = assign19680_e27191;
        locals.var_kusai00l_dn0 = assign19680_e27191_d_n0;
        locals.var_kusai00l_dn2 = assign19680_e27191_d_n2;
        locals.var_kusai00l_dn6 = assign19680_e27191_d_n6;
        locals.var_kusai00l_dn7 = assign19680_e27191_d_n7;
        locals.var_kusai00l_dn10 = assign19680_e27191_d_n10;
        locals.var_kusai00l_dn11 = assign19680_e27191_d_n11;
        locals.var_kusai00l_dn12 = assign19680_e27191_d_n12;
        locals.var_kusai00l_dn17 = assign19680_e27191_d_n17;
        locals.var_kusai00l_rv = 0.0;

        let assign19690_e27195: f64 = (10.0 * 2.220446049250313e-16);
        let assign19690_e27200: f64 = (10.0 * 2.220446049250313e-16);
        let assign19690_e27202: f64 = if ((locals.var_qn0 < assign19690_e27195) || (locals.var_kusai00l < assign19690_e27200)) { 1.0 } else { 0.0 };
        locals.var_guard599 = assign19690_e27202;
        locals.var_guard599_rv = 0.0;

        let (assign19700_e27210,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard599 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign19700_e27210;
        locals.var_flg_ign_rv = 0.0;

        let (assign19710_e27219,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard599 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign19710_e27219;
        locals.var_flg_ign_rv = 0.0;

        locals.var_idsorg = locals.var_ids;
        locals.var_idsorg_dn0 = locals.var_ids_dn0;
        locals.var_idsorg_dn2 = locals.var_ids_dn2;
        locals.var_idsorg_dn6 = locals.var_ids_dn6;
        locals.var_idsorg_dn7 = locals.var_ids_dn7;
        locals.var_idsorg_dn10 = locals.var_ids_dn10;
        locals.var_idsorg_dn11 = locals.var_ids_dn11;
        locals.var_idsorg_dn12 = locals.var_ids_dn12;
        locals.var_idsorg_dn17 = locals.var_ids_dn17;
        locals.var_idsorg_rv = 0.0;

        locals.var_idspt1 = 0.0;
        locals.var_idspt1_dn0 = 0.0;
        locals.var_idspt1_dn2 = 0.0;
        locals.var_idspt1_dn6 = 0.0;
        locals.var_idspt1_dn7 = 0.0;
        locals.var_idspt1_dn10 = 0.0;
        locals.var_idspt1_dn11 = 0.0;
        locals.var_idspt1_dn12 = 0.0;
        locals.var_idspt1_dn17 = 0.0;
        locals.var_idspt1_rv = 0.0;

        let assign19740_e27228: f64 = if ((p.p281 > 0.0) && (p.p285 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard600 = assign19740_e27228;
        locals.var_guard600_rv = 0.0;

        let (assign19750_e27232,) = {
    if (locals.var_guard600 != 0.0) {
        (locals.var_lgleff,)
    } else {
        (locals.var_leff__blk607,)
    }
};
        locals.var_leff__blk607 = assign19750_e27232;
        locals.var_leff__blk607_rv = 0.0;

        let (assign19760_e27236,) = {
    if (locals.var_guard600 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_t_soi__blk611,)
    }
};
        locals.var_t_soi__blk611 = assign19760_e27236;
        locals.var_t_soi__blk611_rv = 0.0;

        let (assign19770_e27248, assign19770_e27248_d_n0, assign19770_e27248_d_n2, assign19770_e27248_d_n6, assign19770_e27248_d_n7, assign19770_e27248_d_n10, assign19770_e27248_d_n11, assign19770_e27248_d_n12, assign19770_e27248_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19770_e27240: f64 = (locals.var_vgs - locals.var_vfb);
        let assign19770_e27242: f64 = (assign19770_e27240 + locals.var_dvth);
        let assign19770_e27244: f64 = (assign19770_e27242 - locals.var_dppg);
        let assign19770_e27246: f64 = (assign19770_e27244 - p.p286);
        (assign19770_e27246, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), (locals.var_dvth_dn10 - locals.var_dppg_dn10), ((locals.var_vgs_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), (locals.var_dvth_dn12 - locals.var_dppg_dn12), (locals.var_dvth_dn17 - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgp__blk612, locals.var_vgp__blk612_dn0, locals.var_vgp__blk612_dn2, locals.var_vgp__blk612_dn6, locals.var_vgp__blk612_dn7, locals.var_vgp__blk612_dn10, locals.var_vgp__blk612_dn11, locals.var_vgp__blk612_dn12, locals.var_vgp__blk612_dn17,)
    }
};
        locals.var_vgp__blk612 = assign19770_e27248;
        locals.var_vgp__blk612_dn0 = assign19770_e27248_d_n0;
        locals.var_vgp__blk612_dn2 = assign19770_e27248_d_n2;
        locals.var_vgp__blk612_dn6 = assign19770_e27248_d_n6;
        locals.var_vgp__blk612_dn7 = assign19770_e27248_d_n7;
        locals.var_vgp__blk612_dn10 = assign19770_e27248_d_n10;
        locals.var_vgp__blk612_dn11 = assign19770_e27248_d_n11;
        locals.var_vgp__blk612_dn12 = assign19770_e27248_d_n12;
        locals.var_vgp__blk612_dn17 = assign19770_e27248_d_n17;
        locals.var_vgp__blk612_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19780_e27254, assign19780_e27254_d_n0, assign19780_e27254_d_n2, assign19780_e27254_d_n6, assign19780_e27254_d_n7, assign19780_e27254_d_n10, assign19780_e27254_d_n11, assign19780_e27254_d_n12, assign19780_e27254_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19780_e27252: f64 = (locals.var_vth + p.p286);
        (assign19780_e27252, locals.var_vth_dn0, locals.var_vth_dn2, locals.var_vth_dn6, locals.var_vth_dn7, locals.var_vth_dn10, locals.var_vth_dn11, locals.var_vth_dn12, locals.var_vth_dn17,)
    } else {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn12, locals.var_wk_vth_dn17,)
    }
};
        locals.var_wk_vth = assign19780_e27254;
        locals.var_wk_vth_dn0 = assign19780_e27254_d_n0;
        locals.var_wk_vth_dn2 = assign19780_e27254_d_n2;
        locals.var_wk_vth_dn6 = assign19780_e27254_d_n6;
        locals.var_wk_vth_dn7 = assign19780_e27254_d_n7;
        locals.var_wk_vth_dn10 = assign19780_e27254_d_n10;
        locals.var_wk_vth_dn11 = assign19780_e27254_d_n11;
        locals.var_wk_vth_dn12 = assign19780_e27254_d_n12;
        locals.var_wk_vth_dn17 = assign19780_e27254_d_n17;
        locals.var_wk_vth_rv = 0.0;

        let (assign19790_e27258,) = {
    if (locals.var_guard600 != 0.0) {
        (p.p285,)
    } else {
        (locals.var_wk_mu,)
    }
};
        locals.var_wk_mu = assign19790_e27258;
        locals.var_wk_mu_rv = 0.0;

        let (assign19800_e27262,) = {
    if (locals.var_guard600 != 0.0) {
        (p.p283,)
    } else {
        (locals.var_wk_xj,)
    }
};
        locals.var_wk_xj = assign19800_e27262;
        locals.var_wk_xj_rv = 0.0;

        let (assign19810_e27266,) = {
    if (locals.var_guard600 != 0.0) {
        (locals.var_mks_njunc,)
    } else {
        (locals.var_uc_wk_njunc,)
    }
};
        locals.var_uc_wk_njunc = assign19810_e27266;
        locals.var_uc_wk_njunc_rv = 0.0;

        let (assign19820_e27279, assign19820_e27279_d_n0, assign19820_e27279_d_n2, assign19820_e27279_d_n6, assign19820_e27279_d_n7, assign19820_e27279_d_n10, assign19820_e27279_d_n11, assign19820_e27279_d_n12, assign19820_e27279_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19820_e27271: f64 = (locals.var_uc_wk_njunc / locals.var_nin);
        let assign19820_e27273: f64 = (assign19820_e27271 * locals.var_nsub);
        let assign19820_e27275: f64 = (assign19820_e27273 / locals.var_nin);
        let assign19820_e27276: f64 = (assign19820_e27275).ln();
        let assign19820_e27277: f64 = (locals.var_beta_inv * assign19820_e27276);
        (assign19820_e27277, (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn0)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn2)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn6)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn7)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275)), ((locals.var_beta_inv_dn10 * assign19820_e27276) + (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn10)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275))), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn11)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn12)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19820_e27271 * locals.var_nsub_dn17)) * locals.var_nin) - (assign19820_e27273 * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)) / assign19820_e27275)),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn12, locals.var_vbipn_dn17,)
    }
};
        locals.var_vbipn = assign19820_e27279;
        locals.var_vbipn_dn0 = assign19820_e27279_d_n0;
        locals.var_vbipn_dn2 = assign19820_e27279_d_n2;
        locals.var_vbipn_dn6 = assign19820_e27279_d_n6;
        locals.var_vbipn_dn7 = assign19820_e27279_d_n7;
        locals.var_vbipn_dn10 = assign19820_e27279_d_n10;
        locals.var_vbipn_dn11 = assign19820_e27279_d_n11;
        locals.var_vbipn_dn12 = assign19820_e27279_d_n12;
        locals.var_vbipn_dn17 = assign19820_e27279_d_n17;
        locals.var_vbipn_rv = 0.0;

        let (assign19830_e27288, assign19830_e27288_d_n0, assign19830_e27288_d_n2, assign19830_e27288_d_n6, assign19830_e27288_d_n7, assign19830_e27288_d_n10, assign19830_e27288_d_n11, assign19830_e27288_d_n12, assign19830_e27288_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let (assign19830_e27286, assign19830_e27286_d_n0, assign19830_e27286_d_n2, assign19830_e27286_d_n6, assign19830_e27286_d_n7, assign19830_e27286_d_n10, assign19830_e27286_d_n11, assign19830_e27286_d_n12, assign19830_e27286_d_n17,) = {
            if (p.p43 == 1.0) {
                (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
            } else {
                (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
            }
        };
        (assign19830_e27286, assign19830_e27286_d_n0, assign19830_e27286_d_n2, assign19830_e27286_d_n6, assign19830_e27286_d_n7, assign19830_e27286_d_n10, assign19830_e27286_d_n11, assign19830_e27286_d_n12, assign19830_e27286_d_n17,)
    } else {
        (locals.var_vbs__blk603, locals.var_vbs__blk603_dn0, locals.var_vbs__blk603_dn2, locals.var_vbs__blk603_dn6, locals.var_vbs__blk603_dn7, locals.var_vbs__blk603_dn10, locals.var_vbs__blk603_dn11, locals.var_vbs__blk603_dn12, locals.var_vbs__blk603_dn17,)
    }
};
        locals.var_vbs__blk603 = assign19830_e27288;
        locals.var_vbs__blk603_dn0 = assign19830_e27288_d_n0;
        locals.var_vbs__blk603_dn2 = assign19830_e27288_d_n2;
        locals.var_vbs__blk603_dn6 = assign19830_e27288_d_n6;
        locals.var_vbs__blk603_dn7 = assign19830_e27288_d_n7;
        locals.var_vbs__blk603_dn10 = assign19830_e27288_d_n10;
        locals.var_vbs__blk603_dn11 = assign19830_e27288_d_n11;
        locals.var_vbs__blk603_dn12 = assign19830_e27288_d_n12;
        locals.var_vbs__blk603_dn17 = assign19830_e27288_d_n17;
        locals.var_vbs__blk603_rv = 0.0;

        let (assign19840_e27309, assign19840_e27309_d_n0, assign19840_e27309_d_n2, assign19840_e27309_d_n6, assign19840_e27309_d_n7, assign19840_e27309_d_n10, assign19840_e27309_d_n11, assign19840_e27309_d_n12, assign19840_e27309_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19840_e27292: f64 = (2.0 * 1.6021918e-19);
        let assign19840_e27295: f64 = (locals.var_vbipn - locals.var_vbs__blk603);
        let assign19840_e27296: f64 = (assign19840_e27292 * assign19840_e27295);
        let assign19840_e27298: f64 = (assign19840_e27296 / 1.034943e-10);
        let assign19840_e27300: f64 = (assign19840_e27298 * locals.var_nsub);
        let assign19840_e27302: f64 = (assign19840_e27300 * locals.var_uc_wk_njunc);
        let assign19840_e27305: f64 = (locals.var_nsub + locals.var_uc_wk_njunc);
        let assign19840_e27306: f64 = (assign19840_e27302 / assign19840_e27305);
        let assign19840_e27307: f64 = (assign19840_e27306).sqrt();
        (assign19840_e27307, (((((((((assign19840_e27292 * (locals.var_vbipn_dn0 - locals.var_vbs__blk603_dn0)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn0)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn0)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)), (((((((((assign19840_e27292 * (locals.var_vbipn_dn2 - locals.var_vbs__blk603_dn2)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn2)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn2)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)), (((((((((assign19840_e27292 * (locals.var_vbipn_dn6 - locals.var_vbs__blk603_dn6)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn6)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn6)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)), (((((((((assign19840_e27292 * (locals.var_vbipn_dn7 - locals.var_vbs__blk603_dn7)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn7)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn7)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)), (((((((((assign19840_e27292 * (locals.var_vbipn_dn10 - locals.var_vbs__blk603_dn10)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn10)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn10)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)), (((((((((assign19840_e27292 * (locals.var_vbipn_dn11 - locals.var_vbs__blk603_dn11)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn11)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn11)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)), (((((((((assign19840_e27292 * (locals.var_vbipn_dn12 - locals.var_vbs__blk603_dn12)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn12)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn12)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)), (((((((((assign19840_e27292 * (locals.var_vbipn_dn17 - locals.var_vbs__blk603_dn17)) / 1.034943e-10) * locals.var_nsub) + (assign19840_e27298 * locals.var_nsub_dn17)) * locals.var_uc_wk_njunc) * assign19840_e27305) - (assign19840_e27302 * locals.var_nsub_dn17)) / (assign19840_e27305 * assign19840_e27305)) / (2.0 * assign19840_e27307)),)
    } else {
        (locals.var_ec__blk608, locals.var_ec__blk608_dn0, locals.var_ec__blk608_dn2, locals.var_ec__blk608_dn6, locals.var_ec__blk608_dn7, locals.var_ec__blk608_dn10, locals.var_ec__blk608_dn11, locals.var_ec__blk608_dn12, locals.var_ec__blk608_dn17,)
    }
};
        locals.var_ec__blk608 = assign19840_e27309;
        locals.var_ec__blk608_dn0 = assign19840_e27309_d_n0;
        locals.var_ec__blk608_dn2 = assign19840_e27309_d_n2;
        locals.var_ec__blk608_dn6 = assign19840_e27309_d_n6;
        locals.var_ec__blk608_dn7 = assign19840_e27309_d_n7;
        locals.var_ec__blk608_dn10 = assign19840_e27309_d_n10;
        locals.var_ec__blk608_dn11 = assign19840_e27309_d_n11;
        locals.var_ec__blk608_dn12 = assign19840_e27309_d_n12;
        locals.var_ec__blk608_dn17 = assign19840_e27309_d_n17;
        locals.var_ec__blk608_rv = 0.0;

        let (assign19850_e27315, assign19850_e27315_d_n0, assign19850_e27315_d_n2, assign19850_e27315_d_n6, assign19850_e27315_d_n7, assign19850_e27315_d_n10, assign19850_e27315_d_n11, assign19850_e27315_d_n12, assign19850_e27315_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19850_e27313: f64 = (locals.var_ec__blk608 * locals.var_leff__blk607);
        (assign19850_e27313, (locals.var_ec__blk608_dn0 * locals.var_leff__blk607), (locals.var_ec__blk608_dn2 * locals.var_leff__blk607), (locals.var_ec__blk608_dn6 * locals.var_leff__blk607), (locals.var_ec__blk608_dn7 * locals.var_leff__blk607), (locals.var_ec__blk608_dn10 * locals.var_leff__blk607), (locals.var_ec__blk608_dn11 * locals.var_leff__blk607), (locals.var_ec__blk608_dn12 * locals.var_leff__blk607), (locals.var_ec__blk608_dn17 * locals.var_leff__blk607),)
    } else {
        (locals.var_wk, locals.var_wk_dn0, locals.var_wk_dn2, locals.var_wk_dn6, locals.var_wk_dn7, locals.var_wk_dn10, locals.var_wk_dn11, locals.var_wk_dn12, locals.var_wk_dn17,)
    }
};
        locals.var_wk = assign19850_e27315;
        locals.var_wk_dn0 = assign19850_e27315_d_n0;
        locals.var_wk_dn2 = assign19850_e27315_d_n2;
        locals.var_wk_dn6 = assign19850_e27315_d_n6;
        locals.var_wk_dn7 = assign19850_e27315_d_n7;
        locals.var_wk_dn10 = assign19850_e27315_d_n10;
        locals.var_wk_dn11 = assign19850_e27315_d_n11;
        locals.var_wk_dn12 = assign19850_e27315_d_n12;
        locals.var_wk_dn17 = assign19850_e27315_d_n17;
        locals.var_wk_rv = 0.0;

        let (assign19860_e27328, assign19860_e27328_d_n0, assign19860_e27328_d_n2, assign19860_e27328_d_n6, assign19860_e27328_d_n7, assign19860_e27328_d_n10, assign19860_e27328_d_n11, assign19860_e27328_d_n12, assign19860_e27328_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19860_e27318: f64 = (-0.25);
        let assign19860_e27320: f64 = (assign19860_e27318 * locals.var_wk);
        let assign19860_e27322: f64 = (assign19860_e27320 * locals.var_wk);
        let assign19860_e27325: f64 = (locals.var_vds + locals.var_wk);
        let assign19860_e27326: f64 = (assign19860_e27322 / assign19860_e27325);
        (assign19860_e27326, ((((((assign19860_e27318 * locals.var_wk_dn0) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn0)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn0 + locals.var_wk_dn0))) / (assign19860_e27325 * assign19860_e27325)), ((((((assign19860_e27318 * locals.var_wk_dn2) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn2)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn2 + locals.var_wk_dn2))) / (assign19860_e27325 * assign19860_e27325)), ((((((assign19860_e27318 * locals.var_wk_dn6) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn6)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn6 + locals.var_wk_dn6))) / (assign19860_e27325 * assign19860_e27325)), ((((((assign19860_e27318 * locals.var_wk_dn7) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn7)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn7 + locals.var_wk_dn7))) / (assign19860_e27325 * assign19860_e27325)), ((((((assign19860_e27318 * locals.var_wk_dn10) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn10)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn10 + locals.var_wk_dn10))) / (assign19860_e27325 * assign19860_e27325)), ((((((assign19860_e27318 * locals.var_wk_dn11) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn11)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn11 + locals.var_wk_dn11))) / (assign19860_e27325 * assign19860_e27325)), ((((((assign19860_e27318 * locals.var_wk_dn12) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn12)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn12 + locals.var_wk_dn12))) / (assign19860_e27325 * assign19860_e27325)), ((((((assign19860_e27318 * locals.var_wk_dn17) * locals.var_wk) + (assign19860_e27320 * locals.var_wk_dn17)) * assign19860_e27325) - (assign19860_e27322 * (locals.var_vds_dn17 + locals.var_wk_dn17))) / (assign19860_e27325 * assign19860_e27325)),)
    } else {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    }
};
        locals.var_dphi_vds = assign19860_e27328;
        locals.var_dphi_vds_dn0 = assign19860_e27328_d_n0;
        locals.var_dphi_vds_dn2 = assign19860_e27328_d_n2;
        locals.var_dphi_vds_dn6 = assign19860_e27328_d_n6;
        locals.var_dphi_vds_dn7 = assign19860_e27328_d_n7;
        locals.var_dphi_vds_dn10 = assign19860_e27328_d_n10;
        locals.var_dphi_vds_dn11 = assign19860_e27328_d_n11;
        locals.var_dphi_vds_dn12 = assign19860_e27328_d_n12;
        locals.var_dphi_vds_dn17 = assign19860_e27328_d_n17;
        locals.var_dphi_vds_rv = 0.0;

        let (assign19870_e27332, assign19870_e27332_d_n0, assign19870_e27332_d_n2, assign19870_e27332_d_n6, assign19870_e27332_d_n7, assign19870_e27332_d_n10, assign19870_e27332_d_n11, assign19870_e27332_d_n12, assign19870_e27332_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    } else {
        (locals.var_vbs__blk627, locals.var_vbs__blk627_dn0, locals.var_vbs__blk627_dn2, locals.var_vbs__blk627_dn6, locals.var_vbs__blk627_dn7, locals.var_vbs__blk627_dn10, locals.var_vbs__blk627_dn11, locals.var_vbs__blk627_dn12, locals.var_vbs__blk627_dn17,)
    }
};
        locals.var_vbs__blk627 = assign19870_e27332;
        locals.var_vbs__blk627_dn0 = assign19870_e27332_d_n0;
        locals.var_vbs__blk627_dn2 = assign19870_e27332_d_n2;
        locals.var_vbs__blk627_dn6 = assign19870_e27332_d_n6;
        locals.var_vbs__blk627_dn7 = assign19870_e27332_d_n7;
        locals.var_vbs__blk627_dn10 = assign19870_e27332_d_n10;
        locals.var_vbs__blk627_dn11 = assign19870_e27332_d_n11;
        locals.var_vbs__blk627_dn12 = assign19870_e27332_d_n12;
        locals.var_vbs__blk627_dn17 = assign19870_e27332_d_n17;
        locals.var_vbs__blk627_rv = 0.0;

        let (assign19880_e27336, assign19880_e27336_d_n0, assign19880_e27336_d_n2, assign19880_e27336_d_n6, assign19880_e27336_d_n7, assign19880_e27336_d_n10, assign19880_e27336_d_n11, assign19880_e27336_d_n12, assign19880_e27336_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn12, locals.var_wk_vth_dn17,)
    } else {
        (locals.var_vth__blk628, locals.var_vth__blk628_dn0, locals.var_vth__blk628_dn2, locals.var_vth__blk628_dn6, locals.var_vth__blk628_dn7, locals.var_vth__blk628_dn10, locals.var_vth__blk628_dn11, locals.var_vth__blk628_dn12, locals.var_vth__blk628_dn17,)
    }
};
        locals.var_vth__blk628 = assign19880_e27336;
        locals.var_vth__blk628_dn0 = assign19880_e27336_d_n0;
        locals.var_vth__blk628_dn2 = assign19880_e27336_d_n2;
        locals.var_vth__blk628_dn6 = assign19880_e27336_d_n6;
        locals.var_vth__blk628_dn7 = assign19880_e27336_d_n7;
        locals.var_vth__blk628_dn10 = assign19880_e27336_d_n10;
        locals.var_vth__blk628_dn11 = assign19880_e27336_d_n11;
        locals.var_vth__blk628_dn12 = assign19880_e27336_d_n12;
        locals.var_vth__blk628_dn17 = assign19880_e27336_d_n17;
        locals.var_vth__blk628_rv = 0.0;

        let (assign19890_e27354, assign19890_e27354_d_n0, assign19890_e27354_d_n2, assign19890_e27354_d_n6, assign19890_e27354_d_n7, assign19890_e27354_d_n10, assign19890_e27354_d_n11, assign19890_e27354_d_n12, assign19890_e27354_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19890_e27343: f64 = (locals.var_vgp__blk612 - locals.var_vbs__blk627);
        let assign19890_e27344: f64 = (locals.var_beta * assign19890_e27343);
        let assign19890_e27346: f64 = (assign19890_e27344 - 1.0);
        let assign19890_e27347: f64 = (4.0 * assign19890_e27346);
        let assign19890_e27350: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign19890_e27351: f64 = (assign19890_e27347 / assign19890_e27350);
        let assign19890_e27352: f64 = (1.0 + assign19890_e27351);
        (assign19890_e27352, ((((4.0 * (locals.var_beta * (locals.var_vgp__blk612_dn0 - locals.var_vbs__blk627_dn0))) * assign19890_e27350) - (assign19890_e27347 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign19890_e27350 * assign19890_e27350)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk612_dn2 - locals.var_vbs__blk627_dn2))) * assign19890_e27350) - (assign19890_e27347 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign19890_e27350 * assign19890_e27350)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk612_dn6 - locals.var_vbs__blk627_dn6))) * assign19890_e27350) - (assign19890_e27347 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign19890_e27350 * assign19890_e27350)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk612_dn7 - locals.var_vbs__blk627_dn7))) * assign19890_e27350) - (assign19890_e27347 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign19890_e27350 * assign19890_e27350)), ((((4.0 * ((locals.var_beta_dn10 * assign19890_e27343) + (locals.var_beta * (locals.var_vgp__blk612_dn10 - locals.var_vbs__blk627_dn10)))) * assign19890_e27350) - (assign19890_e27347 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign19890_e27350 * assign19890_e27350)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk612_dn11 - locals.var_vbs__blk627_dn11))) * assign19890_e27350) - (assign19890_e27347 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign19890_e27350 * assign19890_e27350)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk612_dn12 - locals.var_vbs__blk627_dn12))) * assign19890_e27350) - (assign19890_e27347 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign19890_e27350 * assign19890_e27350)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk612_dn17 - locals.var_vbs__blk627_dn17))) * assign19890_e27350) - (assign19890_e27347 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign19890_e27350 * assign19890_e27350)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign19890_e27354;
        locals.var_tx_dn0 = assign19890_e27354_d_n0;
        locals.var_tx_dn2 = assign19890_e27354_d_n2;
        locals.var_tx_dn6 = assign19890_e27354_d_n6;
        locals.var_tx_dn7 = assign19890_e27354_d_n7;
        locals.var_tx_dn10 = assign19890_e27354_d_n10;
        locals.var_tx_dn11 = assign19890_e27354_d_n11;
        locals.var_tx_dn12 = assign19890_e27354_d_n12;
        locals.var_tx_dn17 = assign19890_e27354_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign19900_e27367, assign19900_e27367_d_n0, assign19900_e27367_d_n2, assign19900_e27367_d_n6, assign19900_e27367_d_n7, assign19900_e27367_d_n10, assign19900_e27367_d_n11, assign19900_e27367_d_n12, assign19900_e27367_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19900_e27359: f64 = (10.0 * 2.220446049250313e-16);
        let (assign19900_e27365, assign19900_e27365_d_n0, assign19900_e27365_d_n2, assign19900_e27365_d_n6, assign19900_e27365_d_n7, assign19900_e27365_d_n10, assign19900_e27365_d_n11, assign19900_e27365_d_n12, assign19900_e27365_d_n17,) = {
            if (locals.var_tx >= assign19900_e27359) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign19900_e27364: f64 = (10.0 * 2.220446049250313e-16);
                (assign19900_e27364, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign19900_e27365, assign19900_e27365_d_n0, assign19900_e27365_d_n2, assign19900_e27365_d_n6, assign19900_e27365_d_n7, assign19900_e27365_d_n10, assign19900_e27365_d_n11, assign19900_e27365_d_n12, assign19900_e27365_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign19900_e27367;
        locals.var_tx_dn0 = assign19900_e27367_d_n0;
        locals.var_tx_dn2 = assign19900_e27367_d_n2;
        locals.var_tx_dn6 = assign19900_e27367_d_n6;
        locals.var_tx_dn7 = assign19900_e27367_d_n7;
        locals.var_tx_dn10 = assign19900_e27367_d_n10;
        locals.var_tx_dn11 = assign19900_e27367_d_n11;
        locals.var_tx_dn12 = assign19900_e27367_d_n12;
        locals.var_tx_dn17 = assign19900_e27367_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign19910_e27382, assign19910_e27382_d_n0, assign19910_e27382_d_n2, assign19910_e27382_d_n6, assign19910_e27382_d_n7, assign19910_e27382_d_n10, assign19910_e27382_d_n11, assign19910_e27382_d_n12, assign19910_e27382_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign19910_e27372: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign19910_e27374: f64 = (assign19910_e27372 * 0.5);
        let assign19910_e27377: f64 = (locals.var_tx).sqrt();
        let assign19910_e27378: f64 = (1.0 - assign19910_e27377);
        let assign19910_e27379: f64 = (assign19910_e27374 * assign19910_e27378);
        let assign19910_e27380: f64 = (locals.var_vgp__blk612 + assign19910_e27379);
        (assign19910_e27380, (locals.var_vgp__blk612_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn0 / (2.0 * assign19910_e27377)))))), (locals.var_vgp__blk612_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn2 / (2.0 * assign19910_e27377)))))), (locals.var_vgp__blk612_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn6 / (2.0 * assign19910_e27377)))))), (locals.var_vgp__blk612_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn7 / (2.0 * assign19910_e27377)))))), (locals.var_vgp__blk612_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn10 / (2.0 * assign19910_e27377)))))), (locals.var_vgp__blk612_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn11 / (2.0 * assign19910_e27377)))))), (locals.var_vgp__blk612_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn12 / (2.0 * assign19910_e27377)))))), (locals.var_vgp__blk612_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign19910_e27378) + (assign19910_e27374 * (-(locals.var_tx_dn17 / (2.0 * assign19910_e27377)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign19910_e27382;
        locals.var_ps0_inia_dn0 = assign19910_e27382_d_n0;
        locals.var_ps0_inia_dn2 = assign19910_e27382_d_n2;
        locals.var_ps0_inia_dn6 = assign19910_e27382_d_n6;
        locals.var_ps0_inia_dn7 = assign19910_e27382_d_n7;
        locals.var_ps0_inia_dn10 = assign19910_e27382_d_n10;
        locals.var_ps0_inia_dn11 = assign19910_e27382_d_n11;
        locals.var_ps0_inia_dn12 = assign19910_e27382_d_n12;
        locals.var_ps0_inia_dn17 = assign19910_e27382_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let assign19920_e27386: f64 = (locals.var_vfb + locals.var_vth__blk628);
        let assign19920_e27388: f64 = (assign19920_e27386 * 0.5);
        let assign19920_e27389: f64 = if locals.var_vgs < assign19920_e27388 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign19920_e27389;
        locals.var_guard629_rv = 0.0;

        let (assign19930_e27395,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard629 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_pprv,)
    }
};
        locals.var_flg_pprv = assign19930_e27395;
        locals.var_flg_pprv_rv = 0.0;

        let assign19940_e27400: f64 = if ((locals.var_flg_pprv == 0.0) || (1.0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard630 = assign19940_e27400;
        locals.var_guard630_rv = 0.0;

        let (assign19950_e27410, assign19950_e27410_d_n0, assign19950_e27410_d_n2, assign19950_e27410_d_n6, assign19950_e27410_d_n7, assign19950_e27410_d_n10, assign19950_e27410_d_n11, assign19950_e27410_d_n12, assign19950_e27410_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) {
        let assign19950_e27407: f64 = (locals.var_ps0_inia - locals.var_vbs__blk627);
        let assign19950_e27408: f64 = (locals.var_beta * assign19950_e27407);
        (assign19950_e27408, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbs__blk627_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbs__blk627_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbs__blk627_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbs__blk627_dn7)), ((locals.var_beta_dn10 * assign19950_e27407) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbs__blk627_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbs__blk627_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbs__blk627_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbs__blk627_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign19950_e27410;
        locals.var_chi_dn0 = assign19950_e27410_d_n0;
        locals.var_chi_dn2 = assign19950_e27410_d_n2;
        locals.var_chi_dn6 = assign19950_e27410_d_n6;
        locals.var_chi_dn7 = assign19950_e27410_d_n7;
        locals.var_chi_dn10 = assign19950_e27410_d_n10;
        locals.var_chi_dn11 = assign19950_e27410_d_n11;
        locals.var_chi_dn12 = assign19950_e27410_d_n12;
        locals.var_chi_dn17 = assign19950_e27410_d_n17;
        locals.var_chi_rv = 0.0;

        let assign19960_e27413: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard631 = assign19960_e27413;
        locals.var_guard631_rv = 0.0;

        let (assign19970_e27425, assign19970_e27425_d_n0, assign19970_e27425_d_n2, assign19970_e27425_d_n6, assign19970_e27425_d_n7, assign19970_e27425_d_n10, assign19970_e27425_d_n11, assign19970_e27425_d_n12, assign19970_e27425_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign19970_e27422: f64 = (locals.var_vgp__blk612 - locals.var_vbs__blk627);
        let assign19970_e27423: f64 = (locals.var_beta * assign19970_e27422);
        (assign19970_e27423, (locals.var_beta * (locals.var_vgp__blk612_dn0 - locals.var_vbs__blk627_dn0)), (locals.var_beta * (locals.var_vgp__blk612_dn2 - locals.var_vbs__blk627_dn2)), (locals.var_beta * (locals.var_vgp__blk612_dn6 - locals.var_vbs__blk627_dn6)), (locals.var_beta * (locals.var_vgp__blk612_dn7 - locals.var_vbs__blk627_dn7)), ((locals.var_beta_dn10 * assign19970_e27422) + (locals.var_beta * (locals.var_vgp__blk612_dn10 - locals.var_vbs__blk627_dn10))), (locals.var_beta * (locals.var_vgp__blk612_dn11 - locals.var_vbs__blk627_dn11)), (locals.var_beta * (locals.var_vgp__blk612_dn12 - locals.var_vbs__blk627_dn12)), (locals.var_beta * (locals.var_vgp__blk612_dn17 - locals.var_vbs__blk627_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign19970_e27425;
        locals.var_ty_dn0 = assign19970_e27425_d_n0;
        locals.var_ty_dn2 = assign19970_e27425_d_n2;
        locals.var_ty_dn6 = assign19970_e27425_d_n6;
        locals.var_ty_dn7 = assign19970_e27425_d_n7;
        locals.var_ty_dn10 = assign19970_e27425_d_n10;
        locals.var_ty_dn11 = assign19970_e27425_d_n11;
        locals.var_ty_dn12 = assign19970_e27425_d_n12;
        locals.var_ty_dn17 = assign19970_e27425_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign19980_e27441, assign19980_e27441_d_n0, assign19980_e27441_d_n2, assign19980_e27441_d_n6, assign19980_e27441_d_n7, assign19980_e27441_d_n10, assign19980_e27441_d_n11, assign19980_e27441_d_n12, assign19980_e27441_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign19980_e27434: f64 = (1.414213562373095 / 108.0);
        let assign19980_e27436: f64 = (assign19980_e27434 * locals.var_beta);
        let assign19980_e27438: f64 = (assign19980_e27436 * locals.var_fac1);
        let assign19980_e27439: f64 = (1.0 / assign19980_e27438);
        (assign19980_e27439, (-((assign19980_e27436 * locals.var_fac1_dn0) / (assign19980_e27438 * assign19980_e27438))), (-((assign19980_e27436 * locals.var_fac1_dn2) / (assign19980_e27438 * assign19980_e27438))), (-((assign19980_e27436 * locals.var_fac1_dn6) / (assign19980_e27438 * assign19980_e27438))), (-((assign19980_e27436 * locals.var_fac1_dn7) / (assign19980_e27438 * assign19980_e27438))), (-((((assign19980_e27434 * locals.var_beta_dn10) * locals.var_fac1) + (assign19980_e27436 * locals.var_fac1_dn10)) / (assign19980_e27438 * assign19980_e27438))), (-((assign19980_e27436 * locals.var_fac1_dn11) / (assign19980_e27438 * assign19980_e27438))), (-((assign19980_e27436 * locals.var_fac1_dn12) / (assign19980_e27438 * assign19980_e27438))), (-((assign19980_e27436 * locals.var_fac1_dn17) / (assign19980_e27438 * assign19980_e27438))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign19980_e27441;
        locals.var_t1_dn0 = assign19980_e27441_d_n0;
        locals.var_t1_dn2 = assign19980_e27441_d_n2;
        locals.var_t1_dn6 = assign19980_e27441_d_n6;
        locals.var_t1_dn7 = assign19980_e27441_d_n7;
        locals.var_t1_dn10 = assign19980_e27441_d_n10;
        locals.var_t1_dn11 = assign19980_e27441_d_n11;
        locals.var_t1_dn12 = assign19980_e27441_d_n12;
        locals.var_t1_dn17 = assign19980_e27441_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign19990_e27453, assign19990_e27453_d_n0, assign19990_e27453_d_n2, assign19990_e27453_d_n6, assign19990_e27453_d_n7, assign19990_e27453_d_n10, assign19990_e27453_d_n11, assign19990_e27453_d_n12, assign19990_e27453_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign19990_e27450: f64 = (3.0 * locals.var_t1);
        let assign19990_e27451: f64 = (81.0 + assign19990_e27450);
        (assign19990_e27451, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign19990_e27453;
        locals.var_t2_dn0 = assign19990_e27453_d_n0;
        locals.var_t2_dn2 = assign19990_e27453_d_n2;
        locals.var_t2_dn6 = assign19990_e27453_d_n6;
        locals.var_t2_dn7 = assign19990_e27453_d_n7;
        locals.var_t2_dn10 = assign19990_e27453_d_n10;
        locals.var_t2_dn11 = assign19990_e27453_d_n11;
        locals.var_t2_dn12 = assign19990_e27453_d_n12;
        locals.var_t2_dn17 = assign19990_e27453_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20000_e27472, assign20000_e27472_d_n0, assign20000_e27472_d_n2, assign20000_e27472_d_n6, assign20000_e27472_d_n7, assign20000_e27472_d_n10, assign20000_e27472_d_n11, assign20000_e27472_d_n12, assign20000_e27472_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20000_e27460: f64 = (-2916.0);
        let assign20000_e27463: f64 = (81.0 * locals.var_t1);
        let assign20000_e27464: f64 = (assign20000_e27460 - assign20000_e27463);
        let assign20000_e27467: f64 = (27.0 * locals.var_t1);
        let assign20000_e27469: f64 = (assign20000_e27467 * locals.var_ty);
        let assign20000_e27470: f64 = (assign20000_e27464 + assign20000_e27469);
        (assign20000_e27470, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign20000_e27467 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20000_e27472;
        locals.var_t3_dn0 = assign20000_e27472_d_n0;
        locals.var_t3_dn2 = assign20000_e27472_d_n2;
        locals.var_t3_dn6 = assign20000_e27472_d_n6;
        locals.var_t3_dn7 = assign20000_e27472_d_n7;
        locals.var_t3_dn10 = assign20000_e27472_d_n10;
        locals.var_t3_dn11 = assign20000_e27472_d_n11;
        locals.var_t3_dn12 = assign20000_e27472_d_n12;
        locals.var_t3_dn17 = assign20000_e27472_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20010_e27492, assign20010_e27492_d_n0, assign20010_e27492_d_n2, assign20010_e27492_d_n6, assign20010_e27492_d_n7, assign20010_e27492_d_n10, assign20010_e27492_d_n11, assign20010_e27492_d_n12, assign20010_e27492_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20010_e27482: f64 = (54.0 + locals.var_t1);
        let assign20010_e27483: f64 = (81.0 * assign20010_e27482);
        let assign20010_e27484: f64 = (1458.0 - assign20010_e27483);
        let assign20010_e27487: f64 = (27.0 * locals.var_t1);
        let assign20010_e27489: f64 = (assign20010_e27487 * locals.var_ty);
        let assign20010_e27490: f64 = (assign20010_e27484 + assign20010_e27489);
        (assign20010_e27490, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign20010_e27487 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20010_e27492;
        locals.var_t4_dn0 = assign20010_e27492_d_n0;
        locals.var_t4_dn2 = assign20010_e27492_d_n2;
        locals.var_t4_dn6 = assign20010_e27492_d_n6;
        locals.var_t4_dn7 = assign20010_e27492_d_n7;
        locals.var_t4_dn10 = assign20010_e27492_d_n10;
        locals.var_t4_dn11 = assign20010_e27492_d_n11;
        locals.var_t4_dn12 = assign20010_e27492_d_n12;
        locals.var_t4_dn17 = assign20010_e27492_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20020_e27502, assign20020_e27502_d_n0, assign20020_e27502_d_n2, assign20020_e27502_d_n6, assign20020_e27502_d_n7, assign20020_e27502_d_n10, assign20020_e27502_d_n11, assign20020_e27502_d_n12, assign20020_e27502_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20020_e27500: f64 = (locals.var_t4 * locals.var_t4);
        (assign20020_e27500, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20020_e27502;
        locals.var_t4_dn0 = assign20020_e27502_d_n0;
        locals.var_t4_dn2 = assign20020_e27502_d_n2;
        locals.var_t4_dn6 = assign20020_e27502_d_n6;
        locals.var_t4_dn7 = assign20020_e27502_d_n7;
        locals.var_t4_dn10 = assign20020_e27502_d_n10;
        locals.var_t4_dn11 = assign20020_e27502_d_n11;
        locals.var_t4_dn12 = assign20020_e27502_d_n12;
        locals.var_t4_dn17 = assign20020_e27502_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20030_e27523, assign20030_e27523_d_n0, assign20030_e27523_d_n2, assign20030_e27523_d_n6, assign20030_e27523_d_n7, assign20030_e27523_d_n10, assign20030_e27523_d_n11, assign20030_e27523_d_n12, assign20030_e27523_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20030_e27511: f64 = (4.0 * locals.var_t2);
        let assign20030_e27513: f64 = (assign20030_e27511 * locals.var_t2);
        let assign20030_e27515: f64 = (assign20030_e27513 * locals.var_t2);
        let assign20030_e27517: f64 = (assign20030_e27515 + locals.var_t4);
        let assign20030_e27518: f64 = (assign20030_e27517).sqrt();
        let assign20030_e27519: f64 = (locals.var_t3 + assign20030_e27518);
        let assign20030_e27521: f64 = (assign20030_e27519).powf(0.3333333333333333);
        (assign20030_e27521, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn0)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn0)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign20030_e27518))) / assign20030_e27519))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn2)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn2)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign20030_e27518))) / assign20030_e27519))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn6)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn6)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign20030_e27518))) / assign20030_e27519))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn7)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn7)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign20030_e27518))) / assign20030_e27519))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn10)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn10)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign20030_e27518))) / assign20030_e27519))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn11)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn11)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign20030_e27518))) / assign20030_e27519))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn12)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn12)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign20030_e27518))) / assign20030_e27519))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20030_e27519).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn17)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign20030_e27518))))) } } else { (assign20030_e27521 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign20030_e27511 * locals.var_t2_dn17)) * locals.var_t2) + (assign20030_e27513 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign20030_e27518))) / assign20030_e27519))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign20030_e27523;
        locals.var_t5_dn0 = assign20030_e27523_d_n0;
        locals.var_t5_dn2 = assign20030_e27523_d_n2;
        locals.var_t5_dn6 = assign20030_e27523_d_n6;
        locals.var_t5_dn7 = assign20030_e27523_d_n7;
        locals.var_t5_dn10 = assign20030_e27523_d_n10;
        locals.var_t5_dn11 = assign20030_e27523_d_n11;
        locals.var_t5_dn12 = assign20030_e27523_d_n12;
        locals.var_t5_dn17 = assign20030_e27523_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign20040_e27547, assign20040_e27547_d_n0, assign20040_e27547_d_n2, assign20040_e27547_d_n6, assign20040_e27547_d_n7, assign20040_e27547_d_n10, assign20040_e27547_d_n11, assign20040_e27547_d_n12, assign20040_e27547_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20040_e27532: f64 = (1.259921049894873 * locals.var_t2);
        let assign20040_e27535: f64 = (3.0 * locals.var_t5);
        let assign20040_e27536: f64 = (assign20040_e27532 / assign20040_e27535);
        let assign20040_e27537: f64 = (3.0 - assign20040_e27536);
        let assign20040_e27541: f64 = (3.0 * 1.259921049894873);
        let assign20040_e27542: f64 = (1.0 / assign20040_e27541);
        let assign20040_e27544: f64 = (assign20040_e27542 * locals.var_t5);
        let assign20040_e27545: f64 = (assign20040_e27537 + assign20040_e27544);
        (assign20040_e27545, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn0))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn2))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn6))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn7))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn10))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn11))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn12))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign20040_e27535) - (assign20040_e27532 * (3.0 * locals.var_t5_dn17))) / (assign20040_e27535 * assign20040_e27535))) + (assign20040_e27542 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign20040_e27547;
        locals.var_tx_dn0 = assign20040_e27547_d_n0;
        locals.var_tx_dn2 = assign20040_e27547_d_n2;
        locals.var_tx_dn6 = assign20040_e27547_d_n6;
        locals.var_tx_dn7 = assign20040_e27547_d_n7;
        locals.var_tx_dn10 = assign20040_e27547_d_n10;
        locals.var_tx_dn11 = assign20040_e27547_d_n11;
        locals.var_tx_dn12 = assign20040_e27547_d_n12;
        locals.var_tx_dn17 = assign20040_e27547_d_n17;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20050_e27559, assign20050_e27559_d_n0, assign20050_e27559_d_n2, assign20050_e27559_d_n6, assign20050_e27559_d_n7, assign20050_e27559_d_n10, assign20050_e27559_d_n11, assign20050_e27559_d_n12, assign20050_e27559_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20050_e27555: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign20050_e27557: f64 = (assign20050_e27555 + locals.var_vbs__blk627);
        (assign20050_e27557, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbs__blk627_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbs__blk627_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbs__blk627_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbs__blk627_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbs__blk627_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbs__blk627_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbs__blk627_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbs__blk627_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20050_e27559;
        locals.var_ps0_inia_dn0 = assign20050_e27559_d_n0;
        locals.var_ps0_inia_dn2 = assign20050_e27559_d_n2;
        locals.var_ps0_inia_dn6 = assign20050_e27559_d_n6;
        locals.var_ps0_inia_dn7 = assign20050_e27559_d_n7;
        locals.var_ps0_inia_dn10 = assign20050_e27559_d_n10;
        locals.var_ps0_inia_dn11 = assign20050_e27559_d_n11;
        locals.var_ps0_inia_dn12 = assign20050_e27559_d_n12;
        locals.var_ps0_inia_dn17 = assign20050_e27559_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20060_e27567, assign20060_e27567_d_n0, assign20060_e27567_d_n2, assign20060_e27567_d_n6, assign20060_e27567_d_n7, assign20060_e27567_d_n10, assign20060_e27567_d_n11, assign20060_e27567_d_n12, assign20060_e27567_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20060_e27567;
        locals.var_ps0_ini_dn0 = assign20060_e27567_d_n0;
        locals.var_ps0_ini_dn2 = assign20060_e27567_d_n2;
        locals.var_ps0_ini_dn6 = assign20060_e27567_d_n6;
        locals.var_ps0_ini_dn7 = assign20060_e27567_d_n7;
        locals.var_ps0_ini_dn10 = assign20060_e27567_d_n10;
        locals.var_ps0_ini_dn11 = assign20060_e27567_d_n11;
        locals.var_ps0_ini_dn12 = assign20060_e27567_d_n12;
        locals.var_ps0_ini_dn17 = assign20060_e27567_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20070_e27570: f64 = (locals.var_vgs - locals.var_shift);
        let assign20070_e27572: f64 = if assign20070_e27570 <= locals.var_vth__blk628 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign20070_e27572;
        locals.var_guard632_rv = 0.0;

        let assign20080_e27575: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign20080_e27575;
        locals.var_guard633_rv = 0.0;

        let (assign20090_e27590, assign20090_e27590_d_n0, assign20090_e27590_d_n2, assign20090_e27590_d_n6, assign20090_e27590_d_n7, assign20090_e27590_d_n10, assign20090_e27590_d_n11, assign20090_e27590_d_n12, assign20090_e27590_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20090_e27588: f64 = (1.0 / locals.var_c_fox);
        (assign20090_e27588, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20090_e27590;
        locals.var_t0_dn0 = assign20090_e27590_d_n0;
        locals.var_t0_dn2 = assign20090_e27590_d_n2;
        locals.var_t0_dn6 = assign20090_e27590_d_n6;
        locals.var_t0_dn7 = assign20090_e27590_d_n7;
        locals.var_t0_dn10 = assign20090_e27590_d_n10;
        locals.var_t0_dn11 = assign20090_e27590_d_n11;
        locals.var_t0_dn12 = assign20090_e27590_d_n12;
        locals.var_t0_dn17 = assign20090_e27590_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20100_e27605, assign20100_e27605_d_n0, assign20100_e27605_d_n2, assign20100_e27605_d_n6, assign20100_e27605_d_n7, assign20100_e27605_d_n10, assign20100_e27605_d_n11, assign20100_e27605_d_n12, assign20100_e27605_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20100_e27603: f64 = (locals.var_t_soi__blk611 / 1.034943e-10);
        (assign20100_e27603, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20100_e27605;
        locals.var_t1_dn0 = assign20100_e27605_d_n0;
        locals.var_t1_dn2 = assign20100_e27605_d_n2;
        locals.var_t1_dn6 = assign20100_e27605_d_n6;
        locals.var_t1_dn7 = assign20100_e27605_d_n7;
        locals.var_t1_dn10 = assign20100_e27605_d_n10;
        locals.var_t1_dn11 = assign20100_e27605_d_n11;
        locals.var_t1_dn12 = assign20100_e27605_d_n12;
        locals.var_t1_dn17 = assign20100_e27605_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20110_e27620, assign20110_e27620_d_n0, assign20110_e27620_d_n2, assign20110_e27620_d_n6, assign20110_e27620_d_n7, assign20110_e27620_d_n10, assign20110_e27620_d_n11, assign20110_e27620_d_n12, assign20110_e27620_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20110_e27618: f64 = (1.0 / locals.var_c_box);
        (assign20110_e27618, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20110_e27620;
        locals.var_t2_dn0 = assign20110_e27620_d_n0;
        locals.var_t2_dn2 = assign20110_e27620_d_n2;
        locals.var_t2_dn6 = assign20110_e27620_d_n6;
        locals.var_t2_dn7 = assign20110_e27620_d_n7;
        locals.var_t2_dn10 = assign20110_e27620_d_n10;
        locals.var_t2_dn11 = assign20110_e27620_d_n11;
        locals.var_t2_dn12 = assign20110_e27620_d_n12;
        locals.var_t2_dn17 = assign20110_e27620_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20120_e27639, assign20120_e27639_d_n0, assign20120_e27639_d_n2, assign20120_e27639_d_n6, assign20120_e27639_d_n7, assign20120_e27639_d_n10, assign20120_e27639_d_n11, assign20120_e27639_d_n12, assign20120_e27639_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20120_e27634: f64 = (locals.var_t0 + locals.var_t1);
        let assign20120_e27636: f64 = (assign20120_e27634 + locals.var_t2);
        let assign20120_e27637: f64 = (1.0 / assign20120_e27636);
        (assign20120_e27637, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20120_e27636 * assign20120_e27636))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20120_e27636 * assign20120_e27636))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20120_e27636 * assign20120_e27636))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20120_e27636 * assign20120_e27636))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20120_e27636 * assign20120_e27636))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20120_e27636 * assign20120_e27636))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20120_e27636 * assign20120_e27636))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20120_e27636 * assign20120_e27636))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20120_e27639;
        locals.var_t3_dn0 = assign20120_e27639_d_n0;
        locals.var_t3_dn2 = assign20120_e27639_d_n2;
        locals.var_t3_dn6 = assign20120_e27639_d_n6;
        locals.var_t3_dn7 = assign20120_e27639_d_n7;
        locals.var_t3_dn10 = assign20120_e27639_d_n10;
        locals.var_t3_dn11 = assign20120_e27639_d_n11;
        locals.var_t3_dn12 = assign20120_e27639_d_n12;
        locals.var_t3_dn17 = assign20120_e27639_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20130_e27665, assign20130_e27665_d_n0, assign20130_e27665_d_n2, assign20130_e27665_d_n6, assign20130_e27665_d_n7, assign20130_e27665_d_n10, assign20130_e27665_d_n11, assign20130_e27665_d_n12, assign20130_e27665_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20130_e27653: f64 = (locals.var_vgp__blk612 - locals.var_vbsbiz);
        let assign20130_e27657: f64 = (0.5 * locals.var_t1);
        let assign20130_e27658: f64 = (locals.var_t2 + assign20130_e27657);
        let assign20130_e27660: f64 = (-locals.var_q_s0_dep_ini);
        let assign20130_e27661: f64 = (assign20130_e27658 * assign20130_e27660);
        let assign20130_e27662: f64 = (assign20130_e27653 + assign20130_e27661);
        let assign20130_e27663: f64 = (locals.var_t3 * assign20130_e27662);
        (assign20130_e27663, ((locals.var_t3_dn0 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20130_e27662) + (locals.var_t3 * ((locals.var_vgp__blk612_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20130_e27660) + (assign20130_e27658 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20130_e27665;
        locals.var_t4_dn0 = assign20130_e27665_d_n0;
        locals.var_t4_dn2 = assign20130_e27665_d_n2;
        locals.var_t4_dn6 = assign20130_e27665_d_n6;
        locals.var_t4_dn7 = assign20130_e27665_d_n7;
        locals.var_t4_dn10 = assign20130_e27665_d_n10;
        locals.var_t4_dn11 = assign20130_e27665_d_n11;
        locals.var_t4_dn12 = assign20130_e27665_d_n12;
        locals.var_t4_dn17 = assign20130_e27665_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20140_e27682, assign20140_e27682_d_n0, assign20140_e27682_d_n2, assign20140_e27682_d_n6, assign20140_e27682_d_n7, assign20140_e27682_d_n10, assign20140_e27682_d_n11, assign20140_e27682_d_n12, assign20140_e27682_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20140_e27679: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20140_e27680: f64 = (locals.var_vgp__blk612 - assign20140_e27679);
        (assign20140_e27680, (locals.var_vgp__blk612_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20140_e27682;
        locals.var_ps0_inia_dn0 = assign20140_e27682_d_n0;
        locals.var_ps0_inia_dn2 = assign20140_e27682_d_n2;
        locals.var_ps0_inia_dn6 = assign20140_e27682_d_n6;
        locals.var_ps0_inia_dn7 = assign20140_e27682_d_n7;
        locals.var_ps0_inia_dn10 = assign20140_e27682_d_n10;
        locals.var_ps0_inia_dn11 = assign20140_e27682_d_n11;
        locals.var_ps0_inia_dn12 = assign20140_e27682_d_n12;
        locals.var_ps0_inia_dn17 = assign20140_e27682_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20150_e27693, assign20150_e27693_d_n0, assign20150_e27693_d_n2, assign20150_e27693_d_n6, assign20150_e27693_d_n7, assign20150_e27693_d_n10, assign20150_e27693_d_n11, assign20150_e27693_d_n12, assign20150_e27693_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20150_e27693;
        locals.var_ps0_ini_dn0 = assign20150_e27693_d_n0;
        locals.var_ps0_ini_dn2 = assign20150_e27693_d_n2;
        locals.var_ps0_ini_dn6 = assign20150_e27693_d_n6;
        locals.var_ps0_ini_dn7 = assign20150_e27693_d_n7;
        locals.var_ps0_ini_dn10 = assign20150_e27693_d_n10;
        locals.var_ps0_ini_dn11 = assign20150_e27693_d_n11;
        locals.var_ps0_ini_dn12 = assign20150_e27693_d_n12;
        locals.var_ps0_ini_dn17 = assign20150_e27693_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20160_e27709, assign20160_e27709_d_n0, assign20160_e27709_d_n2, assign20160_e27709_d_n6, assign20160_e27709_d_n7, assign20160_e27709_d_n10, assign20160_e27709_d_n11, assign20160_e27709_d_n12, assign20160_e27709_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20160_e27705: f64 = (1.0 / locals.var_cnst1soi);
        let assign20160_e27707: f64 = (assign20160_e27705 / locals.var_cnstc_foxi);
        (assign20160_e27707, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20160_e27705 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20160_e27709;
        locals.var_t1_dn0 = assign20160_e27709_d_n0;
        locals.var_t1_dn2 = assign20160_e27709_d_n2;
        locals.var_t1_dn6 = assign20160_e27709_d_n6;
        locals.var_t1_dn7 = assign20160_e27709_d_n7;
        locals.var_t1_dn10 = assign20160_e27709_d_n10;
        locals.var_t1_dn11 = assign20160_e27709_d_n11;
        locals.var_t1_dn12 = assign20160_e27709_d_n12;
        locals.var_t1_dn17 = assign20160_e27709_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20170_e27729, assign20170_e27729_d_n0, assign20170_e27729_d_n2, assign20170_e27729_d_n6, assign20170_e27729_d_n7, assign20170_e27729_d_n10, assign20170_e27729_d_n11, assign20170_e27729_d_n12, assign20170_e27729_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20170_e27722: f64 = (locals.var_vgp__blk612 - locals.var_shift);
        let assign20170_e27723: f64 = (locals.var_t1 * assign20170_e27722);
        let assign20170_e27726: f64 = (locals.var_vgp__blk612 - locals.var_shift);
        let assign20170_e27727: f64 = (assign20170_e27723 * assign20170_e27726);
        (assign20170_e27727, ((((locals.var_t1_dn0 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn0 - locals.var_shift_dn0))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn2 - locals.var_shift_dn2))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn6 - locals.var_shift_dn6))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn7 - locals.var_shift_dn7))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn10 - locals.var_shift_dn10))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn11 - locals.var_shift_dn11))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn12 - locals.var_shift_dn12))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign20170_e27722) + (locals.var_t1 * (locals.var_vgp__blk612_dn17 - locals.var_shift_dn17))) * assign20170_e27726) + (assign20170_e27723 * (locals.var_vgp__blk612_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20170_e27729;
        locals.var_t2_dn0 = assign20170_e27729_d_n0;
        locals.var_t2_dn2 = assign20170_e27729_d_n2;
        locals.var_t2_dn6 = assign20170_e27729_d_n6;
        locals.var_t2_dn7 = assign20170_e27729_d_n7;
        locals.var_t2_dn10 = assign20170_e27729_d_n10;
        locals.var_t2_dn11 = assign20170_e27729_d_n11;
        locals.var_t2_dn12 = assign20170_e27729_d_n12;
        locals.var_t2_dn17 = assign20170_e27729_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20180_e27747, assign20180_e27747_d_n0, assign20180_e27747_d_n2, assign20180_e27747_d_n6, assign20180_e27747_d_n7, assign20180_e27747_d_n10, assign20180_e27747_d_n11, assign20180_e27747_d_n12, assign20180_e27747_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20180_e27743: f64 = (locals.var_vgp__blk612 - locals.var_shift);
        let assign20180_e27744: f64 = (2.0 / assign20180_e27743);
        let assign20180_e27745: f64 = (locals.var_beta + assign20180_e27744);
        (assign20180_e27745, (-((2.0 * (locals.var_vgp__blk612_dn0 - locals.var_shift_dn0)) / (assign20180_e27743 * assign20180_e27743))), (-((2.0 * (locals.var_vgp__blk612_dn2 - locals.var_shift_dn2)) / (assign20180_e27743 * assign20180_e27743))), (-((2.0 * (locals.var_vgp__blk612_dn6 - locals.var_shift_dn6)) / (assign20180_e27743 * assign20180_e27743))), (-((2.0 * (locals.var_vgp__blk612_dn7 - locals.var_shift_dn7)) / (assign20180_e27743 * assign20180_e27743))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp__blk612_dn10 - locals.var_shift_dn10)) / (assign20180_e27743 * assign20180_e27743)))), (-((2.0 * (locals.var_vgp__blk612_dn11 - locals.var_shift_dn11)) / (assign20180_e27743 * assign20180_e27743))), (-((2.0 * (locals.var_vgp__blk612_dn12 - locals.var_shift_dn12)) / (assign20180_e27743 * assign20180_e27743))), (-((2.0 * (locals.var_vgp__blk612_dn17 - locals.var_shift_dn17)) / (assign20180_e27743 * assign20180_e27743))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20180_e27747;
        locals.var_t3_dn0 = assign20180_e27747_d_n0;
        locals.var_t3_dn2 = assign20180_e27747_d_n2;
        locals.var_t3_dn6 = assign20180_e27747_d_n6;
        locals.var_t3_dn7 = assign20180_e27747_d_n7;
        locals.var_t3_dn10 = assign20180_e27747_d_n10;
        locals.var_t3_dn11 = assign20180_e27747_d_n11;
        locals.var_t3_dn12 = assign20180_e27747_d_n12;
        locals.var_t3_dn17 = assign20180_e27747_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20190_e27764, assign20190_e27764_d_n0, assign20190_e27764_d_n2, assign20190_e27764_d_n6, assign20190_e27764_d_n7, assign20190_e27764_d_n10, assign20190_e27764_d_n11, assign20190_e27764_d_n12, assign20190_e27764_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20190_e27758: f64 = (locals.var_t2).ln();
        let assign20190_e27760: f64 = (assign20190_e27758 / locals.var_t3);
        let assign20190_e27762: f64 = (assign20190_e27760 + p.p287);
        (assign20190_e27762, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign20190_e27758 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign20190_e27764;
        locals.var_ps0_inib_dn0 = assign20190_e27764_d_n0;
        locals.var_ps0_inib_dn2 = assign20190_e27764_d_n2;
        locals.var_ps0_inib_dn6 = assign20190_e27764_d_n6;
        locals.var_ps0_inib_dn7 = assign20190_e27764_d_n7;
        locals.var_ps0_inib_dn10 = assign20190_e27764_d_n10;
        locals.var_ps0_inib_dn11 = assign20190_e27764_d_n11;
        locals.var_ps0_inib_dn12 = assign20190_e27764_d_n12;
        locals.var_ps0_inib_dn17 = assign20190_e27764_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign20200_e27780, assign20200_e27780_d_n0, assign20200_e27780_d_n2, assign20200_e27780_d_n6, assign20200_e27780_d_n7, assign20200_e27780_d_n10, assign20200_e27780_d_n11, assign20200_e27780_d_n12, assign20200_e27780_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20200_e27776: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign20200_e27778: f64 = (assign20200_e27776 - 0.0008);
        (assign20200_e27778, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20200_e27780;
        locals.var_tmf1_dn0 = assign20200_e27780_d_n0;
        locals.var_tmf1_dn2 = assign20200_e27780_d_n2;
        locals.var_tmf1_dn6 = assign20200_e27780_d_n6;
        locals.var_tmf1_dn7 = assign20200_e27780_d_n7;
        locals.var_tmf1_dn10 = assign20200_e27780_d_n10;
        locals.var_tmf1_dn11 = assign20200_e27780_d_n11;
        locals.var_tmf1_dn12 = assign20200_e27780_d_n12;
        locals.var_tmf1_dn17 = assign20200_e27780_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20210_e27796, assign20210_e27796_d_n0, assign20210_e27796_d_n2, assign20210_e27796_d_n6, assign20210_e27796_d_n7, assign20210_e27796_d_n10, assign20210_e27796_d_n11, assign20210_e27796_d_n12, assign20210_e27796_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20210_e27792: f64 = (4.0 * locals.var_ps0_inib);
        let assign20210_e27794: f64 = (assign20210_e27792 * 0.0008);
        (assign20210_e27794, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20210_e27796;
        locals.var_tmf2_dn0 = assign20210_e27796_d_n0;
        locals.var_tmf2_dn2 = assign20210_e27796_d_n2;
        locals.var_tmf2_dn6 = assign20210_e27796_d_n6;
        locals.var_tmf2_dn7 = assign20210_e27796_d_n7;
        locals.var_tmf2_dn10 = assign20210_e27796_d_n10;
        locals.var_tmf2_dn11 = assign20210_e27796_d_n11;
        locals.var_tmf2_dn12 = assign20210_e27796_d_n12;
        locals.var_tmf2_dn17 = assign20210_e27796_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20220_e27814, assign20220_e27814_d_n0, assign20220_e27814_d_n2, assign20220_e27814_d_n6, assign20220_e27814_d_n7, assign20220_e27814_d_n10, assign20220_e27814_d_n11, assign20220_e27814_d_n12, assign20220_e27814_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let (assign20220_e27812, assign20220_e27812_d_n0, assign20220_e27812_d_n2, assign20220_e27812_d_n6, assign20220_e27812_d_n7, assign20220_e27812_d_n10, assign20220_e27812_d_n11, assign20220_e27812_d_n12, assign20220_e27812_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign20220_e27811: f64 = (-locals.var_tmf2);
                (assign20220_e27811, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign20220_e27812, assign20220_e27812_d_n0, assign20220_e27812_d_n2, assign20220_e27812_d_n6, assign20220_e27812_d_n7, assign20220_e27812_d_n10, assign20220_e27812_d_n11, assign20220_e27812_d_n12, assign20220_e27812_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20220_e27814;
        locals.var_tmf2_dn0 = assign20220_e27814_d_n0;
        locals.var_tmf2_dn2 = assign20220_e27814_d_n2;
        locals.var_tmf2_dn6 = assign20220_e27814_d_n6;
        locals.var_tmf2_dn7 = assign20220_e27814_d_n7;
        locals.var_tmf2_dn10 = assign20220_e27814_d_n10;
        locals.var_tmf2_dn11 = assign20220_e27814_d_n11;
        locals.var_tmf2_dn12 = assign20220_e27814_d_n12;
        locals.var_tmf2_dn17 = assign20220_e27814_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20230_e27831, assign20230_e27831_d_n0, assign20230_e27831_d_n2, assign20230_e27831_d_n6, assign20230_e27831_d_n7, assign20230_e27831_d_n10, assign20230_e27831_d_n11, assign20230_e27831_d_n12, assign20230_e27831_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20230_e27826: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20230_e27828: f64 = (assign20230_e27826 + locals.var_tmf2);
        let assign20230_e27829: f64 = (assign20230_e27828).sqrt();
        (assign20230_e27829, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20230_e27829)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20230_e27829)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20230_e27829)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20230_e27829)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20230_e27829)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20230_e27829)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20230_e27829)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign20230_e27829)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20230_e27831;
        locals.var_tmf2_dn0 = assign20230_e27831_d_n0;
        locals.var_tmf2_dn2 = assign20230_e27831_d_n2;
        locals.var_tmf2_dn6 = assign20230_e27831_d_n6;
        locals.var_tmf2_dn7 = assign20230_e27831_d_n7;
        locals.var_tmf2_dn10 = assign20230_e27831_d_n10;
        locals.var_tmf2_dn11 = assign20230_e27831_d_n11;
        locals.var_tmf2_dn12 = assign20230_e27831_d_n12;
        locals.var_tmf2_dn17 = assign20230_e27831_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20240_e27849, assign20240_e27849_d_n0, assign20240_e27849_d_n2, assign20240_e27849_d_n6, assign20240_e27849_d_n7, assign20240_e27849_d_n10, assign20240_e27849_d_n11, assign20240_e27849_d_n12, assign20240_e27849_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign20240_e27845: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20240_e27846: f64 = (0.5 * assign20240_e27845);
        let assign20240_e27847: f64 = (locals.var_ps0_inib - assign20240_e27846);
        (assign20240_e27847, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20240_e27849;
        locals.var_ps0_ini_dn0 = assign20240_e27849_d_n0;
        locals.var_ps0_ini_dn2 = assign20240_e27849_d_n2;
        locals.var_ps0_ini_dn6 = assign20240_e27849_d_n6;
        locals.var_ps0_ini_dn7 = assign20240_e27849_d_n7;
        locals.var_ps0_ini_dn10 = assign20240_e27849_d_n10;
        locals.var_ps0_ini_dn11 = assign20240_e27849_d_n11;
        locals.var_ps0_ini_dn12 = assign20240_e27849_d_n12;
        locals.var_ps0_ini_dn17 = assign20240_e27849_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20250_e27852: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign20250_e27852;
        locals.var_guard634_rv = 0.0;

        let assign20260_e27855: f64 = (locals.var_vgs - locals.var_shift);
        let assign20260_e27857: f64 = if assign20260_e27855 <= locals.var_vth__blk628 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign20260_e27857;
        locals.var_guard635_rv = 0.0;

        let (assign20270_e27869, assign20270_e27869_d_n0, assign20270_e27869_d_n2, assign20270_e27869_d_n6, assign20270_e27869_d_n7, assign20270_e27869_d_n10, assign20270_e27869_d_n11, assign20270_e27869_d_n12, assign20270_e27869_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20270_e27867: f64 = (1.0 / locals.var_c_fox);
        (assign20270_e27867, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20270_e27869;
        locals.var_t0_dn0 = assign20270_e27869_d_n0;
        locals.var_t0_dn2 = assign20270_e27869_d_n2;
        locals.var_t0_dn6 = assign20270_e27869_d_n6;
        locals.var_t0_dn7 = assign20270_e27869_d_n7;
        locals.var_t0_dn10 = assign20270_e27869_d_n10;
        locals.var_t0_dn11 = assign20270_e27869_d_n11;
        locals.var_t0_dn12 = assign20270_e27869_d_n12;
        locals.var_t0_dn17 = assign20270_e27869_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20280_e27881, assign20280_e27881_d_n0, assign20280_e27881_d_n2, assign20280_e27881_d_n6, assign20280_e27881_d_n7, assign20280_e27881_d_n10, assign20280_e27881_d_n11, assign20280_e27881_d_n12, assign20280_e27881_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20280_e27879: f64 = (locals.var_t_soi__blk611 / 1.034943e-10);
        (assign20280_e27879, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20280_e27881;
        locals.var_t1_dn0 = assign20280_e27881_d_n0;
        locals.var_t1_dn2 = assign20280_e27881_d_n2;
        locals.var_t1_dn6 = assign20280_e27881_d_n6;
        locals.var_t1_dn7 = assign20280_e27881_d_n7;
        locals.var_t1_dn10 = assign20280_e27881_d_n10;
        locals.var_t1_dn11 = assign20280_e27881_d_n11;
        locals.var_t1_dn12 = assign20280_e27881_d_n12;
        locals.var_t1_dn17 = assign20280_e27881_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20290_e27893, assign20290_e27893_d_n0, assign20290_e27893_d_n2, assign20290_e27893_d_n6, assign20290_e27893_d_n7, assign20290_e27893_d_n10, assign20290_e27893_d_n11, assign20290_e27893_d_n12, assign20290_e27893_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20290_e27891: f64 = (1.0 / locals.var_c_box);
        (assign20290_e27891, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20290_e27893;
        locals.var_t2_dn0 = assign20290_e27893_d_n0;
        locals.var_t2_dn2 = assign20290_e27893_d_n2;
        locals.var_t2_dn6 = assign20290_e27893_d_n6;
        locals.var_t2_dn7 = assign20290_e27893_d_n7;
        locals.var_t2_dn10 = assign20290_e27893_d_n10;
        locals.var_t2_dn11 = assign20290_e27893_d_n11;
        locals.var_t2_dn12 = assign20290_e27893_d_n12;
        locals.var_t2_dn17 = assign20290_e27893_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20300_e27909, assign20300_e27909_d_n0, assign20300_e27909_d_n2, assign20300_e27909_d_n6, assign20300_e27909_d_n7, assign20300_e27909_d_n10, assign20300_e27909_d_n11, assign20300_e27909_d_n12, assign20300_e27909_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20300_e27904: f64 = (locals.var_t0 + locals.var_t1);
        let assign20300_e27906: f64 = (assign20300_e27904 + locals.var_t2);
        let assign20300_e27907: f64 = (1.0 / assign20300_e27906);
        (assign20300_e27907, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20300_e27906 * assign20300_e27906))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20300_e27906 * assign20300_e27906))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20300_e27906 * assign20300_e27906))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20300_e27906 * assign20300_e27906))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20300_e27906 * assign20300_e27906))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20300_e27906 * assign20300_e27906))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20300_e27906 * assign20300_e27906))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20300_e27906 * assign20300_e27906))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20300_e27909;
        locals.var_t3_dn0 = assign20300_e27909_d_n0;
        locals.var_t3_dn2 = assign20300_e27909_d_n2;
        locals.var_t3_dn6 = assign20300_e27909_d_n6;
        locals.var_t3_dn7 = assign20300_e27909_d_n7;
        locals.var_t3_dn10 = assign20300_e27909_d_n10;
        locals.var_t3_dn11 = assign20300_e27909_d_n11;
        locals.var_t3_dn12 = assign20300_e27909_d_n12;
        locals.var_t3_dn17 = assign20300_e27909_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20310_e27932, assign20310_e27932_d_n0, assign20310_e27932_d_n2, assign20310_e27932_d_n6, assign20310_e27932_d_n7, assign20310_e27932_d_n10, assign20310_e27932_d_n11, assign20310_e27932_d_n12, assign20310_e27932_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20310_e27920: f64 = (locals.var_vgp__blk612 - locals.var_vbsbiz);
        let assign20310_e27924: f64 = (0.5 * locals.var_t1);
        let assign20310_e27925: f64 = (locals.var_t2 + assign20310_e27924);
        let assign20310_e27927: f64 = (-locals.var_q_s0_dep_ini);
        let assign20310_e27928: f64 = (assign20310_e27925 * assign20310_e27927);
        let assign20310_e27929: f64 = (assign20310_e27920 + assign20310_e27928);
        let assign20310_e27930: f64 = (locals.var_t3 * assign20310_e27929);
        (assign20310_e27930, ((locals.var_t3_dn0 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20310_e27929) + (locals.var_t3 * ((locals.var_vgp__blk612_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20310_e27927) + (assign20310_e27925 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20310_e27932;
        locals.var_t4_dn0 = assign20310_e27932_d_n0;
        locals.var_t4_dn2 = assign20310_e27932_d_n2;
        locals.var_t4_dn6 = assign20310_e27932_d_n6;
        locals.var_t4_dn7 = assign20310_e27932_d_n7;
        locals.var_t4_dn10 = assign20310_e27932_d_n10;
        locals.var_t4_dn11 = assign20310_e27932_d_n11;
        locals.var_t4_dn12 = assign20310_e27932_d_n12;
        locals.var_t4_dn17 = assign20310_e27932_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20320_e27946, assign20320_e27946_d_n0, assign20320_e27946_d_n2, assign20320_e27946_d_n6, assign20320_e27946_d_n7, assign20320_e27946_d_n10, assign20320_e27946_d_n11, assign20320_e27946_d_n12, assign20320_e27946_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign20320_e27943: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20320_e27944: f64 = (locals.var_vgp__blk612 - assign20320_e27943);
        (assign20320_e27944, (locals.var_vgp__blk612_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20320_e27946;
        locals.var_ps0_inia_dn0 = assign20320_e27946_d_n0;
        locals.var_ps0_inia_dn2 = assign20320_e27946_d_n2;
        locals.var_ps0_inia_dn6 = assign20320_e27946_d_n6;
        locals.var_ps0_inia_dn7 = assign20320_e27946_d_n7;
        locals.var_ps0_inia_dn10 = assign20320_e27946_d_n10;
        locals.var_ps0_inia_dn11 = assign20320_e27946_d_n11;
        locals.var_ps0_inia_dn12 = assign20320_e27946_d_n12;
        locals.var_ps0_inia_dn17 = assign20320_e27946_d_n17;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20330_e27956, assign20330_e27956_d_n0, assign20330_e27956_d_n2, assign20330_e27956_d_n6, assign20330_e27956_d_n7, assign20330_e27956_d_n10, assign20330_e27956_d_n11, assign20330_e27956_d_n12, assign20330_e27956_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20330_e27956;
        locals.var_ps0_ini_dn0 = assign20330_e27956_d_n0;
        locals.var_ps0_ini_dn2 = assign20330_e27956_d_n2;
        locals.var_ps0_ini_dn6 = assign20330_e27956_d_n6;
        locals.var_ps0_ini_dn7 = assign20330_e27956_d_n7;
        locals.var_ps0_ini_dn10 = assign20330_e27956_d_n10;
        locals.var_ps0_ini_dn11 = assign20330_e27956_d_n11;
        locals.var_ps0_ini_dn12 = assign20330_e27956_d_n12;
        locals.var_ps0_ini_dn17 = assign20330_e27956_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20340_e27969, assign20340_e27969_d_n0, assign20340_e27969_d_n2, assign20340_e27969_d_n6, assign20340_e27969_d_n7, assign20340_e27969_d_n10, assign20340_e27969_d_n11, assign20340_e27969_d_n12, assign20340_e27969_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign20340_e27967: f64 = (1.0 / locals.var_c_fox);
        (assign20340_e27967, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20340_e27969;
        locals.var_t0_dn0 = assign20340_e27969_d_n0;
        locals.var_t0_dn2 = assign20340_e27969_d_n2;
        locals.var_t0_dn6 = assign20340_e27969_d_n6;
        locals.var_t0_dn7 = assign20340_e27969_d_n7;
        locals.var_t0_dn10 = assign20340_e27969_d_n10;
        locals.var_t0_dn11 = assign20340_e27969_d_n11;
        locals.var_t0_dn12 = assign20340_e27969_d_n12;
        locals.var_t0_dn17 = assign20340_e27969_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20350_e27982, assign20350_e27982_d_n0, assign20350_e27982_d_n2, assign20350_e27982_d_n6, assign20350_e27982_d_n7, assign20350_e27982_d_n10, assign20350_e27982_d_n11, assign20350_e27982_d_n12, assign20350_e27982_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign20350_e27980: f64 = (locals.var_t_soi__blk611 / 1.034943e-10);
        (assign20350_e27980, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20350_e27982;
        locals.var_t1_dn0 = assign20350_e27982_d_n0;
        locals.var_t1_dn2 = assign20350_e27982_d_n2;
        locals.var_t1_dn6 = assign20350_e27982_d_n6;
        locals.var_t1_dn7 = assign20350_e27982_d_n7;
        locals.var_t1_dn10 = assign20350_e27982_d_n10;
        locals.var_t1_dn11 = assign20350_e27982_d_n11;
        locals.var_t1_dn12 = assign20350_e27982_d_n12;
        locals.var_t1_dn17 = assign20350_e27982_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20360_e27995, assign20360_e27995_d_n0, assign20360_e27995_d_n2, assign20360_e27995_d_n6, assign20360_e27995_d_n7, assign20360_e27995_d_n10, assign20360_e27995_d_n11, assign20360_e27995_d_n12, assign20360_e27995_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign20360_e27993: f64 = (1.0 / locals.var_c_box);
        (assign20360_e27993, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20360_e27995;
        locals.var_t2_dn0 = assign20360_e27995_d_n0;
        locals.var_t2_dn2 = assign20360_e27995_d_n2;
        locals.var_t2_dn6 = assign20360_e27995_d_n6;
        locals.var_t2_dn7 = assign20360_e27995_d_n7;
        locals.var_t2_dn10 = assign20360_e27995_d_n10;
        locals.var_t2_dn11 = assign20360_e27995_d_n11;
        locals.var_t2_dn12 = assign20360_e27995_d_n12;
        locals.var_t2_dn17 = assign20360_e27995_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20370_e28012, assign20370_e28012_d_n0, assign20370_e28012_d_n2, assign20370_e28012_d_n6, assign20370_e28012_d_n7, assign20370_e28012_d_n10, assign20370_e28012_d_n11, assign20370_e28012_d_n12, assign20370_e28012_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign20370_e28007: f64 = (locals.var_t0 + locals.var_t1);
        let assign20370_e28009: f64 = (assign20370_e28007 + locals.var_t2);
        let assign20370_e28010: f64 = (1.0 / assign20370_e28009);
        (assign20370_e28010, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20370_e28009 * assign20370_e28009))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20370_e28009 * assign20370_e28009))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20370_e28009 * assign20370_e28009))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20370_e28009 * assign20370_e28009))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20370_e28009 * assign20370_e28009))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20370_e28009 * assign20370_e28009))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20370_e28009 * assign20370_e28009))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20370_e28009 * assign20370_e28009))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20370_e28012;
        locals.var_t3_dn0 = assign20370_e28012_d_n0;
        locals.var_t3_dn2 = assign20370_e28012_d_n2;
        locals.var_t3_dn6 = assign20370_e28012_d_n6;
        locals.var_t3_dn7 = assign20370_e28012_d_n7;
        locals.var_t3_dn10 = assign20370_e28012_d_n10;
        locals.var_t3_dn11 = assign20370_e28012_d_n11;
        locals.var_t3_dn12 = assign20370_e28012_d_n12;
        locals.var_t3_dn17 = assign20370_e28012_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20380_e28036, assign20380_e28036_d_n0, assign20380_e28036_d_n2, assign20380_e28036_d_n6, assign20380_e28036_d_n7, assign20380_e28036_d_n10, assign20380_e28036_d_n11, assign20380_e28036_d_n12, assign20380_e28036_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign20380_e28024: f64 = (locals.var_vgp__blk612 - locals.var_vbsbiz);
        let assign20380_e28028: f64 = (0.5 * locals.var_t1);
        let assign20380_e28029: f64 = (locals.var_t2 + assign20380_e28028);
        let assign20380_e28031: f64 = (-locals.var_q_s0_dep_ini);
        let assign20380_e28032: f64 = (assign20380_e28029 * assign20380_e28031);
        let assign20380_e28033: f64 = (assign20380_e28024 + assign20380_e28032);
        let assign20380_e28034: f64 = (locals.var_t3 * assign20380_e28033);
        (assign20380_e28034, ((locals.var_t3_dn0 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20380_e28033) + (locals.var_t3 * ((locals.var_vgp__blk612_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20380_e28031) + (assign20380_e28029 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20380_e28036;
        locals.var_t4_dn0 = assign20380_e28036_d_n0;
        locals.var_t4_dn2 = assign20380_e28036_d_n2;
        locals.var_t4_dn6 = assign20380_e28036_d_n6;
        locals.var_t4_dn7 = assign20380_e28036_d_n7;
        locals.var_t4_dn10 = assign20380_e28036_d_n10;
        locals.var_t4_dn11 = assign20380_e28036_d_n11;
        locals.var_t4_dn12 = assign20380_e28036_d_n12;
        locals.var_t4_dn17 = assign20380_e28036_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20390_e28051, assign20390_e28051_d_n0, assign20390_e28051_d_n2, assign20390_e28051_d_n6, assign20390_e28051_d_n7, assign20390_e28051_d_n10, assign20390_e28051_d_n11, assign20390_e28051_d_n12, assign20390_e28051_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign20390_e28048: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20390_e28049: f64 = (locals.var_vgp__blk612 - assign20390_e28048);
        (assign20390_e28049, (locals.var_vgp__blk612_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk612_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20390_e28051;
        locals.var_ps0_inia_dn0 = assign20390_e28051_d_n0;
        locals.var_ps0_inia_dn2 = assign20390_e28051_d_n2;
        locals.var_ps0_inia_dn6 = assign20390_e28051_d_n6;
        locals.var_ps0_inia_dn7 = assign20390_e28051_d_n7;
        locals.var_ps0_inia_dn10 = assign20390_e28051_d_n10;
        locals.var_ps0_inia_dn11 = assign20390_e28051_d_n11;
        locals.var_ps0_inia_dn12 = assign20390_e28051_d_n12;
        locals.var_ps0_inia_dn17 = assign20390_e28051_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20400_e28062, assign20400_e28062_d_n0, assign20400_e28062_d_n2, assign20400_e28062_d_n6, assign20400_e28062_d_n7, assign20400_e28062_d_n10, assign20400_e28062_d_n11, assign20400_e28062_d_n12, assign20400_e28062_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20400_e28062;
        locals.var_ps0_ini_dn0 = assign20400_e28062_d_n0;
        locals.var_ps0_ini_dn2 = assign20400_e28062_d_n2;
        locals.var_ps0_ini_dn6 = assign20400_e28062_d_n6;
        locals.var_ps0_ini_dn7 = assign20400_e28062_d_n7;
        locals.var_ps0_ini_dn10 = assign20400_e28062_d_n10;
        locals.var_ps0_ini_dn11 = assign20400_e28062_d_n11;
        locals.var_ps0_ini_dn12 = assign20400_e28062_d_n12;
        locals.var_ps0_ini_dn17 = assign20400_e28062_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20410_e28065: f64 = (locals.var_vgp__blk612 - locals.var_shift);
        let assign20410_e28067: f64 = if assign20410_e28065 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign20410_e28067;
        locals.var_guard636_rv = 0.0;

        let (assign20420_e28084, assign20420_e28084_d_n0, assign20420_e28084_d_n2, assign20420_e28084_d_n6, assign20420_e28084_d_n7, assign20420_e28084_d_n10, assign20420_e28084_d_n11, assign20420_e28084_d_n12, assign20420_e28084_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign20420_e28080: f64 = (1.0 / locals.var_cnst1soi);
        let assign20420_e28082: f64 = (assign20420_e28080 / locals.var_cnstc_foxi);
        (assign20420_e28082, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20420_e28080 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20420_e28084;
        locals.var_t1_dn0 = assign20420_e28084_d_n0;
        locals.var_t1_dn2 = assign20420_e28084_d_n2;
        locals.var_t1_dn6 = assign20420_e28084_d_n6;
        locals.var_t1_dn7 = assign20420_e28084_d_n7;
        locals.var_t1_dn10 = assign20420_e28084_d_n10;
        locals.var_t1_dn11 = assign20420_e28084_d_n11;
        locals.var_t1_dn12 = assign20420_e28084_d_n12;
        locals.var_t1_dn17 = assign20420_e28084_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20430_e28105, assign20430_e28105_d_n0, assign20430_e28105_d_n2, assign20430_e28105_d_n6, assign20430_e28105_d_n7, assign20430_e28105_d_n10, assign20430_e28105_d_n11, assign20430_e28105_d_n12, assign20430_e28105_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign20430_e28098: f64 = (locals.var_vgp__blk612 - locals.var_shift);
        let assign20430_e28099: f64 = (locals.var_t1 * assign20430_e28098);
        let assign20430_e28102: f64 = (locals.var_vgp__blk612 - locals.var_shift);
        let assign20430_e28103: f64 = (assign20430_e28099 * assign20430_e28102);
        (assign20430_e28103, ((((locals.var_t1_dn0 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn0 - locals.var_shift_dn0))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn2 - locals.var_shift_dn2))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn6 - locals.var_shift_dn6))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn7 - locals.var_shift_dn7))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn10 - locals.var_shift_dn10))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn11 - locals.var_shift_dn11))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn12 - locals.var_shift_dn12))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign20430_e28098) + (locals.var_t1 * (locals.var_vgp__blk612_dn17 - locals.var_shift_dn17))) * assign20430_e28102) + (assign20430_e28099 * (locals.var_vgp__blk612_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20430_e28105;
        locals.var_t2_dn0 = assign20430_e28105_d_n0;
        locals.var_t2_dn2 = assign20430_e28105_d_n2;
        locals.var_t2_dn6 = assign20430_e28105_d_n6;
        locals.var_t2_dn7 = assign20430_e28105_d_n7;
        locals.var_t2_dn10 = assign20430_e28105_d_n10;
        locals.var_t2_dn11 = assign20430_e28105_d_n11;
        locals.var_t2_dn12 = assign20430_e28105_d_n12;
        locals.var_t2_dn17 = assign20430_e28105_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20440_e28124, assign20440_e28124_d_n0, assign20440_e28124_d_n2, assign20440_e28124_d_n6, assign20440_e28124_d_n7, assign20440_e28124_d_n10, assign20440_e28124_d_n11, assign20440_e28124_d_n12, assign20440_e28124_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign20440_e28120: f64 = (locals.var_vgp__blk612 - locals.var_shift);
        let assign20440_e28121: f64 = (2.0 / assign20440_e28120);
        let assign20440_e28122: f64 = (locals.var_beta + assign20440_e28121);
        (assign20440_e28122, (-((2.0 * (locals.var_vgp__blk612_dn0 - locals.var_shift_dn0)) / (assign20440_e28120 * assign20440_e28120))), (-((2.0 * (locals.var_vgp__blk612_dn2 - locals.var_shift_dn2)) / (assign20440_e28120 * assign20440_e28120))), (-((2.0 * (locals.var_vgp__blk612_dn6 - locals.var_shift_dn6)) / (assign20440_e28120 * assign20440_e28120))), (-((2.0 * (locals.var_vgp__blk612_dn7 - locals.var_shift_dn7)) / (assign20440_e28120 * assign20440_e28120))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp__blk612_dn10 - locals.var_shift_dn10)) / (assign20440_e28120 * assign20440_e28120)))), (-((2.0 * (locals.var_vgp__blk612_dn11 - locals.var_shift_dn11)) / (assign20440_e28120 * assign20440_e28120))), (-((2.0 * (locals.var_vgp__blk612_dn12 - locals.var_shift_dn12)) / (assign20440_e28120 * assign20440_e28120))), (-((2.0 * (locals.var_vgp__blk612_dn17 - locals.var_shift_dn17)) / (assign20440_e28120 * assign20440_e28120))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20440_e28124;
        locals.var_t3_dn0 = assign20440_e28124_d_n0;
        locals.var_t3_dn2 = assign20440_e28124_d_n2;
        locals.var_t3_dn6 = assign20440_e28124_d_n6;
        locals.var_t3_dn7 = assign20440_e28124_d_n7;
        locals.var_t3_dn10 = assign20440_e28124_d_n10;
        locals.var_t3_dn11 = assign20440_e28124_d_n11;
        locals.var_t3_dn12 = assign20440_e28124_d_n12;
        locals.var_t3_dn17 = assign20440_e28124_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20450_e28142, assign20450_e28142_d_n0, assign20450_e28142_d_n2, assign20450_e28142_d_n6, assign20450_e28142_d_n7, assign20450_e28142_d_n10, assign20450_e28142_d_n11, assign20450_e28142_d_n12, assign20450_e28142_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign20450_e28136: f64 = (locals.var_t2).ln();
        let assign20450_e28138: f64 = (assign20450_e28136 / locals.var_t3);
        let assign20450_e28140: f64 = (assign20450_e28138 + p.p287);
        (assign20450_e28140, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign20450_e28136 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign20450_e28142;
        locals.var_ps0_inib_dn0 = assign20450_e28142_d_n0;
        locals.var_ps0_inib_dn2 = assign20450_e28142_d_n2;
        locals.var_ps0_inib_dn6 = assign20450_e28142_d_n6;
        locals.var_ps0_inib_dn7 = assign20450_e28142_d_n7;
        locals.var_ps0_inib_dn10 = assign20450_e28142_d_n10;
        locals.var_ps0_inib_dn11 = assign20450_e28142_d_n11;
        locals.var_ps0_inib_dn12 = assign20450_e28142_d_n12;
        locals.var_ps0_inib_dn17 = assign20450_e28142_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let assign20460_e28146: f64 = (locals.var_ps0_inib * 0.98);
        let assign20460_e28148: f64 = (assign20460_e28146 - 0.4);
        let assign20460_e28153: f64 = if ((locals.var_ps0_inia > assign20460_e28148) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard637 = assign20460_e28153;
        locals.var_guard637_rv = 0.0;

        let (assign20470_e28174, assign20470_e28174_d_n0, assign20470_e28174_d_n2, assign20470_e28174_d_n6, assign20470_e28174_d_n7, assign20470_e28174_d_n10, assign20470_e28174_d_n11, assign20470_e28174_d_n12, assign20470_e28174_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20470_e28169: f64 = (locals.var_ps0_inib * 0.98);
        let assign20470_e28170: f64 = (locals.var_ps0_inia - assign20470_e28169);
        let assign20470_e28172: f64 = (assign20470_e28170 + 0.4);
        (assign20470_e28172, (locals.var_ps0_inia_dn0 - (locals.var_ps0_inib_dn0 * 0.98)), (locals.var_ps0_inia_dn2 - (locals.var_ps0_inib_dn2 * 0.98)), (locals.var_ps0_inia_dn6 - (locals.var_ps0_inib_dn6 * 0.98)), (locals.var_ps0_inia_dn7 - (locals.var_ps0_inib_dn7 * 0.98)), (locals.var_ps0_inia_dn10 - (locals.var_ps0_inib_dn10 * 0.98)), (locals.var_ps0_inia_dn11 - (locals.var_ps0_inib_dn11 * 0.98)), (locals.var_ps0_inia_dn12 - (locals.var_ps0_inib_dn12 * 0.98)), (locals.var_ps0_inia_dn17 - (locals.var_ps0_inib_dn17 * 0.98)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20470_e28174;
        locals.var_tmf1_dn0 = assign20470_e28174_d_n0;
        locals.var_tmf1_dn2 = assign20470_e28174_d_n2;
        locals.var_tmf1_dn6 = assign20470_e28174_d_n6;
        locals.var_tmf1_dn7 = assign20470_e28174_d_n7;
        locals.var_tmf1_dn10 = assign20470_e28174_d_n10;
        locals.var_tmf1_dn11 = assign20470_e28174_d_n11;
        locals.var_tmf1_dn12 = assign20470_e28174_d_n12;
        locals.var_tmf1_dn17 = assign20470_e28174_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20480_e28191, assign20480_e28191_d_n0, assign20480_e28191_d_n2, assign20480_e28191_d_n6, assign20480_e28191_d_n7, assign20480_e28191_d_n10, assign20480_e28191_d_n11, assign20480_e28191_d_n12, assign20480_e28191_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20480_e28189: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign20480_e28189, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign20480_e28191;
        locals.var_x2_dn0 = assign20480_e28191_d_n0;
        locals.var_x2_dn2 = assign20480_e28191_d_n2;
        locals.var_x2_dn6 = assign20480_e28191_d_n6;
        locals.var_x2_dn7 = assign20480_e28191_d_n7;
        locals.var_x2_dn10 = assign20480_e28191_d_n10;
        locals.var_x2_dn11 = assign20480_e28191_d_n11;
        locals.var_x2_dn12 = assign20480_e28191_d_n12;
        locals.var_x2_dn17 = assign20480_e28191_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign20490_e28208, assign20490_e28208_d_n0, assign20490_e28208_d_n2, assign20490_e28208_d_n6, assign20490_e28208_d_n7, assign20490_e28208_d_n10, assign20490_e28208_d_n11, assign20490_e28208_d_n12, assign20490_e28208_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20490_e28206: f64 = (0.4 * 0.4);
        (assign20490_e28206, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign20490_e28208;
        locals.var_xmax2_dn0 = assign20490_e28208_d_n0;
        locals.var_xmax2_dn2 = assign20490_e28208_d_n2;
        locals.var_xmax2_dn6 = assign20490_e28208_d_n6;
        locals.var_xmax2_dn7 = assign20490_e28208_d_n7;
        locals.var_xmax2_dn10 = assign20490_e28208_d_n10;
        locals.var_xmax2_dn11 = assign20490_e28208_d_n11;
        locals.var_xmax2_dn12 = assign20490_e28208_d_n12;
        locals.var_xmax2_dn17 = assign20490_e28208_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign20500_e28223, assign20500_e28223_d_n0, assign20500_e28223_d_n2, assign20500_e28223_d_n6, assign20500_e28223_d_n7, assign20500_e28223_d_n10, assign20500_e28223_d_n11, assign20500_e28223_d_n12, assign20500_e28223_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20500_e28223;
        locals.var_xp_dn0 = assign20500_e28223_d_n0;
        locals.var_xp_dn2 = assign20500_e28223_d_n2;
        locals.var_xp_dn6 = assign20500_e28223_d_n6;
        locals.var_xp_dn7 = assign20500_e28223_d_n7;
        locals.var_xp_dn10 = assign20500_e28223_d_n10;
        locals.var_xp_dn11 = assign20500_e28223_d_n11;
        locals.var_xp_dn12 = assign20500_e28223_d_n12;
        locals.var_xp_dn17 = assign20500_e28223_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20510_e28238, assign20510_e28238_d_n0, assign20510_e28238_d_n2, assign20510_e28238_d_n6, assign20510_e28238_d_n7, assign20510_e28238_d_n10, assign20510_e28238_d_n11, assign20510_e28238_d_n12, assign20510_e28238_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20510_e28238;
        locals.var_xmp_dn0 = assign20510_e28238_d_n0;
        locals.var_xmp_dn2 = assign20510_e28238_d_n2;
        locals.var_xmp_dn6 = assign20510_e28238_d_n6;
        locals.var_xmp_dn7 = assign20510_e28238_d_n7;
        locals.var_xmp_dn10 = assign20510_e28238_d_n10;
        locals.var_xmp_dn11 = assign20510_e28238_d_n11;
        locals.var_xmp_dn12 = assign20510_e28238_d_n12;
        locals.var_xmp_dn17 = assign20510_e28238_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20520_e28253,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign20520_e28253;
        locals.var_m0_rv = 0.0;

        let (assign20530_e28268,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20530_e28268;
        locals.var_mm_rv = 0.0;

        let (assign20540_e28283, assign20540_e28283_d_n0, assign20540_e28283_d_n2, assign20540_e28283_d_n6, assign20540_e28283_d_n7, assign20540_e28283_d_n10, assign20540_e28283_d_n11, assign20540_e28283_d_n12, assign20540_e28283_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign20540_e28283;
        locals.var_arg_dn0 = assign20540_e28283_d_n0;
        locals.var_arg_dn2 = assign20540_e28283_d_n2;
        locals.var_arg_dn6 = assign20540_e28283_d_n6;
        locals.var_arg_dn7 = assign20540_e28283_d_n7;
        locals.var_arg_dn10 = assign20540_e28283_d_n10;
        locals.var_arg_dn11 = assign20540_e28283_d_n11;
        locals.var_arg_dn12 = assign20540_e28283_d_n12;
        locals.var_arg_dn17 = assign20540_e28283_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign20550_e28298, assign20550_e28298_d_n0, assign20550_e28298_d_n2, assign20550_e28298_d_n6, assign20550_e28298_d_n7, assign20550_e28298_d_n10, assign20550_e28298_d_n11, assign20550_e28298_d_n12, assign20550_e28298_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20550_e28298;
        locals.var_dnm_dn0 = assign20550_e28298_d_n0;
        locals.var_dnm_dn2 = assign20550_e28298_d_n2;
        locals.var_dnm_dn6 = assign20550_e28298_d_n6;
        locals.var_dnm_dn7 = assign20550_e28298_d_n7;
        locals.var_dnm_dn10 = assign20550_e28298_d_n10;
        locals.var_dnm_dn11 = assign20550_e28298_d_n11;
        locals.var_dnm_dn12 = assign20550_e28298_d_n12;
        locals.var_dnm_dn17 = assign20550_e28298_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20560_e28315, assign20560_e28315_d_n0, assign20560_e28315_d_n2, assign20560_e28315_d_n6, assign20560_e28315_d_n7, assign20560_e28315_d_n10, assign20560_e28315_d_n11, assign20560_e28315_d_n12, assign20560_e28315_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20560_e28313: f64 = (locals.var_xp * locals.var_x2);
        (assign20560_e28313, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20560_e28315;
        locals.var_xp_dn0 = assign20560_e28315_d_n0;
        locals.var_xp_dn2 = assign20560_e28315_d_n2;
        locals.var_xp_dn6 = assign20560_e28315_d_n6;
        locals.var_xp_dn7 = assign20560_e28315_d_n7;
        locals.var_xp_dn10 = assign20560_e28315_d_n10;
        locals.var_xp_dn11 = assign20560_e28315_d_n11;
        locals.var_xp_dn12 = assign20560_e28315_d_n12;
        locals.var_xp_dn17 = assign20560_e28315_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20570_e28332, assign20570_e28332_d_n0, assign20570_e28332_d_n2, assign20570_e28332_d_n6, assign20570_e28332_d_n7, assign20570_e28332_d_n10, assign20570_e28332_d_n11, assign20570_e28332_d_n12, assign20570_e28332_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20570_e28330: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign20570_e28330, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20570_e28332;
        locals.var_xmp_dn0 = assign20570_e28332_d_n0;
        locals.var_xmp_dn2 = assign20570_e28332_d_n2;
        locals.var_xmp_dn6 = assign20570_e28332_d_n6;
        locals.var_xmp_dn7 = assign20570_e28332_d_n7;
        locals.var_xmp_dn10 = assign20570_e28332_d_n10;
        locals.var_xmp_dn11 = assign20570_e28332_d_n11;
        locals.var_xmp_dn12 = assign20570_e28332_d_n12;
        locals.var_xmp_dn17 = assign20570_e28332_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20580_e28349, assign20580_e28349_d_n0, assign20580_e28349_d_n2, assign20580_e28349_d_n6, assign20580_e28349_d_n7, assign20580_e28349_d_n10, assign20580_e28349_d_n11, assign20580_e28349_d_n12, assign20580_e28349_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20580_e28347: f64 = (locals.var_xp * locals.var_x2);
        (assign20580_e28347, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20580_e28349;
        locals.var_xp_dn0 = assign20580_e28349_d_n0;
        locals.var_xp_dn2 = assign20580_e28349_d_n2;
        locals.var_xp_dn6 = assign20580_e28349_d_n6;
        locals.var_xp_dn7 = assign20580_e28349_d_n7;
        locals.var_xp_dn10 = assign20580_e28349_d_n10;
        locals.var_xp_dn11 = assign20580_e28349_d_n11;
        locals.var_xp_dn12 = assign20580_e28349_d_n12;
        locals.var_xp_dn17 = assign20580_e28349_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20590_e28366, assign20590_e28366_d_n0, assign20590_e28366_d_n2, assign20590_e28366_d_n6, assign20590_e28366_d_n7, assign20590_e28366_d_n10, assign20590_e28366_d_n11, assign20590_e28366_d_n12, assign20590_e28366_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20590_e28364: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign20590_e28364, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20590_e28366;
        locals.var_xmp_dn0 = assign20590_e28366_d_n0;
        locals.var_xmp_dn2 = assign20590_e28366_d_n2;
        locals.var_xmp_dn6 = assign20590_e28366_d_n6;
        locals.var_xmp_dn7 = assign20590_e28366_d_n7;
        locals.var_xmp_dn10 = assign20590_e28366_d_n10;
        locals.var_xmp_dn11 = assign20590_e28366_d_n11;
        locals.var_xmp_dn12 = assign20590_e28366_d_n12;
        locals.var_xmp_dn17 = assign20590_e28366_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20600_e28383, assign20600_e28383_d_n0, assign20600_e28383_d_n2, assign20600_e28383_d_n6, assign20600_e28383_d_n7, assign20600_e28383_d_n10, assign20600_e28383_d_n11, assign20600_e28383_d_n12, assign20600_e28383_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20600_e28381: f64 = (locals.var_xp + locals.var_xmp);
        (assign20600_e28381, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign20600_e28383;
        locals.var_arg_dn0 = assign20600_e28383_d_n0;
        locals.var_arg_dn2 = assign20600_e28383_d_n2;
        locals.var_arg_dn6 = assign20600_e28383_d_n6;
        locals.var_arg_dn7 = assign20600_e28383_d_n7;
        locals.var_arg_dn10 = assign20600_e28383_d_n10;
        locals.var_arg_dn11 = assign20600_e28383_d_n11;
        locals.var_arg_dn12 = assign20600_e28383_d_n12;
        locals.var_arg_dn17 = assign20600_e28383_d_n17;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20610_e28398, assign20610_e28398_d_n0, assign20610_e28398_d_n2, assign20610_e28398_d_n6, assign20610_e28398_d_n7, assign20610_e28398_d_n10, assign20610_e28398_d_n11, assign20610_e28398_d_n12, assign20610_e28398_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20610_e28398;
        locals.var_dnm_dn0 = assign20610_e28398_d_n0;
        locals.var_dnm_dn2 = assign20610_e28398_d_n2;
        locals.var_dnm_dn6 = assign20610_e28398_d_n6;
        locals.var_dnm_dn7 = assign20610_e28398_d_n7;
        locals.var_dnm_dn10 = assign20610_e28398_d_n10;
        locals.var_dnm_dn11 = assign20610_e28398_d_n11;
        locals.var_dnm_dn12 = assign20610_e28398_d_n12;
        locals.var_dnm_dn17 = assign20610_e28398_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign20620_e28413: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard638 = assign20620_e28413;
        locals.var_guard638_rv = 0.0;

        let assign20630_e28416: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign20630_e28416;
        locals.var_guard639_rv = 0.0;

        let (assign20640_e28435,) = {
    if ((((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20640_e28435;
        locals.var_mm_rv = 0.0;

        let assign20650_e28438: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign20650_e28438;
        locals.var_guard640_rv = 0.0;

        let (assign20660_e28460,) = {
    if (((((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20660_e28460;
        locals.var_mm_rv = 0.0;

        let assign20670_e28463: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard641 = assign20670_e28463;
        locals.var_guard641_rv = 0.0;

        let (assign20680_e28488,) = {
    if ((((((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20680_e28488;
        locals.var_mm_rv = 0.0;

        let assign20690_e28491: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign20690_e28491;
        locals.var_guard642_rv = 0.0;

        let (assign20700_e28519,) = {
    if (((((((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 == 0.0)) && (locals.var_guard641 == 0.0)) && (locals.var_guard642 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20700_e28519;
        locals.var_mm_rv = 0.0;

        let (assign20710_e28536,) = {
    if (((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign20710_e28536;
        locals.var_m0_rv = 0.0;

        let mut assign20720_loop_guard: usize = 0;
        while {
            let assign20720_cond_e28554: f64 = if ((((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign20720_cond_e28554 != 0.0
        } {
            assign20720_loop_guard += 1;
            assert!(assign20720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign20720_body0_e28572, assign20720_body0_e28572_d_n0, assign20720_body0_e28572_d_n2, assign20720_body0_e28572_d_n6, assign20720_body0_e28572_d_n7, assign20720_body0_e28572_d_n10, assign20720_body0_e28572_d_n11, assign20720_body0_e28572_d_n12, assign20720_body0_e28572_d_n17,) = {
    if (((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) {
        let assign20720_body0_e28570: f64 = (locals.var_dnm).sqrt();
        (assign20720_body0_e28570, (locals.var_dnm_dn0 / (2.0 * assign20720_body0_e28570)), (locals.var_dnm_dn2 / (2.0 * assign20720_body0_e28570)), (locals.var_dnm_dn6 / (2.0 * assign20720_body0_e28570)), (locals.var_dnm_dn7 / (2.0 * assign20720_body0_e28570)), (locals.var_dnm_dn10 / (2.0 * assign20720_body0_e28570)), (locals.var_dnm_dn11 / (2.0 * assign20720_body0_e28570)), (locals.var_dnm_dn12 / (2.0 * assign20720_body0_e28570)), (locals.var_dnm_dn17 / (2.0 * assign20720_body0_e28570)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign20720_body0_e28572;
            locals.var_dnm_dn0 = assign20720_body0_e28572_d_n0;
            locals.var_dnm_dn2 = assign20720_body0_e28572_d_n2;
            locals.var_dnm_dn6 = assign20720_body0_e28572_d_n6;
            locals.var_dnm_dn7 = assign20720_body0_e28572_d_n7;
            locals.var_dnm_dn10 = assign20720_body0_e28572_d_n10;
            locals.var_dnm_dn11 = assign20720_body0_e28572_d_n11;
            locals.var_dnm_dn12 = assign20720_body0_e28572_d_n12;
            locals.var_dnm_dn17 = assign20720_body0_e28572_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign20720_body1_e28591,) = {
    if (((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 != 0.0)) {
        let assign20720_body1_e28589: f64 = (locals.var_m0 + 1.0);
        (assign20720_body1_e28589,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign20720_body1_e28591;
            locals.var_m0_rv = 0.0;
        }

        let (assign20730_e28615, assign20730_e28615_d_n0, assign20730_e28615_d_n2, assign20730_e28615_d_n6, assign20730_e28615_d_n7, assign20730_e28615_d_n10, assign20730_e28615_d_n11, assign20730_e28615_d_n12, assign20730_e28615_d_n17,) = {
    if (((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) && (locals.var_guard638 == 0.0)) {
        let assign20730_e28611: f64 = (2.0 * 2.0);
        let assign20730_e28612: f64 = (1.0 / assign20730_e28611);
        let assign20730_e28613: f64 = (locals.var_dnm).powf(assign20730_e28612);
        (assign20730_e28613, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn0)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn2)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn6)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn7)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn10)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn11)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn12)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20730_e28612) as f64).is_finite() && ((assign20730_e28612) as f64).fract() == 0.0 { if assign20730_e28612 == 0.0 { 0.0 } else { (assign20730_e28612 * ((locals.var_dnm).powf(assign20730_e28612 - 1.0) * locals.var_dnm_dn17)) } } else { (assign20730_e28613 * (assign20730_e28612 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20730_e28615;
        locals.var_dnm_dn0 = assign20730_e28615_d_n0;
        locals.var_dnm_dn2 = assign20730_e28615_d_n2;
        locals.var_dnm_dn6 = assign20730_e28615_d_n6;
        locals.var_dnm_dn7 = assign20730_e28615_d_n7;
        locals.var_dnm_dn10 = assign20730_e28615_d_n10;
        locals.var_dnm_dn11 = assign20730_e28615_d_n11;
        locals.var_dnm_dn12 = assign20730_e28615_d_n12;
        locals.var_dnm_dn17 = assign20730_e28615_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20740_e28632, assign20740_e28632_d_n0, assign20740_e28632_d_n2, assign20740_e28632_d_n6, assign20740_e28632_d_n7, assign20740_e28632_d_n10, assign20740_e28632_d_n11, assign20740_e28632_d_n12, assign20740_e28632_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20740_e28630: f64 = (1.0 / locals.var_dnm);
        (assign20740_e28630, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20740_e28632;
        locals.var_dnm_dn0 = assign20740_e28632_d_n0;
        locals.var_dnm_dn2 = assign20740_e28632_d_n2;
        locals.var_dnm_dn6 = assign20740_e28632_d_n6;
        locals.var_dnm_dn7 = assign20740_e28632_d_n7;
        locals.var_dnm_dn10 = assign20740_e28632_d_n10;
        locals.var_dnm_dn11 = assign20740_e28632_d_n11;
        locals.var_dnm_dn12 = assign20740_e28632_d_n12;
        locals.var_dnm_dn17 = assign20740_e28632_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20750_e28651, assign20750_e28651_d_n0, assign20750_e28651_d_n2, assign20750_e28651_d_n6, assign20750_e28651_d_n7, assign20750_e28651_d_n10, assign20750_e28651_d_n11, assign20750_e28651_d_n12, assign20750_e28651_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20750_e28647: f64 = (locals.var_tmf1 * 0.4);
        let assign20750_e28649: f64 = (assign20750_e28647 * locals.var_dnm);
        (assign20750_e28649, (((locals.var_tmf1_dn0 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 0.4) * locals.var_dnm) + (assign20750_e28647 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign20750_e28651;
        locals.var_tmf0_dn0 = assign20750_e28651_d_n0;
        locals.var_tmf0_dn2 = assign20750_e28651_d_n2;
        locals.var_tmf0_dn6 = assign20750_e28651_d_n6;
        locals.var_tmf0_dn7 = assign20750_e28651_d_n7;
        locals.var_tmf0_dn10 = assign20750_e28651_d_n10;
        locals.var_tmf0_dn11 = assign20750_e28651_d_n11;
        locals.var_tmf0_dn12 = assign20750_e28651_d_n12;
        locals.var_tmf0_dn17 = assign20750_e28651_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign20760_e28672, assign20760_e28672_d_n0, assign20760_e28672_d_n2, assign20760_e28672_d_n6, assign20760_e28672_d_n7, assign20760_e28672_d_n10, assign20760_e28672_d_n11, assign20760_e28672_d_n12, assign20760_e28672_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign20760_e28666: f64 = (locals.var_ps0_inib * 0.98);
        let assign20760_e28668: f64 = (assign20760_e28666 - 0.4);
        let assign20760_e28670: f64 = (assign20760_e28668 + locals.var_tmf0);
        (assign20760_e28670, ((locals.var_ps0_inib_dn0 * 0.98) + locals.var_tmf0_dn0), ((locals.var_ps0_inib_dn2 * 0.98) + locals.var_tmf0_dn2), ((locals.var_ps0_inib_dn6 * 0.98) + locals.var_tmf0_dn6), ((locals.var_ps0_inib_dn7 * 0.98) + locals.var_tmf0_dn7), ((locals.var_ps0_inib_dn10 * 0.98) + locals.var_tmf0_dn10), ((locals.var_ps0_inib_dn11 * 0.98) + locals.var_tmf0_dn11), ((locals.var_ps0_inib_dn12 * 0.98) + locals.var_tmf0_dn12), ((locals.var_ps0_inib_dn17 * 0.98) + locals.var_tmf0_dn17),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20760_e28672;
        locals.var_ps0_ini_dn0 = assign20760_e28672_d_n0;
        locals.var_ps0_ini_dn2 = assign20760_e28672_d_n2;
        locals.var_ps0_ini_dn6 = assign20760_e28672_d_n6;
        locals.var_ps0_ini_dn7 = assign20760_e28672_d_n7;
        locals.var_ps0_ini_dn10 = assign20760_e28672_d_n10;
        locals.var_ps0_ini_dn11 = assign20760_e28672_d_n11;
        locals.var_ps0_ini_dn12 = assign20760_e28672_d_n12;
        locals.var_ps0_ini_dn17 = assign20760_e28672_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20770_e28688, assign20770_e28688_d_n0, assign20770_e28688_d_n2, assign20770_e28688_d_n6, assign20770_e28688_d_n7, assign20770_e28688_d_n10, assign20770_e28688_d_n11, assign20770_e28688_d_n12, assign20770_e28688_d_n17,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard630 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) && (locals.var_guard637 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20770_e28688;
        locals.var_ps0_ini_dn0 = assign20770_e28688_d_n0;
        locals.var_ps0_ini_dn2 = assign20770_e28688_d_n2;
        locals.var_ps0_ini_dn6 = assign20770_e28688_d_n6;
        locals.var_ps0_ini_dn7 = assign20770_e28688_d_n7;
        locals.var_ps0_ini_dn10 = assign20770_e28688_d_n10;
        locals.var_ps0_ini_dn11 = assign20770_e28688_d_n11;
        locals.var_ps0_ini_dn12 = assign20770_e28688_d_n12;
        locals.var_ps0_ini_dn17 = assign20770_e28688_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20780_e28696, assign20780_e28696_d_n0, assign20780_e28696_d_n2, assign20780_e28696_d_n6, assign20780_e28696_d_n7, assign20780_e28696_d_n10, assign20780_e28696_d_n11, assign20780_e28696_d_n12, assign20780_e28696_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign20780_e28693: f64 = (5e-12 / 2.0);
        let assign20780_e28694: f64 = (locals.var_vbs__blk627 + assign20780_e28693);
        (assign20780_e28694, locals.var_vbs__blk627_dn0, locals.var_vbs__blk627_dn2, locals.var_vbs__blk627_dn6, locals.var_vbs__blk627_dn7, locals.var_vbs__blk627_dn10, locals.var_vbs__blk627_dn11, locals.var_vbs__blk627_dn12, locals.var_vbs__blk627_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign20780_e28696;
        locals.var_tx_dn0 = assign20780_e28696_d_n0;
        locals.var_tx_dn2 = assign20780_e28696_d_n2;
        locals.var_tx_dn6 = assign20780_e28696_d_n6;
        locals.var_tx_dn7 = assign20780_e28696_d_n7;
        locals.var_tx_dn10 = assign20780_e28696_d_n10;
        locals.var_tx_dn11 = assign20780_e28696_d_n11;
        locals.var_tx_dn12 = assign20780_e28696_d_n12;
        locals.var_tx_dn17 = assign20780_e28696_d_n17;
        locals.var_tx_rv = 0.0;

        let assign20790_e28699: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard643 = assign20790_e28699;
        locals.var_guard643_rv = 0.0;

        let (assign20800_e28705, assign20800_e28705_d_n0, assign20800_e28705_d_n2, assign20800_e28705_d_n6, assign20800_e28705_d_n7, assign20800_e28705_d_n10, assign20800_e28705_d_n11, assign20800_e28705_d_n12, assign20800_e28705_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard643 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20800_e28705;
        locals.var_ps0_ini_dn0 = assign20800_e28705_d_n0;
        locals.var_ps0_ini_dn2 = assign20800_e28705_d_n2;
        locals.var_ps0_ini_dn6 = assign20800_e28705_d_n6;
        locals.var_ps0_ini_dn7 = assign20800_e28705_d_n7;
        locals.var_ps0_ini_dn10 = assign20800_e28705_d_n10;
        locals.var_ps0_ini_dn11 = assign20800_e28705_d_n11;
        locals.var_ps0_ini_dn12 = assign20800_e28705_d_n12;
        locals.var_ps0_ini_dn17 = assign20800_e28705_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20810_e28709, assign20810_e28709_d_n0, assign20810_e28709_d_n2, assign20810_e28709_d_n6, assign20810_e28709_d_n7, assign20810_e28709_d_n10, assign20810_e28709_d_n11, assign20810_e28709_d_n12, assign20810_e28709_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0__blk610, locals.var_ps0__blk610_dn0, locals.var_ps0__blk610_dn2, locals.var_ps0__blk610_dn6, locals.var_ps0__blk610_dn7, locals.var_ps0__blk610_dn10, locals.var_ps0__blk610_dn11, locals.var_ps0__blk610_dn12, locals.var_ps0__blk610_dn17,)
    }
};
        locals.var_ps0__blk610 = assign20810_e28709;
        locals.var_ps0__blk610_dn0 = assign20810_e28709_d_n0;
        locals.var_ps0__blk610_dn2 = assign20810_e28709_d_n2;
        locals.var_ps0__blk610_dn6 = assign20810_e28709_d_n6;
        locals.var_ps0__blk610_dn7 = assign20810_e28709_d_n7;
        locals.var_ps0__blk610_dn10 = assign20810_e28709_d_n10;
        locals.var_ps0__blk610_dn11 = assign20810_e28709_d_n11;
        locals.var_ps0__blk610_dn12 = assign20810_e28709_d_n12;
        locals.var_ps0__blk610_dn17 = assign20810_e28709_d_n17;
        locals.var_ps0__blk610_rv = 0.0;

        let (assign20820_e28713, assign20820_e28713_d_n0, assign20820_e28713_d_n2, assign20820_e28713_d_n6, assign20820_e28713_d_n7, assign20820_e28713_d_n10, assign20820_e28713_d_n11, assign20820_e28713_d_n12, assign20820_e28713_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign20820_e28713;
        locals.var_psl_lim_dn0 = assign20820_e28713_d_n0;
        locals.var_psl_lim_dn2 = assign20820_e28713_d_n2;
        locals.var_psl_lim_dn6 = assign20820_e28713_d_n6;
        locals.var_psl_lim_dn7 = assign20820_e28713_d_n7;
        locals.var_psl_lim_dn10 = assign20820_e28713_d_n10;
        locals.var_psl_lim_dn11 = assign20820_e28713_d_n11;
        locals.var_psl_lim_dn12 = assign20820_e28713_d_n12;
        locals.var_psl_lim_dn17 = assign20820_e28713_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let (assign20830_e28728, assign20830_e28728_d_n0, assign20830_e28728_d_n2, assign20830_e28728_d_n6, assign20830_e28728_d_n7, assign20830_e28728_d_n10, assign20830_e28728_d_n11, assign20830_e28728_d_n12, assign20830_e28728_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let assign20830_e28719: f64 = (locals.var_ps0_inia - locals.var_ps0__blk610);
        let (assign20830_e28726, assign20830_e28726_d_n0, assign20830_e28726_d_n2, assign20830_e28726_d_n6, assign20830_e28726_d_n7, assign20830_e28726_d_n10, assign20830_e28726_d_n11, assign20830_e28726_d_n12, assign20830_e28726_d_n17,) = {
            if (assign20830_e28719 >= 0.0) {
                let assign20830_e28724: f64 = (locals.var_ps0_inia - locals.var_ps0__blk610);
                (assign20830_e28724, (locals.var_ps0_inia_dn0 - locals.var_ps0__blk610_dn0), (locals.var_ps0_inia_dn2 - locals.var_ps0__blk610_dn2), (locals.var_ps0_inia_dn6 - locals.var_ps0__blk610_dn6), (locals.var_ps0_inia_dn7 - locals.var_ps0__blk610_dn7), (locals.var_ps0_inia_dn10 - locals.var_ps0__blk610_dn10), (locals.var_ps0_inia_dn11 - locals.var_ps0__blk610_dn11), (locals.var_ps0_inia_dn12 - locals.var_ps0__blk610_dn12), (locals.var_ps0_inia_dn17 - locals.var_ps0__blk610_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign20830_e28726, assign20830_e28726_d_n0, assign20830_e28726_d_n2, assign20830_e28726_d_n6, assign20830_e28726_d_n7, assign20830_e28726_d_n10, assign20830_e28726_d_n11, assign20830_e28726_d_n12, assign20830_e28726_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign20830_e28728;
        locals.var_pds_max_dn0 = assign20830_e28728_d_n0;
        locals.var_pds_max_dn2 = assign20830_e28728_d_n2;
        locals.var_pds_max_dn6 = assign20830_e28728_d_n6;
        locals.var_pds_max_dn7 = assign20830_e28728_d_n7;
        locals.var_pds_max_dn10 = assign20830_e28728_d_n10;
        locals.var_pds_max_dn11 = assign20830_e28728_d_n11;
        locals.var_pds_max_dn12 = assign20830_e28728_d_n12;
        locals.var_pds_max_dn17 = assign20830_e28728_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign20840_e28742, assign20840_e28742_d_n0, assign20840_e28742_d_n2, assign20840_e28742_d_n6, assign20840_e28742_d_n7, assign20840_e28742_d_n10, assign20840_e28742_d_n11, assign20840_e28742_d_n12, assign20840_e28742_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let assign20840_e28734: f64 = (1.0 + 0.3);
        let assign20840_e28736: f64 = (assign20840_e28734 * locals.var_pds_max);
        let assign20840_e28738: f64 = (assign20840_e28736 - p.p287);
        let assign20840_e28740: f64 = (assign20840_e28738 - 0.03);
        (assign20840_e28740, (assign20840_e28734 * locals.var_pds_max_dn0), (assign20840_e28734 * locals.var_pds_max_dn2), (assign20840_e28734 * locals.var_pds_max_dn6), (assign20840_e28734 * locals.var_pds_max_dn7), (assign20840_e28734 * locals.var_pds_max_dn10), (assign20840_e28734 * locals.var_pds_max_dn11), (assign20840_e28734 * locals.var_pds_max_dn12), (assign20840_e28734 * locals.var_pds_max_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20840_e28742;
        locals.var_tmf1_dn0 = assign20840_e28742_d_n0;
        locals.var_tmf1_dn2 = assign20840_e28742_d_n2;
        locals.var_tmf1_dn6 = assign20840_e28742_d_n6;
        locals.var_tmf1_dn7 = assign20840_e28742_d_n7;
        locals.var_tmf1_dn10 = assign20840_e28742_d_n10;
        locals.var_tmf1_dn11 = assign20840_e28742_d_n11;
        locals.var_tmf1_dn12 = assign20840_e28742_d_n12;
        locals.var_tmf1_dn17 = assign20840_e28742_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20850_e28756, assign20850_e28756_d_n0, assign20850_e28756_d_n2, assign20850_e28756_d_n6, assign20850_e28756_d_n7, assign20850_e28756_d_n10, assign20850_e28756_d_n11, assign20850_e28756_d_n12, assign20850_e28756_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let assign20850_e28749: f64 = (1.0 + 0.3);
        let assign20850_e28751: f64 = (assign20850_e28749 * locals.var_pds_max);
        let assign20850_e28752: f64 = (4.0 * assign20850_e28751);
        let assign20850_e28754: f64 = (assign20850_e28752 * 0.03);
        (assign20850_e28754, ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign20850_e28749 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20850_e28756;
        locals.var_tmf2_dn0 = assign20850_e28756_d_n0;
        locals.var_tmf2_dn2 = assign20850_e28756_d_n2;
        locals.var_tmf2_dn6 = assign20850_e28756_d_n6;
        locals.var_tmf2_dn7 = assign20850_e28756_d_n7;
        locals.var_tmf2_dn10 = assign20850_e28756_d_n10;
        locals.var_tmf2_dn11 = assign20850_e28756_d_n11;
        locals.var_tmf2_dn12 = assign20850_e28756_d_n12;
        locals.var_tmf2_dn17 = assign20850_e28756_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20860_e28768, assign20860_e28768_d_n0, assign20860_e28768_d_n2, assign20860_e28768_d_n6, assign20860_e28768_d_n7, assign20860_e28768_d_n10, assign20860_e28768_d_n11, assign20860_e28768_d_n12, assign20860_e28768_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let (assign20860_e28766, assign20860_e28766_d_n0, assign20860_e28766_d_n2, assign20860_e28766_d_n6, assign20860_e28766_d_n7, assign20860_e28766_d_n10, assign20860_e28766_d_n11, assign20860_e28766_d_n12, assign20860_e28766_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign20860_e28765: f64 = (-locals.var_tmf2);
                (assign20860_e28765, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign20860_e28766, assign20860_e28766_d_n0, assign20860_e28766_d_n2, assign20860_e28766_d_n6, assign20860_e28766_d_n7, assign20860_e28766_d_n10, assign20860_e28766_d_n11, assign20860_e28766_d_n12, assign20860_e28766_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20860_e28768;
        locals.var_tmf2_dn0 = assign20860_e28768_d_n0;
        locals.var_tmf2_dn2 = assign20860_e28768_d_n2;
        locals.var_tmf2_dn6 = assign20860_e28768_d_n6;
        locals.var_tmf2_dn7 = assign20860_e28768_d_n7;
        locals.var_tmf2_dn10 = assign20860_e28768_d_n10;
        locals.var_tmf2_dn11 = assign20860_e28768_d_n11;
        locals.var_tmf2_dn12 = assign20860_e28768_d_n12;
        locals.var_tmf2_dn17 = assign20860_e28768_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20870_e28779, assign20870_e28779_d_n0, assign20870_e28779_d_n2, assign20870_e28779_d_n6, assign20870_e28779_d_n7, assign20870_e28779_d_n10, assign20870_e28779_d_n11, assign20870_e28779_d_n12, assign20870_e28779_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let assign20870_e28774: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20870_e28776: f64 = (assign20870_e28774 + locals.var_tmf2);
        let assign20870_e28777: f64 = (assign20870_e28776).sqrt();
        (assign20870_e28777, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20870_e28777)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20870_e28777)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20870_e28777)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20870_e28777)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20870_e28777)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20870_e28777)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20870_e28777)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign20870_e28777)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20870_e28779;
        locals.var_tmf2_dn0 = assign20870_e28779_d_n0;
        locals.var_tmf2_dn2 = assign20870_e28779_d_n2;
        locals.var_tmf2_dn6 = assign20870_e28779_d_n6;
        locals.var_tmf2_dn7 = assign20870_e28779_d_n7;
        locals.var_tmf2_dn10 = assign20870_e28779_d_n10;
        locals.var_tmf2_dn11 = assign20870_e28779_d_n11;
        locals.var_tmf2_dn12 = assign20870_e28779_d_n12;
        locals.var_tmf2_dn17 = assign20870_e28779_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20880_e28795, assign20880_e28795_d_n0, assign20880_e28795_d_n2, assign20880_e28795_d_n6, assign20880_e28795_d_n7, assign20880_e28795_d_n10, assign20880_e28795_d_n11, assign20880_e28795_d_n12, assign20880_e28795_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let assign20880_e28785: f64 = (1.0 + 0.3);
        let assign20880_e28787: f64 = (assign20880_e28785 * locals.var_pds_max);
        let assign20880_e28791: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20880_e28792: f64 = (0.5 * assign20880_e28791);
        let assign20880_e28793: f64 = (assign20880_e28787 - assign20880_e28792);
        (assign20880_e28793, ((assign20880_e28785 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign20880_e28785 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign20880_e28785 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign20880_e28785 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign20880_e28785 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign20880_e28785 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign20880_e28785 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign20880_e28785 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20880_e28795;
        locals.var_pds_ini_dn0 = assign20880_e28795_d_n0;
        locals.var_pds_ini_dn2 = assign20880_e28795_d_n2;
        locals.var_pds_ini_dn6 = assign20880_e28795_d_n6;
        locals.var_pds_ini_dn7 = assign20880_e28795_d_n7;
        locals.var_pds_ini_dn10 = assign20880_e28795_d_n10;
        locals.var_pds_ini_dn11 = assign20880_e28795_d_n11;
        locals.var_pds_ini_dn12 = assign20880_e28795_d_n12;
        locals.var_pds_ini_dn17 = assign20880_e28795_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign20890_e28806, assign20890_e28806_d_n0, assign20890_e28806_d_n2, assign20890_e28806_d_n6, assign20890_e28806_d_n7, assign20890_e28806_d_n10, assign20890_e28806_d_n11, assign20890_e28806_d_n12, assign20890_e28806_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let (assign20890_e28804, assign20890_e28804_d_n0, assign20890_e28804_d_n2, assign20890_e28804_d_n6, assign20890_e28804_d_n7, assign20890_e28804_d_n10, assign20890_e28804_d_n11, assign20890_e28804_d_n12, assign20890_e28804_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign20890_e28804, assign20890_e28804_d_n0, assign20890_e28804_d_n2, assign20890_e28804_d_n6, assign20890_e28804_d_n7, assign20890_e28804_d_n10, assign20890_e28804_d_n11, assign20890_e28804_d_n12, assign20890_e28804_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20890_e28806;
        locals.var_pds_ini_dn0 = assign20890_e28806_d_n0;
        locals.var_pds_ini_dn2 = assign20890_e28806_d_n2;
        locals.var_pds_ini_dn6 = assign20890_e28806_d_n6;
        locals.var_pds_ini_dn7 = assign20890_e28806_d_n7;
        locals.var_pds_ini_dn10 = assign20890_e28806_d_n10;
        locals.var_pds_ini_dn11 = assign20890_e28806_d_n11;
        locals.var_pds_ini_dn12 = assign20890_e28806_d_n12;
        locals.var_pds_ini_dn17 = assign20890_e28806_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign20900_e28809: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign20900_e28809;
        locals.var_guard644_rv = 0.0;

        let (assign20910_e28817, assign20910_e28817_d_n0, assign20910_e28817_d_n2, assign20910_e28817_d_n6, assign20910_e28817_d_n7, assign20910_e28817_d_n10, assign20910_e28817_d_n11, assign20910_e28817_d_n12, assign20910_e28817_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (0.0 != 0.0)) && (locals.var_guard644 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20910_e28817;
        locals.var_pds_ini_dn0 = assign20910_e28817_d_n0;
        locals.var_pds_ini_dn2 = assign20910_e28817_d_n2;
        locals.var_pds_ini_dn6 = assign20910_e28817_d_n6;
        locals.var_pds_ini_dn7 = assign20910_e28817_d_n7;
        locals.var_pds_ini_dn10 = assign20910_e28817_d_n10;
        locals.var_pds_ini_dn11 = assign20910_e28817_d_n11;
        locals.var_pds_ini_dn12 = assign20910_e28817_d_n12;
        locals.var_pds_ini_dn17 = assign20910_e28817_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign20920_e28820: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard645 = assign20920_e28820;
        locals.var_guard645_rv = 0.0;

        let (assign20930_e28831, assign20930_e28831_d_n0, assign20930_e28831_d_n2, assign20930_e28831_d_n6, assign20930_e28831_d_n7, assign20930_e28831_d_n10, assign20930_e28831_d_n11, assign20930_e28831_d_n12, assign20930_e28831_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (0.0 != 0.0)) && (locals.var_guard644 == 0.0)) && (locals.var_guard645 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20930_e28831;
        locals.var_pds_ini_dn0 = assign20930_e28831_d_n0;
        locals.var_pds_ini_dn2 = assign20930_e28831_d_n2;
        locals.var_pds_ini_dn6 = assign20930_e28831_d_n6;
        locals.var_pds_ini_dn7 = assign20930_e28831_d_n7;
        locals.var_pds_ini_dn10 = assign20930_e28831_d_n10;
        locals.var_pds_ini_dn11 = assign20930_e28831_d_n11;
        locals.var_pds_ini_dn12 = assign20930_e28831_d_n12;
        locals.var_pds_ini_dn17 = assign20930_e28831_d_n17;
        locals.var_pds_ini_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20940_e28839, assign20940_e28839_d_n0, assign20940_e28839_d_n2, assign20940_e28839_d_n6, assign20940_e28839_d_n7, assign20940_e28839_d_n10, assign20940_e28839_d_n11, assign20940_e28839_d_n12, assign20940_e28839_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (0.0 != 0.0)) {
        let assign20940_e28837: f64 = (locals.var_ps0__blk610 + locals.var_pds_ini);
        (assign20940_e28837, (locals.var_ps0__blk610_dn0 + locals.var_pds_ini_dn0), (locals.var_ps0__blk610_dn2 + locals.var_pds_ini_dn2), (locals.var_ps0__blk610_dn6 + locals.var_pds_ini_dn6), (locals.var_ps0__blk610_dn7 + locals.var_pds_ini_dn7), (locals.var_ps0__blk610_dn10 + locals.var_pds_ini_dn10), (locals.var_ps0__blk610_dn11 + locals.var_pds_ini_dn11), (locals.var_ps0__blk610_dn12 + locals.var_pds_ini_dn12), (locals.var_ps0__blk610_dn17 + locals.var_pds_ini_dn17),)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign20940_e28839;
        locals.var_psl_lim_dn0 = assign20940_e28839_d_n0;
        locals.var_psl_lim_dn2 = assign20940_e28839_d_n2;
        locals.var_psl_lim_dn6 = assign20940_e28839_d_n6;
        locals.var_psl_lim_dn7 = assign20940_e28839_d_n7;
        locals.var_psl_lim_dn10 = assign20940_e28839_d_n10;
        locals.var_psl_lim_dn11 = assign20940_e28839_d_n11;
        locals.var_psl_lim_dn12 = assign20940_e28839_d_n12;
        locals.var_psl_lim_dn17 = assign20940_e28839_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let assign20950_e28842: f64 = if p.p282 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard646 = assign20950_e28842;
        locals.var_guard646_rv = 0.0;

        let (assign20960_e28848, assign20960_e28848_d_n0, assign20960_e28848_d_n2, assign20960_e28848_d_n6, assign20960_e28848_d_n7, assign20960_e28848_d_n10, assign20960_e28848_d_n11, assign20960_e28848_d_n12, assign20960_e28848_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) {
        (locals.var_ps0__blk610, locals.var_ps0__blk610_dn0, locals.var_ps0__blk610_dn2, locals.var_ps0__blk610_dn6, locals.var_ps0__blk610_dn7, locals.var_ps0__blk610_dn10, locals.var_ps0__blk610_dn11, locals.var_ps0__blk610_dn12, locals.var_ps0__blk610_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20960_e28848;
        locals.var_ps0_ini_dn0 = assign20960_e28848_d_n0;
        locals.var_ps0_ini_dn2 = assign20960_e28848_d_n2;
        locals.var_ps0_ini_dn6 = assign20960_e28848_d_n6;
        locals.var_ps0_ini_dn7 = assign20960_e28848_d_n7;
        locals.var_ps0_ini_dn10 = assign20960_e28848_d_n10;
        locals.var_ps0_ini_dn11 = assign20960_e28848_d_n11;
        locals.var_ps0_ini_dn12 = assign20960_e28848_d_n12;
        locals.var_ps0_ini_dn17 = assign20960_e28848_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20970_e28854, assign20970_e28854_d_n0, assign20970_e28854_d_n2, assign20970_e28854_d_n6, assign20970_e28854_d_n7, assign20970_e28854_d_n10, assign20970_e28854_d_n11, assign20970_e28854_d_n12, assign20970_e28854_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    } else {
        (locals.var_vbcs_cl__blk647, locals.var_vbcs_cl__blk647_dn0, locals.var_vbcs_cl__blk647_dn2, locals.var_vbcs_cl__blk647_dn6, locals.var_vbcs_cl__blk647_dn7, locals.var_vbcs_cl__blk647_dn10, locals.var_vbcs_cl__blk647_dn11, locals.var_vbcs_cl__blk647_dn12, locals.var_vbcs_cl__blk647_dn17,)
    }
};
        locals.var_vbcs_cl__blk647 = assign20970_e28854;
        locals.var_vbcs_cl__blk647_dn0 = assign20970_e28854_d_n0;
        locals.var_vbcs_cl__blk647_dn2 = assign20970_e28854_d_n2;
        locals.var_vbcs_cl__blk647_dn6 = assign20970_e28854_d_n6;
        locals.var_vbcs_cl__blk647_dn7 = assign20970_e28854_d_n7;
        locals.var_vbcs_cl__blk647_dn10 = assign20970_e28854_d_n10;
        locals.var_vbcs_cl__blk647_dn11 = assign20970_e28854_d_n11;
        locals.var_vbcs_cl__blk647_dn12 = assign20970_e28854_d_n12;
        locals.var_vbcs_cl__blk647_dn17 = assign20970_e28854_d_n17;
        locals.var_vbcs_cl__blk647_rv = 0.0;

        let (assign20980_e28868, assign20980_e28868_d_n0, assign20980_e28868_d_n2, assign20980_e28868_d_n6, assign20980_e28868_d_n7, assign20980_e28868_d_n10, assign20980_e28868_d_n11, assign20980_e28868_d_n12, assign20980_e28868_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) {
        let assign20980_e28860: f64 = (locals.var_vfb - locals.var_dvth);
        let assign20980_e28862: f64 = (assign20980_e28860 + locals.var_dppg);
        let assign20980_e28864: f64 = (assign20980_e28862 + locals.var_vbcs_cl__blk647);
        let assign20980_e28866: f64 = (assign20980_e28864 + p.p286);
        (assign20980_e28866, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbcs_cl__blk647_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbcs_cl__blk647_dn2), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbcs_cl__blk647_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbcs_cl__blk647_dn7), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbcs_cl__blk647_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbcs_cl__blk647_dn11), (((-locals.var_dvth_dn12) + locals.var_dppg_dn12) + locals.var_vbcs_cl__blk647_dn12), (((-locals.var_dvth_dn17) + locals.var_dppg_dn17) + locals.var_vbcs_cl__blk647_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign20980_e28868;
        locals.var_vgs_fb_dn0 = assign20980_e28868_d_n0;
        locals.var_vgs_fb_dn2 = assign20980_e28868_d_n2;
        locals.var_vgs_fb_dn6 = assign20980_e28868_d_n6;
        locals.var_vgs_fb_dn7 = assign20980_e28868_d_n7;
        locals.var_vgs_fb_dn10 = assign20980_e28868_d_n10;
        locals.var_vgs_fb_dn11 = assign20980_e28868_d_n11;
        locals.var_vgs_fb_dn12 = assign20980_e28868_d_n12;
        locals.var_vgs_fb_dn17 = assign20980_e28868_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let assign20990_e28871: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard649 = assign20990_e28871;
        locals.var_guard649_rv = 0.0;

        let (assign21000_e28880,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21000_e28878: f64 = (-1.0);
        (assign21000_e28878,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign21000_e28880;
        locals.var_flg_zone_rv = 0.0;

        let (assign21010_e28896, assign21010_e28896_d_n0, assign21010_e28896_d_n2, assign21010_e28896_d_n6, assign21010_e28896_d_n7, assign21010_e28896_d_n10, assign21010_e28896_d_n11, assign21010_e28896_d_n12, assign21010_e28896_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21010_e28888: f64 = (2.0 * locals.var_beta_inv);
        let assign21010_e28890: f64 = (-locals.var_vgs_min);
        let assign21010_e28892: f64 = (assign21010_e28890 / locals.var_fac1);
        let assign21010_e28893: f64 = (assign21010_e28892).ln();
        let assign21010_e28894: f64 = (assign21010_e28888 * assign21010_e28893);
        (assign21010_e28894, (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892)), (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892)), (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892)), (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892)), (((2.0 * locals.var_beta_inv_dn10) * assign21010_e28893) + (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892))), (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892)), (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892)), (assign21010_e28888 * ((-((assign21010_e28890 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign21010_e28892)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign21010_e28896;
        locals.var_ps0_min_dn0 = assign21010_e28896_d_n0;
        locals.var_ps0_min_dn2 = assign21010_e28896_d_n2;
        locals.var_ps0_min_dn6 = assign21010_e28896_d_n6;
        locals.var_ps0_min_dn7 = assign21010_e28896_d_n7;
        locals.var_ps0_min_dn10 = assign21010_e28896_d_n10;
        locals.var_ps0_min_dn11 = assign21010_e28896_d_n11;
        locals.var_ps0_min_dn12 = assign21010_e28896_d_n12;
        locals.var_ps0_min_dn17 = assign21010_e28896_d_n17;
        locals.var_ps0_min_rv = 0.0;

        let (assign21020_e28908, assign21020_e28908_d_n0, assign21020_e28908_d_n2, assign21020_e28908_d_n6, assign21020_e28908_d_n7, assign21020_e28908_d_n10, assign21020_e28908_d_n11, assign21020_e28908_d_n12, assign21020_e28908_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21020_e28905: f64 = (locals.var_vgp__blk612 - locals.var_vbcs_cl__blk647);
        let assign21020_e28906: f64 = (locals.var_beta * assign21020_e28905);
        (assign21020_e28906, (locals.var_beta * (locals.var_vgp__blk612_dn0 - locals.var_vbcs_cl__blk647_dn0)), (locals.var_beta * (locals.var_vgp__blk612_dn2 - locals.var_vbcs_cl__blk647_dn2)), (locals.var_beta * (locals.var_vgp__blk612_dn6 - locals.var_vbcs_cl__blk647_dn6)), (locals.var_beta * (locals.var_vgp__blk612_dn7 - locals.var_vbcs_cl__blk647_dn7)), ((locals.var_beta_dn10 * assign21020_e28905) + (locals.var_beta * (locals.var_vgp__blk612_dn10 - locals.var_vbcs_cl__blk647_dn10))), (locals.var_beta * (locals.var_vgp__blk612_dn11 - locals.var_vbcs_cl__blk647_dn11)), (locals.var_beta * (locals.var_vgp__blk612_dn12 - locals.var_vbcs_cl__blk647_dn12)), (locals.var_beta * (locals.var_vgp__blk612_dn17 - locals.var_vbcs_cl__blk647_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign21020_e28908;
        locals.var_tx_dn0 = assign21020_e28908_d_n0;
        locals.var_tx_dn2 = assign21020_e28908_d_n2;
        locals.var_tx_dn6 = assign21020_e28908_d_n6;
        locals.var_tx_dn7 = assign21020_e28908_d_n7;
        locals.var_tx_dn10 = assign21020_e28908_d_n10;
        locals.var_tx_dn11 = assign21020_e28908_d_n11;
        locals.var_tx_dn12 = assign21020_e28908_d_n12;
        locals.var_tx_dn17 = assign21020_e28908_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign21030_e28920, assign21030_e28920_d_n0, assign21030_e28920_d_n2, assign21030_e28920_d_n6, assign21030_e28920_d_n7, assign21030_e28920_d_n10, assign21030_e28920_d_n11, assign21030_e28920_d_n12, assign21030_e28920_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21030_e28917: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign21030_e28918: f64 = (1.0 / assign21030_e28917);
        (assign21030_e28918, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign21030_e28917 * assign21030_e28917))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign21030_e28917 * assign21030_e28917))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign21030_e28917 * assign21030_e28917))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign21030_e28917 * assign21030_e28917))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign21030_e28917 * assign21030_e28917))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign21030_e28917 * assign21030_e28917))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign21030_e28917 * assign21030_e28917))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign21030_e28917 * assign21030_e28917))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21030_e28920;
        locals.var_t1_dn0 = assign21030_e28920_d_n0;
        locals.var_t1_dn2 = assign21030_e28920_d_n2;
        locals.var_t1_dn6 = assign21030_e28920_d_n6;
        locals.var_t1_dn7 = assign21030_e28920_d_n7;
        locals.var_t1_dn10 = assign21030_e28920_d_n10;
        locals.var_t1_dn11 = assign21030_e28920_d_n11;
        locals.var_t1_dn12 = assign21030_e28920_d_n12;
        locals.var_t1_dn17 = assign21030_e28920_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21040_e28930, assign21040_e28930_d_n0, assign21040_e28930_d_n2, assign21040_e28930_d_n6, assign21040_e28930_d_n7, assign21040_e28930_d_n10, assign21040_e28930_d_n11, assign21040_e28930_d_n12, assign21040_e28930_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21040_e28928: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign21040_e28928, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign21040_e28930;
        locals.var_ty_dn0 = assign21040_e28930_d_n0;
        locals.var_ty_dn2 = assign21040_e28930_d_n2;
        locals.var_ty_dn6 = assign21040_e28930_d_n6;
        locals.var_ty_dn7 = assign21040_e28930_d_n7;
        locals.var_ty_dn10 = assign21040_e28930_d_n10;
        locals.var_ty_dn11 = assign21040_e28930_d_n11;
        locals.var_ty_dn12 = assign21040_e28930_d_n12;
        locals.var_ty_dn17 = assign21040_e28930_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign21050_e28944, assign21050_e28944_d_n0, assign21050_e28944_d_n2, assign21050_e28944_d_n6, assign21050_e28944_d_n7, assign21050_e28944_d_n10, assign21050_e28944_d_n11, assign21050_e28944_d_n12, assign21050_e28944_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21050_e28939: f64 = (3.0 * 1.414213562373095);
        let assign21050_e28941: f64 = (assign21050_e28939 * locals.var_ty);
        let assign21050_e28942: f64 = (2.0 + assign21050_e28941);
        (assign21050_e28942, (assign21050_e28939 * locals.var_ty_dn0), (assign21050_e28939 * locals.var_ty_dn2), (assign21050_e28939 * locals.var_ty_dn6), (assign21050_e28939 * locals.var_ty_dn7), (assign21050_e28939 * locals.var_ty_dn10), (assign21050_e28939 * locals.var_ty_dn11), (assign21050_e28939 * locals.var_ty_dn12), (assign21050_e28939 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign21050_e28944;
        locals.var_ac41_dn0 = assign21050_e28944_d_n0;
        locals.var_ac41_dn2 = assign21050_e28944_d_n2;
        locals.var_ac41_dn6 = assign21050_e28944_d_n6;
        locals.var_ac41_dn7 = assign21050_e28944_d_n7;
        locals.var_ac41_dn10 = assign21050_e28944_d_n10;
        locals.var_ac41_dn11 = assign21050_e28944_d_n11;
        locals.var_ac41_dn12 = assign21050_e28944_d_n12;
        locals.var_ac41_dn17 = assign21050_e28944_d_n17;
        locals.var_ac41_rv = 0.0;

        let (assign21060_e28958, assign21060_e28958_d_n0, assign21060_e28958_d_n2, assign21060_e28958_d_n6, assign21060_e28958_d_n7, assign21060_e28958_d_n10, assign21060_e28958_d_n11, assign21060_e28958_d_n12, assign21060_e28958_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21060_e28952: f64 = (8.0 * locals.var_ac41);
        let assign21060_e28954: f64 = (assign21060_e28952 * locals.var_ac41);
        let assign21060_e28956: f64 = (assign21060_e28954 * locals.var_ac41);
        (assign21060_e28956, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign21060_e28952 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign21060_e28954 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign21060_e28958;
        locals.var_ac4_dn0 = assign21060_e28958_d_n0;
        locals.var_ac4_dn2 = assign21060_e28958_d_n2;
        locals.var_ac4_dn6 = assign21060_e28958_d_n6;
        locals.var_ac4_dn7 = assign21060_e28958_d_n7;
        locals.var_ac4_dn10 = assign21060_e28958_d_n10;
        locals.var_ac4_dn11 = assign21060_e28958_d_n11;
        locals.var_ac4_dn12 = assign21060_e28958_d_n12;
        locals.var_ac4_dn17 = assign21060_e28958_d_n17;
        locals.var_ac4_rv = 0.0;

        let (assign21070_e28968, assign21070_e28968_d_n0, assign21070_e28968_d_n2, assign21070_e28968_d_n6, assign21070_e28968_d_n7, assign21070_e28968_d_n10, assign21070_e28968_d_n11, assign21070_e28968_d_n12, assign21070_e28968_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21070_e28966: f64 = (locals.var_tx - 2.0);
        (assign21070_e28966, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign21070_e28968;
        locals.var_t4_dn0 = assign21070_e28968_d_n0;
        locals.var_t4_dn2 = assign21070_e28968_d_n2;
        locals.var_t4_dn6 = assign21070_e28968_d_n6;
        locals.var_t4_dn7 = assign21070_e28968_d_n7;
        locals.var_t4_dn10 = assign21070_e28968_d_n10;
        locals.var_t4_dn11 = assign21070_e28968_d_n11;
        locals.var_t4_dn12 = assign21070_e28968_d_n12;
        locals.var_t4_dn17 = assign21070_e28968_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign21080_e28980, assign21080_e28980_d_n0, assign21080_e28980_d_n2, assign21080_e28980_d_n6, assign21080_e28980_d_n7, assign21080_e28980_d_n10, assign21080_e28980_d_n11, assign21080_e28980_d_n12, assign21080_e28980_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21080_e28976: f64 = (9.0 * locals.var_ty);
        let assign21080_e28978: f64 = (assign21080_e28976 * locals.var_t4);
        (assign21080_e28978, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign21080_e28976 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign21080_e28980;
        locals.var_t5_dn0 = assign21080_e28980_d_n0;
        locals.var_t5_dn2 = assign21080_e28980_d_n2;
        locals.var_t5_dn6 = assign21080_e28980_d_n6;
        locals.var_t5_dn7 = assign21080_e28980_d_n7;
        locals.var_t5_dn10 = assign21080_e28980_d_n10;
        locals.var_t5_dn11 = assign21080_e28980_d_n11;
        locals.var_t5_dn12 = assign21080_e28980_d_n12;
        locals.var_t5_dn17 = assign21080_e28980_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign21090_e28992, assign21090_e28992_d_n0, assign21090_e28992_d_n2, assign21090_e28992_d_n6, assign21090_e28992_d_n7, assign21090_e28992_d_n10, assign21090_e28992_d_n11, assign21090_e28992_d_n12, assign21090_e28992_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21090_e28988: f64 = (7.0 * 1.414213562373095);
        let assign21090_e28990: f64 = (assign21090_e28988 - locals.var_t5);
        (assign21090_e28990, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign21090_e28992;
        locals.var_ac31_dn0 = assign21090_e28992_d_n0;
        locals.var_ac31_dn2 = assign21090_e28992_d_n2;
        locals.var_ac31_dn6 = assign21090_e28992_d_n6;
        locals.var_ac31_dn7 = assign21090_e28992_d_n7;
        locals.var_ac31_dn10 = assign21090_e28992_d_n10;
        locals.var_ac31_dn11 = assign21090_e28992_d_n11;
        locals.var_ac31_dn12 = assign21090_e28992_d_n12;
        locals.var_ac31_dn17 = assign21090_e28992_d_n17;
        locals.var_ac31_rv = 0.0;

        let (assign21100_e29002, assign21100_e29002_d_n0, assign21100_e29002_d_n2, assign21100_e29002_d_n6, assign21100_e29002_d_n7, assign21100_e29002_d_n10, assign21100_e29002_d_n11, assign21100_e29002_d_n12, assign21100_e29002_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21100_e29000: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign21100_e29000, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign21100_e29002;
        locals.var_ac3_dn0 = assign21100_e29002_d_n0;
        locals.var_ac3_dn2 = assign21100_e29002_d_n2;
        locals.var_ac3_dn6 = assign21100_e29002_d_n6;
        locals.var_ac3_dn7 = assign21100_e29002_d_n7;
        locals.var_ac3_dn10 = assign21100_e29002_d_n10;
        locals.var_ac3_dn11 = assign21100_e29002_d_n11;
        locals.var_ac3_dn12 = assign21100_e29002_d_n12;
        locals.var_ac3_dn17 = assign21100_e29002_d_n17;
        locals.var_ac3_rv = 0.0;

        let assign21110_e29006: f64 = (locals.var_ac3 * 1e-8);
        let assign21110_e29007: f64 = if locals.var_ac4 < assign21110_e29006 { 1.0 } else { 0.0 };
        locals.var_guard650 = assign21110_e29007;
        locals.var_guard650_rv = 0.0;

        let (assign21120_e29030, assign21120_e29030_d_n0, assign21120_e29030_d_n2, assign21120_e29030_d_n6, assign21120_e29030_d_n7, assign21120_e29030_d_n10, assign21120_e29030_d_n11, assign21120_e29030_d_n12, assign21120_e29030_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21120_e29016: f64 = (-7.0);
        let assign21120_e29018: f64 = (assign21120_e29016 * 1.414213562373095);
        let assign21120_e29020: f64 = (assign21120_e29018 + locals.var_ac31);
        let assign21120_e29023: f64 = (0.5 * locals.var_ac4);
        let assign21120_e29025: f64 = (assign21120_e29023 / locals.var_ac31);
        let assign21120_e29026: f64 = (assign21120_e29020 + assign21120_e29025);
        let assign21120_e29028: f64 = (assign21120_e29026 + locals.var_t5);
        (assign21120_e29028, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign21120_e29023 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign21120_e29030;
        locals.var_ac1_dn0 = assign21120_e29030_d_n0;
        locals.var_ac1_dn2 = assign21120_e29030_d_n2;
        locals.var_ac1_dn6 = assign21120_e29030_d_n6;
        locals.var_ac1_dn7 = assign21120_e29030_d_n7;
        locals.var_ac1_dn10 = assign21120_e29030_d_n10;
        locals.var_ac1_dn11 = assign21120_e29030_d_n11;
        locals.var_ac1_dn12 = assign21120_e29030_d_n12;
        locals.var_ac1_dn17 = assign21120_e29030_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign21130_e29044, assign21130_e29044_d_n0, assign21130_e29044_d_n2, assign21130_e29044_d_n6, assign21130_e29044_d_n7, assign21130_e29044_d_n10, assign21130_e29044_d_n11, assign21130_e29044_d_n12, assign21130_e29044_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign21130_e29041: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign21130_e29042: f64 = (assign21130_e29041).sqrt();
        (assign21130_e29042, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign21130_e29042)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign21130_e29042)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign21130_e29042)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign21130_e29042)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign21130_e29042)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign21130_e29042)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign21130_e29042)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign21130_e29042)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign21130_e29044;
        locals.var_ac2_dn0 = assign21130_e29044_d_n0;
        locals.var_ac2_dn2 = assign21130_e29044_d_n2;
        locals.var_ac2_dn6 = assign21130_e29044_d_n6;
        locals.var_ac2_dn7 = assign21130_e29044_d_n7;
        locals.var_ac2_dn10 = assign21130_e29044_d_n10;
        locals.var_ac2_dn11 = assign21130_e29044_d_n11;
        locals.var_ac2_dn12 = assign21130_e29044_d_n12;
        locals.var_ac2_dn17 = assign21130_e29044_d_n17;
        locals.var_ac2_rv = 0.0;

        let (assign21140_e29062, assign21140_e29062_d_n0, assign21140_e29062_d_n2, assign21140_e29062_d_n6, assign21140_e29062_d_n7, assign21140_e29062_d_n10, assign21140_e29062_d_n11, assign21140_e29062_d_n12, assign21140_e29062_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign21140_e29054: f64 = (-7.0);
        let assign21140_e29056: f64 = (assign21140_e29054 * 1.414213562373095);
        let assign21140_e29058: f64 = (assign21140_e29056 + locals.var_ac2);
        let assign21140_e29060: f64 = (assign21140_e29058 + locals.var_t5);
        (assign21140_e29060, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign21140_e29062;
        locals.var_ac1_dn0 = assign21140_e29062_d_n0;
        locals.var_ac1_dn2 = assign21140_e29062_d_n2;
        locals.var_ac1_dn6 = assign21140_e29062_d_n6;
        locals.var_ac1_dn7 = assign21140_e29062_d_n7;
        locals.var_ac1_dn10 = assign21140_e29062_d_n10;
        locals.var_ac1_dn11 = assign21140_e29062_d_n11;
        locals.var_ac1_dn12 = assign21140_e29062_d_n12;
        locals.var_ac1_dn17 = assign21140_e29062_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign21150_e29072, assign21150_e29072_d_n0, assign21150_e29072_d_n2, assign21150_e29072_d_n6, assign21150_e29072_d_n7, assign21150_e29072_d_n10, assign21150_e29072_d_n11, assign21150_e29072_d_n12, assign21150_e29072_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21150_e29070: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign21150_e29070, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign21150_e29070 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign21150_e29072;
        locals.var_acd_dn0 = assign21150_e29072_d_n0;
        locals.var_acd_dn2 = assign21150_e29072_d_n2;
        locals.var_acd_dn6 = assign21150_e29072_d_n6;
        locals.var_acd_dn7 = assign21150_e29072_d_n7;
        locals.var_acd_dn10 = assign21150_e29072_d_n10;
        locals.var_acd_dn11 = assign21150_e29072_d_n11;
        locals.var_acd_dn12 = assign21150_e29072_d_n12;
        locals.var_acd_dn17 = assign21150_e29072_d_n17;
        locals.var_acd_rv = 0.0;

        let (assign21160_e29097, assign21160_e29097_d_n0, assign21160_e29097_d_n2, assign21160_e29097_d_n6, assign21160_e29097_d_n7, assign21160_e29097_d_n10, assign21160_e29097_d_n11, assign21160_e29097_d_n12, assign21160_e29097_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21160_e29079: f64 = (-4.0);
        let assign21160_e29081: f64 = (assign21160_e29079 * 1.414213562373095);
        let assign21160_e29084: f64 = (12.0 * locals.var_ty);
        let assign21160_e29085: f64 = (assign21160_e29081 - assign21160_e29084);
        let assign21160_e29088: f64 = (2.0 * locals.var_acd);
        let assign21160_e29089: f64 = (assign21160_e29085 + assign21160_e29088);
        let assign21160_e29092: f64 = (1.414213562373095 * locals.var_acd);
        let assign21160_e29094: f64 = (assign21160_e29092 * locals.var_acd);
        let assign21160_e29095: f64 = (assign21160_e29089 + assign21160_e29094);
        (assign21160_e29095, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign21160_e29092 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign21160_e29097;
        locals.var_acn_dn0 = assign21160_e29097_d_n0;
        locals.var_acn_dn2 = assign21160_e29097_d_n2;
        locals.var_acn_dn6 = assign21160_e29097_d_n6;
        locals.var_acn_dn7 = assign21160_e29097_d_n7;
        locals.var_acn_dn10 = assign21160_e29097_d_n10;
        locals.var_acn_dn11 = assign21160_e29097_d_n11;
        locals.var_acn_dn12 = assign21160_e29097_d_n12;
        locals.var_acn_dn17 = assign21160_e29097_d_n17;
        locals.var_acn_rv = 0.0;

        let (assign21170_e29107, assign21170_e29107_d_n0, assign21170_e29107_d_n2, assign21170_e29107_d_n6, assign21170_e29107_d_n7, assign21170_e29107_d_n10, assign21170_e29107_d_n11, assign21170_e29107_d_n12, assign21170_e29107_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21170_e29105: f64 = (1.0 / locals.var_acd);
        (assign21170_e29105, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21170_e29107;
        locals.var_t1_dn0 = assign21170_e29107_d_n0;
        locals.var_t1_dn2 = assign21170_e29107_d_n2;
        locals.var_t1_dn6 = assign21170_e29107_d_n6;
        locals.var_t1_dn7 = assign21170_e29107_d_n7;
        locals.var_t1_dn10 = assign21170_e29107_d_n10;
        locals.var_t1_dn11 = assign21170_e29107_d_n11;
        locals.var_t1_dn12 = assign21170_e29107_d_n12;
        locals.var_t1_dn17 = assign21170_e29107_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21180_e29117, assign21180_e29117_d_n0, assign21180_e29117_d_n2, assign21180_e29117_d_n6, assign21180_e29117_d_n7, assign21180_e29117_d_n10, assign21180_e29117_d_n11, assign21180_e29117_d_n12, assign21180_e29117_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21180_e29115: f64 = (locals.var_acn * locals.var_t1);
        (assign21180_e29115, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign21180_e29117;
        locals.var_chi_dn0 = assign21180_e29117_d_n0;
        locals.var_chi_dn2 = assign21180_e29117_d_n2;
        locals.var_chi_dn6 = assign21180_e29117_d_n6;
        locals.var_chi_dn7 = assign21180_e29117_d_n7;
        locals.var_chi_dn10 = assign21180_e29117_d_n10;
        locals.var_chi_dn11 = assign21180_e29117_d_n11;
        locals.var_chi_dn12 = assign21180_e29117_d_n12;
        locals.var_chi_dn17 = assign21180_e29117_d_n17;
        locals.var_chi_rv = 0.0;

        let (assign21190_e29129, assign21190_e29129_d_n0, assign21190_e29129_d_n2, assign21190_e29129_d_n6, assign21190_e29129_d_n7, assign21190_e29129_d_n10, assign21190_e29129_d_n11, assign21190_e29129_d_n12, assign21190_e29129_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21190_e29125: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign21190_e29127: f64 = (assign21190_e29125 + locals.var_vbcs_cl__blk647);
        (assign21190_e29127, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl__blk647_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl__blk647_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl__blk647_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl__blk647_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl__blk647_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl__blk647_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl__blk647_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl__blk647_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign21190_e29129;
        locals.var_psa_dn0 = assign21190_e29129_d_n0;
        locals.var_psa_dn2 = assign21190_e29129_d_n2;
        locals.var_psa_dn6 = assign21190_e29129_d_n6;
        locals.var_psa_dn7 = assign21190_e29129_d_n7;
        locals.var_psa_dn10 = assign21190_e29129_d_n10;
        locals.var_psa_dn11 = assign21190_e29129_d_n11;
        locals.var_psa_dn12 = assign21190_e29129_d_n12;
        locals.var_psa_dn17 = assign21190_e29129_d_n17;
        locals.var_psa_rv = 0.0;

        let (assign21200_e29139, assign21200_e29139_d_n0, assign21200_e29139_d_n2, assign21200_e29139_d_n6, assign21200_e29139_d_n7, assign21200_e29139_d_n10, assign21200_e29139_d_n11, assign21200_e29139_d_n12, assign21200_e29139_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21200_e29137: f64 = (locals.var_psa - locals.var_vbcs_cl__blk647);
        (assign21200_e29137, (locals.var_psa_dn0 - locals.var_vbcs_cl__blk647_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl__blk647_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl__blk647_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl__blk647_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl__blk647_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl__blk647_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl__blk647_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl__blk647_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21200_e29139;
        locals.var_t1_dn0 = assign21200_e29139_d_n0;
        locals.var_t1_dn2 = assign21200_e29139_d_n2;
        locals.var_t1_dn6 = assign21200_e29139_d_n6;
        locals.var_t1_dn7 = assign21200_e29139_d_n7;
        locals.var_t1_dn10 = assign21200_e29139_d_n10;
        locals.var_t1_dn11 = assign21200_e29139_d_n11;
        locals.var_t1_dn12 = assign21200_e29139_d_n12;
        locals.var_t1_dn17 = assign21200_e29139_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21210_e29149, assign21210_e29149_d_n0, assign21210_e29149_d_n2, assign21210_e29149_d_n6, assign21210_e29149_d_n7, assign21210_e29149_d_n10, assign21210_e29149_d_n11, assign21210_e29149_d_n12, assign21210_e29149_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21210_e29147: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign21210_e29147, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21210_e29149;
        locals.var_t2_dn0 = assign21210_e29149_d_n0;
        locals.var_t2_dn2 = assign21210_e29149_d_n2;
        locals.var_t2_dn6 = assign21210_e29149_d_n6;
        locals.var_t2_dn7 = assign21210_e29149_d_n7;
        locals.var_t2_dn10 = assign21210_e29149_d_n10;
        locals.var_t2_dn11 = assign21210_e29149_d_n11;
        locals.var_t2_dn12 = assign21210_e29149_d_n12;
        locals.var_t2_dn17 = assign21210_e29149_d_n17;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21220_e29162, assign21220_e29162_d_n0, assign21220_e29162_d_n2, assign21220_e29162_d_n6, assign21220_e29162_d_n7, assign21220_e29162_d_n10, assign21220_e29162_d_n11, assign21220_e29162_d_n12, assign21220_e29162_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21220_e29158: f64 = (locals.var_t2 * locals.var_t2);
        let assign21220_e29159: f64 = (1.0 + assign21220_e29158);
        let assign21220_e29160: f64 = (assign21220_e29159).sqrt();
        (assign21220_e29160, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign21220_e29160)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign21220_e29160)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign21220_e29160)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign21220_e29160)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign21220_e29160)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign21220_e29160)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign21220_e29160)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign21220_e29160)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign21220_e29162;
        locals.var_t3_dn0 = assign21220_e29162_d_n0;
        locals.var_t3_dn2 = assign21220_e29162_d_n2;
        locals.var_t3_dn6 = assign21220_e29162_d_n6;
        locals.var_t3_dn7 = assign21220_e29162_d_n7;
        locals.var_t3_dn10 = assign21220_e29162_d_n10;
        locals.var_t3_dn11 = assign21220_e29162_d_n11;
        locals.var_t3_dn12 = assign21220_e29162_d_n12;
        locals.var_t3_dn17 = assign21220_e29162_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign21230_e29174, assign21230_e29174_d_n0, assign21230_e29174_d_n2, assign21230_e29174_d_n6, assign21230_e29174_d_n7, assign21230_e29174_d_n10, assign21230_e29174_d_n11, assign21230_e29174_d_n12, assign21230_e29174_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21230_e29170: f64 = (locals.var_t1 / locals.var_t3);
        let assign21230_e29172: f64 = (assign21230_e29170 + locals.var_vbcs_cl__blk647);
        (assign21230_e29172, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk647_dn17),)
    } else {
        (locals.var_ps0__blk610, locals.var_ps0__blk610_dn0, locals.var_ps0__blk610_dn2, locals.var_ps0__blk610_dn6, locals.var_ps0__blk610_dn7, locals.var_ps0__blk610_dn10, locals.var_ps0__blk610_dn11, locals.var_ps0__blk610_dn12, locals.var_ps0__blk610_dn17,)
    }
};
        locals.var_ps0__blk610 = assign21230_e29174;
        locals.var_ps0__blk610_dn0 = assign21230_e29174_d_n0;
        locals.var_ps0__blk610_dn2 = assign21230_e29174_d_n2;
        locals.var_ps0__blk610_dn6 = assign21230_e29174_d_n6;
        locals.var_ps0__blk610_dn7 = assign21230_e29174_d_n7;
        locals.var_ps0__blk610_dn10 = assign21230_e29174_d_n10;
        locals.var_ps0__blk610_dn11 = assign21230_e29174_d_n11;
        locals.var_ps0__blk610_dn12 = assign21230_e29174_d_n12;
        locals.var_ps0__blk610_dn17 = assign21230_e29174_d_n17;
        locals.var_ps0__blk610_rv = 0.0;

        let (assign21240_e29188, assign21240_e29188_d_n0, assign21240_e29188_d_n2, assign21240_e29188_d_n6, assign21240_e29188_d_n7, assign21240_e29188_d_n10, assign21240_e29188_d_n11, assign21240_e29188_d_n12, assign21240_e29188_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21240_e29184: f64 = (locals.var_vbcs_cl__blk647 - p.p287);
        let assign21240_e29185: f64 = (locals.var_beta * assign21240_e29184);
        let assign21240_e29186: f64 = (assign21240_e29185).exp();
        (assign21240_e29186, (assign21240_e29186 * (locals.var_beta * locals.var_vbcs_cl__blk647_dn0)), (assign21240_e29186 * (locals.var_beta * locals.var_vbcs_cl__blk647_dn2)), (assign21240_e29186 * (locals.var_beta * locals.var_vbcs_cl__blk647_dn6)), (assign21240_e29186 * (locals.var_beta * locals.var_vbcs_cl__blk647_dn7)), (assign21240_e29186 * ((locals.var_beta_dn10 * assign21240_e29184) + (locals.var_beta * locals.var_vbcs_cl__blk647_dn10))), (assign21240_e29186 * (locals.var_beta * locals.var_vbcs_cl__blk647_dn11)), (assign21240_e29186 * (locals.var_beta * locals.var_vbcs_cl__blk647_dn12)), (assign21240_e29186 * (locals.var_beta * locals.var_vbcs_cl__blk647_dn17)),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn12, locals.var_exp_bvbsvds_dn17,)
    }
};
        locals.var_exp_bvbsvds = assign21240_e29188;
        locals.var_exp_bvbsvds_dn0 = assign21240_e29188_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign21240_e29188_d_n2;
        locals.var_exp_bvbsvds_dn6 = assign21240_e29188_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign21240_e29188_d_n7;
        locals.var_exp_bvbsvds_dn10 = assign21240_e29188_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign21240_e29188_d_n11;
        locals.var_exp_bvbsvds_dn12 = assign21240_e29188_d_n12;
        locals.var_exp_bvbsvds_dn17 = assign21240_e29188_d_n17;
        locals.var_exp_bvbsvds_rv = 0.0;

        let (assign21250_e29197,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign21250_e29197;
        locals.var_flg_conv_rv = 0.0;

        let (assign21260_e29206, assign21260_e29206_d_n0, assign21260_e29206_d_n2, assign21260_e29206_d_n6, assign21260_e29206_d_n7, assign21260_e29206_d_n10, assign21260_e29206_d_n11, assign21260_e29206_d_n12, assign21260_e29206_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi__blk648, locals.var_phi_s0_soi__blk648_dn0, locals.var_phi_s0_soi__blk648_dn2, locals.var_phi_s0_soi__blk648_dn6, locals.var_phi_s0_soi__blk648_dn7, locals.var_phi_s0_soi__blk648_dn10, locals.var_phi_s0_soi__blk648_dn11, locals.var_phi_s0_soi__blk648_dn12, locals.var_phi_s0_soi__blk648_dn17,)
    }
};
        locals.var_phi_s0_soi__blk648 = assign21260_e29206;
        locals.var_phi_s0_soi__blk648_dn0 = assign21260_e29206_d_n0;
        locals.var_phi_s0_soi__blk648_dn2 = assign21260_e29206_d_n2;
        locals.var_phi_s0_soi__blk648_dn6 = assign21260_e29206_d_n6;
        locals.var_phi_s0_soi__blk648_dn7 = assign21260_e29206_d_n7;
        locals.var_phi_s0_soi__blk648_dn10 = assign21260_e29206_d_n10;
        locals.var_phi_s0_soi__blk648_dn11 = assign21260_e29206_d_n11;
        locals.var_phi_s0_soi__blk648_dn12 = assign21260_e29206_d_n12;
        locals.var_phi_s0_soi__blk648_dn17 = assign21260_e29206_d_n17;
        locals.var_phi_s0_soi__blk648_rv = 0.0;

        let (assign21270_e29223, assign21270_e29223_d_n0, assign21270_e29223_d_n2, assign21270_e29223_d_n6, assign21270_e29223_d_n7, assign21270_e29223_d_n10, assign21270_e29223_d_n11, assign21270_e29223_d_n12, assign21270_e29223_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21270_e29215: f64 = (locals.var_q_nsub * p.p237);
        let assign21270_e29217: f64 = (assign21270_e29215 * p.p237);
        let assign21270_e29219: f64 = (assign21270_e29217 / 2.0);
        let assign21270_e29221: f64 = (assign21270_e29219 / 1.034943e-10);
        (assign21270_e29221, ((((locals.var_q_nsub_dn0 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn12 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn17 * p.p237) * p.p237) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn12, locals.var_dphi_sb_dn17,)
    }
};
        locals.var_dphi_sb = assign21270_e29223;
        locals.var_dphi_sb_dn0 = assign21270_e29223_d_n0;
        locals.var_dphi_sb_dn2 = assign21270_e29223_d_n2;
        locals.var_dphi_sb_dn6 = assign21270_e29223_d_n6;
        locals.var_dphi_sb_dn7 = assign21270_e29223_d_n7;
        locals.var_dphi_sb_dn10 = assign21270_e29223_d_n10;
        locals.var_dphi_sb_dn11 = assign21270_e29223_d_n11;
        locals.var_dphi_sb_dn12 = assign21270_e29223_d_n12;
        locals.var_dphi_sb_dn17 = assign21270_e29223_d_n17;
        locals.var_dphi_sb_rv = 0.0;

        let (assign21280_e29237, assign21280_e29237_d_n0, assign21280_e29237_d_n2, assign21280_e29237_d_n6, assign21280_e29237_d_n7, assign21280_e29237_d_n10, assign21280_e29237_d_n11, assign21280_e29237_d_n12, assign21280_e29237_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21280_e29232: f64 = (2.0 * locals.var_beta);
        let assign21280_e29234: f64 = (assign21280_e29232 * locals.var_dphi_sb);
        let assign21280_e29235: f64 = (assign21280_e29234).sqrt();
        (assign21280_e29235, ((assign21280_e29232 * locals.var_dphi_sb_dn0) / (2.0 * assign21280_e29235)), ((assign21280_e29232 * locals.var_dphi_sb_dn2) / (2.0 * assign21280_e29235)), ((assign21280_e29232 * locals.var_dphi_sb_dn6) / (2.0 * assign21280_e29235)), ((assign21280_e29232 * locals.var_dphi_sb_dn7) / (2.0 * assign21280_e29235)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign21280_e29232 * locals.var_dphi_sb_dn10)) / (2.0 * assign21280_e29235)), ((assign21280_e29232 * locals.var_dphi_sb_dn11) / (2.0 * assign21280_e29235)), ((assign21280_e29232 * locals.var_dphi_sb_dn12) / (2.0 * assign21280_e29235)), ((assign21280_e29232 * locals.var_dphi_sb_dn17) / (2.0 * assign21280_e29235)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign21280_e29237;
        locals.var_t0_dn0 = assign21280_e29237_d_n0;
        locals.var_t0_dn2 = assign21280_e29237_d_n2;
        locals.var_t0_dn6 = assign21280_e29237_d_n6;
        locals.var_t0_dn7 = assign21280_e29237_d_n7;
        locals.var_t0_dn10 = assign21280_e29237_d_n10;
        locals.var_t0_dn11 = assign21280_e29237_d_n11;
        locals.var_t0_dn12 = assign21280_e29237_d_n12;
        locals.var_t0_dn17 = assign21280_e29237_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign21290_e29253, assign21290_e29253_d_n0, assign21290_e29253_d_n2, assign21290_e29253_d_n6, assign21290_e29253_d_n7, assign21290_e29253_d_n10, assign21290_e29253_d_n11, assign21290_e29253_d_n12, assign21290_e29253_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21290_e29245: f64 = (locals.var_t0).exp();
        let assign21290_e29247: f64 = (-locals.var_t0);
        let assign21290_e29248: f64 = (assign21290_e29247).exp();
        let assign21290_e29249: f64 = (assign21290_e29245 + assign21290_e29248);
        let assign21290_e29251: f64 = (assign21290_e29249 / 2.0);
        (assign21290_e29251, (((assign21290_e29245 * locals.var_t0_dn0) + (assign21290_e29248 * (-locals.var_t0_dn0))) / 2.0), (((assign21290_e29245 * locals.var_t0_dn2) + (assign21290_e29248 * (-locals.var_t0_dn2))) / 2.0), (((assign21290_e29245 * locals.var_t0_dn6) + (assign21290_e29248 * (-locals.var_t0_dn6))) / 2.0), (((assign21290_e29245 * locals.var_t0_dn7) + (assign21290_e29248 * (-locals.var_t0_dn7))) / 2.0), (((assign21290_e29245 * locals.var_t0_dn10) + (assign21290_e29248 * (-locals.var_t0_dn10))) / 2.0), (((assign21290_e29245 * locals.var_t0_dn11) + (assign21290_e29248 * (-locals.var_t0_dn11))) / 2.0), (((assign21290_e29245 * locals.var_t0_dn12) + (assign21290_e29248 * (-locals.var_t0_dn12))) / 2.0), (((assign21290_e29245 * locals.var_t0_dn17) + (assign21290_e29248 * (-locals.var_t0_dn17))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21290_e29253;
        locals.var_t1_dn0 = assign21290_e29253_d_n0;
        locals.var_t1_dn2 = assign21290_e29253_d_n2;
        locals.var_t1_dn6 = assign21290_e29253_d_n6;
        locals.var_t1_dn7 = assign21290_e29253_d_n7;
        locals.var_t1_dn10 = assign21290_e29253_d_n10;
        locals.var_t1_dn11 = assign21290_e29253_d_n11;
        locals.var_t1_dn12 = assign21290_e29253_d_n12;
        locals.var_t1_dn17 = assign21290_e29253_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21300_e29265, assign21300_e29265_d_n0, assign21300_e29265_d_n2, assign21300_e29265_d_n6, assign21300_e29265_d_n7, assign21300_e29265_d_n10, assign21300_e29265_d_n11, assign21300_e29265_d_n12, assign21300_e29265_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21300_e29261: f64 = (locals.var_t1).ln();
        let assign21300_e29263: f64 = (assign21300_e29261 / locals.var_dphi_sb);
        (assign21300_e29263, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn12 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn12)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn17 / locals.var_t1) * locals.var_dphi_sb) - (assign21300_e29261 * locals.var_dphi_sb_dn17)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn12, locals.var_c_sb_dn17,)
    }
};
        locals.var_c_sb = assign21300_e29265;
        locals.var_c_sb_dn0 = assign21300_e29265_d_n0;
        locals.var_c_sb_dn2 = assign21300_e29265_d_n2;
        locals.var_c_sb_dn6 = assign21300_e29265_d_n6;
        locals.var_c_sb_dn7 = assign21300_e29265_d_n7;
        locals.var_c_sb_dn10 = assign21300_e29265_d_n10;
        locals.var_c_sb_dn11 = assign21300_e29265_d_n11;
        locals.var_c_sb_dn12 = assign21300_e29265_d_n12;
        locals.var_c_sb_dn17 = assign21300_e29265_d_n17;
        locals.var_c_sb_rv = 0.0;

        let (assign21310_e29274,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign21310_e29274;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign21320_loop_guard: usize = 0;
        while {
            let assign21320_cond_e29284: f64 = (locals.var_lp_s0_max + 1.0);
            let assign21320_cond_e29286: f64 = if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_lp_s0 <= assign21320_cond_e29284)) { 1.0 } else { 0.0 };
            assign21320_cond_e29286 != 0.0
        } {
            assign21320_loop_guard += 1;
            assert!(assign21320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign21320_body0_e29297, assign21320_body0_e29297_d_n0, assign21320_body0_e29297_d_n2, assign21320_body0_e29297_d_n6, assign21320_body0_e29297_d_n7, assign21320_body0_e29297_d_n10, assign21320_body0_e29297_d_n11, assign21320_body0_e29297_d_n12, assign21320_body0_e29297_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21320_body0_e29295: f64 = (locals.var_phi_s0_soi__blk648 - locals.var_vbcs_cl__blk647);
        (assign21320_body0_e29295, (locals.var_phi_s0_soi__blk648_dn0 - locals.var_vbcs_cl__blk647_dn0), (locals.var_phi_s0_soi__blk648_dn2 - locals.var_vbcs_cl__blk647_dn2), (locals.var_phi_s0_soi__blk648_dn6 - locals.var_vbcs_cl__blk647_dn6), (locals.var_phi_s0_soi__blk648_dn7 - locals.var_vbcs_cl__blk647_dn7), (locals.var_phi_s0_soi__blk648_dn10 - locals.var_vbcs_cl__blk647_dn10), (locals.var_phi_s0_soi__blk648_dn11 - locals.var_vbcs_cl__blk647_dn11), (locals.var_phi_s0_soi__blk648_dn12 - locals.var_vbcs_cl__blk647_dn12), (locals.var_phi_s0_soi__blk648_dn17 - locals.var_vbcs_cl__blk647_dn17),)
    } else {
        (locals.var_phi_soi0, locals.var_phi_soi0_dn0, locals.var_phi_soi0_dn2, locals.var_phi_soi0_dn6, locals.var_phi_soi0_dn7, locals.var_phi_soi0_dn10, locals.var_phi_soi0_dn11, locals.var_phi_soi0_dn12, locals.var_phi_soi0_dn17,)
    }
};
            locals.var_phi_soi0 = assign21320_body0_e29297;
            locals.var_phi_soi0_dn0 = assign21320_body0_e29297_d_n0;
            locals.var_phi_soi0_dn2 = assign21320_body0_e29297_d_n2;
            locals.var_phi_soi0_dn6 = assign21320_body0_e29297_d_n6;
            locals.var_phi_soi0_dn7 = assign21320_body0_e29297_d_n7;
            locals.var_phi_soi0_dn10 = assign21320_body0_e29297_d_n10;
            locals.var_phi_soi0_dn11 = assign21320_body0_e29297_d_n11;
            locals.var_phi_soi0_dn12 = assign21320_body0_e29297_d_n12;
            locals.var_phi_soi0_dn17 = assign21320_body0_e29297_d_n17;
            locals.var_phi_soi0_rv = 0.0;
            let (assign21320_body1_e29308, assign21320_body1_e29308_d_n0, assign21320_body1_e29308_d_n2, assign21320_body1_e29308_d_n6, assign21320_body1_e29308_d_n7, assign21320_body1_e29308_d_n10, assign21320_body1_e29308_d_n11, assign21320_body1_e29308_d_n12, assign21320_body1_e29308_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21320_body1_e29306: f64 = (locals.var_beta * locals.var_phi_soi0);
        (assign21320_body1_e29306, (locals.var_beta * locals.var_phi_soi0_dn0), (locals.var_beta * locals.var_phi_soi0_dn2), (locals.var_beta * locals.var_phi_soi0_dn6), (locals.var_beta * locals.var_phi_soi0_dn7), ((locals.var_beta_dn10 * locals.var_phi_soi0) + (locals.var_beta * locals.var_phi_soi0_dn10)), (locals.var_beta * locals.var_phi_soi0_dn11), (locals.var_beta * locals.var_phi_soi0_dn12), (locals.var_beta * locals.var_phi_soi0_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign21320_body1_e29308;
            locals.var_chi_dn0 = assign21320_body1_e29308_d_n0;
            locals.var_chi_dn2 = assign21320_body1_e29308_d_n2;
            locals.var_chi_dn6 = assign21320_body1_e29308_d_n6;
            locals.var_chi_dn7 = assign21320_body1_e29308_d_n7;
            locals.var_chi_dn10 = assign21320_body1_e29308_d_n10;
            locals.var_chi_dn11 = assign21320_body1_e29308_d_n11;
            locals.var_chi_dn12 = assign21320_body1_e29308_d_n12;
            locals.var_chi_dn17 = assign21320_body1_e29308_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign21320_body2_e29321, assign21320_body2_e29321_d_n0, assign21320_body2_e29321_d_n2, assign21320_body2_e29321_d_n6, assign21320_body2_e29321_d_n7, assign21320_body2_e29321_d_n10, assign21320_body2_e29321_d_n11, assign21320_body2_e29321_d_n12, assign21320_body2_e29321_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21320_body2_e29318: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        let assign21320_body2_e29319: f64 = (locals.var_c_sb * assign21320_body2_e29318);
        (assign21320_body2_e29319, ((locals.var_c_sb_dn0 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign21320_body2_e29318) + (locals.var_c_sb * (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign21320_body2_e29321;
            locals.var_ty_dn0 = assign21320_body2_e29321_d_n0;
            locals.var_ty_dn2 = assign21320_body2_e29321_d_n2;
            locals.var_ty_dn6 = assign21320_body2_e29321_d_n6;
            locals.var_ty_dn7 = assign21320_body2_e29321_d_n7;
            locals.var_ty_dn10 = assign21320_body2_e29321_d_n10;
            locals.var_ty_dn11 = assign21320_body2_e29321_d_n11;
            locals.var_ty_dn12 = assign21320_body2_e29321_d_n12;
            locals.var_ty_dn17 = assign21320_body2_e29321_d_n17;
            locals.var_ty_rv = 0.0;
            let assign21320_body3_e29324: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard651 = assign21320_body3_e29324;
            locals.var_guard651_rv = 0.0;
            let (assign21320_body4_e29336, assign21320_body4_e29336_d_n0, assign21320_body4_e29336_d_n2, assign21320_body4_e29336_d_n6, assign21320_body4_e29336_d_n7, assign21320_body4_e29336_d_n10, assign21320_body4_e29336_d_n11, assign21320_body4_e29336_d_n12, assign21320_body4_e29336_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21320_body4_e29334: f64 = (locals.var_ty).exp();
        (assign21320_body4_e29334, (assign21320_body4_e29334 * locals.var_ty_dn0), (assign21320_body4_e29334 * locals.var_ty_dn2), (assign21320_body4_e29334 * locals.var_ty_dn6), (assign21320_body4_e29334 * locals.var_ty_dn7), (assign21320_body4_e29334 * locals.var_ty_dn10), (assign21320_body4_e29334 * locals.var_ty_dn11), (assign21320_body4_e29334 * locals.var_ty_dn12), (assign21320_body4_e29334 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21320_body4_e29336;
            locals.var_t1_dn0 = assign21320_body4_e29336_d_n0;
            locals.var_t1_dn2 = assign21320_body4_e29336_d_n2;
            locals.var_t1_dn6 = assign21320_body4_e29336_d_n6;
            locals.var_t1_dn7 = assign21320_body4_e29336_d_n7;
            locals.var_t1_dn10 = assign21320_body4_e29336_d_n10;
            locals.var_t1_dn11 = assign21320_body4_e29336_d_n11;
            locals.var_t1_dn12 = assign21320_body4_e29336_d_n12;
            locals.var_t1_dn17 = assign21320_body4_e29336_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21320_body5_e29351, assign21320_body5_e29351_d_n0, assign21320_body5_e29351_d_n2, assign21320_body5_e29351_d_n6, assign21320_body5_e29351_d_n7, assign21320_body5_e29351_d_n10, assign21320_body5_e29351_d_n11, assign21320_body5_e29351_d_n12, assign21320_body5_e29351_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21320_body5_e29346: f64 = (-locals.var_c_sb);
        let assign21320_body5_e29348: f64 = (assign21320_body5_e29346 * locals.var_dphi_sb);
        let assign21320_body5_e29349: f64 = (assign21320_body5_e29348).exp();
        (assign21320_body5_e29349, (assign21320_body5_e29349 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn0))), (assign21320_body5_e29349 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn2))), (assign21320_body5_e29349 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn6))), (assign21320_body5_e29349 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn7))), (assign21320_body5_e29349 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn10))), (assign21320_body5_e29349 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn11))), (assign21320_body5_e29349 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn12))), (assign21320_body5_e29349 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign21320_body5_e29346 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21320_body5_e29351;
            locals.var_t0_dn0 = assign21320_body5_e29351_d_n0;
            locals.var_t0_dn2 = assign21320_body5_e29351_d_n2;
            locals.var_t0_dn6 = assign21320_body5_e29351_d_n6;
            locals.var_t0_dn7 = assign21320_body5_e29351_d_n7;
            locals.var_t0_dn10 = assign21320_body5_e29351_d_n10;
            locals.var_t0_dn11 = assign21320_body5_e29351_d_n11;
            locals.var_t0_dn12 = assign21320_body5_e29351_d_n12;
            locals.var_t0_dn17 = assign21320_body5_e29351_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21320_body6_e29364, assign21320_body6_e29364_d_n0, assign21320_body6_e29364_d_n2, assign21320_body6_e29364_d_n6, assign21320_body6_e29364_d_n7, assign21320_body6_e29364_d_n10, assign21320_body6_e29364_d_n11, assign21320_body6_e29364_d_n12, assign21320_body6_e29364_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21320_body6_e29362: f64 = (locals.var_t1 - locals.var_t0);
        (assign21320_body6_e29362, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign21320_body6_e29364;
            locals.var_t2_dn0 = assign21320_body6_e29364_d_n0;
            locals.var_t2_dn2 = assign21320_body6_e29364_d_n2;
            locals.var_t2_dn6 = assign21320_body6_e29364_d_n6;
            locals.var_t2_dn7 = assign21320_body6_e29364_d_n7;
            locals.var_t2_dn10 = assign21320_body6_e29364_d_n10;
            locals.var_t2_dn11 = assign21320_body6_e29364_d_n11;
            locals.var_t2_dn12 = assign21320_body6_e29364_d_n12;
            locals.var_t2_dn17 = assign21320_body6_e29364_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign21320_body7_e29380, assign21320_body7_e29380_d_n0, assign21320_body7_e29380_d_n2, assign21320_body7_e29380_d_n6, assign21320_body7_e29380_d_n7, assign21320_body7_e29380_d_n10, assign21320_body7_e29380_d_n11, assign21320_body7_e29380_d_n12, assign21320_body7_e29380_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21320_body7_e29375: f64 = (1.0 + locals.var_t2);
        let assign21320_body7_e29376: f64 = (assign21320_body7_e29375).ln();
        let assign21320_body7_e29378: f64 = (assign21320_body7_e29376 / locals.var_c_sb);
        (assign21320_body7_e29378, ((((locals.var_t2_dn0 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign21320_body7_e29375) * locals.var_c_sb) - (assign21320_body7_e29376 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign21320_body7_e29380;
            locals.var_phi_soib_dn0 = assign21320_body7_e29380_d_n0;
            locals.var_phi_soib_dn2 = assign21320_body7_e29380_d_n2;
            locals.var_phi_soib_dn6 = assign21320_body7_e29380_d_n6;
            locals.var_phi_soib_dn7 = assign21320_body7_e29380_d_n7;
            locals.var_phi_soib_dn10 = assign21320_body7_e29380_d_n10;
            locals.var_phi_soib_dn11 = assign21320_body7_e29380_d_n11;
            locals.var_phi_soib_dn12 = assign21320_body7_e29380_d_n12;
            locals.var_phi_soib_dn17 = assign21320_body7_e29380_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign21320_body8_e29395, assign21320_body8_e29395_d_n0, assign21320_body8_e29395_d_n2, assign21320_body8_e29395_d_n6, assign21320_body8_e29395_d_n7, assign21320_body8_e29395_d_n10, assign21320_body8_e29395_d_n11, assign21320_body8_e29395_d_n12, assign21320_body8_e29395_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21320_body8_e29392: f64 = (1.0 + locals.var_t2);
        let assign21320_body8_e29393: f64 = (locals.var_t1 / assign21320_body8_e29392);
        (assign21320_body8_e29393, (((locals.var_t1_dn0 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn0)) / (assign21320_body8_e29392 * assign21320_body8_e29392)), (((locals.var_t1_dn2 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn2)) / (assign21320_body8_e29392 * assign21320_body8_e29392)), (((locals.var_t1_dn6 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn6)) / (assign21320_body8_e29392 * assign21320_body8_e29392)), (((locals.var_t1_dn7 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn7)) / (assign21320_body8_e29392 * assign21320_body8_e29392)), (((locals.var_t1_dn10 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn10)) / (assign21320_body8_e29392 * assign21320_body8_e29392)), (((locals.var_t1_dn11 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn11)) / (assign21320_body8_e29392 * assign21320_body8_e29392)), (((locals.var_t1_dn12 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn12)) / (assign21320_body8_e29392 * assign21320_body8_e29392)), (((locals.var_t1_dn17 * assign21320_body8_e29392) - (locals.var_t1 * locals.var_t2_dn17)) / (assign21320_body8_e29392 * assign21320_body8_e29392)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign21320_body8_e29395;
            locals.var_phi_soib_dpss_dn0 = assign21320_body8_e29395_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign21320_body8_e29395_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign21320_body8_e29395_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign21320_body8_e29395_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign21320_body8_e29395_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign21320_body8_e29395_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign21320_body8_e29395_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign21320_body8_e29395_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign21320_body9_e29409, assign21320_body9_e29409_d_n0, assign21320_body9_e29409_d_n2, assign21320_body9_e29409_d_n6, assign21320_body9_e29409_d_n7, assign21320_body9_e29409_d_n10, assign21320_body9_e29409_d_n11, assign21320_body9_e29409_d_n12, assign21320_body9_e29409_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard651 == 0.0)) {
        let assign21320_body9_e29407: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        (assign21320_body9_e29407, (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign21320_body9_e29409;
            locals.var_phi_soib_dn0 = assign21320_body9_e29409_d_n0;
            locals.var_phi_soib_dn2 = assign21320_body9_e29409_d_n2;
            locals.var_phi_soib_dn6 = assign21320_body9_e29409_d_n6;
            locals.var_phi_soib_dn7 = assign21320_body9_e29409_d_n7;
            locals.var_phi_soib_dn10 = assign21320_body9_e29409_d_n10;
            locals.var_phi_soib_dn11 = assign21320_body9_e29409_d_n11;
            locals.var_phi_soib_dn12 = assign21320_body9_e29409_d_n12;
            locals.var_phi_soib_dn17 = assign21320_body9_e29409_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign21320_body10_e29421, assign21320_body10_e29421_d_n0, assign21320_body10_e29421_d_n2, assign21320_body10_e29421_d_n6, assign21320_body10_e29421_d_n7, assign21320_body10_e29421_d_n10, assign21320_body10_e29421_d_n11, assign21320_body10_e29421_d_n12, assign21320_body10_e29421_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard651 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign21320_body10_e29421;
            locals.var_phi_soib_dpss_dn0 = assign21320_body10_e29421_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign21320_body10_e29421_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign21320_body10_e29421_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign21320_body10_e29421_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign21320_body10_e29421_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign21320_body10_e29421_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign21320_body10_e29421_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign21320_body10_e29421_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign21320_body11_e29432, assign21320_body11_e29432_d_n0, assign21320_body11_e29432_d_n2, assign21320_body11_e29432_d_n6, assign21320_body11_e29432_d_n7, assign21320_body11_e29432_d_n10, assign21320_body11_e29432_d_n11, assign21320_body11_e29432_d_n12, assign21320_body11_e29432_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21320_body11_e29430: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign21320_body11_e29430, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign21320_body11_e29432;
            locals.var_chib_dn0 = assign21320_body11_e29432_d_n0;
            locals.var_chib_dn2 = assign21320_body11_e29432_d_n2;
            locals.var_chib_dn6 = assign21320_body11_e29432_d_n6;
            locals.var_chib_dn7 = assign21320_body11_e29432_d_n7;
            locals.var_chib_dn10 = assign21320_body11_e29432_d_n10;
            locals.var_chib_dn11 = assign21320_body11_e29432_d_n11;
            locals.var_chib_dn12 = assign21320_body11_e29432_d_n12;
            locals.var_chib_dn17 = assign21320_body11_e29432_d_n17;
            locals.var_chib_rv = 0.0;
            let assign21320_body12_e29434: f64 = (locals.var_chi).abs();
            let assign21320_body12_e29436: f64 = if assign21320_body12_e29434 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard652 = assign21320_body12_e29436;
            locals.var_guard652_rv = 0.0;
            let (assign21320_body13_e29454, assign21320_body13_e29454_d_n0, assign21320_body13_e29454_d_n2, assign21320_body13_e29454_d_n6, assign21320_body13_e29454_d_n7, assign21320_body13_e29454_d_n10, assign21320_body13_e29454_d_n11, assign21320_body13_e29454_d_n12, assign21320_body13_e29454_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21320_body13_e29448: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign21320_body13_e29449: f64 = (1.0 - assign21320_body13_e29448);
        let assign21320_body13_e29451: f64 = (assign21320_body13_e29449 / 2.0);
        let assign21320_body13_e29452: f64 = (assign21320_body13_e29451).sqrt();
        (assign21320_body13_e29452, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign21320_body13_e29452)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign21320_body13_e29452)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign21320_body13_e29452)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign21320_body13_e29452)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign21320_body13_e29452)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign21320_body13_e29452)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign21320_body13_e29452)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign21320_body13_e29452)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21320_body13_e29454;
            locals.var_t0_dn0 = assign21320_body13_e29454_d_n0;
            locals.var_t0_dn2 = assign21320_body13_e29454_d_n2;
            locals.var_t0_dn6 = assign21320_body13_e29454_d_n6;
            locals.var_t0_dn7 = assign21320_body13_e29454_d_n7;
            locals.var_t0_dn10 = assign21320_body13_e29454_d_n10;
            locals.var_t0_dn11 = assign21320_body13_e29454_d_n11;
            locals.var_t0_dn12 = assign21320_body13_e29454_d_n12;
            locals.var_t0_dn17 = assign21320_body13_e29454_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21320_body14_e29467, assign21320_body14_e29467_d_n0, assign21320_body14_e29467_d_n2, assign21320_body14_e29467_d_n6, assign21320_body14_e29467_d_n7, assign21320_body14_e29467_d_n10, assign21320_body14_e29467_d_n11, assign21320_body14_e29467_d_n12, assign21320_body14_e29467_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21320_body14_e29465: f64 = (locals.var_chi * locals.var_t0);
        (assign21320_body14_e29465, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21320_body14_e29467;
            locals.var_fb_dn0 = assign21320_body14_e29467_d_n0;
            locals.var_fb_dn2 = assign21320_body14_e29467_d_n2;
            locals.var_fb_dn6 = assign21320_body14_e29467_d_n6;
            locals.var_fb_dn7 = assign21320_body14_e29467_d_n7;
            locals.var_fb_dn10 = assign21320_body14_e29467_d_n10;
            locals.var_fb_dn11 = assign21320_body14_e29467_d_n11;
            locals.var_fb_dn12 = assign21320_body14_e29467_d_n12;
            locals.var_fb_dn17 = assign21320_body14_e29467_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21320_body15_e29480, assign21320_body15_e29480_d_n0, assign21320_body15_e29480_d_n2, assign21320_body15_e29480_d_n6, assign21320_body15_e29480_d_n7, assign21320_body15_e29480_d_n10, assign21320_body15_e29480_d_n11, assign21320_body15_e29480_d_n12, assign21320_body15_e29480_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21320_body15_e29478: f64 = (locals.var_beta * locals.var_t0);
        (assign21320_body15_e29478, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21320_body15_e29480;
            locals.var_fb_dpss_dn0 = assign21320_body15_e29480_d_n0;
            locals.var_fb_dpss_dn2 = assign21320_body15_e29480_d_n2;
            locals.var_fb_dpss_dn6 = assign21320_body15_e29480_d_n6;
            locals.var_fb_dpss_dn7 = assign21320_body15_e29480_d_n7;
            locals.var_fb_dpss_dn10 = assign21320_body15_e29480_d_n10;
            locals.var_fb_dpss_dn11 = assign21320_body15_e29480_d_n11;
            locals.var_fb_dpss_dn12 = assign21320_body15_e29480_d_n12;
            locals.var_fb_dpss_dn17 = assign21320_body15_e29480_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21320_body16_e29483: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard653 = assign21320_body16_e29483;
            locals.var_guard653_rv = 0.0;
            let (assign21320_body17_e29497, assign21320_body17_e29497_d_n0, assign21320_body17_e29497_d_n2, assign21320_body17_e29497_d_n6, assign21320_body17_e29497_d_n7, assign21320_body17_e29497_d_n10, assign21320_body17_e29497_d_n11, assign21320_body17_e29497_d_n12, assign21320_body17_e29497_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign21320_body17_e29495: f64 = (-locals.var_fb);
        (assign21320_body17_e29495, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21320_body17_e29497;
            locals.var_fb_dn0 = assign21320_body17_e29497_d_n0;
            locals.var_fb_dn2 = assign21320_body17_e29497_d_n2;
            locals.var_fb_dn6 = assign21320_body17_e29497_d_n6;
            locals.var_fb_dn7 = assign21320_body17_e29497_d_n7;
            locals.var_fb_dn10 = assign21320_body17_e29497_d_n10;
            locals.var_fb_dn11 = assign21320_body17_e29497_d_n11;
            locals.var_fb_dn12 = assign21320_body17_e29497_d_n12;
            locals.var_fb_dn17 = assign21320_body17_e29497_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21320_body18_e29511, assign21320_body18_e29511_d_n0, assign21320_body18_e29511_d_n2, assign21320_body18_e29511_d_n6, assign21320_body18_e29511_d_n7, assign21320_body18_e29511_d_n10, assign21320_body18_e29511_d_n11, assign21320_body18_e29511_d_n12, assign21320_body18_e29511_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign21320_body18_e29509: f64 = (-locals.var_fb_dpss);
        (assign21320_body18_e29509, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21320_body18_e29511;
            locals.var_fb_dpss_dn0 = assign21320_body18_e29511_d_n0;
            locals.var_fb_dpss_dn2 = assign21320_body18_e29511_d_n2;
            locals.var_fb_dpss_dn6 = assign21320_body18_e29511_d_n6;
            locals.var_fb_dpss_dn7 = assign21320_body18_e29511_d_n7;
            locals.var_fb_dpss_dn10 = assign21320_body18_e29511_d_n10;
            locals.var_fb_dpss_dn11 = assign21320_body18_e29511_d_n11;
            locals.var_fb_dpss_dn12 = assign21320_body18_e29511_d_n12;
            locals.var_fb_dpss_dn17 = assign21320_body18_e29511_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21320_body19_e29513: f64 = (locals.var_chi).abs();
            let assign21320_body19_e29515: f64 = if assign21320_body19_e29513 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard654 = assign21320_body19_e29515;
            locals.var_guard654_rv = 0.0;
            let (assign21320_body20_e29551, assign21320_body20_e29551_d_n0, assign21320_body20_e29551_d_n2, assign21320_body20_e29551_d_n6, assign21320_body20_e29551_d_n7, assign21320_body20_e29551_d_n10, assign21320_body20_e29551_d_n11, assign21320_body20_e29551_d_n12, assign21320_body20_e29551_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21320_body20_e29529: f64 = (locals.var_chi * locals.var_chi);
        let assign21320_body20_e29531: f64 = (assign21320_body20_e29529 / 2.0);
        let assign21320_body20_e29535: f64 = (locals.var_chi / 3.0);
        let assign21320_body20_e29539: f64 = (locals.var_chi / 4.0);
        let assign21320_body20_e29543: f64 = (locals.var_chi / 5.0);
        let assign21320_body20_e29544: f64 = (1.0 - assign21320_body20_e29543);
        let assign21320_body20_e29545: f64 = (assign21320_body20_e29539 * assign21320_body20_e29544);
        let assign21320_body20_e29546: f64 = (1.0 - assign21320_body20_e29545);
        let assign21320_body20_e29547: f64 = (assign21320_body20_e29535 * assign21320_body20_e29546);
        let assign21320_body20_e29548: f64 = (1.0 - assign21320_body20_e29547);
        let assign21320_body20_e29549: f64 = (assign21320_body20_e29531 * assign21320_body20_e29548);
        (assign21320_body20_e29549, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn0 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn0 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn2 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn2 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn6 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn6 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn7 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn7 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn10 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn10 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn11 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn11 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn12 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn12 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign21320_body20_e29548) + (assign21320_body20_e29531 * (-(((locals.var_chi_dn17 / 3.0) * assign21320_body20_e29546) + (assign21320_body20_e29535 * (-(((locals.var_chi_dn17 / 4.0) * assign21320_body20_e29544) + (assign21320_body20_e29539 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21320_body20_e29551;
            locals.var_t0_dn0 = assign21320_body20_e29551_d_n0;
            locals.var_t0_dn2 = assign21320_body20_e29551_d_n2;
            locals.var_t0_dn6 = assign21320_body20_e29551_d_n6;
            locals.var_t0_dn7 = assign21320_body20_e29551_d_n7;
            locals.var_t0_dn10 = assign21320_body20_e29551_d_n10;
            locals.var_t0_dn11 = assign21320_body20_e29551_d_n11;
            locals.var_t0_dn12 = assign21320_body20_e29551_d_n12;
            locals.var_t0_dn17 = assign21320_body20_e29551_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21320_body21_e29583, assign21320_body21_e29583_d_n0, assign21320_body21_e29583_d_n2, assign21320_body21_e29583_d_n6, assign21320_body21_e29583_d_n7, assign21320_body21_e29583_d_n10, assign21320_body21_e29583_d_n11, assign21320_body21_e29583_d_n12, assign21320_body21_e29583_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21320_body21_e29567: f64 = (locals.var_chi / 2.0);
        let assign21320_body21_e29571: f64 = (locals.var_chi / 3.0);
        let assign21320_body21_e29575: f64 = (locals.var_chi / 4.0);
        let assign21320_body21_e29576: f64 = (1.0 - assign21320_body21_e29575);
        let assign21320_body21_e29577: f64 = (assign21320_body21_e29571 * assign21320_body21_e29576);
        let assign21320_body21_e29578: f64 = (1.0 - assign21320_body21_e29577);
        let assign21320_body21_e29579: f64 = (assign21320_body21_e29567 * assign21320_body21_e29578);
        let assign21320_body21_e29580: f64 = (1.0 - assign21320_body21_e29579);
        let assign21320_body21_e29581: f64 = (locals.var_chi * assign21320_body21_e29580);
        (assign21320_body21_e29581, ((locals.var_chi_dn0 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn0 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn2 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn6 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn7 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn10 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn11 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn12 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign21320_body21_e29580) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign21320_body21_e29578) + (assign21320_body21_e29567 * (-(((locals.var_chi_dn17 / 3.0) * assign21320_body21_e29576) + (assign21320_body21_e29571 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21320_body21_e29583;
            locals.var_t1_dn0 = assign21320_body21_e29583_d_n0;
            locals.var_t1_dn2 = assign21320_body21_e29583_d_n2;
            locals.var_t1_dn6 = assign21320_body21_e29583_d_n6;
            locals.var_t1_dn7 = assign21320_body21_e29583_d_n7;
            locals.var_t1_dn10 = assign21320_body21_e29583_d_n10;
            locals.var_t1_dn11 = assign21320_body21_e29583_d_n11;
            locals.var_t1_dn12 = assign21320_body21_e29583_d_n12;
            locals.var_t1_dn17 = assign21320_body21_e29583_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21320_body22_e29619, assign21320_body22_e29619_d_n0, assign21320_body22_e29619_d_n2, assign21320_body22_e29619_d_n6, assign21320_body22_e29619_d_n7, assign21320_body22_e29619_d_n10, assign21320_body22_e29619_d_n11, assign21320_body22_e29619_d_n12, assign21320_body22_e29619_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21320_body22_e29597: f64 = (locals.var_chib * locals.var_chib);
        let assign21320_body22_e29599: f64 = (assign21320_body22_e29597 / 2.0);
        let assign21320_body22_e29603: f64 = (locals.var_chib / 3.0);
        let assign21320_body22_e29607: f64 = (locals.var_chib / 4.0);
        let assign21320_body22_e29611: f64 = (locals.var_chib / 5.0);
        let assign21320_body22_e29612: f64 = (1.0 - assign21320_body22_e29611);
        let assign21320_body22_e29613: f64 = (assign21320_body22_e29607 * assign21320_body22_e29612);
        let assign21320_body22_e29614: f64 = (1.0 - assign21320_body22_e29613);
        let assign21320_body22_e29615: f64 = (assign21320_body22_e29603 * assign21320_body22_e29614);
        let assign21320_body22_e29616: f64 = (1.0 - assign21320_body22_e29615);
        let assign21320_body22_e29617: f64 = (assign21320_body22_e29599 * assign21320_body22_e29616);
        (assign21320_body22_e29617, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn0 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn0 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn2 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn2 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn6 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn6 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn7 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn7 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn10 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn10 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn11 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn11 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn12 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn12 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign21320_body22_e29616) + (assign21320_body22_e29599 * (-(((locals.var_chib_dn17 / 3.0) * assign21320_body22_e29614) + (assign21320_body22_e29603 * (-(((locals.var_chib_dn17 / 4.0) * assign21320_body22_e29612) + (assign21320_body22_e29607 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign21320_body22_e29619;
            locals.var_t2_dn0 = assign21320_body22_e29619_d_n0;
            locals.var_t2_dn2 = assign21320_body22_e29619_d_n2;
            locals.var_t2_dn6 = assign21320_body22_e29619_d_n6;
            locals.var_t2_dn7 = assign21320_body22_e29619_d_n7;
            locals.var_t2_dn10 = assign21320_body22_e29619_d_n10;
            locals.var_t2_dn11 = assign21320_body22_e29619_d_n11;
            locals.var_t2_dn12 = assign21320_body22_e29619_d_n12;
            locals.var_t2_dn17 = assign21320_body22_e29619_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign21320_body23_e29651, assign21320_body23_e29651_d_n0, assign21320_body23_e29651_d_n2, assign21320_body23_e29651_d_n6, assign21320_body23_e29651_d_n7, assign21320_body23_e29651_d_n10, assign21320_body23_e29651_d_n11, assign21320_body23_e29651_d_n12, assign21320_body23_e29651_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21320_body23_e29635: f64 = (locals.var_chib / 2.0);
        let assign21320_body23_e29639: f64 = (locals.var_chib / 3.0);
        let assign21320_body23_e29643: f64 = (locals.var_chib / 4.0);
        let assign21320_body23_e29644: f64 = (1.0 - assign21320_body23_e29643);
        let assign21320_body23_e29645: f64 = (assign21320_body23_e29639 * assign21320_body23_e29644);
        let assign21320_body23_e29646: f64 = (1.0 - assign21320_body23_e29645);
        let assign21320_body23_e29647: f64 = (assign21320_body23_e29635 * assign21320_body23_e29646);
        let assign21320_body23_e29648: f64 = (1.0 - assign21320_body23_e29647);
        let assign21320_body23_e29649: f64 = (locals.var_chib * assign21320_body23_e29648);
        (assign21320_body23_e29649, ((locals.var_chib_dn0 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn0 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn2 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn6 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn7 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn10 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn11 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn12 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign21320_body23_e29648) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign21320_body23_e29646) + (assign21320_body23_e29635 * (-(((locals.var_chib_dn17 / 3.0) * assign21320_body23_e29644) + (assign21320_body23_e29639 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign21320_body23_e29651;
            locals.var_t3_dn0 = assign21320_body23_e29651_d_n0;
            locals.var_t3_dn2 = assign21320_body23_e29651_d_n2;
            locals.var_t3_dn6 = assign21320_body23_e29651_d_n6;
            locals.var_t3_dn7 = assign21320_body23_e29651_d_n7;
            locals.var_t3_dn10 = assign21320_body23_e29651_d_n10;
            locals.var_t3_dn11 = assign21320_body23_e29651_d_n11;
            locals.var_t3_dn12 = assign21320_body23_e29651_d_n12;
            locals.var_t3_dn17 = assign21320_body23_e29651_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign21320_body24_e29668, assign21320_body24_e29668_d_n0, assign21320_body24_e29668_d_n2, assign21320_body24_e29668_d_n6, assign21320_body24_e29668_d_n7, assign21320_body24_e29668_d_n10, assign21320_body24_e29668_d_n11, assign21320_body24_e29668_d_n12, assign21320_body24_e29668_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21320_body24_e29665: f64 = (locals.var_t0 - locals.var_t2);
        let assign21320_body24_e29666: f64 = (assign21320_body24_e29665).sqrt();
        (assign21320_body24_e29666, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign21320_body24_e29666)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign21320_body24_e29666)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign21320_body24_e29666)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign21320_body24_e29666)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign21320_body24_e29666)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign21320_body24_e29666)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign21320_body24_e29666)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign21320_body24_e29666)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21320_body24_e29668;
            locals.var_fb_dn0 = assign21320_body24_e29668_d_n0;
            locals.var_fb_dn2 = assign21320_body24_e29668_d_n2;
            locals.var_fb_dn6 = assign21320_body24_e29668_d_n6;
            locals.var_fb_dn7 = assign21320_body24_e29668_d_n7;
            locals.var_fb_dn10 = assign21320_body24_e29668_d_n10;
            locals.var_fb_dn11 = assign21320_body24_e29668_d_n11;
            locals.var_fb_dn12 = assign21320_body24_e29668_d_n12;
            locals.var_fb_dn17 = assign21320_body24_e29668_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21320_body25_e29692, assign21320_body25_e29692_d_n0, assign21320_body25_e29692_d_n2, assign21320_body25_e29692_d_n6, assign21320_body25_e29692_d_n7, assign21320_body25_e29692_d_n10, assign21320_body25_e29692_d_n11, assign21320_body25_e29692_d_n12, assign21320_body25_e29692_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21320_body25_e29682: f64 = (locals.var_beta * 0.5);
        let assign21320_body25_e29686: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign21320_body25_e29687: f64 = (locals.var_t1 - assign21320_body25_e29686);
        let assign21320_body25_e29688: f64 = (assign21320_body25_e29682 * assign21320_body25_e29687);
        let assign21320_body25_e29690: f64 = (assign21320_body25_e29688 / locals.var_fb);
        (assign21320_body25_e29690, ((((assign21320_body25_e29682 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body25_e29682 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body25_e29682 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body25_e29682 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign21320_body25_e29687) + (assign21320_body25_e29682 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body25_e29682 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body25_e29682 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body25_e29682 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign21320_body25_e29688 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21320_body25_e29692;
            locals.var_fb_dpss_dn0 = assign21320_body25_e29692_d_n0;
            locals.var_fb_dpss_dn2 = assign21320_body25_e29692_d_n2;
            locals.var_fb_dpss_dn6 = assign21320_body25_e29692_d_n6;
            locals.var_fb_dpss_dn7 = assign21320_body25_e29692_d_n7;
            locals.var_fb_dpss_dn10 = assign21320_body25_e29692_d_n10;
            locals.var_fb_dpss_dn11 = assign21320_body25_e29692_d_n11;
            locals.var_fb_dpss_dn12 = assign21320_body25_e29692_d_n12;
            locals.var_fb_dpss_dn17 = assign21320_body25_e29692_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign21320_body26_e29709, assign21320_body26_e29709_d_n0, assign21320_body26_e29709_d_n2, assign21320_body26_e29709_d_n6, assign21320_body26_e29709_d_n7, assign21320_body26_e29709_d_n10, assign21320_body26_e29709_d_n11, assign21320_body26_e29709_d_n12, assign21320_body26_e29709_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign21320_body26_e29706: f64 = (-locals.var_chi);
        let assign21320_body26_e29707: f64 = (assign21320_body26_e29706).exp();
        (assign21320_body26_e29707, (assign21320_body26_e29707 * (-locals.var_chi_dn0)), (assign21320_body26_e29707 * (-locals.var_chi_dn2)), (assign21320_body26_e29707 * (-locals.var_chi_dn6)), (assign21320_body26_e29707 * (-locals.var_chi_dn7)), (assign21320_body26_e29707 * (-locals.var_chi_dn10)), (assign21320_body26_e29707 * (-locals.var_chi_dn11)), (assign21320_body26_e29707 * (-locals.var_chi_dn12)), (assign21320_body26_e29707 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21320_body26_e29709;
            locals.var_t0_dn0 = assign21320_body26_e29709_d_n0;
            locals.var_t0_dn2 = assign21320_body26_e29709_d_n2;
            locals.var_t0_dn6 = assign21320_body26_e29709_d_n6;
            locals.var_t0_dn7 = assign21320_body26_e29709_d_n7;
            locals.var_t0_dn10 = assign21320_body26_e29709_d_n10;
            locals.var_t0_dn11 = assign21320_body26_e29709_d_n11;
            locals.var_t0_dn12 = assign21320_body26_e29709_d_n12;
            locals.var_t0_dn17 = assign21320_body26_e29709_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21320_body27_e29726, assign21320_body27_e29726_d_n0, assign21320_body27_e29726_d_n2, assign21320_body27_e29726_d_n6, assign21320_body27_e29726_d_n7, assign21320_body27_e29726_d_n10, assign21320_body27_e29726_d_n11, assign21320_body27_e29726_d_n12, assign21320_body27_e29726_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign21320_body27_e29723: f64 = (-locals.var_chib);
        let assign21320_body27_e29724: f64 = (assign21320_body27_e29723).exp();
        (assign21320_body27_e29724, (assign21320_body27_e29724 * (-locals.var_chib_dn0)), (assign21320_body27_e29724 * (-locals.var_chib_dn2)), (assign21320_body27_e29724 * (-locals.var_chib_dn6)), (assign21320_body27_e29724 * (-locals.var_chib_dn7)), (assign21320_body27_e29724 * (-locals.var_chib_dn10)), (assign21320_body27_e29724 * (-locals.var_chib_dn11)), (assign21320_body27_e29724 * (-locals.var_chib_dn12)), (assign21320_body27_e29724 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21320_body27_e29726;
            locals.var_t1_dn0 = assign21320_body27_e29726_d_n0;
            locals.var_t1_dn2 = assign21320_body27_e29726_d_n2;
            locals.var_t1_dn6 = assign21320_body27_e29726_d_n6;
            locals.var_t1_dn7 = assign21320_body27_e29726_d_n7;
            locals.var_t1_dn10 = assign21320_body27_e29726_d_n10;
            locals.var_t1_dn11 = assign21320_body27_e29726_d_n11;
            locals.var_t1_dn12 = assign21320_body27_e29726_d_n12;
            locals.var_t1_dn17 = assign21320_body27_e29726_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21320_body28_e29748, assign21320_body28_e29748_d_n0, assign21320_body28_e29748_d_n2, assign21320_body28_e29748_d_n6, assign21320_body28_e29748_d_n7, assign21320_body28_e29748_d_n10, assign21320_body28_e29748_d_n11, assign21320_body28_e29748_d_n12, assign21320_body28_e29748_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign21320_body28_e29741: f64 = (locals.var_chi - locals.var_chib);
        let assign21320_body28_e29744: f64 = (locals.var_t0 - locals.var_t1);
        let assign21320_body28_e29745: f64 = (assign21320_body28_e29741 + assign21320_body28_e29744);
        let assign21320_body28_e29746: f64 = (assign21320_body28_e29745).sqrt();
        (assign21320_body28_e29746, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign21320_body28_e29746)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign21320_body28_e29746)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign21320_body28_e29746)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign21320_body28_e29746)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign21320_body28_e29746)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign21320_body28_e29746)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign21320_body28_e29746)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign21320_body28_e29746)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21320_body28_e29748;
            locals.var_fb_dn0 = assign21320_body28_e29748_d_n0;
            locals.var_fb_dn2 = assign21320_body28_e29748_d_n2;
            locals.var_fb_dn6 = assign21320_body28_e29748_d_n6;
            locals.var_fb_dn7 = assign21320_body28_e29748_d_n7;
            locals.var_fb_dn10 = assign21320_body28_e29748_d_n10;
            locals.var_fb_dn11 = assign21320_body28_e29748_d_n11;
            locals.var_fb_dn12 = assign21320_body28_e29748_d_n12;
            locals.var_fb_dn17 = assign21320_body28_e29748_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21320_body29_e29777, assign21320_body29_e29777_d_n0, assign21320_body29_e29777_d_n2, assign21320_body29_e29777_d_n6, assign21320_body29_e29777_d_n7, assign21320_body29_e29777_d_n10, assign21320_body29_e29777_d_n11, assign21320_body29_e29777_d_n12, assign21320_body29_e29777_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign21320_body29_e29763: f64 = (locals.var_beta * 0.5);
        let assign21320_body29_e29766: f64 = (1.0 - locals.var_t0);
        let assign21320_body29_e29770: f64 = (1.0 - locals.var_t1);
        let assign21320_body29_e29771: f64 = (locals.var_phi_soib_dpss * assign21320_body29_e29770);
        let assign21320_body29_e29772: f64 = (assign21320_body29_e29766 - assign21320_body29_e29771);
        let assign21320_body29_e29773: f64 = (assign21320_body29_e29763 * assign21320_body29_e29772);
        let assign21320_body29_e29775: f64 = (assign21320_body29_e29773 / locals.var_fb);
        (assign21320_body29_e29775, ((((assign21320_body29_e29763 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body29_e29763 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body29_e29763 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body29_e29763 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign21320_body29_e29772) + (assign21320_body29_e29763 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body29_e29763 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body29_e29763 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign21320_body29_e29763 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign21320_body29_e29770) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign21320_body29_e29773 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21320_body29_e29777;
            locals.var_fb_dpss_dn0 = assign21320_body29_e29777_d_n0;
            locals.var_fb_dpss_dn2 = assign21320_body29_e29777_d_n2;
            locals.var_fb_dpss_dn6 = assign21320_body29_e29777_d_n6;
            locals.var_fb_dpss_dn7 = assign21320_body29_e29777_d_n7;
            locals.var_fb_dpss_dn10 = assign21320_body29_e29777_d_n10;
            locals.var_fb_dpss_dn11 = assign21320_body29_e29777_d_n11;
            locals.var_fb_dpss_dn12 = assign21320_body29_e29777_d_n12;
            locals.var_fb_dpss_dn17 = assign21320_body29_e29777_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21320_body30_e29784: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard655 = assign21320_body30_e29784;
            locals.var_guard655_rv = 0.0;
            let (assign21320_body31_e29796,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard655 != 0.0)) {
        let assign21320_body31_e29794: f64 = (-1.0);
        (assign21320_body31_e29794,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign21320_body31_e29796;
            locals.var_flg_zone_rv = 0.0;
            let assign21320_body32_e29799: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard656 = assign21320_body32_e29799;
            locals.var_guard656_rv = 0.0;
            let (assign21320_body33_e29811, assign21320_body33_e29811_d_n0, assign21320_body33_e29811_d_n2, assign21320_body33_e29811_d_n6, assign21320_body33_e29811_d_n7, assign21320_body33_e29811_d_n10, assign21320_body33_e29811_d_n11, assign21320_body33_e29811_d_n12, assign21320_body33_e29811_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 != 0.0)) {
        let assign21320_body33_e29809: f64 = (-locals.var_fb);
        (assign21320_body33_e29809, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21320_body33_e29811;
            locals.var_fs02_dn0 = assign21320_body33_e29811_d_n0;
            locals.var_fs02_dn2 = assign21320_body33_e29811_d_n2;
            locals.var_fs02_dn6 = assign21320_body33_e29811_d_n6;
            locals.var_fs02_dn7 = assign21320_body33_e29811_d_n7;
            locals.var_fs02_dn10 = assign21320_body33_e29811_d_n10;
            locals.var_fs02_dn11 = assign21320_body33_e29811_d_n11;
            locals.var_fs02_dn12 = assign21320_body33_e29811_d_n12;
            locals.var_fs02_dn17 = assign21320_body33_e29811_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21320_body34_e29823, assign21320_body34_e29823_d_n0, assign21320_body34_e29823_d_n2, assign21320_body34_e29823_d_n6, assign21320_body34_e29823_d_n7, assign21320_body34_e29823_d_n10, assign21320_body34_e29823_d_n11, assign21320_body34_e29823_d_n12, assign21320_body34_e29823_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 != 0.0)) {
        let assign21320_body34_e29821: f64 = (-locals.var_fb_dpss);
        (assign21320_body34_e29821, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21320_body34_e29823;
            locals.var_fs02_dps0_dn0 = assign21320_body34_e29823_d_n0;
            locals.var_fs02_dps0_dn2 = assign21320_body34_e29823_d_n2;
            locals.var_fs02_dps0_dn6 = assign21320_body34_e29823_d_n6;
            locals.var_fs02_dps0_dn7 = assign21320_body34_e29823_d_n7;
            locals.var_fs02_dps0_dn10 = assign21320_body34_e29823_d_n10;
            locals.var_fs02_dps0_dn11 = assign21320_body34_e29823_d_n11;
            locals.var_fs02_dps0_dn12 = assign21320_body34_e29823_d_n12;
            locals.var_fs02_dps0_dn17 = assign21320_body34_e29823_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign21320_body35_e29826: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard657 = assign21320_body35_e29826;
            locals.var_guard657_rv = 0.0;
            let (assign21320_body36_e29840, assign21320_body36_e29840_d_n0, assign21320_body36_e29840_d_n2, assign21320_body36_e29840_d_n6, assign21320_body36_e29840_d_n7, assign21320_body36_e29840_d_n10, assign21320_body36_e29840_d_n11, assign21320_body36_e29840_d_n12, assign21320_body36_e29840_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21320_body36_e29840;
            locals.var_fs02_dn0 = assign21320_body36_e29840_d_n0;
            locals.var_fs02_dn2 = assign21320_body36_e29840_d_n2;
            locals.var_fs02_dn6 = assign21320_body36_e29840_d_n6;
            locals.var_fs02_dn7 = assign21320_body36_e29840_d_n7;
            locals.var_fs02_dn10 = assign21320_body36_e29840_d_n10;
            locals.var_fs02_dn11 = assign21320_body36_e29840_d_n11;
            locals.var_fs02_dn12 = assign21320_body36_e29840_d_n12;
            locals.var_fs02_dn17 = assign21320_body36_e29840_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21320_body37_e29854, assign21320_body37_e29854_d_n0, assign21320_body37_e29854_d_n2, assign21320_body37_e29854_d_n6, assign21320_body37_e29854_d_n7, assign21320_body37_e29854_d_n10, assign21320_body37_e29854_d_n11, assign21320_body37_e29854_d_n12, assign21320_body37_e29854_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21320_body37_e29854;
            locals.var_fs02_dps0_dn0 = assign21320_body37_e29854_d_n0;
            locals.var_fs02_dps0_dn2 = assign21320_body37_e29854_d_n2;
            locals.var_fs02_dps0_dn6 = assign21320_body37_e29854_d_n6;
            locals.var_fs02_dps0_dn7 = assign21320_body37_e29854_d_n7;
            locals.var_fs02_dps0_dn10 = assign21320_body37_e29854_d_n10;
            locals.var_fs02_dps0_dn11 = assign21320_body37_e29854_d_n11;
            locals.var_fs02_dps0_dn12 = assign21320_body37_e29854_d_n12;
            locals.var_fs02_dps0_dn17 = assign21320_body37_e29854_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign21320_body38_e29873, assign21320_body38_e29873_d_n0, assign21320_body38_e29873_d_n2, assign21320_body38_e29873_d_n6, assign21320_body38_e29873_d_n7, assign21320_body38_e29873_d_n10, assign21320_body38_e29873_d_n11, assign21320_body38_e29873_d_n12, assign21320_body38_e29873_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) {
        let assign21320_body38_e29870: f64 = (locals.var_phi_s0_soi__blk648 - p.p287);
        let assign21320_body38_e29871: f64 = (locals.var_beta * assign21320_body38_e29870);
        (assign21320_body38_e29871, (locals.var_beta * locals.var_phi_s0_soi__blk648_dn0), (locals.var_beta * locals.var_phi_s0_soi__blk648_dn2), (locals.var_beta * locals.var_phi_s0_soi__blk648_dn6), (locals.var_beta * locals.var_phi_s0_soi__blk648_dn7), ((locals.var_beta_dn10 * assign21320_body38_e29870) + (locals.var_beta * locals.var_phi_s0_soi__blk648_dn10)), (locals.var_beta * locals.var_phi_s0_soi__blk648_dn11), (locals.var_beta * locals.var_phi_s0_soi__blk648_dn12), (locals.var_beta * locals.var_phi_s0_soi__blk648_dn17),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn17,)
    }
};
            locals.var_rho = assign21320_body38_e29873;
            locals.var_rho_dn0 = assign21320_body38_e29873_d_n0;
            locals.var_rho_dn2 = assign21320_body38_e29873_d_n2;
            locals.var_rho_dn6 = assign21320_body38_e29873_d_n6;
            locals.var_rho_dn7 = assign21320_body38_e29873_d_n7;
            locals.var_rho_dn10 = assign21320_body38_e29873_d_n10;
            locals.var_rho_dn11 = assign21320_body38_e29873_d_n11;
            locals.var_rho_dn12 = assign21320_body38_e29873_d_n12;
            locals.var_rho_dn17 = assign21320_body38_e29873_d_n17;
            locals.var_rho_rv = 0.0;
            let (assign21320_body39_e29889, assign21320_body39_e29889_d_n0, assign21320_body39_e29889_d_n2, assign21320_body39_e29889_d_n6, assign21320_body39_e29889_d_n7, assign21320_body39_e29889_d_n10, assign21320_body39_e29889_d_n11, assign21320_body39_e29889_d_n12, assign21320_body39_e29889_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) {
        let assign21320_body39_e29887: f64 = (locals.var_rho).exp();
        (assign21320_body39_e29887, (assign21320_body39_e29887 * locals.var_rho_dn0), (assign21320_body39_e29887 * locals.var_rho_dn2), (assign21320_body39_e29887 * locals.var_rho_dn6), (assign21320_body39_e29887 * locals.var_rho_dn7), (assign21320_body39_e29887 * locals.var_rho_dn10), (assign21320_body39_e29887 * locals.var_rho_dn11), (assign21320_body39_e29887 * locals.var_rho_dn12), (assign21320_body39_e29887 * locals.var_rho_dn17),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn12, locals.var_exp_rho_dn17,)
    }
};
            locals.var_exp_rho = assign21320_body39_e29889;
            locals.var_exp_rho_dn0 = assign21320_body39_e29889_d_n0;
            locals.var_exp_rho_dn2 = assign21320_body39_e29889_d_n2;
            locals.var_exp_rho_dn6 = assign21320_body39_e29889_d_n6;
            locals.var_exp_rho_dn7 = assign21320_body39_e29889_d_n7;
            locals.var_exp_rho_dn10 = assign21320_body39_e29889_d_n10;
            locals.var_exp_rho_dn11 = assign21320_body39_e29889_d_n11;
            locals.var_exp_rho_dn12 = assign21320_body39_e29889_d_n12;
            locals.var_exp_rho_dn17 = assign21320_body39_e29889_d_n17;
            locals.var_exp_rho_rv = 0.0;
            let (assign21320_body40_e29912, assign21320_body40_e29912_d_n0, assign21320_body40_e29912_d_n2, assign21320_body40_e29912_d_n6, assign21320_body40_e29912_d_n7, assign21320_body40_e29912_d_n10, assign21320_body40_e29912_d_n11, assign21320_body40_e29912_d_n12, assign21320_body40_e29912_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) {
        let assign21320_body40_e29907: f64 = (locals.var_chi + 1.0);
        let assign21320_body40_e29908: f64 = (locals.var_exp_bvbsvds * assign21320_body40_e29907);
        let assign21320_body40_e29909: f64 = (locals.var_exp_rho - assign21320_body40_e29908);
        let assign21320_body40_e29910: f64 = (locals.var_cnst1soi * assign21320_body40_e29909);
        (assign21320_body40_e29910, ((locals.var_cnst1soi_dn0 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn12 - ((locals.var_exp_bvbsvds_dn12 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign21320_body40_e29909) + (locals.var_cnst1soi * (locals.var_exp_rho_dn17 - ((locals.var_exp_bvbsvds_dn17 * assign21320_body40_e29907) + (locals.var_exp_bvbsvds * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign21320_body40_e29912;
            locals.var_fs01_dn0 = assign21320_body40_e29912_d_n0;
            locals.var_fs01_dn2 = assign21320_body40_e29912_d_n2;
            locals.var_fs01_dn6 = assign21320_body40_e29912_d_n6;
            locals.var_fs01_dn7 = assign21320_body40_e29912_d_n7;
            locals.var_fs01_dn10 = assign21320_body40_e29912_d_n10;
            locals.var_fs01_dn11 = assign21320_body40_e29912_d_n11;
            locals.var_fs01_dn12 = assign21320_body40_e29912_d_n12;
            locals.var_fs01_dn17 = assign21320_body40_e29912_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign21320_body41_e29933, assign21320_body41_e29933_d_n0, assign21320_body41_e29933_d_n2, assign21320_body41_e29933_d_n6, assign21320_body41_e29933_d_n7, assign21320_body41_e29933_d_n10, assign21320_body41_e29933_d_n11, assign21320_body41_e29933_d_n12, assign21320_body41_e29933_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) {
        let assign21320_body41_e29927: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign21320_body41_e29930: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign21320_body41_e29931: f64 = (assign21320_body41_e29927 * assign21320_body41_e29930);
        (assign21320_body41_e29931, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn12 - locals.var_exp_bvbsvds_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign21320_body41_e29930) + (assign21320_body41_e29927 * (locals.var_exp_rho_dn17 - locals.var_exp_bvbsvds_dn17))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign21320_body41_e29933;
            locals.var_fs01_dps0_dn0 = assign21320_body41_e29933_d_n0;
            locals.var_fs01_dps0_dn2 = assign21320_body41_e29933_d_n2;
            locals.var_fs01_dps0_dn6 = assign21320_body41_e29933_d_n6;
            locals.var_fs01_dps0_dn7 = assign21320_body41_e29933_d_n7;
            locals.var_fs01_dps0_dn10 = assign21320_body41_e29933_d_n10;
            locals.var_fs01_dps0_dn11 = assign21320_body41_e29933_d_n11;
            locals.var_fs01_dps0_dn12 = assign21320_body41_e29933_d_n12;
            locals.var_fs01_dps0_dn17 = assign21320_body41_e29933_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign21320_body42_e29953, assign21320_body42_e29953_d_n0, assign21320_body42_e29953_d_n2, assign21320_body42_e29953_d_n6, assign21320_body42_e29953_d_n7, assign21320_body42_e29953_d_n10, assign21320_body42_e29953_d_n11, assign21320_body42_e29953_d_n12, assign21320_body42_e29953_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) {
        let assign21320_body42_e29948: f64 = (locals.var_fb * locals.var_fb);
        let assign21320_body42_e29950: f64 = (assign21320_body42_e29948 + locals.var_fs01);
        let assign21320_body42_e29951: f64 = (assign21320_body42_e29950).sqrt();
        (assign21320_body42_e29951, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign21320_body42_e29951)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign21320_body42_e29951)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign21320_body42_e29951)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign21320_body42_e29951)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign21320_body42_e29951)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign21320_body42_e29951)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign21320_body42_e29951)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fs01_dn17) / (2.0 * assign21320_body42_e29951)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21320_body42_e29953;
            locals.var_fs02_dn0 = assign21320_body42_e29953_d_n0;
            locals.var_fs02_dn2 = assign21320_body42_e29953_d_n2;
            locals.var_fs02_dn6 = assign21320_body42_e29953_d_n6;
            locals.var_fs02_dn7 = assign21320_body42_e29953_d_n7;
            locals.var_fs02_dn10 = assign21320_body42_e29953_d_n10;
            locals.var_fs02_dn11 = assign21320_body42_e29953_d_n11;
            locals.var_fs02_dn12 = assign21320_body42_e29953_d_n12;
            locals.var_fs02_dn17 = assign21320_body42_e29953_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21320_body43_e29978, assign21320_body43_e29978_d_n0, assign21320_body43_e29978_d_n2, assign21320_body43_e29978_d_n6, assign21320_body43_e29978_d_n7, assign21320_body43_e29978_d_n10, assign21320_body43_e29978_d_n11, assign21320_body43_e29978_d_n12, assign21320_body43_e29978_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard656 == 0.0)) && (locals.var_guard657 == 0.0)) {
        let assign21320_body43_e29969: f64 = (2.0 * locals.var_fb_dpss);
        let assign21320_body43_e29971: f64 = (assign21320_body43_e29969 * locals.var_fb);
        let assign21320_body43_e29973: f64 = (assign21320_body43_e29971 + locals.var_fs01_dps0);
        let assign21320_body43_e29974: f64 = (0.5 * assign21320_body43_e29973);
        let assign21320_body43_e29976: f64 = (assign21320_body43_e29974 / locals.var_fs02);
        (assign21320_body43_e29976, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign21320_body43_e29969 * locals.var_fb_dn17)) + locals.var_fs01_dps0_dn17)) * locals.var_fs02) - (assign21320_body43_e29974 * locals.var_fs02_dn17)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21320_body43_e29978;
            locals.var_fs02_dps0_dn0 = assign21320_body43_e29978_d_n0;
            locals.var_fs02_dps0_dn2 = assign21320_body43_e29978_d_n2;
            locals.var_fs02_dps0_dn6 = assign21320_body43_e29978_d_n6;
            locals.var_fs02_dps0_dn7 = assign21320_body43_e29978_d_n7;
            locals.var_fs02_dps0_dn10 = assign21320_body43_e29978_d_n10;
            locals.var_fs02_dps0_dn11 = assign21320_body43_e29978_d_n11;
            locals.var_fs02_dps0_dn12 = assign21320_body43_e29978_d_n12;
            locals.var_fs02_dps0_dn17 = assign21320_body43_e29978_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign21320_body44_e29994, assign21320_body44_e29994_d_n0, assign21320_body44_e29994_d_n2, assign21320_body44_e29994_d_n6, assign21320_body44_e29994_d_n7, assign21320_body44_e29994_d_n10, assign21320_body44_e29994_d_n11, assign21320_body44_e29994_d_n12, assign21320_body44_e29994_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21320_body44_e29986: f64 = (-locals.var_vgp__blk612);
        let assign21320_body44_e29988: f64 = (assign21320_body44_e29986 + locals.var_phi_s0_soi__blk648);
        let assign21320_body44_e29991: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign21320_body44_e29992: f64 = (assign21320_body44_e29988 + assign21320_body44_e29991);
        (assign21320_body44_e29992, (((-locals.var_vgp__blk612_dn0) + locals.var_phi_s0_soi__blk648_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgp__blk612_dn2) + locals.var_phi_s0_soi__blk648_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (((-locals.var_vgp__blk612_dn6) + locals.var_phi_s0_soi__blk648_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgp__blk612_dn7) + locals.var_phi_s0_soi__blk648_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgp__blk612_dn10) + locals.var_phi_s0_soi__blk648_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (((-locals.var_vgp__blk612_dn11) + locals.var_phi_s0_soi__blk648_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (((-locals.var_vgp__blk612_dn12) + locals.var_phi_s0_soi__blk648_dn12) + ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))), (((-locals.var_vgp__blk612_dn17) + locals.var_phi_s0_soi__blk648_dn17) + ((locals.var_fac1_dn17 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn17))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, locals.var_fs0_dn17,)
    }
};
            locals.var_fs0 = assign21320_body44_e29994;
            locals.var_fs0_dn0 = assign21320_body44_e29994_d_n0;
            locals.var_fs0_dn2 = assign21320_body44_e29994_d_n2;
            locals.var_fs0_dn6 = assign21320_body44_e29994_d_n6;
            locals.var_fs0_dn7 = assign21320_body44_e29994_d_n7;
            locals.var_fs0_dn10 = assign21320_body44_e29994_d_n10;
            locals.var_fs0_dn11 = assign21320_body44_e29994_d_n11;
            locals.var_fs0_dn12 = assign21320_body44_e29994_d_n12;
            locals.var_fs0_dn17 = assign21320_body44_e29994_d_n17;
            locals.var_fs0_rv = 0.0;
            let (assign21320_body45_e30007, assign21320_body45_e30007_d_n0, assign21320_body45_e30007_d_n2, assign21320_body45_e30007_d_n6, assign21320_body45_e30007_d_n7, assign21320_body45_e30007_d_n10, assign21320_body45_e30007_d_n11, assign21320_body45_e30007_d_n12, assign21320_body45_e30007_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21320_body45_e30004: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign21320_body45_e30005: f64 = (1.0 + assign21320_body45_e30004);
        (assign21320_body45_e30005, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12)), ((locals.var_fac1_dn17 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn17)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, locals.var_fs0_dps0_dn17,)
    }
};
            locals.var_fs0_dps0 = assign21320_body45_e30007;
            locals.var_fs0_dps0_dn0 = assign21320_body45_e30007_d_n0;
            locals.var_fs0_dps0_dn2 = assign21320_body45_e30007_d_n2;
            locals.var_fs0_dps0_dn6 = assign21320_body45_e30007_d_n6;
            locals.var_fs0_dps0_dn7 = assign21320_body45_e30007_d_n7;
            locals.var_fs0_dps0_dn10 = assign21320_body45_e30007_d_n10;
            locals.var_fs0_dps0_dn11 = assign21320_body45_e30007_d_n11;
            locals.var_fs0_dps0_dn12 = assign21320_body45_e30007_d_n12;
            locals.var_fs0_dps0_dn17 = assign21320_body45_e30007_d_n17;
            locals.var_fs0_dps0_rv = 0.0;
            let assign21320_body46_e30010: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard658 = assign21320_body46_e30010;
            locals.var_guard658_rv = 0.0;
            let (assign21320_body47_e30023,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign21320_body47_e30021: f64 = (locals.var_lp_s0_max + 1.0);
        (assign21320_body47_e30021,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign21320_body47_e30023;
            locals.var_lp_s0_rv = 0.0;
            let (assign21320_body48_e30038, assign21320_body48_e30038_d_n0, assign21320_body48_e30038_d_n2, assign21320_body48_e30038_d_n6, assign21320_body48_e30038_d_n7, assign21320_body48_e30038_d_n10, assign21320_body48_e30038_d_n11, assign21320_body48_e30038_d_n12, assign21320_body48_e30038_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard658 == 0.0)) {
        let assign21320_body48_e30034: f64 = (-locals.var_fs0);
        let assign21320_body48_e30036: f64 = (assign21320_body48_e30034 / locals.var_fs0_dps0);
        (assign21320_body48_e30036, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn17) * locals.var_fs0_dps0) - (assign21320_body48_e30034 * locals.var_fs0_dps0_dn17)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign21320_body48_e30038;
            locals.var_dps0_dn0 = assign21320_body48_e30038_d_n0;
            locals.var_dps0_dn2 = assign21320_body48_e30038_d_n2;
            locals.var_dps0_dn6 = assign21320_body48_e30038_d_n6;
            locals.var_dps0_dn7 = assign21320_body48_e30038_d_n7;
            locals.var_dps0_dn10 = assign21320_body48_e30038_d_n10;
            locals.var_dps0_dn11 = assign21320_body48_e30038_d_n11;
            locals.var_dps0_dn12 = assign21320_body48_e30038_d_n12;
            locals.var_dps0_dn17 = assign21320_body48_e30038_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign21320_body49_e30063, assign21320_body49_e30063_d_n0, assign21320_body49_e30063_d_n2, assign21320_body49_e30063_d_n6, assign21320_body49_e30063_d_n7, assign21320_body49_e30063_d_n10, assign21320_body49_e30063_d_n11, assign21320_body49_e30063_d_n12, assign21320_body49_e30063_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard658 == 0.0)) {
        let assign21320_body49_e30050: f64 = (0.5 * 0.1);
        let assign21320_body49_e30054: f64 = (locals.var_phi_s0_soi__blk648).abs();
        let (assign21320_body49_e30059, assign21320_body49_e30059_d_n0, assign21320_body49_e30059_d_n2, assign21320_body49_e30059_d_n6, assign21320_body49_e30059_d_n7, assign21320_body49_e30059_d_n10, assign21320_body49_e30059_d_n11, assign21320_body49_e30059_d_n12, assign21320_body49_e30059_d_n17,) = {
            if (1.0 >= assign21320_body49_e30054) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign21320_body49_e30058: f64 = (locals.var_phi_s0_soi__blk648).abs();
                (assign21320_body49_e30058, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn0 } else { (-locals.var_phi_s0_soi__blk648_dn0) }, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn2 } else { (-locals.var_phi_s0_soi__blk648_dn2) }, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn6 } else { (-locals.var_phi_s0_soi__blk648_dn6) }, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn7 } else { (-locals.var_phi_s0_soi__blk648_dn7) }, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn10 } else { (-locals.var_phi_s0_soi__blk648_dn10) }, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn11 } else { (-locals.var_phi_s0_soi__blk648_dn11) }, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn12 } else { (-locals.var_phi_s0_soi__blk648_dn12) }, if locals.var_phi_s0_soi__blk648 >= 0.0 { locals.var_phi_s0_soi__blk648_dn17 } else { (-locals.var_phi_s0_soi__blk648_dn17) },)
            }
        };
        let assign21320_body49_e30060: f64 = (1.0 + assign21320_body49_e30059);
        let assign21320_body49_e30061: f64 = (assign21320_body49_e30050 * assign21320_body49_e30060);
        (assign21320_body49_e30061, (assign21320_body49_e30050 * assign21320_body49_e30059_d_n0), (assign21320_body49_e30050 * assign21320_body49_e30059_d_n2), (assign21320_body49_e30050 * assign21320_body49_e30059_d_n6), (assign21320_body49_e30050 * assign21320_body49_e30059_d_n7), (assign21320_body49_e30050 * assign21320_body49_e30059_d_n10), (assign21320_body49_e30050 * assign21320_body49_e30059_d_n11), (assign21320_body49_e30050 * assign21320_body49_e30059_d_n12), (assign21320_body49_e30050 * assign21320_body49_e30059_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign21320_body49_e30063;
            locals.var_dplim_dn0 = assign21320_body49_e30063_d_n0;
            locals.var_dplim_dn2 = assign21320_body49_e30063_d_n2;
            locals.var_dplim_dn6 = assign21320_body49_e30063_d_n6;
            locals.var_dplim_dn7 = assign21320_body49_e30063_d_n7;
            locals.var_dplim_dn10 = assign21320_body49_e30063_d_n10;
            locals.var_dplim_dn11 = assign21320_body49_e30063_d_n11;
            locals.var_dplim_dn12 = assign21320_body49_e30063_d_n12;
            locals.var_dplim_dn17 = assign21320_body49_e30063_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign21320_body50_e30065: f64 = (locals.var_dps0).abs();
            let assign21320_body50_e30067: f64 = if assign21320_body50_e30065 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard659 = assign21320_body50_e30067;
            locals.var_guard659_rv = 0.0;
            let (assign21320_body51_e30089, assign21320_body51_e30089_d_n0, assign21320_body51_e30089_d_n2, assign21320_body51_e30089_d_n6, assign21320_body51_e30089_d_n7, assign21320_body51_e30089_d_n10, assign21320_body51_e30089_d_n11, assign21320_body51_e30089_d_n12, assign21320_body51_e30089_d_n17,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard658 == 0.0)) && (locals.var_guard659 != 0.0)) {
        let (assign21320_body51_e30086,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign21320_body51_e30085: f64 = (-1.0);
                (assign21320_body51_e30085,)
            }
        };
        let assign21320_body51_e30087: f64 = (locals.var_dplim * assign21320_body51_e30086);
        (assign21320_body51_e30087, (locals.var_dplim_dn0 * assign21320_body51_e30086), (locals.var_dplim_dn2 * assign21320_body51_e30086), (locals.var_dplim_dn6 * assign21320_body51_e30086), (locals.var_dplim_dn7 * assign21320_body51_e30086), (locals.var_dplim_dn10 * assign21320_body51_e30086), (locals.var_dplim_dn11 * assign21320_body51_e30086), (locals.var_dplim_dn12 * assign21320_body51_e30086), (locals.var_dplim_dn17 * assign21320_body51_e30086),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign21320_body51_e30089;
            locals.var_dps0_dn0 = assign21320_body51_e30089_d_n0;
            locals.var_dps0_dn2 = assign21320_body51_e30089_d_n2;
            locals.var_dps0_dn6 = assign21320_body51_e30089_d_n6;
            locals.var_dps0_dn7 = assign21320_body51_e30089_d_n7;
            locals.var_dps0_dn10 = assign21320_body51_e30089_d_n10;
            locals.var_dps0_dn11 = assign21320_body51_e30089_d_n11;
            locals.var_dps0_dn12 = assign21320_body51_e30089_d_n12;
            locals.var_dps0_dn17 = assign21320_body51_e30089_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign21320_body52_e30103, assign21320_body52_e30103_d_n0, assign21320_body52_e30103_d_n2, assign21320_body52_e30103_d_n6, assign21320_body52_e30103_d_n7, assign21320_body52_e30103_d_n10, assign21320_body52_e30103_d_n11, assign21320_body52_e30103_d_n12, assign21320_body52_e30103_d_n17,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard658 == 0.0)) {
        let assign21320_body52_e30101: f64 = (locals.var_phi_s0_soi__blk648 + locals.var_dps0);
        (assign21320_body52_e30101, (locals.var_phi_s0_soi__blk648_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_soi__blk648_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_soi__blk648_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_soi__blk648_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_soi__blk648_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_soi__blk648_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_soi__blk648_dn12 + locals.var_dps0_dn12), (locals.var_phi_s0_soi__blk648_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_phi_s0_soi__blk648, locals.var_phi_s0_soi__blk648_dn0, locals.var_phi_s0_soi__blk648_dn2, locals.var_phi_s0_soi__blk648_dn6, locals.var_phi_s0_soi__blk648_dn7, locals.var_phi_s0_soi__blk648_dn10, locals.var_phi_s0_soi__blk648_dn11, locals.var_phi_s0_soi__blk648_dn12, locals.var_phi_s0_soi__blk648_dn17,)
    }
};
            locals.var_phi_s0_soi__blk648 = assign21320_body52_e30103;
            locals.var_phi_s0_soi__blk648_dn0 = assign21320_body52_e30103_d_n0;
            locals.var_phi_s0_soi__blk648_dn2 = assign21320_body52_e30103_d_n2;
            locals.var_phi_s0_soi__blk648_dn6 = assign21320_body52_e30103_d_n6;
            locals.var_phi_s0_soi__blk648_dn7 = assign21320_body52_e30103_d_n7;
            locals.var_phi_s0_soi__blk648_dn10 = assign21320_body52_e30103_d_n10;
            locals.var_phi_s0_soi__blk648_dn11 = assign21320_body52_e30103_d_n11;
            locals.var_phi_s0_soi__blk648_dn12 = assign21320_body52_e30103_d_n12;
            locals.var_phi_s0_soi__blk648_dn17 = assign21320_body52_e30103_d_n17;
            locals.var_phi_s0_soi__blk648_rv = 0.0;
            let assign21320_body53_e30105: f64 = (locals.var_dps0).abs();
            let assign21320_body53_e30109: f64 = (locals.var_fs0).abs();
            let assign21320_body53_e30112: f64 = if ((assign21320_body53_e30105 <= 5e-12) && (assign21320_body53_e30109 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard660 = assign21320_body53_e30112;
            locals.var_guard660_rv = 0.0;
            let (assign21320_body54_e30126,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard658 == 0.0)) && (locals.var_guard660 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign21320_body54_e30126;
            locals.var_flg_conv_rv = 0.0;
            let (assign21320_body55_e30137,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        let assign21320_body55_e30135: f64 = (locals.var_lp_s0 + 1.0);
        (assign21320_body55_e30135,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign21320_body55_e30137;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign21330_e30146, assign21330_e30146_d_n0, assign21330_e30146_d_n2, assign21330_e30146_d_n6, assign21330_e30146_d_n7, assign21330_e30146_d_n10, assign21330_e30146_d_n11, assign21330_e30146_d_n12, assign21330_e30146_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard649 == 0.0)) {
        (locals.var_phi_s0_soi__blk648, locals.var_phi_s0_soi__blk648_dn0, locals.var_phi_s0_soi__blk648_dn2, locals.var_phi_s0_soi__blk648_dn6, locals.var_phi_s0_soi__blk648_dn7, locals.var_phi_s0_soi__blk648_dn10, locals.var_phi_s0_soi__blk648_dn11, locals.var_phi_s0_soi__blk648_dn12, locals.var_phi_s0_soi__blk648_dn17,)
    } else {
        (locals.var_ps0__blk610, locals.var_ps0__blk610_dn0, locals.var_ps0__blk610_dn2, locals.var_ps0__blk610_dn6, locals.var_ps0__blk610_dn7, locals.var_ps0__blk610_dn10, locals.var_ps0__blk610_dn11, locals.var_ps0__blk610_dn12, locals.var_ps0__blk610_dn17,)
    }
};
        locals.var_ps0__blk610 = assign21330_e30146;
        locals.var_ps0__blk610_dn0 = assign21330_e30146_d_n0;
        locals.var_ps0__blk610_dn2 = assign21330_e30146_d_n2;
        locals.var_ps0__blk610_dn6 = assign21330_e30146_d_n6;
        locals.var_ps0__blk610_dn7 = assign21330_e30146_d_n7;
        locals.var_ps0__blk610_dn10 = assign21330_e30146_d_n10;
        locals.var_ps0__blk610_dn11 = assign21330_e30146_d_n11;
        locals.var_ps0__blk610_dn12 = assign21330_e30146_d_n12;
        locals.var_ps0__blk610_dn17 = assign21330_e30146_d_n17;
        locals.var_ps0__blk610_rv = 0.0;

        let (assign21340_e30155, assign21340_e30155_d_n0, assign21340_e30155_d_n2, assign21340_e30155_d_n6, assign21340_e30155_d_n7, assign21340_e30155_d_n10, assign21340_e30155_d_n11, assign21340_e30155_d_n12, assign21340_e30155_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21340_e30149: f64 = (-locals.var_beta);
        let assign21340_e30152: f64 = (locals.var_ps0__blk610 - locals.var_dphi_vds);
        let assign21340_e30153: f64 = (assign21340_e30149 * assign21340_e30152);
        (assign21340_e30153, (assign21340_e30149 * (locals.var_ps0__blk610_dn0 - locals.var_dphi_vds_dn0)), (assign21340_e30149 * (locals.var_ps0__blk610_dn2 - locals.var_dphi_vds_dn2)), (assign21340_e30149 * (locals.var_ps0__blk610_dn6 - locals.var_dphi_vds_dn6)), (assign21340_e30149 * (locals.var_ps0__blk610_dn7 - locals.var_dphi_vds_dn7)), (((-locals.var_beta_dn10) * assign21340_e30152) + (assign21340_e30149 * (locals.var_ps0__blk610_dn10 - locals.var_dphi_vds_dn10))), (assign21340_e30149 * (locals.var_ps0__blk610_dn11 - locals.var_dphi_vds_dn11)), (assign21340_e30149 * (locals.var_ps0__blk610_dn12 - locals.var_dphi_vds_dn12)), (assign21340_e30149 * (locals.var_ps0__blk610_dn17 - locals.var_dphi_vds_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign21340_e30155;
        locals.var_t5_dn0 = assign21340_e30155_d_n0;
        locals.var_t5_dn2 = assign21340_e30155_d_n2;
        locals.var_t5_dn6 = assign21340_e30155_d_n6;
        locals.var_t5_dn7 = assign21340_e30155_d_n7;
        locals.var_t5_dn10 = assign21340_e30155_d_n10;
        locals.var_t5_dn11 = assign21340_e30155_d_n11;
        locals.var_t5_dn12 = assign21340_e30155_d_n12;
        locals.var_t5_dn17 = assign21340_e30155_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign21350_e30165,) = {
    if (locals.var_guard600 != 0.0) {
        let (assign21350_e30163,) = {
            if (locals.var_t5 >= 0.0) {
                (1.0,)
            } else {
                let assign21350_e30162: f64 = (-1.0);
                (assign21350_e30162,)
            }
        };
        (assign21350_e30163,)
    } else {
        (locals.var_t5sign,)
    }
};
        locals.var_t5sign = assign21350_e30165;
        locals.var_t5sign_rv = 0.0;

        let (assign21360_e30171, assign21360_e30171_d_n0, assign21360_e30171_d_n2, assign21360_e30171_d_n6, assign21360_e30171_d_n7, assign21360_e30171_d_n10, assign21360_e30171_d_n11, assign21360_e30171_d_n12, assign21360_e30171_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21360_e30169: f64 = (locals.var_t5sign * locals.var_t5);
        (assign21360_e30169, (locals.var_t5sign * locals.var_t5_dn0), (locals.var_t5sign * locals.var_t5_dn2), (locals.var_t5sign * locals.var_t5_dn6), (locals.var_t5sign * locals.var_t5_dn7), (locals.var_t5sign * locals.var_t5_dn10), (locals.var_t5sign * locals.var_t5_dn11), (locals.var_t5sign * locals.var_t5_dn12), (locals.var_t5sign * locals.var_t5_dn17),)
    } else {
        (locals.var_t5y, locals.var_t5y_dn0, locals.var_t5y_dn2, locals.var_t5y_dn6, locals.var_t5y_dn7, locals.var_t5y_dn10, locals.var_t5y_dn11, locals.var_t5y_dn12, locals.var_t5y_dn17,)
    }
};
        locals.var_t5y = assign21360_e30171;
        locals.var_t5y_dn0 = assign21360_e30171_d_n0;
        locals.var_t5y_dn2 = assign21360_e30171_d_n2;
        locals.var_t5y_dn6 = assign21360_e30171_d_n6;
        locals.var_t5y_dn7 = assign21360_e30171_d_n7;
        locals.var_t5y_dn10 = assign21360_e30171_d_n10;
        locals.var_t5y_dn11 = assign21360_e30171_d_n11;
        locals.var_t5y_dn12 = assign21360_e30171_d_n12;
        locals.var_t5y_dn17 = assign21360_e30171_d_n17;
        locals.var_t5y_rv = 0.0;

        let (assign21370_e30176, assign21370_e30176_d_n0, assign21370_e30176_d_n2, assign21370_e30176_d_n6, assign21370_e30176_d_n7, assign21370_e30176_d_n10, assign21370_e30176_d_n11, assign21370_e30176_d_n12, assign21370_e30176_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21370_e30174: f64 = (locals.var_t5).exp();
        (assign21370_e30174, (assign21370_e30174 * locals.var_t5_dn0), (assign21370_e30174 * locals.var_t5_dn2), (assign21370_e30174 * locals.var_t5_dn6), (assign21370_e30174 * locals.var_t5_dn7), (assign21370_e30174 * locals.var_t5_dn10), (assign21370_e30174 * locals.var_t5_dn11), (assign21370_e30174 * locals.var_t5_dn12), (assign21370_e30174 * locals.var_t5_dn17),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign21370_e30176;
        locals.var_t6_dn0 = assign21370_e30176_d_n0;
        locals.var_t6_dn2 = assign21370_e30176_d_n2;
        locals.var_t6_dn6 = assign21370_e30176_d_n6;
        locals.var_t6_dn7 = assign21370_e30176_d_n7;
        locals.var_t6_dn10 = assign21370_e30176_d_n10;
        locals.var_t6_dn11 = assign21370_e30176_d_n11;
        locals.var_t6_dn12 = assign21370_e30176_d_n12;
        locals.var_t6_dn17 = assign21370_e30176_d_n17;
        locals.var_t6_rv = 0.0;

        let (assign21380_e30184, assign21380_e30184_d_n0, assign21380_e30184_d_n2, assign21380_e30184_d_n6, assign21380_e30184_d_n7, assign21380_e30184_d_n10, assign21380_e30184_d_n11, assign21380_e30184_d_n12, assign21380_e30184_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21380_e30180: f64 = (locals.var_t6 - 1.0);
        let assign21380_e30182: f64 = (assign21380_e30180 - locals.var_t5);
        (assign21380_e30182, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn11 - locals.var_t5_dn11), (locals.var_t6_dn12 - locals.var_t5_dn12), (locals.var_t6_dn17 - locals.var_t5_dn17),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
        locals.var_t7 = assign21380_e30184;
        locals.var_t7_dn0 = assign21380_e30184_d_n0;
        locals.var_t7_dn2 = assign21380_e30184_d_n2;
        locals.var_t7_dn6 = assign21380_e30184_d_n6;
        locals.var_t7_dn7 = assign21380_e30184_d_n7;
        locals.var_t7_dn10 = assign21380_e30184_d_n10;
        locals.var_t7_dn11 = assign21380_e30184_d_n11;
        locals.var_t7_dn12 = assign21380_e30184_d_n12;
        locals.var_t7_dn17 = assign21380_e30184_d_n17;
        locals.var_t7_rv = 0.0;

        let assign21390_e30187: f64 = if locals.var_t5 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign21390_e30187;
        locals.var_guard661_rv = 0.0;

        let (assign21400_e30197, assign21400_e30197_d_n0, assign21400_e30197_d_n2, assign21400_e30197_d_n6, assign21400_e30197_d_n7, assign21400_e30197_d_n10, assign21400_e30197_d_n11, assign21400_e30197_d_n12, assign21400_e30197_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21400_e30192: f64 = (-locals.var_cnst0soi);
        let assign21400_e30194: f64 = (locals.var_t7).sqrt();
        let assign21400_e30195: f64 = (assign21400_e30192 * assign21400_e30194);
        (assign21400_e30195, (((-locals.var_cnst0soi_dn0) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn0 / (2.0 * assign21400_e30194)))), (((-locals.var_cnst0soi_dn2) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn2 / (2.0 * assign21400_e30194)))), (((-locals.var_cnst0soi_dn6) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn6 / (2.0 * assign21400_e30194)))), (((-locals.var_cnst0soi_dn7) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn7 / (2.0 * assign21400_e30194)))), (((-locals.var_cnst0soi_dn10) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn10 / (2.0 * assign21400_e30194)))), (((-locals.var_cnst0soi_dn11) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn11 / (2.0 * assign21400_e30194)))), (((-locals.var_cnst0soi_dn12) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn12 / (2.0 * assign21400_e30194)))), (((-locals.var_cnst0soi_dn17) * assign21400_e30194) + (assign21400_e30192 * (locals.var_t7_dn17 / (2.0 * assign21400_e30194)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21400_e30197;
        locals.var_qbu_dn0 = assign21400_e30197_d_n0;
        locals.var_qbu_dn2 = assign21400_e30197_d_n2;
        locals.var_qbu_dn6 = assign21400_e30197_d_n6;
        locals.var_qbu_dn7 = assign21400_e30197_d_n7;
        locals.var_qbu_dn10 = assign21400_e30197_d_n10;
        locals.var_qbu_dn11 = assign21400_e30197_d_n11;
        locals.var_qbu_dn12 = assign21400_e30197_d_n12;
        locals.var_qbu_dn17 = assign21400_e30197_d_n17;
        locals.var_qbu_rv = 0.0;

        let assign21410_e30200: f64 = if locals.var_t5y > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign21410_e30200;
        locals.var_guard662_rv = 0.0;

        let (assign21420_e30212, assign21420_e30212_d_n0, assign21420_e30212_d_n2, assign21420_e30212_d_n6, assign21420_e30212_d_n7, assign21420_e30212_d_n10, assign21420_e30212_d_n11, assign21420_e30212_d_n12, assign21420_e30212_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign21420_e30209: f64 = (locals.var_t7).sqrt();
        let assign21420_e30210: f64 = (locals.var_cnst0soi * assign21420_e30209);
        (assign21420_e30210, ((locals.var_cnst0soi_dn0 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn0 / (2.0 * assign21420_e30209)))), ((locals.var_cnst0soi_dn2 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn2 / (2.0 * assign21420_e30209)))), ((locals.var_cnst0soi_dn6 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn6 / (2.0 * assign21420_e30209)))), ((locals.var_cnst0soi_dn7 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn7 / (2.0 * assign21420_e30209)))), ((locals.var_cnst0soi_dn10 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn10 / (2.0 * assign21420_e30209)))), ((locals.var_cnst0soi_dn11 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn11 / (2.0 * assign21420_e30209)))), ((locals.var_cnst0soi_dn12 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn12 / (2.0 * assign21420_e30209)))), ((locals.var_cnst0soi_dn17 * assign21420_e30209) + (locals.var_cnst0soi * (locals.var_t7_dn17 / (2.0 * assign21420_e30209)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21420_e30212;
        locals.var_qbu_dn0 = assign21420_e30212_d_n0;
        locals.var_qbu_dn2 = assign21420_e30212_d_n2;
        locals.var_qbu_dn6 = assign21420_e30212_d_n6;
        locals.var_qbu_dn7 = assign21420_e30212_d_n7;
        locals.var_qbu_dn10 = assign21420_e30212_d_n10;
        locals.var_qbu_dn11 = assign21420_e30212_d_n11;
        locals.var_qbu_dn12 = assign21420_e30212_d_n12;
        locals.var_qbu_dn17 = assign21420_e30212_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign21430_e30240, assign21430_e30240_d_n0, assign21430_e30240_d_n2, assign21430_e30240_d_n6, assign21430_e30240_d_n7, assign21430_e30240_d_n10, assign21430_e30240_d_n11, assign21430_e30240_d_n12, assign21430_e30240_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 == 0.0)) {
        let assign21430_e30221: f64 = (-locals.var_t5sign);
        let assign21430_e30223: f64 = (assign21430_e30221 * locals.var_t5y);
        let assign21430_e30225: f64 = (assign21430_e30223 * 0.7071067811865475);
        let assign21430_e30229: f64 = (locals.var_t5y * 0.3333333333333333);
        let assign21430_e30233: f64 = (0.25 * locals.var_t5y);
        let assign21430_e30234: f64 = (1.0 + assign21430_e30233);
        let assign21430_e30235: f64 = (assign21430_e30229 * assign21430_e30234);
        let assign21430_e30236: f64 = (1.0 + assign21430_e30235);
        let assign21430_e30237: f64 = (assign21430_e30236).sqrt();
        let assign21430_e30238: f64 = (assign21430_e30225 * assign21430_e30237);
        (assign21430_e30238, ((((assign21430_e30221 * locals.var_t5y_dn0) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn0 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn0))) / (2.0 * assign21430_e30237)))), ((((assign21430_e30221 * locals.var_t5y_dn2) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn2 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn2))) / (2.0 * assign21430_e30237)))), ((((assign21430_e30221 * locals.var_t5y_dn6) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn6 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn6))) / (2.0 * assign21430_e30237)))), ((((assign21430_e30221 * locals.var_t5y_dn7) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn7 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn7))) / (2.0 * assign21430_e30237)))), ((((assign21430_e30221 * locals.var_t5y_dn10) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn10 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn10))) / (2.0 * assign21430_e30237)))), ((((assign21430_e30221 * locals.var_t5y_dn11) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn11 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn11))) / (2.0 * assign21430_e30237)))), ((((assign21430_e30221 * locals.var_t5y_dn12) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn12 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn12))) / (2.0 * assign21430_e30237)))), ((((assign21430_e30221 * locals.var_t5y_dn17) * 0.7071067811865475) * assign21430_e30237) + (assign21430_e30225 * ((((locals.var_t5y_dn17 * 0.3333333333333333) * assign21430_e30234) + (assign21430_e30229 * (0.25 * locals.var_t5y_dn17))) / (2.0 * assign21430_e30237)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21430_e30240;
        locals.var_qbu_dn0 = assign21430_e30240_d_n0;
        locals.var_qbu_dn2 = assign21430_e30240_d_n2;
        locals.var_qbu_dn6 = assign21430_e30240_d_n6;
        locals.var_qbu_dn7 = assign21430_e30240_d_n7;
        locals.var_qbu_dn10 = assign21430_e30240_d_n10;
        locals.var_qbu_dn11 = assign21430_e30240_d_n11;
        locals.var_qbu_dn12 = assign21430_e30240_d_n12;
        locals.var_qbu_dn17 = assign21430_e30240_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign21440_e30253, assign21440_e30253_d_n0, assign21440_e30253_d_n2, assign21440_e30253_d_n6, assign21440_e30253_d_n7, assign21440_e30253_d_n10, assign21440_e30253_d_n11, assign21440_e30253_d_n12, assign21440_e30253_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21440_e30244: f64 = (locals.var_qbu * locals.var_qbu);
        let assign21440_e30247: f64 = (4.0 * 1e-6);
        let assign21440_e30249: f64 = (assign21440_e30247 * 1e-6);
        let assign21440_e30250: f64 = (assign21440_e30244 + assign21440_e30249);
        let assign21440_e30251: f64 = (assign21440_e30250).sqrt();
        (assign21440_e30251, (((locals.var_qbu_dn0 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn0)) / (2.0 * assign21440_e30251)), (((locals.var_qbu_dn2 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn2)) / (2.0 * assign21440_e30251)), (((locals.var_qbu_dn6 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn6)) / (2.0 * assign21440_e30251)), (((locals.var_qbu_dn7 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn7)) / (2.0 * assign21440_e30251)), (((locals.var_qbu_dn10 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn10)) / (2.0 * assign21440_e30251)), (((locals.var_qbu_dn11 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn11)) / (2.0 * assign21440_e30251)), (((locals.var_qbu_dn12 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn12)) / (2.0 * assign21440_e30251)), (((locals.var_qbu_dn17 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn17)) / (2.0 * assign21440_e30251)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21440_e30253;
        locals.var_tmf1_dn0 = assign21440_e30253_d_n0;
        locals.var_tmf1_dn2 = assign21440_e30253_d_n2;
        locals.var_tmf1_dn6 = assign21440_e30253_d_n6;
        locals.var_tmf1_dn7 = assign21440_e30253_d_n7;
        locals.var_tmf1_dn10 = assign21440_e30253_d_n10;
        locals.var_tmf1_dn11 = assign21440_e30253_d_n11;
        locals.var_tmf1_dn12 = assign21440_e30253_d_n12;
        locals.var_tmf1_dn17 = assign21440_e30253_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21450_e30265, assign21450_e30265_d_n0, assign21450_e30265_d_n2, assign21450_e30265_d_n6, assign21450_e30265_d_n7, assign21450_e30265_d_n10, assign21450_e30265_d_n11, assign21450_e30265_d_n12, assign21450_e30265_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21450_e30258: f64 = (locals.var_qbu + locals.var_tmf1);
        let assign21450_e30259: f64 = (0.5 * assign21450_e30258);
        let assign21450_e30262: f64 = (1e-10 * 1e-6);
        let assign21450_e30263: f64 = (assign21450_e30259 + assign21450_e30262);
        (assign21450_e30263, (0.5 * (locals.var_qbu_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_qbu_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_qbu_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_qbu_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_qbu_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_qbu_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_qbu_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_qbu_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn12, locals.var_wqbu_dn17,)
    }
};
        locals.var_wqbu = assign21450_e30265;
        locals.var_wqbu_dn0 = assign21450_e30265_d_n0;
        locals.var_wqbu_dn2 = assign21450_e30265_d_n2;
        locals.var_wqbu_dn6 = assign21450_e30265_d_n6;
        locals.var_wqbu_dn7 = assign21450_e30265_d_n7;
        locals.var_wqbu_dn10 = assign21450_e30265_d_n10;
        locals.var_wqbu_dn11 = assign21450_e30265_d_n11;
        locals.var_wqbu_dn12 = assign21450_e30265_d_n12;
        locals.var_wqbu_dn17 = assign21450_e30265_d_n17;
        locals.var_wqbu_rv = 0.0;

        let assign21460_e30268: f64 = if locals.var_wqbu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign21460_e30268;
        locals.var_guard663_rv = 0.0;

        let (assign21470_e30274, assign21470_e30274_d_n0, assign21470_e30274_d_n2, assign21470_e30274_d_n6, assign21470_e30274_d_n7, assign21470_e30274_d_n10, assign21470_e30274_d_n11, assign21470_e30274_d_n12, assign21470_e30274_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn12, locals.var_wqbu_dn17,)
    }
};
        locals.var_wqbu = assign21470_e30274;
        locals.var_wqbu_dn0 = assign21470_e30274_d_n0;
        locals.var_wqbu_dn2 = assign21470_e30274_d_n2;
        locals.var_wqbu_dn6 = assign21470_e30274_d_n6;
        locals.var_wqbu_dn7 = assign21470_e30274_d_n7;
        locals.var_wqbu_dn10 = assign21470_e30274_d_n10;
        locals.var_wqbu_dn11 = assign21470_e30274_d_n11;
        locals.var_wqbu_dn12 = assign21470_e30274_d_n12;
        locals.var_wqbu_dn17 = assign21470_e30274_d_n17;
        locals.var_wqbu_rv = 0.0;

        let (assign21480_e30282, assign21480_e30282_d_n0, assign21480_e30282_d_n2, assign21480_e30282_d_n6, assign21480_e30282_d_n7, assign21480_e30282_d_n10, assign21480_e30282_d_n11, assign21480_e30282_d_n12, assign21480_e30282_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21480_e30279: f64 = (1.6021918e-19 * locals.var_nsub);
        let assign21480_e30280: f64 = (locals.var_wqbu / assign21480_e30279);
        (assign21480_e30280, (((locals.var_wqbu_dn0 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn0))) / (assign21480_e30279 * assign21480_e30279)), (((locals.var_wqbu_dn2 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn2))) / (assign21480_e30279 * assign21480_e30279)), (((locals.var_wqbu_dn6 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn6))) / (assign21480_e30279 * assign21480_e30279)), (((locals.var_wqbu_dn7 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn7))) / (assign21480_e30279 * assign21480_e30279)), (((locals.var_wqbu_dn10 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn10))) / (assign21480_e30279 * assign21480_e30279)), (((locals.var_wqbu_dn11 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn11))) / (assign21480_e30279 * assign21480_e30279)), (((locals.var_wqbu_dn12 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn12))) / (assign21480_e30279 * assign21480_e30279)), (((locals.var_wqbu_dn17 * assign21480_e30279) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn17))) / (assign21480_e30279 * assign21480_e30279)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn12, locals.var_wdep_dn17,)
    }
};
        locals.var_wdep = assign21480_e30282;
        locals.var_wdep_dn0 = assign21480_e30282_d_n0;
        locals.var_wdep_dn2 = assign21480_e30282_d_n2;
        locals.var_wdep_dn6 = assign21480_e30282_d_n6;
        locals.var_wdep_dn7 = assign21480_e30282_d_n7;
        locals.var_wdep_dn10 = assign21480_e30282_d_n10;
        locals.var_wdep_dn11 = assign21480_e30282_d_n11;
        locals.var_wdep_dn12 = assign21480_e30282_d_n12;
        locals.var_wdep_dn17 = assign21480_e30282_d_n17;
        locals.var_wdep_rv = 0.0;

        let (assign21490_e30288, assign21490_e30288_d_n0, assign21490_e30288_d_n2, assign21490_e30288_d_n6, assign21490_e30288_d_n7, assign21490_e30288_d_n10, assign21490_e30288_d_n11, assign21490_e30288_d_n12, assign21490_e30288_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21490_e30286: f64 = (locals.var_wdep - locals.var_wk_xj);
        (assign21490_e30286, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn12, locals.var_wdep_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21490_e30288;
        locals.var_t1_dn0 = assign21490_e30288_d_n0;
        locals.var_t1_dn2 = assign21490_e30288_d_n2;
        locals.var_t1_dn6 = assign21490_e30288_d_n6;
        locals.var_t1_dn7 = assign21490_e30288_d_n7;
        locals.var_t1_dn10 = assign21490_e30288_d_n10;
        locals.var_t1_dn11 = assign21490_e30288_d_n11;
        locals.var_t1_dn12 = assign21490_e30288_d_n12;
        locals.var_t1_dn17 = assign21490_e30288_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21500_e30294, assign21500_e30294_d_n0, assign21500_e30294_d_n2, assign21500_e30294_d_n6, assign21500_e30294_d_n7, assign21500_e30294_d_n10, assign21500_e30294_d_n11, assign21500_e30294_d_n12, assign21500_e30294_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21500_e30292: f64 = (locals.var_wdep * 0.01);
        (assign21500_e30292, (locals.var_wdep_dn0 * 0.01), (locals.var_wdep_dn2 * 0.01), (locals.var_wdep_dn6 * 0.01), (locals.var_wdep_dn7 * 0.01), (locals.var_wdep_dn10 * 0.01), (locals.var_wdep_dn11 * 0.01), (locals.var_wdep_dn12 * 0.01), (locals.var_wdep_dn17 * 0.01),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn12, locals.var_delta_dn17,)
    }
};
        locals.var_delta = assign21500_e30294;
        locals.var_delta_dn0 = assign21500_e30294_d_n0;
        locals.var_delta_dn2 = assign21500_e30294_d_n2;
        locals.var_delta_dn6 = assign21500_e30294_d_n6;
        locals.var_delta_dn7 = assign21500_e30294_d_n7;
        locals.var_delta_dn10 = assign21500_e30294_d_n10;
        locals.var_delta_dn11 = assign21500_e30294_d_n11;
        locals.var_delta_dn12 = assign21500_e30294_d_n12;
        locals.var_delta_dn17 = assign21500_e30294_d_n17;
        locals.var_delta_rv = 0.0;

        let (assign21510_e30307, assign21510_e30307_d_n0, assign21510_e30307_d_n2, assign21510_e30307_d_n6, assign21510_e30307_d_n7, assign21510_e30307_d_n10, assign21510_e30307_d_n11, assign21510_e30307_d_n12, assign21510_e30307_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21510_e30298: f64 = (locals.var_t1 * locals.var_t1);
        let assign21510_e30301: f64 = (4.0 * locals.var_delta);
        let assign21510_e30303: f64 = (assign21510_e30301 * locals.var_delta);
        let assign21510_e30304: f64 = (assign21510_e30298 + assign21510_e30303);
        let assign21510_e30305: f64 = (assign21510_e30304).sqrt();
        (assign21510_e30305, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn0))) / (2.0 * assign21510_e30305)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn2))) / (2.0 * assign21510_e30305)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn6))) / (2.0 * assign21510_e30305)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn7))) / (2.0 * assign21510_e30305)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn10))) / (2.0 * assign21510_e30305)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn11))) / (2.0 * assign21510_e30305)), ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + (((4.0 * locals.var_delta_dn12) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn12))) / (2.0 * assign21510_e30305)), ((((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) + (((4.0 * locals.var_delta_dn17) * locals.var_delta) + (assign21510_e30301 * locals.var_delta_dn17))) / (2.0 * assign21510_e30305)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21510_e30307;
        locals.var_tmf1_dn0 = assign21510_e30307_d_n0;
        locals.var_tmf1_dn2 = assign21510_e30307_d_n2;
        locals.var_tmf1_dn6 = assign21510_e30307_d_n6;
        locals.var_tmf1_dn7 = assign21510_e30307_d_n7;
        locals.var_tmf1_dn10 = assign21510_e30307_d_n10;
        locals.var_tmf1_dn11 = assign21510_e30307_d_n11;
        locals.var_tmf1_dn12 = assign21510_e30307_d_n12;
        locals.var_tmf1_dn17 = assign21510_e30307_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21520_e30319, assign21520_e30319_d_n0, assign21520_e30319_d_n2, assign21520_e30319_d_n6, assign21520_e30319_d_n7, assign21520_e30319_d_n10, assign21520_e30319_d_n11, assign21520_e30319_d_n12, assign21520_e30319_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21520_e30312: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign21520_e30313: f64 = (0.5 * assign21520_e30312);
        let assign21520_e30316: f64 = (1e-10 * locals.var_delta);
        let assign21520_e30317: f64 = (assign21520_e30313 + assign21520_e30316);
        (assign21520_e30317, ((0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0)) + (1e-10 * locals.var_delta_dn0)), ((0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2)) + (1e-10 * locals.var_delta_dn2)), ((0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6)) + (1e-10 * locals.var_delta_dn6)), ((0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7)) + (1e-10 * locals.var_delta_dn7)), ((0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10)) + (1e-10 * locals.var_delta_dn10)), ((0.5 * (locals.var_t1_dn11 + locals.var_tmf1_dn11)) + (1e-10 * locals.var_delta_dn11)), ((0.5 * (locals.var_t1_dn12 + locals.var_tmf1_dn12)) + (1e-10 * locals.var_delta_dn12)), ((0.5 * (locals.var_t1_dn17 + locals.var_tmf1_dn17)) + (1e-10 * locals.var_delta_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21520_e30319;
        locals.var_t2_dn0 = assign21520_e30319_d_n0;
        locals.var_t2_dn2 = assign21520_e30319_d_n2;
        locals.var_t2_dn6 = assign21520_e30319_d_n6;
        locals.var_t2_dn7 = assign21520_e30319_d_n7;
        locals.var_t2_dn10 = assign21520_e30319_d_n10;
        locals.var_t2_dn11 = assign21520_e30319_d_n11;
        locals.var_t2_dn12 = assign21520_e30319_d_n12;
        locals.var_t2_dn17 = assign21520_e30319_d_n17;
        locals.var_t2_rv = 0.0;

        let assign21530_e30322: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign21530_e30322;
        locals.var_guard664_rv = 0.0;

        let (assign21540_e30328, assign21540_e30328_d_n0, assign21540_e30328_d_n2, assign21540_e30328_d_n6, assign21540_e30328_d_n7, assign21540_e30328_d_n10, assign21540_e30328_d_n11, assign21540_e30328_d_n12, assign21540_e30328_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard664 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21540_e30328;
        locals.var_t2_dn0 = assign21540_e30328_d_n0;
        locals.var_t2_dn2 = assign21540_e30328_d_n2;
        locals.var_t2_dn6 = assign21540_e30328_d_n6;
        locals.var_t2_dn7 = assign21540_e30328_d_n7;
        locals.var_t2_dn10 = assign21540_e30328_d_n10;
        locals.var_t2_dn11 = assign21540_e30328_d_n11;
        locals.var_t2_dn12 = assign21540_e30328_d_n12;
        locals.var_t2_dn17 = assign21540_e30328_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign21550_e30338, assign21550_e30338_d_n0, assign21550_e30338_d_n2, assign21550_e30338_d_n6, assign21550_e30338_d_n7, assign21550_e30338_d_n10, assign21550_e30338_d_n11, assign21550_e30338_d_n12, assign21550_e30338_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21550_e30332: f64 = (locals.var_t2 / locals.var_wdep);
        let assign21550_e30334: f64 = (assign21550_e30332 * locals.var_t2);
        let assign21550_e30336: f64 = (assign21550_e30334 / locals.var_wdep);
        (assign21550_e30336, ((((((((locals.var_t2_dn0 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn0)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn0)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn0)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn2 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn2)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn2)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn2)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn6 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn6)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn6)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn6)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn7 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn7)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn7)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn7)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn10 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn10)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn10)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn10)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn11 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn11)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn11)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn11)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn12 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn12)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn12)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn12)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn17 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn17)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21550_e30332 * locals.var_t2_dn17)) * locals.var_wdep) - (assign21550_e30334 * locals.var_wdep_dn17)) / (locals.var_wdep * locals.var_wdep)),)
    } else {
        (locals.var_wfactor, locals.var_wfactor_dn0, locals.var_wfactor_dn2, locals.var_wfactor_dn6, locals.var_wfactor_dn7, locals.var_wfactor_dn10, locals.var_wfactor_dn11, locals.var_wfactor_dn12, locals.var_wfactor_dn17,)
    }
};
        locals.var_wfactor = assign21550_e30338;
        locals.var_wfactor_dn0 = assign21550_e30338_d_n0;
        locals.var_wfactor_dn2 = assign21550_e30338_d_n2;
        locals.var_wfactor_dn6 = assign21550_e30338_d_n6;
        locals.var_wfactor_dn7 = assign21550_e30338_d_n7;
        locals.var_wfactor_dn10 = assign21550_e30338_d_n10;
        locals.var_wfactor_dn11 = assign21550_e30338_d_n11;
        locals.var_wfactor_dn12 = assign21550_e30338_d_n12;
        locals.var_wfactor_dn17 = assign21550_e30338_d_n17;
        locals.var_wfactor_rv = 0.0;

        let (assign21560_e30348, assign21560_e30348_d_n0, assign21560_e30348_d_n2, assign21560_e30348_d_n6, assign21560_e30348_d_n7, assign21560_e30348_d_n10, assign21560_e30348_d_n11, assign21560_e30348_d_n12, assign21560_e30348_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21560_e30342: f64 = (locals.var_ps0__blk610 - locals.var_dphi_vds);
        let assign21560_e30344: f64 = (assign21560_e30342 * locals.var_wfactor);
        let assign21560_e30346: f64 = (assign21560_e30344 + locals.var_dphi_vds);
        (assign21560_e30346, ((((locals.var_ps0__blk610_dn0 - locals.var_dphi_vds_dn0) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn0)) + locals.var_dphi_vds_dn0), ((((locals.var_ps0__blk610_dn2 - locals.var_dphi_vds_dn2) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn2)) + locals.var_dphi_vds_dn2), ((((locals.var_ps0__blk610_dn6 - locals.var_dphi_vds_dn6) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn6)) + locals.var_dphi_vds_dn6), ((((locals.var_ps0__blk610_dn7 - locals.var_dphi_vds_dn7) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn7)) + locals.var_dphi_vds_dn7), ((((locals.var_ps0__blk610_dn10 - locals.var_dphi_vds_dn10) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn10)) + locals.var_dphi_vds_dn10), ((((locals.var_ps0__blk610_dn11 - locals.var_dphi_vds_dn11) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn11)) + locals.var_dphi_vds_dn11), ((((locals.var_ps0__blk610_dn12 - locals.var_dphi_vds_dn12) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn12)) + locals.var_dphi_vds_dn12), ((((locals.var_ps0__blk610_dn17 - locals.var_dphi_vds_dn17) * locals.var_wfactor) + (assign21560_e30342 * locals.var_wfactor_dn17)) + locals.var_dphi_vds_dn17),)
    } else {
        (locals.var_phim, locals.var_phim_dn0, locals.var_phim_dn2, locals.var_phim_dn6, locals.var_phim_dn7, locals.var_phim_dn10, locals.var_phim_dn11, locals.var_phim_dn12, locals.var_phim_dn17,)
    }
};
        locals.var_phim = assign21560_e30348;
        locals.var_phim_dn0 = assign21560_e30348_d_n0;
        locals.var_phim_dn2 = assign21560_e30348_d_n2;
        locals.var_phim_dn6 = assign21560_e30348_d_n6;
        locals.var_phim_dn7 = assign21560_e30348_d_n7;
        locals.var_phim_dn10 = assign21560_e30348_d_n10;
        locals.var_phim_dn11 = assign21560_e30348_d_n11;
        locals.var_phim_dn12 = assign21560_e30348_d_n12;
        locals.var_phim_dn17 = assign21560_e30348_d_n17;
        locals.var_phim_rv = 0.0;

        let (assign21570_e30362, assign21570_e30362_d_n0, assign21570_e30362_d_n2, assign21570_e30362_d_n6, assign21570_e30362_d_n7, assign21570_e30362_d_n10, assign21570_e30362_d_n11, assign21570_e30362_d_n12, assign21570_e30362_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21570_e30352: f64 = (locals.var_beta * locals.var_phim);
        let assign21570_e30353: f64 = (assign21570_e30352).exp();
        let assign21570_e30357: f64 = (locals.var_phim - locals.var_vds);
        let assign21570_e30358: f64 = (locals.var_beta * assign21570_e30357);
        let assign21570_e30359: f64 = (assign21570_e30358).exp();
        let assign21570_e30360: f64 = (assign21570_e30353 - assign21570_e30359);
        (assign21570_e30360, ((assign21570_e30353 * (locals.var_beta * locals.var_phim_dn0)) - (assign21570_e30359 * (locals.var_beta * (locals.var_phim_dn0 - locals.var_vds_dn0)))), ((assign21570_e30353 * (locals.var_beta * locals.var_phim_dn2)) - (assign21570_e30359 * (locals.var_beta * (locals.var_phim_dn2 - locals.var_vds_dn2)))), ((assign21570_e30353 * (locals.var_beta * locals.var_phim_dn6)) - (assign21570_e30359 * (locals.var_beta * (locals.var_phim_dn6 - locals.var_vds_dn6)))), ((assign21570_e30353 * (locals.var_beta * locals.var_phim_dn7)) - (assign21570_e30359 * (locals.var_beta * (locals.var_phim_dn7 - locals.var_vds_dn7)))), ((assign21570_e30353 * ((locals.var_beta_dn10 * locals.var_phim) + (locals.var_beta * locals.var_phim_dn10))) - (assign21570_e30359 * ((locals.var_beta_dn10 * assign21570_e30357) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_vds_dn10))))), ((assign21570_e30353 * (locals.var_beta * locals.var_phim_dn11)) - (assign21570_e30359 * (locals.var_beta * (locals.var_phim_dn11 - locals.var_vds_dn11)))), ((assign21570_e30353 * (locals.var_beta * locals.var_phim_dn12)) - (assign21570_e30359 * (locals.var_beta * (locals.var_phim_dn12 - locals.var_vds_dn12)))), ((assign21570_e30353 * (locals.var_beta * locals.var_phim_dn17)) - (assign21570_e30359 * (locals.var_beta * (locals.var_phim_dn17 - locals.var_vds_dn17)))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign21570_e30362;
        locals.var_ty_dn0 = assign21570_e30362_d_n0;
        locals.var_ty_dn2 = assign21570_e30362_d_n2;
        locals.var_ty_dn6 = assign21570_e30362_d_n6;
        locals.var_ty_dn7 = assign21570_e30362_d_n7;
        locals.var_ty_dn10 = assign21570_e30362_d_n10;
        locals.var_ty_dn11 = assign21570_e30362_d_n11;
        locals.var_ty_dn12 = assign21570_e30362_d_n12;
        locals.var_ty_dn17 = assign21570_e30362_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign21580_e30373,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21580_e30366: f64 = (2.0 * 1.6021918e-19);
        let assign21580_e30368: f64 = (assign21580_e30366 * locals.var_uc_wk_njunc);
        let assign21580_e30370: f64 = (assign21580_e30368 * 1.034943e-10);
        let assign21580_e30371: f64 = (assign21580_e30370).sqrt();
        (assign21580_e30371,)
    } else {
        (locals.var_conpt00,)
    }
};
        locals.var_conpt00 = assign21580_e30373;
        locals.var_conpt00_rv = 0.0;

        let (assign21590_e30380, assign21590_e30380_d_n10,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21590_e30377: f64 = (locals.var_beta_inv).sqrt();
        let assign21590_e30378: f64 = (locals.var_conpt00 * assign21590_e30377);
        (assign21590_e30378, (locals.var_conpt00 * (locals.var_beta_inv_dn10 / (2.0 * assign21590_e30377))),)
    } else {
        (locals.var_conpt0, locals.var_conpt0_dn10,)
    }
};
        locals.var_conpt0 = assign21590_e30380;
        locals.var_conpt0_dn10 = assign21590_e30380_d_n10;
        locals.var_conpt0_rv = 0.0;

        let (assign21600_e30388, assign21600_e30388_d_n0, assign21600_e30388_d_n2, assign21600_e30388_d_n6, assign21600_e30388_d_n7, assign21600_e30388_d_n10, assign21600_e30388_d_n11, assign21600_e30388_d_n12, assign21600_e30388_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21600_e30385: f64 = (locals.var_phim - locals.var_dphi_vds);
        let assign21600_e30386: f64 = (locals.var_beta * assign21600_e30385);
        (assign21600_e30386, (locals.var_beta * (locals.var_phim_dn0 - locals.var_dphi_vds_dn0)), (locals.var_beta * (locals.var_phim_dn2 - locals.var_dphi_vds_dn2)), (locals.var_beta * (locals.var_phim_dn6 - locals.var_dphi_vds_dn6)), (locals.var_beta * (locals.var_phim_dn7 - locals.var_dphi_vds_dn7)), ((locals.var_beta_dn10 * assign21600_e30385) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_dphi_vds_dn10))), (locals.var_beta * (locals.var_phim_dn11 - locals.var_dphi_vds_dn11)), (locals.var_beta * (locals.var_phim_dn12 - locals.var_dphi_vds_dn12)), (locals.var_beta * (locals.var_phim_dn17 - locals.var_dphi_vds_dn17)),)
    } else {
        (locals.var_t1w__blk609, locals.var_t1w__blk609_dn0, locals.var_t1w__blk609_dn2, locals.var_t1w__blk609_dn6, locals.var_t1w__blk609_dn7, locals.var_t1w__blk609_dn10, locals.var_t1w__blk609_dn11, locals.var_t1w__blk609_dn12, locals.var_t1w__blk609_dn17,)
    }
};
        locals.var_t1w__blk609 = assign21600_e30388;
        locals.var_t1w__blk609_dn0 = assign21600_e30388_d_n0;
        locals.var_t1w__blk609_dn2 = assign21600_e30388_d_n2;
        locals.var_t1w__blk609_dn6 = assign21600_e30388_d_n6;
        locals.var_t1w__blk609_dn7 = assign21600_e30388_d_n7;
        locals.var_t1w__blk609_dn10 = assign21600_e30388_d_n10;
        locals.var_t1w__blk609_dn11 = assign21600_e30388_d_n11;
        locals.var_t1w__blk609_dn12 = assign21600_e30388_d_n12;
        locals.var_t1w__blk609_dn17 = assign21600_e30388_d_n17;
        locals.var_t1w__blk609_rv = 0.0;

        let assign21610_e30393: f64 = (0.2 * locals.var_beta);
        let assign21610_e30394: f64 = assign21610_e30393;
        let assign21610_e30398: f64 = (0.2 * locals.var_beta);
        let assign21610_e30401: f64 = if ((locals.var_t1w__blk609 < assign21610_e30394) && (assign21610_e30398 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard665 = assign21610_e30401;
        locals.var_guard665_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        locals: &mut StampLocals,
    ) {
        let (assign21620_e30413, assign21620_e30413_d_n0, assign21620_e30413_d_n2, assign21620_e30413_d_n6, assign21620_e30413_d_n7, assign21620_e30413_d_n10, assign21620_e30413_d_n11, assign21620_e30413_d_n12, assign21620_e30413_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21620_e30408: f64 = (0.2 * locals.var_beta);
        let assign21620_e30409: f64 = assign21620_e30408;
        let assign21620_e30411: f64 = (assign21620_e30409 - locals.var_t1w__blk609);
        (assign21620_e30411, (-locals.var_t1w__blk609_dn0), (-locals.var_t1w__blk609_dn2), (-locals.var_t1w__blk609_dn6), (-locals.var_t1w__blk609_dn7), ((0.2 * locals.var_beta_dn10) - locals.var_t1w__blk609_dn10), (-locals.var_t1w__blk609_dn11), (-locals.var_t1w__blk609_dn12), (-locals.var_t1w__blk609_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21620_e30413;
        locals.var_tmf1_dn0 = assign21620_e30413_d_n0;
        locals.var_tmf1_dn2 = assign21620_e30413_d_n2;
        locals.var_tmf1_dn6 = assign21620_e30413_d_n6;
        locals.var_tmf1_dn7 = assign21620_e30413_d_n7;
        locals.var_tmf1_dn10 = assign21620_e30413_d_n10;
        locals.var_tmf1_dn11 = assign21620_e30413_d_n11;
        locals.var_tmf1_dn12 = assign21620_e30413_d_n12;
        locals.var_tmf1_dn17 = assign21620_e30413_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21630_e30421, assign21630_e30421_d_n0, assign21630_e30421_d_n2, assign21630_e30421_d_n6, assign21630_e30421_d_n7, assign21630_e30421_d_n10, assign21630_e30421_d_n11, assign21630_e30421_d_n12, assign21630_e30421_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21630_e30419: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign21630_e30419, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign21630_e30421;
        locals.var_x2_dn0 = assign21630_e30421_d_n0;
        locals.var_x2_dn2 = assign21630_e30421_d_n2;
        locals.var_x2_dn6 = assign21630_e30421_d_n6;
        locals.var_x2_dn7 = assign21630_e30421_d_n7;
        locals.var_x2_dn10 = assign21630_e30421_d_n10;
        locals.var_x2_dn11 = assign21630_e30421_d_n11;
        locals.var_x2_dn12 = assign21630_e30421_d_n12;
        locals.var_x2_dn17 = assign21630_e30421_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign21640_e30433, assign21640_e30433_d_n0, assign21640_e30433_d_n2, assign21640_e30433_d_n6, assign21640_e30433_d_n7, assign21640_e30433_d_n10, assign21640_e30433_d_n11, assign21640_e30433_d_n12, assign21640_e30433_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21640_e30427: f64 = (0.2 * locals.var_beta);
        let assign21640_e30430: f64 = (0.2 * locals.var_beta);
        let assign21640_e30431: f64 = (assign21640_e30427 * assign21640_e30430);
        (assign21640_e30431, 0.0, 0.0, 0.0, 0.0, (((0.2 * locals.var_beta_dn10) * assign21640_e30430) + (assign21640_e30427 * (0.2 * locals.var_beta_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign21640_e30433;
        locals.var_xmax2_dn0 = assign21640_e30433_d_n0;
        locals.var_xmax2_dn2 = assign21640_e30433_d_n2;
        locals.var_xmax2_dn6 = assign21640_e30433_d_n6;
        locals.var_xmax2_dn7 = assign21640_e30433_d_n7;
        locals.var_xmax2_dn10 = assign21640_e30433_d_n10;
        locals.var_xmax2_dn11 = assign21640_e30433_d_n11;
        locals.var_xmax2_dn12 = assign21640_e30433_d_n12;
        locals.var_xmax2_dn17 = assign21640_e30433_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign21650_e30439, assign21650_e30439_d_n0, assign21650_e30439_d_n2, assign21650_e30439_d_n6, assign21650_e30439_d_n7, assign21650_e30439_d_n10, assign21650_e30439_d_n11, assign21650_e30439_d_n12, assign21650_e30439_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign21650_e30439;
        locals.var_xp_dn0 = assign21650_e30439_d_n0;
        locals.var_xp_dn2 = assign21650_e30439_d_n2;
        locals.var_xp_dn6 = assign21650_e30439_d_n6;
        locals.var_xp_dn7 = assign21650_e30439_d_n7;
        locals.var_xp_dn10 = assign21650_e30439_d_n10;
        locals.var_xp_dn11 = assign21650_e30439_d_n11;
        locals.var_xp_dn12 = assign21650_e30439_d_n12;
        locals.var_xp_dn17 = assign21650_e30439_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign21660_e30445, assign21660_e30445_d_n0, assign21660_e30445_d_n2, assign21660_e30445_d_n6, assign21660_e30445_d_n7, assign21660_e30445_d_n10, assign21660_e30445_d_n11, assign21660_e30445_d_n12, assign21660_e30445_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign21660_e30445;
        locals.var_xmp_dn0 = assign21660_e30445_d_n0;
        locals.var_xmp_dn2 = assign21660_e30445_d_n2;
        locals.var_xmp_dn6 = assign21660_e30445_d_n6;
        locals.var_xmp_dn7 = assign21660_e30445_d_n7;
        locals.var_xmp_dn10 = assign21660_e30445_d_n10;
        locals.var_xmp_dn11 = assign21660_e30445_d_n11;
        locals.var_xmp_dn12 = assign21660_e30445_d_n12;
        locals.var_xmp_dn17 = assign21660_e30445_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign21670_e30451,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign21670_e30451;
        locals.var_m0_rv = 0.0;

        let (assign21680_e30457,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21680_e30457;
        locals.var_mm_rv = 0.0;

        let (assign21690_e30463, assign21690_e30463_d_n0, assign21690_e30463_d_n2, assign21690_e30463_d_n6, assign21690_e30463_d_n7, assign21690_e30463_d_n10, assign21690_e30463_d_n11, assign21690_e30463_d_n12, assign21690_e30463_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign21690_e30463;
        locals.var_arg_dn0 = assign21690_e30463_d_n0;
        locals.var_arg_dn2 = assign21690_e30463_d_n2;
        locals.var_arg_dn6 = assign21690_e30463_d_n6;
        locals.var_arg_dn7 = assign21690_e30463_d_n7;
        locals.var_arg_dn10 = assign21690_e30463_d_n10;
        locals.var_arg_dn11 = assign21690_e30463_d_n11;
        locals.var_arg_dn12 = assign21690_e30463_d_n12;
        locals.var_arg_dn17 = assign21690_e30463_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign21700_e30469, assign21700_e30469_d_n0, assign21700_e30469_d_n2, assign21700_e30469_d_n6, assign21700_e30469_d_n7, assign21700_e30469_d_n10, assign21700_e30469_d_n11, assign21700_e30469_d_n12, assign21700_e30469_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21700_e30469;
        locals.var_dnm_dn0 = assign21700_e30469_d_n0;
        locals.var_dnm_dn2 = assign21700_e30469_d_n2;
        locals.var_dnm_dn6 = assign21700_e30469_d_n6;
        locals.var_dnm_dn7 = assign21700_e30469_d_n7;
        locals.var_dnm_dn10 = assign21700_e30469_d_n10;
        locals.var_dnm_dn11 = assign21700_e30469_d_n11;
        locals.var_dnm_dn12 = assign21700_e30469_d_n12;
        locals.var_dnm_dn17 = assign21700_e30469_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21710_e30477, assign21710_e30477_d_n0, assign21710_e30477_d_n2, assign21710_e30477_d_n6, assign21710_e30477_d_n7, assign21710_e30477_d_n10, assign21710_e30477_d_n11, assign21710_e30477_d_n12, assign21710_e30477_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21710_e30475: f64 = (locals.var_xp * locals.var_x2);
        (assign21710_e30475, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign21710_e30477;
        locals.var_xp_dn0 = assign21710_e30477_d_n0;
        locals.var_xp_dn2 = assign21710_e30477_d_n2;
        locals.var_xp_dn6 = assign21710_e30477_d_n6;
        locals.var_xp_dn7 = assign21710_e30477_d_n7;
        locals.var_xp_dn10 = assign21710_e30477_d_n10;
        locals.var_xp_dn11 = assign21710_e30477_d_n11;
        locals.var_xp_dn12 = assign21710_e30477_d_n12;
        locals.var_xp_dn17 = assign21710_e30477_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign21720_e30485, assign21720_e30485_d_n0, assign21720_e30485_d_n2, assign21720_e30485_d_n6, assign21720_e30485_d_n7, assign21720_e30485_d_n10, assign21720_e30485_d_n11, assign21720_e30485_d_n12, assign21720_e30485_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21720_e30483: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign21720_e30483, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign21720_e30485;
        locals.var_xmp_dn0 = assign21720_e30485_d_n0;
        locals.var_xmp_dn2 = assign21720_e30485_d_n2;
        locals.var_xmp_dn6 = assign21720_e30485_d_n6;
        locals.var_xmp_dn7 = assign21720_e30485_d_n7;
        locals.var_xmp_dn10 = assign21720_e30485_d_n10;
        locals.var_xmp_dn11 = assign21720_e30485_d_n11;
        locals.var_xmp_dn12 = assign21720_e30485_d_n12;
        locals.var_xmp_dn17 = assign21720_e30485_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign21730_e30493, assign21730_e30493_d_n0, assign21730_e30493_d_n2, assign21730_e30493_d_n6, assign21730_e30493_d_n7, assign21730_e30493_d_n10, assign21730_e30493_d_n11, assign21730_e30493_d_n12, assign21730_e30493_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21730_e30491: f64 = (locals.var_xp + locals.var_xmp);
        (assign21730_e30491, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign21730_e30493;
        locals.var_arg_dn0 = assign21730_e30493_d_n0;
        locals.var_arg_dn2 = assign21730_e30493_d_n2;
        locals.var_arg_dn6 = assign21730_e30493_d_n6;
        locals.var_arg_dn7 = assign21730_e30493_d_n7;
        locals.var_arg_dn10 = assign21730_e30493_d_n10;
        locals.var_arg_dn11 = assign21730_e30493_d_n11;
        locals.var_arg_dn12 = assign21730_e30493_d_n12;
        locals.var_arg_dn17 = assign21730_e30493_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign21740_e30499, assign21740_e30499_d_n0, assign21740_e30499_d_n2, assign21740_e30499_d_n6, assign21740_e30499_d_n7, assign21740_e30499_d_n10, assign21740_e30499_d_n11, assign21740_e30499_d_n12, assign21740_e30499_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21740_e30499;
        locals.var_dnm_dn0 = assign21740_e30499_d_n0;
        locals.var_dnm_dn2 = assign21740_e30499_d_n2;
        locals.var_dnm_dn6 = assign21740_e30499_d_n6;
        locals.var_dnm_dn7 = assign21740_e30499_d_n7;
        locals.var_dnm_dn10 = assign21740_e30499_d_n10;
        locals.var_dnm_dn11 = assign21740_e30499_d_n11;
        locals.var_dnm_dn12 = assign21740_e30499_d_n12;
        locals.var_dnm_dn17 = assign21740_e30499_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign21750_e30514: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard666 = assign21750_e30514;
        locals.var_guard666_rv = 0.0;

        let assign21760_e30517: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign21760_e30517;
        locals.var_guard667_rv = 0.0;

        let (assign21770_e30527,) = {
    if ((((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21770_e30527;
        locals.var_mm_rv = 0.0;

        let assign21780_e30530: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign21780_e30530;
        locals.var_guard668_rv = 0.0;

        let (assign21790_e30543,) = {
    if (((((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21790_e30543;
        locals.var_mm_rv = 0.0;

        let assign21800_e30546: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign21800_e30546;
        locals.var_guard669_rv = 0.0;

        let (assign21810_e30562,) = {
    if ((((((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 == 0.0)) && (locals.var_guard669 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21810_e30562;
        locals.var_mm_rv = 0.0;

        let assign21820_e30565: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign21820_e30565;
        locals.var_guard670_rv = 0.0;

        let (assign21830_e30584,) = {
    if (((((((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 == 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard670 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21830_e30584;
        locals.var_mm_rv = 0.0;

        let (assign21840_e30592,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign21840_e30592;
        locals.var_m0_rv = 0.0;

        let mut assign21850_loop_guard: usize = 0;
        while {
            let assign21850_cond_e30601: f64 = if ((((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign21850_cond_e30601 != 0.0
        } {
            assign21850_loop_guard += 1;
            assert!(assign21850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign21850_body0_e30610, assign21850_body0_e30610_d_n0, assign21850_body0_e30610_d_n2, assign21850_body0_e30610_d_n6, assign21850_body0_e30610_d_n7, assign21850_body0_e30610_d_n10, assign21850_body0_e30610_d_n11, assign21850_body0_e30610_d_n12, assign21850_body0_e30610_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        let assign21850_body0_e30608: f64 = (locals.var_dnm).sqrt();
        (assign21850_body0_e30608, (locals.var_dnm_dn0 / (2.0 * assign21850_body0_e30608)), (locals.var_dnm_dn2 / (2.0 * assign21850_body0_e30608)), (locals.var_dnm_dn6 / (2.0 * assign21850_body0_e30608)), (locals.var_dnm_dn7 / (2.0 * assign21850_body0_e30608)), (locals.var_dnm_dn10 / (2.0 * assign21850_body0_e30608)), (locals.var_dnm_dn11 / (2.0 * assign21850_body0_e30608)), (locals.var_dnm_dn12 / (2.0 * assign21850_body0_e30608)), (locals.var_dnm_dn17 / (2.0 * assign21850_body0_e30608)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign21850_body0_e30610;
            locals.var_dnm_dn0 = assign21850_body0_e30610_d_n0;
            locals.var_dnm_dn2 = assign21850_body0_e30610_d_n2;
            locals.var_dnm_dn6 = assign21850_body0_e30610_d_n6;
            locals.var_dnm_dn7 = assign21850_body0_e30610_d_n7;
            locals.var_dnm_dn10 = assign21850_body0_e30610_d_n10;
            locals.var_dnm_dn11 = assign21850_body0_e30610_d_n11;
            locals.var_dnm_dn12 = assign21850_body0_e30610_d_n12;
            locals.var_dnm_dn17 = assign21850_body0_e30610_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign21850_body1_e30620,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        let assign21850_body1_e30618: f64 = (locals.var_m0 + 1.0);
        (assign21850_body1_e30618,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign21850_body1_e30620;
            locals.var_m0_rv = 0.0;
        }

        let (assign21860_e30635, assign21860_e30635_d_n0, assign21860_e30635_d_n2, assign21860_e30635_d_n6, assign21860_e30635_d_n7, assign21860_e30635_d_n10, assign21860_e30635_d_n11, assign21860_e30635_d_n12, assign21860_e30635_d_n17,) = {
    if (((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 == 0.0)) {
        let assign21860_e30631: f64 = 2.0;
        let assign21860_e30632: f64 = (1.0 / assign21860_e30631);
        let assign21860_e30633: f64 = (locals.var_dnm).powf(assign21860_e30632);
        (assign21860_e30633, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn0)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn2)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn6)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn7)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn10)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn11)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn12)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21860_e30632) as f64).is_finite() && ((assign21860_e30632) as f64).fract() == 0.0 { if assign21860_e30632 == 0.0 { 0.0 } else { (assign21860_e30632 * ((locals.var_dnm).powf(assign21860_e30632 - 1.0) * locals.var_dnm_dn17)) } } else { (assign21860_e30633 * (assign21860_e30632 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21860_e30635;
        locals.var_dnm_dn0 = assign21860_e30635_d_n0;
        locals.var_dnm_dn2 = assign21860_e30635_d_n2;
        locals.var_dnm_dn6 = assign21860_e30635_d_n6;
        locals.var_dnm_dn7 = assign21860_e30635_d_n7;
        locals.var_dnm_dn10 = assign21860_e30635_d_n10;
        locals.var_dnm_dn11 = assign21860_e30635_d_n11;
        locals.var_dnm_dn12 = assign21860_e30635_d_n12;
        locals.var_dnm_dn17 = assign21860_e30635_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21870_e30643, assign21870_e30643_d_n0, assign21870_e30643_d_n2, assign21870_e30643_d_n6, assign21870_e30643_d_n7, assign21870_e30643_d_n10, assign21870_e30643_d_n11, assign21870_e30643_d_n12, assign21870_e30643_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21870_e30641: f64 = (1.0 / locals.var_dnm);
        (assign21870_e30641, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21870_e30643;
        locals.var_dnm_dn0 = assign21870_e30643_d_n0;
        locals.var_dnm_dn2 = assign21870_e30643_d_n2;
        locals.var_dnm_dn6 = assign21870_e30643_d_n6;
        locals.var_dnm_dn7 = assign21870_e30643_d_n7;
        locals.var_dnm_dn10 = assign21870_e30643_d_n10;
        locals.var_dnm_dn11 = assign21870_e30643_d_n11;
        locals.var_dnm_dn12 = assign21870_e30643_d_n12;
        locals.var_dnm_dn17 = assign21870_e30643_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21880_e30655, assign21880_e30655_d_n0, assign21880_e30655_d_n2, assign21880_e30655_d_n6, assign21880_e30655_d_n7, assign21880_e30655_d_n10, assign21880_e30655_d_n11, assign21880_e30655_d_n12, assign21880_e30655_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21880_e30650: f64 = (0.2 * locals.var_beta);
        let assign21880_e30651: f64 = (locals.var_tmf1 * assign21880_e30650);
        let assign21880_e30653: f64 = (assign21880_e30651 * locals.var_dnm);
        (assign21880_e30653, (((locals.var_tmf1_dn0 * assign21880_e30650) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign21880_e30650) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * assign21880_e30650) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign21880_e30650) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign21880_e30650) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn10))) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign21880_e30650) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * assign21880_e30650) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * assign21880_e30650) * locals.var_dnm) + (assign21880_e30651 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign21880_e30655;
        locals.var_tmf0_dn0 = assign21880_e30655_d_n0;
        locals.var_tmf0_dn2 = assign21880_e30655_d_n2;
        locals.var_tmf0_dn6 = assign21880_e30655_d_n6;
        locals.var_tmf0_dn7 = assign21880_e30655_d_n7;
        locals.var_tmf0_dn10 = assign21880_e30655_d_n10;
        locals.var_tmf0_dn11 = assign21880_e30655_d_n11;
        locals.var_tmf0_dn12 = assign21880_e30655_d_n12;
        locals.var_tmf0_dn17 = assign21880_e30655_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign21890_e30667, assign21890_e30667_d_n0, assign21890_e30667_d_n2, assign21890_e30667_d_n6, assign21890_e30667_d_n7, assign21890_e30667_d_n10, assign21890_e30667_d_n11, assign21890_e30667_d_n12, assign21890_e30667_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign21890_e30662: f64 = (0.2 * locals.var_beta);
        let assign21890_e30663: f64 = assign21890_e30662;
        let assign21890_e30665: f64 = (assign21890_e30663 - locals.var_tmf0);
        (assign21890_e30665, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), ((0.2 * locals.var_beta_dn10) - locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21890_e30667;
        locals.var_t1_dn0 = assign21890_e30667_d_n0;
        locals.var_t1_dn2 = assign21890_e30667_d_n2;
        locals.var_t1_dn6 = assign21890_e30667_d_n6;
        locals.var_t1_dn7 = assign21890_e30667_d_n7;
        locals.var_t1_dn10 = assign21890_e30667_d_n10;
        locals.var_t1_dn11 = assign21890_e30667_d_n11;
        locals.var_t1_dn12 = assign21890_e30667_d_n12;
        locals.var_t1_dn17 = assign21890_e30667_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21900_e30674, assign21900_e30674_d_n0, assign21900_e30674_d_n2, assign21900_e30674_d_n6, assign21900_e30674_d_n7, assign21900_e30674_d_n10, assign21900_e30674_d_n11, assign21900_e30674_d_n12, assign21900_e30674_d_n17,) = {
    if ((locals.var_guard600 != 0.0) && (locals.var_guard665 == 0.0)) {
        (locals.var_t1w__blk609, locals.var_t1w__blk609_dn0, locals.var_t1w__blk609_dn2, locals.var_t1w__blk609_dn6, locals.var_t1w__blk609_dn7, locals.var_t1w__blk609_dn10, locals.var_t1w__blk609_dn11, locals.var_t1w__blk609_dn12, locals.var_t1w__blk609_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21900_e30674;
        locals.var_t1_dn0 = assign21900_e30674_d_n0;
        locals.var_t1_dn2 = assign21900_e30674_d_n2;
        locals.var_t1_dn6 = assign21900_e30674_d_n6;
        locals.var_t1_dn7 = assign21900_e30674_d_n7;
        locals.var_t1_dn10 = assign21900_e30674_d_n10;
        locals.var_t1_dn11 = assign21900_e30674_d_n11;
        locals.var_t1_dn12 = assign21900_e30674_d_n12;
        locals.var_t1_dn17 = assign21900_e30674_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21910_e30683, assign21910_e30683_d_n0, assign21910_e30683_d_n2, assign21910_e30683_d_n6, assign21910_e30683_d_n7, assign21910_e30683_d_n10, assign21910_e30683_d_n11, assign21910_e30683_d_n12, assign21910_e30683_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21910_e30679: f64 = (10.0 * 2.220446049250313e-16);
        let assign21910_e30680: f64 = (locals.var_t1 + assign21910_e30679);
        let assign21910_e30681: f64 = (assign21910_e30680).sqrt();
        (assign21910_e30681, (locals.var_t1_dn0 / (2.0 * assign21910_e30681)), (locals.var_t1_dn2 / (2.0 * assign21910_e30681)), (locals.var_t1_dn6 / (2.0 * assign21910_e30681)), (locals.var_t1_dn7 / (2.0 * assign21910_e30681)), (locals.var_t1_dn10 / (2.0 * assign21910_e30681)), (locals.var_t1_dn11 / (2.0 * assign21910_e30681)), (locals.var_t1_dn12 / (2.0 * assign21910_e30681)), (locals.var_t1_dn17 / (2.0 * assign21910_e30681)),)
    } else {
        (locals.var_sq1npt, locals.var_sq1npt_dn0, locals.var_sq1npt_dn2, locals.var_sq1npt_dn6, locals.var_sq1npt_dn7, locals.var_sq1npt_dn10, locals.var_sq1npt_dn11, locals.var_sq1npt_dn12, locals.var_sq1npt_dn17,)
    }
};
        locals.var_sq1npt = assign21910_e30683;
        locals.var_sq1npt_dn0 = assign21910_e30683_d_n0;
        locals.var_sq1npt_dn2 = assign21910_e30683_d_n2;
        locals.var_sq1npt_dn6 = assign21910_e30683_d_n6;
        locals.var_sq1npt_dn7 = assign21910_e30683_d_n7;
        locals.var_sq1npt_dn10 = assign21910_e30683_d_n10;
        locals.var_sq1npt_dn11 = assign21910_e30683_d_n11;
        locals.var_sq1npt_dn12 = assign21910_e30683_d_n12;
        locals.var_sq1npt_dn17 = assign21910_e30683_d_n17;
        locals.var_sq1npt_rv = 0.0;

        let (assign21920_e30689, assign21920_e30689_d_n0, assign21920_e30689_d_n2, assign21920_e30689_d_n6, assign21920_e30689_d_n7, assign21920_e30689_d_n10, assign21920_e30689_d_n11, assign21920_e30689_d_n12, assign21920_e30689_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21920_e30687: f64 = (locals.var_conpt0 * locals.var_sq1npt);
        (assign21920_e30687, (locals.var_conpt0 * locals.var_sq1npt_dn0), (locals.var_conpt0 * locals.var_sq1npt_dn2), (locals.var_conpt0 * locals.var_sq1npt_dn6), (locals.var_conpt0 * locals.var_sq1npt_dn7), ((locals.var_conpt0_dn10 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn10)), (locals.var_conpt0 * locals.var_sq1npt_dn11), (locals.var_conpt0 * locals.var_sq1npt_dn12), (locals.var_conpt0 * locals.var_sq1npt_dn17),)
    } else {
        (locals.var_qn0npt, locals.var_qn0npt_dn0, locals.var_qn0npt_dn2, locals.var_qn0npt_dn6, locals.var_qn0npt_dn7, locals.var_qn0npt_dn10, locals.var_qn0npt_dn11, locals.var_qn0npt_dn12, locals.var_qn0npt_dn17,)
    }
};
        locals.var_qn0npt = assign21920_e30689;
        locals.var_qn0npt_dn0 = assign21920_e30689_d_n0;
        locals.var_qn0npt_dn2 = assign21920_e30689_d_n2;
        locals.var_qn0npt_dn6 = assign21920_e30689_d_n6;
        locals.var_qn0npt_dn7 = assign21920_e30689_d_n7;
        locals.var_qn0npt_dn10 = assign21920_e30689_d_n10;
        locals.var_qn0npt_dn11 = assign21920_e30689_d_n11;
        locals.var_qn0npt_dn12 = assign21920_e30689_d_n12;
        locals.var_qn0npt_dn17 = assign21920_e30689_d_n17;
        locals.var_qn0npt_rv = 0.0;

        let (assign21930_e30699, assign21930_e30699_d_n0, assign21930_e30699_d_n2, assign21930_e30699_d_n6, assign21930_e30699_d_n7, assign21930_e30699_d_n10, assign21930_e30699_d_n11, assign21930_e30699_d_n12, assign21930_e30699_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21930_e30693: f64 = (2.0 * locals.var_beta_inv);
        let assign21930_e30695: f64 = (assign21930_e30693 / locals.var_leff__blk607);
        let assign21930_e30697: f64 = (assign21930_e30695 * locals.var_qn0npt);
        (assign21930_e30697, (assign21930_e30695 * locals.var_qn0npt_dn0), (assign21930_e30695 * locals.var_qn0npt_dn2), (assign21930_e30695 * locals.var_qn0npt_dn6), (assign21930_e30695 * locals.var_qn0npt_dn7), ((((2.0 * locals.var_beta_inv_dn10) / locals.var_leff__blk607) * locals.var_qn0npt) + (assign21930_e30695 * locals.var_qn0npt_dn10)), (assign21930_e30695 * locals.var_qn0npt_dn11), (assign21930_e30695 * locals.var_qn0npt_dn12), (assign21930_e30695 * locals.var_qn0npt_dn17),)
    } else {
        (locals.var_wk_jnpt_a, locals.var_wk_jnpt_a_dn0, locals.var_wk_jnpt_a_dn2, locals.var_wk_jnpt_a_dn6, locals.var_wk_jnpt_a_dn7, locals.var_wk_jnpt_a_dn10, locals.var_wk_jnpt_a_dn11, locals.var_wk_jnpt_a_dn12, locals.var_wk_jnpt_a_dn17,)
    }
};
        locals.var_wk_jnpt_a = assign21930_e30699;
        locals.var_wk_jnpt_a_dn0 = assign21930_e30699_d_n0;
        locals.var_wk_jnpt_a_dn2 = assign21930_e30699_d_n2;
        locals.var_wk_jnpt_a_dn6 = assign21930_e30699_d_n6;
        locals.var_wk_jnpt_a_dn7 = assign21930_e30699_d_n7;
        locals.var_wk_jnpt_a_dn10 = assign21930_e30699_d_n10;
        locals.var_wk_jnpt_a_dn11 = assign21930_e30699_d_n11;
        locals.var_wk_jnpt_a_dn12 = assign21930_e30699_d_n12;
        locals.var_wk_jnpt_a_dn17 = assign21930_e30699_d_n17;
        locals.var_wk_jnpt_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21940_e30709, assign21940_e30709_d_n0, assign21940_e30709_d_n2, assign21940_e30709_d_n6, assign21940_e30709_d_n7, assign21940_e30709_d_n10, assign21940_e30709_d_n11, assign21940_e30709_d_n12, assign21940_e30709_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21940_e30703: f64 = (locals.var_wk_jnpt_a * locals.var_wk_mu);
        let assign21940_e30705: f64 = (assign21940_e30703 * locals.var_weff_nf);
        let assign21940_e30707: f64 = (assign21940_e30705 * locals.var_ty);
        (assign21940_e30707, ((((locals.var_wk_jnpt_a_dn0 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn0)), ((((locals.var_wk_jnpt_a_dn2 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn2)), ((((locals.var_wk_jnpt_a_dn6 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn6)), ((((locals.var_wk_jnpt_a_dn7 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn7)), ((((locals.var_wk_jnpt_a_dn10 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn10)), ((((locals.var_wk_jnpt_a_dn11 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn11)), ((((locals.var_wk_jnpt_a_dn12 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn12)), ((((locals.var_wk_jnpt_a_dn17 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21940_e30705 * locals.var_ty_dn17)),)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn10, locals.var_idspt1_dn11, locals.var_idspt1_dn12, locals.var_idspt1_dn17,)
    }
};
        locals.var_idspt1 = assign21940_e30709;
        locals.var_idspt1_dn0 = assign21940_e30709_d_n0;
        locals.var_idspt1_dn2 = assign21940_e30709_d_n2;
        locals.var_idspt1_dn6 = assign21940_e30709_d_n6;
        locals.var_idspt1_dn7 = assign21940_e30709_d_n7;
        locals.var_idspt1_dn10 = assign21940_e30709_d_n10;
        locals.var_idspt1_dn11 = assign21940_e30709_d_n11;
        locals.var_idspt1_dn12 = assign21940_e30709_d_n12;
        locals.var_idspt1_dn17 = assign21940_e30709_d_n17;
        locals.var_idspt1_rv = 0.0;

        let (assign21950_e30715, assign21950_e30715_d_n0, assign21950_e30715_d_n2, assign21950_e30715_d_n6, assign21950_e30715_d_n7, assign21950_e30715_d_n10, assign21950_e30715_d_n11, assign21950_e30715_d_n12, assign21950_e30715_d_n17,) = {
    if (locals.var_guard600 != 0.0) {
        let assign21950_e30713: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign21950_e30713, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn11 + locals.var_idspt1_dn11), (locals.var_idsorg_dn12 + locals.var_idspt1_dn12), (locals.var_idsorg_dn17 + locals.var_idspt1_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign21950_e30715;
        locals.var_ids_dn0 = assign21950_e30715_d_n0;
        locals.var_ids_dn2 = assign21950_e30715_d_n2;
        locals.var_ids_dn6 = assign21950_e30715_d_n6;
        locals.var_ids_dn7 = assign21950_e30715_d_n7;
        locals.var_ids_dn10 = assign21950_e30715_d_n10;
        locals.var_ids_dn11 = assign21950_e30715_d_n11;
        locals.var_ids_dn12 = assign21950_e30715_d_n12;
        locals.var_ids_dn17 = assign21950_e30715_d_n17;
        locals.var_ids_rv = 0.0;

        let assign21960_e30718: f64 = (locals.var_idspt0 + locals.var_idspt1);
        locals.var_idspt = assign21960_e30718;
        locals.var_idspt_dn0 = (locals.var_idspt0_dn0 + locals.var_idspt1_dn0);
        locals.var_idspt_dn2 = (locals.var_idspt0_dn2 + locals.var_idspt1_dn2);
        locals.var_idspt_dn6 = (locals.var_idspt0_dn6 + locals.var_idspt1_dn6);
        locals.var_idspt_dn7 = (locals.var_idspt0_dn7 + locals.var_idspt1_dn7);
        locals.var_idspt_dn10 = (locals.var_idspt0_dn10 + locals.var_idspt1_dn10);
        locals.var_idspt_dn11 = (locals.var_idspt0_dn11 + locals.var_idspt1_dn11);
        locals.var_idspt_dn12 = (locals.var_idspt0_dn12 + locals.var_idspt1_dn12);
        locals.var_idspt_dn17 = (locals.var_idspt0_dn17 + locals.var_idspt1_dn17);
        locals.var_idspt_rv = 0.0;

        let assign21970_e30725: f64 = if ((p.p43 == 1.0) || (p.p45 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard671 = assign21970_e30725;
        locals.var_guard671_rv = 0.0;

        let assign21980_e30732: f64 = if ((locals.var_flg_noqi == 1.0) || (p.p25 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard684 = assign21980_e30732;
        locals.var_guard684_rv = 0.0;

        let (assign21990_e30738, assign21990_e30738_d_n0, assign21990_e30738_d_n2, assign21990_e30738_d_n6, assign21990_e30738_d_n7, assign21990_e30738_d_n10, assign21990_e30738_d_n11, assign21990_e30738_d_n12, assign21990_e30738_d_n17,) = {
    if ((locals.var_guard671 != 0.0) && (locals.var_guard684 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign21990_e30738;
        locals.var_isub_dn0 = assign21990_e30738_d_n0;
        locals.var_isub_dn2 = assign21990_e30738_d_n2;
        locals.var_isub_dn6 = assign21990_e30738_d_n6;
        locals.var_isub_dn7 = assign21990_e30738_d_n7;
        locals.var_isub_dn10 = assign21990_e30738_d_n10;
        locals.var_isub_dn11 = assign21990_e30738_d_n11;
        locals.var_isub_dn12 = assign21990_e30738_d_n12;
        locals.var_isub_dn17 = assign21990_e30738_d_n17;
        locals.var_isub_rv = 0.0;

        let assign22000_e30745: f64 = if ((p.p117 <= 0.0) || (locals.var_mks_vmax <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard685 = assign22000_e30745;
        locals.var_guard685_rv = 0.0;

        let (assign22010_e30754, assign22010_e30754_d_n0, assign22010_e30754_d_n2, assign22010_e30754_d_n6, assign22010_e30754_d_n7, assign22010_e30754_d_n10, assign22010_e30754_d_n11, assign22010_e30754_d_n12, assign22010_e30754_d_n17,) = {
    if (((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign22010_e30754;
        locals.var_isub_dn0 = assign22010_e30754_d_n0;
        locals.var_isub_dn2 = assign22010_e30754_d_n2;
        locals.var_isub_dn6 = assign22010_e30754_d_n6;
        locals.var_isub_dn7 = assign22010_e30754_d_n7;
        locals.var_isub_dn10 = assign22010_e30754_d_n10;
        locals.var_isub_dn11 = assign22010_e30754_d_n11;
        locals.var_isub_dn12 = assign22010_e30754_d_n12;
        locals.var_isub_dn17 = assign22010_e30754_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign22020_e30772, assign22020_e30772_d_n0, assign22020_e30772_d_n2, assign22020_e30772_d_n6, assign22020_e30772_d_n7, assign22020_e30772_d_n10, assign22020_e30772_d_n11, assign22020_e30772_d_n12, assign22020_e30772_d_n17,) = {
    if (((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) {
        let assign22020_e30764: f64 = (locals.var_vgsz - locals.var_vfbsub0);
        let assign22020_e30766: f64 = (assign22020_e30764 + locals.var_dvth);
        let assign22020_e30768: f64 = (assign22020_e30766 - locals.var_dppg);
        let assign22020_e30770: f64 = (assign22020_e30768 + p.p48);
        (assign22020_e30770, ((locals.var_vgsz_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_vgsz_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    }
};
        locals.var_vgpsub = assign22020_e30772;
        locals.var_vgpsub_dn0 = assign22020_e30772_d_n0;
        locals.var_vgpsub_dn2 = assign22020_e30772_d_n2;
        locals.var_vgpsub_dn6 = assign22020_e30772_d_n6;
        locals.var_vgpsub_dn7 = assign22020_e30772_d_n7;
        locals.var_vgpsub_dn10 = assign22020_e30772_d_n10;
        locals.var_vgpsub_dn11 = assign22020_e30772_d_n11;
        locals.var_vgpsub_dn12 = assign22020_e30772_d_n12;
        locals.var_vgpsub_dn17 = assign22020_e30772_d_n17;
        locals.var_vgpsub_rv = 0.0;

        let assign22030_e30775: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign22030_e30775;
        locals.var_guard686_rv = 0.0;

        let (assign22040_e30787, assign22040_e30787_d_n0, assign22040_e30787_d_n2, assign22040_e30787_d_n6, assign22040_e30787_d_n7, assign22040_e30787_d_n10, assign22040_e30787_d_n11, assign22040_e30787_d_n12, assign22040_e30787_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_t1__blk672, locals.var_t1__blk672_dn0, locals.var_t1__blk672_dn2, locals.var_t1__blk672_dn6, locals.var_t1__blk672_dn7, locals.var_t1__blk672_dn10, locals.var_t1__blk672_dn11, locals.var_t1__blk672_dn12, locals.var_t1__blk672_dn17,)
    }
};
        locals.var_t1__blk672 = assign22040_e30787;
        locals.var_t1__blk672_dn0 = assign22040_e30787_d_n0;
        locals.var_t1__blk672_dn2 = assign22040_e30787_d_n2;
        locals.var_t1__blk672_dn6 = assign22040_e30787_d_n6;
        locals.var_t1__blk672_dn7 = assign22040_e30787_d_n7;
        locals.var_t1__blk672_dn10 = assign22040_e30787_d_n10;
        locals.var_t1__blk672_dn11 = assign22040_e30787_d_n11;
        locals.var_t1__blk672_dn12 = assign22040_e30787_d_n12;
        locals.var_t1__blk672_dn17 = assign22040_e30787_d_n17;
        locals.var_t1__blk672_rv = 0.0;

        let (assign22050_e30801, assign22050_e30801_d_n0, assign22050_e30801_d_n2, assign22050_e30801_d_n6, assign22050_e30801_d_n7, assign22050_e30801_d_n10, assign22050_e30801_d_n11, assign22050_e30801_d_n12, assign22050_e30801_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22050_e30799: f64 = (locals.var_c_fox * locals.var_c_fox);
        (assign22050_e30799, ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)), ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)), ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)), ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)), ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)), ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)), ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)), ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_t7__blk679, locals.var_t7__blk679_dn0, locals.var_t7__blk679_dn2, locals.var_t7__blk679_dn6, locals.var_t7__blk679_dn7, locals.var_t7__blk679_dn10, locals.var_t7__blk679_dn11, locals.var_t7__blk679_dn12, locals.var_t7__blk679_dn17,)
    }
};
        locals.var_t7__blk679 = assign22050_e30801;
        locals.var_t7__blk679_dn0 = assign22050_e30801_d_n0;
        locals.var_t7__blk679_dn2 = assign22050_e30801_d_n2;
        locals.var_t7__blk679_dn6 = assign22050_e30801_d_n6;
        locals.var_t7__blk679_dn7 = assign22050_e30801_d_n7;
        locals.var_t7__blk679_dn10 = assign22050_e30801_d_n10;
        locals.var_t7__blk679_dn11 = assign22050_e30801_d_n11;
        locals.var_t7__blk679_dn12 = assign22050_e30801_d_n12;
        locals.var_t7__blk679_dn17 = assign22050_e30801_d_n17;
        locals.var_t7__blk679_rv = 0.0;

        let (assign22060_e30813, assign22060_e30813_d_n0, assign22060_e30813_d_n2, assign22060_e30813_d_n6, assign22060_e30813_d_n7, assign22060_e30813_d_n10, assign22060_e30813_d_n11, assign22060_e30813_d_n12, assign22060_e30813_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        (locals.var_qnsub_esi, locals.var_qnsub_esi_dn0, locals.var_qnsub_esi_dn2, locals.var_qnsub_esi_dn6, locals.var_qnsub_esi_dn7, locals.var_qnsub_esi_dn10, locals.var_qnsub_esi_dn11, locals.var_qnsub_esi_dn12, locals.var_qnsub_esi_dn17,)
    } else {
        (locals.var_t8__blk680, locals.var_t8__blk680_dn0, locals.var_t8__blk680_dn2, locals.var_t8__blk680_dn6, locals.var_t8__blk680_dn7, locals.var_t8__blk680_dn10, locals.var_t8__blk680_dn11, locals.var_t8__blk680_dn12, locals.var_t8__blk680_dn17,)
    }
};
        locals.var_t8__blk680 = assign22060_e30813;
        locals.var_t8__blk680_dn0 = assign22060_e30813_d_n0;
        locals.var_t8__blk680_dn2 = assign22060_e30813_d_n2;
        locals.var_t8__blk680_dn6 = assign22060_e30813_d_n6;
        locals.var_t8__blk680_dn7 = assign22060_e30813_d_n7;
        locals.var_t8__blk680_dn10 = assign22060_e30813_d_n10;
        locals.var_t8__blk680_dn11 = assign22060_e30813_d_n11;
        locals.var_t8__blk680_dn12 = assign22060_e30813_d_n12;
        locals.var_t8__blk680_dn17 = assign22060_e30813_d_n17;
        locals.var_t8__blk680_rv = 0.0;

        let (assign22070_e30827, assign22070_e30827_d_n0, assign22070_e30827_d_n2, assign22070_e30827_d_n6, assign22070_e30827_d_n7, assign22070_e30827_d_n10, assign22070_e30827_d_n11, assign22070_e30827_d_n12, assign22070_e30827_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22070_e30825: f64 = (locals.var_t8__blk680 / locals.var_t7__blk679);
        (assign22070_e30825, (((locals.var_t8__blk680_dn0 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn0)) / (locals.var_t7__blk679 * locals.var_t7__blk679)), (((locals.var_t8__blk680_dn2 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn2)) / (locals.var_t7__blk679 * locals.var_t7__blk679)), (((locals.var_t8__blk680_dn6 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn6)) / (locals.var_t7__blk679 * locals.var_t7__blk679)), (((locals.var_t8__blk680_dn7 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn7)) / (locals.var_t7__blk679 * locals.var_t7__blk679)), (((locals.var_t8__blk680_dn10 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn10)) / (locals.var_t7__blk679 * locals.var_t7__blk679)), (((locals.var_t8__blk680_dn11 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn11)) / (locals.var_t7__blk679 * locals.var_t7__blk679)), (((locals.var_t8__blk680_dn12 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn12)) / (locals.var_t7__blk679 * locals.var_t7__blk679)), (((locals.var_t8__blk680_dn17 * locals.var_t7__blk679) - (locals.var_t8__blk680 * locals.var_t7__blk679_dn17)) / (locals.var_t7__blk679 * locals.var_t7__blk679)),)
    } else {
        (locals.var_t3__blk674, locals.var_t3__blk674_dn0, locals.var_t3__blk674_dn2, locals.var_t3__blk674_dn6, locals.var_t3__blk674_dn7, locals.var_t3__blk674_dn10, locals.var_t3__blk674_dn11, locals.var_t3__blk674_dn12, locals.var_t3__blk674_dn17,)
    }
};
        locals.var_t3__blk674 = assign22070_e30827;
        locals.var_t3__blk674_dn0 = assign22070_e30827_d_n0;
        locals.var_t3__blk674_dn2 = assign22070_e30827_d_n2;
        locals.var_t3__blk674_dn6 = assign22070_e30827_d_n6;
        locals.var_t3__blk674_dn7 = assign22070_e30827_d_n7;
        locals.var_t3__blk674_dn10 = assign22070_e30827_d_n10;
        locals.var_t3__blk674_dn11 = assign22070_e30827_d_n11;
        locals.var_t3__blk674_dn12 = assign22070_e30827_d_n12;
        locals.var_t3__blk674_dn17 = assign22070_e30827_d_n17;
        locals.var_t3__blk674_rv = 0.0;

        let (assign22080_e30841, assign22080_e30841_d_n0, assign22080_e30841_d_n2, assign22080_e30841_d_n6, assign22080_e30841_d_n7, assign22080_e30841_d_n10, assign22080_e30841_d_n11, assign22080_e30841_d_n12, assign22080_e30841_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22080_e30839: f64 = (2.0 / locals.var_t8__blk680);
        (assign22080_e30839, (-((2.0 * locals.var_t8__blk680_dn0) / (locals.var_t8__blk680 * locals.var_t8__blk680))), (-((2.0 * locals.var_t8__blk680_dn2) / (locals.var_t8__blk680 * locals.var_t8__blk680))), (-((2.0 * locals.var_t8__blk680_dn6) / (locals.var_t8__blk680 * locals.var_t8__blk680))), (-((2.0 * locals.var_t8__blk680_dn7) / (locals.var_t8__blk680 * locals.var_t8__blk680))), (-((2.0 * locals.var_t8__blk680_dn10) / (locals.var_t8__blk680 * locals.var_t8__blk680))), (-((2.0 * locals.var_t8__blk680_dn11) / (locals.var_t8__blk680 * locals.var_t8__blk680))), (-((2.0 * locals.var_t8__blk680_dn12) / (locals.var_t8__blk680 * locals.var_t8__blk680))), (-((2.0 * locals.var_t8__blk680_dn17) / (locals.var_t8__blk680 * locals.var_t8__blk680))),)
    } else {
        (locals.var_t9__blk681, locals.var_t9__blk681_dn0, locals.var_t9__blk681_dn2, locals.var_t9__blk681_dn6, locals.var_t9__blk681_dn7, locals.var_t9__blk681_dn10, locals.var_t9__blk681_dn11, locals.var_t9__blk681_dn12, locals.var_t9__blk681_dn17,)
    }
};
        locals.var_t9__blk681 = assign22080_e30841;
        locals.var_t9__blk681_dn0 = assign22080_e30841_d_n0;
        locals.var_t9__blk681_dn2 = assign22080_e30841_d_n2;
        locals.var_t9__blk681_dn6 = assign22080_e30841_d_n6;
        locals.var_t9__blk681_dn7 = assign22080_e30841_d_n7;
        locals.var_t9__blk681_dn10 = assign22080_e30841_d_n10;
        locals.var_t9__blk681_dn11 = assign22080_e30841_d_n11;
        locals.var_t9__blk681_dn12 = assign22080_e30841_d_n12;
        locals.var_t9__blk681_dn17 = assign22080_e30841_d_n17;
        locals.var_t9__blk681_rv = 0.0;

        let (assign22090_e30855, assign22090_e30855_d_n0, assign22090_e30855_d_n2, assign22090_e30855_d_n6, assign22090_e30855_d_n7, assign22090_e30855_d_n10, assign22090_e30855_d_n11, assign22090_e30855_d_n12, assign22090_e30855_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22090_e30853: f64 = (locals.var_t9__blk681 * locals.var_t7__blk679);
        (assign22090_e30853, ((locals.var_t9__blk681_dn0 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn0)), ((locals.var_t9__blk681_dn2 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn2)), ((locals.var_t9__blk681_dn6 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn6)), ((locals.var_t9__blk681_dn7 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn7)), ((locals.var_t9__blk681_dn10 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn10)), ((locals.var_t9__blk681_dn11 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn11)), ((locals.var_t9__blk681_dn12 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn12)), ((locals.var_t9__blk681_dn17 * locals.var_t7__blk679) + (locals.var_t9__blk681 * locals.var_t7__blk679_dn17)),)
    } else {
        (locals.var_t4__blk675, locals.var_t4__blk675_dn0, locals.var_t4__blk675_dn2, locals.var_t4__blk675_dn6, locals.var_t4__blk675_dn7, locals.var_t4__blk675_dn10, locals.var_t4__blk675_dn11, locals.var_t4__blk675_dn12, locals.var_t4__blk675_dn17,)
    }
};
        locals.var_t4__blk675 = assign22090_e30855;
        locals.var_t4__blk675_dn0 = assign22090_e30855_d_n0;
        locals.var_t4__blk675_dn2 = assign22090_e30855_d_n2;
        locals.var_t4__blk675_dn6 = assign22090_e30855_d_n6;
        locals.var_t4__blk675_dn7 = assign22090_e30855_d_n7;
        locals.var_t4__blk675_dn10 = assign22090_e30855_d_n10;
        locals.var_t4__blk675_dn11 = assign22090_e30855_d_n11;
        locals.var_t4__blk675_dn12 = assign22090_e30855_d_n12;
        locals.var_t4__blk675_dn17 = assign22090_e30855_d_n17;
        locals.var_t4__blk675_rv = 0.0;

        let (assign22100_e30873, assign22100_e30873_d_n0, assign22100_e30873_d_n2, assign22100_e30873_d_n6, assign22100_e30873_d_n7, assign22100_e30873_d_n10, assign22100_e30873_d_n11, assign22100_e30873_d_n12, assign22100_e30873_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22100_e30867: f64 = (locals.var_t1__blk672 - locals.var_beta_inv);
        let assign22100_e30870: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign22100_e30871: f64 = (assign22100_e30867 - assign22100_e30870);
        (assign22100_e30871, (locals.var_t1__blk672_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk672_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk672_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk672_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk672_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk672_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk672_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk672_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk676, locals.var_t5__blk676_dn0, locals.var_t5__blk676_dn2, locals.var_t5__blk676_dn6, locals.var_t5__blk676_dn7, locals.var_t5__blk676_dn10, locals.var_t5__blk676_dn11, locals.var_t5__blk676_dn12, locals.var_t5__blk676_dn17,)
    }
};
        locals.var_t5__blk676 = assign22100_e30873;
        locals.var_t5__blk676_dn0 = assign22100_e30873_d_n0;
        locals.var_t5__blk676_dn2 = assign22100_e30873_d_n2;
        locals.var_t5__blk676_dn6 = assign22100_e30873_d_n6;
        locals.var_t5__blk676_dn7 = assign22100_e30873_d_n7;
        locals.var_t5__blk676_dn10 = assign22100_e30873_d_n10;
        locals.var_t5__blk676_dn11 = assign22100_e30873_d_n11;
        locals.var_t5__blk676_dn12 = assign22100_e30873_d_n12;
        locals.var_t5__blk676_dn17 = assign22100_e30873_d_n17;
        locals.var_t5__blk676_rv = 0.0;

        let (assign22110_e30889, assign22110_e30889_d_n0, assign22110_e30889_d_n2, assign22110_e30889_d_n6, assign22110_e30889_d_n7, assign22110_e30889_d_n10, assign22110_e30889_d_n11, assign22110_e30889_d_n12, assign22110_e30889_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22110_e30885: f64 = (p.p49 * locals.var_qhs);
        let assign22110_e30887: f64 = (assign22110_e30885 / locals.var_c_soi);
        (assign22110_e30887, ((p.p49 * locals.var_qhs_dn0) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn2) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn6) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn7) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn10) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn11) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn12) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn17) / locals.var_c_soi),)
    } else {
        (locals.var_dvbssub, locals.var_dvbssub_dn0, locals.var_dvbssub_dn2, locals.var_dvbssub_dn6, locals.var_dvbssub_dn7, locals.var_dvbssub_dn10, locals.var_dvbssub_dn11, locals.var_dvbssub_dn12, locals.var_dvbssub_dn17,)
    }
};
        locals.var_dvbssub = assign22110_e30889;
        locals.var_dvbssub_dn0 = assign22110_e30889_d_n0;
        locals.var_dvbssub_dn2 = assign22110_e30889_d_n2;
        locals.var_dvbssub_dn6 = assign22110_e30889_d_n6;
        locals.var_dvbssub_dn7 = assign22110_e30889_d_n7;
        locals.var_dvbssub_dn10 = assign22110_e30889_d_n10;
        locals.var_dvbssub_dn11 = assign22110_e30889_d_n11;
        locals.var_dvbssub_dn12 = assign22110_e30889_d_n12;
        locals.var_dvbssub_dn17 = assign22110_e30889_d_n17;
        locals.var_dvbssub_rv = 0.0;

        let (assign22120_e30905, assign22120_e30905_d_n0, assign22120_e30905_d_n2, assign22120_e30905_d_n6, assign22120_e30905_d_n7, assign22120_e30905_d_n10, assign22120_e30905_d_n11, assign22120_e30905_d_n12, assign22120_e30905_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22120_e30902: f64 = (locals.var_xvbs * locals.var_dvbssub);
        let assign22120_e30903: f64 = (locals.var_t5__blk676 - assign22120_e30902);
        (assign22120_e30903, (locals.var_t5__blk676_dn0 - (locals.var_xvbs * locals.var_dvbssub_dn0)), (locals.var_t5__blk676_dn2 - (locals.var_xvbs * locals.var_dvbssub_dn2)), (locals.var_t5__blk676_dn6 - (locals.var_xvbs * locals.var_dvbssub_dn6)), (locals.var_t5__blk676_dn7 - (locals.var_xvbs * locals.var_dvbssub_dn7)), (locals.var_t5__blk676_dn10 - (locals.var_xvbs * locals.var_dvbssub_dn10)), (locals.var_t5__blk676_dn11 - (locals.var_xvbs * locals.var_dvbssub_dn11)), (locals.var_t5__blk676_dn12 - (locals.var_xvbs * locals.var_dvbssub_dn12)), (locals.var_t5__blk676_dn17 - (locals.var_xvbs * locals.var_dvbssub_dn17)),)
    } else {
        (locals.var_t5__blk676, locals.var_t5__blk676_dn0, locals.var_t5__blk676_dn2, locals.var_t5__blk676_dn6, locals.var_t5__blk676_dn7, locals.var_t5__blk676_dn10, locals.var_t5__blk676_dn11, locals.var_t5__blk676_dn12, locals.var_t5__blk676_dn17,)
    }
};
        locals.var_t5__blk676 = assign22120_e30905;
        locals.var_t5__blk676_dn0 = assign22120_e30905_d_n0;
        locals.var_t5__blk676_dn2 = assign22120_e30905_d_n2;
        locals.var_t5__blk676_dn6 = assign22120_e30905_d_n6;
        locals.var_t5__blk676_dn7 = assign22120_e30905_d_n7;
        locals.var_t5__blk676_dn10 = assign22120_e30905_d_n10;
        locals.var_t5__blk676_dn11 = assign22120_e30905_d_n11;
        locals.var_t5__blk676_dn12 = assign22120_e30905_d_n12;
        locals.var_t5__blk676_dn17 = assign22120_e30905_d_n17;
        locals.var_t5__blk676_rv = 0.0;

        let (assign22130_e30921, assign22130_e30921_d_n0, assign22130_e30921_d_n2, assign22130_e30921_d_n6, assign22130_e30921_d_n7, assign22130_e30921_d_n10, assign22130_e30921_d_n11, assign22130_e30921_d_n12, assign22130_e30921_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22130_e30918: f64 = (locals.var_t4__blk675 * locals.var_t5__blk676);
        let assign22130_e30919: f64 = (1.0 + assign22130_e30918);
        (assign22130_e30919, ((locals.var_t4__blk675_dn0 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn0)), ((locals.var_t4__blk675_dn2 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn2)), ((locals.var_t4__blk675_dn6 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn6)), ((locals.var_t4__blk675_dn7 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn7)), ((locals.var_t4__blk675_dn10 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn10)), ((locals.var_t4__blk675_dn11 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn11)), ((locals.var_t4__blk675_dn12 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn12)), ((locals.var_t4__blk675_dn17 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn17)),)
    } else {
        (locals.var_t6w__blk678, locals.var_t6w__blk678_dn0, locals.var_t6w__blk678_dn2, locals.var_t6w__blk678_dn6, locals.var_t6w__blk678_dn7, locals.var_t6w__blk678_dn10, locals.var_t6w__blk678_dn11, locals.var_t6w__blk678_dn12, locals.var_t6w__blk678_dn17,)
    }
};
        locals.var_t6w__blk678 = assign22130_e30921;
        locals.var_t6w__blk678_dn0 = assign22130_e30921_d_n0;
        locals.var_t6w__blk678_dn2 = assign22130_e30921_d_n2;
        locals.var_t6w__blk678_dn6 = assign22130_e30921_d_n6;
        locals.var_t6w__blk678_dn7 = assign22130_e30921_d_n7;
        locals.var_t6w__blk678_dn10 = assign22130_e30921_d_n10;
        locals.var_t6w__blk678_dn11 = assign22130_e30921_d_n11;
        locals.var_t6w__blk678_dn12 = assign22130_e30921_d_n12;
        locals.var_t6w__blk678_dn17 = assign22130_e30921_d_n17;
        locals.var_t6w__blk678_rv = 0.0;

        let (assign22140_e30942, assign22140_e30942_d_n0, assign22140_e30942_d_n2, assign22140_e30942_d_n6, assign22140_e30942_d_n7, assign22140_e30942_d_n10, assign22140_e30942_d_n11, assign22140_e30942_d_n12, assign22140_e30942_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22140_e30933: f64 = (locals.var_t6w__blk678 * locals.var_t6w__blk678);
        let assign22140_e30936: f64 = (4.0 * 0.001);
        let assign22140_e30938: f64 = (assign22140_e30936 * 0.001);
        let assign22140_e30939: f64 = (assign22140_e30933 + assign22140_e30938);
        let assign22140_e30940: f64 = (assign22140_e30939).sqrt();
        (assign22140_e30940, (((locals.var_t6w__blk678_dn0 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn0)) / (2.0 * assign22140_e30940)), (((locals.var_t6w__blk678_dn2 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn2)) / (2.0 * assign22140_e30940)), (((locals.var_t6w__blk678_dn6 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn6)) / (2.0 * assign22140_e30940)), (((locals.var_t6w__blk678_dn7 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn7)) / (2.0 * assign22140_e30940)), (((locals.var_t6w__blk678_dn10 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn10)) / (2.0 * assign22140_e30940)), (((locals.var_t6w__blk678_dn11 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn11)) / (2.0 * assign22140_e30940)), (((locals.var_t6w__blk678_dn12 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn12)) / (2.0 * assign22140_e30940)), (((locals.var_t6w__blk678_dn17 * locals.var_t6w__blk678) + (locals.var_t6w__blk678 * locals.var_t6w__blk678_dn17)) / (2.0 * assign22140_e30940)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22140_e30942;
        locals.var_tmf1_dn0 = assign22140_e30942_d_n0;
        locals.var_tmf1_dn2 = assign22140_e30942_d_n2;
        locals.var_tmf1_dn6 = assign22140_e30942_d_n6;
        locals.var_tmf1_dn7 = assign22140_e30942_d_n7;
        locals.var_tmf1_dn10 = assign22140_e30942_d_n10;
        locals.var_tmf1_dn11 = assign22140_e30942_d_n11;
        locals.var_tmf1_dn12 = assign22140_e30942_d_n12;
        locals.var_tmf1_dn17 = assign22140_e30942_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22150_e30962, assign22150_e30962_d_n0, assign22150_e30962_d_n2, assign22150_e30962_d_n6, assign22150_e30962_d_n7, assign22150_e30962_d_n10, assign22150_e30962_d_n11, assign22150_e30962_d_n12, assign22150_e30962_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22150_e30955: f64 = (locals.var_t6w__blk678 + locals.var_tmf1);
        let assign22150_e30956: f64 = (0.5 * assign22150_e30955);
        let assign22150_e30959: f64 = (1e-10 * 0.001);
        let assign22150_e30960: f64 = (assign22150_e30956 + assign22150_e30959);
        (assign22150_e30960, (0.5 * (locals.var_t6w__blk678_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk678_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk678_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk678_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk678_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk678_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk678_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk678_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22150_e30962;
        locals.var_t6__blk677_dn0 = assign22150_e30962_d_n0;
        locals.var_t6__blk677_dn2 = assign22150_e30962_d_n2;
        locals.var_t6__blk677_dn6 = assign22150_e30962_d_n6;
        locals.var_t6__blk677_dn7 = assign22150_e30962_d_n7;
        locals.var_t6__blk677_dn10 = assign22150_e30962_d_n10;
        locals.var_t6__blk677_dn11 = assign22150_e30962_d_n11;
        locals.var_t6__blk677_dn12 = assign22150_e30962_d_n12;
        locals.var_t6__blk677_dn17 = assign22150_e30962_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let assign22160_e30965: f64 = if locals.var_t6__blk677 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign22160_e30965;
        locals.var_guard687_rv = 0.0;

        let (assign22170_e30979, assign22170_e30979_d_n0, assign22170_e30979_d_n2, assign22170_e30979_d_n6, assign22170_e30979_d_n7, assign22170_e30979_d_n10, assign22170_e30979_d_n11, assign22170_e30979_d_n12, assign22170_e30979_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22170_e30979;
        locals.var_t6__blk677_dn0 = assign22170_e30979_d_n0;
        locals.var_t6__blk677_dn2 = assign22170_e30979_d_n2;
        locals.var_t6__blk677_dn6 = assign22170_e30979_d_n6;
        locals.var_t6__blk677_dn7 = assign22170_e30979_d_n7;
        locals.var_t6__blk677_dn10 = assign22170_e30979_d_n10;
        locals.var_t6__blk677_dn11 = assign22170_e30979_d_n11;
        locals.var_t6__blk677_dn12 = assign22170_e30979_d_n12;
        locals.var_t6__blk677_dn17 = assign22170_e30979_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let (assign22180_e30993, assign22180_e30993_d_n0, assign22180_e30993_d_n2, assign22180_e30993_d_n6, assign22180_e30993_d_n7, assign22180_e30993_d_n10, assign22180_e30993_d_n11, assign22180_e30993_d_n12, assign22180_e30993_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22180_e30991: f64 = (locals.var_t6__blk677 + 1e-50);
        (assign22180_e30991, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22180_e30993;
        locals.var_t6__blk677_dn0 = assign22180_e30993_d_n0;
        locals.var_t6__blk677_dn2 = assign22180_e30993_d_n2;
        locals.var_t6__blk677_dn6 = assign22180_e30993_d_n6;
        locals.var_t6__blk677_dn7 = assign22180_e30993_d_n7;
        locals.var_t6__blk677_dn10 = assign22180_e30993_d_n10;
        locals.var_t6__blk677_dn11 = assign22180_e30993_d_n11;
        locals.var_t6__blk677_dn12 = assign22180_e30993_d_n12;
        locals.var_t6__blk677_dn17 = assign22180_e30993_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let (assign22190_e31006, assign22190_e31006_d_n0, assign22190_e31006_d_n2, assign22190_e31006_d_n6, assign22190_e31006_d_n7, assign22190_e31006_d_n10, assign22190_e31006_d_n11, assign22190_e31006_d_n12, assign22190_e31006_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22190_e31004: f64 = (locals.var_t6__blk677).sqrt();
        (assign22190_e31004, (locals.var_t6__blk677_dn0 / (2.0 * assign22190_e31004)), (locals.var_t6__blk677_dn2 / (2.0 * assign22190_e31004)), (locals.var_t6__blk677_dn6 / (2.0 * assign22190_e31004)), (locals.var_t6__blk677_dn7 / (2.0 * assign22190_e31004)), (locals.var_t6__blk677_dn10 / (2.0 * assign22190_e31004)), (locals.var_t6__blk677_dn11 / (2.0 * assign22190_e31004)), (locals.var_t6__blk677_dn12 / (2.0 * assign22190_e31004)), (locals.var_t6__blk677_dn17 / (2.0 * assign22190_e31004)),)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22190_e31006;
        locals.var_t6__blk677_dn0 = assign22190_e31006_d_n0;
        locals.var_t6__blk677_dn2 = assign22190_e31006_d_n2;
        locals.var_t6__blk677_dn6 = assign22190_e31006_d_n6;
        locals.var_t6__blk677_dn7 = assign22190_e31006_d_n7;
        locals.var_t6__blk677_dn10 = assign22190_e31006_d_n10;
        locals.var_t6__blk677_dn11 = assign22190_e31006_d_n11;
        locals.var_t6__blk677_dn12 = assign22190_e31006_d_n12;
        locals.var_t6__blk677_dn17 = assign22190_e31006_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let (assign22200_e31026, assign22200_e31026_d_n0, assign22200_e31026_d_n2, assign22200_e31026_d_n6, assign22200_e31026_d_n7, assign22200_e31026_d_n10, assign22200_e31026_d_n11, assign22200_e31026_d_n12, assign22200_e31026_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22200_e31018: f64 = (locals.var_t1__blk672 * locals.var_uc_svgs);
        let assign22200_e31022: f64 = (1.0 - locals.var_t6__blk677);
        let assign22200_e31023: f64 = (locals.var_t3__blk674 * assign22200_e31022);
        let assign22200_e31024: f64 = (assign22200_e31018 + assign22200_e31023);
        (assign22200_e31024, ((locals.var_t1__blk672_dn0 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn0 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn0)))), ((locals.var_t1__blk672_dn2 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn2 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn2)))), ((locals.var_t1__blk672_dn6 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn6 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn6)))), ((locals.var_t1__blk672_dn7 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn7 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn7)))), ((locals.var_t1__blk672_dn10 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn10 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn10)))), ((locals.var_t1__blk672_dn11 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn11 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn11)))), ((locals.var_t1__blk672_dn12 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn12 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn12)))), ((locals.var_t1__blk672_dn17 * locals.var_uc_svgs) + ((locals.var_t3__blk674_dn17 * assign22200_e31022) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn17)))),)
    } else {
        (locals.var_psislsat__blk682, locals.var_psislsat__blk682_dn0, locals.var_psislsat__blk682_dn2, locals.var_psislsat__blk682_dn6, locals.var_psislsat__blk682_dn7, locals.var_psislsat__blk682_dn10, locals.var_psislsat__blk682_dn11, locals.var_psislsat__blk682_dn12, locals.var_psislsat__blk682_dn17,)
    }
};
        locals.var_psislsat__blk682 = assign22200_e31026;
        locals.var_psislsat__blk682_dn0 = assign22200_e31026_d_n0;
        locals.var_psislsat__blk682_dn2 = assign22200_e31026_d_n2;
        locals.var_psislsat__blk682_dn6 = assign22200_e31026_d_n6;
        locals.var_psislsat__blk682_dn7 = assign22200_e31026_d_n7;
        locals.var_psislsat__blk682_dn10 = assign22200_e31026_d_n10;
        locals.var_psislsat__blk682_dn11 = assign22200_e31026_d_n11;
        locals.var_psislsat__blk682_dn12 = assign22200_e31026_d_n12;
        locals.var_psislsat__blk682_dn17 = assign22200_e31026_d_n17;
        locals.var_psislsat__blk682_rv = 0.0;

        let (assign22210_e31048, assign22210_e31048_d_n0, assign22210_e31048_d_n2, assign22210_e31048_d_n6, assign22210_e31048_d_n7, assign22210_e31048_d_n10, assign22210_e31048_d_n11, assign22210_e31048_d_n12, assign22210_e31048_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22210_e31038: f64 = (p.p122 * locals.var_vdsz);
        let assign22210_e31040: f64 = (assign22210_e31038 + locals.var_ps0z);
        let assign22210_e31043: f64 = (locals.var_xgate * locals.var_zvgs);
        let assign22210_e31045: f64 = (assign22210_e31043 * locals.var_psislsat__blk682);
        let assign22210_e31046: f64 = (assign22210_e31040 - assign22210_e31045);
        (assign22210_e31046, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0z_dn0) - (assign22210_e31043 * locals.var_psislsat__blk682_dn0)), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0z_dn2) - (assign22210_e31043 * locals.var_psislsat__blk682_dn2)), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0z_dn6) - (assign22210_e31043 * locals.var_psislsat__blk682_dn6)), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0z_dn7) - (assign22210_e31043 * locals.var_psislsat__blk682_dn7)), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0z_dn10) - (assign22210_e31043 * locals.var_psislsat__blk682_dn10)), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0z_dn11) - (assign22210_e31043 * locals.var_psislsat__blk682_dn11)), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0z_dn12) - (assign22210_e31043 * locals.var_psislsat__blk682_dn12)), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0z_dn17) - (assign22210_e31043 * locals.var_psislsat__blk682_dn17)),)
    } else {
        (locals.var_psisubsat__blk683, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    }
};
        locals.var_psisubsat__blk683 = assign22210_e31048;
        locals.var_psisubsat__blk683_dn0 = assign22210_e31048_d_n0;
        locals.var_psisubsat__blk683_dn2 = assign22210_e31048_d_n2;
        locals.var_psisubsat__blk683_dn6 = assign22210_e31048_d_n6;
        locals.var_psisubsat__blk683_dn7 = assign22210_e31048_d_n7;
        locals.var_psisubsat__blk683_dn10 = assign22210_e31048_d_n10;
        locals.var_psisubsat__blk683_dn11 = assign22210_e31048_d_n11;
        locals.var_psisubsat__blk683_dn12 = assign22210_e31048_d_n12;
        locals.var_psisubsat__blk683_dn17 = assign22210_e31048_d_n17;
        locals.var_psisubsat__blk683_rv = 0.0;

        let (assign22220_e31069, assign22220_e31069_d_n0, assign22220_e31069_d_n2, assign22220_e31069_d_n6, assign22220_e31069_d_n7, assign22220_e31069_d_n10, assign22220_e31069_d_n11, assign22220_e31069_d_n12, assign22220_e31069_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22220_e31060: f64 = (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683);
        let assign22220_e31063: f64 = (4.0 * 0.01);
        let assign22220_e31065: f64 = (assign22220_e31063 * 0.01);
        let assign22220_e31066: f64 = (assign22220_e31060 + assign22220_e31065);
        let assign22220_e31067: f64 = (assign22220_e31066).sqrt();
        (assign22220_e31067, (((locals.var_psisubsat__blk683_dn0 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn0)) / (2.0 * assign22220_e31067)), (((locals.var_psisubsat__blk683_dn2 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn2)) / (2.0 * assign22220_e31067)), (((locals.var_psisubsat__blk683_dn6 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn6)) / (2.0 * assign22220_e31067)), (((locals.var_psisubsat__blk683_dn7 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn7)) / (2.0 * assign22220_e31067)), (((locals.var_psisubsat__blk683_dn10 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn10)) / (2.0 * assign22220_e31067)), (((locals.var_psisubsat__blk683_dn11 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn11)) / (2.0 * assign22220_e31067)), (((locals.var_psisubsat__blk683_dn12 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn12)) / (2.0 * assign22220_e31067)), (((locals.var_psisubsat__blk683_dn17 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn17)) / (2.0 * assign22220_e31067)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22220_e31069;
        locals.var_tmf1_dn0 = assign22220_e31069_d_n0;
        locals.var_tmf1_dn2 = assign22220_e31069_d_n2;
        locals.var_tmf1_dn6 = assign22220_e31069_d_n6;
        locals.var_tmf1_dn7 = assign22220_e31069_d_n7;
        locals.var_tmf1_dn10 = assign22220_e31069_d_n10;
        locals.var_tmf1_dn11 = assign22220_e31069_d_n11;
        locals.var_tmf1_dn12 = assign22220_e31069_d_n12;
        locals.var_tmf1_dn17 = assign22220_e31069_d_n17;
        locals.var_tmf1_rv = 0.0;

    }
}
