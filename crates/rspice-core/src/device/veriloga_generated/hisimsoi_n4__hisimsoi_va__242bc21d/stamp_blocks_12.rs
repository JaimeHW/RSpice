#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18000_e25561, assign18000_e25561_d_n0, assign18000_e25561_d_n2, assign18000_e25561_d_n6, assign18000_e25561_d_n7, assign18000_e25561_d_n10, assign18000_e25561_d_n11, assign18000_e25561_d_n12, assign18000_e25561_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18000_e25559: f64 = (1.0 / locals.var_t1__blk533);
        (assign18000_e25559, (-(locals.var_t1__blk533_dn0 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn2 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn6 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn7 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn10 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn11 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn12 / (locals.var_t1__blk533 * locals.var_t1__blk533))), (-(locals.var_t1__blk533_dn17 / (locals.var_t1__blk533 * locals.var_t1__blk533))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn12, locals.var_muun_dn17,)
    }
};
        locals.var_muun = assign18000_e25561;
        locals.var_muun_dn0 = assign18000_e25561_d_n0;
        locals.var_muun_dn2 = assign18000_e25561_d_n2;
        locals.var_muun_dn6 = assign18000_e25561_d_n6;
        locals.var_muun_dn7 = assign18000_e25561_d_n7;
        locals.var_muun_dn10 = assign18000_e25561_d_n10;
        locals.var_muun_dn11 = assign18000_e25561_d_n11;
        locals.var_muun_dn12 = assign18000_e25561_d_n12;
        locals.var_muun_dn17 = assign18000_e25561_d_n17;
        locals.var_muun_rv = 0.0;

        let (assign18010_e25567, assign18010_e25567_d_n0, assign18010_e25567_d_n2, assign18010_e25567_d_n6, assign18010_e25567_d_n7, assign18010_e25567_d_n10, assign18010_e25567_d_n11, assign18010_e25567_d_n12, assign18010_e25567_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18010_e25565: f64 = (locals.var_muun * 0.0001);
        (assign18010_e25565, (locals.var_muun_dn0 * 0.0001), (locals.var_muun_dn2 * 0.0001), (locals.var_muun_dn6 * 0.0001), (locals.var_muun_dn7 * 0.0001), (locals.var_muun_dn10 * 0.0001), (locals.var_muun_dn11 * 0.0001), (locals.var_muun_dn12 * 0.0001), (locals.var_muun_dn17 * 0.0001),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn12, locals.var_muun_dn17,)
    }
};
        locals.var_muun = assign18010_e25567;
        locals.var_muun_dn0 = assign18010_e25567_d_n0;
        locals.var_muun_dn2 = assign18010_e25567_d_n2;
        locals.var_muun_dn6 = assign18010_e25567_d_n6;
        locals.var_muun_dn7 = assign18010_e25567_d_n7;
        locals.var_muun_dn10 = assign18010_e25567_d_n10;
        locals.var_muun_dn11 = assign18010_e25567_d_n11;
        locals.var_muun_dn12 = assign18010_e25567_d_n12;
        locals.var_muun_dn17 = assign18010_e25567_d_n17;
        locals.var_muun_rv = 0.0;

        let (assign18020_e25575, assign18020_e25575_d_n0, assign18020_e25575_d_n2, assign18020_e25575_d_n6, assign18020_e25575_d_n7, assign18020_e25575_d_n10, assign18020_e25575_d_n11, assign18020_e25575_d_n12, assign18020_e25575_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18020_e25571: f64 = (locals.var_beta * locals.var_qn0);
        let assign18020_e25573: f64 = (assign18020_e25571 * locals.var_lch);
        (assign18020_e25573, (((locals.var_beta * locals.var_qn0_dn0) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn0)), (((locals.var_beta * locals.var_qn0_dn2) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn2)), (((locals.var_beta * locals.var_qn0_dn6) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn6)), (((locals.var_beta * locals.var_qn0_dn7) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn7)), ((((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn10)), (((locals.var_beta * locals.var_qn0_dn11) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn11)), (((locals.var_beta * locals.var_qn0_dn12) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn12)), (((locals.var_beta * locals.var_qn0_dn17) * locals.var_lch) + (assign18020_e25571 * locals.var_lch_dn17)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18020_e25575;
        locals.var_t2__blk546_dn0 = assign18020_e25575_d_n0;
        locals.var_t2__blk546_dn2 = assign18020_e25575_d_n2;
        locals.var_t2__blk546_dn6 = assign18020_e25575_d_n6;
        locals.var_t2__blk546_dn7 = assign18020_e25575_d_n7;
        locals.var_t2__blk546_dn10 = assign18020_e25575_d_n10;
        locals.var_t2__blk546_dn11 = assign18020_e25575_d_n11;
        locals.var_t2__blk546_dn12 = assign18020_e25575_d_n12;
        locals.var_t2__blk546_dn17 = assign18020_e25575_d_n17;
        locals.var_t2__blk546_rv = 0.0;

        let (assign18030_e25588, assign18030_e25588_d_n0, assign18030_e25588_d_n2, assign18030_e25588_d_n6, assign18030_e25588_d_n7, assign18030_e25588_d_n10, assign18030_e25588_d_n11, assign18030_e25588_d_n12, assign18030_e25588_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18030_e25579: f64 = (locals.var_t2__blk546 * locals.var_t2__blk546);
        let assign18030_e25582: f64 = (4.0 * 1e-50);
        let assign18030_e25584: f64 = (assign18030_e25582 * 1e-50);
        let assign18030_e25585: f64 = (assign18030_e25579 + assign18030_e25584);
        let assign18030_e25586: f64 = (assign18030_e25585).sqrt();
        (assign18030_e25586, (((locals.var_t2__blk546_dn0 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn0)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn2 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn2)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn6 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn6)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn7 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn7)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn10 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn10)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn11 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn11)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn12 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn12)) / (2.0 * assign18030_e25586)), (((locals.var_t2__blk546_dn17 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn17)) / (2.0 * assign18030_e25586)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18030_e25588;
        locals.var_tmf1_dn0 = assign18030_e25588_d_n0;
        locals.var_tmf1_dn2 = assign18030_e25588_d_n2;
        locals.var_tmf1_dn6 = assign18030_e25588_d_n6;
        locals.var_tmf1_dn7 = assign18030_e25588_d_n7;
        locals.var_tmf1_dn10 = assign18030_e25588_d_n10;
        locals.var_tmf1_dn11 = assign18030_e25588_d_n11;
        locals.var_tmf1_dn12 = assign18030_e25588_d_n12;
        locals.var_tmf1_dn17 = assign18030_e25588_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18040_e25600, assign18040_e25600_d_n0, assign18040_e25600_d_n2, assign18040_e25600_d_n6, assign18040_e25600_d_n7, assign18040_e25600_d_n10, assign18040_e25600_d_n11, assign18040_e25600_d_n12, assign18040_e25600_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18040_e25593: f64 = (locals.var_t2__blk546 + locals.var_tmf1);
        let assign18040_e25594: f64 = (0.5 * assign18040_e25593);
        let assign18040_e25597: f64 = (1e-10 * 1e-50);
        let assign18040_e25598: f64 = (assign18040_e25594 + assign18040_e25597);
        (assign18040_e25598, (0.5 * (locals.var_t2__blk546_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t2__blk546_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t2__blk546_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t2__blk546_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t2__blk546_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t2__blk546_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t2__blk546_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t2__blk546_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18040_e25600;
        locals.var_t2__blk546_dn0 = assign18040_e25600_d_n0;
        locals.var_t2__blk546_dn2 = assign18040_e25600_d_n2;
        locals.var_t2__blk546_dn6 = assign18040_e25600_d_n6;
        locals.var_t2__blk546_dn7 = assign18040_e25600_d_n7;
        locals.var_t2__blk546_dn10 = assign18040_e25600_d_n10;
        locals.var_t2__blk546_dn11 = assign18040_e25600_d_n11;
        locals.var_t2__blk546_dn12 = assign18040_e25600_d_n12;
        locals.var_t2__blk546_dn17 = assign18040_e25600_d_n17;
        locals.var_t2__blk546_rv = 0.0;

        let assign18050_e25603: f64 = if locals.var_t2__blk546 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign18050_e25603;
        locals.var_guard554_rv = 0.0;

        let (assign18060_e25609, assign18060_e25609_d_n0, assign18060_e25609_d_n2, assign18060_e25609_d_n6, assign18060_e25609_d_n7, assign18060_e25609_d_n10, assign18060_e25609_d_n11, assign18060_e25609_d_n12, assign18060_e25609_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard554 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18060_e25609;
        locals.var_t2__blk546_dn0 = assign18060_e25609_d_n0;
        locals.var_t2__blk546_dn2 = assign18060_e25609_d_n2;
        locals.var_t2__blk546_dn6 = assign18060_e25609_d_n6;
        locals.var_t2__blk546_dn7 = assign18060_e25609_d_n7;
        locals.var_t2__blk546_dn10 = assign18060_e25609_d_n10;
        locals.var_t2__blk546_dn11 = assign18060_e25609_d_n11;
        locals.var_t2__blk546_dn12 = assign18060_e25609_d_n12;
        locals.var_t2__blk546_dn17 = assign18060_e25609_d_n17;
        locals.var_t2__blk546_rv = 0.0;

        let (assign18070_e25615, assign18070_e25615_d_n0, assign18070_e25615_d_n2, assign18070_e25615_d_n6, assign18070_e25615_d_n7, assign18070_e25615_d_n10, assign18070_e25615_d_n11, assign18070_e25615_d_n12, assign18070_e25615_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18070_e25613: f64 = (1.0 / locals.var_t2__blk546);
        (assign18070_e25613, (-(locals.var_t2__blk546_dn0 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn2 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn6 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn7 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn10 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn11 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn12 / (locals.var_t2__blk546 * locals.var_t2__blk546))), (-(locals.var_t2__blk546_dn17 / (locals.var_t2__blk546 * locals.var_t2__blk546))),)
    } else {
        (locals.var_t1__blk547, locals.var_t1__blk547_dn0, locals.var_t1__blk547_dn2, locals.var_t1__blk547_dn6, locals.var_t1__blk547_dn7, locals.var_t1__blk547_dn10, locals.var_t1__blk547_dn11, locals.var_t1__blk547_dn12, locals.var_t1__blk547_dn17,)
    }
};
        locals.var_t1__blk547 = assign18070_e25615;
        locals.var_t1__blk547_dn0 = assign18070_e25615_d_n0;
        locals.var_t1__blk547_dn2 = assign18070_e25615_d_n2;
        locals.var_t1__blk547_dn6 = assign18070_e25615_d_n6;
        locals.var_t1__blk547_dn7 = assign18070_e25615_d_n7;
        locals.var_t1__blk547_dn10 = assign18070_e25615_d_n10;
        locals.var_t1__blk547_dn11 = assign18070_e25615_d_n11;
        locals.var_t1__blk547_dn12 = assign18070_e25615_d_n12;
        locals.var_t1__blk547_dn17 = assign18070_e25615_d_n17;
        locals.var_t1__blk547_rv = 0.0;

        let (assign18080_e25621, assign18080_e25621_d_n0, assign18080_e25621_d_n2, assign18080_e25621_d_n6, assign18080_e25621_d_n7, assign18080_e25621_d_n10, assign18080_e25621_d_n11, assign18080_e25621_d_n12, assign18080_e25621_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18080_e25619: f64 = (locals.var_idd * locals.var_t1__blk547);
        (assign18080_e25619, ((locals.var_idd_dn0 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn0)), ((locals.var_idd_dn2 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn2)), ((locals.var_idd_dn6 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn6)), ((locals.var_idd_dn7 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn7)), ((locals.var_idd_dn10 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn10)), ((locals.var_idd_dn11 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn11)), ((locals.var_idd_dn12 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn12)), ((locals.var_idd_dn17 * locals.var_t1__blk547) + (locals.var_idd * locals.var_t1__blk547_dn17)),)
    } else {
        (locals.var_ty__blk548, locals.var_ty__blk548_dn0, locals.var_ty__blk548_dn2, locals.var_ty__blk548_dn6, locals.var_ty__blk548_dn7, locals.var_ty__blk548_dn10, locals.var_ty__blk548_dn11, locals.var_ty__blk548_dn12, locals.var_ty__blk548_dn17,)
    }
};
        locals.var_ty__blk548 = assign18080_e25621;
        locals.var_ty__blk548_dn0 = assign18080_e25621_d_n0;
        locals.var_ty__blk548_dn2 = assign18080_e25621_d_n2;
        locals.var_ty__blk548_dn6 = assign18080_e25621_d_n6;
        locals.var_ty__blk548_dn7 = assign18080_e25621_d_n7;
        locals.var_ty__blk548_dn10 = assign18080_e25621_d_n10;
        locals.var_ty__blk548_dn11 = assign18080_e25621_d_n11;
        locals.var_ty__blk548_dn12 = assign18080_e25621_d_n12;
        locals.var_ty__blk548_dn17 = assign18080_e25621_d_n17;
        locals.var_ty__blk548_rv = 0.0;

        let (assign18090_e25629, assign18090_e25629_d_n0, assign18090_e25629_d_n2, assign18090_e25629_d_n6, assign18090_e25629_d_n7, assign18090_e25629_d_n10, assign18090_e25629_d_n11, assign18090_e25629_d_n12, assign18090_e25629_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18090_e25625: f64 = (0.2 * locals.var_vmaxe);
        let assign18090_e25627: f64 = (assign18090_e25625 / locals.var_muun);
        (assign18090_e25627, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn12) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn12)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn17) * locals.var_muun) - (assign18090_e25625 * locals.var_muun_dn17)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18090_e25629;
        locals.var_t2__blk546_dn0 = assign18090_e25629_d_n0;
        locals.var_t2__blk546_dn2 = assign18090_e25629_d_n2;
        locals.var_t2__blk546_dn6 = assign18090_e25629_d_n6;
        locals.var_t2__blk546_dn7 = assign18090_e25629_d_n7;
        locals.var_t2__blk546_dn10 = assign18090_e25629_d_n10;
        locals.var_t2__blk546_dn11 = assign18090_e25629_d_n11;
        locals.var_t2__blk546_dn12 = assign18090_e25629_d_n12;
        locals.var_t2__blk546_dn17 = assign18090_e25629_d_n17;
        locals.var_t2__blk546_rv = 0.0;

        let (assign18100_e25640, assign18100_e25640_d_n0, assign18100_e25640_d_n2, assign18100_e25640_d_n6, assign18100_e25640_d_n7, assign18100_e25640_d_n10, assign18100_e25640_d_n11, assign18100_e25640_d_n12, assign18100_e25640_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18100_e25633: f64 = (locals.var_ty__blk548 * locals.var_ty__blk548);
        let assign18100_e25636: f64 = (locals.var_t2__blk546 * locals.var_t2__blk546);
        let assign18100_e25637: f64 = (assign18100_e25633 + assign18100_e25636);
        let assign18100_e25638: f64 = (assign18100_e25637).sqrt();
        (assign18100_e25638, ((((locals.var_ty__blk548_dn0 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn0)) + ((locals.var_t2__blk546_dn0 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn0))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn2 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn2)) + ((locals.var_t2__blk546_dn2 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn2))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn6 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn6)) + ((locals.var_t2__blk546_dn6 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn6))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn7 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn7)) + ((locals.var_t2__blk546_dn7 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn7))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn10 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn10)) + ((locals.var_t2__blk546_dn10 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn10))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn11 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn11)) + ((locals.var_t2__blk546_dn11 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn11))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn12 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn12)) + ((locals.var_t2__blk546_dn12 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn12))) / (2.0 * assign18100_e25638)), ((((locals.var_ty__blk548_dn17 * locals.var_ty__blk548) + (locals.var_ty__blk548 * locals.var_ty__blk548_dn17)) + ((locals.var_t2__blk546_dn17 * locals.var_t2__blk546) + (locals.var_t2__blk546 * locals.var_t2__blk546_dn17))) / (2.0 * assign18100_e25638)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn12, locals.var_ey_dn17,)
    }
};
        locals.var_ey = assign18100_e25640;
        locals.var_ey_dn0 = assign18100_e25640_d_n0;
        locals.var_ey_dn2 = assign18100_e25640_d_n2;
        locals.var_ey_dn6 = assign18100_e25640_d_n6;
        locals.var_ey_dn7 = assign18100_e25640_d_n7;
        locals.var_ey_dn10 = assign18100_e25640_d_n10;
        locals.var_ey_dn11 = assign18100_e25640_d_n11;
        locals.var_ey_dn12 = assign18100_e25640_d_n12;
        locals.var_ey_dn17 = assign18100_e25640_d_n17;
        locals.var_ey_rv = 0.0;

        let (assign18110_e25646, assign18110_e25646_d_n0, assign18110_e25646_d_n2, assign18110_e25646_d_n6, assign18110_e25646_d_n7, assign18110_e25646_d_n10, assign18110_e25646_d_n11, assign18110_e25646_d_n12, assign18110_e25646_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18110_e25644: f64 = (locals.var_muun * locals.var_ey);
        (assign18110_e25644, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn12 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn12)), ((locals.var_muun_dn17 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn17)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn12, locals.var_em_dn17,)
    }
};
        locals.var_em = assign18110_e25646;
        locals.var_em_dn0 = assign18110_e25646_d_n0;
        locals.var_em_dn2 = assign18110_e25646_d_n2;
        locals.var_em_dn6 = assign18110_e25646_d_n6;
        locals.var_em_dn7 = assign18110_e25646_d_n7;
        locals.var_em_dn10 = assign18110_e25646_d_n10;
        locals.var_em_dn11 = assign18110_e25646_d_n11;
        locals.var_em_dn12 = assign18110_e25646_d_n12;
        locals.var_em_dn17 = assign18110_e25646_d_n17;
        locals.var_em_rv = 0.0;

        let (assign18120_e25652, assign18120_e25652_d_n0, assign18120_e25652_d_n2, assign18120_e25652_d_n6, assign18120_e25652_d_n7, assign18120_e25652_d_n10, assign18120_e25652_d_n11, assign18120_e25652_d_n12, assign18120_e25652_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18120_e25650: f64 = (locals.var_em / locals.var_vmaxe);
        (assign18120_e25650, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn12 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn12)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn17 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn17)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1__blk547, locals.var_t1__blk547_dn0, locals.var_t1__blk547_dn2, locals.var_t1__blk547_dn6, locals.var_t1__blk547_dn7, locals.var_t1__blk547_dn10, locals.var_t1__blk547_dn11, locals.var_t1__blk547_dn12, locals.var_t1__blk547_dn17,)
    }
};
        locals.var_t1__blk547 = assign18120_e25652;
        locals.var_t1__blk547_dn0 = assign18120_e25652_d_n0;
        locals.var_t1__blk547_dn2 = assign18120_e25652_d_n2;
        locals.var_t1__blk547_dn6 = assign18120_e25652_d_n6;
        locals.var_t1__blk547_dn7 = assign18120_e25652_d_n7;
        locals.var_t1__blk547_dn10 = assign18120_e25652_d_n10;
        locals.var_t1__blk547_dn11 = assign18120_e25652_d_n11;
        locals.var_t1__blk547_dn12 = assign18120_e25652_d_n12;
        locals.var_t1__blk547_dn17 = assign18120_e25652_d_n17;
        locals.var_t1__blk547_rv = 0.0;

        let assign18130_e25656: f64 = (10.0 * 2.220446049250313e-16);
        let assign18130_e25657: f64 = (1.0 - assign18130_e25656);
        let assign18130_e25664: f64 = (10.0 * 2.220446049250313e-16);
        let assign18130_e25665: f64 = (1.0 + assign18130_e25664);
        let assign18130_e25667: f64 = if ((assign18130_e25657 <= p.p113) && (p.p113 <= assign18130_e25665)) { 1.0 } else { 0.0 };
        locals.var_guard555 = assign18130_e25667;
        locals.var_guard555_rv = 0.0;

        let (assign18140_e25673, assign18140_e25673_d_n0, assign18140_e25673_d_n2, assign18140_e25673_d_n6, assign18140_e25673_d_n7, assign18140_e25673_d_n10, assign18140_e25673_d_n11, assign18140_e25673_d_n12, assign18140_e25673_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard555 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk550, locals.var_t3__blk550_dn0, locals.var_t3__blk550_dn2, locals.var_t3__blk550_dn6, locals.var_t3__blk550_dn7, locals.var_t3__blk550_dn10, locals.var_t3__blk550_dn11, locals.var_t3__blk550_dn12, locals.var_t3__blk550_dn17,)
    }
};
        locals.var_t3__blk550 = assign18140_e25673;
        locals.var_t3__blk550_dn0 = assign18140_e25673_d_n0;
        locals.var_t3__blk550_dn2 = assign18140_e25673_d_n2;
        locals.var_t3__blk550_dn6 = assign18140_e25673_d_n6;
        locals.var_t3__blk550_dn7 = assign18140_e25673_d_n7;
        locals.var_t3__blk550_dn10 = assign18140_e25673_d_n10;
        locals.var_t3__blk550_dn11 = assign18140_e25673_d_n11;
        locals.var_t3__blk550_dn12 = assign18140_e25673_d_n12;
        locals.var_t3__blk550_dn17 = assign18140_e25673_d_n17;
        locals.var_t3__blk550_rv = 0.0;

        let assign18150_e25677: f64 = (10.0 * 2.220446049250313e-16);
        let assign18150_e25678: f64 = (2.0 - assign18150_e25677);
        let assign18150_e25685: f64 = (10.0 * 2.220446049250313e-16);
        let assign18150_e25686: f64 = (2.0 + assign18150_e25685);
        let assign18150_e25688: f64 = if ((assign18150_e25678 <= p.p113) && (p.p113 <= assign18150_e25686)) { 1.0 } else { 0.0 };
        locals.var_guard556 = assign18150_e25688;
        locals.var_guard556_rv = 0.0;

        let (assign18160_e25697, assign18160_e25697_d_n0, assign18160_e25697_d_n2, assign18160_e25697_d_n6, assign18160_e25697_d_n7, assign18160_e25697_d_n10, assign18160_e25697_d_n11, assign18160_e25697_d_n12, assign18160_e25697_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard555 == 0.0)) && (locals.var_guard556 != 0.0)) {
        (locals.var_t1__blk547, locals.var_t1__blk547_dn0, locals.var_t1__blk547_dn2, locals.var_t1__blk547_dn6, locals.var_t1__blk547_dn7, locals.var_t1__blk547_dn10, locals.var_t1__blk547_dn11, locals.var_t1__blk547_dn12, locals.var_t1__blk547_dn17,)
    } else {
        (locals.var_t3__blk550, locals.var_t3__blk550_dn0, locals.var_t3__blk550_dn2, locals.var_t3__blk550_dn6, locals.var_t3__blk550_dn7, locals.var_t3__blk550_dn10, locals.var_t3__blk550_dn11, locals.var_t3__blk550_dn12, locals.var_t3__blk550_dn17,)
    }
};
        locals.var_t3__blk550 = assign18160_e25697;
        locals.var_t3__blk550_dn0 = assign18160_e25697_d_n0;
        locals.var_t3__blk550_dn2 = assign18160_e25697_d_n2;
        locals.var_t3__blk550_dn6 = assign18160_e25697_d_n6;
        locals.var_t3__blk550_dn7 = assign18160_e25697_d_n7;
        locals.var_t3__blk550_dn10 = assign18160_e25697_d_n10;
        locals.var_t3__blk550_dn11 = assign18160_e25697_d_n11;
        locals.var_t3__blk550_dn12 = assign18160_e25697_d_n12;
        locals.var_t3__blk550_dn17 = assign18160_e25697_d_n17;
        locals.var_t3__blk550_rv = 0.0;

        let (assign18170_e25711, assign18170_e25711_d_n0, assign18170_e25711_d_n2, assign18170_e25711_d_n6, assign18170_e25711_d_n7, assign18170_e25711_d_n10, assign18170_e25711_d_n11, assign18170_e25711_d_n12, assign18170_e25711_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard555 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign18170_e25708: f64 = (p.p113 - 1.0);
        let assign18170_e25709: f64 = (locals.var_t1__blk547).powf(assign18170_e25708);
        (assign18170_e25709, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn0)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn0 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn2)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn2 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn6)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn6 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn7)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn7 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn10)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn10 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn11)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn11 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn12)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn12 / locals.var_t1__blk547))) }, if 0.0 == 0.0 && ((assign18170_e25708) as f64).is_finite() && ((assign18170_e25708) as f64).fract() == 0.0 { if assign18170_e25708 == 0.0 { 0.0 } else { (assign18170_e25708 * ((locals.var_t1__blk547).powf(assign18170_e25708 - 1.0) * locals.var_t1__blk547_dn17)) } } else { (assign18170_e25709 * (assign18170_e25708 * (locals.var_t1__blk547_dn17 / locals.var_t1__blk547))) },)
    } else {
        (locals.var_t3__blk550, locals.var_t3__blk550_dn0, locals.var_t3__blk550_dn2, locals.var_t3__blk550_dn6, locals.var_t3__blk550_dn7, locals.var_t3__blk550_dn10, locals.var_t3__blk550_dn11, locals.var_t3__blk550_dn12, locals.var_t3__blk550_dn17,)
    }
};
        locals.var_t3__blk550 = assign18170_e25711;
        locals.var_t3__blk550_dn0 = assign18170_e25711_d_n0;
        locals.var_t3__blk550_dn2 = assign18170_e25711_d_n2;
        locals.var_t3__blk550_dn6 = assign18170_e25711_d_n6;
        locals.var_t3__blk550_dn7 = assign18170_e25711_d_n7;
        locals.var_t3__blk550_dn10 = assign18170_e25711_d_n10;
        locals.var_t3__blk550_dn11 = assign18170_e25711_d_n11;
        locals.var_t3__blk550_dn12 = assign18170_e25711_d_n12;
        locals.var_t3__blk550_dn17 = assign18170_e25711_d_n17;
        locals.var_t3__blk550_rv = 0.0;

        let (assign18180_e25717, assign18180_e25717_d_n0, assign18180_e25717_d_n2, assign18180_e25717_d_n6, assign18180_e25717_d_n7, assign18180_e25717_d_n10, assign18180_e25717_d_n11, assign18180_e25717_d_n12, assign18180_e25717_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18180_e25715: f64 = (locals.var_t1__blk547 * locals.var_t3__blk550);
        (assign18180_e25715, ((locals.var_t1__blk547_dn0 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn0)), ((locals.var_t1__blk547_dn2 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn2)), ((locals.var_t1__blk547_dn6 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn6)), ((locals.var_t1__blk547_dn7 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn7)), ((locals.var_t1__blk547_dn10 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn10)), ((locals.var_t1__blk547_dn11 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn11)), ((locals.var_t1__blk547_dn12 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn12)), ((locals.var_t1__blk547_dn17 * locals.var_t3__blk550) + (locals.var_t1__blk547 * locals.var_t3__blk550_dn17)),)
    } else {
        (locals.var_t2__blk546, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    }
};
        locals.var_t2__blk546 = assign18180_e25717;
        locals.var_t2__blk546_dn0 = assign18180_e25717_d_n0;
        locals.var_t2__blk546_dn2 = assign18180_e25717_d_n2;
        locals.var_t2__blk546_dn6 = assign18180_e25717_d_n6;
        locals.var_t2__blk546_dn7 = assign18180_e25717_d_n7;
        locals.var_t2__blk546_dn10 = assign18180_e25717_d_n10;
        locals.var_t2__blk546_dn11 = assign18180_e25717_d_n11;
        locals.var_t2__blk546_dn12 = assign18180_e25717_d_n12;
        locals.var_t2__blk546_dn17 = assign18180_e25717_d_n17;
        locals.var_t2__blk546_rv = 0.0;

        let (assign18190_e25723, assign18190_e25723_d_n0, assign18190_e25723_d_n2, assign18190_e25723_d_n6, assign18190_e25723_d_n7, assign18190_e25723_d_n10, assign18190_e25723_d_n11, assign18190_e25723_d_n12, assign18190_e25723_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18190_e25721: f64 = (1.0 + locals.var_t2__blk546);
        (assign18190_e25721, locals.var_t2__blk546_dn0, locals.var_t2__blk546_dn2, locals.var_t2__blk546_dn6, locals.var_t2__blk546_dn7, locals.var_t2__blk546_dn10, locals.var_t2__blk546_dn11, locals.var_t2__blk546_dn12, locals.var_t2__blk546_dn17,)
    } else {
        (locals.var_t4__blk551, locals.var_t4__blk551_dn0, locals.var_t4__blk551_dn2, locals.var_t4__blk551_dn6, locals.var_t4__blk551_dn7, locals.var_t4__blk551_dn10, locals.var_t4__blk551_dn11, locals.var_t4__blk551_dn12, locals.var_t4__blk551_dn17,)
    }
};
        locals.var_t4__blk551 = assign18190_e25723;
        locals.var_t4__blk551_dn0 = assign18190_e25723_d_n0;
        locals.var_t4__blk551_dn2 = assign18190_e25723_d_n2;
        locals.var_t4__blk551_dn6 = assign18190_e25723_d_n6;
        locals.var_t4__blk551_dn7 = assign18190_e25723_d_n7;
        locals.var_t4__blk551_dn10 = assign18190_e25723_d_n10;
        locals.var_t4__blk551_dn11 = assign18190_e25723_d_n11;
        locals.var_t4__blk551_dn12 = assign18190_e25723_d_n12;
        locals.var_t4__blk551_dn17 = assign18190_e25723_d_n17;
        locals.var_t4__blk551_rv = 0.0;

        let assign18200_e25727: f64 = (10.0 * 2.220446049250313e-16);
        let assign18200_e25728: f64 = (1.0 - assign18200_e25727);
        let assign18200_e25735: f64 = (10.0 * 2.220446049250313e-16);
        let assign18200_e25736: f64 = (1.0 + assign18200_e25735);
        let assign18200_e25738: f64 = if ((assign18200_e25728 <= p.p113) && (p.p113 <= assign18200_e25736)) { 1.0 } else { 0.0 };
        locals.var_guard557 = assign18200_e25738;
        locals.var_guard557_rv = 0.0;

        let (assign18210_e25746, assign18210_e25746_d_n0, assign18210_e25746_d_n2, assign18210_e25746_d_n6, assign18210_e25746_d_n7, assign18210_e25746_d_n10, assign18210_e25746_d_n11, assign18210_e25746_d_n12, assign18210_e25746_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard557 != 0.0)) {
        let assign18210_e25744: f64 = (1.0 / locals.var_t4__blk551);
        (assign18210_e25744, (-(locals.var_t4__blk551_dn0 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn2 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn6 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn7 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn10 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn11 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn12 / (locals.var_t4__blk551 * locals.var_t4__blk551))), (-(locals.var_t4__blk551_dn17 / (locals.var_t4__blk551 * locals.var_t4__blk551))),)
    } else {
        (locals.var_t5__blk552, locals.var_t5__blk552_dn0, locals.var_t5__blk552_dn2, locals.var_t5__blk552_dn6, locals.var_t5__blk552_dn7, locals.var_t5__blk552_dn10, locals.var_t5__blk552_dn11, locals.var_t5__blk552_dn12, locals.var_t5__blk552_dn17,)
    }
};
        locals.var_t5__blk552 = assign18210_e25746;
        locals.var_t5__blk552_dn0 = assign18210_e25746_d_n0;
        locals.var_t5__blk552_dn2 = assign18210_e25746_d_n2;
        locals.var_t5__blk552_dn6 = assign18210_e25746_d_n6;
        locals.var_t5__blk552_dn7 = assign18210_e25746_d_n7;
        locals.var_t5__blk552_dn10 = assign18210_e25746_d_n10;
        locals.var_t5__blk552_dn11 = assign18210_e25746_d_n11;
        locals.var_t5__blk552_dn12 = assign18210_e25746_d_n12;
        locals.var_t5__blk552_dn17 = assign18210_e25746_d_n17;
        locals.var_t5__blk552_rv = 0.0;

        let assign18220_e25750: f64 = (10.0 * 2.220446049250313e-16);
        let assign18220_e25751: f64 = (2.0 - assign18220_e25750);
        let assign18220_e25758: f64 = (10.0 * 2.220446049250313e-16);
        let assign18220_e25759: f64 = (2.0 + assign18220_e25758);
        let assign18220_e25761: f64 = if ((assign18220_e25751 <= p.p113) && (p.p113 <= assign18220_e25759)) { 1.0 } else { 0.0 };
        locals.var_guard558 = assign18220_e25761;
        locals.var_guard558_rv = 0.0;

        let (assign18230_e25773, assign18230_e25773_d_n0, assign18230_e25773_d_n2, assign18230_e25773_d_n6, assign18230_e25773_d_n7, assign18230_e25773_d_n10, assign18230_e25773_d_n11, assign18230_e25773_d_n12, assign18230_e25773_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 != 0.0)) {
        let assign18230_e25770: f64 = (locals.var_t4__blk551).sqrt();
        let assign18230_e25771: f64 = (1.0 / assign18230_e25770);
        (assign18230_e25771, (-((locals.var_t4__blk551_dn0 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn2 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn6 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn7 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn10 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn11 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn12 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))), (-((locals.var_t4__blk551_dn17 / (2.0 * assign18230_e25770)) / (assign18230_e25770 * assign18230_e25770))),)
    } else {
        (locals.var_t5__blk552, locals.var_t5__blk552_dn0, locals.var_t5__blk552_dn2, locals.var_t5__blk552_dn6, locals.var_t5__blk552_dn7, locals.var_t5__blk552_dn10, locals.var_t5__blk552_dn11, locals.var_t5__blk552_dn12, locals.var_t5__blk552_dn17,)
    }
};
        locals.var_t5__blk552 = assign18230_e25773;
        locals.var_t5__blk552_dn0 = assign18230_e25773_d_n0;
        locals.var_t5__blk552_dn2 = assign18230_e25773_d_n2;
        locals.var_t5__blk552_dn6 = assign18230_e25773_d_n6;
        locals.var_t5__blk552_dn7 = assign18230_e25773_d_n7;
        locals.var_t5__blk552_dn10 = assign18230_e25773_d_n10;
        locals.var_t5__blk552_dn11 = assign18230_e25773_d_n11;
        locals.var_t5__blk552_dn12 = assign18230_e25773_d_n12;
        locals.var_t5__blk552_dn17 = assign18230_e25773_d_n17;
        locals.var_t5__blk552_rv = 0.0;

        let (assign18240_e25790, assign18240_e25790_d_n0, assign18240_e25790_d_n2, assign18240_e25790_d_n6, assign18240_e25790_d_n7, assign18240_e25790_d_n10, assign18240_e25790_d_n11, assign18240_e25790_d_n12, assign18240_e25790_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign18240_e25783: f64 = (-1.0);
        let assign18240_e25785: f64 = (assign18240_e25783 / p.p113);
        let assign18240_e25787: f64 = (assign18240_e25785 - 1.0);
        let assign18240_e25788: f64 = (locals.var_t4__blk551).powf(assign18240_e25787);
        (assign18240_e25788, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn0)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn0 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn2)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn2 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn6)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn6 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn7)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn7 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn10)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn10 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn11)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn11 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn12)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn12 / locals.var_t4__blk551))) }, if 0.0 == 0.0 && ((assign18240_e25787) as f64).is_finite() && ((assign18240_e25787) as f64).fract() == 0.0 { if assign18240_e25787 == 0.0 { 0.0 } else { (assign18240_e25787 * ((locals.var_t4__blk551).powf(assign18240_e25787 - 1.0) * locals.var_t4__blk551_dn17)) } } else { (assign18240_e25788 * (assign18240_e25787 * (locals.var_t4__blk551_dn17 / locals.var_t4__blk551))) },)
    } else {
        (locals.var_t6__blk553, locals.var_t6__blk553_dn0, locals.var_t6__blk553_dn2, locals.var_t6__blk553_dn6, locals.var_t6__blk553_dn7, locals.var_t6__blk553_dn10, locals.var_t6__blk553_dn11, locals.var_t6__blk553_dn12, locals.var_t6__blk553_dn17,)
    }
};
        locals.var_t6__blk553 = assign18240_e25790;
        locals.var_t6__blk553_dn0 = assign18240_e25790_d_n0;
        locals.var_t6__blk553_dn2 = assign18240_e25790_d_n2;
        locals.var_t6__blk553_dn6 = assign18240_e25790_d_n6;
        locals.var_t6__blk553_dn7 = assign18240_e25790_d_n7;
        locals.var_t6__blk553_dn10 = assign18240_e25790_d_n10;
        locals.var_t6__blk553_dn11 = assign18240_e25790_d_n11;
        locals.var_t6__blk553_dn12 = assign18240_e25790_d_n12;
        locals.var_t6__blk553_dn17 = assign18240_e25790_d_n17;
        locals.var_t6__blk553_rv = 0.0;

        let (assign18250_e25802, assign18250_e25802_d_n0, assign18250_e25802_d_n2, assign18250_e25802_d_n6, assign18250_e25802_d_n7, assign18250_e25802_d_n10, assign18250_e25802_d_n11, assign18250_e25802_d_n12, assign18250_e25802_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign18250_e25800: f64 = (locals.var_t4__blk551 * locals.var_t6__blk553);
        (assign18250_e25800, ((locals.var_t4__blk551_dn0 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn0)), ((locals.var_t4__blk551_dn2 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn2)), ((locals.var_t4__blk551_dn6 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn6)), ((locals.var_t4__blk551_dn7 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn7)), ((locals.var_t4__blk551_dn10 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn10)), ((locals.var_t4__blk551_dn11 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn11)), ((locals.var_t4__blk551_dn12 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn12)), ((locals.var_t4__blk551_dn17 * locals.var_t6__blk553) + (locals.var_t4__blk551 * locals.var_t6__blk553_dn17)),)
    } else {
        (locals.var_t5__blk552, locals.var_t5__blk552_dn0, locals.var_t5__blk552_dn2, locals.var_t5__blk552_dn6, locals.var_t5__blk552_dn7, locals.var_t5__blk552_dn10, locals.var_t5__blk552_dn11, locals.var_t5__blk552_dn12, locals.var_t5__blk552_dn17,)
    }
};
        locals.var_t5__blk552 = assign18250_e25802;
        locals.var_t5__blk552_dn0 = assign18250_e25802_d_n0;
        locals.var_t5__blk552_dn2 = assign18250_e25802_d_n2;
        locals.var_t5__blk552_dn6 = assign18250_e25802_d_n6;
        locals.var_t5__blk552_dn7 = assign18250_e25802_d_n7;
        locals.var_t5__blk552_dn10 = assign18250_e25802_d_n10;
        locals.var_t5__blk552_dn11 = assign18250_e25802_d_n11;
        locals.var_t5__blk552_dn12 = assign18250_e25802_d_n12;
        locals.var_t5__blk552_dn17 = assign18250_e25802_d_n17;
        locals.var_t5__blk552_rv = 0.0;

        let (assign18260_e25808, assign18260_e25808_d_n0, assign18260_e25808_d_n2, assign18260_e25808_d_n6, assign18260_e25808_d_n7, assign18260_e25808_d_n10, assign18260_e25808_d_n11, assign18260_e25808_d_n12, assign18260_e25808_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18260_e25806: f64 = (locals.var_muun * locals.var_t5__blk552);
        (assign18260_e25806, ((locals.var_muun_dn0 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn0)), ((locals.var_muun_dn2 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn2)), ((locals.var_muun_dn6 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn6)), ((locals.var_muun_dn7 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn7)), ((locals.var_muun_dn10 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn10)), ((locals.var_muun_dn11 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn11)), ((locals.var_muun_dn12 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn12)), ((locals.var_muun_dn17 * locals.var_t5__blk552) + (locals.var_muun * locals.var_t5__blk552_dn17)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn12, locals.var_mu_dn17,)
    }
};
        locals.var_mu = assign18260_e25808;
        locals.var_mu_dn0 = assign18260_e25808_d_n0;
        locals.var_mu_dn2 = assign18260_e25808_d_n2;
        locals.var_mu_dn6 = assign18260_e25808_d_n6;
        locals.var_mu_dn7 = assign18260_e25808_d_n7;
        locals.var_mu_dn10 = assign18260_e25808_d_n10;
        locals.var_mu_dn11 = assign18260_e25808_d_n11;
        locals.var_mu_dn12 = assign18260_e25808_d_n12;
        locals.var_mu_dn17 = assign18260_e25808_d_n17;
        locals.var_mu_rv = 0.0;

        let (assign18270_e25818, assign18270_e25818_d_n0, assign18270_e25818_d_n2, assign18270_e25818_d_n6, assign18270_e25818_d_n7, assign18270_e25818_d_n10, assign18270_e25818_d_n11, assign18270_e25818_d_n12, assign18270_e25818_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18270_e25812: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign18270_e25815: f64 = (locals.var_leff - locals.var_lred);
        let assign18270_e25816: f64 = (assign18270_e25812 / assign18270_e25815);
        (assign18270_e25816, (-((assign18270_e25812 * (-locals.var_lred_dn0)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn2)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn6)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn7)) / (assign18270_e25815 * assign18270_e25815))), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * assign18270_e25815) - (assign18270_e25812 * (-locals.var_lred_dn10))) / (assign18270_e25815 * assign18270_e25815)), (-((assign18270_e25812 * (-locals.var_lred_dn11)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn12)) / (assign18270_e25815 * assign18270_e25815))), (-((assign18270_e25812 * (-locals.var_lred_dn17)) / (assign18270_e25815 * assign18270_e25815))),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn12, locals.var_betawl_dn17,)
    }
};
        locals.var_betawl = assign18270_e25818;
        locals.var_betawl_dn0 = assign18270_e25818_d_n0;
        locals.var_betawl_dn2 = assign18270_e25818_d_n2;
        locals.var_betawl_dn6 = assign18270_e25818_d_n6;
        locals.var_betawl_dn7 = assign18270_e25818_d_n7;
        locals.var_betawl_dn10 = assign18270_e25818_d_n10;
        locals.var_betawl_dn11 = assign18270_e25818_d_n11;
        locals.var_betawl_dn12 = assign18270_e25818_d_n12;
        locals.var_betawl_dn17 = assign18270_e25818_d_n17;
        locals.var_betawl_rv = 0.0;

        let (assign18280_e25826, assign18280_e25826_d_n0, assign18280_e25826_d_n2, assign18280_e25826_d_n6, assign18280_e25826_d_n7, assign18280_e25826_d_n10, assign18280_e25826_d_n11, assign18280_e25826_d_n12, assign18280_e25826_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18280_e25822: f64 = (locals.var_betawl * locals.var_idd);
        let assign18280_e25824: f64 = (assign18280_e25822 * locals.var_mu);
        (assign18280_e25824, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn12)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn17)) * locals.var_mu) + (assign18280_e25822 * locals.var_mu_dn17)),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn12, locals.var_ids0_dn17,)
    }
};
        locals.var_ids0 = assign18280_e25826;
        locals.var_ids0_dn0 = assign18280_e25826_d_n0;
        locals.var_ids0_dn2 = assign18280_e25826_d_n2;
        locals.var_ids0_dn6 = assign18280_e25826_d_n6;
        locals.var_ids0_dn7 = assign18280_e25826_d_n7;
        locals.var_ids0_dn10 = assign18280_e25826_d_n10;
        locals.var_ids0_dn11 = assign18280_e25826_d_n11;
        locals.var_ids0_dn12 = assign18280_e25826_d_n12;
        locals.var_ids0_dn17 = assign18280_e25826_d_n17;
        locals.var_ids0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18290_e25830, assign18290_e25830_d_n0, assign18290_e25830_d_n2, assign18290_e25830_d_n6, assign18290_e25830_d_n7, assign18290_e25830_d_n10, assign18290_e25830_d_n11, assign18290_e25830_d_n12, assign18290_e25830_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18290_e25830;
        locals.var_idspt_dn0 = assign18290_e25830_d_n0;
        locals.var_idspt_dn2 = assign18290_e25830_d_n2;
        locals.var_idspt_dn6 = assign18290_e25830_d_n6;
        locals.var_idspt_dn7 = assign18290_e25830_d_n7;
        locals.var_idspt_dn10 = assign18290_e25830_d_n10;
        locals.var_idspt_dn11 = assign18290_e25830_d_n11;
        locals.var_idspt_dn12 = assign18290_e25830_d_n12;
        locals.var_idspt_dn17 = assign18290_e25830_d_n17;
        locals.var_idspt_rv = 0.0;

        let assign18300_e25837: f64 = if ((p.p281 > 0.0) && (p.p244 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard568 = assign18300_e25837;
        locals.var_guard568_rv = 0.0;

        let (assign18310_e25847, assign18310_e25847_d_n0, assign18310_e25847_d_n2, assign18310_e25847_d_n6, assign18310_e25847_d_n7, assign18310_e25847_d_n10, assign18310_e25847_d_n11, assign18310_e25847_d_n12, assign18310_e25847_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18310_e25844: f64 = (locals.var_vds - locals.var_pds);
        let assign18310_e25845: f64 = (0.5 * assign18310_e25844);
        (assign18310_e25845, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn12 - locals.var_pds_dn12)), (0.5 * (locals.var_vds_dn17 - locals.var_pds_dn17)),)
    } else {
        (locals.var_t1__blk559, locals.var_t1__blk559_dn0, locals.var_t1__blk559_dn2, locals.var_t1__blk559_dn6, locals.var_t1__blk559_dn7, locals.var_t1__blk559_dn10, locals.var_t1__blk559_dn11, locals.var_t1__blk559_dn12, locals.var_t1__blk559_dn17,)
    }
};
        locals.var_t1__blk559 = assign18310_e25847;
        locals.var_t1__blk559_dn0 = assign18310_e25847_d_n0;
        locals.var_t1__blk559_dn2 = assign18310_e25847_d_n2;
        locals.var_t1__blk559_dn6 = assign18310_e25847_d_n6;
        locals.var_t1__blk559_dn7 = assign18310_e25847_d_n7;
        locals.var_t1__blk559_dn10 = assign18310_e25847_d_n10;
        locals.var_t1__blk559_dn11 = assign18310_e25847_d_n11;
        locals.var_t1__blk559_dn12 = assign18310_e25847_d_n12;
        locals.var_t1__blk559_dn17 = assign18310_e25847_d_n17;
        locals.var_t1__blk559_rv = 0.0;

        let (assign18320_e25857, assign18320_e25857_d_n0, assign18320_e25857_d_n2, assign18320_e25857_d_n6, assign18320_e25857_d_n7, assign18320_e25857_d_n10, assign18320_e25857_d_n11, assign18320_e25857_d_n12, assign18320_e25857_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18320_e25853: f64 = (2.0 * locals.var_t1__blk559);
        let assign18320_e25855: f64 = (assign18320_e25853 / 0.01);
        (assign18320_e25855, ((2.0 * locals.var_t1__blk559_dn0) / 0.01), ((2.0 * locals.var_t1__blk559_dn2) / 0.01), ((2.0 * locals.var_t1__blk559_dn6) / 0.01), ((2.0 * locals.var_t1__blk559_dn7) / 0.01), ((2.0 * locals.var_t1__blk559_dn10) / 0.01), ((2.0 * locals.var_t1__blk559_dn11) / 0.01), ((2.0 * locals.var_t1__blk559_dn12) / 0.01), ((2.0 * locals.var_t1__blk559_dn17) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18320_e25857;
        locals.var_tmf1_dn0 = assign18320_e25857_d_n0;
        locals.var_tmf1_dn2 = assign18320_e25857_d_n2;
        locals.var_tmf1_dn6 = assign18320_e25857_d_n6;
        locals.var_tmf1_dn7 = assign18320_e25857_d_n7;
        locals.var_tmf1_dn10 = assign18320_e25857_d_n10;
        locals.var_tmf1_dn11 = assign18320_e25857_d_n11;
        locals.var_tmf1_dn12 = assign18320_e25857_d_n12;
        locals.var_tmf1_dn17 = assign18320_e25857_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18330_e25899, assign18330_e25899_d_n0, assign18330_e25899_d_n2, assign18330_e25899_d_n6, assign18330_e25899_d_n7, assign18330_e25899_d_n10, assign18330_e25899_d_n11, assign18330_e25899_d_n12, assign18330_e25899_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18330_e25865: f64 = (1.0 / 2.0);
        let assign18330_e25869: f64 = (1.0 / 6.0);
        let assign18330_e25873: f64 = (1.0 / 24.0);
        let assign18330_e25877: f64 = (1.0 / 120.0);
        let assign18330_e25881: f64 = (1.0 / 720.0);
        let assign18330_e25885: f64 = (1.0 / 5040.0);
        let assign18330_e25886: f64 = (locals.var_tmf1 * assign18330_e25885);
        let assign18330_e25887: f64 = (assign18330_e25881 + assign18330_e25886);
        let assign18330_e25888: f64 = (locals.var_tmf1 * assign18330_e25887);
        let assign18330_e25889: f64 = (assign18330_e25877 + assign18330_e25888);
        let assign18330_e25890: f64 = (locals.var_tmf1 * assign18330_e25889);
        let assign18330_e25891: f64 = (assign18330_e25873 + assign18330_e25890);
        let assign18330_e25892: f64 = (locals.var_tmf1 * assign18330_e25891);
        let assign18330_e25893: f64 = (assign18330_e25869 + assign18330_e25892);
        let assign18330_e25894: f64 = (locals.var_tmf1 * assign18330_e25893);
        let assign18330_e25895: f64 = (assign18330_e25865 + assign18330_e25894);
        let assign18330_e25896: f64 = (locals.var_tmf1 * assign18330_e25895);
        let assign18330_e25897: f64 = (1.0 + assign18330_e25896);
        (assign18330_e25897, ((locals.var_tmf1_dn0 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn2 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn6 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn7 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn10 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn11 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn12 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign18330_e25885))))))))))), ((locals.var_tmf1_dn17 * assign18330_e25895) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25893) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25891) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25889) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign18330_e25887) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign18330_e25885))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign18330_e25899;
        locals.var_tmf2_dn0 = assign18330_e25899_d_n0;
        locals.var_tmf2_dn2 = assign18330_e25899_d_n2;
        locals.var_tmf2_dn6 = assign18330_e25899_d_n6;
        locals.var_tmf2_dn7 = assign18330_e25899_d_n7;
        locals.var_tmf2_dn10 = assign18330_e25899_d_n10;
        locals.var_tmf2_dn11 = assign18330_e25899_d_n11;
        locals.var_tmf2_dn12 = assign18330_e25899_d_n12;
        locals.var_tmf2_dn17 = assign18330_e25899_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign18340_e25907, assign18340_e25907_d_n0, assign18340_e25907_d_n2, assign18340_e25907_d_n6, assign18340_e25907_d_n7, assign18340_e25907_d_n10, assign18340_e25907_d_n11, assign18340_e25907_d_n12, assign18340_e25907_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18340_e25905: f64 = (0.01 / locals.var_tmf2);
        (assign18340_e25905, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6__blk565, locals.var_t6__blk565_dn0, locals.var_t6__blk565_dn2, locals.var_t6__blk565_dn6, locals.var_t6__blk565_dn7, locals.var_t6__blk565_dn10, locals.var_t6__blk565_dn11, locals.var_t6__blk565_dn12, locals.var_t6__blk565_dn17,)
    }
};
        locals.var_t6__blk565 = assign18340_e25907;
        locals.var_t6__blk565_dn0 = assign18340_e25907_d_n0;
        locals.var_t6__blk565_dn2 = assign18340_e25907_d_n2;
        locals.var_t6__blk565_dn6 = assign18340_e25907_d_n6;
        locals.var_t6__blk565_dn7 = assign18340_e25907_d_n7;
        locals.var_t6__blk565_dn10 = assign18340_e25907_d_n10;
        locals.var_t6__blk565_dn11 = assign18340_e25907_d_n11;
        locals.var_t6__blk565_dn12 = assign18340_e25907_d_n12;
        locals.var_t6__blk565_dn17 = assign18340_e25907_d_n17;
        locals.var_t6__blk565_rv = 0.0;

        let (assign18350_e25917, assign18350_e25917_d_n0, assign18350_e25917_d_n2, assign18350_e25917_d_n6, assign18350_e25917_d_n7, assign18350_e25917_d_n10, assign18350_e25917_d_n11, assign18350_e25917_d_n12, assign18350_e25917_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18350_e25914: f64 = (locals.var_ps0 + locals.var_t6__blk565);
        let assign18350_e25915: f64 = (1.1 - assign18350_e25914);
        (assign18350_e25915, (-(locals.var_ps0_dn0 + locals.var_t6__blk565_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6__blk565_dn2)), (-(locals.var_ps0_dn6 + locals.var_t6__blk565_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6__blk565_dn7)), (-(locals.var_ps0_dn10 + locals.var_t6__blk565_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6__blk565_dn11)), (-(locals.var_ps0_dn12 + locals.var_t6__blk565_dn12)), (-(locals.var_ps0_dn17 + locals.var_t6__blk565_dn17)),)
    } else {
        (locals.var_t1__blk559, locals.var_t1__blk559_dn0, locals.var_t1__blk559_dn2, locals.var_t1__blk559_dn6, locals.var_t1__blk559_dn7, locals.var_t1__blk559_dn10, locals.var_t1__blk559_dn11, locals.var_t1__blk559_dn12, locals.var_t1__blk559_dn17,)
    }
};
        locals.var_t1__blk559 = assign18350_e25917;
        locals.var_t1__blk559_dn0 = assign18350_e25917_d_n0;
        locals.var_t1__blk559_dn2 = assign18350_e25917_d_n2;
        locals.var_t1__blk559_dn6 = assign18350_e25917_d_n6;
        locals.var_t1__blk559_dn7 = assign18350_e25917_d_n7;
        locals.var_t1__blk559_dn10 = assign18350_e25917_d_n10;
        locals.var_t1__blk559_dn11 = assign18350_e25917_d_n11;
        locals.var_t1__blk559_dn12 = assign18350_e25917_d_n12;
        locals.var_t1__blk559_dn17 = assign18350_e25917_d_n17;
        locals.var_t1__blk559_rv = 0.0;

        let (assign18360_e25932, assign18360_e25932_d_n0, assign18360_e25932_d_n2, assign18360_e25932_d_n6, assign18360_e25932_d_n7, assign18360_e25932_d_n10, assign18360_e25932_d_n11, assign18360_e25932_d_n12, assign18360_e25932_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18360_e25923: f64 = (locals.var_t1__blk559 * locals.var_t1__blk559);
        let assign18360_e25926: f64 = (4.0 * 0.05);
        let assign18360_e25928: f64 = (assign18360_e25926 * 0.05);
        let assign18360_e25929: f64 = (assign18360_e25923 + assign18360_e25928);
        let assign18360_e25930: f64 = (assign18360_e25929).sqrt();
        (assign18360_e25930, (((locals.var_t1__blk559_dn0 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn0)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn2 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn2)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn6 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn6)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn7 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn7)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn10 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn10)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn11 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn11)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn12 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn12)) / (2.0 * assign18360_e25930)), (((locals.var_t1__blk559_dn17 * locals.var_t1__blk559) + (locals.var_t1__blk559 * locals.var_t1__blk559_dn17)) / (2.0 * assign18360_e25930)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18360_e25932;
        locals.var_tmf1_dn0 = assign18360_e25932_d_n0;
        locals.var_tmf1_dn2 = assign18360_e25932_d_n2;
        locals.var_tmf1_dn6 = assign18360_e25932_d_n6;
        locals.var_tmf1_dn7 = assign18360_e25932_d_n7;
        locals.var_tmf1_dn10 = assign18360_e25932_d_n10;
        locals.var_tmf1_dn11 = assign18360_e25932_d_n11;
        locals.var_tmf1_dn12 = assign18360_e25932_d_n12;
        locals.var_tmf1_dn17 = assign18360_e25932_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18370_e25946, assign18370_e25946_d_n0, assign18370_e25946_d_n2, assign18370_e25946_d_n6, assign18370_e25946_d_n7, assign18370_e25946_d_n10, assign18370_e25946_d_n11, assign18370_e25946_d_n12, assign18370_e25946_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18370_e25939: f64 = (locals.var_t1__blk559 + locals.var_tmf1);
        let assign18370_e25940: f64 = (0.5 * assign18370_e25939);
        let assign18370_e25943: f64 = (1e-10 * 0.05);
        let assign18370_e25944: f64 = (assign18370_e25940 + assign18370_e25943);
        (assign18370_e25944, (0.5 * (locals.var_t1__blk559_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1__blk559_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1__blk559_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1__blk559_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1__blk559_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1__blk559_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1__blk559_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1__blk559_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk567, locals.var_t2__blk567_dn0, locals.var_t2__blk567_dn2, locals.var_t2__blk567_dn6, locals.var_t2__blk567_dn7, locals.var_t2__blk567_dn10, locals.var_t2__blk567_dn11, locals.var_t2__blk567_dn12, locals.var_t2__blk567_dn17,)
    }
};
        locals.var_t2__blk567 = assign18370_e25946;
        locals.var_t2__blk567_dn0 = assign18370_e25946_d_n0;
        locals.var_t2__blk567_dn2 = assign18370_e25946_d_n2;
        locals.var_t2__blk567_dn6 = assign18370_e25946_d_n6;
        locals.var_t2__blk567_dn7 = assign18370_e25946_d_n7;
        locals.var_t2__blk567_dn10 = assign18370_e25946_d_n10;
        locals.var_t2__blk567_dn11 = assign18370_e25946_d_n11;
        locals.var_t2__blk567_dn12 = assign18370_e25946_d_n12;
        locals.var_t2__blk567_dn17 = assign18370_e25946_d_n17;
        locals.var_t2__blk567_rv = 0.0;

        let assign18380_e25949: f64 = if locals.var_t2__blk567 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign18380_e25949;
        locals.var_guard569_rv = 0.0;

        let (assign18390_e25957, assign18390_e25957_d_n0, assign18390_e25957_d_n2, assign18390_e25957_d_n6, assign18390_e25957_d_n7, assign18390_e25957_d_n10, assign18390_e25957_d_n11, assign18390_e25957_d_n12, assign18390_e25957_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk567, locals.var_t2__blk567_dn0, locals.var_t2__blk567_dn2, locals.var_t2__blk567_dn6, locals.var_t2__blk567_dn7, locals.var_t2__blk567_dn10, locals.var_t2__blk567_dn11, locals.var_t2__blk567_dn12, locals.var_t2__blk567_dn17,)
    }
};
        locals.var_t2__blk567 = assign18390_e25957;
        locals.var_t2__blk567_dn0 = assign18390_e25957_d_n0;
        locals.var_t2__blk567_dn2 = assign18390_e25957_d_n2;
        locals.var_t2__blk567_dn6 = assign18390_e25957_d_n6;
        locals.var_t2__blk567_dn7 = assign18390_e25957_d_n7;
        locals.var_t2__blk567_dn10 = assign18390_e25957_d_n10;
        locals.var_t2__blk567_dn11 = assign18390_e25957_d_n11;
        locals.var_t2__blk567_dn12 = assign18390_e25957_d_n12;
        locals.var_t2__blk567_dn17 = assign18390_e25957_d_n17;
        locals.var_t2__blk567_rv = 0.0;

        let (assign18400_e25965, assign18400_e25965_d_n0, assign18400_e25965_d_n2, assign18400_e25965_d_n6, assign18400_e25965_d_n7, assign18400_e25965_d_n10, assign18400_e25965_d_n11, assign18400_e25965_d_n12, assign18400_e25965_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18400_e25963: f64 = (locals.var_beta * locals.var_ptl0);
        (assign18400_e25963, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_ptl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk560, locals.var_t0__blk560_dn0, locals.var_t0__blk560_dn2, locals.var_t0__blk560_dn6, locals.var_t0__blk560_dn7, locals.var_t0__blk560_dn10, locals.var_t0__blk560_dn11, locals.var_t0__blk560_dn12, locals.var_t0__blk560_dn17,)
    }
};
        locals.var_t0__blk560 = assign18400_e25965;
        locals.var_t0__blk560_dn0 = assign18400_e25965_d_n0;
        locals.var_t0__blk560_dn2 = assign18400_e25965_d_n2;
        locals.var_t0__blk560_dn6 = assign18400_e25965_d_n6;
        locals.var_t0__blk560_dn7 = assign18400_e25965_d_n7;
        locals.var_t0__blk560_dn10 = assign18400_e25965_d_n10;
        locals.var_t0__blk560_dn11 = assign18400_e25965_d_n11;
        locals.var_t0__blk560_dn12 = assign18400_e25965_d_n12;
        locals.var_t0__blk560_dn17 = assign18400_e25965_d_n17;
        locals.var_t0__blk560_rv = 0.0;

        let (assign18410_e25973, assign18410_e25973_d_n0, assign18410_e25973_d_n2, assign18410_e25973_d_n6, assign18410_e25973_d_n7, assign18410_e25973_d_n10, assign18410_e25973_d_n11, assign18410_e25973_d_n12, assign18410_e25973_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18410_e25971: f64 = (locals.var_c_fox * locals.var_t0__blk560);
        (assign18410_e25971, ((locals.var_c_fox_dn0 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn0)), ((locals.var_c_fox_dn2 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn2)), ((locals.var_c_fox_dn6 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn6)), ((locals.var_c_fox_dn7 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn7)), ((locals.var_c_fox_dn10 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn10)), ((locals.var_c_fox_dn11 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn11)), ((locals.var_c_fox_dn12 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn12)), ((locals.var_c_fox_dn17 * locals.var_t0__blk560) + (locals.var_c_fox * locals.var_t0__blk560_dn17)),)
    } else {
        (locals.var_t3__blk561, locals.var_t3__blk561_dn0, locals.var_t3__blk561_dn2, locals.var_t3__blk561_dn6, locals.var_t3__blk561_dn7, locals.var_t3__blk561_dn10, locals.var_t3__blk561_dn11, locals.var_t3__blk561_dn12, locals.var_t3__blk561_dn17,)
    }
};
        locals.var_t3__blk561 = assign18410_e25973;
        locals.var_t3__blk561_dn0 = assign18410_e25973_d_n0;
        locals.var_t3__blk561_dn2 = assign18410_e25973_d_n2;
        locals.var_t3__blk561_dn6 = assign18410_e25973_d_n6;
        locals.var_t3__blk561_dn7 = assign18410_e25973_d_n7;
        locals.var_t3__blk561_dn10 = assign18410_e25973_d_n10;
        locals.var_t3__blk561_dn11 = assign18410_e25973_d_n11;
        locals.var_t3__blk561_dn12 = assign18410_e25973_d_n12;
        locals.var_t3__blk561_dn17 = assign18410_e25973_d_n17;
        locals.var_t3__blk561_rv = 0.0;

        let (assign18420_e25981, assign18420_e25981_d_n0, assign18420_e25981_d_n2, assign18420_e25981_d_n6, assign18420_e25981_d_n7, assign18420_e25981_d_n10, assign18420_e25981_d_n11, assign18420_e25981_d_n12, assign18420_e25981_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18420_e25979: f64 = (locals.var_t2__blk567).powf(p.p245);
        (assign18420_e25979, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn0)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn0 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn2)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn2 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn6)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn6 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn7)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn7 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn10)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn10 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn11)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn11 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn12)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn12 / locals.var_t2__blk567))) }, if 0.0 == 0.0 && ((p.p245) as f64).is_finite() && ((p.p245) as f64).fract() == 0.0 { if p.p245 == 0.0 { 0.0 } else { (p.p245 * ((locals.var_t2__blk567).powf(p.p245 - 1.0) * locals.var_t2__blk567_dn17)) } } else { (assign18420_e25979 * (p.p245 * (locals.var_t2__blk567_dn17 / locals.var_t2__blk567))) },)
    } else {
        (locals.var_t0__blk560, locals.var_t0__blk560_dn0, locals.var_t0__blk560_dn2, locals.var_t0__blk560_dn6, locals.var_t0__blk560_dn7, locals.var_t0__blk560_dn10, locals.var_t0__blk560_dn11, locals.var_t0__blk560_dn12, locals.var_t0__blk560_dn17,)
    }
};
        locals.var_t0__blk560 = assign18420_e25981;
        locals.var_t0__blk560_dn0 = assign18420_e25981_d_n0;
        locals.var_t0__blk560_dn2 = assign18420_e25981_d_n2;
        locals.var_t0__blk560_dn6 = assign18420_e25981_d_n6;
        locals.var_t0__blk560_dn7 = assign18420_e25981_d_n7;
        locals.var_t0__blk560_dn10 = assign18420_e25981_d_n10;
        locals.var_t0__blk560_dn11 = assign18420_e25981_d_n11;
        locals.var_t0__blk560_dn12 = assign18420_e25981_d_n12;
        locals.var_t0__blk560_dn17 = assign18420_e25981_d_n17;
        locals.var_t0__blk560_rv = 0.0;

        let (assign18430_e25989, assign18430_e25989_d_n0, assign18430_e25989_d_n2, assign18430_e25989_d_n6, assign18430_e25989_d_n7, assign18430_e25989_d_n10, assign18430_e25989_d_n11, assign18430_e25989_d_n12, assign18430_e25989_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18430_e25987: f64 = (locals.var_t3__blk561 * locals.var_t0__blk560);
        (assign18430_e25987, ((locals.var_t3__blk561_dn0 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn0)), ((locals.var_t3__blk561_dn2 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn2)), ((locals.var_t3__blk561_dn6 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn6)), ((locals.var_t3__blk561_dn7 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn7)), ((locals.var_t3__blk561_dn10 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn10)), ((locals.var_t3__blk561_dn11 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn11)), ((locals.var_t3__blk561_dn12 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn12)), ((locals.var_t3__blk561_dn17 * locals.var_t0__blk560) + (locals.var_t3__blk561 * locals.var_t0__blk560_dn17)),)
    } else {
        (locals.var_t9__blk562, locals.var_t9__blk562_dn0, locals.var_t9__blk562_dn2, locals.var_t9__blk562_dn6, locals.var_t9__blk562_dn7, locals.var_t9__blk562_dn10, locals.var_t9__blk562_dn11, locals.var_t9__blk562_dn12, locals.var_t9__blk562_dn17,)
    }
};
        locals.var_t9__blk562 = assign18430_e25989;
        locals.var_t9__blk562_dn0 = assign18430_e25989_d_n0;
        locals.var_t9__blk562_dn2 = assign18430_e25989_d_n2;
        locals.var_t9__blk562_dn6 = assign18430_e25989_d_n6;
        locals.var_t9__blk562_dn7 = assign18430_e25989_d_n7;
        locals.var_t9__blk562_dn10 = assign18430_e25989_d_n10;
        locals.var_t9__blk562_dn11 = assign18430_e25989_d_n11;
        locals.var_t9__blk562_dn12 = assign18430_e25989_d_n12;
        locals.var_t9__blk562_dn17 = assign18430_e25989_d_n17;
        locals.var_t9__blk562_rv = 0.0;

        let (assign18440_e25999, assign18440_e25999_d_n0, assign18440_e25999_d_n2, assign18440_e25999_d_n6, assign18440_e25999_d_n7, assign18440_e25999_d_n10, assign18440_e25999_d_n11, assign18440_e25999_d_n12, assign18440_e25999_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18440_e25996: f64 = (locals.var_vdsz * p.p246);
        let assign18440_e25997: f64 = (1.0 + assign18440_e25996);
        (assign18440_e25997, (locals.var_vdsz_dn0 * p.p246), (locals.var_vdsz_dn2 * p.p246), (locals.var_vdsz_dn6 * p.p246), (locals.var_vdsz_dn7 * p.p246), (locals.var_vdsz_dn10 * p.p246), (locals.var_vdsz_dn11 * p.p246), (locals.var_vdsz_dn12 * p.p246), (locals.var_vdsz_dn17 * p.p246),)
    } else {
        (locals.var_t4__blk563, locals.var_t4__blk563_dn0, locals.var_t4__blk563_dn2, locals.var_t4__blk563_dn6, locals.var_t4__blk563_dn7, locals.var_t4__blk563_dn10, locals.var_t4__blk563_dn11, locals.var_t4__blk563_dn12, locals.var_t4__blk563_dn17,)
    }
};
        locals.var_t4__blk563 = assign18440_e25999;
        locals.var_t4__blk563_dn0 = assign18440_e25999_d_n0;
        locals.var_t4__blk563_dn2 = assign18440_e25999_d_n2;
        locals.var_t4__blk563_dn6 = assign18440_e25999_d_n6;
        locals.var_t4__blk563_dn7 = assign18440_e25999_d_n7;
        locals.var_t4__blk563_dn10 = assign18440_e25999_d_n10;
        locals.var_t4__blk563_dn11 = assign18440_e25999_d_n11;
        locals.var_t4__blk563_dn12 = assign18440_e25999_d_n12;
        locals.var_t4__blk563_dn17 = assign18440_e25999_d_n17;
        locals.var_t4__blk563_rv = 0.0;

        let (assign18450_e26005, assign18450_e26005_d_n0, assign18450_e26005_d_n2, assign18450_e26005_d_n6, assign18450_e26005_d_n7, assign18450_e26005_d_n10, assign18450_e26005_d_n11, assign18450_e26005_d_n12, assign18450_e26005_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk560, locals.var_t0__blk560_dn0, locals.var_t0__blk560_dn2, locals.var_t0__blk560_dn6, locals.var_t0__blk560_dn7, locals.var_t0__blk560_dn10, locals.var_t0__blk560_dn11, locals.var_t0__blk560_dn12, locals.var_t0__blk560_dn17,)
    }
};
        locals.var_t0__blk560 = assign18450_e26005;
        locals.var_t0__blk560_dn0 = assign18450_e26005_d_n0;
        locals.var_t0__blk560_dn2 = assign18450_e26005_d_n2;
        locals.var_t0__blk560_dn6 = assign18450_e26005_d_n6;
        locals.var_t0__blk560_dn7 = assign18450_e26005_d_n7;
        locals.var_t0__blk560_dn10 = assign18450_e26005_d_n10;
        locals.var_t0__blk560_dn11 = assign18450_e26005_d_n11;
        locals.var_t0__blk560_dn12 = assign18450_e26005_d_n12;
        locals.var_t0__blk560_dn17 = assign18450_e26005_d_n17;
        locals.var_t0__blk560_rv = 0.0;

        let assign18460_e26012: f64 = if ((locals.var_subversion < 3.0) || (p.p43 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard570 = assign18460_e26012;
        locals.var_guard570_rv = 0.0;

        let (assign18470_e26024, assign18470_e26024_d_n0, assign18470_e26024_d_n2, assign18470_e26024_d_n6, assign18470_e26024_d_n7, assign18470_e26024_d_n10, assign18470_e26024_d_n11, assign18470_e26024_d_n12, assign18470_e26024_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign18470_e26020: f64 = (locals.var_ps0 + locals.var_t6__blk565);
        let assign18470_e26022: f64 = (assign18470_e26020 - locals.var_vbsz);
        (assign18470_e26022, ((locals.var_ps0_dn0 + locals.var_t6__blk565_dn0) - locals.var_vbsz_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk565_dn2) - locals.var_vbsz_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk565_dn6) - locals.var_vbsz_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk565_dn7) - locals.var_vbsz_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk565_dn10) - locals.var_vbsz_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk565_dn11) - locals.var_vbsz_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk565_dn12) - locals.var_vbsz_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk565_dn17) - locals.var_vbsz_dn17),)
    } else {
        (locals.var_t5__blk564, locals.var_t5__blk564_dn0, locals.var_t5__blk564_dn2, locals.var_t5__blk564_dn6, locals.var_t5__blk564_dn7, locals.var_t5__blk564_dn10, locals.var_t5__blk564_dn11, locals.var_t5__blk564_dn12, locals.var_t5__blk564_dn17,)
    }
};
        locals.var_t5__blk564 = assign18470_e26024;
        locals.var_t5__blk564_dn0 = assign18470_e26024_d_n0;
        locals.var_t5__blk564_dn2 = assign18470_e26024_d_n2;
        locals.var_t5__blk564_dn6 = assign18470_e26024_d_n6;
        locals.var_t5__blk564_dn7 = assign18470_e26024_d_n7;
        locals.var_t5__blk564_dn10 = assign18470_e26024_d_n10;
        locals.var_t5__blk564_dn11 = assign18470_e26024_d_n11;
        locals.var_t5__blk564_dn12 = assign18470_e26024_d_n12;
        locals.var_t5__blk564_dn17 = assign18470_e26024_d_n17;
        locals.var_t5__blk564_rv = 0.0;

        let (assign18480_e26037, assign18480_e26037_d_n0, assign18480_e26037_d_n2, assign18480_e26037_d_n6, assign18480_e26037_d_n7, assign18480_e26037_d_n10, assign18480_e26037_d_n11, assign18480_e26037_d_n12, assign18480_e26037_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign18480_e26033: f64 = (locals.var_ps0 + locals.var_t6__blk565);
        let assign18480_e26035: f64 = (assign18480_e26033 - locals.var_phi_b0_soi);
        (assign18480_e26035, ((locals.var_ps0_dn0 + locals.var_t6__blk565_dn0) - locals.var_phi_b0_soi_dn0), ((locals.var_ps0_dn2 + locals.var_t6__blk565_dn2) - locals.var_phi_b0_soi_dn2), ((locals.var_ps0_dn6 + locals.var_t6__blk565_dn6) - locals.var_phi_b0_soi_dn6), ((locals.var_ps0_dn7 + locals.var_t6__blk565_dn7) - locals.var_phi_b0_soi_dn7), ((locals.var_ps0_dn10 + locals.var_t6__blk565_dn10) - locals.var_phi_b0_soi_dn10), ((locals.var_ps0_dn11 + locals.var_t6__blk565_dn11) - locals.var_phi_b0_soi_dn11), ((locals.var_ps0_dn12 + locals.var_t6__blk565_dn12) - locals.var_phi_b0_soi_dn12), ((locals.var_ps0_dn17 + locals.var_t6__blk565_dn17) - locals.var_phi_b0_soi_dn17),)
    } else {
        (locals.var_t5__blk564, locals.var_t5__blk564_dn0, locals.var_t5__blk564_dn2, locals.var_t5__blk564_dn6, locals.var_t5__blk564_dn7, locals.var_t5__blk564_dn10, locals.var_t5__blk564_dn11, locals.var_t5__blk564_dn12, locals.var_t5__blk564_dn17,)
    }
};
        locals.var_t5__blk564 = assign18480_e26037;
        locals.var_t5__blk564_dn0 = assign18480_e26037_d_n0;
        locals.var_t5__blk564_dn2 = assign18480_e26037_d_n2;
        locals.var_t5__blk564_dn6 = assign18480_e26037_d_n6;
        locals.var_t5__blk564_dn7 = assign18480_e26037_d_n7;
        locals.var_t5__blk564_dn10 = assign18480_e26037_d_n10;
        locals.var_t5__blk564_dn11 = assign18480_e26037_d_n11;
        locals.var_t5__blk564_dn12 = assign18480_e26037_d_n12;
        locals.var_t5__blk564_dn17 = assign18480_e26037_d_n17;
        locals.var_t5__blk564_rv = 0.0;

        let (assign18490_e26049, assign18490_e26049_d_n0, assign18490_e26049_d_n2, assign18490_e26049_d_n6, assign18490_e26049_d_n7, assign18490_e26049_d_n10, assign18490_e26049_d_n11, assign18490_e26049_d_n12, assign18490_e26049_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18490_e26044: f64 = (locals.var_vdsz * locals.var_t0__blk560);
        let assign18490_e26046: f64 = (assign18490_e26044 * locals.var_t5__blk564);
        let assign18490_e26047: f64 = (locals.var_t4__blk563 + assign18490_e26046);
        (assign18490_e26047, (locals.var_t4__blk563_dn0 + ((((locals.var_vdsz_dn0 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn0)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn0))), (locals.var_t4__blk563_dn2 + ((((locals.var_vdsz_dn2 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn2)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn2))), (locals.var_t4__blk563_dn6 + ((((locals.var_vdsz_dn6 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn6)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn6))), (locals.var_t4__blk563_dn7 + ((((locals.var_vdsz_dn7 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn7)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn7))), (locals.var_t4__blk563_dn10 + ((((locals.var_vdsz_dn10 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn10)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn10))), (locals.var_t4__blk563_dn11 + ((((locals.var_vdsz_dn11 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn11)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn11))), (locals.var_t4__blk563_dn12 + ((((locals.var_vdsz_dn12 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn12)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn12))), (locals.var_t4__blk563_dn17 + ((((locals.var_vdsz_dn17 * locals.var_t0__blk560) + (locals.var_vdsz * locals.var_t0__blk560_dn17)) * locals.var_t5__blk564) + (assign18490_e26044 * locals.var_t5__blk564_dn17))),)
    } else {
        (locals.var_t4__blk563, locals.var_t4__blk563_dn0, locals.var_t4__blk563_dn2, locals.var_t4__blk563_dn6, locals.var_t4__blk563_dn7, locals.var_t4__blk563_dn10, locals.var_t4__blk563_dn11, locals.var_t4__blk563_dn12, locals.var_t4__blk563_dn17,)
    }
};
        locals.var_t4__blk563 = assign18490_e26049;
        locals.var_t4__blk563_dn0 = assign18490_e26049_d_n0;
        locals.var_t4__blk563_dn2 = assign18490_e26049_d_n2;
        locals.var_t4__blk563_dn6 = assign18490_e26049_d_n6;
        locals.var_t4__blk563_dn7 = assign18490_e26049_d_n7;
        locals.var_t4__blk563_dn10 = assign18490_e26049_d_n10;
        locals.var_t4__blk563_dn11 = assign18490_e26049_d_n11;
        locals.var_t4__blk563_dn12 = assign18490_e26049_d_n12;
        locals.var_t4__blk563_dn17 = assign18490_e26049_d_n17;
        locals.var_t4__blk563_rv = 0.0;

        let (assign18500_e26057, assign18500_e26057_d_n0, assign18500_e26057_d_n2, assign18500_e26057_d_n6, assign18500_e26057_d_n7, assign18500_e26057_d_n10, assign18500_e26057_d_n11, assign18500_e26057_d_n12, assign18500_e26057_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        let assign18500_e26055: f64 = (locals.var_t9__blk562 * locals.var_t4__blk563);
        (assign18500_e26055, ((locals.var_t9__blk562_dn0 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn0)), ((locals.var_t9__blk562_dn2 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn2)), ((locals.var_t9__blk562_dn6 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn6)), ((locals.var_t9__blk562_dn7 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn7)), ((locals.var_t9__blk562_dn10 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn10)), ((locals.var_t9__blk562_dn11 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn11)), ((locals.var_t9__blk562_dn12 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn12)), ((locals.var_t9__blk562_dn17 * locals.var_t4__blk563) + (locals.var_t9__blk562 * locals.var_t4__blk563_dn17)),)
    } else {
        (locals.var_t6__blk565, locals.var_t6__blk565_dn0, locals.var_t6__blk565_dn2, locals.var_t6__blk565_dn6, locals.var_t6__blk565_dn7, locals.var_t6__blk565_dn10, locals.var_t6__blk565_dn11, locals.var_t6__blk565_dn12, locals.var_t6__blk565_dn17,)
    }
};
        locals.var_t6__blk565 = assign18500_e26057;
        locals.var_t6__blk565_dn0 = assign18500_e26057_d_n0;
        locals.var_t6__blk565_dn2 = assign18500_e26057_d_n2;
        locals.var_t6__blk565_dn6 = assign18500_e26057_d_n6;
        locals.var_t6__blk565_dn7 = assign18500_e26057_d_n7;
        locals.var_t6__blk565_dn10 = assign18500_e26057_d_n10;
        locals.var_t6__blk565_dn11 = assign18500_e26057_d_n11;
        locals.var_t6__blk565_dn12 = assign18500_e26057_d_n12;
        locals.var_t6__blk565_dn17 = assign18500_e26057_d_n17;
        locals.var_t6__blk565_rv = 0.0;

        let (assign18510_e26063, assign18510_e26063_d_n0, assign18510_e26063_d_n2, assign18510_e26063_d_n6, assign18510_e26063_d_n7, assign18510_e26063_d_n10, assign18510_e26063_d_n11, assign18510_e26063_d_n12, assign18510_e26063_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 != 0.0)) {
        (locals.var_t6__blk565, locals.var_t6__blk565_dn0, locals.var_t6__blk565_dn2, locals.var_t6__blk565_dn6, locals.var_t6__blk565_dn7, locals.var_t6__blk565_dn10, locals.var_t6__blk565_dn11, locals.var_t6__blk565_dn12, locals.var_t6__blk565_dn17,)
    } else {
        (locals.var_t9__blk562, locals.var_t9__blk562_dn0, locals.var_t9__blk562_dn2, locals.var_t9__blk562_dn6, locals.var_t9__blk562_dn7, locals.var_t9__blk562_dn10, locals.var_t9__blk562_dn11, locals.var_t9__blk562_dn12, locals.var_t9__blk562_dn17,)
    }
};
        locals.var_t9__blk562 = assign18510_e26063;
        locals.var_t9__blk562_dn0 = assign18510_e26063_d_n0;
        locals.var_t9__blk562_dn2 = assign18510_e26063_d_n2;
        locals.var_t9__blk562_dn6 = assign18510_e26063_d_n6;
        locals.var_t9__blk562_dn7 = assign18510_e26063_d_n7;
        locals.var_t9__blk562_dn10 = assign18510_e26063_d_n10;
        locals.var_t9__blk562_dn11 = assign18510_e26063_d_n11;
        locals.var_t9__blk562_dn12 = assign18510_e26063_d_n12;
        locals.var_t9__blk562_dn17 = assign18510_e26063_d_n17;
        locals.var_t9__blk562_rv = 0.0;

        let (assign18520_e26070, assign18520_e26070_d_n0, assign18520_e26070_d_n2, assign18520_e26070_d_n6, assign18520_e26070_d_n7, assign18520_e26070_d_n10, assign18520_e26070_d_n11, assign18520_e26070_d_n12, assign18520_e26070_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard568 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9__blk562, locals.var_t9__blk562_dn0, locals.var_t9__blk562_dn2, locals.var_t9__blk562_dn6, locals.var_t9__blk562_dn7, locals.var_t9__blk562_dn10, locals.var_t9__blk562_dn11, locals.var_t9__blk562_dn12, locals.var_t9__blk562_dn17,)
    }
};
        locals.var_t9__blk562 = assign18520_e26070;
        locals.var_t9__blk562_dn0 = assign18520_e26070_d_n0;
        locals.var_t9__blk562_dn2 = assign18520_e26070_d_n2;
        locals.var_t9__blk562_dn6 = assign18520_e26070_d_n6;
        locals.var_t9__blk562_dn7 = assign18520_e26070_d_n7;
        locals.var_t9__blk562_dn10 = assign18520_e26070_d_n10;
        locals.var_t9__blk562_dn11 = assign18520_e26070_d_n11;
        locals.var_t9__blk562_dn12 = assign18520_e26070_d_n12;
        locals.var_t9__blk562_dn17 = assign18520_e26070_d_n17;
        locals.var_t9__blk562_rv = 0.0;

        let assign18530_e26073: f64 = if p.p248 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign18530_e26073;
        locals.var_guard571_rv = 0.0;

        let (assign18540_e26081, assign18540_e26081_d_n0, assign18540_e26081_d_n2, assign18540_e26081_d_n6, assign18540_e26081_d_n7, assign18540_e26081_d_n10, assign18540_e26081_d_n11, assign18540_e26081_d_n12, assign18540_e26081_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 != 0.0)) {
        let assign18540_e26079: f64 = (locals.var_beta * locals.var_gdl0);
        (assign18540_e26079, 0.0, 0.0, 0.0, 0.0, (locals.var_beta_dn10 * locals.var_gdl0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk559, locals.var_t1__blk559_dn0, locals.var_t1__blk559_dn2, locals.var_t1__blk559_dn6, locals.var_t1__blk559_dn7, locals.var_t1__blk559_dn10, locals.var_t1__blk559_dn11, locals.var_t1__blk559_dn12, locals.var_t1__blk559_dn17,)
    }
};
        locals.var_t1__blk559 = assign18540_e26081;
        locals.var_t1__blk559_dn0 = assign18540_e26081_d_n0;
        locals.var_t1__blk559_dn2 = assign18540_e26081_d_n2;
        locals.var_t1__blk559_dn6 = assign18540_e26081_d_n6;
        locals.var_t1__blk559_dn7 = assign18540_e26081_d_n7;
        locals.var_t1__blk559_dn10 = assign18540_e26081_d_n10;
        locals.var_t1__blk559_dn11 = assign18540_e26081_d_n11;
        locals.var_t1__blk559_dn12 = assign18540_e26081_d_n12;
        locals.var_t1__blk559_dn17 = assign18540_e26081_d_n17;
        locals.var_t1__blk559_rv = 0.0;

        let (assign18550_e26089, assign18550_e26089_d_n0, assign18550_e26089_d_n2, assign18550_e26089_d_n6, assign18550_e26089_d_n7, assign18550_e26089_d_n10, assign18550_e26089_d_n11, assign18550_e26089_d_n12, assign18550_e26089_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 != 0.0)) {
        let assign18550_e26087: f64 = (locals.var_c_fox * locals.var_t1__blk559);
        (assign18550_e26087, ((locals.var_c_fox_dn0 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn0)), ((locals.var_c_fox_dn2 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn2)), ((locals.var_c_fox_dn6 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn6)), ((locals.var_c_fox_dn7 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn7)), ((locals.var_c_fox_dn10 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn10)), ((locals.var_c_fox_dn11 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn11)), ((locals.var_c_fox_dn12 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn12)), ((locals.var_c_fox_dn17 * locals.var_t1__blk559) + (locals.var_c_fox * locals.var_t1__blk559_dn17)),)
    } else {
        (locals.var_t2__blk567, locals.var_t2__blk567_dn0, locals.var_t2__blk567_dn2, locals.var_t2__blk567_dn6, locals.var_t2__blk567_dn7, locals.var_t2__blk567_dn10, locals.var_t2__blk567_dn11, locals.var_t2__blk567_dn12, locals.var_t2__blk567_dn17,)
    }
};
        locals.var_t2__blk567 = assign18550_e26089;
        locals.var_t2__blk567_dn0 = assign18550_e26089_d_n0;
        locals.var_t2__blk567_dn2 = assign18550_e26089_d_n2;
        locals.var_t2__blk567_dn6 = assign18550_e26089_d_n6;
        locals.var_t2__blk567_dn7 = assign18550_e26089_d_n7;
        locals.var_t2__blk567_dn10 = assign18550_e26089_d_n10;
        locals.var_t2__blk567_dn11 = assign18550_e26089_d_n11;
        locals.var_t2__blk567_dn12 = assign18550_e26089_d_n12;
        locals.var_t2__blk567_dn17 = assign18550_e26089_d_n17;
        locals.var_t2__blk567_rv = 0.0;

        let (assign18560_e26097, assign18560_e26097_d_n0, assign18560_e26097_d_n2, assign18560_e26097_d_n6, assign18560_e26097_d_n7, assign18560_e26097_d_n10, assign18560_e26097_d_n11, assign18560_e26097_d_n12, assign18560_e26097_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 != 0.0)) {
        let assign18560_e26095: f64 = (locals.var_t2__blk567 * locals.var_vdsz);
        (assign18560_e26095, ((locals.var_t2__blk567_dn0 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn0)), ((locals.var_t2__blk567_dn2 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn2)), ((locals.var_t2__blk567_dn6 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn6)), ((locals.var_t2__blk567_dn7 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn7)), ((locals.var_t2__blk567_dn10 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn10)), ((locals.var_t2__blk567_dn11 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn11)), ((locals.var_t2__blk567_dn12 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn12)), ((locals.var_t2__blk567_dn17 * locals.var_vdsz) + (locals.var_t2__blk567 * locals.var_vdsz_dn17)),)
    } else {
        (locals.var_t8__blk566, locals.var_t8__blk566_dn0, locals.var_t8__blk566_dn2, locals.var_t8__blk566_dn6, locals.var_t8__blk566_dn7, locals.var_t8__blk566_dn10, locals.var_t8__blk566_dn11, locals.var_t8__blk566_dn12, locals.var_t8__blk566_dn17,)
    }
};
        locals.var_t8__blk566 = assign18560_e26097;
        locals.var_t8__blk566_dn0 = assign18560_e26097_d_n0;
        locals.var_t8__blk566_dn2 = assign18560_e26097_d_n2;
        locals.var_t8__blk566_dn6 = assign18560_e26097_d_n6;
        locals.var_t8__blk566_dn7 = assign18560_e26097_d_n7;
        locals.var_t8__blk566_dn10 = assign18560_e26097_d_n10;
        locals.var_t8__blk566_dn11 = assign18560_e26097_d_n11;
        locals.var_t8__blk566_dn12 = assign18560_e26097_d_n12;
        locals.var_t8__blk566_dn17 = assign18560_e26097_d_n17;
        locals.var_t8__blk566_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18570_e26104, assign18570_e26104_d_n0, assign18570_e26104_d_n2, assign18570_e26104_d_n6, assign18570_e26104_d_n7, assign18570_e26104_d_n10, assign18570_e26104_d_n11, assign18570_e26104_d_n12, assign18570_e26104_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard571 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8__blk566, locals.var_t8__blk566_dn0, locals.var_t8__blk566_dn2, locals.var_t8__blk566_dn6, locals.var_t8__blk566_dn7, locals.var_t8__blk566_dn10, locals.var_t8__blk566_dn11, locals.var_t8__blk566_dn12, locals.var_t8__blk566_dn17,)
    }
};
        locals.var_t8__blk566 = assign18570_e26104;
        locals.var_t8__blk566_dn0 = assign18570_e26104_d_n0;
        locals.var_t8__blk566_dn2 = assign18570_e26104_d_n2;
        locals.var_t8__blk566_dn6 = assign18570_e26104_d_n6;
        locals.var_t8__blk566_dn7 = assign18570_e26104_d_n7;
        locals.var_t8__blk566_dn10 = assign18570_e26104_d_n10;
        locals.var_t8__blk566_dn11 = assign18570_e26104_d_n11;
        locals.var_t8__blk566_dn12 = assign18570_e26104_d_n12;
        locals.var_t8__blk566_dn17 = assign18570_e26104_d_n17;
        locals.var_t8__blk566_rv = 0.0;

        let assign18580_e26107: f64 = (locals.var_t9__blk562 + locals.var_t8__blk566);
        let assign18580_e26109: f64 = if assign18580_e26107 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign18580_e26109;
        locals.var_guard572_rv = 0.0;

        let (assign18590_e26119, assign18590_e26119_d_n0, assign18590_e26119_d_n2, assign18590_e26119_d_n6, assign18590_e26119_d_n7, assign18590_e26119_d_n10, assign18590_e26119_d_n11, assign18590_e26119_d_n12, assign18590_e26119_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18590_e26116: f64 = (locals.var_t9__blk562 + locals.var_t8__blk566);
        let assign18590_e26117: f64 = (locals.var_pds * assign18590_e26116);
        (assign18590_e26117, ((locals.var_pds_dn0 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn0 + locals.var_t8__blk566_dn0))), ((locals.var_pds_dn2 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn2 + locals.var_t8__blk566_dn2))), ((locals.var_pds_dn6 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn6 + locals.var_t8__blk566_dn6))), ((locals.var_pds_dn7 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn7 + locals.var_t8__blk566_dn7))), ((locals.var_pds_dn10 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn10 + locals.var_t8__blk566_dn10))), ((locals.var_pds_dn11 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn11 + locals.var_t8__blk566_dn11))), ((locals.var_pds_dn12 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn12 + locals.var_t8__blk566_dn12))), ((locals.var_pds_dn17 * assign18590_e26116) + (locals.var_pds * (locals.var_t9__blk562_dn17 + locals.var_t8__blk566_dn17))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn12, locals.var_idd1_dn17,)
    }
};
        locals.var_idd1 = assign18590_e26119;
        locals.var_idd1_dn0 = assign18590_e26119_d_n0;
        locals.var_idd1_dn2 = assign18590_e26119_d_n2;
        locals.var_idd1_dn6 = assign18590_e26119_d_n6;
        locals.var_idd1_dn7 = assign18590_e26119_d_n7;
        locals.var_idd1_dn10 = assign18590_e26119_d_n10;
        locals.var_idd1_dn11 = assign18590_e26119_d_n11;
        locals.var_idd1_dn12 = assign18590_e26119_d_n12;
        locals.var_idd1_dn17 = assign18590_e26119_d_n17;
        locals.var_idd1_rv = 0.0;

        let (assign18600_e26129, assign18600_e26129_d_n0, assign18600_e26129_d_n2, assign18600_e26129_d_n6, assign18600_e26129_d_n7, assign18600_e26129_d_n10, assign18600_e26129_d_n11, assign18600_e26129_d_n12, assign18600_e26129_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard572 != 0.0)) {
        let assign18600_e26125: f64 = (locals.var_betawl * locals.var_idd1);
        let assign18600_e26127: f64 = (assign18600_e26125 * locals.var_mu);
        (assign18600_e26127, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn12)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn17)) * locals.var_mu) + (assign18600_e26125 * locals.var_mu_dn17)),)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    }
};
        locals.var_idspt = assign18600_e26129;
        locals.var_idspt_dn0 = assign18600_e26129_d_n0;
        locals.var_idspt_dn2 = assign18600_e26129_d_n2;
        locals.var_idspt_dn6 = assign18600_e26129_d_n6;
        locals.var_idspt_dn7 = assign18600_e26129_d_n7;
        locals.var_idspt_dn10 = assign18600_e26129_d_n10;
        locals.var_idspt_dn11 = assign18600_e26129_d_n11;
        locals.var_idspt_dn12 = assign18600_e26129_d_n12;
        locals.var_idspt_dn17 = assign18600_e26129_d_n17;
        locals.var_idspt_rv = 0.0;

        let (assign18610_e26135, assign18610_e26135_d_n0, assign18610_e26135_d_n2, assign18610_e26135_d_n6, assign18610_e26135_d_n7, assign18610_e26135_d_n10, assign18610_e26135_d_n11, assign18610_e26135_d_n12, assign18610_e26135_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign18610_e26133: f64 = (locals.var_ids0 + locals.var_idspt);
        (assign18610_e26133, (locals.var_ids0_dn0 + locals.var_idspt_dn0), (locals.var_ids0_dn2 + locals.var_idspt_dn2), (locals.var_ids0_dn6 + locals.var_idspt_dn6), (locals.var_ids0_dn7 + locals.var_idspt_dn7), (locals.var_ids0_dn10 + locals.var_idspt_dn10), (locals.var_ids0_dn11 + locals.var_idspt_dn11), (locals.var_ids0_dn12 + locals.var_idspt_dn12), (locals.var_ids0_dn17 + locals.var_idspt_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign18610_e26135;
        locals.var_ids_dn0 = assign18610_e26135_d_n0;
        locals.var_ids_dn2 = assign18610_e26135_d_n2;
        locals.var_ids_dn6 = assign18610_e26135_d_n6;
        locals.var_ids_dn7 = assign18610_e26135_d_n7;
        locals.var_ids_dn10 = assign18610_e26135_d_n10;
        locals.var_ids_dn11 = assign18610_e26135_d_n11;
        locals.var_ids_dn12 = assign18610_e26135_d_n12;
        locals.var_ids_dn17 = assign18610_e26135_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign18620_e26139, assign18620_e26139_d_n0, assign18620_e26139_d_n2, assign18620_e26139_d_n6, assign18620_e26139_d_n7, assign18620_e26139_d_n10, assign18620_e26139_d_n11, assign18620_e26139_d_n12, assign18620_e26139_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn6, locals.var_idspt_dn7, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12, locals.var_idspt_dn17,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn12, locals.var_idspt0_dn17,)
    }
};
        locals.var_idspt0 = assign18620_e26139;
        locals.var_idspt0_dn0 = assign18620_e26139_d_n0;
        locals.var_idspt0_dn2 = assign18620_e26139_d_n2;
        locals.var_idspt0_dn6 = assign18620_e26139_d_n6;
        locals.var_idspt0_dn7 = assign18620_e26139_d_n7;
        locals.var_idspt0_dn10 = assign18620_e26139_d_n10;
        locals.var_idspt0_dn11 = assign18620_e26139_d_n11;
        locals.var_idspt0_dn12 = assign18620_e26139_d_n12;
        locals.var_idspt0_dn17 = assign18620_e26139_d_n17;
        locals.var_idspt0_rv = 0.0;

        let assign18630_e26142: f64 = if p.p33 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign18630_e26142;
        locals.var_guard582_rv = 0.0;

        let (assign18640_e26148, assign18640_e26148_d_n0, assign18640_e26148_d_n2, assign18640_e26148_d_n6, assign18640_e26148_d_n7, assign18640_e26148_d_n10, assign18640_e26148_d_n11, assign18640_e26148_d_n12, assign18640_e26148_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn12, locals.var_wdpl_dn17,)
    } else {
        (locals.var_t2__blk575, locals.var_t2__blk575_dn0, locals.var_t2__blk575_dn2, locals.var_t2__blk575_dn6, locals.var_t2__blk575_dn7, locals.var_t2__blk575_dn10, locals.var_t2__blk575_dn11, locals.var_t2__blk575_dn12, locals.var_t2__blk575_dn17,)
    }
};
        locals.var_t2__blk575 = assign18640_e26148;
        locals.var_t2__blk575_dn0 = assign18640_e26148_d_n0;
        locals.var_t2__blk575_dn2 = assign18640_e26148_d_n2;
        locals.var_t2__blk575_dn6 = assign18640_e26148_d_n6;
        locals.var_t2__blk575_dn7 = assign18640_e26148_d_n7;
        locals.var_t2__blk575_dn10 = assign18640_e26148_d_n10;
        locals.var_t2__blk575_dn11 = assign18640_e26148_d_n11;
        locals.var_t2__blk575_dn12 = assign18640_e26148_d_n12;
        locals.var_t2__blk575_dn17 = assign18640_e26148_d_n17;
        locals.var_t2__blk575_rv = 0.0;

        let (assign18650_e26156, assign18650_e26156_d_n0, assign18650_e26156_d_n2, assign18650_e26156_d_n6, assign18650_e26156_d_n7, assign18650_e26156_d_n10, assign18650_e26156_d_n11, assign18650_e26156_d_n12, assign18650_e26156_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18650_e26154: f64 = (locals.var_lgatesm - p.p71);
        (assign18650_e26154, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk576, locals.var_t3__blk576_dn0, locals.var_t3__blk576_dn2, locals.var_t3__blk576_dn6, locals.var_t3__blk576_dn7, locals.var_t3__blk576_dn10, locals.var_t3__blk576_dn11, locals.var_t3__blk576_dn12, locals.var_t3__blk576_dn17,)
    }
};
        locals.var_t3__blk576 = assign18650_e26156;
        locals.var_t3__blk576_dn0 = assign18650_e26156_d_n0;
        locals.var_t3__blk576_dn2 = assign18650_e26156_d_n2;
        locals.var_t3__blk576_dn6 = assign18650_e26156_d_n6;
        locals.var_t3__blk576_dn7 = assign18650_e26156_d_n7;
        locals.var_t3__blk576_dn10 = assign18650_e26156_d_n10;
        locals.var_t3__blk576_dn11 = assign18650_e26156_d_n11;
        locals.var_t3__blk576_dn12 = assign18650_e26156_d_n12;
        locals.var_t3__blk576_dn17 = assign18650_e26156_d_n17;
        locals.var_t3__blk576_rv = 0.0;

        let (assign18660_e26166, assign18660_e26166_d_n0, assign18660_e26166_d_n2, assign18660_e26166_d_n6, assign18660_e26166_d_n7, assign18660_e26166_d_n10, assign18660_e26166_d_n11, assign18660_e26166_d_n12, assign18660_e26166_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18660_e26163: f64 = (locals.var_t3__blk576 * locals.var_t3__blk576);
        let assign18660_e26164: f64 = (1.0 / assign18660_e26163);
        (assign18660_e26164, (-(((locals.var_t3__blk576_dn0 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn0)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn2 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn2)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn6 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn6)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn7 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn7)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn10 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn10)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn11 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn11)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn12 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn12)) / (assign18660_e26163 * assign18660_e26163))), (-(((locals.var_t3__blk576_dn17 * locals.var_t3__blk576) + (locals.var_t3__blk576 * locals.var_t3__blk576_dn17)) / (assign18660_e26163 * assign18660_e26163))),)
    } else {
        (locals.var_t4__blk577, locals.var_t4__blk577_dn0, locals.var_t4__blk577_dn2, locals.var_t4__blk577_dn6, locals.var_t4__blk577_dn7, locals.var_t4__blk577_dn10, locals.var_t4__blk577_dn11, locals.var_t4__blk577_dn12, locals.var_t4__blk577_dn17,)
    }
};
        locals.var_t4__blk577 = assign18660_e26166;
        locals.var_t4__blk577_dn0 = assign18660_e26166_d_n0;
        locals.var_t4__blk577_dn2 = assign18660_e26166_d_n2;
        locals.var_t4__blk577_dn6 = assign18660_e26166_d_n6;
        locals.var_t4__blk577_dn7 = assign18660_e26166_d_n7;
        locals.var_t4__blk577_dn10 = assign18660_e26166_d_n10;
        locals.var_t4__blk577_dn11 = assign18660_e26166_d_n11;
        locals.var_t4__blk577_dn12 = assign18660_e26166_d_n12;
        locals.var_t4__blk577_dn17 = assign18660_e26166_d_n17;
        locals.var_t4__blk577_rv = 0.0;

        let (assign18670_e26184, assign18670_e26184_d_n0, assign18670_e26184_d_n2, assign18670_e26184_d_n6, assign18670_e26184_d_n7, assign18670_e26184_d_n10, assign18670_e26184_d_n11, assign18670_e26184_d_n12, assign18670_e26184_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18670_e26173: f64 = (p.p69 - locals.var_pb20b);
        let assign18670_e26174: f64 = (2.0 * assign18670_e26173);
        let assign18670_e26177: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        let assign18670_e26178: f64 = (assign18670_e26174 * assign18670_e26177);
        let assign18670_e26180: f64 = (assign18670_e26178 * locals.var_t2__blk575);
        let assign18670_e26182: f64 = (assign18670_e26180 * locals.var_t4__blk577);
        (assign18670_e26182, (((((((2.0 * (-locals.var_pb20b_dn0)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn0))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn0)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn2))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn2)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn2)), (((((((2.0 * (-locals.var_pb20b_dn6)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn6))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn6)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn7))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn7)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn7)), (((((((2.0 * (-locals.var_pb20b_dn10)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn10))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn10)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn11))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn11)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn11)), (((((((2.0 * (-locals.var_pb20b_dn12)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn12))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn12)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn12)), (((((((2.0 * (-locals.var_pb20b_dn17)) * assign18670_e26177) + (assign18670_e26174 * (1.034943e-10 * locals.var_c_fox_inv_dn17))) * locals.var_t2__blk575) + (assign18670_e26178 * locals.var_t2__blk575_dn17)) * locals.var_t4__blk577) + (assign18670_e26180 * locals.var_t4__blk577_dn17)),)
    } else {
        (locals.var_t5__blk578, locals.var_t5__blk578_dn0, locals.var_t5__blk578_dn2, locals.var_t5__blk578_dn6, locals.var_t5__blk578_dn7, locals.var_t5__blk578_dn10, locals.var_t5__blk578_dn11, locals.var_t5__blk578_dn12, locals.var_t5__blk578_dn17,)
    }
};
        locals.var_t5__blk578 = assign18670_e26184;
        locals.var_t5__blk578_dn0 = assign18670_e26184_d_n0;
        locals.var_t5__blk578_dn2 = assign18670_e26184_d_n2;
        locals.var_t5__blk578_dn6 = assign18670_e26184_d_n6;
        locals.var_t5__blk578_dn7 = assign18670_e26184_d_n7;
        locals.var_t5__blk578_dn10 = assign18670_e26184_d_n10;
        locals.var_t5__blk578_dn11 = assign18670_e26184_d_n11;
        locals.var_t5__blk578_dn12 = assign18670_e26184_d_n12;
        locals.var_t5__blk578_dn17 = assign18670_e26184_d_n17;
        locals.var_t5__blk578_rv = 0.0;

        let (assign18680_e26192, assign18680_e26192_d_n0, assign18680_e26192_d_n2, assign18680_e26192_d_n6, assign18680_e26192_d_n7, assign18680_e26192_d_n10, assign18680_e26192_d_n11, assign18680_e26192_d_n12, assign18680_e26192_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18680_e26190: f64 = (locals.var_t5__blk578 * locals.var_sqrt_pbsum);
        (assign18680_e26190, ((locals.var_t5__blk578_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5__blk578_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5__blk578_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5__blk578_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5__blk578_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5__blk578_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5__blk578_dn12 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn12)), ((locals.var_t5__blk578_dn17 * locals.var_sqrt_pbsum) + (locals.var_t5__blk578 * locals.var_sqrt_pbsum_dn17)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn12, locals.var_dvth0_dn17,)
    }
};
        locals.var_dvth0 = assign18680_e26192;
        locals.var_dvth0_dn0 = assign18680_e26192_d_n0;
        locals.var_dvth0_dn2 = assign18680_e26192_d_n2;
        locals.var_dvth0_dn6 = assign18680_e26192_d_n6;
        locals.var_dvth0_dn7 = assign18680_e26192_d_n7;
        locals.var_dvth0_dn10 = assign18680_e26192_d_n10;
        locals.var_dvth0_dn11 = assign18680_e26192_d_n11;
        locals.var_dvth0_dn12 = assign18680_e26192_d_n12;
        locals.var_dvth0_dn17 = assign18680_e26192_d_n17;
        locals.var_dvth0_rv = 0.0;

        let (assign18690_e26202, assign18690_e26202_d_n0, assign18690_e26202_d_n2, assign18690_e26202_d_n6, assign18690_e26202_d_n7, assign18690_e26202_d_n10, assign18690_e26202_d_n11, assign18690_e26202_d_n12, assign18690_e26202_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18690_e26199: f64 = (p.p155 * locals.var_vdsz);
        let assign18690_e26200: f64 = (p.p154 + assign18690_e26199);
        (assign18690_e26200, (p.p155 * locals.var_vdsz_dn0), (p.p155 * locals.var_vdsz_dn2), (p.p155 * locals.var_vdsz_dn6), (p.p155 * locals.var_vdsz_dn7), (p.p155 * locals.var_vdsz_dn10), (p.p155 * locals.var_vdsz_dn11), (p.p155 * locals.var_vdsz_dn12), (p.p155 * locals.var_vdsz_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18690_e26202;
        locals.var_t1w_dn0 = assign18690_e26202_d_n0;
        locals.var_t1w_dn2 = assign18690_e26202_d_n2;
        locals.var_t1w_dn6 = assign18690_e26202_d_n6;
        locals.var_t1w_dn7 = assign18690_e26202_d_n7;
        locals.var_t1w_dn10 = assign18690_e26202_d_n10;
        locals.var_t1w_dn11 = assign18690_e26202_d_n11;
        locals.var_t1w_dn12 = assign18690_e26202_d_n12;
        locals.var_t1w_dn17 = assign18690_e26202_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign18700_e26210, assign18700_e26210_d_n0, assign18700_e26210_d_n2, assign18700_e26210_d_n6, assign18700_e26210_d_n7, assign18700_e26210_d_n10, assign18700_e26210_d_n11, assign18700_e26210_d_n12, assign18700_e26210_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18700_e26208: f64 = (locals.var_dvth0 * locals.var_t1w);
        (assign18700_e26208, ((locals.var_dvth0_dn0 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn2)), ((locals.var_dvth0_dn6 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn7)), ((locals.var_dvth0_dn10 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn11)), ((locals.var_dvth0_dn12 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn12)), ((locals.var_dvth0_dn17 * locals.var_t1w) + (locals.var_dvth0 * locals.var_t1w_dn17)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn12, locals.var_dvthscsti_dn17,)
    }
};
        locals.var_dvthscsti = assign18700_e26210;
        locals.var_dvthscsti_dn0 = assign18700_e26210_d_n0;
        locals.var_dvthscsti_dn2 = assign18700_e26210_d_n2;
        locals.var_dvthscsti_dn6 = assign18700_e26210_d_n6;
        locals.var_dvthscsti_dn7 = assign18700_e26210_d_n7;
        locals.var_dvthscsti_dn10 = assign18700_e26210_d_n10;
        locals.var_dvthscsti_dn11 = assign18700_e26210_d_n11;
        locals.var_dvthscsti_dn12 = assign18700_e26210_d_n12;
        locals.var_dvthscsti_dn17 = assign18700_e26210_d_n17;
        locals.var_dvthscsti_rv = 0.0;

        let (assign18710_e26220, assign18710_e26220_d_n0, assign18710_e26220_d_n2, assign18710_e26220_d_n6, assign18710_e26220_d_n7, assign18710_e26220_d_n10, assign18710_e26220_d_n11, assign18710_e26220_d_n12, assign18710_e26220_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18710_e26217: f64 = (p.p157 * locals.var_vds);
        let assign18710_e26218: f64 = (p.p156 - assign18710_e26217);
        (assign18710_e26218, (-(p.p157 * locals.var_vds_dn0)), (-(p.p157 * locals.var_vds_dn2)), (-(p.p157 * locals.var_vds_dn6)), (-(p.p157 * locals.var_vds_dn7)), (-(p.p157 * locals.var_vds_dn10)), (-(p.p157 * locals.var_vds_dn11)), (-(p.p157 * locals.var_vds_dn12)), (-(p.p157 * locals.var_vds_dn17)),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18710_e26220;
        locals.var_t1__blk573_dn0 = assign18710_e26220_d_n0;
        locals.var_t1__blk573_dn2 = assign18710_e26220_d_n2;
        locals.var_t1__blk573_dn6 = assign18710_e26220_d_n6;
        locals.var_t1__blk573_dn7 = assign18710_e26220_d_n7;
        locals.var_t1__blk573_dn10 = assign18710_e26220_d_n10;
        locals.var_t1__blk573_dn11 = assign18710_e26220_d_n11;
        locals.var_t1__blk573_dn12 = assign18710_e26220_d_n12;
        locals.var_t1__blk573_dn17 = assign18710_e26220_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let (assign18720_e26232, assign18720_e26232_d_n0, assign18720_e26232_d_n2, assign18720_e26232_d_n6, assign18720_e26232_d_n7, assign18720_e26232_d_n10, assign18720_e26232_d_n11, assign18720_e26232_d_n12, assign18720_e26232_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18720_e26226: f64 = (locals.var_vgsz - locals.var_vfb);
        let assign18720_e26228: f64 = (assign18720_e26226 + locals.var_t1__blk573);
        let assign18720_e26230: f64 = (assign18720_e26228 + locals.var_dvthscsti);
        (assign18720_e26230, ((locals.var_vgsz_dn0 + locals.var_t1__blk573_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz_dn2 + locals.var_t1__blk573_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz_dn6 + locals.var_t1__blk573_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz_dn7 + locals.var_t1__blk573_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz_dn10 + locals.var_t1__blk573_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz_dn11 + locals.var_t1__blk573_dn11) + locals.var_dvthscsti_dn11), ((locals.var_vgsz_dn12 + locals.var_t1__blk573_dn12) + locals.var_dvthscsti_dn12), ((locals.var_vgsz_dn17 + locals.var_t1__blk573_dn17) + locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn12, locals.var_vgssti_dn17,)
    }
};
        locals.var_vgssti = assign18720_e26232;
        locals.var_vgssti_dn0 = assign18720_e26232_d_n0;
        locals.var_vgssti_dn2 = assign18720_e26232_d_n2;
        locals.var_vgssti_dn6 = assign18720_e26232_d_n6;
        locals.var_vgssti_dn7 = assign18720_e26232_d_n7;
        locals.var_vgssti_dn10 = assign18720_e26232_d_n10;
        locals.var_vgssti_dn11 = assign18720_e26232_d_n11;
        locals.var_vgssti_dn12 = assign18720_e26232_d_n12;
        locals.var_vgssti_dn17 = assign18720_e26232_d_n17;
        locals.var_vgssti_rv = 0.0;

        let (assign18730_e26242, assign18730_e26242_d_n0, assign18730_e26242_d_n2, assign18730_e26242_d_n6, assign18730_e26242_d_n7, assign18730_e26242_d_n10, assign18730_e26242_d_n11, assign18730_e26242_d_n12, assign18730_e26242_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18730_e26238: f64 = (locals.var_costi0_p2 * locals.var_c_fox_inv);
        let assign18730_e26240: f64 = (assign18730_e26238 * locals.var_c_fox_inv);
        (assign18730_e26240, ((((locals.var_costi0_p2_dn0 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn2)), ((((locals.var_costi0_p2_dn6 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn7)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn7)), ((((locals.var_costi0_p2_dn10 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn11)), ((((locals.var_costi0_p2_dn12 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn12)), ((((locals.var_costi0_p2_dn17 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn17)) * locals.var_c_fox_inv) + (assign18730_e26238 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn12, locals.var_costi3_dn17,)
    }
};
        locals.var_costi3 = assign18730_e26242;
        locals.var_costi3_dn0 = assign18730_e26242_d_n0;
        locals.var_costi3_dn2 = assign18730_e26242_d_n2;
        locals.var_costi3_dn6 = assign18730_e26242_d_n6;
        locals.var_costi3_dn7 = assign18730_e26242_d_n7;
        locals.var_costi3_dn10 = assign18730_e26242_d_n10;
        locals.var_costi3_dn11 = assign18730_e26242_d_n11;
        locals.var_costi3_dn12 = assign18730_e26242_d_n12;
        locals.var_costi3_dn17 = assign18730_e26242_d_n17;
        locals.var_costi3_rv = 0.0;

        let (assign18740_e26252, assign18740_e26252_d_n0, assign18740_e26252_d_n2, assign18740_e26252_d_n6, assign18740_e26252_d_n7, assign18740_e26252_d_n10, assign18740_e26252_d_n11, assign18740_e26252_d_n12, assign18740_e26252_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18740_e26248: f64 = (locals.var_costi3 * locals.var_beta);
        let assign18740_e26250: f64 = (assign18740_e26248 * 0.5);
        (assign18740_e26250, ((locals.var_costi3_dn0 * locals.var_beta) * 0.5), ((locals.var_costi3_dn2 * locals.var_beta) * 0.5), ((locals.var_costi3_dn6 * locals.var_beta) * 0.5), ((locals.var_costi3_dn7 * locals.var_beta) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), ((locals.var_costi3_dn11 * locals.var_beta) * 0.5), ((locals.var_costi3_dn12 * locals.var_beta) * 0.5), ((locals.var_costi3_dn17 * locals.var_beta) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn12, locals.var_costi4_dn17,)
    }
};
        locals.var_costi4 = assign18740_e26252;
        locals.var_costi4_dn0 = assign18740_e26252_d_n0;
        locals.var_costi4_dn2 = assign18740_e26252_d_n2;
        locals.var_costi4_dn6 = assign18740_e26252_d_n6;
        locals.var_costi4_dn7 = assign18740_e26252_d_n7;
        locals.var_costi4_dn10 = assign18740_e26252_d_n10;
        locals.var_costi4_dn11 = assign18740_e26252_d_n11;
        locals.var_costi4_dn12 = assign18740_e26252_d_n12;
        locals.var_costi4_dn17 = assign18740_e26252_d_n17;
        locals.var_costi4_rv = 0.0;

        let (assign18750_e26262, assign18750_e26262_d_n0, assign18750_e26262_d_n2, assign18750_e26262_d_n6, assign18750_e26262_d_n7, assign18750_e26262_d_n10, assign18750_e26262_d_n11, assign18750_e26262_d_n12, assign18750_e26262_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18750_e26258: f64 = (locals.var_costi4 * locals.var_beta);
        let assign18750_e26260: f64 = (assign18750_e26258 * 2.0);
        (assign18750_e26260, ((locals.var_costi4_dn0 * locals.var_beta) * 2.0), ((locals.var_costi4_dn2 * locals.var_beta) * 2.0), ((locals.var_costi4_dn6 * locals.var_beta) * 2.0), ((locals.var_costi4_dn7 * locals.var_beta) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), ((locals.var_costi4_dn11 * locals.var_beta) * 2.0), ((locals.var_costi4_dn12 * locals.var_beta) * 2.0), ((locals.var_costi4_dn17 * locals.var_beta) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn12, locals.var_costi5_dn17,)
    }
};
        locals.var_costi5 = assign18750_e26262;
        locals.var_costi5_dn0 = assign18750_e26262_d_n0;
        locals.var_costi5_dn2 = assign18750_e26262_d_n2;
        locals.var_costi5_dn6 = assign18750_e26262_d_n6;
        locals.var_costi5_dn7 = assign18750_e26262_d_n7;
        locals.var_costi5_dn10 = assign18750_e26262_d_n10;
        locals.var_costi5_dn11 = assign18750_e26262_d_n11;
        locals.var_costi5_dn12 = assign18750_e26262_d_n12;
        locals.var_costi5_dn17 = assign18750_e26262_d_n17;
        locals.var_costi5_rv = 0.0;

        let (assign18760_e26282, assign18760_e26282_d_n0, assign18760_e26282_d_n2, assign18760_e26282_d_n6, assign18760_e26282_d_n7, assign18760_e26282_d_n10, assign18760_e26282_d_n11, assign18760_e26282_d_n12, assign18760_e26282_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18760_e26270: f64 = (locals.var_beta * 0.25);
        let assign18760_e26271: f64 = (locals.var_costi3 * assign18760_e26270);
        let assign18760_e26272: f64 = (locals.var_beta_inv - assign18760_e26271);
        let assign18760_e26274: f64 = (assign18760_e26272 + locals.var_vfb);
        let assign18760_e26276: f64 = (assign18760_e26274 - p.p156);
        let assign18760_e26278: f64 = (assign18760_e26276 - locals.var_dvthscsti);
        let assign18760_e26280: f64 = (assign18760_e26278 + 1e-50);
        (assign18760_e26280, ((-(locals.var_costi3_dn0 * assign18760_e26270)) - locals.var_dvthscsti_dn0), ((-(locals.var_costi3_dn2 * assign18760_e26270)) - locals.var_dvthscsti_dn2), ((-(locals.var_costi3_dn6 * assign18760_e26270)) - locals.var_dvthscsti_dn6), ((-(locals.var_costi3_dn7 * assign18760_e26270)) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * assign18760_e26270) + (locals.var_costi3 * (locals.var_beta_dn10 * 0.25)))) - locals.var_dvthscsti_dn10), ((-(locals.var_costi3_dn11 * assign18760_e26270)) - locals.var_dvthscsti_dn11), ((-(locals.var_costi3_dn12 * assign18760_e26270)) - locals.var_dvthscsti_dn12), ((-(locals.var_costi3_dn17 * assign18760_e26270)) - locals.var_dvthscsti_dn17),)
    } else {
        (locals.var_t10__blk579, locals.var_t10__blk579_dn0, locals.var_t10__blk579_dn2, locals.var_t10__blk579_dn6, locals.var_t10__blk579_dn7, locals.var_t10__blk579_dn10, locals.var_t10__blk579_dn11, locals.var_t10__blk579_dn12, locals.var_t10__blk579_dn17,)
    }
};
        locals.var_t10__blk579 = assign18760_e26282;
        locals.var_t10__blk579_dn0 = assign18760_e26282_d_n0;
        locals.var_t10__blk579_dn2 = assign18760_e26282_d_n2;
        locals.var_t10__blk579_dn6 = assign18760_e26282_d_n6;
        locals.var_t10__blk579_dn7 = assign18760_e26282_d_n7;
        locals.var_t10__blk579_dn10 = assign18760_e26282_d_n10;
        locals.var_t10__blk579_dn11 = assign18760_e26282_d_n11;
        locals.var_t10__blk579_dn12 = assign18760_e26282_d_n12;
        locals.var_t10__blk579_dn17 = assign18760_e26282_d_n17;
        locals.var_t10__blk579_rv = 0.0;

        let (assign18770_e26292, assign18770_e26292_d_n0, assign18770_e26292_d_n2, assign18770_e26292_d_n6, assign18770_e26292_d_n7, assign18770_e26292_d_n10, assign18770_e26292_d_n11, assign18770_e26292_d_n12, assign18770_e26292_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18770_e26288: f64 = (locals.var_vgsz - locals.var_t10__blk579);
        let assign18770_e26290: f64 = (assign18770_e26288 - 0.005);
        (assign18770_e26290, (locals.var_vgsz_dn0 - locals.var_t10__blk579_dn0), (locals.var_vgsz_dn2 - locals.var_t10__blk579_dn2), (locals.var_vgsz_dn6 - locals.var_t10__blk579_dn6), (locals.var_vgsz_dn7 - locals.var_t10__blk579_dn7), (locals.var_vgsz_dn10 - locals.var_t10__blk579_dn10), (locals.var_vgsz_dn11 - locals.var_t10__blk579_dn11), (locals.var_vgsz_dn12 - locals.var_t10__blk579_dn12), (locals.var_vgsz_dn17 - locals.var_t10__blk579_dn17),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18770_e26292;
        locals.var_t1__blk573_dn0 = assign18770_e26292_d_n0;
        locals.var_t1__blk573_dn2 = assign18770_e26292_d_n2;
        locals.var_t1__blk573_dn6 = assign18770_e26292_d_n6;
        locals.var_t1__blk573_dn7 = assign18770_e26292_d_n7;
        locals.var_t1__blk573_dn10 = assign18770_e26292_d_n10;
        locals.var_t1__blk573_dn11 = assign18770_e26292_d_n11;
        locals.var_t1__blk573_dn12 = assign18770_e26292_d_n12;
        locals.var_t1__blk573_dn17 = assign18770_e26292_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let (assign18780_e26304, assign18780_e26304_d_n0, assign18780_e26304_d_n2, assign18780_e26304_d_n6, assign18780_e26304_d_n7, assign18780_e26304_d_n10, assign18780_e26304_d_n11, assign18780_e26304_d_n12, assign18780_e26304_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let (assign18780_e26302,) = {
            if (locals.var_t10__blk579 >= 0.0) {
                (1.0,)
            } else {
                let assign18780_e26301: f64 = (-1.0);
                (assign18780_e26301,)
            }
        };
        (assign18780_e26302, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18780_e26304;
        locals.var_t0_dn0 = assign18780_e26304_d_n0;
        locals.var_t0_dn2 = assign18780_e26304_d_n2;
        locals.var_t0_dn6 = assign18780_e26304_d_n6;
        locals.var_t0_dn7 = assign18780_e26304_d_n7;
        locals.var_t0_dn10 = assign18780_e26304_d_n10;
        locals.var_t0_dn11 = assign18780_e26304_d_n11;
        locals.var_t0_dn12 = assign18780_e26304_d_n12;
        locals.var_t0_dn17 = assign18780_e26304_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18790_e26321, assign18790_e26321_d_n0, assign18790_e26321_d_n2, assign18790_e26321_d_n6, assign18790_e26321_d_n7, assign18790_e26321_d_n10, assign18790_e26321_d_n11, assign18790_e26321_d_n12, assign18790_e26321_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18790_e26310: f64 = (locals.var_t1__blk573 * locals.var_t1__blk573);
        let assign18790_e26313: f64 = (locals.var_t0 * 4.0);
        let assign18790_e26315: f64 = (assign18790_e26313 * locals.var_t10__blk579);
        let assign18790_e26317: f64 = (assign18790_e26315 * 0.005);
        let assign18790_e26318: f64 = (assign18790_e26310 + assign18790_e26317);
        let assign18790_e26319: f64 = (assign18790_e26318).sqrt();
        (assign18790_e26319, ((((locals.var_t1__blk573_dn0 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn0)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn2 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn2)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn6 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn6)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn7 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn7)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn10 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn10)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn11 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn11)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn12 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn12)) + ((((locals.var_t0_dn12 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn12)) * 0.005)) / (2.0 * assign18790_e26319)), ((((locals.var_t1__blk573_dn17 * locals.var_t1__blk573) + (locals.var_t1__blk573 * locals.var_t1__blk573_dn17)) + ((((locals.var_t0_dn17 * 4.0) * locals.var_t10__blk579) + (assign18790_e26313 * locals.var_t10__blk579_dn17)) * 0.005)) / (2.0 * assign18790_e26319)),)
    } else {
        (locals.var_t2__blk575, locals.var_t2__blk575_dn0, locals.var_t2__blk575_dn2, locals.var_t2__blk575_dn6, locals.var_t2__blk575_dn7, locals.var_t2__blk575_dn10, locals.var_t2__blk575_dn11, locals.var_t2__blk575_dn12, locals.var_t2__blk575_dn17,)
    }
};
        locals.var_t2__blk575 = assign18790_e26321;
        locals.var_t2__blk575_dn0 = assign18790_e26321_d_n0;
        locals.var_t2__blk575_dn2 = assign18790_e26321_d_n2;
        locals.var_t2__blk575_dn6 = assign18790_e26321_d_n6;
        locals.var_t2__blk575_dn7 = assign18790_e26321_d_n7;
        locals.var_t2__blk575_dn10 = assign18790_e26321_d_n10;
        locals.var_t2__blk575_dn11 = assign18790_e26321_d_n11;
        locals.var_t2__blk575_dn12 = assign18790_e26321_d_n12;
        locals.var_t2__blk575_dn17 = assign18790_e26321_d_n17;
        locals.var_t2__blk575_rv = 0.0;

        let (assign18800_e26341, assign18800_e26341_d_n0, assign18800_e26341_d_n2, assign18800_e26341_d_n6, assign18800_e26341_d_n7, assign18800_e26341_d_n10, assign18800_e26341_d_n11, assign18800_e26341_d_n12, assign18800_e26341_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18800_e26329: f64 = (locals.var_t1__blk573 + locals.var_t2__blk575);
        let assign18800_e26330: f64 = (0.5 * assign18800_e26329);
        let assign18800_e26331: f64 = (locals.var_t10__blk579 + assign18800_e26330);
        let assign18800_e26333: f64 = (assign18800_e26331 - locals.var_vfb);
        let assign18800_e26335: f64 = (assign18800_e26333 + p.p156);
        let assign18800_e26337: f64 = (assign18800_e26335 + locals.var_dvthscsti);
        let assign18800_e26339: f64 = (assign18800_e26337 - locals.var_vbspz);
        (assign18800_e26339, (((locals.var_t10__blk579_dn0 + (0.5 * (locals.var_t1__blk573_dn0 + locals.var_t2__blk575_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbspz_dn0), (((locals.var_t10__blk579_dn2 + (0.5 * (locals.var_t1__blk573_dn2 + locals.var_t2__blk575_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbspz_dn2), (((locals.var_t10__blk579_dn6 + (0.5 * (locals.var_t1__blk573_dn6 + locals.var_t2__blk575_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbspz_dn6), (((locals.var_t10__blk579_dn7 + (0.5 * (locals.var_t1__blk573_dn7 + locals.var_t2__blk575_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbspz_dn7), (((locals.var_t10__blk579_dn10 + (0.5 * (locals.var_t1__blk573_dn10 + locals.var_t2__blk575_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbspz_dn10), (((locals.var_t10__blk579_dn11 + (0.5 * (locals.var_t1__blk573_dn11 + locals.var_t2__blk575_dn11))) + locals.var_dvthscsti_dn11) - locals.var_vbspz_dn11), (((locals.var_t10__blk579_dn12 + (0.5 * (locals.var_t1__blk573_dn12 + locals.var_t2__blk575_dn12))) + locals.var_dvthscsti_dn12) - locals.var_vbspz_dn12), (((locals.var_t10__blk579_dn17 + (0.5 * (locals.var_t1__blk573_dn17 + locals.var_t2__blk575_dn17))) + locals.var_dvthscsti_dn17) - locals.var_vbspz_dn17),)
    } else {
        (locals.var_t3__blk576, locals.var_t3__blk576_dn0, locals.var_t3__blk576_dn2, locals.var_t3__blk576_dn6, locals.var_t3__blk576_dn7, locals.var_t3__blk576_dn10, locals.var_t3__blk576_dn11, locals.var_t3__blk576_dn12, locals.var_t3__blk576_dn17,)
    }
};
        locals.var_t3__blk576 = assign18800_e26341;
        locals.var_t3__blk576_dn0 = assign18800_e26341_d_n0;
        locals.var_t3__blk576_dn2 = assign18800_e26341_d_n2;
        locals.var_t3__blk576_dn6 = assign18800_e26341_d_n6;
        locals.var_t3__blk576_dn7 = assign18800_e26341_d_n7;
        locals.var_t3__blk576_dn10 = assign18800_e26341_d_n10;
        locals.var_t3__blk576_dn11 = assign18800_e26341_d_n11;
        locals.var_t3__blk576_dn12 = assign18800_e26341_d_n12;
        locals.var_t3__blk576_dn17 = assign18800_e26341_d_n17;
        locals.var_t3__blk576_rv = 0.0;

        let (assign18810_e26351, assign18810_e26351_d_n0, assign18810_e26351_d_n2, assign18810_e26351_d_n6, assign18810_e26351_d_n7, assign18810_e26351_d_n10, assign18810_e26351_d_n11, assign18810_e26351_d_n12, assign18810_e26351_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18810_e26347: f64 = (locals.var_beta * locals.var_t3__blk576);
        let assign18810_e26349: f64 = (assign18810_e26347 - 1.0);
        (assign18810_e26349, (locals.var_beta * locals.var_t3__blk576_dn0), (locals.var_beta * locals.var_t3__blk576_dn2), (locals.var_beta * locals.var_t3__blk576_dn6), (locals.var_beta * locals.var_t3__blk576_dn7), ((locals.var_beta_dn10 * locals.var_t3__blk576) + (locals.var_beta * locals.var_t3__blk576_dn10)), (locals.var_beta * locals.var_t3__blk576_dn11), (locals.var_beta * locals.var_t3__blk576_dn12), (locals.var_beta * locals.var_t3__blk576_dn17),)
    } else {
        (locals.var_t4__blk577, locals.var_t4__blk577_dn0, locals.var_t4__blk577_dn2, locals.var_t4__blk577_dn6, locals.var_t4__blk577_dn7, locals.var_t4__blk577_dn10, locals.var_t4__blk577_dn11, locals.var_t4__blk577_dn12, locals.var_t4__blk577_dn17,)
    }
};
        locals.var_t4__blk577 = assign18810_e26351;
        locals.var_t4__blk577_dn0 = assign18810_e26351_d_n0;
        locals.var_t4__blk577_dn2 = assign18810_e26351_d_n2;
        locals.var_t4__blk577_dn6 = assign18810_e26351_d_n6;
        locals.var_t4__blk577_dn7 = assign18810_e26351_d_n7;
        locals.var_t4__blk577_dn10 = assign18810_e26351_d_n10;
        locals.var_t4__blk577_dn11 = assign18810_e26351_d_n11;
        locals.var_t4__blk577_dn12 = assign18810_e26351_d_n12;
        locals.var_t4__blk577_dn17 = assign18810_e26351_d_n17;
        locals.var_t4__blk577_rv = 0.0;

        let (assign18820_e26359, assign18820_e26359_d_n0, assign18820_e26359_d_n2, assign18820_e26359_d_n6, assign18820_e26359_d_n7, assign18820_e26359_d_n10, assign18820_e26359_d_n11, assign18820_e26359_d_n12, assign18820_e26359_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18820_e26357: f64 = (4.0 / locals.var_costi5);
        (assign18820_e26357, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn12) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn17) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5__blk578, locals.var_t5__blk578_dn0, locals.var_t5__blk578_dn2, locals.var_t5__blk578_dn6, locals.var_t5__blk578_dn7, locals.var_t5__blk578_dn10, locals.var_t5__blk578_dn11, locals.var_t5__blk578_dn12, locals.var_t5__blk578_dn17,)
    }
};
        locals.var_t5__blk578 = assign18820_e26359;
        locals.var_t5__blk578_dn0 = assign18820_e26359_d_n0;
        locals.var_t5__blk578_dn2 = assign18820_e26359_d_n2;
        locals.var_t5__blk578_dn6 = assign18820_e26359_d_n6;
        locals.var_t5__blk578_dn7 = assign18820_e26359_d_n7;
        locals.var_t5__blk578_dn10 = assign18820_e26359_d_n10;
        locals.var_t5__blk578_dn11 = assign18820_e26359_d_n11;
        locals.var_t5__blk578_dn12 = assign18820_e26359_d_n12;
        locals.var_t5__blk578_dn17 = assign18820_e26359_d_n17;
        locals.var_t5__blk578_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign18830_e26369, assign18830_e26369_d_n0, assign18830_e26369_d_n2, assign18830_e26369_d_n6, assign18830_e26369_d_n7, assign18830_e26369_d_n10, assign18830_e26369_d_n11, assign18830_e26369_d_n12, assign18830_e26369_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18830_e26366: f64 = (locals.var_t4__blk577 * locals.var_t5__blk578);
        let assign18830_e26367: f64 = (1.0 + assign18830_e26366);
        (assign18830_e26367, ((locals.var_t4__blk577_dn0 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn0)), ((locals.var_t4__blk577_dn2 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn2)), ((locals.var_t4__blk577_dn6 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn6)), ((locals.var_t4__blk577_dn7 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn7)), ((locals.var_t4__blk577_dn10 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn10)), ((locals.var_t4__blk577_dn11 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn11)), ((locals.var_t4__blk577_dn12 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn12)), ((locals.var_t4__blk577_dn17 * locals.var_t5__blk578) + (locals.var_t4__blk577 * locals.var_t5__blk578_dn17)),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18830_e26369;
        locals.var_t1w_dn0 = assign18830_e26369_d_n0;
        locals.var_t1w_dn2 = assign18830_e26369_d_n2;
        locals.var_t1w_dn6 = assign18830_e26369_d_n6;
        locals.var_t1w_dn7 = assign18830_e26369_d_n7;
        locals.var_t1w_dn10 = assign18830_e26369_d_n10;
        locals.var_t1w_dn11 = assign18830_e26369_d_n11;
        locals.var_t1w_dn12 = assign18830_e26369_d_n12;
        locals.var_t1w_dn17 = assign18830_e26369_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign18840_e26384, assign18840_e26384_d_n0, assign18840_e26384_d_n2, assign18840_e26384_d_n6, assign18840_e26384_d_n7, assign18840_e26384_d_n10, assign18840_e26384_d_n11, assign18840_e26384_d_n12, assign18840_e26384_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18840_e26375: f64 = (locals.var_t1w * locals.var_t1w);
        let assign18840_e26378: f64 = (4.0 * 0.01);
        let assign18840_e26380: f64 = (assign18840_e26378 * 0.01);
        let assign18840_e26381: f64 = (assign18840_e26375 + assign18840_e26380);
        let assign18840_e26382: f64 = (assign18840_e26381).sqrt();
        (assign18840_e26382, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign18840_e26382)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign18840_e26382)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18840_e26384;
        locals.var_tmf1_dn0 = assign18840_e26384_d_n0;
        locals.var_tmf1_dn2 = assign18840_e26384_d_n2;
        locals.var_tmf1_dn6 = assign18840_e26384_d_n6;
        locals.var_tmf1_dn7 = assign18840_e26384_d_n7;
        locals.var_tmf1_dn10 = assign18840_e26384_d_n10;
        locals.var_tmf1_dn11 = assign18840_e26384_d_n11;
        locals.var_tmf1_dn12 = assign18840_e26384_d_n12;
        locals.var_tmf1_dn17 = assign18840_e26384_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign18850_e26398, assign18850_e26398_d_n0, assign18850_e26398_d_n2, assign18850_e26398_d_n6, assign18850_e26398_d_n7, assign18850_e26398_d_n10, assign18850_e26398_d_n11, assign18850_e26398_d_n12, assign18850_e26398_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18850_e26391: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign18850_e26392: f64 = (0.5 * assign18850_e26391);
        let assign18850_e26395: f64 = (1e-10 * 0.01);
        let assign18850_e26396: f64 = (assign18850_e26392 + assign18850_e26395);
        (assign18850_e26396, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18850_e26398;
        locals.var_t1__blk573_dn0 = assign18850_e26398_d_n0;
        locals.var_t1__blk573_dn2 = assign18850_e26398_d_n2;
        locals.var_t1__blk573_dn6 = assign18850_e26398_d_n6;
        locals.var_t1__blk573_dn7 = assign18850_e26398_d_n7;
        locals.var_t1__blk573_dn10 = assign18850_e26398_d_n10;
        locals.var_t1__blk573_dn11 = assign18850_e26398_d_n11;
        locals.var_t1__blk573_dn12 = assign18850_e26398_d_n12;
        locals.var_t1__blk573_dn17 = assign18850_e26398_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let assign18860_e26401: f64 = if locals.var_t1__blk573 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign18860_e26401;
        locals.var_guard583_rv = 0.0;

        let (assign18870_e26409, assign18870_e26409_d_n0, assign18870_e26409_d_n2, assign18870_e26409_d_n6, assign18870_e26409_d_n7, assign18870_e26409_d_n10, assign18870_e26409_d_n11, assign18870_e26409_d_n12, assign18870_e26409_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard583 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18870_e26409;
        locals.var_t1__blk573_dn0 = assign18870_e26409_d_n0;
        locals.var_t1__blk573_dn2 = assign18870_e26409_d_n2;
        locals.var_t1__blk573_dn6 = assign18870_e26409_d_n6;
        locals.var_t1__blk573_dn7 = assign18870_e26409_d_n7;
        locals.var_t1__blk573_dn10 = assign18870_e26409_d_n10;
        locals.var_t1__blk573_dn11 = assign18870_e26409_d_n11;
        locals.var_t1__blk573_dn12 = assign18870_e26409_d_n12;
        locals.var_t1__blk573_dn17 = assign18870_e26409_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let (assign18880_e26418, assign18880_e26418_d_n0, assign18880_e26418_d_n2, assign18880_e26418_d_n6, assign18880_e26418_d_n7, assign18880_e26418_d_n10, assign18880_e26418_d_n11, assign18880_e26418_d_n12, assign18880_e26418_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18880_e26415: f64 = (locals.var_t1__blk573 + 1e-50);
        let assign18880_e26416: f64 = (assign18880_e26415).sqrt();
        (assign18880_e26416, (locals.var_t1__blk573_dn0 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn2 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn6 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn7 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn10 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn11 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn12 / (2.0 * assign18880_e26416)), (locals.var_t1__blk573_dn17 / (2.0 * assign18880_e26416)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn12, locals.var_costi6_dn17,)
    }
};
        locals.var_costi6 = assign18880_e26418;
        locals.var_costi6_dn0 = assign18880_e26418_d_n0;
        locals.var_costi6_dn2 = assign18880_e26418_d_n2;
        locals.var_costi6_dn6 = assign18880_e26418_d_n6;
        locals.var_costi6_dn7 = assign18880_e26418_d_n7;
        locals.var_costi6_dn10 = assign18880_e26418_d_n10;
        locals.var_costi6_dn11 = assign18880_e26418_d_n11;
        locals.var_costi6_dn12 = assign18880_e26418_d_n12;
        locals.var_costi6_dn17 = assign18880_e26418_d_n17;
        locals.var_costi6_rv = 0.0;

        let (assign18890_e26430, assign18890_e26430_d_n0, assign18890_e26430_d_n2, assign18890_e26430_d_n6, assign18890_e26430_d_n7, assign18890_e26430_d_n10, assign18890_e26430_d_n11, assign18890_e26430_d_n12, assign18890_e26430_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18890_e26426: f64 = (1.0 - locals.var_costi6);
        let assign18890_e26427: f64 = (locals.var_costi4 * assign18890_e26426);
        let assign18890_e26428: f64 = (locals.var_vgssti + assign18890_e26427);
        (assign18890_e26428, (locals.var_vgssti_dn0 + ((locals.var_costi4_dn0 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn0)))), (locals.var_vgssti_dn2 + ((locals.var_costi4_dn2 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn2)))), (locals.var_vgssti_dn6 + ((locals.var_costi4_dn6 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn6)))), (locals.var_vgssti_dn7 + ((locals.var_costi4_dn7 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn7)))), (locals.var_vgssti_dn10 + ((locals.var_costi4_dn10 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn10)))), (locals.var_vgssti_dn11 + ((locals.var_costi4_dn11 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn11)))), (locals.var_vgssti_dn12 + ((locals.var_costi4_dn12 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn12)))), (locals.var_vgssti_dn17 + ((locals.var_costi4_dn17 * assign18890_e26426) + (locals.var_costi4 * (-locals.var_costi6_dn17)))),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn12, locals.var_psasti_dn17,)
    }
};
        locals.var_psasti = assign18890_e26430;
        locals.var_psasti_dn0 = assign18890_e26430_d_n0;
        locals.var_psasti_dn2 = assign18890_e26430_d_n2;
        locals.var_psasti_dn6 = assign18890_e26430_d_n6;
        locals.var_psasti_dn7 = assign18890_e26430_d_n7;
        locals.var_psasti_dn10 = assign18890_e26430_d_n10;
        locals.var_psasti_dn11 = assign18890_e26430_d_n11;
        locals.var_psasti_dn12 = assign18890_e26430_d_n12;
        locals.var_psasti_dn17 = assign18890_e26430_d_n17;
        locals.var_psasti_rv = 0.0;

        let (assign18900_e26444, assign18900_e26444_d_n0, assign18900_e26444_d_n2, assign18900_e26444_d_n6, assign18900_e26444_d_n7, assign18900_e26444_d_n10, assign18900_e26444_d_n11, assign18900_e26444_d_n12, assign18900_e26444_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18900_e26439: f64 = (locals.var_vgssti + 1e-50);
        let assign18900_e26440: f64 = (2.0 / assign18900_e26439);
        let assign18900_e26441: f64 = (locals.var_beta + assign18900_e26440);
        let assign18900_e26442: f64 = (1.0 / assign18900_e26441);
        (assign18900_e26442, (-((-((2.0 * locals.var_vgssti_dn0) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn2) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn6) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn7) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (assign18900_e26439 * assign18900_e26439)))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn11) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn12) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))), (-((-((2.0 * locals.var_vgssti_dn17) / (assign18900_e26439 * assign18900_e26439))) / (assign18900_e26441 * assign18900_e26441))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18900_e26444;
        locals.var_t0_dn0 = assign18900_e26444_d_n0;
        locals.var_t0_dn2 = assign18900_e26444_d_n2;
        locals.var_t0_dn6 = assign18900_e26444_d_n6;
        locals.var_t0_dn7 = assign18900_e26444_d_n7;
        locals.var_t0_dn10 = assign18900_e26444_d_n10;
        locals.var_t0_dn11 = assign18900_e26444_d_n11;
        locals.var_t0_dn12 = assign18900_e26444_d_n12;
        locals.var_t0_dn17 = assign18900_e26444_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18910_e26461, assign18910_e26461_d_n0, assign18910_e26461_d_n2, assign18910_e26461_d_n6, assign18910_e26461_d_n7, assign18910_e26461_d_n10, assign18910_e26461_d_n11, assign18910_e26461_d_n12, assign18910_e26461_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18910_e26450: f64 = (1.0 / locals.var_costi1);
        let assign18910_e26452: f64 = (assign18910_e26450 / locals.var_costi3);
        let assign18910_e26455: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign18910_e26456: f64 = (assign18910_e26452 * assign18910_e26455);
        let assign18910_e26457: f64 = (assign18910_e26456).ln();
        let assign18910_e26459: f64 = (assign18910_e26457 * locals.var_t0);
        (assign18910_e26459, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn7)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn11)), (((((((((-(locals.var_costi1_dn12 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn12)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn12 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn12)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn12)), (((((((((-(locals.var_costi1_dn17 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign18910_e26450 * locals.var_costi3_dn17)) / (locals.var_costi3 * locals.var_costi3)) * assign18910_e26455) + (assign18910_e26452 * ((locals.var_vgssti_dn17 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn17)))) / assign18910_e26456) * locals.var_t0) + (assign18910_e26457 * locals.var_t0_dn17)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn12, locals.var_psbsti_dn17,)
    }
};
        locals.var_psbsti = assign18910_e26461;
        locals.var_psbsti_dn0 = assign18910_e26461_d_n0;
        locals.var_psbsti_dn2 = assign18910_e26461_d_n2;
        locals.var_psbsti_dn6 = assign18910_e26461_d_n6;
        locals.var_psbsti_dn7 = assign18910_e26461_d_n7;
        locals.var_psbsti_dn10 = assign18910_e26461_d_n10;
        locals.var_psbsti_dn11 = assign18910_e26461_d_n11;
        locals.var_psbsti_dn12 = assign18910_e26461_d_n12;
        locals.var_psbsti_dn17 = assign18910_e26461_d_n17;
        locals.var_psbsti_rv = 0.0;

        let (assign18920_e26471, assign18920_e26471_d_n0, assign18920_e26471_d_n2, assign18920_e26471_d_n6, assign18920_e26471_d_n7, assign18920_e26471_d_n10, assign18920_e26471_d_n11, assign18920_e26471_d_n12, assign18920_e26471_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18920_e26468: f64 = (locals.var_vgssti + 1e-50);
        let assign18920_e26469: f64 = (locals.var_psbsti / assign18920_e26468);
        (assign18920_e26469, (((locals.var_psbsti_dn0 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn0)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn2 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn2)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn6 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn6)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn7 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn7)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn10 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn10)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn11 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn11)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn12 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn12)) / (assign18920_e26468 * assign18920_e26468)), (((locals.var_psbsti_dn17 * assign18920_e26468) - (locals.var_psbsti * locals.var_vgssti_dn17)) / (assign18920_e26468 * assign18920_e26468)),)
    } else {
        (locals.var_t3__blk576, locals.var_t3__blk576_dn0, locals.var_t3__blk576_dn2, locals.var_t3__blk576_dn6, locals.var_t3__blk576_dn7, locals.var_t3__blk576_dn10, locals.var_t3__blk576_dn11, locals.var_t3__blk576_dn12, locals.var_t3__blk576_dn17,)
    }
};
        locals.var_t3__blk576 = assign18920_e26471;
        locals.var_t3__blk576_dn0 = assign18920_e26471_d_n0;
        locals.var_t3__blk576_dn2 = assign18920_e26471_d_n2;
        locals.var_t3__blk576_dn6 = assign18920_e26471_d_n6;
        locals.var_t3__blk576_dn7 = assign18920_e26471_d_n7;
        locals.var_t3__blk576_dn10 = assign18920_e26471_d_n10;
        locals.var_t3__blk576_dn11 = assign18920_e26471_d_n11;
        locals.var_t3__blk576_dn12 = assign18920_e26471_d_n12;
        locals.var_t3__blk576_dn17 = assign18920_e26471_d_n17;
        locals.var_t3__blk576_rv = 0.0;

        let (assign18930_e26481, assign18930_e26481_d_n0, assign18930_e26481_d_n2, assign18930_e26481_d_n6, assign18930_e26481_d_n7, assign18930_e26481_d_n10, assign18930_e26481_d_n11, assign18930_e26481_d_n12, assign18930_e26481_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18930_e26477: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign18930_e26479: f64 = (assign18930_e26477 - 0.002);
        (assign18930_e26479, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn12 - locals.var_psasti_dn12), (locals.var_psbsti_dn17 - locals.var_psasti_dn17),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn12, locals.var_psab_dn17,)
    }
};
        locals.var_psab = assign18930_e26481;
        locals.var_psab_dn0 = assign18930_e26481_d_n0;
        locals.var_psab_dn2 = assign18930_e26481_d_n2;
        locals.var_psab_dn6 = assign18930_e26481_d_n6;
        locals.var_psab_dn7 = assign18930_e26481_d_n7;
        locals.var_psab_dn10 = assign18930_e26481_d_n10;
        locals.var_psab_dn11 = assign18930_e26481_d_n11;
        locals.var_psab_dn12 = assign18930_e26481_d_n12;
        locals.var_psab_dn17 = assign18930_e26481_d_n17;
        locals.var_psab_rv = 0.0;

        let (assign18940_e26496, assign18940_e26496_d_n0, assign18940_e26496_d_n2, assign18940_e26496_d_n6, assign18940_e26496_d_n7, assign18940_e26496_d_n10, assign18940_e26496_d_n11, assign18940_e26496_d_n12, assign18940_e26496_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18940_e26487: f64 = (locals.var_psab * locals.var_psab);
        let assign18940_e26490: f64 = (4.0 * 0.002);
        let assign18940_e26492: f64 = (assign18940_e26490 * locals.var_psbsti);
        let assign18940_e26493: f64 = (assign18940_e26487 + assign18940_e26492);
        let assign18940_e26494: f64 = (assign18940_e26493).sqrt();
        (assign18940_e26494, ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign18940_e26490 * locals.var_psbsti_dn0)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign18940_e26490 * locals.var_psbsti_dn2)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign18940_e26490 * locals.var_psbsti_dn6)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign18940_e26490 * locals.var_psbsti_dn7)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign18940_e26490 * locals.var_psbsti_dn10)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign18940_e26490 * locals.var_psbsti_dn11)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn12 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn12)) + (assign18940_e26490 * locals.var_psbsti_dn12)) / (2.0 * assign18940_e26494)), ((((locals.var_psab_dn17 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn17)) + (assign18940_e26490 * locals.var_psbsti_dn17)) / (2.0 * assign18940_e26494)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18940_e26496;
        locals.var_t0_dn0 = assign18940_e26496_d_n0;
        locals.var_t0_dn2 = assign18940_e26496_d_n2;
        locals.var_t0_dn6 = assign18940_e26496_d_n6;
        locals.var_t0_dn7 = assign18940_e26496_d_n7;
        locals.var_t0_dn10 = assign18940_e26496_d_n10;
        locals.var_t0_dn11 = assign18940_e26496_d_n11;
        locals.var_t0_dn12 = assign18940_e26496_d_n12;
        locals.var_t0_dn17 = assign18940_e26496_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18950_e26508, assign18950_e26508_d_n0, assign18950_e26508_d_n2, assign18950_e26508_d_n6, assign18950_e26508_d_n7, assign18950_e26508_d_n10, assign18950_e26508_d_n11, assign18950_e26508_d_n12, assign18950_e26508_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18950_e26504: f64 = (locals.var_psab + locals.var_t0);
        let assign18950_e26505: f64 = (0.5 * assign18950_e26504);
        let assign18950_e26506: f64 = (locals.var_psbsti - assign18950_e26505);
        (assign18950_e26506, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + locals.var_t0_dn7))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + locals.var_t0_dn11))), (locals.var_psbsti_dn12 - (0.5 * (locals.var_psab_dn12 + locals.var_t0_dn12))), (locals.var_psbsti_dn17 - (0.5 * (locals.var_psab_dn17 + locals.var_t0_dn17))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12, locals.var_psti_dn17,)
    }
};
        locals.var_psti = assign18950_e26508;
        locals.var_psti_dn0 = assign18950_e26508_d_n0;
        locals.var_psti_dn2 = assign18950_e26508_d_n2;
        locals.var_psti_dn6 = assign18950_e26508_d_n6;
        locals.var_psti_dn7 = assign18950_e26508_d_n7;
        locals.var_psti_dn10 = assign18950_e26508_d_n10;
        locals.var_psti_dn11 = assign18950_e26508_d_n11;
        locals.var_psti_dn12 = assign18950_e26508_d_n12;
        locals.var_psti_dn17 = assign18950_e26508_d_n17;
        locals.var_psti_rv = 0.0;

        let (assign18960_e26516, assign18960_e26516_d_n0, assign18960_e26516_d_n2, assign18960_e26516_d_n6, assign18960_e26516_d_n7, assign18960_e26516_d_n10, assign18960_e26516_d_n11, assign18960_e26516_d_n12, assign18960_e26516_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18960_e26514: f64 = (1.0 / locals.var_t0);
        (assign18960_e26514, (-(locals.var_t0_dn0 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn2 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn12 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn17 / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign18960_e26516;
        locals.var_t1__blk573_dn0 = assign18960_e26516_d_n0;
        locals.var_t1__blk573_dn2 = assign18960_e26516_d_n2;
        locals.var_t1__blk573_dn6 = assign18960_e26516_d_n6;
        locals.var_t1__blk573_dn7 = assign18960_e26516_d_n7;
        locals.var_t1__blk573_dn10 = assign18960_e26516_d_n10;
        locals.var_t1__blk573_dn11 = assign18960_e26516_d_n11;
        locals.var_t1__blk573_dn12 = assign18960_e26516_d_n12;
        locals.var_t1__blk573_dn17 = assign18960_e26516_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let (assign18970_e26527, assign18970_e26527_d_n0, assign18970_e26527_d_n2, assign18970_e26527_d_n6, assign18970_e26527_d_n7, assign18970_e26527_d_n10, assign18970_e26527_d_n11, assign18970_e26527_d_n12, assign18970_e26527_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18970_e26523: f64 = (locals.var_beta * locals.var_psti);
        let assign18970_e26524: f64 = (assign18970_e26523).exp();
        let assign18970_e26525: f64 = (locals.var_costi1 * assign18970_e26524);
        (assign18970_e26525, ((locals.var_costi1_dn0 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn0)))), ((locals.var_costi1_dn2 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn2)))), ((locals.var_costi1_dn6 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn6)))), ((locals.var_costi1_dn7 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn7)))), ((locals.var_costi1_dn10 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))))), ((locals.var_costi1_dn11 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn11)))), ((locals.var_costi1_dn12 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn12)))), ((locals.var_costi1_dn17 * assign18970_e26524) + (locals.var_costi1 * (assign18970_e26524 * (locals.var_beta * locals.var_psti_dn17)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign18970_e26527;
        locals.var_t0_dn0 = assign18970_e26527_d_n0;
        locals.var_t0_dn2 = assign18970_e26527_d_n2;
        locals.var_t0_dn6 = assign18970_e26527_d_n6;
        locals.var_t0_dn7 = assign18970_e26527_d_n7;
        locals.var_t0_dn10 = assign18970_e26527_d_n10;
        locals.var_t0_dn11 = assign18970_e26527_d_n11;
        locals.var_t0_dn12 = assign18970_e26527_d_n12;
        locals.var_t0_dn17 = assign18970_e26527_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign18980_e26541, assign18980_e26541_d_n0, assign18980_e26541_d_n2, assign18980_e26541_d_n6, assign18980_e26541_d_n7, assign18980_e26541_d_n10, assign18980_e26541_d_n11, assign18980_e26541_d_n12, assign18980_e26541_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18980_e26534: f64 = (locals.var_psti - locals.var_vbspz);
        let assign18980_e26535: f64 = (locals.var_beta * assign18980_e26534);
        let assign18980_e26537: f64 = (assign18980_e26535 - 1.0);
        let assign18980_e26539: f64 = (assign18980_e26537 + locals.var_t0);
        (assign18980_e26539, ((locals.var_beta * (locals.var_psti_dn0 - locals.var_vbspz_dn0)) + locals.var_t0_dn0), ((locals.var_beta * (locals.var_psti_dn2 - locals.var_vbspz_dn2)) + locals.var_t0_dn2), ((locals.var_beta * (locals.var_psti_dn6 - locals.var_vbspz_dn6)) + locals.var_t0_dn6), ((locals.var_beta * (locals.var_psti_dn7 - locals.var_vbspz_dn7)) + locals.var_t0_dn7), (((locals.var_beta_dn10 * assign18980_e26534) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbspz_dn10))) + locals.var_t0_dn10), ((locals.var_beta * (locals.var_psti_dn11 - locals.var_vbspz_dn11)) + locals.var_t0_dn11), ((locals.var_beta * (locals.var_psti_dn12 - locals.var_vbspz_dn12)) + locals.var_t0_dn12), ((locals.var_beta * (locals.var_psti_dn17 - locals.var_vbspz_dn17)) + locals.var_t0_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign18980_e26541;
        locals.var_t1w_dn0 = assign18980_e26541_d_n0;
        locals.var_t1w_dn2 = assign18980_e26541_d_n2;
        locals.var_t1w_dn6 = assign18980_e26541_d_n6;
        locals.var_t1w_dn7 = assign18980_e26541_d_n7;
        locals.var_t1w_dn10 = assign18980_e26541_d_n10;
        locals.var_t1w_dn11 = assign18980_e26541_d_n11;
        locals.var_t1w_dn12 = assign18980_e26541_d_n12;
        locals.var_t1w_dn17 = assign18980_e26541_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign18990_e26556, assign18990_e26556_d_n0, assign18990_e26556_d_n2, assign18990_e26556_d_n6, assign18990_e26556_d_n7, assign18990_e26556_d_n10, assign18990_e26556_d_n11, assign18990_e26556_d_n12, assign18990_e26556_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign18990_e26547: f64 = (locals.var_t1w * locals.var_t1w);
        let assign18990_e26550: f64 = (4.0 * 0.01);
        let assign18990_e26552: f64 = (assign18990_e26550 * 0.01);
        let assign18990_e26553: f64 = (assign18990_e26547 + assign18990_e26552);
        let assign18990_e26554: f64 = (assign18990_e26553).sqrt();
        (assign18990_e26554, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign18990_e26554)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign18990_e26554)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign18990_e26554)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign18990_e26554)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign18990_e26554)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign18990_e26554)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign18990_e26554)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign18990_e26554)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign18990_e26556;
        locals.var_tmf1_dn0 = assign18990_e26556_d_n0;
        locals.var_tmf1_dn2 = assign18990_e26556_d_n2;
        locals.var_tmf1_dn6 = assign18990_e26556_d_n6;
        locals.var_tmf1_dn7 = assign18990_e26556_d_n7;
        locals.var_tmf1_dn10 = assign18990_e26556_d_n10;
        locals.var_tmf1_dn11 = assign18990_e26556_d_n11;
        locals.var_tmf1_dn12 = assign18990_e26556_d_n12;
        locals.var_tmf1_dn17 = assign18990_e26556_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19000_e26570, assign19000_e26570_d_n0, assign19000_e26570_d_n2, assign19000_e26570_d_n6, assign19000_e26570_d_n7, assign19000_e26570_d_n10, assign19000_e26570_d_n11, assign19000_e26570_d_n12, assign19000_e26570_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19000_e26563: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19000_e26564: f64 = (0.5 * assign19000_e26563);
        let assign19000_e26567: f64 = (1e-10 * 0.01);
        let assign19000_e26568: f64 = (assign19000_e26564 + assign19000_e26567);
        (assign19000_e26568, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign19000_e26570;
        locals.var_t1__blk573_dn0 = assign19000_e26570_d_n0;
        locals.var_t1__blk573_dn2 = assign19000_e26570_d_n2;
        locals.var_t1__blk573_dn6 = assign19000_e26570_d_n6;
        locals.var_t1__blk573_dn7 = assign19000_e26570_d_n7;
        locals.var_t1__blk573_dn10 = assign19000_e26570_d_n10;
        locals.var_t1__blk573_dn11 = assign19000_e26570_d_n11;
        locals.var_t1__blk573_dn12 = assign19000_e26570_d_n12;
        locals.var_t1__blk573_dn17 = assign19000_e26570_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let assign19010_e26573: f64 = if locals.var_t1__blk573 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign19010_e26573;
        locals.var_guard584_rv = 0.0;

        let (assign19020_e26581, assign19020_e26581_d_n0, assign19020_e26581_d_n2, assign19020_e26581_d_n6, assign19020_e26581_d_n7, assign19020_e26581_d_n10, assign19020_e26581_d_n11, assign19020_e26581_d_n12, assign19020_e26581_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard584 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign19020_e26581;
        locals.var_t1__blk573_dn0 = assign19020_e26581_d_n0;
        locals.var_t1__blk573_dn2 = assign19020_e26581_d_n2;
        locals.var_t1__blk573_dn6 = assign19020_e26581_d_n6;
        locals.var_t1__blk573_dn7 = assign19020_e26581_d_n7;
        locals.var_t1__blk573_dn10 = assign19020_e26581_d_n10;
        locals.var_t1__blk573_dn11 = assign19020_e26581_d_n11;
        locals.var_t1__blk573_dn12 = assign19020_e26581_d_n12;
        locals.var_t1__blk573_dn17 = assign19020_e26581_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let (assign19030_e26592, assign19030_e26592_d_n0, assign19030_e26592_d_n2, assign19030_e26592_d_n6, assign19030_e26592_d_n7, assign19030_e26592_d_n10, assign19030_e26592_d_n11, assign19030_e26592_d_n12, assign19030_e26592_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19030_e26588: f64 = (10.0 * 2.220446049250313e-16);
        let assign19030_e26589: f64 = (locals.var_t1__blk573 + assign19030_e26588);
        let assign19030_e26590: f64 = (assign19030_e26589).sqrt();
        (assign19030_e26590, (locals.var_t1__blk573_dn0 / (2.0 * assign19030_e26590)), (locals.var_t1__blk573_dn2 / (2.0 * assign19030_e26590)), (locals.var_t1__blk573_dn6 / (2.0 * assign19030_e26590)), (locals.var_t1__blk573_dn7 / (2.0 * assign19030_e26590)), (locals.var_t1__blk573_dn10 / (2.0 * assign19030_e26590)), (locals.var_t1__blk573_dn11 / (2.0 * assign19030_e26590)), (locals.var_t1__blk573_dn12 / (2.0 * assign19030_e26590)), (locals.var_t1__blk573_dn17 / (2.0 * assign19030_e26590)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12, locals.var_sq1sti_dn17,)
    }
};
        locals.var_sq1sti = assign19030_e26592;
        locals.var_sq1sti_dn0 = assign19030_e26592_d_n0;
        locals.var_sq1sti_dn2 = assign19030_e26592_d_n2;
        locals.var_sq1sti_dn6 = assign19030_e26592_d_n6;
        locals.var_sq1sti_dn7 = assign19030_e26592_d_n7;
        locals.var_sq1sti_dn10 = assign19030_e26592_d_n10;
        locals.var_sq1sti_dn11 = assign19030_e26592_d_n11;
        locals.var_sq1sti_dn12 = assign19030_e26592_d_n12;
        locals.var_sq1sti_dn17 = assign19030_e26592_d_n17;
        locals.var_sq1sti_rv = 0.0;

        let (assign19040_e26604, assign19040_e26604_d_n0, assign19040_e26604_d_n2, assign19040_e26604_d_n6, assign19040_e26604_d_n7, assign19040_e26604_d_n10, assign19040_e26604_d_n11, assign19040_e26604_d_n12, assign19040_e26604_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19040_e26599: f64 = (locals.var_psti - locals.var_vbspz);
        let assign19040_e26600: f64 = (locals.var_beta * assign19040_e26599);
        let assign19040_e26602: f64 = (assign19040_e26600 - 1.0);
        (assign19040_e26602, (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbspz_dn0)), (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbspz_dn2)), (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbspz_dn6)), (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbspz_dn7)), ((locals.var_beta_dn10 * assign19040_e26599) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbspz_dn10))), (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbspz_dn11)), (locals.var_beta * (locals.var_psti_dn12 - locals.var_vbspz_dn12)), (locals.var_beta * (locals.var_psti_dn17 - locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19040_e26604;
        locals.var_t1w_dn0 = assign19040_e26604_d_n0;
        locals.var_t1w_dn2 = assign19040_e26604_d_n2;
        locals.var_t1w_dn6 = assign19040_e26604_d_n6;
        locals.var_t1w_dn7 = assign19040_e26604_d_n7;
        locals.var_t1w_dn10 = assign19040_e26604_d_n10;
        locals.var_t1w_dn11 = assign19040_e26604_d_n11;
        locals.var_t1w_dn12 = assign19040_e26604_d_n12;
        locals.var_t1w_dn17 = assign19040_e26604_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19050_e26619, assign19050_e26619_d_n0, assign19050_e26619_d_n2, assign19050_e26619_d_n6, assign19050_e26619_d_n7, assign19050_e26619_d_n10, assign19050_e26619_d_n11, assign19050_e26619_d_n12, assign19050_e26619_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19050_e26610: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19050_e26613: f64 = (4.0 * 0.01);
        let assign19050_e26615: f64 = (assign19050_e26613 * 0.01);
        let assign19050_e26616: f64 = (assign19050_e26610 + assign19050_e26615);
        let assign19050_e26617: f64 = (assign19050_e26616).sqrt();
        (assign19050_e26617, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19050_e26617)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19050_e26617)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19050_e26617)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19050_e26617)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19050_e26617)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19050_e26617)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19050_e26617)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19050_e26617)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19050_e26619;
        locals.var_tmf1_dn0 = assign19050_e26619_d_n0;
        locals.var_tmf1_dn2 = assign19050_e26619_d_n2;
        locals.var_tmf1_dn6 = assign19050_e26619_d_n6;
        locals.var_tmf1_dn7 = assign19050_e26619_d_n7;
        locals.var_tmf1_dn10 = assign19050_e26619_d_n10;
        locals.var_tmf1_dn11 = assign19050_e26619_d_n11;
        locals.var_tmf1_dn12 = assign19050_e26619_d_n12;
        locals.var_tmf1_dn17 = assign19050_e26619_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19060_e26633, assign19060_e26633_d_n0, assign19060_e26633_d_n2, assign19060_e26633_d_n6, assign19060_e26633_d_n7, assign19060_e26633_d_n10, assign19060_e26633_d_n11, assign19060_e26633_d_n12, assign19060_e26633_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19060_e26626: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19060_e26627: f64 = (0.5 * assign19060_e26626);
        let assign19060_e26630: f64 = (1e-10 * 0.01);
        let assign19060_e26631: f64 = (assign19060_e26627 + assign19060_e26630);
        (assign19060_e26631, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign19060_e26633;
        locals.var_t1__blk573_dn0 = assign19060_e26633_d_n0;
        locals.var_t1__blk573_dn2 = assign19060_e26633_d_n2;
        locals.var_t1__blk573_dn6 = assign19060_e26633_d_n6;
        locals.var_t1__blk573_dn7 = assign19060_e26633_d_n7;
        locals.var_t1__blk573_dn10 = assign19060_e26633_d_n10;
        locals.var_t1__blk573_dn11 = assign19060_e26633_d_n11;
        locals.var_t1__blk573_dn12 = assign19060_e26633_d_n12;
        locals.var_t1__blk573_dn17 = assign19060_e26633_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let assign19070_e26636: f64 = if locals.var_t1__blk573 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign19070_e26636;
        locals.var_guard585_rv = 0.0;

        let (assign19080_e26644, assign19080_e26644_d_n0, assign19080_e26644_d_n2, assign19080_e26644_d_n6, assign19080_e26644_d_n7, assign19080_e26644_d_n10, assign19080_e26644_d_n11, assign19080_e26644_d_n12, assign19080_e26644_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard585 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign19080_e26644;
        locals.var_t1__blk573_dn0 = assign19080_e26644_d_n0;
        locals.var_t1__blk573_dn2 = assign19080_e26644_d_n2;
        locals.var_t1__blk573_dn6 = assign19080_e26644_d_n6;
        locals.var_t1__blk573_dn7 = assign19080_e26644_d_n7;
        locals.var_t1__blk573_dn10 = assign19080_e26644_d_n10;
        locals.var_t1__blk573_dn11 = assign19080_e26644_d_n11;
        locals.var_t1__blk573_dn12 = assign19080_e26644_d_n12;
        locals.var_t1__blk573_dn17 = assign19080_e26644_d_n17;
        locals.var_t1__blk573_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign19090_e26655, assign19090_e26655_d_n0, assign19090_e26655_d_n2, assign19090_e26655_d_n6, assign19090_e26655_d_n7, assign19090_e26655_d_n10, assign19090_e26655_d_n11, assign19090_e26655_d_n12, assign19090_e26655_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19090_e26651: f64 = (10.0 * 2.220446049250313e-16);
        let assign19090_e26652: f64 = (locals.var_t1__blk573 + assign19090_e26651);
        let assign19090_e26653: f64 = (assign19090_e26652).sqrt();
        (assign19090_e26653, (locals.var_t1__blk573_dn0 / (2.0 * assign19090_e26653)), (locals.var_t1__blk573_dn2 / (2.0 * assign19090_e26653)), (locals.var_t1__blk573_dn6 / (2.0 * assign19090_e26653)), (locals.var_t1__blk573_dn7 / (2.0 * assign19090_e26653)), (locals.var_t1__blk573_dn10 / (2.0 * assign19090_e26653)), (locals.var_t1__blk573_dn11 / (2.0 * assign19090_e26653)), (locals.var_t1__blk573_dn12 / (2.0 * assign19090_e26653)), (locals.var_t1__blk573_dn17 / (2.0 * assign19090_e26653)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12, locals.var_sq2sti_dn17,)
    }
};
        locals.var_sq2sti = assign19090_e26655;
        locals.var_sq2sti_dn0 = assign19090_e26655_d_n0;
        locals.var_sq2sti_dn2 = assign19090_e26655_d_n2;
        locals.var_sq2sti_dn6 = assign19090_e26655_d_n6;
        locals.var_sq2sti_dn7 = assign19090_e26655_d_n7;
        locals.var_sq2sti_dn10 = assign19090_e26655_d_n10;
        locals.var_sq2sti_dn11 = assign19090_e26655_d_n11;
        locals.var_sq2sti_dn12 = assign19090_e26655_d_n12;
        locals.var_sq2sti_dn17 = assign19090_e26655_d_n17;
        locals.var_sq2sti_rv = 0.0;

        let (assign19100_e26665, assign19100_e26665_d_n0, assign19100_e26665_d_n2, assign19100_e26665_d_n6, assign19100_e26665_d_n7, assign19100_e26665_d_n10, assign19100_e26665_d_n11, assign19100_e26665_d_n12, assign19100_e26665_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19100_e26662: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign19100_e26663: f64 = (locals.var_costi0 * assign19100_e26662);
        (assign19100_e26663, ((locals.var_costi0_dn0 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn6 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn10 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn12 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn12 - locals.var_sq2sti_dn12))), ((locals.var_costi0_dn17 * assign19100_e26662) + (locals.var_costi0 * (locals.var_sq1sti_dn17 - locals.var_sq2sti_dn17))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn12, locals.var_qn0sti_dn17,)
    }
};
        locals.var_qn0sti = assign19100_e26665;
        locals.var_qn0sti_dn0 = assign19100_e26665_d_n0;
        locals.var_qn0sti_dn2 = assign19100_e26665_d_n2;
        locals.var_qn0sti_dn6 = assign19100_e26665_d_n6;
        locals.var_qn0sti_dn7 = assign19100_e26665_d_n7;
        locals.var_qn0sti_dn10 = assign19100_e26665_d_n10;
        locals.var_qn0sti_dn11 = assign19100_e26665_d_n11;
        locals.var_qn0sti_dn12 = assign19100_e26665_d_n12;
        locals.var_qn0sti_dn17 = assign19100_e26665_d_n17;
        locals.var_qn0sti_rv = 0.0;

        let (assign19110_e26673, assign19110_e26673_d_n0, assign19110_e26673_d_n2, assign19110_e26673_d_n6, assign19110_e26673_d_n7, assign19110_e26673_d_n10, assign19110_e26673_d_n11, assign19110_e26673_d_n12, assign19110_e26673_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19110_e26671: f64 = (locals.var_psasti - locals.var_psti);
        (assign19110_e26671, (locals.var_psasti_dn0 - locals.var_psti_dn0), (locals.var_psasti_dn2 - locals.var_psti_dn2), (locals.var_psasti_dn6 - locals.var_psti_dn6), (locals.var_psasti_dn7 - locals.var_psti_dn7), (locals.var_psasti_dn10 - locals.var_psti_dn10), (locals.var_psasti_dn11 - locals.var_psti_dn11), (locals.var_psasti_dn12 - locals.var_psti_dn12), (locals.var_psasti_dn17 - locals.var_psti_dn17),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn12, locals.var_t1w_dn17,)
    }
};
        locals.var_t1w = assign19110_e26673;
        locals.var_t1w_dn0 = assign19110_e26673_d_n0;
        locals.var_t1w_dn2 = assign19110_e26673_d_n2;
        locals.var_t1w_dn6 = assign19110_e26673_d_n6;
        locals.var_t1w_dn7 = assign19110_e26673_d_n7;
        locals.var_t1w_dn10 = assign19110_e26673_d_n10;
        locals.var_t1w_dn11 = assign19110_e26673_d_n11;
        locals.var_t1w_dn12 = assign19110_e26673_d_n12;
        locals.var_t1w_dn17 = assign19110_e26673_d_n17;
        locals.var_t1w_rv = 0.0;

        let (assign19120_e26688, assign19120_e26688_d_n0, assign19120_e26688_d_n2, assign19120_e26688_d_n6, assign19120_e26688_d_n7, assign19120_e26688_d_n10, assign19120_e26688_d_n11, assign19120_e26688_d_n12, assign19120_e26688_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19120_e26679: f64 = (locals.var_t1w * locals.var_t1w);
        let assign19120_e26682: f64 = (4.0 * 0.1);
        let assign19120_e26684: f64 = (assign19120_e26682 * 0.1);
        let assign19120_e26685: f64 = (assign19120_e26679 + assign19120_e26684);
        let assign19120_e26686: f64 = (assign19120_e26685).sqrt();
        (assign19120_e26686, (((locals.var_t1w_dn0 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn0)) / (2.0 * assign19120_e26686)), (((locals.var_t1w_dn2 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn2)) / (2.0 * assign19120_e26686)), (((locals.var_t1w_dn6 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn6)) / (2.0 * assign19120_e26686)), (((locals.var_t1w_dn7 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn7)) / (2.0 * assign19120_e26686)), (((locals.var_t1w_dn10 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn10)) / (2.0 * assign19120_e26686)), (((locals.var_t1w_dn11 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn11)) / (2.0 * assign19120_e26686)), (((locals.var_t1w_dn12 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn12)) / (2.0 * assign19120_e26686)), (((locals.var_t1w_dn17 * locals.var_t1w) + (locals.var_t1w * locals.var_t1w_dn17)) / (2.0 * assign19120_e26686)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19120_e26688;
        locals.var_tmf1_dn0 = assign19120_e26688_d_n0;
        locals.var_tmf1_dn2 = assign19120_e26688_d_n2;
        locals.var_tmf1_dn6 = assign19120_e26688_d_n6;
        locals.var_tmf1_dn7 = assign19120_e26688_d_n7;
        locals.var_tmf1_dn10 = assign19120_e26688_d_n10;
        locals.var_tmf1_dn11 = assign19120_e26688_d_n11;
        locals.var_tmf1_dn12 = assign19120_e26688_d_n12;
        locals.var_tmf1_dn17 = assign19120_e26688_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19130_e26702, assign19130_e26702_d_n0, assign19130_e26702_d_n2, assign19130_e26702_d_n6, assign19130_e26702_d_n7, assign19130_e26702_d_n10, assign19130_e26702_d_n11, assign19130_e26702_d_n12, assign19130_e26702_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19130_e26695: f64 = (locals.var_t1w + locals.var_tmf1);
        let assign19130_e26696: f64 = (0.5 * assign19130_e26695);
        let assign19130_e26699: f64 = (1e-10 * 0.1);
        let assign19130_e26700: f64 = (assign19130_e26696 + assign19130_e26699);
        (assign19130_e26700, (0.5 * (locals.var_t1w_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1w_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1w_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1w_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1w_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1w_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1w_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t1w_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign19130_e26702;
        locals.var_t1__blk573_dn0 = assign19130_e26702_d_n0;
        locals.var_t1__blk573_dn2 = assign19130_e26702_d_n2;
        locals.var_t1__blk573_dn6 = assign19130_e26702_d_n6;
        locals.var_t1__blk573_dn7 = assign19130_e26702_d_n7;
        locals.var_t1__blk573_dn10 = assign19130_e26702_d_n10;
        locals.var_t1__blk573_dn11 = assign19130_e26702_d_n11;
        locals.var_t1__blk573_dn12 = assign19130_e26702_d_n12;
        locals.var_t1__blk573_dn17 = assign19130_e26702_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let assign19140_e26705: f64 = if locals.var_t1__blk573 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign19140_e26705;
        locals.var_guard586_rv = 0.0;

        let (assign19150_e26713, assign19150_e26713_d_n0, assign19150_e26713_d_n2, assign19150_e26713_d_n6, assign19150_e26713_d_n7, assign19150_e26713_d_n10, assign19150_e26713_d_n11, assign19150_e26713_d_n12, assign19150_e26713_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard586 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk573, locals.var_t1__blk573_dn0, locals.var_t1__blk573_dn2, locals.var_t1__blk573_dn6, locals.var_t1__blk573_dn7, locals.var_t1__blk573_dn10, locals.var_t1__blk573_dn11, locals.var_t1__blk573_dn12, locals.var_t1__blk573_dn17,)
    }
};
        locals.var_t1__blk573 = assign19150_e26713;
        locals.var_t1__blk573_dn0 = assign19150_e26713_d_n0;
        locals.var_t1__blk573_dn2 = assign19150_e26713_d_n2;
        locals.var_t1__blk573_dn6 = assign19150_e26713_d_n6;
        locals.var_t1__blk573_dn7 = assign19150_e26713_d_n7;
        locals.var_t1__blk573_dn10 = assign19150_e26713_d_n10;
        locals.var_t1__blk573_dn11 = assign19150_e26713_d_n11;
        locals.var_t1__blk573_dn12 = assign19150_e26713_d_n12;
        locals.var_t1__blk573_dn17 = assign19150_e26713_d_n17;
        locals.var_t1__blk573_rv = 0.0;

        let (assign19160_e26725, assign19160_e26725_d_n0, assign19160_e26725_d_n2, assign19160_e26725_d_n6, assign19160_e26725_d_n7, assign19160_e26725_d_n10, assign19160_e26725_d_n11, assign19160_e26725_d_n12, assign19160_e26725_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19160_e26721: f64 = (10.0 * 2.220446049250313e-16);
        let assign19160_e26722: f64 = (locals.var_t1__blk573 + assign19160_e26721);
        let assign19160_e26723: f64 = (locals.var_vds / assign19160_e26722);
        (assign19160_e26723, (((locals.var_vds_dn0 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn0)) / (assign19160_e26722 * assign19160_e26722)), (((locals.var_vds_dn2 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn2)) / (assign19160_e26722 * assign19160_e26722)), (((locals.var_vds_dn6 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn6)) / (assign19160_e26722 * assign19160_e26722)), (((locals.var_vds_dn7 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn7)) / (assign19160_e26722 * assign19160_e26722)), (((locals.var_vds_dn10 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn10)) / (assign19160_e26722 * assign19160_e26722)), (((locals.var_vds_dn11 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn11)) / (assign19160_e26722 * assign19160_e26722)), (((locals.var_vds_dn12 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn12)) / (assign19160_e26722 * assign19160_e26722)), (((locals.var_vds_dn17 * assign19160_e26722) - (locals.var_vds * locals.var_t1__blk573_dn17)) / (assign19160_e26722 * assign19160_e26722)),)
    } else {
        (locals.var_tx__blk580, locals.var_tx__blk580_dn0, locals.var_tx__blk580_dn2, locals.var_tx__blk580_dn6, locals.var_tx__blk580_dn7, locals.var_tx__blk580_dn10, locals.var_tx__blk580_dn11, locals.var_tx__blk580_dn12, locals.var_tx__blk580_dn17,)
    }
};
        locals.var_tx__blk580 = assign19160_e26725;
        locals.var_tx__blk580_dn0 = assign19160_e26725_d_n0;
        locals.var_tx__blk580_dn2 = assign19160_e26725_d_n2;
        locals.var_tx__blk580_dn6 = assign19160_e26725_d_n6;
        locals.var_tx__blk580_dn7 = assign19160_e26725_d_n7;
        locals.var_tx__blk580_dn10 = assign19160_e26725_d_n10;
        locals.var_tx__blk580_dn11 = assign19160_e26725_d_n11;
        locals.var_tx__blk580_dn12 = assign19160_e26725_d_n12;
        locals.var_tx__blk580_dn17 = assign19160_e26725_d_n17;
        locals.var_tx__blk580_rv = 0.0;

        let (assign19170_e26733, assign19170_e26733_d_n0, assign19170_e26733_d_n2, assign19170_e26733_d_n6, assign19170_e26733_d_n7, assign19170_e26733_d_n10, assign19170_e26733_d_n11, assign19170_e26733_d_n12, assign19170_e26733_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19170_e26731: f64 = (locals.var_tx__blk580 * locals.var_tx__blk580);
        (assign19170_e26731, ((locals.var_tx__blk580_dn0 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn0)), ((locals.var_tx__blk580_dn2 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn2)), ((locals.var_tx__blk580_dn6 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn6)), ((locals.var_tx__blk580_dn7 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn7)), ((locals.var_tx__blk580_dn10 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn10)), ((locals.var_tx__blk580_dn11 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn11)), ((locals.var_tx__blk580_dn12 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn12)), ((locals.var_tx__blk580_dn17 * locals.var_tx__blk580) + (locals.var_tx__blk580 * locals.var_tx__blk580_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign19170_e26733;
        locals.var_x2_dn0 = assign19170_e26733_d_n0;
        locals.var_x2_dn2 = assign19170_e26733_d_n2;
        locals.var_x2_dn6 = assign19170_e26733_d_n6;
        locals.var_x2_dn7 = assign19170_e26733_d_n7;
        locals.var_x2_dn10 = assign19170_e26733_d_n10;
        locals.var_x2_dn11 = assign19170_e26733_d_n11;
        locals.var_x2_dn12 = assign19170_e26733_d_n12;
        locals.var_x2_dn17 = assign19170_e26733_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign19180_e26741, assign19180_e26741_d_n0, assign19180_e26741_d_n2, assign19180_e26741_d_n6, assign19180_e26741_d_n7, assign19180_e26741_d_n10, assign19180_e26741_d_n11, assign19180_e26741_d_n12, assign19180_e26741_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19180_e26739: f64 = 1.0;
        (assign19180_e26739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign19180_e26741;
        locals.var_xmax2_dn0 = assign19180_e26741_d_n0;
        locals.var_xmax2_dn2 = assign19180_e26741_d_n2;
        locals.var_xmax2_dn6 = assign19180_e26741_d_n6;
        locals.var_xmax2_dn7 = assign19180_e26741_d_n7;
        locals.var_xmax2_dn10 = assign19180_e26741_d_n10;
        locals.var_xmax2_dn11 = assign19180_e26741_d_n11;
        locals.var_xmax2_dn12 = assign19180_e26741_d_n12;
        locals.var_xmax2_dn17 = assign19180_e26741_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign19190_e26747, assign19190_e26747_d_n0, assign19190_e26747_d_n2, assign19190_e26747_d_n6, assign19190_e26747_d_n7, assign19190_e26747_d_n10, assign19190_e26747_d_n11, assign19190_e26747_d_n12, assign19190_e26747_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19190_e26747;
        locals.var_xp_dn0 = assign19190_e26747_d_n0;
        locals.var_xp_dn2 = assign19190_e26747_d_n2;
        locals.var_xp_dn6 = assign19190_e26747_d_n6;
        locals.var_xp_dn7 = assign19190_e26747_d_n7;
        locals.var_xp_dn10 = assign19190_e26747_d_n10;
        locals.var_xp_dn11 = assign19190_e26747_d_n11;
        locals.var_xp_dn12 = assign19190_e26747_d_n12;
        locals.var_xp_dn17 = assign19190_e26747_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19200_e26753, assign19200_e26753_d_n0, assign19200_e26753_d_n2, assign19200_e26753_d_n6, assign19200_e26753_d_n7, assign19200_e26753_d_n10, assign19200_e26753_d_n11, assign19200_e26753_d_n12, assign19200_e26753_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19200_e26753;
        locals.var_xmp_dn0 = assign19200_e26753_d_n0;
        locals.var_xmp_dn2 = assign19200_e26753_d_n2;
        locals.var_xmp_dn6 = assign19200_e26753_d_n6;
        locals.var_xmp_dn7 = assign19200_e26753_d_n7;
        locals.var_xmp_dn10 = assign19200_e26753_d_n10;
        locals.var_xmp_dn11 = assign19200_e26753_d_n11;
        locals.var_xmp_dn12 = assign19200_e26753_d_n12;
        locals.var_xmp_dn17 = assign19200_e26753_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19210_e26759,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign19210_e26759;
        locals.var_m0_rv = 0.0;

        let (assign19220_e26765,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19220_e26765;
        locals.var_mm_rv = 0.0;

        let (assign19230_e26771, assign19230_e26771_d_n0, assign19230_e26771_d_n2, assign19230_e26771_d_n6, assign19230_e26771_d_n7, assign19230_e26771_d_n10, assign19230_e26771_d_n11, assign19230_e26771_d_n12, assign19230_e26771_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign19230_e26771;
        locals.var_arg_dn0 = assign19230_e26771_d_n0;
        locals.var_arg_dn2 = assign19230_e26771_d_n2;
        locals.var_arg_dn6 = assign19230_e26771_d_n6;
        locals.var_arg_dn7 = assign19230_e26771_d_n7;
        locals.var_arg_dn10 = assign19230_e26771_d_n10;
        locals.var_arg_dn11 = assign19230_e26771_d_n11;
        locals.var_arg_dn12 = assign19230_e26771_d_n12;
        locals.var_arg_dn17 = assign19230_e26771_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign19240_e26777, assign19240_e26777_d_n0, assign19240_e26777_d_n2, assign19240_e26777_d_n6, assign19240_e26777_d_n7, assign19240_e26777_d_n10, assign19240_e26777_d_n11, assign19240_e26777_d_n12, assign19240_e26777_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19240_e26777;
        locals.var_dnm_dn0 = assign19240_e26777_d_n0;
        locals.var_dnm_dn2 = assign19240_e26777_d_n2;
        locals.var_dnm_dn6 = assign19240_e26777_d_n6;
        locals.var_dnm_dn7 = assign19240_e26777_d_n7;
        locals.var_dnm_dn10 = assign19240_e26777_d_n10;
        locals.var_dnm_dn11 = assign19240_e26777_d_n11;
        locals.var_dnm_dn12 = assign19240_e26777_d_n12;
        locals.var_dnm_dn17 = assign19240_e26777_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19250_e26785, assign19250_e26785_d_n0, assign19250_e26785_d_n2, assign19250_e26785_d_n6, assign19250_e26785_d_n7, assign19250_e26785_d_n10, assign19250_e26785_d_n11, assign19250_e26785_d_n12, assign19250_e26785_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19250_e26783: f64 = (locals.var_xp * locals.var_x2);
        (assign19250_e26783, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19250_e26785;
        locals.var_xp_dn0 = assign19250_e26785_d_n0;
        locals.var_xp_dn2 = assign19250_e26785_d_n2;
        locals.var_xp_dn6 = assign19250_e26785_d_n6;
        locals.var_xp_dn7 = assign19250_e26785_d_n7;
        locals.var_xp_dn10 = assign19250_e26785_d_n10;
        locals.var_xp_dn11 = assign19250_e26785_d_n11;
        locals.var_xp_dn12 = assign19250_e26785_d_n12;
        locals.var_xp_dn17 = assign19250_e26785_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19260_e26793, assign19260_e26793_d_n0, assign19260_e26793_d_n2, assign19260_e26793_d_n6, assign19260_e26793_d_n7, assign19260_e26793_d_n10, assign19260_e26793_d_n11, assign19260_e26793_d_n12, assign19260_e26793_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19260_e26791: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19260_e26791, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19260_e26793;
        locals.var_xmp_dn0 = assign19260_e26793_d_n0;
        locals.var_xmp_dn2 = assign19260_e26793_d_n2;
        locals.var_xmp_dn6 = assign19260_e26793_d_n6;
        locals.var_xmp_dn7 = assign19260_e26793_d_n7;
        locals.var_xmp_dn10 = assign19260_e26793_d_n10;
        locals.var_xmp_dn11 = assign19260_e26793_d_n11;
        locals.var_xmp_dn12 = assign19260_e26793_d_n12;
        locals.var_xmp_dn17 = assign19260_e26793_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19270_e26801, assign19270_e26801_d_n0, assign19270_e26801_d_n2, assign19270_e26801_d_n6, assign19270_e26801_d_n7, assign19270_e26801_d_n10, assign19270_e26801_d_n11, assign19270_e26801_d_n12, assign19270_e26801_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19270_e26799: f64 = (locals.var_xp * locals.var_x2);
        (assign19270_e26799, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19270_e26801;
        locals.var_xp_dn0 = assign19270_e26801_d_n0;
        locals.var_xp_dn2 = assign19270_e26801_d_n2;
        locals.var_xp_dn6 = assign19270_e26801_d_n6;
        locals.var_xp_dn7 = assign19270_e26801_d_n7;
        locals.var_xp_dn10 = assign19270_e26801_d_n10;
        locals.var_xp_dn11 = assign19270_e26801_d_n11;
        locals.var_xp_dn12 = assign19270_e26801_d_n12;
        locals.var_xp_dn17 = assign19270_e26801_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19280_e26809, assign19280_e26809_d_n0, assign19280_e26809_d_n2, assign19280_e26809_d_n6, assign19280_e26809_d_n7, assign19280_e26809_d_n10, assign19280_e26809_d_n11, assign19280_e26809_d_n12, assign19280_e26809_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19280_e26807: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19280_e26807, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19280_e26809;
        locals.var_xmp_dn0 = assign19280_e26809_d_n0;
        locals.var_xmp_dn2 = assign19280_e26809_d_n2;
        locals.var_xmp_dn6 = assign19280_e26809_d_n6;
        locals.var_xmp_dn7 = assign19280_e26809_d_n7;
        locals.var_xmp_dn10 = assign19280_e26809_d_n10;
        locals.var_xmp_dn11 = assign19280_e26809_d_n11;
        locals.var_xmp_dn12 = assign19280_e26809_d_n12;
        locals.var_xmp_dn17 = assign19280_e26809_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19290_e26817, assign19290_e26817_d_n0, assign19290_e26817_d_n2, assign19290_e26817_d_n6, assign19290_e26817_d_n7, assign19290_e26817_d_n10, assign19290_e26817_d_n11, assign19290_e26817_d_n12, assign19290_e26817_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19290_e26815: f64 = (locals.var_xp * locals.var_x2);
        (assign19290_e26815, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19290_e26817;
        locals.var_xp_dn0 = assign19290_e26817_d_n0;
        locals.var_xp_dn2 = assign19290_e26817_d_n2;
        locals.var_xp_dn6 = assign19290_e26817_d_n6;
        locals.var_xp_dn7 = assign19290_e26817_d_n7;
        locals.var_xp_dn10 = assign19290_e26817_d_n10;
        locals.var_xp_dn11 = assign19290_e26817_d_n11;
        locals.var_xp_dn12 = assign19290_e26817_d_n12;
        locals.var_xp_dn17 = assign19290_e26817_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19300_e26825, assign19300_e26825_d_n0, assign19300_e26825_d_n2, assign19300_e26825_d_n6, assign19300_e26825_d_n7, assign19300_e26825_d_n10, assign19300_e26825_d_n11, assign19300_e26825_d_n12, assign19300_e26825_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19300_e26823: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19300_e26823, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19300_e26825;
        locals.var_xmp_dn0 = assign19300_e26825_d_n0;
        locals.var_xmp_dn2 = assign19300_e26825_d_n2;
        locals.var_xmp_dn6 = assign19300_e26825_d_n6;
        locals.var_xmp_dn7 = assign19300_e26825_d_n7;
        locals.var_xmp_dn10 = assign19300_e26825_d_n10;
        locals.var_xmp_dn11 = assign19300_e26825_d_n11;
        locals.var_xmp_dn12 = assign19300_e26825_d_n12;
        locals.var_xmp_dn17 = assign19300_e26825_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19310_e26833, assign19310_e26833_d_n0, assign19310_e26833_d_n2, assign19310_e26833_d_n6, assign19310_e26833_d_n7, assign19310_e26833_d_n10, assign19310_e26833_d_n11, assign19310_e26833_d_n12, assign19310_e26833_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19310_e26831: f64 = (locals.var_xp * locals.var_x2);
        (assign19310_e26831, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign19310_e26833;
        locals.var_xp_dn0 = assign19310_e26833_d_n0;
        locals.var_xp_dn2 = assign19310_e26833_d_n2;
        locals.var_xp_dn6 = assign19310_e26833_d_n6;
        locals.var_xp_dn7 = assign19310_e26833_d_n7;
        locals.var_xp_dn10 = assign19310_e26833_d_n10;
        locals.var_xp_dn11 = assign19310_e26833_d_n11;
        locals.var_xp_dn12 = assign19310_e26833_d_n12;
        locals.var_xp_dn17 = assign19310_e26833_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign19320_e26841, assign19320_e26841_d_n0, assign19320_e26841_d_n2, assign19320_e26841_d_n6, assign19320_e26841_d_n7, assign19320_e26841_d_n10, assign19320_e26841_d_n11, assign19320_e26841_d_n12, assign19320_e26841_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19320_e26839: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign19320_e26839, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign19320_e26841;
        locals.var_xmp_dn0 = assign19320_e26841_d_n0;
        locals.var_xmp_dn2 = assign19320_e26841_d_n2;
        locals.var_xmp_dn6 = assign19320_e26841_d_n6;
        locals.var_xmp_dn7 = assign19320_e26841_d_n7;
        locals.var_xmp_dn10 = assign19320_e26841_d_n10;
        locals.var_xmp_dn11 = assign19320_e26841_d_n11;
        locals.var_xmp_dn12 = assign19320_e26841_d_n12;
        locals.var_xmp_dn17 = assign19320_e26841_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign19330_e26849, assign19330_e26849_d_n0, assign19330_e26849_d_n2, assign19330_e26849_d_n6, assign19330_e26849_d_n7, assign19330_e26849_d_n10, assign19330_e26849_d_n11, assign19330_e26849_d_n12, assign19330_e26849_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19330_e26847: f64 = (locals.var_xp + locals.var_xmp);
        (assign19330_e26847, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign19330_e26849;
        locals.var_arg_dn0 = assign19330_e26849_d_n0;
        locals.var_arg_dn2 = assign19330_e26849_d_n2;
        locals.var_arg_dn6 = assign19330_e26849_d_n6;
        locals.var_arg_dn7 = assign19330_e26849_d_n7;
        locals.var_arg_dn10 = assign19330_e26849_d_n10;
        locals.var_arg_dn11 = assign19330_e26849_d_n11;
        locals.var_arg_dn12 = assign19330_e26849_d_n12;
        locals.var_arg_dn17 = assign19330_e26849_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign19340_e26855, assign19340_e26855_d_n0, assign19340_e26855_d_n2, assign19340_e26855_d_n6, assign19340_e26855_d_n7, assign19340_e26855_d_n10, assign19340_e26855_d_n11, assign19340_e26855_d_n12, assign19340_e26855_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19340_e26855;
        locals.var_dnm_dn0 = assign19340_e26855_d_n0;
        locals.var_dnm_dn2 = assign19340_e26855_d_n2;
        locals.var_dnm_dn6 = assign19340_e26855_d_n6;
        locals.var_dnm_dn7 = assign19340_e26855_d_n7;
        locals.var_dnm_dn10 = assign19340_e26855_d_n10;
        locals.var_dnm_dn11 = assign19340_e26855_d_n11;
        locals.var_dnm_dn12 = assign19340_e26855_d_n12;
        locals.var_dnm_dn17 = assign19340_e26855_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign19350_e26870: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard587 = assign19350_e26870;
        locals.var_guard587_rv = 0.0;

        let assign19360_e26873: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign19360_e26873;
        locals.var_guard588_rv = 0.0;

        let (assign19370_e26883,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19370_e26883;
        locals.var_mm_rv = 0.0;

        let assign19380_e26886: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign19380_e26886;
        locals.var_guard589_rv = 0.0;

        let (assign19390_e26899,) = {
    if (((((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 == 0.0)) && (locals.var_guard589 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19390_e26899;
        locals.var_mm_rv = 0.0;

        let assign19400_e26902: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign19400_e26902;
        locals.var_guard590_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19410_e26918,) = {
    if ((((((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 == 0.0)) && (locals.var_guard589 == 0.0)) && (locals.var_guard590 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19410_e26918;
        locals.var_mm_rv = 0.0;

        let assign19420_e26921: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign19420_e26921;
        locals.var_guard591_rv = 0.0;

        let (assign19430_e26940,) = {
    if (((((((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 == 0.0)) && (locals.var_guard589 == 0.0)) && (locals.var_guard590 == 0.0)) && (locals.var_guard591 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign19430_e26940;
        locals.var_mm_rv = 0.0;

        let (assign19440_e26948,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign19440_e26948;
        locals.var_m0_rv = 0.0;

        let mut assign19450_loop_guard: usize = 0;
        while {
            let assign19450_cond_e26957: f64 = if ((((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign19450_cond_e26957 != 0.0
        } {
            assign19450_loop_guard += 1;
            assert!(assign19450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign19450_body0_e26966, assign19450_body0_e26966_d_n0, assign19450_body0_e26966_d_n2, assign19450_body0_e26966_d_n6, assign19450_body0_e26966_d_n7, assign19450_body0_e26966_d_n10, assign19450_body0_e26966_d_n11, assign19450_body0_e26966_d_n12, assign19450_body0_e26966_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign19450_body0_e26964: f64 = (locals.var_dnm).sqrt();
        (assign19450_body0_e26964, (locals.var_dnm_dn0 / (2.0 * assign19450_body0_e26964)), (locals.var_dnm_dn2 / (2.0 * assign19450_body0_e26964)), (locals.var_dnm_dn6 / (2.0 * assign19450_body0_e26964)), (locals.var_dnm_dn7 / (2.0 * assign19450_body0_e26964)), (locals.var_dnm_dn10 / (2.0 * assign19450_body0_e26964)), (locals.var_dnm_dn11 / (2.0 * assign19450_body0_e26964)), (locals.var_dnm_dn12 / (2.0 * assign19450_body0_e26964)), (locals.var_dnm_dn17 / (2.0 * assign19450_body0_e26964)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign19450_body0_e26966;
            locals.var_dnm_dn0 = assign19450_body0_e26966_d_n0;
            locals.var_dnm_dn2 = assign19450_body0_e26966_d_n2;
            locals.var_dnm_dn6 = assign19450_body0_e26966_d_n6;
            locals.var_dnm_dn7 = assign19450_body0_e26966_d_n7;
            locals.var_dnm_dn10 = assign19450_body0_e26966_d_n10;
            locals.var_dnm_dn11 = assign19450_body0_e26966_d_n11;
            locals.var_dnm_dn12 = assign19450_body0_e26966_d_n12;
            locals.var_dnm_dn17 = assign19450_body0_e26966_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign19450_body1_e26976,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign19450_body1_e26974: f64 = (locals.var_m0 + 1.0);
        (assign19450_body1_e26974,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign19450_body1_e26976;
            locals.var_m0_rv = 0.0;
        }

        let (assign19460_e26991, assign19460_e26991_d_n0, assign19460_e26991_d_n2, assign19460_e26991_d_n6, assign19460_e26991_d_n7, assign19460_e26991_d_n10, assign19460_e26991_d_n11, assign19460_e26991_d_n12, assign19460_e26991_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign19460_e26987: f64 = (2.0 * 4.0);
        let assign19460_e26988: f64 = (1.0 / assign19460_e26987);
        let assign19460_e26989: f64 = (locals.var_dnm).powf(assign19460_e26988);
        (assign19460_e26989, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn0)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn2)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn6)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn7)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn10)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn11)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn12)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign19460_e26988) as f64).is_finite() && ((assign19460_e26988) as f64).fract() == 0.0 { if assign19460_e26988 == 0.0 { 0.0 } else { (assign19460_e26988 * ((locals.var_dnm).powf(assign19460_e26988 - 1.0) * locals.var_dnm_dn17)) } } else { (assign19460_e26989 * (assign19460_e26988 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19460_e26991;
        locals.var_dnm_dn0 = assign19460_e26991_d_n0;
        locals.var_dnm_dn2 = assign19460_e26991_d_n2;
        locals.var_dnm_dn6 = assign19460_e26991_d_n6;
        locals.var_dnm_dn7 = assign19460_e26991_d_n7;
        locals.var_dnm_dn10 = assign19460_e26991_d_n10;
        locals.var_dnm_dn11 = assign19460_e26991_d_n11;
        locals.var_dnm_dn12 = assign19460_e26991_d_n12;
        locals.var_dnm_dn17 = assign19460_e26991_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19470_e26999, assign19470_e26999_d_n0, assign19470_e26999_d_n2, assign19470_e26999_d_n6, assign19470_e26999_d_n7, assign19470_e26999_d_n10, assign19470_e26999_d_n11, assign19470_e26999_d_n12, assign19470_e26999_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19470_e26997: f64 = (1.0 / locals.var_dnm);
        (assign19470_e26997, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign19470_e26999;
        locals.var_dnm_dn0 = assign19470_e26999_d_n0;
        locals.var_dnm_dn2 = assign19470_e26999_d_n2;
        locals.var_dnm_dn6 = assign19470_e26999_d_n6;
        locals.var_dnm_dn7 = assign19470_e26999_d_n7;
        locals.var_dnm_dn10 = assign19470_e26999_d_n10;
        locals.var_dnm_dn11 = assign19470_e26999_d_n11;
        locals.var_dnm_dn12 = assign19470_e26999_d_n12;
        locals.var_dnm_dn17 = assign19470_e26999_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign19480_e27009, assign19480_e27009_d_n0, assign19480_e27009_d_n2, assign19480_e27009_d_n6, assign19480_e27009_d_n7, assign19480_e27009_d_n10, assign19480_e27009_d_n11, assign19480_e27009_d_n12, assign19480_e27009_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19480_e27005: f64 = locals.var_tx__blk580;
        let assign19480_e27007: f64 = (assign19480_e27005 * locals.var_dnm);
        (assign19480_e27007, ((locals.var_tx__blk580_dn0 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn0)), ((locals.var_tx__blk580_dn2 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn2)), ((locals.var_tx__blk580_dn6 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn6)), ((locals.var_tx__blk580_dn7 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn7)), ((locals.var_tx__blk580_dn10 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn10)), ((locals.var_tx__blk580_dn11 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn11)), ((locals.var_tx__blk580_dn12 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn12)), ((locals.var_tx__blk580_dn17 * locals.var_dnm) + (assign19480_e27005 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_ty__blk581, locals.var_ty__blk581_dn0, locals.var_ty__blk581_dn2, locals.var_ty__blk581_dn6, locals.var_ty__blk581_dn7, locals.var_ty__blk581_dn10, locals.var_ty__blk581_dn11, locals.var_ty__blk581_dn12, locals.var_ty__blk581_dn17,)
    }
};
        locals.var_ty__blk581 = assign19480_e27009;
        locals.var_ty__blk581_dn0 = assign19480_e27009_d_n0;
        locals.var_ty__blk581_dn2 = assign19480_e27009_d_n2;
        locals.var_ty__blk581_dn6 = assign19480_e27009_d_n6;
        locals.var_ty__blk581_dn7 = assign19480_e27009_d_n7;
        locals.var_ty__blk581_dn10 = assign19480_e27009_d_n10;
        locals.var_ty__blk581_dn11 = assign19480_e27009_d_n11;
        locals.var_ty__blk581_dn12 = assign19480_e27009_d_n12;
        locals.var_ty__blk581_dn17 = assign19480_e27009_d_n17;
        locals.var_ty__blk581_rv = 0.0;

        let (assign19490_e27021, assign19490_e27021_d_n10,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19490_e27015: f64 = (2.0 * locals.var_uc_wsti);
        let assign19490_e27017: f64 = (assign19490_e27015 * p.p9);
        let assign19490_e27019: f64 = (assign19490_e27017 * locals.var_beta_inv);
        (assign19490_e27019, (assign19490_e27017 * locals.var_beta_inv_dn10),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn10,)
    }
};
        locals.var_costi7 = assign19490_e27021;
        locals.var_costi7_dn10 = assign19490_e27021_d_n10;
        locals.var_costi7_rv = 0.0;

        let (assign19500_e27035, assign19500_e27035_d_n0, assign19500_e27035_d_n2, assign19500_e27035_d_n6, assign19500_e27035_d_n7, assign19500_e27035_d_n10, assign19500_e27035_d_n11, assign19500_e27035_d_n12, assign19500_e27035_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19500_e27027: f64 = (locals.var_costi7 * locals.var_mu);
        let assign19500_e27029: f64 = (assign19500_e27027 * locals.var_qn0sti);
        let assign19500_e27031: f64 = (assign19500_e27029 * locals.var_ty__blk581);
        let assign19500_e27033: f64 = (assign19500_e27031 / locals.var_lch);
        (assign19500_e27033, ((((((((locals.var_costi7 * locals.var_mu_dn0) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn0)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn0)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn2) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn2)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn2)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn6) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn6)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn6)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn7) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn7)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn7)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn10)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn10)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn11) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn11)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn11)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn12) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn12)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn12)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((((((locals.var_costi7 * locals.var_mu_dn17) * locals.var_qn0sti) + (assign19500_e27027 * locals.var_qn0sti_dn17)) * locals.var_ty__blk581) + (assign19500_e27029 * locals.var_ty__blk581_dn17)) * locals.var_lch) - (assign19500_e27031 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12, locals.var_idssti_dn17,)
    }
};
        locals.var_idssti = assign19500_e27035;
        locals.var_idssti_dn0 = assign19500_e27035_d_n0;
        locals.var_idssti_dn2 = assign19500_e27035_d_n2;
        locals.var_idssti_dn6 = assign19500_e27035_d_n6;
        locals.var_idssti_dn7 = assign19500_e27035_d_n7;
        locals.var_idssti_dn10 = assign19500_e27035_d_n10;
        locals.var_idssti_dn11 = assign19500_e27035_d_n11;
        locals.var_idssti_dn12 = assign19500_e27035_d_n12;
        locals.var_idssti_dn17 = assign19500_e27035_d_n17;
        locals.var_idssti_rv = 0.0;

        let (assign19510_e27043, assign19510_e27043_d_n0, assign19510_e27043_d_n2, assign19510_e27043_d_n6, assign19510_e27043_d_n7, assign19510_e27043_d_n10, assign19510_e27043_d_n11, assign19510_e27043_d_n12, assign19510_e27043_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard582 != 0.0)) {
        let assign19510_e27041: f64 = (locals.var_ids + locals.var_idssti);
        (assign19510_e27041, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn7 + locals.var_idssti_dn7), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn11 + locals.var_idssti_dn11), (locals.var_ids_dn12 + locals.var_idssti_dn12), (locals.var_ids_dn17 + locals.var_idssti_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign19510_e27043;
        locals.var_ids_dn0 = assign19510_e27043_d_n0;
        locals.var_ids_dn2 = assign19510_e27043_d_n2;
        locals.var_ids_dn6 = assign19510_e27043_d_n6;
        locals.var_ids_dn7 = assign19510_e27043_d_n7;
        locals.var_ids_dn10 = assign19510_e27043_d_n10;
        locals.var_ids_dn11 = assign19510_e27043_d_n11;
        locals.var_ids_dn12 = assign19510_e27043_d_n12;
        locals.var_ids_dn17 = assign19510_e27043_d_n17;
        locals.var_ids_rv = 0.0;

        let assign19520_e27050: f64 = if ((p.p30 != 0.0) && (p.p32 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard592 = assign19520_e27050;
        locals.var_guard592_rv = 0.0;

        let (assign19530_e27058, assign19530_e27058_d_n0, assign19530_e27058_d_n2, assign19530_e27058_d_n6, assign19530_e27058_d_n7, assign19530_e27058_d_n10, assign19530_e27058_d_n11, assign19530_e27058_d_n12, assign19530_e27058_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19530_e27056: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign19530_e27056, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn7 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn7)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn11 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn11)), ((locals.var_vgvt_dn12 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn12)), ((locals.var_vgvt_dn17 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn17)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19530_e27058;
        locals.var_kusai00_dn0 = assign19530_e27058_d_n0;
        locals.var_kusai00_dn2 = assign19530_e27058_d_n2;
        locals.var_kusai00_dn6 = assign19530_e27058_d_n6;
        locals.var_kusai00_dn7 = assign19530_e27058_d_n7;
        locals.var_kusai00_dn10 = assign19530_e27058_d_n10;
        locals.var_kusai00_dn11 = assign19530_e27058_d_n11;
        locals.var_kusai00_dn12 = assign19530_e27058_d_n12;
        locals.var_kusai00_dn17 = assign19530_e27058_d_n17;
        locals.var_kusai00_rv = 0.0;

        let (assign19540_e27070, assign19540_e27070_d_n0, assign19540_e27070_d_n2, assign19540_e27070_d_n6, assign19540_e27070_d_n7, assign19540_e27070_d_n10, assign19540_e27070_d_n11, assign19540_e27070_d_n12, assign19540_e27070_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19540_e27064: f64 = (2.0 * locals.var_beta_inv);
        let assign19540_e27066: f64 = (assign19540_e27064 * locals.var_c_fox_inv);
        let assign19540_e27068: f64 = (assign19540_e27066 * locals.var_idd);
        (assign19540_e27068, (((assign19540_e27064 * locals.var_c_fox_inv_dn0) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn0)), (((assign19540_e27064 * locals.var_c_fox_inv_dn2) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn2)), (((assign19540_e27064 * locals.var_c_fox_inv_dn6) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn6)), (((assign19540_e27064 * locals.var_c_fox_inv_dn7) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn7)), (((((2.0 * locals.var_beta_inv_dn10) * locals.var_c_fox_inv) + (assign19540_e27064 * locals.var_c_fox_inv_dn10)) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn10)), (((assign19540_e27064 * locals.var_c_fox_inv_dn11) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn11)), (((assign19540_e27064 * locals.var_c_fox_inv_dn12) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn12)), (((assign19540_e27064 * locals.var_c_fox_inv_dn17) * locals.var_idd) + (assign19540_e27066 * locals.var_idd_dn17)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn6, locals.var_kusaidd_dn7, locals.var_kusaidd_dn10, locals.var_kusaidd_dn11, locals.var_kusaidd_dn12, locals.var_kusaidd_dn17,)
    }
};
        locals.var_kusaidd = assign19540_e27070;
        locals.var_kusaidd_dn0 = assign19540_e27070_d_n0;
        locals.var_kusaidd_dn2 = assign19540_e27070_d_n2;
        locals.var_kusaidd_dn6 = assign19540_e27070_d_n6;
        locals.var_kusaidd_dn7 = assign19540_e27070_d_n7;
        locals.var_kusaidd_dn10 = assign19540_e27070_d_n10;
        locals.var_kusaidd_dn11 = assign19540_e27070_d_n11;
        locals.var_kusaidd_dn12 = assign19540_e27070_d_n12;
        locals.var_kusaidd_dn17 = assign19540_e27070_d_n17;
        locals.var_kusaidd_rv = 0.0;

        let (assign19550_e27078, assign19550_e27078_d_n0, assign19550_e27078_d_n2, assign19550_e27078_d_n6, assign19550_e27078_d_n7, assign19550_e27078_d_n10, assign19550_e27078_d_n11, assign19550_e27078_d_n12, assign19550_e27078_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19550_e27076: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign19550_e27076, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn7 - locals.var_kusaidd_dn7), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn11 - locals.var_kusaidd_dn11), (locals.var_kusai00_dn12 - locals.var_kusaidd_dn12), (locals.var_kusai00_dn17 - locals.var_kusaidd_dn17),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19550_e27078;
        locals.var_kusail_dn0 = assign19550_e27078_d_n0;
        locals.var_kusail_dn2 = assign19550_e27078_d_n2;
        locals.var_kusail_dn6 = assign19550_e27078_d_n6;
        locals.var_kusail_dn7 = assign19550_e27078_d_n7;
        locals.var_kusail_dn10 = assign19550_e27078_d_n10;
        locals.var_kusail_dn11 = assign19550_e27078_d_n11;
        locals.var_kusail_dn12 = assign19550_e27078_d_n12;
        locals.var_kusail_dn17 = assign19550_e27078_d_n17;
        locals.var_kusail_rv = 0.0;

        let (assign19560_e27093, assign19560_e27093_d_n0, assign19560_e27093_d_n2, assign19560_e27093_d_n6, assign19560_e27093_d_n7, assign19560_e27093_d_n10, assign19560_e27093_d_n11, assign19560_e27093_d_n12, assign19560_e27093_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19560_e27084: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign19560_e27087: f64 = (4.0 * 0.001);
        let assign19560_e27089: f64 = (assign19560_e27087 * 0.001);
        let assign19560_e27090: f64 = (assign19560_e27084 + assign19560_e27089);
        let assign19560_e27091: f64 = (assign19560_e27090).sqrt();
        (assign19560_e27091, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign19560_e27091)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign19560_e27091)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign19560_e27091)), (((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)) / (2.0 * assign19560_e27091)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign19560_e27091)), (((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)) / (2.0 * assign19560_e27091)), (((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)) / (2.0 * assign19560_e27091)), (((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)) / (2.0 * assign19560_e27091)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19560_e27093;
        locals.var_tmf1_dn0 = assign19560_e27093_d_n0;
        locals.var_tmf1_dn2 = assign19560_e27093_d_n2;
        locals.var_tmf1_dn6 = assign19560_e27093_d_n6;
        locals.var_tmf1_dn7 = assign19560_e27093_d_n7;
        locals.var_tmf1_dn10 = assign19560_e27093_d_n10;
        locals.var_tmf1_dn11 = assign19560_e27093_d_n11;
        locals.var_tmf1_dn12 = assign19560_e27093_d_n12;
        locals.var_tmf1_dn17 = assign19560_e27093_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19570_e27107, assign19570_e27107_d_n0, assign19570_e27107_d_n2, assign19570_e27107_d_n6, assign19570_e27107_d_n7, assign19570_e27107_d_n10, assign19570_e27107_d_n11, assign19570_e27107_d_n12, assign19570_e27107_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19570_e27100: f64 = (locals.var_kusai00 + locals.var_tmf1);
        let assign19570_e27101: f64 = (0.5 * assign19570_e27100);
        let assign19570_e27104: f64 = (1e-10 * 0.001);
        let assign19570_e27105: f64 = (assign19570_e27101 + assign19570_e27104);
        (assign19570_e27105, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_kusai00_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_kusai00_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_kusai00_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_kusai00_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19570_e27107;
        locals.var_kusai00_dn0 = assign19570_e27107_d_n0;
        locals.var_kusai00_dn2 = assign19570_e27107_d_n2;
        locals.var_kusai00_dn6 = assign19570_e27107_d_n6;
        locals.var_kusai00_dn7 = assign19570_e27107_d_n7;
        locals.var_kusai00_dn10 = assign19570_e27107_d_n10;
        locals.var_kusai00_dn11 = assign19570_e27107_d_n11;
        locals.var_kusai00_dn12 = assign19570_e27107_d_n12;
        locals.var_kusai00_dn17 = assign19570_e27107_d_n17;
        locals.var_kusai00_rv = 0.0;

        let assign19580_e27110: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign19580_e27110;
        locals.var_guard593_rv = 0.0;

        let (assign19590_e27118, assign19590_e27118_d_n0, assign19590_e27118_d_n2, assign19590_e27118_d_n6, assign19590_e27118_d_n7, assign19590_e27118_d_n10, assign19590_e27118_d_n11, assign19590_e27118_d_n12, assign19590_e27118_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12, locals.var_kusai00_dn17,)
    }
};
        locals.var_kusai00 = assign19590_e27118;
        locals.var_kusai00_dn0 = assign19590_e27118_d_n0;
        locals.var_kusai00_dn2 = assign19590_e27118_d_n2;
        locals.var_kusai00_dn6 = assign19590_e27118_d_n6;
        locals.var_kusai00_dn7 = assign19590_e27118_d_n7;
        locals.var_kusai00_dn10 = assign19590_e27118_d_n10;
        locals.var_kusai00_dn11 = assign19590_e27118_d_n11;
        locals.var_kusai00_dn12 = assign19590_e27118_d_n12;
        locals.var_kusai00_dn17 = assign19590_e27118_d_n17;
        locals.var_kusai00_rv = 0.0;

        let (assign19600_e27133, assign19600_e27133_d_n0, assign19600_e27133_d_n2, assign19600_e27133_d_n6, assign19600_e27133_d_n7, assign19600_e27133_d_n10, assign19600_e27133_d_n11, assign19600_e27133_d_n12, assign19600_e27133_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19600_e27124: f64 = (locals.var_kusail * locals.var_kusail);
        let assign19600_e27127: f64 = (4.0 * 0.001);
        let assign19600_e27129: f64 = (assign19600_e27127 * 0.001);
        let assign19600_e27130: f64 = (assign19600_e27124 + assign19600_e27129);
        let assign19600_e27131: f64 = (assign19600_e27130).sqrt();
        (assign19600_e27131, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign19600_e27131)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign19600_e27131)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign19600_e27131)), (((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)) / (2.0 * assign19600_e27131)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign19600_e27131)), (((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)) / (2.0 * assign19600_e27131)), (((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)) / (2.0 * assign19600_e27131)), (((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)) / (2.0 * assign19600_e27131)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign19600_e27133;
        locals.var_tmf1_dn0 = assign19600_e27133_d_n0;
        locals.var_tmf1_dn2 = assign19600_e27133_d_n2;
        locals.var_tmf1_dn6 = assign19600_e27133_d_n6;
        locals.var_tmf1_dn7 = assign19600_e27133_d_n7;
        locals.var_tmf1_dn10 = assign19600_e27133_d_n10;
        locals.var_tmf1_dn11 = assign19600_e27133_d_n11;
        locals.var_tmf1_dn12 = assign19600_e27133_d_n12;
        locals.var_tmf1_dn17 = assign19600_e27133_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign19610_e27147, assign19610_e27147_d_n0, assign19610_e27147_d_n2, assign19610_e27147_d_n6, assign19610_e27147_d_n7, assign19610_e27147_d_n10, assign19610_e27147_d_n11, assign19610_e27147_d_n12, assign19610_e27147_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19610_e27140: f64 = (locals.var_kusail + locals.var_tmf1);
        let assign19610_e27141: f64 = (0.5 * assign19610_e27140);
        let assign19610_e27144: f64 = (1e-10 * 0.001);
        let assign19610_e27145: f64 = (assign19610_e27141 + assign19610_e27144);
        (assign19610_e27145, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_kusail_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_kusail_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_kusail_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_kusail_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19610_e27147;
        locals.var_kusail_dn0 = assign19610_e27147_d_n0;
        locals.var_kusail_dn2 = assign19610_e27147_d_n2;
        locals.var_kusail_dn6 = assign19610_e27147_d_n6;
        locals.var_kusail_dn7 = assign19610_e27147_d_n7;
        locals.var_kusail_dn10 = assign19610_e27147_d_n10;
        locals.var_kusail_dn11 = assign19610_e27147_d_n11;
        locals.var_kusail_dn12 = assign19610_e27147_d_n12;
        locals.var_kusail_dn17 = assign19610_e27147_d_n17;
        locals.var_kusail_rv = 0.0;

        let assign19620_e27150: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign19620_e27150;
        locals.var_guard594_rv = 0.0;

        let (assign19630_e27158, assign19630_e27158_d_n0, assign19630_e27158_d_n2, assign19630_e27158_d_n6, assign19630_e27158_d_n7, assign19630_e27158_d_n10, assign19630_e27158_d_n11, assign19630_e27158_d_n12, assign19630_e27158_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard594 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12, locals.var_kusail_dn17,)
    }
};
        locals.var_kusail = assign19630_e27158;
        locals.var_kusail_dn0 = assign19630_e27158_d_n0;
        locals.var_kusail_dn2 = assign19630_e27158_d_n2;
        locals.var_kusail_dn6 = assign19630_e27158_d_n6;
        locals.var_kusail_dn7 = assign19630_e27158_d_n7;
        locals.var_kusail_dn10 = assign19630_e27158_d_n10;
        locals.var_kusail_dn11 = assign19630_e27158_d_n11;
        locals.var_kusail_dn12 = assign19630_e27158_d_n12;
        locals.var_kusail_dn17 = assign19630_e27158_d_n17;
        locals.var_kusail_rv = 0.0;

        let (assign19640_e27166, assign19640_e27166_d_n0, assign19640_e27166_d_n2, assign19640_e27166_d_n6, assign19640_e27166_d_n7, assign19640_e27166_d_n10, assign19640_e27166_d_n11, assign19640_e27166_d_n12, assign19640_e27166_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign19640_e27164: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign19640_e27164, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn7 - locals.var_kusail_dn7), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn11 - locals.var_kusail_dn11), (locals.var_kusai00_dn12 - locals.var_kusail_dn12), (locals.var_kusai00_dn17 - locals.var_kusail_dn17),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn6, locals.var_kusai00l_dn7, locals.var_kusai00l_dn10, locals.var_kusai00l_dn11, locals.var_kusai00l_dn12, locals.var_kusai00l_dn17,)
    }
};
        locals.var_kusai00l = assign19640_e27166;
        locals.var_kusai00l_dn0 = assign19640_e27166_d_n0;
        locals.var_kusai00l_dn2 = assign19640_e27166_d_n2;
        locals.var_kusai00l_dn6 = assign19640_e27166_d_n6;
        locals.var_kusai00l_dn7 = assign19640_e27166_d_n7;
        locals.var_kusai00l_dn10 = assign19640_e27166_d_n10;
        locals.var_kusai00l_dn11 = assign19640_e27166_d_n11;
        locals.var_kusai00l_dn12 = assign19640_e27166_d_n12;
        locals.var_kusai00l_dn17 = assign19640_e27166_d_n17;
        locals.var_kusai00l_rv = 0.0;

        let assign19650_e27170: f64 = (10.0 * 2.220446049250313e-16);
        let assign19650_e27175: f64 = (10.0 * 2.220446049250313e-16);
        let assign19650_e27177: f64 = if ((locals.var_qn0 < assign19650_e27170) || (locals.var_kusai00l < assign19650_e27175)) { 1.0 } else { 0.0 };
        locals.var_guard595 = assign19650_e27177;
        locals.var_guard595_rv = 0.0;

        let (assign19660_e27185,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard595 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign19660_e27185;
        locals.var_flg_ign_rv = 0.0;

        let (assign19670_e27194,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard595 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign19670_e27194;
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

        let assign19700_e27203: f64 = if ((p.p281 > 0.0) && (p.p285 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard596 = assign19700_e27203;
        locals.var_guard596_rv = 0.0;

        let (assign19710_e27207,) = {
    if (locals.var_guard596 != 0.0) {
        (locals.var_lgleff,)
    } else {
        (locals.var_leff__blk603,)
    }
};
        locals.var_leff__blk603 = assign19710_e27207;
        locals.var_leff__blk603_rv = 0.0;

        let (assign19720_e27211,) = {
    if (locals.var_guard596 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_t_soi__blk607,)
    }
};
        locals.var_t_soi__blk607 = assign19720_e27211;
        locals.var_t_soi__blk607_rv = 0.0;

        let (assign19730_e27223, assign19730_e27223_d_n0, assign19730_e27223_d_n2, assign19730_e27223_d_n6, assign19730_e27223_d_n7, assign19730_e27223_d_n10, assign19730_e27223_d_n11, assign19730_e27223_d_n12, assign19730_e27223_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19730_e27215: f64 = (locals.var_vgs - locals.var_vfb);
        let assign19730_e27217: f64 = (assign19730_e27215 + locals.var_dvth);
        let assign19730_e27219: f64 = (assign19730_e27217 - locals.var_dppg);
        let assign19730_e27221: f64 = (assign19730_e27219 - p.p286);
        (assign19730_e27221, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), (locals.var_dvth_dn10 - locals.var_dppg_dn10), ((locals.var_vgs_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), (locals.var_dvth_dn12 - locals.var_dppg_dn12), (locals.var_dvth_dn17 - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgp__blk608, locals.var_vgp__blk608_dn0, locals.var_vgp__blk608_dn2, locals.var_vgp__blk608_dn6, locals.var_vgp__blk608_dn7, locals.var_vgp__blk608_dn10, locals.var_vgp__blk608_dn11, locals.var_vgp__blk608_dn12, locals.var_vgp__blk608_dn17,)
    }
};
        locals.var_vgp__blk608 = assign19730_e27223;
        locals.var_vgp__blk608_dn0 = assign19730_e27223_d_n0;
        locals.var_vgp__blk608_dn2 = assign19730_e27223_d_n2;
        locals.var_vgp__blk608_dn6 = assign19730_e27223_d_n6;
        locals.var_vgp__blk608_dn7 = assign19730_e27223_d_n7;
        locals.var_vgp__blk608_dn10 = assign19730_e27223_d_n10;
        locals.var_vgp__blk608_dn11 = assign19730_e27223_d_n11;
        locals.var_vgp__blk608_dn12 = assign19730_e27223_d_n12;
        locals.var_vgp__blk608_dn17 = assign19730_e27223_d_n17;
        locals.var_vgp__blk608_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19740_e27229, assign19740_e27229_d_n0, assign19740_e27229_d_n2, assign19740_e27229_d_n6, assign19740_e27229_d_n7, assign19740_e27229_d_n10, assign19740_e27229_d_n11, assign19740_e27229_d_n12, assign19740_e27229_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19740_e27227: f64 = (locals.var_vth + p.p286);
        (assign19740_e27227, locals.var_vth_dn0, locals.var_vth_dn2, locals.var_vth_dn6, locals.var_vth_dn7, locals.var_vth_dn10, locals.var_vth_dn11, locals.var_vth_dn12, locals.var_vth_dn17,)
    } else {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn12, locals.var_wk_vth_dn17,)
    }
};
        locals.var_wk_vth = assign19740_e27229;
        locals.var_wk_vth_dn0 = assign19740_e27229_d_n0;
        locals.var_wk_vth_dn2 = assign19740_e27229_d_n2;
        locals.var_wk_vth_dn6 = assign19740_e27229_d_n6;
        locals.var_wk_vth_dn7 = assign19740_e27229_d_n7;
        locals.var_wk_vth_dn10 = assign19740_e27229_d_n10;
        locals.var_wk_vth_dn11 = assign19740_e27229_d_n11;
        locals.var_wk_vth_dn12 = assign19740_e27229_d_n12;
        locals.var_wk_vth_dn17 = assign19740_e27229_d_n17;
        locals.var_wk_vth_rv = 0.0;

        let (assign19750_e27233,) = {
    if (locals.var_guard596 != 0.0) {
        (p.p285,)
    } else {
        (locals.var_wk_mu,)
    }
};
        locals.var_wk_mu = assign19750_e27233;
        locals.var_wk_mu_rv = 0.0;

        let (assign19760_e27237,) = {
    if (locals.var_guard596 != 0.0) {
        (p.p283,)
    } else {
        (locals.var_wk_xj,)
    }
};
        locals.var_wk_xj = assign19760_e27237;
        locals.var_wk_xj_rv = 0.0;

        let (assign19770_e27241,) = {
    if (locals.var_guard596 != 0.0) {
        (locals.var_mks_njunc,)
    } else {
        (locals.var_uc_wk_njunc,)
    }
};
        locals.var_uc_wk_njunc = assign19770_e27241;
        locals.var_uc_wk_njunc_rv = 0.0;

        let (assign19780_e27254, assign19780_e27254_d_n0, assign19780_e27254_d_n2, assign19780_e27254_d_n6, assign19780_e27254_d_n7, assign19780_e27254_d_n10, assign19780_e27254_d_n11, assign19780_e27254_d_n12, assign19780_e27254_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19780_e27246: f64 = (locals.var_uc_wk_njunc / locals.var_nin);
        let assign19780_e27248: f64 = (assign19780_e27246 * locals.var_nsub);
        let assign19780_e27250: f64 = (assign19780_e27248 / locals.var_nin);
        let assign19780_e27251: f64 = (assign19780_e27250).ln();
        let assign19780_e27252: f64 = (locals.var_beta_inv * assign19780_e27251);
        (assign19780_e27252, (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn0)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn2)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn6)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn7)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250)), ((locals.var_beta_inv_dn10 * assign19780_e27251) + (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn10)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250))), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn11)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn12)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250)), (locals.var_beta_inv * (((((((-((locals.var_uc_wk_njunc * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign19780_e27246 * locals.var_nsub_dn17)) * locals.var_nin) - (assign19780_e27248 * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)) / assign19780_e27250)),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn12, locals.var_vbipn_dn17,)
    }
};
        locals.var_vbipn = assign19780_e27254;
        locals.var_vbipn_dn0 = assign19780_e27254_d_n0;
        locals.var_vbipn_dn2 = assign19780_e27254_d_n2;
        locals.var_vbipn_dn6 = assign19780_e27254_d_n6;
        locals.var_vbipn_dn7 = assign19780_e27254_d_n7;
        locals.var_vbipn_dn10 = assign19780_e27254_d_n10;
        locals.var_vbipn_dn11 = assign19780_e27254_d_n11;
        locals.var_vbipn_dn12 = assign19780_e27254_d_n12;
        locals.var_vbipn_dn17 = assign19780_e27254_d_n17;
        locals.var_vbipn_rv = 0.0;

        let (assign19790_e27263, assign19790_e27263_d_n0, assign19790_e27263_d_n2, assign19790_e27263_d_n6, assign19790_e27263_d_n7, assign19790_e27263_d_n10, assign19790_e27263_d_n11, assign19790_e27263_d_n12, assign19790_e27263_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let (assign19790_e27261, assign19790_e27261_d_n0, assign19790_e27261_d_n2, assign19790_e27261_d_n6, assign19790_e27261_d_n7, assign19790_e27261_d_n10, assign19790_e27261_d_n11, assign19790_e27261_d_n12, assign19790_e27261_d_n17,) = {
            if (p.p43 == 1.0) {
                (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
            } else {
                (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
            }
        };
        (assign19790_e27261, assign19790_e27261_d_n0, assign19790_e27261_d_n2, assign19790_e27261_d_n6, assign19790_e27261_d_n7, assign19790_e27261_d_n10, assign19790_e27261_d_n11, assign19790_e27261_d_n12, assign19790_e27261_d_n17,)
    } else {
        (locals.var_vbs__blk599, locals.var_vbs__blk599_dn0, locals.var_vbs__blk599_dn2, locals.var_vbs__blk599_dn6, locals.var_vbs__blk599_dn7, locals.var_vbs__blk599_dn10, locals.var_vbs__blk599_dn11, locals.var_vbs__blk599_dn12, locals.var_vbs__blk599_dn17,)
    }
};
        locals.var_vbs__blk599 = assign19790_e27263;
        locals.var_vbs__blk599_dn0 = assign19790_e27263_d_n0;
        locals.var_vbs__blk599_dn2 = assign19790_e27263_d_n2;
        locals.var_vbs__blk599_dn6 = assign19790_e27263_d_n6;
        locals.var_vbs__blk599_dn7 = assign19790_e27263_d_n7;
        locals.var_vbs__blk599_dn10 = assign19790_e27263_d_n10;
        locals.var_vbs__blk599_dn11 = assign19790_e27263_d_n11;
        locals.var_vbs__blk599_dn12 = assign19790_e27263_d_n12;
        locals.var_vbs__blk599_dn17 = assign19790_e27263_d_n17;
        locals.var_vbs__blk599_rv = 0.0;

        let (assign19800_e27284, assign19800_e27284_d_n0, assign19800_e27284_d_n2, assign19800_e27284_d_n6, assign19800_e27284_d_n7, assign19800_e27284_d_n10, assign19800_e27284_d_n11, assign19800_e27284_d_n12, assign19800_e27284_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19800_e27267: f64 = (2.0 * 1.6021918e-19);
        let assign19800_e27270: f64 = (locals.var_vbipn - locals.var_vbs__blk599);
        let assign19800_e27271: f64 = (assign19800_e27267 * assign19800_e27270);
        let assign19800_e27273: f64 = (assign19800_e27271 / 1.034943e-10);
        let assign19800_e27275: f64 = (assign19800_e27273 * locals.var_nsub);
        let assign19800_e27277: f64 = (assign19800_e27275 * locals.var_uc_wk_njunc);
        let assign19800_e27280: f64 = (locals.var_nsub + locals.var_uc_wk_njunc);
        let assign19800_e27281: f64 = (assign19800_e27277 / assign19800_e27280);
        let assign19800_e27282: f64 = (assign19800_e27281).sqrt();
        (assign19800_e27282, (((((((((assign19800_e27267 * (locals.var_vbipn_dn0 - locals.var_vbs__blk599_dn0)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn0)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn0)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)), (((((((((assign19800_e27267 * (locals.var_vbipn_dn2 - locals.var_vbs__blk599_dn2)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn2)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn2)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)), (((((((((assign19800_e27267 * (locals.var_vbipn_dn6 - locals.var_vbs__blk599_dn6)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn6)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn6)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)), (((((((((assign19800_e27267 * (locals.var_vbipn_dn7 - locals.var_vbs__blk599_dn7)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn7)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn7)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)), (((((((((assign19800_e27267 * (locals.var_vbipn_dn10 - locals.var_vbs__blk599_dn10)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn10)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn10)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)), (((((((((assign19800_e27267 * (locals.var_vbipn_dn11 - locals.var_vbs__blk599_dn11)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn11)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn11)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)), (((((((((assign19800_e27267 * (locals.var_vbipn_dn12 - locals.var_vbs__blk599_dn12)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn12)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn12)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)), (((((((((assign19800_e27267 * (locals.var_vbipn_dn17 - locals.var_vbs__blk599_dn17)) / 1.034943e-10) * locals.var_nsub) + (assign19800_e27273 * locals.var_nsub_dn17)) * locals.var_uc_wk_njunc) * assign19800_e27280) - (assign19800_e27277 * locals.var_nsub_dn17)) / (assign19800_e27280 * assign19800_e27280)) / (2.0 * assign19800_e27282)),)
    } else {
        (locals.var_ec__blk604, locals.var_ec__blk604_dn0, locals.var_ec__blk604_dn2, locals.var_ec__blk604_dn6, locals.var_ec__blk604_dn7, locals.var_ec__blk604_dn10, locals.var_ec__blk604_dn11, locals.var_ec__blk604_dn12, locals.var_ec__blk604_dn17,)
    }
};
        locals.var_ec__blk604 = assign19800_e27284;
        locals.var_ec__blk604_dn0 = assign19800_e27284_d_n0;
        locals.var_ec__blk604_dn2 = assign19800_e27284_d_n2;
        locals.var_ec__blk604_dn6 = assign19800_e27284_d_n6;
        locals.var_ec__blk604_dn7 = assign19800_e27284_d_n7;
        locals.var_ec__blk604_dn10 = assign19800_e27284_d_n10;
        locals.var_ec__blk604_dn11 = assign19800_e27284_d_n11;
        locals.var_ec__blk604_dn12 = assign19800_e27284_d_n12;
        locals.var_ec__blk604_dn17 = assign19800_e27284_d_n17;
        locals.var_ec__blk604_rv = 0.0;

        let (assign19810_e27290, assign19810_e27290_d_n0, assign19810_e27290_d_n2, assign19810_e27290_d_n6, assign19810_e27290_d_n7, assign19810_e27290_d_n10, assign19810_e27290_d_n11, assign19810_e27290_d_n12, assign19810_e27290_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19810_e27288: f64 = (locals.var_ec__blk604 * locals.var_leff__blk603);
        (assign19810_e27288, (locals.var_ec__blk604_dn0 * locals.var_leff__blk603), (locals.var_ec__blk604_dn2 * locals.var_leff__blk603), (locals.var_ec__blk604_dn6 * locals.var_leff__blk603), (locals.var_ec__blk604_dn7 * locals.var_leff__blk603), (locals.var_ec__blk604_dn10 * locals.var_leff__blk603), (locals.var_ec__blk604_dn11 * locals.var_leff__blk603), (locals.var_ec__blk604_dn12 * locals.var_leff__blk603), (locals.var_ec__blk604_dn17 * locals.var_leff__blk603),)
    } else {
        (locals.var_wk, locals.var_wk_dn0, locals.var_wk_dn2, locals.var_wk_dn6, locals.var_wk_dn7, locals.var_wk_dn10, locals.var_wk_dn11, locals.var_wk_dn12, locals.var_wk_dn17,)
    }
};
        locals.var_wk = assign19810_e27290;
        locals.var_wk_dn0 = assign19810_e27290_d_n0;
        locals.var_wk_dn2 = assign19810_e27290_d_n2;
        locals.var_wk_dn6 = assign19810_e27290_d_n6;
        locals.var_wk_dn7 = assign19810_e27290_d_n7;
        locals.var_wk_dn10 = assign19810_e27290_d_n10;
        locals.var_wk_dn11 = assign19810_e27290_d_n11;
        locals.var_wk_dn12 = assign19810_e27290_d_n12;
        locals.var_wk_dn17 = assign19810_e27290_d_n17;
        locals.var_wk_rv = 0.0;

        let (assign19820_e27303, assign19820_e27303_d_n0, assign19820_e27303_d_n2, assign19820_e27303_d_n6, assign19820_e27303_d_n7, assign19820_e27303_d_n10, assign19820_e27303_d_n11, assign19820_e27303_d_n12, assign19820_e27303_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19820_e27293: f64 = (-0.25);
        let assign19820_e27295: f64 = (assign19820_e27293 * locals.var_wk);
        let assign19820_e27297: f64 = (assign19820_e27295 * locals.var_wk);
        let assign19820_e27300: f64 = (locals.var_vds + locals.var_wk);
        let assign19820_e27301: f64 = (assign19820_e27297 / assign19820_e27300);
        (assign19820_e27301, ((((((assign19820_e27293 * locals.var_wk_dn0) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn0)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn0 + locals.var_wk_dn0))) / (assign19820_e27300 * assign19820_e27300)), ((((((assign19820_e27293 * locals.var_wk_dn2) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn2)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn2 + locals.var_wk_dn2))) / (assign19820_e27300 * assign19820_e27300)), ((((((assign19820_e27293 * locals.var_wk_dn6) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn6)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn6 + locals.var_wk_dn6))) / (assign19820_e27300 * assign19820_e27300)), ((((((assign19820_e27293 * locals.var_wk_dn7) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn7)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn7 + locals.var_wk_dn7))) / (assign19820_e27300 * assign19820_e27300)), ((((((assign19820_e27293 * locals.var_wk_dn10) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn10)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn10 + locals.var_wk_dn10))) / (assign19820_e27300 * assign19820_e27300)), ((((((assign19820_e27293 * locals.var_wk_dn11) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn11)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn11 + locals.var_wk_dn11))) / (assign19820_e27300 * assign19820_e27300)), ((((((assign19820_e27293 * locals.var_wk_dn12) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn12)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn12 + locals.var_wk_dn12))) / (assign19820_e27300 * assign19820_e27300)), ((((((assign19820_e27293 * locals.var_wk_dn17) * locals.var_wk) + (assign19820_e27295 * locals.var_wk_dn17)) * assign19820_e27300) - (assign19820_e27297 * (locals.var_vds_dn17 + locals.var_wk_dn17))) / (assign19820_e27300 * assign19820_e27300)),)
    } else {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    }
};
        locals.var_dphi_vds = assign19820_e27303;
        locals.var_dphi_vds_dn0 = assign19820_e27303_d_n0;
        locals.var_dphi_vds_dn2 = assign19820_e27303_d_n2;
        locals.var_dphi_vds_dn6 = assign19820_e27303_d_n6;
        locals.var_dphi_vds_dn7 = assign19820_e27303_d_n7;
        locals.var_dphi_vds_dn10 = assign19820_e27303_d_n10;
        locals.var_dphi_vds_dn11 = assign19820_e27303_d_n11;
        locals.var_dphi_vds_dn12 = assign19820_e27303_d_n12;
        locals.var_dphi_vds_dn17 = assign19820_e27303_d_n17;
        locals.var_dphi_vds_rv = 0.0;

        let (assign19830_e27307, assign19830_e27307_d_n0, assign19830_e27307_d_n2, assign19830_e27307_d_n6, assign19830_e27307_d_n7, assign19830_e27307_d_n10, assign19830_e27307_d_n11, assign19830_e27307_d_n12, assign19830_e27307_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    } else {
        (locals.var_vbs__blk623, locals.var_vbs__blk623_dn0, locals.var_vbs__blk623_dn2, locals.var_vbs__blk623_dn6, locals.var_vbs__blk623_dn7, locals.var_vbs__blk623_dn10, locals.var_vbs__blk623_dn11, locals.var_vbs__blk623_dn12, locals.var_vbs__blk623_dn17,)
    }
};
        locals.var_vbs__blk623 = assign19830_e27307;
        locals.var_vbs__blk623_dn0 = assign19830_e27307_d_n0;
        locals.var_vbs__blk623_dn2 = assign19830_e27307_d_n2;
        locals.var_vbs__blk623_dn6 = assign19830_e27307_d_n6;
        locals.var_vbs__blk623_dn7 = assign19830_e27307_d_n7;
        locals.var_vbs__blk623_dn10 = assign19830_e27307_d_n10;
        locals.var_vbs__blk623_dn11 = assign19830_e27307_d_n11;
        locals.var_vbs__blk623_dn12 = assign19830_e27307_d_n12;
        locals.var_vbs__blk623_dn17 = assign19830_e27307_d_n17;
        locals.var_vbs__blk623_rv = 0.0;

        let (assign19840_e27311, assign19840_e27311_d_n0, assign19840_e27311_d_n2, assign19840_e27311_d_n6, assign19840_e27311_d_n7, assign19840_e27311_d_n10, assign19840_e27311_d_n11, assign19840_e27311_d_n12, assign19840_e27311_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn12, locals.var_wk_vth_dn17,)
    } else {
        (locals.var_vth__blk624, locals.var_vth__blk624_dn0, locals.var_vth__blk624_dn2, locals.var_vth__blk624_dn6, locals.var_vth__blk624_dn7, locals.var_vth__blk624_dn10, locals.var_vth__blk624_dn11, locals.var_vth__blk624_dn12, locals.var_vth__blk624_dn17,)
    }
};
        locals.var_vth__blk624 = assign19840_e27311;
        locals.var_vth__blk624_dn0 = assign19840_e27311_d_n0;
        locals.var_vth__blk624_dn2 = assign19840_e27311_d_n2;
        locals.var_vth__blk624_dn6 = assign19840_e27311_d_n6;
        locals.var_vth__blk624_dn7 = assign19840_e27311_d_n7;
        locals.var_vth__blk624_dn10 = assign19840_e27311_d_n10;
        locals.var_vth__blk624_dn11 = assign19840_e27311_d_n11;
        locals.var_vth__blk624_dn12 = assign19840_e27311_d_n12;
        locals.var_vth__blk624_dn17 = assign19840_e27311_d_n17;
        locals.var_vth__blk624_rv = 0.0;

        let (assign19850_e27329, assign19850_e27329_d_n0, assign19850_e27329_d_n2, assign19850_e27329_d_n6, assign19850_e27329_d_n7, assign19850_e27329_d_n10, assign19850_e27329_d_n11, assign19850_e27329_d_n12, assign19850_e27329_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19850_e27318: f64 = (locals.var_vgp__blk608 - locals.var_vbs__blk623);
        let assign19850_e27319: f64 = (locals.var_beta * assign19850_e27318);
        let assign19850_e27321: f64 = (assign19850_e27319 - 1.0);
        let assign19850_e27322: f64 = (4.0 * assign19850_e27321);
        let assign19850_e27325: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign19850_e27326: f64 = (assign19850_e27322 / assign19850_e27325);
        let assign19850_e27327: f64 = (1.0 + assign19850_e27326);
        (assign19850_e27327, ((((4.0 * (locals.var_beta * (locals.var_vgp__blk608_dn0 - locals.var_vbs__blk623_dn0))) * assign19850_e27325) - (assign19850_e27322 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign19850_e27325 * assign19850_e27325)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk608_dn2 - locals.var_vbs__blk623_dn2))) * assign19850_e27325) - (assign19850_e27322 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign19850_e27325 * assign19850_e27325)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk608_dn6 - locals.var_vbs__blk623_dn6))) * assign19850_e27325) - (assign19850_e27322 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign19850_e27325 * assign19850_e27325)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk608_dn7 - locals.var_vbs__blk623_dn7))) * assign19850_e27325) - (assign19850_e27322 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign19850_e27325 * assign19850_e27325)), ((((4.0 * ((locals.var_beta_dn10 * assign19850_e27318) + (locals.var_beta * (locals.var_vgp__blk608_dn10 - locals.var_vbs__blk623_dn10)))) * assign19850_e27325) - (assign19850_e27322 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign19850_e27325 * assign19850_e27325)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk608_dn11 - locals.var_vbs__blk623_dn11))) * assign19850_e27325) - (assign19850_e27322 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign19850_e27325 * assign19850_e27325)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk608_dn12 - locals.var_vbs__blk623_dn12))) * assign19850_e27325) - (assign19850_e27322 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign19850_e27325 * assign19850_e27325)), ((((4.0 * (locals.var_beta * (locals.var_vgp__blk608_dn17 - locals.var_vbs__blk623_dn17))) * assign19850_e27325) - (assign19850_e27322 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign19850_e27325 * assign19850_e27325)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign19850_e27329;
        locals.var_tx_dn0 = assign19850_e27329_d_n0;
        locals.var_tx_dn2 = assign19850_e27329_d_n2;
        locals.var_tx_dn6 = assign19850_e27329_d_n6;
        locals.var_tx_dn7 = assign19850_e27329_d_n7;
        locals.var_tx_dn10 = assign19850_e27329_d_n10;
        locals.var_tx_dn11 = assign19850_e27329_d_n11;
        locals.var_tx_dn12 = assign19850_e27329_d_n12;
        locals.var_tx_dn17 = assign19850_e27329_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign19860_e27342, assign19860_e27342_d_n0, assign19860_e27342_d_n2, assign19860_e27342_d_n6, assign19860_e27342_d_n7, assign19860_e27342_d_n10, assign19860_e27342_d_n11, assign19860_e27342_d_n12, assign19860_e27342_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19860_e27334: f64 = (10.0 * 2.220446049250313e-16);
        let (assign19860_e27340, assign19860_e27340_d_n0, assign19860_e27340_d_n2, assign19860_e27340_d_n6, assign19860_e27340_d_n7, assign19860_e27340_d_n10, assign19860_e27340_d_n11, assign19860_e27340_d_n12, assign19860_e27340_d_n17,) = {
            if (locals.var_tx >= assign19860_e27334) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign19860_e27339: f64 = (10.0 * 2.220446049250313e-16);
                (assign19860_e27339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign19860_e27340, assign19860_e27340_d_n0, assign19860_e27340_d_n2, assign19860_e27340_d_n6, assign19860_e27340_d_n7, assign19860_e27340_d_n10, assign19860_e27340_d_n11, assign19860_e27340_d_n12, assign19860_e27340_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign19860_e27342;
        locals.var_tx_dn0 = assign19860_e27342_d_n0;
        locals.var_tx_dn2 = assign19860_e27342_d_n2;
        locals.var_tx_dn6 = assign19860_e27342_d_n6;
        locals.var_tx_dn7 = assign19860_e27342_d_n7;
        locals.var_tx_dn10 = assign19860_e27342_d_n10;
        locals.var_tx_dn11 = assign19860_e27342_d_n11;
        locals.var_tx_dn12 = assign19860_e27342_d_n12;
        locals.var_tx_dn17 = assign19860_e27342_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign19870_e27357, assign19870_e27357_d_n0, assign19870_e27357_d_n2, assign19870_e27357_d_n6, assign19870_e27357_d_n7, assign19870_e27357_d_n10, assign19870_e27357_d_n11, assign19870_e27357_d_n12, assign19870_e27357_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign19870_e27347: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign19870_e27349: f64 = (assign19870_e27347 * 0.5);
        let assign19870_e27352: f64 = (locals.var_tx).sqrt();
        let assign19870_e27353: f64 = (1.0 - assign19870_e27352);
        let assign19870_e27354: f64 = (assign19870_e27349 * assign19870_e27353);
        let assign19870_e27355: f64 = (locals.var_vgp__blk608 + assign19870_e27354);
        (assign19870_e27355, (locals.var_vgp__blk608_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn0 / (2.0 * assign19870_e27352)))))), (locals.var_vgp__blk608_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn2 / (2.0 * assign19870_e27352)))))), (locals.var_vgp__blk608_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn6 / (2.0 * assign19870_e27352)))))), (locals.var_vgp__blk608_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn7 / (2.0 * assign19870_e27352)))))), (locals.var_vgp__blk608_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn10 / (2.0 * assign19870_e27352)))))), (locals.var_vgp__blk608_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn11 / (2.0 * assign19870_e27352)))))), (locals.var_vgp__blk608_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn12 / (2.0 * assign19870_e27352)))))), (locals.var_vgp__blk608_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign19870_e27353) + (assign19870_e27349 * (-(locals.var_tx_dn17 / (2.0 * assign19870_e27352)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign19870_e27357;
        locals.var_ps0_inia_dn0 = assign19870_e27357_d_n0;
        locals.var_ps0_inia_dn2 = assign19870_e27357_d_n2;
        locals.var_ps0_inia_dn6 = assign19870_e27357_d_n6;
        locals.var_ps0_inia_dn7 = assign19870_e27357_d_n7;
        locals.var_ps0_inia_dn10 = assign19870_e27357_d_n10;
        locals.var_ps0_inia_dn11 = assign19870_e27357_d_n11;
        locals.var_ps0_inia_dn12 = assign19870_e27357_d_n12;
        locals.var_ps0_inia_dn17 = assign19870_e27357_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let assign19880_e27361: f64 = (locals.var_vfb + locals.var_vth__blk624);
        let assign19880_e27363: f64 = (assign19880_e27361 * 0.5);
        let assign19880_e27364: f64 = if locals.var_vgs < assign19880_e27363 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign19880_e27364;
        locals.var_guard625_rv = 0.0;

        let (assign19890_e27370,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard625 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_pprv,)
    }
};
        locals.var_flg_pprv = assign19890_e27370;
        locals.var_flg_pprv_rv = 0.0;

        let assign19900_e27375: f64 = if ((locals.var_flg_pprv == 0.0) || (1.0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard626 = assign19900_e27375;
        locals.var_guard626_rv = 0.0;

        let (assign19910_e27385, assign19910_e27385_d_n0, assign19910_e27385_d_n2, assign19910_e27385_d_n6, assign19910_e27385_d_n7, assign19910_e27385_d_n10, assign19910_e27385_d_n11, assign19910_e27385_d_n12, assign19910_e27385_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign19910_e27382: f64 = (locals.var_ps0_inia - locals.var_vbs__blk623);
        let assign19910_e27383: f64 = (locals.var_beta * assign19910_e27382);
        (assign19910_e27383, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbs__blk623_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbs__blk623_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbs__blk623_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbs__blk623_dn7)), ((locals.var_beta_dn10 * assign19910_e27382) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbs__blk623_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbs__blk623_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbs__blk623_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbs__blk623_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign19910_e27385;
        locals.var_chi_dn0 = assign19910_e27385_d_n0;
        locals.var_chi_dn2 = assign19910_e27385_d_n2;
        locals.var_chi_dn6 = assign19910_e27385_d_n6;
        locals.var_chi_dn7 = assign19910_e27385_d_n7;
        locals.var_chi_dn10 = assign19910_e27385_d_n10;
        locals.var_chi_dn11 = assign19910_e27385_d_n11;
        locals.var_chi_dn12 = assign19910_e27385_d_n12;
        locals.var_chi_dn17 = assign19910_e27385_d_n17;
        locals.var_chi_rv = 0.0;

        let assign19920_e27388: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard627 = assign19920_e27388;
        locals.var_guard627_rv = 0.0;

        let (assign19930_e27400, assign19930_e27400_d_n0, assign19930_e27400_d_n2, assign19930_e27400_d_n6, assign19930_e27400_d_n7, assign19930_e27400_d_n10, assign19930_e27400_d_n11, assign19930_e27400_d_n12, assign19930_e27400_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign19930_e27397: f64 = (locals.var_vgp__blk608 - locals.var_vbs__blk623);
        let assign19930_e27398: f64 = (locals.var_beta * assign19930_e27397);
        (assign19930_e27398, (locals.var_beta * (locals.var_vgp__blk608_dn0 - locals.var_vbs__blk623_dn0)), (locals.var_beta * (locals.var_vgp__blk608_dn2 - locals.var_vbs__blk623_dn2)), (locals.var_beta * (locals.var_vgp__blk608_dn6 - locals.var_vbs__blk623_dn6)), (locals.var_beta * (locals.var_vgp__blk608_dn7 - locals.var_vbs__blk623_dn7)), ((locals.var_beta_dn10 * assign19930_e27397) + (locals.var_beta * (locals.var_vgp__blk608_dn10 - locals.var_vbs__blk623_dn10))), (locals.var_beta * (locals.var_vgp__blk608_dn11 - locals.var_vbs__blk623_dn11)), (locals.var_beta * (locals.var_vgp__blk608_dn12 - locals.var_vbs__blk623_dn12)), (locals.var_beta * (locals.var_vgp__blk608_dn17 - locals.var_vbs__blk623_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign19930_e27400;
        locals.var_ty_dn0 = assign19930_e27400_d_n0;
        locals.var_ty_dn2 = assign19930_e27400_d_n2;
        locals.var_ty_dn6 = assign19930_e27400_d_n6;
        locals.var_ty_dn7 = assign19930_e27400_d_n7;
        locals.var_ty_dn10 = assign19930_e27400_d_n10;
        locals.var_ty_dn11 = assign19930_e27400_d_n11;
        locals.var_ty_dn12 = assign19930_e27400_d_n12;
        locals.var_ty_dn17 = assign19930_e27400_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign19940_e27416, assign19940_e27416_d_n0, assign19940_e27416_d_n2, assign19940_e27416_d_n6, assign19940_e27416_d_n7, assign19940_e27416_d_n10, assign19940_e27416_d_n11, assign19940_e27416_d_n12, assign19940_e27416_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign19940_e27409: f64 = (1.414213562373095 / 108.0);
        let assign19940_e27411: f64 = (assign19940_e27409 * locals.var_beta);
        let assign19940_e27413: f64 = (assign19940_e27411 * locals.var_fac1);
        let assign19940_e27414: f64 = (1.0 / assign19940_e27413);
        (assign19940_e27414, (-((assign19940_e27411 * locals.var_fac1_dn0) / (assign19940_e27413 * assign19940_e27413))), (-((assign19940_e27411 * locals.var_fac1_dn2) / (assign19940_e27413 * assign19940_e27413))), (-((assign19940_e27411 * locals.var_fac1_dn6) / (assign19940_e27413 * assign19940_e27413))), (-((assign19940_e27411 * locals.var_fac1_dn7) / (assign19940_e27413 * assign19940_e27413))), (-((((assign19940_e27409 * locals.var_beta_dn10) * locals.var_fac1) + (assign19940_e27411 * locals.var_fac1_dn10)) / (assign19940_e27413 * assign19940_e27413))), (-((assign19940_e27411 * locals.var_fac1_dn11) / (assign19940_e27413 * assign19940_e27413))), (-((assign19940_e27411 * locals.var_fac1_dn12) / (assign19940_e27413 * assign19940_e27413))), (-((assign19940_e27411 * locals.var_fac1_dn17) / (assign19940_e27413 * assign19940_e27413))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign19940_e27416;
        locals.var_t1_dn0 = assign19940_e27416_d_n0;
        locals.var_t1_dn2 = assign19940_e27416_d_n2;
        locals.var_t1_dn6 = assign19940_e27416_d_n6;
        locals.var_t1_dn7 = assign19940_e27416_d_n7;
        locals.var_t1_dn10 = assign19940_e27416_d_n10;
        locals.var_t1_dn11 = assign19940_e27416_d_n11;
        locals.var_t1_dn12 = assign19940_e27416_d_n12;
        locals.var_t1_dn17 = assign19940_e27416_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign19950_e27428, assign19950_e27428_d_n0, assign19950_e27428_d_n2, assign19950_e27428_d_n6, assign19950_e27428_d_n7, assign19950_e27428_d_n10, assign19950_e27428_d_n11, assign19950_e27428_d_n12, assign19950_e27428_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign19950_e27425: f64 = (3.0 * locals.var_t1);
        let assign19950_e27426: f64 = (81.0 + assign19950_e27425);
        (assign19950_e27426, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign19950_e27428;
        locals.var_t2_dn0 = assign19950_e27428_d_n0;
        locals.var_t2_dn2 = assign19950_e27428_d_n2;
        locals.var_t2_dn6 = assign19950_e27428_d_n6;
        locals.var_t2_dn7 = assign19950_e27428_d_n7;
        locals.var_t2_dn10 = assign19950_e27428_d_n10;
        locals.var_t2_dn11 = assign19950_e27428_d_n11;
        locals.var_t2_dn12 = assign19950_e27428_d_n12;
        locals.var_t2_dn17 = assign19950_e27428_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign19960_e27447, assign19960_e27447_d_n0, assign19960_e27447_d_n2, assign19960_e27447_d_n6, assign19960_e27447_d_n7, assign19960_e27447_d_n10, assign19960_e27447_d_n11, assign19960_e27447_d_n12, assign19960_e27447_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign19960_e27435: f64 = (-2916.0);
        let assign19960_e27438: f64 = (81.0 * locals.var_t1);
        let assign19960_e27439: f64 = (assign19960_e27435 - assign19960_e27438);
        let assign19960_e27442: f64 = (27.0 * locals.var_t1);
        let assign19960_e27444: f64 = (assign19960_e27442 * locals.var_ty);
        let assign19960_e27445: f64 = (assign19960_e27439 + assign19960_e27444);
        (assign19960_e27445, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign19960_e27442 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign19960_e27447;
        locals.var_t3_dn0 = assign19960_e27447_d_n0;
        locals.var_t3_dn2 = assign19960_e27447_d_n2;
        locals.var_t3_dn6 = assign19960_e27447_d_n6;
        locals.var_t3_dn7 = assign19960_e27447_d_n7;
        locals.var_t3_dn10 = assign19960_e27447_d_n10;
        locals.var_t3_dn11 = assign19960_e27447_d_n11;
        locals.var_t3_dn12 = assign19960_e27447_d_n12;
        locals.var_t3_dn17 = assign19960_e27447_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign19970_e27467, assign19970_e27467_d_n0, assign19970_e27467_d_n2, assign19970_e27467_d_n6, assign19970_e27467_d_n7, assign19970_e27467_d_n10, assign19970_e27467_d_n11, assign19970_e27467_d_n12, assign19970_e27467_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign19970_e27457: f64 = (54.0 + locals.var_t1);
        let assign19970_e27458: f64 = (81.0 * assign19970_e27457);
        let assign19970_e27459: f64 = (1458.0 - assign19970_e27458);
        let assign19970_e27462: f64 = (27.0 * locals.var_t1);
        let assign19970_e27464: f64 = (assign19970_e27462 * locals.var_ty);
        let assign19970_e27465: f64 = (assign19970_e27459 + assign19970_e27464);
        (assign19970_e27465, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign19970_e27462 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign19970_e27467;
        locals.var_t4_dn0 = assign19970_e27467_d_n0;
        locals.var_t4_dn2 = assign19970_e27467_d_n2;
        locals.var_t4_dn6 = assign19970_e27467_d_n6;
        locals.var_t4_dn7 = assign19970_e27467_d_n7;
        locals.var_t4_dn10 = assign19970_e27467_d_n10;
        locals.var_t4_dn11 = assign19970_e27467_d_n11;
        locals.var_t4_dn12 = assign19970_e27467_d_n12;
        locals.var_t4_dn17 = assign19970_e27467_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign19980_e27477, assign19980_e27477_d_n0, assign19980_e27477_d_n2, assign19980_e27477_d_n6, assign19980_e27477_d_n7, assign19980_e27477_d_n10, assign19980_e27477_d_n11, assign19980_e27477_d_n12, assign19980_e27477_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign19980_e27475: f64 = (locals.var_t4 * locals.var_t4);
        (assign19980_e27475, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign19980_e27477;
        locals.var_t4_dn0 = assign19980_e27477_d_n0;
        locals.var_t4_dn2 = assign19980_e27477_d_n2;
        locals.var_t4_dn6 = assign19980_e27477_d_n6;
        locals.var_t4_dn7 = assign19980_e27477_d_n7;
        locals.var_t4_dn10 = assign19980_e27477_d_n10;
        locals.var_t4_dn11 = assign19980_e27477_d_n11;
        locals.var_t4_dn12 = assign19980_e27477_d_n12;
        locals.var_t4_dn17 = assign19980_e27477_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign19990_e27498, assign19990_e27498_d_n0, assign19990_e27498_d_n2, assign19990_e27498_d_n6, assign19990_e27498_d_n7, assign19990_e27498_d_n10, assign19990_e27498_d_n11, assign19990_e27498_d_n12, assign19990_e27498_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign19990_e27486: f64 = (4.0 * locals.var_t2);
        let assign19990_e27488: f64 = (assign19990_e27486 * locals.var_t2);
        let assign19990_e27490: f64 = (assign19990_e27488 * locals.var_t2);
        let assign19990_e27492: f64 = (assign19990_e27490 + locals.var_t4);
        let assign19990_e27493: f64 = (assign19990_e27492).sqrt();
        let assign19990_e27494: f64 = (locals.var_t3 + assign19990_e27493);
        let assign19990_e27496: f64 = (assign19990_e27494).powf(0.3333333333333333);
        (assign19990_e27496, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn0)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn0)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign19990_e27493))) / assign19990_e27494))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn2)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn2)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign19990_e27493))) / assign19990_e27494))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn6)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn6)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign19990_e27493))) / assign19990_e27494))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn7)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn7)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign19990_e27493))) / assign19990_e27494))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn10)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn10)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign19990_e27493))) / assign19990_e27494))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn11)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn11)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign19990_e27493))) / assign19990_e27494))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn12)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn12)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign19990_e27493))) / assign19990_e27494))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign19990_e27494).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn17)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign19990_e27493))))) } } else { (assign19990_e27496 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign19990_e27486 * locals.var_t2_dn17)) * locals.var_t2) + (assign19990_e27488 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign19990_e27493))) / assign19990_e27494))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign19990_e27498;
        locals.var_t5_dn0 = assign19990_e27498_d_n0;
        locals.var_t5_dn2 = assign19990_e27498_d_n2;
        locals.var_t5_dn6 = assign19990_e27498_d_n6;
        locals.var_t5_dn7 = assign19990_e27498_d_n7;
        locals.var_t5_dn10 = assign19990_e27498_d_n10;
        locals.var_t5_dn11 = assign19990_e27498_d_n11;
        locals.var_t5_dn12 = assign19990_e27498_d_n12;
        locals.var_t5_dn17 = assign19990_e27498_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign20000_e27522, assign20000_e27522_d_n0, assign20000_e27522_d_n2, assign20000_e27522_d_n6, assign20000_e27522_d_n7, assign20000_e27522_d_n10, assign20000_e27522_d_n11, assign20000_e27522_d_n12, assign20000_e27522_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign20000_e27507: f64 = (1.259921049894873 * locals.var_t2);
        let assign20000_e27510: f64 = (3.0 * locals.var_t5);
        let assign20000_e27511: f64 = (assign20000_e27507 / assign20000_e27510);
        let assign20000_e27512: f64 = (3.0 - assign20000_e27511);
        let assign20000_e27516: f64 = (3.0 * 1.259921049894873);
        let assign20000_e27517: f64 = (1.0 / assign20000_e27516);
        let assign20000_e27519: f64 = (assign20000_e27517 * locals.var_t5);
        let assign20000_e27520: f64 = (assign20000_e27512 + assign20000_e27519);
        (assign20000_e27520, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn0))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn2))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn6))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn7))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn10))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn11))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn12))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign20000_e27510) - (assign20000_e27507 * (3.0 * locals.var_t5_dn17))) / (assign20000_e27510 * assign20000_e27510))) + (assign20000_e27517 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign20000_e27522;
        locals.var_tx_dn0 = assign20000_e27522_d_n0;
        locals.var_tx_dn2 = assign20000_e27522_d_n2;
        locals.var_tx_dn6 = assign20000_e27522_d_n6;
        locals.var_tx_dn7 = assign20000_e27522_d_n7;
        locals.var_tx_dn10 = assign20000_e27522_d_n10;
        locals.var_tx_dn11 = assign20000_e27522_d_n11;
        locals.var_tx_dn12 = assign20000_e27522_d_n12;
        locals.var_tx_dn17 = assign20000_e27522_d_n17;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20010_e27534, assign20010_e27534_d_n0, assign20010_e27534_d_n2, assign20010_e27534_d_n6, assign20010_e27534_d_n7, assign20010_e27534_d_n10, assign20010_e27534_d_n11, assign20010_e27534_d_n12, assign20010_e27534_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign20010_e27530: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign20010_e27532: f64 = (assign20010_e27530 + locals.var_vbs__blk623);
        (assign20010_e27532, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbs__blk623_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbs__blk623_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbs__blk623_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbs__blk623_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbs__blk623_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbs__blk623_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbs__blk623_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbs__blk623_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20010_e27534;
        locals.var_ps0_inia_dn0 = assign20010_e27534_d_n0;
        locals.var_ps0_inia_dn2 = assign20010_e27534_d_n2;
        locals.var_ps0_inia_dn6 = assign20010_e27534_d_n6;
        locals.var_ps0_inia_dn7 = assign20010_e27534_d_n7;
        locals.var_ps0_inia_dn10 = assign20010_e27534_d_n10;
        locals.var_ps0_inia_dn11 = assign20010_e27534_d_n11;
        locals.var_ps0_inia_dn12 = assign20010_e27534_d_n12;
        locals.var_ps0_inia_dn17 = assign20010_e27534_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20020_e27542, assign20020_e27542_d_n0, assign20020_e27542_d_n2, assign20020_e27542_d_n6, assign20020_e27542_d_n7, assign20020_e27542_d_n10, assign20020_e27542_d_n11, assign20020_e27542_d_n12, assign20020_e27542_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20020_e27542;
        locals.var_ps0_ini_dn0 = assign20020_e27542_d_n0;
        locals.var_ps0_ini_dn2 = assign20020_e27542_d_n2;
        locals.var_ps0_ini_dn6 = assign20020_e27542_d_n6;
        locals.var_ps0_ini_dn7 = assign20020_e27542_d_n7;
        locals.var_ps0_ini_dn10 = assign20020_e27542_d_n10;
        locals.var_ps0_ini_dn11 = assign20020_e27542_d_n11;
        locals.var_ps0_ini_dn12 = assign20020_e27542_d_n12;
        locals.var_ps0_ini_dn17 = assign20020_e27542_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20030_e27545: f64 = (locals.var_vgs - locals.var_shift);
        let assign20030_e27547: f64 = if assign20030_e27545 <= locals.var_vth__blk624 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign20030_e27547;
        locals.var_guard628_rv = 0.0;

        let assign20040_e27550: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign20040_e27550;
        locals.var_guard629_rv = 0.0;

        let (assign20050_e27565, assign20050_e27565_d_n0, assign20050_e27565_d_n2, assign20050_e27565_d_n6, assign20050_e27565_d_n7, assign20050_e27565_d_n10, assign20050_e27565_d_n11, assign20050_e27565_d_n12, assign20050_e27565_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20050_e27563: f64 = (1.0 / locals.var_c_fox);
        (assign20050_e27563, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20050_e27565;
        locals.var_t0_dn0 = assign20050_e27565_d_n0;
        locals.var_t0_dn2 = assign20050_e27565_d_n2;
        locals.var_t0_dn6 = assign20050_e27565_d_n6;
        locals.var_t0_dn7 = assign20050_e27565_d_n7;
        locals.var_t0_dn10 = assign20050_e27565_d_n10;
        locals.var_t0_dn11 = assign20050_e27565_d_n11;
        locals.var_t0_dn12 = assign20050_e27565_d_n12;
        locals.var_t0_dn17 = assign20050_e27565_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20060_e27580, assign20060_e27580_d_n0, assign20060_e27580_d_n2, assign20060_e27580_d_n6, assign20060_e27580_d_n7, assign20060_e27580_d_n10, assign20060_e27580_d_n11, assign20060_e27580_d_n12, assign20060_e27580_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20060_e27578: f64 = (locals.var_t_soi__blk607 / 1.034943e-10);
        (assign20060_e27578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20060_e27580;
        locals.var_t1_dn0 = assign20060_e27580_d_n0;
        locals.var_t1_dn2 = assign20060_e27580_d_n2;
        locals.var_t1_dn6 = assign20060_e27580_d_n6;
        locals.var_t1_dn7 = assign20060_e27580_d_n7;
        locals.var_t1_dn10 = assign20060_e27580_d_n10;
        locals.var_t1_dn11 = assign20060_e27580_d_n11;
        locals.var_t1_dn12 = assign20060_e27580_d_n12;
        locals.var_t1_dn17 = assign20060_e27580_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20070_e27595, assign20070_e27595_d_n0, assign20070_e27595_d_n2, assign20070_e27595_d_n6, assign20070_e27595_d_n7, assign20070_e27595_d_n10, assign20070_e27595_d_n11, assign20070_e27595_d_n12, assign20070_e27595_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20070_e27593: f64 = (1.0 / locals.var_c_box);
        (assign20070_e27593, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20070_e27595;
        locals.var_t2_dn0 = assign20070_e27595_d_n0;
        locals.var_t2_dn2 = assign20070_e27595_d_n2;
        locals.var_t2_dn6 = assign20070_e27595_d_n6;
        locals.var_t2_dn7 = assign20070_e27595_d_n7;
        locals.var_t2_dn10 = assign20070_e27595_d_n10;
        locals.var_t2_dn11 = assign20070_e27595_d_n11;
        locals.var_t2_dn12 = assign20070_e27595_d_n12;
        locals.var_t2_dn17 = assign20070_e27595_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20080_e27614, assign20080_e27614_d_n0, assign20080_e27614_d_n2, assign20080_e27614_d_n6, assign20080_e27614_d_n7, assign20080_e27614_d_n10, assign20080_e27614_d_n11, assign20080_e27614_d_n12, assign20080_e27614_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20080_e27609: f64 = (locals.var_t0 + locals.var_t1);
        let assign20080_e27611: f64 = (assign20080_e27609 + locals.var_t2);
        let assign20080_e27612: f64 = (1.0 / assign20080_e27611);
        (assign20080_e27612, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20080_e27611 * assign20080_e27611))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20080_e27611 * assign20080_e27611))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20080_e27611 * assign20080_e27611))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20080_e27611 * assign20080_e27611))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20080_e27611 * assign20080_e27611))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20080_e27611 * assign20080_e27611))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20080_e27611 * assign20080_e27611))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20080_e27611 * assign20080_e27611))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20080_e27614;
        locals.var_t3_dn0 = assign20080_e27614_d_n0;
        locals.var_t3_dn2 = assign20080_e27614_d_n2;
        locals.var_t3_dn6 = assign20080_e27614_d_n6;
        locals.var_t3_dn7 = assign20080_e27614_d_n7;
        locals.var_t3_dn10 = assign20080_e27614_d_n10;
        locals.var_t3_dn11 = assign20080_e27614_d_n11;
        locals.var_t3_dn12 = assign20080_e27614_d_n12;
        locals.var_t3_dn17 = assign20080_e27614_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20090_e27640, assign20090_e27640_d_n0, assign20090_e27640_d_n2, assign20090_e27640_d_n6, assign20090_e27640_d_n7, assign20090_e27640_d_n10, assign20090_e27640_d_n11, assign20090_e27640_d_n12, assign20090_e27640_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20090_e27628: f64 = (locals.var_vgp__blk608 - locals.var_vbsbiz);
        let assign20090_e27632: f64 = (0.5 * locals.var_t1);
        let assign20090_e27633: f64 = (locals.var_t2 + assign20090_e27632);
        let assign20090_e27635: f64 = (-locals.var_q_s0_dep_ini);
        let assign20090_e27636: f64 = (assign20090_e27633 * assign20090_e27635);
        let assign20090_e27637: f64 = (assign20090_e27628 + assign20090_e27636);
        let assign20090_e27638: f64 = (locals.var_t3 * assign20090_e27637);
        (assign20090_e27638, ((locals.var_t3_dn0 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20090_e27637) + (locals.var_t3 * ((locals.var_vgp__blk608_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20090_e27635) + (assign20090_e27633 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20090_e27640;
        locals.var_t4_dn0 = assign20090_e27640_d_n0;
        locals.var_t4_dn2 = assign20090_e27640_d_n2;
        locals.var_t4_dn6 = assign20090_e27640_d_n6;
        locals.var_t4_dn7 = assign20090_e27640_d_n7;
        locals.var_t4_dn10 = assign20090_e27640_d_n10;
        locals.var_t4_dn11 = assign20090_e27640_d_n11;
        locals.var_t4_dn12 = assign20090_e27640_d_n12;
        locals.var_t4_dn17 = assign20090_e27640_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20100_e27657, assign20100_e27657_d_n0, assign20100_e27657_d_n2, assign20100_e27657_d_n6, assign20100_e27657_d_n7, assign20100_e27657_d_n10, assign20100_e27657_d_n11, assign20100_e27657_d_n12, assign20100_e27657_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) && (locals.var_guard629 != 0.0)) {
        let assign20100_e27654: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20100_e27655: f64 = (locals.var_vgp__blk608 - assign20100_e27654);
        (assign20100_e27655, (locals.var_vgp__blk608_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20100_e27657;
        locals.var_ps0_inia_dn0 = assign20100_e27657_d_n0;
        locals.var_ps0_inia_dn2 = assign20100_e27657_d_n2;
        locals.var_ps0_inia_dn6 = assign20100_e27657_d_n6;
        locals.var_ps0_inia_dn7 = assign20100_e27657_d_n7;
        locals.var_ps0_inia_dn10 = assign20100_e27657_d_n10;
        locals.var_ps0_inia_dn11 = assign20100_e27657_d_n11;
        locals.var_ps0_inia_dn12 = assign20100_e27657_d_n12;
        locals.var_ps0_inia_dn17 = assign20100_e27657_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20110_e27668, assign20110_e27668_d_n0, assign20110_e27668_d_n2, assign20110_e27668_d_n6, assign20110_e27668_d_n7, assign20110_e27668_d_n10, assign20110_e27668_d_n11, assign20110_e27668_d_n12, assign20110_e27668_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20110_e27668;
        locals.var_ps0_ini_dn0 = assign20110_e27668_d_n0;
        locals.var_ps0_ini_dn2 = assign20110_e27668_d_n2;
        locals.var_ps0_ini_dn6 = assign20110_e27668_d_n6;
        locals.var_ps0_ini_dn7 = assign20110_e27668_d_n7;
        locals.var_ps0_ini_dn10 = assign20110_e27668_d_n10;
        locals.var_ps0_ini_dn11 = assign20110_e27668_d_n11;
        locals.var_ps0_ini_dn12 = assign20110_e27668_d_n12;
        locals.var_ps0_ini_dn17 = assign20110_e27668_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20120_e27684, assign20120_e27684_d_n0, assign20120_e27684_d_n2, assign20120_e27684_d_n6, assign20120_e27684_d_n7, assign20120_e27684_d_n10, assign20120_e27684_d_n11, assign20120_e27684_d_n12, assign20120_e27684_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20120_e27680: f64 = (1.0 / locals.var_cnst1soi);
        let assign20120_e27682: f64 = (assign20120_e27680 / locals.var_cnstc_foxi);
        (assign20120_e27682, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20120_e27680 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20120_e27684;
        locals.var_t1_dn0 = assign20120_e27684_d_n0;
        locals.var_t1_dn2 = assign20120_e27684_d_n2;
        locals.var_t1_dn6 = assign20120_e27684_d_n6;
        locals.var_t1_dn7 = assign20120_e27684_d_n7;
        locals.var_t1_dn10 = assign20120_e27684_d_n10;
        locals.var_t1_dn11 = assign20120_e27684_d_n11;
        locals.var_t1_dn12 = assign20120_e27684_d_n12;
        locals.var_t1_dn17 = assign20120_e27684_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20130_e27704, assign20130_e27704_d_n0, assign20130_e27704_d_n2, assign20130_e27704_d_n6, assign20130_e27704_d_n7, assign20130_e27704_d_n10, assign20130_e27704_d_n11, assign20130_e27704_d_n12, assign20130_e27704_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20130_e27697: f64 = (locals.var_vgp__blk608 - locals.var_shift);
        let assign20130_e27698: f64 = (locals.var_t1 * assign20130_e27697);
        let assign20130_e27701: f64 = (locals.var_vgp__blk608 - locals.var_shift);
        let assign20130_e27702: f64 = (assign20130_e27698 * assign20130_e27701);
        (assign20130_e27702, ((((locals.var_t1_dn0 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn0 - locals.var_shift_dn0))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn2 - locals.var_shift_dn2))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn6 - locals.var_shift_dn6))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn7 - locals.var_shift_dn7))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn10 - locals.var_shift_dn10))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn11 - locals.var_shift_dn11))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn12 - locals.var_shift_dn12))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign20130_e27697) + (locals.var_t1 * (locals.var_vgp__blk608_dn17 - locals.var_shift_dn17))) * assign20130_e27701) + (assign20130_e27698 * (locals.var_vgp__blk608_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20130_e27704;
        locals.var_t2_dn0 = assign20130_e27704_d_n0;
        locals.var_t2_dn2 = assign20130_e27704_d_n2;
        locals.var_t2_dn6 = assign20130_e27704_d_n6;
        locals.var_t2_dn7 = assign20130_e27704_d_n7;
        locals.var_t2_dn10 = assign20130_e27704_d_n10;
        locals.var_t2_dn11 = assign20130_e27704_d_n11;
        locals.var_t2_dn12 = assign20130_e27704_d_n12;
        locals.var_t2_dn17 = assign20130_e27704_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20140_e27722, assign20140_e27722_d_n0, assign20140_e27722_d_n2, assign20140_e27722_d_n6, assign20140_e27722_d_n7, assign20140_e27722_d_n10, assign20140_e27722_d_n11, assign20140_e27722_d_n12, assign20140_e27722_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20140_e27718: f64 = (locals.var_vgp__blk608 - locals.var_shift);
        let assign20140_e27719: f64 = (2.0 / assign20140_e27718);
        let assign20140_e27720: f64 = (locals.var_beta + assign20140_e27719);
        (assign20140_e27720, (-((2.0 * (locals.var_vgp__blk608_dn0 - locals.var_shift_dn0)) / (assign20140_e27718 * assign20140_e27718))), (-((2.0 * (locals.var_vgp__blk608_dn2 - locals.var_shift_dn2)) / (assign20140_e27718 * assign20140_e27718))), (-((2.0 * (locals.var_vgp__blk608_dn6 - locals.var_shift_dn6)) / (assign20140_e27718 * assign20140_e27718))), (-((2.0 * (locals.var_vgp__blk608_dn7 - locals.var_shift_dn7)) / (assign20140_e27718 * assign20140_e27718))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp__blk608_dn10 - locals.var_shift_dn10)) / (assign20140_e27718 * assign20140_e27718)))), (-((2.0 * (locals.var_vgp__blk608_dn11 - locals.var_shift_dn11)) / (assign20140_e27718 * assign20140_e27718))), (-((2.0 * (locals.var_vgp__blk608_dn12 - locals.var_shift_dn12)) / (assign20140_e27718 * assign20140_e27718))), (-((2.0 * (locals.var_vgp__blk608_dn17 - locals.var_shift_dn17)) / (assign20140_e27718 * assign20140_e27718))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20140_e27722;
        locals.var_t3_dn0 = assign20140_e27722_d_n0;
        locals.var_t3_dn2 = assign20140_e27722_d_n2;
        locals.var_t3_dn6 = assign20140_e27722_d_n6;
        locals.var_t3_dn7 = assign20140_e27722_d_n7;
        locals.var_t3_dn10 = assign20140_e27722_d_n10;
        locals.var_t3_dn11 = assign20140_e27722_d_n11;
        locals.var_t3_dn12 = assign20140_e27722_d_n12;
        locals.var_t3_dn17 = assign20140_e27722_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20150_e27739, assign20150_e27739_d_n0, assign20150_e27739_d_n2, assign20150_e27739_d_n6, assign20150_e27739_d_n7, assign20150_e27739_d_n10, assign20150_e27739_d_n11, assign20150_e27739_d_n12, assign20150_e27739_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20150_e27733: f64 = (locals.var_t2).ln();
        let assign20150_e27735: f64 = (assign20150_e27733 / locals.var_t3);
        let assign20150_e27737: f64 = (assign20150_e27735 + p.p287);
        (assign20150_e27737, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign20150_e27733 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign20150_e27739;
        locals.var_ps0_inib_dn0 = assign20150_e27739_d_n0;
        locals.var_ps0_inib_dn2 = assign20150_e27739_d_n2;
        locals.var_ps0_inib_dn6 = assign20150_e27739_d_n6;
        locals.var_ps0_inib_dn7 = assign20150_e27739_d_n7;
        locals.var_ps0_inib_dn10 = assign20150_e27739_d_n10;
        locals.var_ps0_inib_dn11 = assign20150_e27739_d_n11;
        locals.var_ps0_inib_dn12 = assign20150_e27739_d_n12;
        locals.var_ps0_inib_dn17 = assign20150_e27739_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign20160_e27755, assign20160_e27755_d_n0, assign20160_e27755_d_n2, assign20160_e27755_d_n6, assign20160_e27755_d_n7, assign20160_e27755_d_n10, assign20160_e27755_d_n11, assign20160_e27755_d_n12, assign20160_e27755_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20160_e27751: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign20160_e27753: f64 = (assign20160_e27751 - 0.0008);
        (assign20160_e27753, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20160_e27755;
        locals.var_tmf1_dn0 = assign20160_e27755_d_n0;
        locals.var_tmf1_dn2 = assign20160_e27755_d_n2;
        locals.var_tmf1_dn6 = assign20160_e27755_d_n6;
        locals.var_tmf1_dn7 = assign20160_e27755_d_n7;
        locals.var_tmf1_dn10 = assign20160_e27755_d_n10;
        locals.var_tmf1_dn11 = assign20160_e27755_d_n11;
        locals.var_tmf1_dn12 = assign20160_e27755_d_n12;
        locals.var_tmf1_dn17 = assign20160_e27755_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20170_e27771, assign20170_e27771_d_n0, assign20170_e27771_d_n2, assign20170_e27771_d_n6, assign20170_e27771_d_n7, assign20170_e27771_d_n10, assign20170_e27771_d_n11, assign20170_e27771_d_n12, assign20170_e27771_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20170_e27767: f64 = (4.0 * locals.var_ps0_inib);
        let assign20170_e27769: f64 = (assign20170_e27767 * 0.0008);
        (assign20170_e27769, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20170_e27771;
        locals.var_tmf2_dn0 = assign20170_e27771_d_n0;
        locals.var_tmf2_dn2 = assign20170_e27771_d_n2;
        locals.var_tmf2_dn6 = assign20170_e27771_d_n6;
        locals.var_tmf2_dn7 = assign20170_e27771_d_n7;
        locals.var_tmf2_dn10 = assign20170_e27771_d_n10;
        locals.var_tmf2_dn11 = assign20170_e27771_d_n11;
        locals.var_tmf2_dn12 = assign20170_e27771_d_n12;
        locals.var_tmf2_dn17 = assign20170_e27771_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20180_e27789, assign20180_e27789_d_n0, assign20180_e27789_d_n2, assign20180_e27789_d_n6, assign20180_e27789_d_n7, assign20180_e27789_d_n10, assign20180_e27789_d_n11, assign20180_e27789_d_n12, assign20180_e27789_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let (assign20180_e27787, assign20180_e27787_d_n0, assign20180_e27787_d_n2, assign20180_e27787_d_n6, assign20180_e27787_d_n7, assign20180_e27787_d_n10, assign20180_e27787_d_n11, assign20180_e27787_d_n12, assign20180_e27787_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign20180_e27786: f64 = (-locals.var_tmf2);
                (assign20180_e27786, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign20180_e27787, assign20180_e27787_d_n0, assign20180_e27787_d_n2, assign20180_e27787_d_n6, assign20180_e27787_d_n7, assign20180_e27787_d_n10, assign20180_e27787_d_n11, assign20180_e27787_d_n12, assign20180_e27787_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20180_e27789;
        locals.var_tmf2_dn0 = assign20180_e27789_d_n0;
        locals.var_tmf2_dn2 = assign20180_e27789_d_n2;
        locals.var_tmf2_dn6 = assign20180_e27789_d_n6;
        locals.var_tmf2_dn7 = assign20180_e27789_d_n7;
        locals.var_tmf2_dn10 = assign20180_e27789_d_n10;
        locals.var_tmf2_dn11 = assign20180_e27789_d_n11;
        locals.var_tmf2_dn12 = assign20180_e27789_d_n12;
        locals.var_tmf2_dn17 = assign20180_e27789_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20190_e27806, assign20190_e27806_d_n0, assign20190_e27806_d_n2, assign20190_e27806_d_n6, assign20190_e27806_d_n7, assign20190_e27806_d_n10, assign20190_e27806_d_n11, assign20190_e27806_d_n12, assign20190_e27806_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20190_e27801: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20190_e27803: f64 = (assign20190_e27801 + locals.var_tmf2);
        let assign20190_e27804: f64 = (assign20190_e27803).sqrt();
        (assign20190_e27804, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20190_e27804)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20190_e27804)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20190_e27804)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20190_e27804)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20190_e27804)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20190_e27804)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20190_e27804)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign20190_e27804)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20190_e27806;
        locals.var_tmf2_dn0 = assign20190_e27806_d_n0;
        locals.var_tmf2_dn2 = assign20190_e27806_d_n2;
        locals.var_tmf2_dn6 = assign20190_e27806_d_n6;
        locals.var_tmf2_dn7 = assign20190_e27806_d_n7;
        locals.var_tmf2_dn10 = assign20190_e27806_d_n10;
        locals.var_tmf2_dn11 = assign20190_e27806_d_n11;
        locals.var_tmf2_dn12 = assign20190_e27806_d_n12;
        locals.var_tmf2_dn17 = assign20190_e27806_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20200_e27824, assign20200_e27824_d_n0, assign20200_e27824_d_n2, assign20200_e27824_d_n6, assign20200_e27824_d_n7, assign20200_e27824_d_n10, assign20200_e27824_d_n11, assign20200_e27824_d_n12, assign20200_e27824_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign20200_e27820: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20200_e27821: f64 = (0.5 * assign20200_e27820);
        let assign20200_e27822: f64 = (locals.var_ps0_inib - assign20200_e27821);
        (assign20200_e27822, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20200_e27824;
        locals.var_ps0_ini_dn0 = assign20200_e27824_d_n0;
        locals.var_ps0_ini_dn2 = assign20200_e27824_d_n2;
        locals.var_ps0_ini_dn6 = assign20200_e27824_d_n6;
        locals.var_ps0_ini_dn7 = assign20200_e27824_d_n7;
        locals.var_ps0_ini_dn10 = assign20200_e27824_d_n10;
        locals.var_ps0_ini_dn11 = assign20200_e27824_d_n11;
        locals.var_ps0_ini_dn12 = assign20200_e27824_d_n12;
        locals.var_ps0_ini_dn17 = assign20200_e27824_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20210_e27827: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign20210_e27827;
        locals.var_guard630_rv = 0.0;

        let assign20220_e27830: f64 = (locals.var_vgs - locals.var_shift);
        let assign20220_e27832: f64 = if assign20220_e27830 <= locals.var_vth__blk624 { 1.0 } else { 0.0 };
        locals.var_guard631 = assign20220_e27832;
        locals.var_guard631_rv = 0.0;

        let (assign20230_e27844, assign20230_e27844_d_n0, assign20230_e27844_d_n2, assign20230_e27844_d_n6, assign20230_e27844_d_n7, assign20230_e27844_d_n10, assign20230_e27844_d_n11, assign20230_e27844_d_n12, assign20230_e27844_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20230_e27842: f64 = (1.0 / locals.var_c_fox);
        (assign20230_e27842, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20230_e27844;
        locals.var_t0_dn0 = assign20230_e27844_d_n0;
        locals.var_t0_dn2 = assign20230_e27844_d_n2;
        locals.var_t0_dn6 = assign20230_e27844_d_n6;
        locals.var_t0_dn7 = assign20230_e27844_d_n7;
        locals.var_t0_dn10 = assign20230_e27844_d_n10;
        locals.var_t0_dn11 = assign20230_e27844_d_n11;
        locals.var_t0_dn12 = assign20230_e27844_d_n12;
        locals.var_t0_dn17 = assign20230_e27844_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20240_e27856, assign20240_e27856_d_n0, assign20240_e27856_d_n2, assign20240_e27856_d_n6, assign20240_e27856_d_n7, assign20240_e27856_d_n10, assign20240_e27856_d_n11, assign20240_e27856_d_n12, assign20240_e27856_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20240_e27854: f64 = (locals.var_t_soi__blk607 / 1.034943e-10);
        (assign20240_e27854, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20240_e27856;
        locals.var_t1_dn0 = assign20240_e27856_d_n0;
        locals.var_t1_dn2 = assign20240_e27856_d_n2;
        locals.var_t1_dn6 = assign20240_e27856_d_n6;
        locals.var_t1_dn7 = assign20240_e27856_d_n7;
        locals.var_t1_dn10 = assign20240_e27856_d_n10;
        locals.var_t1_dn11 = assign20240_e27856_d_n11;
        locals.var_t1_dn12 = assign20240_e27856_d_n12;
        locals.var_t1_dn17 = assign20240_e27856_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20250_e27868, assign20250_e27868_d_n0, assign20250_e27868_d_n2, assign20250_e27868_d_n6, assign20250_e27868_d_n7, assign20250_e27868_d_n10, assign20250_e27868_d_n11, assign20250_e27868_d_n12, assign20250_e27868_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20250_e27866: f64 = (1.0 / locals.var_c_box);
        (assign20250_e27866, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20250_e27868;
        locals.var_t2_dn0 = assign20250_e27868_d_n0;
        locals.var_t2_dn2 = assign20250_e27868_d_n2;
        locals.var_t2_dn6 = assign20250_e27868_d_n6;
        locals.var_t2_dn7 = assign20250_e27868_d_n7;
        locals.var_t2_dn10 = assign20250_e27868_d_n10;
        locals.var_t2_dn11 = assign20250_e27868_d_n11;
        locals.var_t2_dn12 = assign20250_e27868_d_n12;
        locals.var_t2_dn17 = assign20250_e27868_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20260_e27884, assign20260_e27884_d_n0, assign20260_e27884_d_n2, assign20260_e27884_d_n6, assign20260_e27884_d_n7, assign20260_e27884_d_n10, assign20260_e27884_d_n11, assign20260_e27884_d_n12, assign20260_e27884_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20260_e27879: f64 = (locals.var_t0 + locals.var_t1);
        let assign20260_e27881: f64 = (assign20260_e27879 + locals.var_t2);
        let assign20260_e27882: f64 = (1.0 / assign20260_e27881);
        (assign20260_e27882, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20260_e27881 * assign20260_e27881))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20260_e27881 * assign20260_e27881))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20260_e27881 * assign20260_e27881))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20260_e27881 * assign20260_e27881))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20260_e27881 * assign20260_e27881))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20260_e27881 * assign20260_e27881))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20260_e27881 * assign20260_e27881))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20260_e27881 * assign20260_e27881))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20260_e27884;
        locals.var_t3_dn0 = assign20260_e27884_d_n0;
        locals.var_t3_dn2 = assign20260_e27884_d_n2;
        locals.var_t3_dn6 = assign20260_e27884_d_n6;
        locals.var_t3_dn7 = assign20260_e27884_d_n7;
        locals.var_t3_dn10 = assign20260_e27884_d_n10;
        locals.var_t3_dn11 = assign20260_e27884_d_n11;
        locals.var_t3_dn12 = assign20260_e27884_d_n12;
        locals.var_t3_dn17 = assign20260_e27884_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20270_e27907, assign20270_e27907_d_n0, assign20270_e27907_d_n2, assign20270_e27907_d_n6, assign20270_e27907_d_n7, assign20270_e27907_d_n10, assign20270_e27907_d_n11, assign20270_e27907_d_n12, assign20270_e27907_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20270_e27895: f64 = (locals.var_vgp__blk608 - locals.var_vbsbiz);
        let assign20270_e27899: f64 = (0.5 * locals.var_t1);
        let assign20270_e27900: f64 = (locals.var_t2 + assign20270_e27899);
        let assign20270_e27902: f64 = (-locals.var_q_s0_dep_ini);
        let assign20270_e27903: f64 = (assign20270_e27900 * assign20270_e27902);
        let assign20270_e27904: f64 = (assign20270_e27895 + assign20270_e27903);
        let assign20270_e27905: f64 = (locals.var_t3 * assign20270_e27904);
        (assign20270_e27905, ((locals.var_t3_dn0 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20270_e27904) + (locals.var_t3 * ((locals.var_vgp__blk608_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20270_e27902) + (assign20270_e27900 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20270_e27907;
        locals.var_t4_dn0 = assign20270_e27907_d_n0;
        locals.var_t4_dn2 = assign20270_e27907_d_n2;
        locals.var_t4_dn6 = assign20270_e27907_d_n6;
        locals.var_t4_dn7 = assign20270_e27907_d_n7;
        locals.var_t4_dn10 = assign20270_e27907_d_n10;
        locals.var_t4_dn11 = assign20270_e27907_d_n11;
        locals.var_t4_dn12 = assign20270_e27907_d_n12;
        locals.var_t4_dn17 = assign20270_e27907_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20280_e27921, assign20280_e27921_d_n0, assign20280_e27921_d_n2, assign20280_e27921_d_n6, assign20280_e27921_d_n7, assign20280_e27921_d_n10, assign20280_e27921_d_n11, assign20280_e27921_d_n12, assign20280_e27921_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign20280_e27918: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20280_e27919: f64 = (locals.var_vgp__blk608 - assign20280_e27918);
        (assign20280_e27919, (locals.var_vgp__blk608_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20280_e27921;
        locals.var_ps0_inia_dn0 = assign20280_e27921_d_n0;
        locals.var_ps0_inia_dn2 = assign20280_e27921_d_n2;
        locals.var_ps0_inia_dn6 = assign20280_e27921_d_n6;
        locals.var_ps0_inia_dn7 = assign20280_e27921_d_n7;
        locals.var_ps0_inia_dn10 = assign20280_e27921_d_n10;
        locals.var_ps0_inia_dn11 = assign20280_e27921_d_n11;
        locals.var_ps0_inia_dn12 = assign20280_e27921_d_n12;
        locals.var_ps0_inia_dn17 = assign20280_e27921_d_n17;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20290_e27931, assign20290_e27931_d_n0, assign20290_e27931_d_n2, assign20290_e27931_d_n6, assign20290_e27931_d_n7, assign20290_e27931_d_n10, assign20290_e27931_d_n11, assign20290_e27931_d_n12, assign20290_e27931_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20290_e27931;
        locals.var_ps0_ini_dn0 = assign20290_e27931_d_n0;
        locals.var_ps0_ini_dn2 = assign20290_e27931_d_n2;
        locals.var_ps0_ini_dn6 = assign20290_e27931_d_n6;
        locals.var_ps0_ini_dn7 = assign20290_e27931_d_n7;
        locals.var_ps0_ini_dn10 = assign20290_e27931_d_n10;
        locals.var_ps0_ini_dn11 = assign20290_e27931_d_n11;
        locals.var_ps0_ini_dn12 = assign20290_e27931_d_n12;
        locals.var_ps0_ini_dn17 = assign20290_e27931_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20300_e27944, assign20300_e27944_d_n0, assign20300_e27944_d_n2, assign20300_e27944_d_n6, assign20300_e27944_d_n7, assign20300_e27944_d_n10, assign20300_e27944_d_n11, assign20300_e27944_d_n12, assign20300_e27944_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) {
        let assign20300_e27942: f64 = (1.0 / locals.var_c_fox);
        (assign20300_e27942, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn7 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn17 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign20300_e27944;
        locals.var_t0_dn0 = assign20300_e27944_d_n0;
        locals.var_t0_dn2 = assign20300_e27944_d_n2;
        locals.var_t0_dn6 = assign20300_e27944_d_n6;
        locals.var_t0_dn7 = assign20300_e27944_d_n7;
        locals.var_t0_dn10 = assign20300_e27944_d_n10;
        locals.var_t0_dn11 = assign20300_e27944_d_n11;
        locals.var_t0_dn12 = assign20300_e27944_d_n12;
        locals.var_t0_dn17 = assign20300_e27944_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign20310_e27957, assign20310_e27957_d_n0, assign20310_e27957_d_n2, assign20310_e27957_d_n6, assign20310_e27957_d_n7, assign20310_e27957_d_n10, assign20310_e27957_d_n11, assign20310_e27957_d_n12, assign20310_e27957_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) {
        let assign20310_e27955: f64 = (locals.var_t_soi__blk607 / 1.034943e-10);
        (assign20310_e27955, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20310_e27957;
        locals.var_t1_dn0 = assign20310_e27957_d_n0;
        locals.var_t1_dn2 = assign20310_e27957_d_n2;
        locals.var_t1_dn6 = assign20310_e27957_d_n6;
        locals.var_t1_dn7 = assign20310_e27957_d_n7;
        locals.var_t1_dn10 = assign20310_e27957_d_n10;
        locals.var_t1_dn11 = assign20310_e27957_d_n11;
        locals.var_t1_dn12 = assign20310_e27957_d_n12;
        locals.var_t1_dn17 = assign20310_e27957_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20320_e27970, assign20320_e27970_d_n0, assign20320_e27970_d_n2, assign20320_e27970_d_n6, assign20320_e27970_d_n7, assign20320_e27970_d_n10, assign20320_e27970_d_n11, assign20320_e27970_d_n12, assign20320_e27970_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) {
        let assign20320_e27968: f64 = (1.0 / locals.var_c_box);
        (assign20320_e27968, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20320_e27970;
        locals.var_t2_dn0 = assign20320_e27970_d_n0;
        locals.var_t2_dn2 = assign20320_e27970_d_n2;
        locals.var_t2_dn6 = assign20320_e27970_d_n6;
        locals.var_t2_dn7 = assign20320_e27970_d_n7;
        locals.var_t2_dn10 = assign20320_e27970_d_n10;
        locals.var_t2_dn11 = assign20320_e27970_d_n11;
        locals.var_t2_dn12 = assign20320_e27970_d_n12;
        locals.var_t2_dn17 = assign20320_e27970_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20330_e27987, assign20330_e27987_d_n0, assign20330_e27987_d_n2, assign20330_e27987_d_n6, assign20330_e27987_d_n7, assign20330_e27987_d_n10, assign20330_e27987_d_n11, assign20330_e27987_d_n12, assign20330_e27987_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) {
        let assign20330_e27982: f64 = (locals.var_t0 + locals.var_t1);
        let assign20330_e27984: f64 = (assign20330_e27982 + locals.var_t2);
        let assign20330_e27985: f64 = (1.0 / assign20330_e27984);
        (assign20330_e27985, (-(((locals.var_t0_dn0 + locals.var_t1_dn0) + locals.var_t2_dn0) / (assign20330_e27984 * assign20330_e27984))), (-(((locals.var_t0_dn2 + locals.var_t1_dn2) + locals.var_t2_dn2) / (assign20330_e27984 * assign20330_e27984))), (-(((locals.var_t0_dn6 + locals.var_t1_dn6) + locals.var_t2_dn6) / (assign20330_e27984 * assign20330_e27984))), (-(((locals.var_t0_dn7 + locals.var_t1_dn7) + locals.var_t2_dn7) / (assign20330_e27984 * assign20330_e27984))), (-(((locals.var_t0_dn10 + locals.var_t1_dn10) + locals.var_t2_dn10) / (assign20330_e27984 * assign20330_e27984))), (-(((locals.var_t0_dn11 + locals.var_t1_dn11) + locals.var_t2_dn11) / (assign20330_e27984 * assign20330_e27984))), (-(((locals.var_t0_dn12 + locals.var_t1_dn12) + locals.var_t2_dn12) / (assign20330_e27984 * assign20330_e27984))), (-(((locals.var_t0_dn17 + locals.var_t1_dn17) + locals.var_t2_dn17) / (assign20330_e27984 * assign20330_e27984))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20330_e27987;
        locals.var_t3_dn0 = assign20330_e27987_d_n0;
        locals.var_t3_dn2 = assign20330_e27987_d_n2;
        locals.var_t3_dn6 = assign20330_e27987_d_n6;
        locals.var_t3_dn7 = assign20330_e27987_d_n7;
        locals.var_t3_dn10 = assign20330_e27987_d_n10;
        locals.var_t3_dn11 = assign20330_e27987_d_n11;
        locals.var_t3_dn12 = assign20330_e27987_d_n12;
        locals.var_t3_dn17 = assign20330_e27987_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20340_e28011, assign20340_e28011_d_n0, assign20340_e28011_d_n2, assign20340_e28011_d_n6, assign20340_e28011_d_n7, assign20340_e28011_d_n10, assign20340_e28011_d_n11, assign20340_e28011_d_n12, assign20340_e28011_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) {
        let assign20340_e27999: f64 = (locals.var_vgp__blk608 - locals.var_vbsbiz);
        let assign20340_e28003: f64 = (0.5 * locals.var_t1);
        let assign20340_e28004: f64 = (locals.var_t2 + assign20340_e28003);
        let assign20340_e28006: f64 = (-locals.var_q_s0_dep_ini);
        let assign20340_e28007: f64 = (assign20340_e28004 * assign20340_e28006);
        let assign20340_e28008: f64 = (assign20340_e27999 + assign20340_e28007);
        let assign20340_e28009: f64 = (locals.var_t3 * assign20340_e28008);
        (assign20340_e28009, ((locals.var_t3_dn0 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn0 - locals.var_vbsbiz_dn0) + (((locals.var_t2_dn0 + (0.5 * locals.var_t1_dn0)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn0)))))), ((locals.var_t3_dn2 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn2 - locals.var_vbsbiz_dn2) + (((locals.var_t2_dn2 + (0.5 * locals.var_t1_dn2)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn2)))))), ((locals.var_t3_dn6 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn6 - locals.var_vbsbiz_dn6) + (((locals.var_t2_dn6 + (0.5 * locals.var_t1_dn6)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn6)))))), ((locals.var_t3_dn7 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn7 - locals.var_vbsbiz_dn7) + (((locals.var_t2_dn7 + (0.5 * locals.var_t1_dn7)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn7)))))), ((locals.var_t3_dn10 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn10 - locals.var_vbsbiz_dn10) + (((locals.var_t2_dn10 + (0.5 * locals.var_t1_dn10)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn10)))))), ((locals.var_t3_dn11 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn11 - locals.var_vbsbiz_dn11) + (((locals.var_t2_dn11 + (0.5 * locals.var_t1_dn11)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn11)))))), ((locals.var_t3_dn12 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn12 - locals.var_vbsbiz_dn12) + (((locals.var_t2_dn12 + (0.5 * locals.var_t1_dn12)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn12)))))), ((locals.var_t3_dn17 * assign20340_e28008) + (locals.var_t3 * ((locals.var_vgp__blk608_dn17 - locals.var_vbsbiz_dn17) + (((locals.var_t2_dn17 + (0.5 * locals.var_t1_dn17)) * assign20340_e28006) + (assign20340_e28004 * (-locals.var_q_s0_dep_ini_dn17)))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign20340_e28011;
        locals.var_t4_dn0 = assign20340_e28011_d_n0;
        locals.var_t4_dn2 = assign20340_e28011_d_n2;
        locals.var_t4_dn6 = assign20340_e28011_d_n6;
        locals.var_t4_dn7 = assign20340_e28011_d_n7;
        locals.var_t4_dn10 = assign20340_e28011_d_n10;
        locals.var_t4_dn11 = assign20340_e28011_d_n11;
        locals.var_t4_dn12 = assign20340_e28011_d_n12;
        locals.var_t4_dn17 = assign20340_e28011_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign20350_e28026, assign20350_e28026_d_n0, assign20350_e28026_d_n2, assign20350_e28026_d_n6, assign20350_e28026_d_n7, assign20350_e28026_d_n10, assign20350_e28026_d_n11, assign20350_e28026_d_n12, assign20350_e28026_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) {
        let assign20350_e28023: f64 = (locals.var_t4 / locals.var_c_fox);
        let assign20350_e28024: f64 = (locals.var_vgp__blk608 - assign20350_e28023);
        (assign20350_e28024, (locals.var_vgp__blk608_dn0 - (((locals.var_t4_dn0 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn0)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn2 - (((locals.var_t4_dn2 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn2)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn6 - (((locals.var_t4_dn6 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn6)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn7 - (((locals.var_t4_dn7 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn7)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn10 - (((locals.var_t4_dn10 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn10)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn11 - (((locals.var_t4_dn11 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn11)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn12 - (((locals.var_t4_dn12 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn12)) / (locals.var_c_fox * locals.var_c_fox))), (locals.var_vgp__blk608_dn17 - (((locals.var_t4_dn17 * locals.var_c_fox) - (locals.var_t4 * locals.var_c_fox_dn17)) / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign20350_e28026;
        locals.var_ps0_inia_dn0 = assign20350_e28026_d_n0;
        locals.var_ps0_inia_dn2 = assign20350_e28026_d_n2;
        locals.var_ps0_inia_dn6 = assign20350_e28026_d_n6;
        locals.var_ps0_inia_dn7 = assign20350_e28026_d_n7;
        locals.var_ps0_inia_dn10 = assign20350_e28026_d_n10;
        locals.var_ps0_inia_dn11 = assign20350_e28026_d_n11;
        locals.var_ps0_inia_dn12 = assign20350_e28026_d_n12;
        locals.var_ps0_inia_dn17 = assign20350_e28026_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign20360_e28037, assign20360_e28037_d_n0, assign20360_e28037_d_n2, assign20360_e28037_d_n6, assign20360_e28037_d_n7, assign20360_e28037_d_n10, assign20360_e28037_d_n11, assign20360_e28037_d_n12, assign20360_e28037_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20360_e28037;
        locals.var_ps0_ini_dn0 = assign20360_e28037_d_n0;
        locals.var_ps0_ini_dn2 = assign20360_e28037_d_n2;
        locals.var_ps0_ini_dn6 = assign20360_e28037_d_n6;
        locals.var_ps0_ini_dn7 = assign20360_e28037_d_n7;
        locals.var_ps0_ini_dn10 = assign20360_e28037_d_n10;
        locals.var_ps0_ini_dn11 = assign20360_e28037_d_n11;
        locals.var_ps0_ini_dn12 = assign20360_e28037_d_n12;
        locals.var_ps0_ini_dn17 = assign20360_e28037_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign20370_e28040: f64 = (locals.var_vgp__blk608 - locals.var_shift);
        let assign20370_e28042: f64 = if assign20370_e28040 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign20370_e28042;
        locals.var_guard632_rv = 0.0;

        let (assign20380_e28059, assign20380_e28059_d_n0, assign20380_e28059_d_n2, assign20380_e28059_d_n6, assign20380_e28059_d_n7, assign20380_e28059_d_n10, assign20380_e28059_d_n11, assign20380_e28059_d_n12, assign20380_e28059_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) {
        let assign20380_e28055: f64 = (1.0 / locals.var_cnst1soi);
        let assign20380_e28057: f64 = (assign20380_e28055 / locals.var_cnstc_foxi);
        (assign20380_e28057, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign20380_e28055 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20380_e28059;
        locals.var_t1_dn0 = assign20380_e28059_d_n0;
        locals.var_t1_dn2 = assign20380_e28059_d_n2;
        locals.var_t1_dn6 = assign20380_e28059_d_n6;
        locals.var_t1_dn7 = assign20380_e28059_d_n7;
        locals.var_t1_dn10 = assign20380_e28059_d_n10;
        locals.var_t1_dn11 = assign20380_e28059_d_n11;
        locals.var_t1_dn12 = assign20380_e28059_d_n12;
        locals.var_t1_dn17 = assign20380_e28059_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign20390_e28080, assign20390_e28080_d_n0, assign20390_e28080_d_n2, assign20390_e28080_d_n6, assign20390_e28080_d_n7, assign20390_e28080_d_n10, assign20390_e28080_d_n11, assign20390_e28080_d_n12, assign20390_e28080_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) {
        let assign20390_e28073: f64 = (locals.var_vgp__blk608 - locals.var_shift);
        let assign20390_e28074: f64 = (locals.var_t1 * assign20390_e28073);
        let assign20390_e28077: f64 = (locals.var_vgp__blk608 - locals.var_shift);
        let assign20390_e28078: f64 = (assign20390_e28074 * assign20390_e28077);
        (assign20390_e28078, ((((locals.var_t1_dn0 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn0 - locals.var_shift_dn0))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn2 - locals.var_shift_dn2))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn6 - locals.var_shift_dn6))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn7 - locals.var_shift_dn7))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn10 - locals.var_shift_dn10))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn11 - locals.var_shift_dn11))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn12 - locals.var_shift_dn12))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign20390_e28073) + (locals.var_t1 * (locals.var_vgp__blk608_dn17 - locals.var_shift_dn17))) * assign20390_e28077) + (assign20390_e28074 * (locals.var_vgp__blk608_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign20390_e28080;
        locals.var_t2_dn0 = assign20390_e28080_d_n0;
        locals.var_t2_dn2 = assign20390_e28080_d_n2;
        locals.var_t2_dn6 = assign20390_e28080_d_n6;
        locals.var_t2_dn7 = assign20390_e28080_d_n7;
        locals.var_t2_dn10 = assign20390_e28080_d_n10;
        locals.var_t2_dn11 = assign20390_e28080_d_n11;
        locals.var_t2_dn12 = assign20390_e28080_d_n12;
        locals.var_t2_dn17 = assign20390_e28080_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign20400_e28099, assign20400_e28099_d_n0, assign20400_e28099_d_n2, assign20400_e28099_d_n6, assign20400_e28099_d_n7, assign20400_e28099_d_n10, assign20400_e28099_d_n11, assign20400_e28099_d_n12, assign20400_e28099_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) {
        let assign20400_e28095: f64 = (locals.var_vgp__blk608 - locals.var_shift);
        let assign20400_e28096: f64 = (2.0 / assign20400_e28095);
        let assign20400_e28097: f64 = (locals.var_beta + assign20400_e28096);
        (assign20400_e28097, (-((2.0 * (locals.var_vgp__blk608_dn0 - locals.var_shift_dn0)) / (assign20400_e28095 * assign20400_e28095))), (-((2.0 * (locals.var_vgp__blk608_dn2 - locals.var_shift_dn2)) / (assign20400_e28095 * assign20400_e28095))), (-((2.0 * (locals.var_vgp__blk608_dn6 - locals.var_shift_dn6)) / (assign20400_e28095 * assign20400_e28095))), (-((2.0 * (locals.var_vgp__blk608_dn7 - locals.var_shift_dn7)) / (assign20400_e28095 * assign20400_e28095))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp__blk608_dn10 - locals.var_shift_dn10)) / (assign20400_e28095 * assign20400_e28095)))), (-((2.0 * (locals.var_vgp__blk608_dn11 - locals.var_shift_dn11)) / (assign20400_e28095 * assign20400_e28095))), (-((2.0 * (locals.var_vgp__blk608_dn12 - locals.var_shift_dn12)) / (assign20400_e28095 * assign20400_e28095))), (-((2.0 * (locals.var_vgp__blk608_dn17 - locals.var_shift_dn17)) / (assign20400_e28095 * assign20400_e28095))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign20400_e28099;
        locals.var_t3_dn0 = assign20400_e28099_d_n0;
        locals.var_t3_dn2 = assign20400_e28099_d_n2;
        locals.var_t3_dn6 = assign20400_e28099_d_n6;
        locals.var_t3_dn7 = assign20400_e28099_d_n7;
        locals.var_t3_dn10 = assign20400_e28099_d_n10;
        locals.var_t3_dn11 = assign20400_e28099_d_n11;
        locals.var_t3_dn12 = assign20400_e28099_d_n12;
        locals.var_t3_dn17 = assign20400_e28099_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign20410_e28117, assign20410_e28117_d_n0, assign20410_e28117_d_n2, assign20410_e28117_d_n6, assign20410_e28117_d_n7, assign20410_e28117_d_n10, assign20410_e28117_d_n11, assign20410_e28117_d_n12, assign20410_e28117_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) {
        let assign20410_e28111: f64 = (locals.var_t2).ln();
        let assign20410_e28113: f64 = (assign20410_e28111 / locals.var_t3);
        let assign20410_e28115: f64 = (assign20410_e28113 + p.p287);
        (assign20410_e28115, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign20410_e28111 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign20410_e28117;
        locals.var_ps0_inib_dn0 = assign20410_e28117_d_n0;
        locals.var_ps0_inib_dn2 = assign20410_e28117_d_n2;
        locals.var_ps0_inib_dn6 = assign20410_e28117_d_n6;
        locals.var_ps0_inib_dn7 = assign20410_e28117_d_n7;
        locals.var_ps0_inib_dn10 = assign20410_e28117_d_n10;
        locals.var_ps0_inib_dn11 = assign20410_e28117_d_n11;
        locals.var_ps0_inib_dn12 = assign20410_e28117_d_n12;
        locals.var_ps0_inib_dn17 = assign20410_e28117_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let assign20420_e28121: f64 = (locals.var_ps0_inib * 0.98);
        let assign20420_e28123: f64 = (assign20420_e28121 - 0.4);
        let assign20420_e28128: f64 = if ((locals.var_ps0_inia > assign20420_e28123) && (0.4 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard633 = assign20420_e28128;
        locals.var_guard633_rv = 0.0;

        let (assign20430_e28149, assign20430_e28149_d_n0, assign20430_e28149_d_n2, assign20430_e28149_d_n6, assign20430_e28149_d_n7, assign20430_e28149_d_n10, assign20430_e28149_d_n11, assign20430_e28149_d_n12, assign20430_e28149_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20430_e28144: f64 = (locals.var_ps0_inib * 0.98);
        let assign20430_e28145: f64 = (locals.var_ps0_inia - assign20430_e28144);
        let assign20430_e28147: f64 = (assign20430_e28145 + 0.4);
        (assign20430_e28147, (locals.var_ps0_inia_dn0 - (locals.var_ps0_inib_dn0 * 0.98)), (locals.var_ps0_inia_dn2 - (locals.var_ps0_inib_dn2 * 0.98)), (locals.var_ps0_inia_dn6 - (locals.var_ps0_inib_dn6 * 0.98)), (locals.var_ps0_inia_dn7 - (locals.var_ps0_inib_dn7 * 0.98)), (locals.var_ps0_inia_dn10 - (locals.var_ps0_inib_dn10 * 0.98)), (locals.var_ps0_inia_dn11 - (locals.var_ps0_inib_dn11 * 0.98)), (locals.var_ps0_inia_dn12 - (locals.var_ps0_inib_dn12 * 0.98)), (locals.var_ps0_inia_dn17 - (locals.var_ps0_inib_dn17 * 0.98)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20430_e28149;
        locals.var_tmf1_dn0 = assign20430_e28149_d_n0;
        locals.var_tmf1_dn2 = assign20430_e28149_d_n2;
        locals.var_tmf1_dn6 = assign20430_e28149_d_n6;
        locals.var_tmf1_dn7 = assign20430_e28149_d_n7;
        locals.var_tmf1_dn10 = assign20430_e28149_d_n10;
        locals.var_tmf1_dn11 = assign20430_e28149_d_n11;
        locals.var_tmf1_dn12 = assign20430_e28149_d_n12;
        locals.var_tmf1_dn17 = assign20430_e28149_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20440_e28166, assign20440_e28166_d_n0, assign20440_e28166_d_n2, assign20440_e28166_d_n6, assign20440_e28166_d_n7, assign20440_e28166_d_n10, assign20440_e28166_d_n11, assign20440_e28166_d_n12, assign20440_e28166_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20440_e28164: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign20440_e28164, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign20440_e28166;
        locals.var_x2_dn0 = assign20440_e28166_d_n0;
        locals.var_x2_dn2 = assign20440_e28166_d_n2;
        locals.var_x2_dn6 = assign20440_e28166_d_n6;
        locals.var_x2_dn7 = assign20440_e28166_d_n7;
        locals.var_x2_dn10 = assign20440_e28166_d_n10;
        locals.var_x2_dn11 = assign20440_e28166_d_n11;
        locals.var_x2_dn12 = assign20440_e28166_d_n12;
        locals.var_x2_dn17 = assign20440_e28166_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign20450_e28183, assign20450_e28183_d_n0, assign20450_e28183_d_n2, assign20450_e28183_d_n6, assign20450_e28183_d_n7, assign20450_e28183_d_n10, assign20450_e28183_d_n11, assign20450_e28183_d_n12, assign20450_e28183_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20450_e28181: f64 = (0.4 * 0.4);
        (assign20450_e28181, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign20450_e28183;
        locals.var_xmax2_dn0 = assign20450_e28183_d_n0;
        locals.var_xmax2_dn2 = assign20450_e28183_d_n2;
        locals.var_xmax2_dn6 = assign20450_e28183_d_n6;
        locals.var_xmax2_dn7 = assign20450_e28183_d_n7;
        locals.var_xmax2_dn10 = assign20450_e28183_d_n10;
        locals.var_xmax2_dn11 = assign20450_e28183_d_n11;
        locals.var_xmax2_dn12 = assign20450_e28183_d_n12;
        locals.var_xmax2_dn17 = assign20450_e28183_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign20460_e28198, assign20460_e28198_d_n0, assign20460_e28198_d_n2, assign20460_e28198_d_n6, assign20460_e28198_d_n7, assign20460_e28198_d_n10, assign20460_e28198_d_n11, assign20460_e28198_d_n12, assign20460_e28198_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20460_e28198;
        locals.var_xp_dn0 = assign20460_e28198_d_n0;
        locals.var_xp_dn2 = assign20460_e28198_d_n2;
        locals.var_xp_dn6 = assign20460_e28198_d_n6;
        locals.var_xp_dn7 = assign20460_e28198_d_n7;
        locals.var_xp_dn10 = assign20460_e28198_d_n10;
        locals.var_xp_dn11 = assign20460_e28198_d_n11;
        locals.var_xp_dn12 = assign20460_e28198_d_n12;
        locals.var_xp_dn17 = assign20460_e28198_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20470_e28213, assign20470_e28213_d_n0, assign20470_e28213_d_n2, assign20470_e28213_d_n6, assign20470_e28213_d_n7, assign20470_e28213_d_n10, assign20470_e28213_d_n11, assign20470_e28213_d_n12, assign20470_e28213_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20470_e28213;
        locals.var_xmp_dn0 = assign20470_e28213_d_n0;
        locals.var_xmp_dn2 = assign20470_e28213_d_n2;
        locals.var_xmp_dn6 = assign20470_e28213_d_n6;
        locals.var_xmp_dn7 = assign20470_e28213_d_n7;
        locals.var_xmp_dn10 = assign20470_e28213_d_n10;
        locals.var_xmp_dn11 = assign20470_e28213_d_n11;
        locals.var_xmp_dn12 = assign20470_e28213_d_n12;
        locals.var_xmp_dn17 = assign20470_e28213_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20480_e28228,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign20480_e28228;
        locals.var_m0_rv = 0.0;

        let (assign20490_e28243,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20490_e28243;
        locals.var_mm_rv = 0.0;

        let (assign20500_e28258, assign20500_e28258_d_n0, assign20500_e28258_d_n2, assign20500_e28258_d_n6, assign20500_e28258_d_n7, assign20500_e28258_d_n10, assign20500_e28258_d_n11, assign20500_e28258_d_n12, assign20500_e28258_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign20500_e28258;
        locals.var_arg_dn0 = assign20500_e28258_d_n0;
        locals.var_arg_dn2 = assign20500_e28258_d_n2;
        locals.var_arg_dn6 = assign20500_e28258_d_n6;
        locals.var_arg_dn7 = assign20500_e28258_d_n7;
        locals.var_arg_dn10 = assign20500_e28258_d_n10;
        locals.var_arg_dn11 = assign20500_e28258_d_n11;
        locals.var_arg_dn12 = assign20500_e28258_d_n12;
        locals.var_arg_dn17 = assign20500_e28258_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign20510_e28273, assign20510_e28273_d_n0, assign20510_e28273_d_n2, assign20510_e28273_d_n6, assign20510_e28273_d_n7, assign20510_e28273_d_n10, assign20510_e28273_d_n11, assign20510_e28273_d_n12, assign20510_e28273_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20510_e28273;
        locals.var_dnm_dn0 = assign20510_e28273_d_n0;
        locals.var_dnm_dn2 = assign20510_e28273_d_n2;
        locals.var_dnm_dn6 = assign20510_e28273_d_n6;
        locals.var_dnm_dn7 = assign20510_e28273_d_n7;
        locals.var_dnm_dn10 = assign20510_e28273_d_n10;
        locals.var_dnm_dn11 = assign20510_e28273_d_n11;
        locals.var_dnm_dn12 = assign20510_e28273_d_n12;
        locals.var_dnm_dn17 = assign20510_e28273_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20520_e28290, assign20520_e28290_d_n0, assign20520_e28290_d_n2, assign20520_e28290_d_n6, assign20520_e28290_d_n7, assign20520_e28290_d_n10, assign20520_e28290_d_n11, assign20520_e28290_d_n12, assign20520_e28290_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20520_e28288: f64 = (locals.var_xp * locals.var_x2);
        (assign20520_e28288, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20520_e28290;
        locals.var_xp_dn0 = assign20520_e28290_d_n0;
        locals.var_xp_dn2 = assign20520_e28290_d_n2;
        locals.var_xp_dn6 = assign20520_e28290_d_n6;
        locals.var_xp_dn7 = assign20520_e28290_d_n7;
        locals.var_xp_dn10 = assign20520_e28290_d_n10;
        locals.var_xp_dn11 = assign20520_e28290_d_n11;
        locals.var_xp_dn12 = assign20520_e28290_d_n12;
        locals.var_xp_dn17 = assign20520_e28290_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20530_e28307, assign20530_e28307_d_n0, assign20530_e28307_d_n2, assign20530_e28307_d_n6, assign20530_e28307_d_n7, assign20530_e28307_d_n10, assign20530_e28307_d_n11, assign20530_e28307_d_n12, assign20530_e28307_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20530_e28305: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign20530_e28305, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20530_e28307;
        locals.var_xmp_dn0 = assign20530_e28307_d_n0;
        locals.var_xmp_dn2 = assign20530_e28307_d_n2;
        locals.var_xmp_dn6 = assign20530_e28307_d_n6;
        locals.var_xmp_dn7 = assign20530_e28307_d_n7;
        locals.var_xmp_dn10 = assign20530_e28307_d_n10;
        locals.var_xmp_dn11 = assign20530_e28307_d_n11;
        locals.var_xmp_dn12 = assign20530_e28307_d_n12;
        locals.var_xmp_dn17 = assign20530_e28307_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20540_e28324, assign20540_e28324_d_n0, assign20540_e28324_d_n2, assign20540_e28324_d_n6, assign20540_e28324_d_n7, assign20540_e28324_d_n10, assign20540_e28324_d_n11, assign20540_e28324_d_n12, assign20540_e28324_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20540_e28322: f64 = (locals.var_xp * locals.var_x2);
        (assign20540_e28322, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign20540_e28324;
        locals.var_xp_dn0 = assign20540_e28324_d_n0;
        locals.var_xp_dn2 = assign20540_e28324_d_n2;
        locals.var_xp_dn6 = assign20540_e28324_d_n6;
        locals.var_xp_dn7 = assign20540_e28324_d_n7;
        locals.var_xp_dn10 = assign20540_e28324_d_n10;
        locals.var_xp_dn11 = assign20540_e28324_d_n11;
        locals.var_xp_dn12 = assign20540_e28324_d_n12;
        locals.var_xp_dn17 = assign20540_e28324_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign20550_e28341, assign20550_e28341_d_n0, assign20550_e28341_d_n2, assign20550_e28341_d_n6, assign20550_e28341_d_n7, assign20550_e28341_d_n10, assign20550_e28341_d_n11, assign20550_e28341_d_n12, assign20550_e28341_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20550_e28339: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign20550_e28339, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign20550_e28341;
        locals.var_xmp_dn0 = assign20550_e28341_d_n0;
        locals.var_xmp_dn2 = assign20550_e28341_d_n2;
        locals.var_xmp_dn6 = assign20550_e28341_d_n6;
        locals.var_xmp_dn7 = assign20550_e28341_d_n7;
        locals.var_xmp_dn10 = assign20550_e28341_d_n10;
        locals.var_xmp_dn11 = assign20550_e28341_d_n11;
        locals.var_xmp_dn12 = assign20550_e28341_d_n12;
        locals.var_xmp_dn17 = assign20550_e28341_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign20560_e28358, assign20560_e28358_d_n0, assign20560_e28358_d_n2, assign20560_e28358_d_n6, assign20560_e28358_d_n7, assign20560_e28358_d_n10, assign20560_e28358_d_n11, assign20560_e28358_d_n12, assign20560_e28358_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20560_e28356: f64 = (locals.var_xp + locals.var_xmp);
        (assign20560_e28356, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign20560_e28358;
        locals.var_arg_dn0 = assign20560_e28358_d_n0;
        locals.var_arg_dn2 = assign20560_e28358_d_n2;
        locals.var_arg_dn6 = assign20560_e28358_d_n6;
        locals.var_arg_dn7 = assign20560_e28358_d_n7;
        locals.var_arg_dn10 = assign20560_e28358_d_n10;
        locals.var_arg_dn11 = assign20560_e28358_d_n11;
        locals.var_arg_dn12 = assign20560_e28358_d_n12;
        locals.var_arg_dn17 = assign20560_e28358_d_n17;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20570_e28373, assign20570_e28373_d_n0, assign20570_e28373_d_n2, assign20570_e28373_d_n6, assign20570_e28373_d_n7, assign20570_e28373_d_n10, assign20570_e28373_d_n11, assign20570_e28373_d_n12, assign20570_e28373_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20570_e28373;
        locals.var_dnm_dn0 = assign20570_e28373_d_n0;
        locals.var_dnm_dn2 = assign20570_e28373_d_n2;
        locals.var_dnm_dn6 = assign20570_e28373_d_n6;
        locals.var_dnm_dn7 = assign20570_e28373_d_n7;
        locals.var_dnm_dn10 = assign20570_e28373_d_n10;
        locals.var_dnm_dn11 = assign20570_e28373_d_n11;
        locals.var_dnm_dn12 = assign20570_e28373_d_n12;
        locals.var_dnm_dn17 = assign20570_e28373_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign20580_e28388: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard634 = assign20580_e28388;
        locals.var_guard634_rv = 0.0;

        let assign20590_e28391: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign20590_e28391;
        locals.var_guard635_rv = 0.0;

        let (assign20600_e28410,) = {
    if ((((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20600_e28410;
        locals.var_mm_rv = 0.0;

        let assign20610_e28413: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign20610_e28413;
        locals.var_guard636_rv = 0.0;

        let (assign20620_e28435,) = {
    if (((((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20620_e28435;
        locals.var_mm_rv = 0.0;

        let assign20630_e28438: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign20630_e28438;
        locals.var_guard637_rv = 0.0;

        let (assign20640_e28463,) = {
    if ((((((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20640_e28463;
        locals.var_mm_rv = 0.0;

        let assign20650_e28466: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign20650_e28466;
        locals.var_guard638_rv = 0.0;

        let (assign20660_e28494,) = {
    if (((((((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign20660_e28494;
        locals.var_mm_rv = 0.0;

        let (assign20670_e28511,) = {
    if (((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign20670_e28511;
        locals.var_m0_rv = 0.0;

        let mut assign20680_loop_guard: usize = 0;
        while {
            let assign20680_cond_e28529: f64 = if ((((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign20680_cond_e28529 != 0.0
        } {
            assign20680_loop_guard += 1;
            assert!(assign20680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign20680_body0_e28547, assign20680_body0_e28547_d_n0, assign20680_body0_e28547_d_n2, assign20680_body0_e28547_d_n6, assign20680_body0_e28547_d_n7, assign20680_body0_e28547_d_n10, assign20680_body0_e28547_d_n11, assign20680_body0_e28547_d_n12, assign20680_body0_e28547_d_n17,) = {
    if (((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign20680_body0_e28545: f64 = (locals.var_dnm).sqrt();
        (assign20680_body0_e28545, (locals.var_dnm_dn0 / (2.0 * assign20680_body0_e28545)), (locals.var_dnm_dn2 / (2.0 * assign20680_body0_e28545)), (locals.var_dnm_dn6 / (2.0 * assign20680_body0_e28545)), (locals.var_dnm_dn7 / (2.0 * assign20680_body0_e28545)), (locals.var_dnm_dn10 / (2.0 * assign20680_body0_e28545)), (locals.var_dnm_dn11 / (2.0 * assign20680_body0_e28545)), (locals.var_dnm_dn12 / (2.0 * assign20680_body0_e28545)), (locals.var_dnm_dn17 / (2.0 * assign20680_body0_e28545)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign20680_body0_e28547;
            locals.var_dnm_dn0 = assign20680_body0_e28547_d_n0;
            locals.var_dnm_dn2 = assign20680_body0_e28547_d_n2;
            locals.var_dnm_dn6 = assign20680_body0_e28547_d_n6;
            locals.var_dnm_dn7 = assign20680_body0_e28547_d_n7;
            locals.var_dnm_dn10 = assign20680_body0_e28547_d_n10;
            locals.var_dnm_dn11 = assign20680_body0_e28547_d_n11;
            locals.var_dnm_dn12 = assign20680_body0_e28547_d_n12;
            locals.var_dnm_dn17 = assign20680_body0_e28547_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign20680_body1_e28566,) = {
    if (((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign20680_body1_e28564: f64 = (locals.var_m0 + 1.0);
        (assign20680_body1_e28564,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign20680_body1_e28566;
            locals.var_m0_rv = 0.0;
        }

        let (assign20690_e28590, assign20690_e28590_d_n0, assign20690_e28590_d_n2, assign20690_e28590_d_n6, assign20690_e28590_d_n7, assign20690_e28590_d_n10, assign20690_e28590_d_n11, assign20690_e28590_d_n12, assign20690_e28590_d_n17,) = {
    if (((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 == 0.0)) {
        let assign20690_e28586: f64 = (2.0 * 2.0);
        let assign20690_e28587: f64 = (1.0 / assign20690_e28586);
        let assign20690_e28588: f64 = (locals.var_dnm).powf(assign20690_e28587);
        (assign20690_e28588, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn0)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn2)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn6)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn7)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn10)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn11)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn12)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign20690_e28587) as f64).is_finite() && ((assign20690_e28587) as f64).fract() == 0.0 { if assign20690_e28587 == 0.0 { 0.0 } else { (assign20690_e28587 * ((locals.var_dnm).powf(assign20690_e28587 - 1.0) * locals.var_dnm_dn17)) } } else { (assign20690_e28588 * (assign20690_e28587 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20690_e28590;
        locals.var_dnm_dn0 = assign20690_e28590_d_n0;
        locals.var_dnm_dn2 = assign20690_e28590_d_n2;
        locals.var_dnm_dn6 = assign20690_e28590_d_n6;
        locals.var_dnm_dn7 = assign20690_e28590_d_n7;
        locals.var_dnm_dn10 = assign20690_e28590_d_n10;
        locals.var_dnm_dn11 = assign20690_e28590_d_n11;
        locals.var_dnm_dn12 = assign20690_e28590_d_n12;
        locals.var_dnm_dn17 = assign20690_e28590_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20700_e28607, assign20700_e28607_d_n0, assign20700_e28607_d_n2, assign20700_e28607_d_n6, assign20700_e28607_d_n7, assign20700_e28607_d_n10, assign20700_e28607_d_n11, assign20700_e28607_d_n12, assign20700_e28607_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20700_e28605: f64 = (1.0 / locals.var_dnm);
        (assign20700_e28605, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign20700_e28607;
        locals.var_dnm_dn0 = assign20700_e28607_d_n0;
        locals.var_dnm_dn2 = assign20700_e28607_d_n2;
        locals.var_dnm_dn6 = assign20700_e28607_d_n6;
        locals.var_dnm_dn7 = assign20700_e28607_d_n7;
        locals.var_dnm_dn10 = assign20700_e28607_d_n10;
        locals.var_dnm_dn11 = assign20700_e28607_d_n11;
        locals.var_dnm_dn12 = assign20700_e28607_d_n12;
        locals.var_dnm_dn17 = assign20700_e28607_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign20710_e28626, assign20710_e28626_d_n0, assign20710_e28626_d_n2, assign20710_e28626_d_n6, assign20710_e28626_d_n7, assign20710_e28626_d_n10, assign20710_e28626_d_n11, assign20710_e28626_d_n12, assign20710_e28626_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20710_e28622: f64 = (locals.var_tmf1 * 0.4);
        let assign20710_e28624: f64 = (assign20710_e28622 * locals.var_dnm);
        (assign20710_e28624, (((locals.var_tmf1_dn0 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 0.4) * locals.var_dnm) + (assign20710_e28622 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign20710_e28626;
        locals.var_tmf0_dn0 = assign20710_e28626_d_n0;
        locals.var_tmf0_dn2 = assign20710_e28626_d_n2;
        locals.var_tmf0_dn6 = assign20710_e28626_d_n6;
        locals.var_tmf0_dn7 = assign20710_e28626_d_n7;
        locals.var_tmf0_dn10 = assign20710_e28626_d_n10;
        locals.var_tmf0_dn11 = assign20710_e28626_d_n11;
        locals.var_tmf0_dn12 = assign20710_e28626_d_n12;
        locals.var_tmf0_dn17 = assign20710_e28626_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign20720_e28647, assign20720_e28647_d_n0, assign20720_e28647_d_n2, assign20720_e28647_d_n6, assign20720_e28647_d_n7, assign20720_e28647_d_n10, assign20720_e28647_d_n11, assign20720_e28647_d_n12, assign20720_e28647_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign20720_e28641: f64 = (locals.var_ps0_inib * 0.98);
        let assign20720_e28643: f64 = (assign20720_e28641 - 0.4);
        let assign20720_e28645: f64 = (assign20720_e28643 + locals.var_tmf0);
        (assign20720_e28645, ((locals.var_ps0_inib_dn0 * 0.98) + locals.var_tmf0_dn0), ((locals.var_ps0_inib_dn2 * 0.98) + locals.var_tmf0_dn2), ((locals.var_ps0_inib_dn6 * 0.98) + locals.var_tmf0_dn6), ((locals.var_ps0_inib_dn7 * 0.98) + locals.var_tmf0_dn7), ((locals.var_ps0_inib_dn10 * 0.98) + locals.var_tmf0_dn10), ((locals.var_ps0_inib_dn11 * 0.98) + locals.var_tmf0_dn11), ((locals.var_ps0_inib_dn12 * 0.98) + locals.var_tmf0_dn12), ((locals.var_ps0_inib_dn17 * 0.98) + locals.var_tmf0_dn17),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20720_e28647;
        locals.var_ps0_ini_dn0 = assign20720_e28647_d_n0;
        locals.var_ps0_ini_dn2 = assign20720_e28647_d_n2;
        locals.var_ps0_ini_dn6 = assign20720_e28647_d_n6;
        locals.var_ps0_ini_dn7 = assign20720_e28647_d_n7;
        locals.var_ps0_ini_dn10 = assign20720_e28647_d_n10;
        locals.var_ps0_ini_dn11 = assign20720_e28647_d_n11;
        locals.var_ps0_ini_dn12 = assign20720_e28647_d_n12;
        locals.var_ps0_ini_dn17 = assign20720_e28647_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20730_e28663, assign20730_e28663_d_n0, assign20730_e28663_d_n2, assign20730_e28663_d_n6, assign20730_e28663_d_n7, assign20730_e28663_d_n10, assign20730_e28663_d_n11, assign20730_e28663_d_n12, assign20730_e28663_d_n17,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 == 0.0)) && (locals.var_guard632 != 0.0)) && (locals.var_guard633 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20730_e28663;
        locals.var_ps0_ini_dn0 = assign20730_e28663_d_n0;
        locals.var_ps0_ini_dn2 = assign20730_e28663_d_n2;
        locals.var_ps0_ini_dn6 = assign20730_e28663_d_n6;
        locals.var_ps0_ini_dn7 = assign20730_e28663_d_n7;
        locals.var_ps0_ini_dn10 = assign20730_e28663_d_n10;
        locals.var_ps0_ini_dn11 = assign20730_e28663_d_n11;
        locals.var_ps0_ini_dn12 = assign20730_e28663_d_n12;
        locals.var_ps0_ini_dn17 = assign20730_e28663_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20740_e28671, assign20740_e28671_d_n0, assign20740_e28671_d_n2, assign20740_e28671_d_n6, assign20740_e28671_d_n7, assign20740_e28671_d_n10, assign20740_e28671_d_n11, assign20740_e28671_d_n12, assign20740_e28671_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign20740_e28668: f64 = (5e-12 / 2.0);
        let assign20740_e28669: f64 = (locals.var_vbs__blk623 + assign20740_e28668);
        (assign20740_e28669, locals.var_vbs__blk623_dn0, locals.var_vbs__blk623_dn2, locals.var_vbs__blk623_dn6, locals.var_vbs__blk623_dn7, locals.var_vbs__blk623_dn10, locals.var_vbs__blk623_dn11, locals.var_vbs__blk623_dn12, locals.var_vbs__blk623_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign20740_e28671;
        locals.var_tx_dn0 = assign20740_e28671_d_n0;
        locals.var_tx_dn2 = assign20740_e28671_d_n2;
        locals.var_tx_dn6 = assign20740_e28671_d_n6;
        locals.var_tx_dn7 = assign20740_e28671_d_n7;
        locals.var_tx_dn10 = assign20740_e28671_d_n10;
        locals.var_tx_dn11 = assign20740_e28671_d_n11;
        locals.var_tx_dn12 = assign20740_e28671_d_n12;
        locals.var_tx_dn17 = assign20740_e28671_d_n17;
        locals.var_tx_rv = 0.0;

        let assign20750_e28674: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard639 = assign20750_e28674;
        locals.var_guard639_rv = 0.0;

        let (assign20760_e28680, assign20760_e28680_d_n0, assign20760_e28680_d_n2, assign20760_e28680_d_n6, assign20760_e28680_d_n7, assign20760_e28680_d_n10, assign20760_e28680_d_n11, assign20760_e28680_d_n12, assign20760_e28680_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard639 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20760_e28680;
        locals.var_ps0_ini_dn0 = assign20760_e28680_d_n0;
        locals.var_ps0_ini_dn2 = assign20760_e28680_d_n2;
        locals.var_ps0_ini_dn6 = assign20760_e28680_d_n6;
        locals.var_ps0_ini_dn7 = assign20760_e28680_d_n7;
        locals.var_ps0_ini_dn10 = assign20760_e28680_d_n10;
        locals.var_ps0_ini_dn11 = assign20760_e28680_d_n11;
        locals.var_ps0_ini_dn12 = assign20760_e28680_d_n12;
        locals.var_ps0_ini_dn17 = assign20760_e28680_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20770_e28684, assign20770_e28684_d_n0, assign20770_e28684_d_n2, assign20770_e28684_d_n6, assign20770_e28684_d_n7, assign20770_e28684_d_n10, assign20770_e28684_d_n11, assign20770_e28684_d_n12, assign20770_e28684_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0__blk606, locals.var_ps0__blk606_dn0, locals.var_ps0__blk606_dn2, locals.var_ps0__blk606_dn6, locals.var_ps0__blk606_dn7, locals.var_ps0__blk606_dn10, locals.var_ps0__blk606_dn11, locals.var_ps0__blk606_dn12, locals.var_ps0__blk606_dn17,)
    }
};
        locals.var_ps0__blk606 = assign20770_e28684;
        locals.var_ps0__blk606_dn0 = assign20770_e28684_d_n0;
        locals.var_ps0__blk606_dn2 = assign20770_e28684_d_n2;
        locals.var_ps0__blk606_dn6 = assign20770_e28684_d_n6;
        locals.var_ps0__blk606_dn7 = assign20770_e28684_d_n7;
        locals.var_ps0__blk606_dn10 = assign20770_e28684_d_n10;
        locals.var_ps0__blk606_dn11 = assign20770_e28684_d_n11;
        locals.var_ps0__blk606_dn12 = assign20770_e28684_d_n12;
        locals.var_ps0__blk606_dn17 = assign20770_e28684_d_n17;
        locals.var_ps0__blk606_rv = 0.0;

        let (assign20780_e28688, assign20780_e28688_d_n0, assign20780_e28688_d_n2, assign20780_e28688_d_n6, assign20780_e28688_d_n7, assign20780_e28688_d_n10, assign20780_e28688_d_n11, assign20780_e28688_d_n12, assign20780_e28688_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign20780_e28688;
        locals.var_psl_lim_dn0 = assign20780_e28688_d_n0;
        locals.var_psl_lim_dn2 = assign20780_e28688_d_n2;
        locals.var_psl_lim_dn6 = assign20780_e28688_d_n6;
        locals.var_psl_lim_dn7 = assign20780_e28688_d_n7;
        locals.var_psl_lim_dn10 = assign20780_e28688_d_n10;
        locals.var_psl_lim_dn11 = assign20780_e28688_d_n11;
        locals.var_psl_lim_dn12 = assign20780_e28688_d_n12;
        locals.var_psl_lim_dn17 = assign20780_e28688_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let (assign20790_e28703, assign20790_e28703_d_n0, assign20790_e28703_d_n2, assign20790_e28703_d_n6, assign20790_e28703_d_n7, assign20790_e28703_d_n10, assign20790_e28703_d_n11, assign20790_e28703_d_n12, assign20790_e28703_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let assign20790_e28694: f64 = (locals.var_ps0_inia - locals.var_ps0__blk606);
        let (assign20790_e28701, assign20790_e28701_d_n0, assign20790_e28701_d_n2, assign20790_e28701_d_n6, assign20790_e28701_d_n7, assign20790_e28701_d_n10, assign20790_e28701_d_n11, assign20790_e28701_d_n12, assign20790_e28701_d_n17,) = {
            if (assign20790_e28694 >= 0.0) {
                let assign20790_e28699: f64 = (locals.var_ps0_inia - locals.var_ps0__blk606);
                (assign20790_e28699, (locals.var_ps0_inia_dn0 - locals.var_ps0__blk606_dn0), (locals.var_ps0_inia_dn2 - locals.var_ps0__blk606_dn2), (locals.var_ps0_inia_dn6 - locals.var_ps0__blk606_dn6), (locals.var_ps0_inia_dn7 - locals.var_ps0__blk606_dn7), (locals.var_ps0_inia_dn10 - locals.var_ps0__blk606_dn10), (locals.var_ps0_inia_dn11 - locals.var_ps0__blk606_dn11), (locals.var_ps0_inia_dn12 - locals.var_ps0__blk606_dn12), (locals.var_ps0_inia_dn17 - locals.var_ps0__blk606_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign20790_e28701, assign20790_e28701_d_n0, assign20790_e28701_d_n2, assign20790_e28701_d_n6, assign20790_e28701_d_n7, assign20790_e28701_d_n10, assign20790_e28701_d_n11, assign20790_e28701_d_n12, assign20790_e28701_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign20790_e28703;
        locals.var_pds_max_dn0 = assign20790_e28703_d_n0;
        locals.var_pds_max_dn2 = assign20790_e28703_d_n2;
        locals.var_pds_max_dn6 = assign20790_e28703_d_n6;
        locals.var_pds_max_dn7 = assign20790_e28703_d_n7;
        locals.var_pds_max_dn10 = assign20790_e28703_d_n10;
        locals.var_pds_max_dn11 = assign20790_e28703_d_n11;
        locals.var_pds_max_dn12 = assign20790_e28703_d_n12;
        locals.var_pds_max_dn17 = assign20790_e28703_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign20800_e28717, assign20800_e28717_d_n0, assign20800_e28717_d_n2, assign20800_e28717_d_n6, assign20800_e28717_d_n7, assign20800_e28717_d_n10, assign20800_e28717_d_n11, assign20800_e28717_d_n12, assign20800_e28717_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let assign20800_e28709: f64 = (1.0 + 0.3);
        let assign20800_e28711: f64 = (assign20800_e28709 * locals.var_pds_max);
        let assign20800_e28713: f64 = (assign20800_e28711 - p.p287);
        let assign20800_e28715: f64 = (assign20800_e28713 - 0.03);
        (assign20800_e28715, (assign20800_e28709 * locals.var_pds_max_dn0), (assign20800_e28709 * locals.var_pds_max_dn2), (assign20800_e28709 * locals.var_pds_max_dn6), (assign20800_e28709 * locals.var_pds_max_dn7), (assign20800_e28709 * locals.var_pds_max_dn10), (assign20800_e28709 * locals.var_pds_max_dn11), (assign20800_e28709 * locals.var_pds_max_dn12), (assign20800_e28709 * locals.var_pds_max_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign20800_e28717;
        locals.var_tmf1_dn0 = assign20800_e28717_d_n0;
        locals.var_tmf1_dn2 = assign20800_e28717_d_n2;
        locals.var_tmf1_dn6 = assign20800_e28717_d_n6;
        locals.var_tmf1_dn7 = assign20800_e28717_d_n7;
        locals.var_tmf1_dn10 = assign20800_e28717_d_n10;
        locals.var_tmf1_dn11 = assign20800_e28717_d_n11;
        locals.var_tmf1_dn12 = assign20800_e28717_d_n12;
        locals.var_tmf1_dn17 = assign20800_e28717_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign20810_e28731, assign20810_e28731_d_n0, assign20810_e28731_d_n2, assign20810_e28731_d_n6, assign20810_e28731_d_n7, assign20810_e28731_d_n10, assign20810_e28731_d_n11, assign20810_e28731_d_n12, assign20810_e28731_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let assign20810_e28724: f64 = (1.0 + 0.3);
        let assign20810_e28726: f64 = (assign20810_e28724 * locals.var_pds_max);
        let assign20810_e28727: f64 = (4.0 * assign20810_e28726);
        let assign20810_e28729: f64 = (assign20810_e28727 * 0.03);
        (assign20810_e28729, ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign20810_e28724 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20810_e28731;
        locals.var_tmf2_dn0 = assign20810_e28731_d_n0;
        locals.var_tmf2_dn2 = assign20810_e28731_d_n2;
        locals.var_tmf2_dn6 = assign20810_e28731_d_n6;
        locals.var_tmf2_dn7 = assign20810_e28731_d_n7;
        locals.var_tmf2_dn10 = assign20810_e28731_d_n10;
        locals.var_tmf2_dn11 = assign20810_e28731_d_n11;
        locals.var_tmf2_dn12 = assign20810_e28731_d_n12;
        locals.var_tmf2_dn17 = assign20810_e28731_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20820_e28743, assign20820_e28743_d_n0, assign20820_e28743_d_n2, assign20820_e28743_d_n6, assign20820_e28743_d_n7, assign20820_e28743_d_n10, assign20820_e28743_d_n11, assign20820_e28743_d_n12, assign20820_e28743_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let (assign20820_e28741, assign20820_e28741_d_n0, assign20820_e28741_d_n2, assign20820_e28741_d_n6, assign20820_e28741_d_n7, assign20820_e28741_d_n10, assign20820_e28741_d_n11, assign20820_e28741_d_n12, assign20820_e28741_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign20820_e28740: f64 = (-locals.var_tmf2);
                (assign20820_e28740, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign20820_e28741, assign20820_e28741_d_n0, assign20820_e28741_d_n2, assign20820_e28741_d_n6, assign20820_e28741_d_n7, assign20820_e28741_d_n10, assign20820_e28741_d_n11, assign20820_e28741_d_n12, assign20820_e28741_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20820_e28743;
        locals.var_tmf2_dn0 = assign20820_e28743_d_n0;
        locals.var_tmf2_dn2 = assign20820_e28743_d_n2;
        locals.var_tmf2_dn6 = assign20820_e28743_d_n6;
        locals.var_tmf2_dn7 = assign20820_e28743_d_n7;
        locals.var_tmf2_dn10 = assign20820_e28743_d_n10;
        locals.var_tmf2_dn11 = assign20820_e28743_d_n11;
        locals.var_tmf2_dn12 = assign20820_e28743_d_n12;
        locals.var_tmf2_dn17 = assign20820_e28743_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20830_e28754, assign20830_e28754_d_n0, assign20830_e28754_d_n2, assign20830_e28754_d_n6, assign20830_e28754_d_n7, assign20830_e28754_d_n10, assign20830_e28754_d_n11, assign20830_e28754_d_n12, assign20830_e28754_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let assign20830_e28749: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20830_e28751: f64 = (assign20830_e28749 + locals.var_tmf2);
        let assign20830_e28752: f64 = (assign20830_e28751).sqrt();
        (assign20830_e28752, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20830_e28752)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20830_e28752)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20830_e28752)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20830_e28752)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20830_e28752)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20830_e28752)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20830_e28752)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign20830_e28752)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign20830_e28754;
        locals.var_tmf2_dn0 = assign20830_e28754_d_n0;
        locals.var_tmf2_dn2 = assign20830_e28754_d_n2;
        locals.var_tmf2_dn6 = assign20830_e28754_d_n6;
        locals.var_tmf2_dn7 = assign20830_e28754_d_n7;
        locals.var_tmf2_dn10 = assign20830_e28754_d_n10;
        locals.var_tmf2_dn11 = assign20830_e28754_d_n11;
        locals.var_tmf2_dn12 = assign20830_e28754_d_n12;
        locals.var_tmf2_dn17 = assign20830_e28754_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign20840_e28770, assign20840_e28770_d_n0, assign20840_e28770_d_n2, assign20840_e28770_d_n6, assign20840_e28770_d_n7, assign20840_e28770_d_n10, assign20840_e28770_d_n11, assign20840_e28770_d_n12, assign20840_e28770_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let assign20840_e28760: f64 = (1.0 + 0.3);
        let assign20840_e28762: f64 = (assign20840_e28760 * locals.var_pds_max);
        let assign20840_e28766: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20840_e28767: f64 = (0.5 * assign20840_e28766);
        let assign20840_e28768: f64 = (assign20840_e28762 - assign20840_e28767);
        (assign20840_e28768, ((assign20840_e28760 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign20840_e28760 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign20840_e28760 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign20840_e28760 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign20840_e28760 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign20840_e28760 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign20840_e28760 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign20840_e28760 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20840_e28770;
        locals.var_pds_ini_dn0 = assign20840_e28770_d_n0;
        locals.var_pds_ini_dn2 = assign20840_e28770_d_n2;
        locals.var_pds_ini_dn6 = assign20840_e28770_d_n6;
        locals.var_pds_ini_dn7 = assign20840_e28770_d_n7;
        locals.var_pds_ini_dn10 = assign20840_e28770_d_n10;
        locals.var_pds_ini_dn11 = assign20840_e28770_d_n11;
        locals.var_pds_ini_dn12 = assign20840_e28770_d_n12;
        locals.var_pds_ini_dn17 = assign20840_e28770_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign20850_e28781, assign20850_e28781_d_n0, assign20850_e28781_d_n2, assign20850_e28781_d_n6, assign20850_e28781_d_n7, assign20850_e28781_d_n10, assign20850_e28781_d_n11, assign20850_e28781_d_n12, assign20850_e28781_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let (assign20850_e28779, assign20850_e28779_d_n0, assign20850_e28779_d_n2, assign20850_e28779_d_n6, assign20850_e28779_d_n7, assign20850_e28779_d_n10, assign20850_e28779_d_n11, assign20850_e28779_d_n12, assign20850_e28779_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign20850_e28779, assign20850_e28779_d_n0, assign20850_e28779_d_n2, assign20850_e28779_d_n6, assign20850_e28779_d_n7, assign20850_e28779_d_n10, assign20850_e28779_d_n11, assign20850_e28779_d_n12, assign20850_e28779_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20850_e28781;
        locals.var_pds_ini_dn0 = assign20850_e28781_d_n0;
        locals.var_pds_ini_dn2 = assign20850_e28781_d_n2;
        locals.var_pds_ini_dn6 = assign20850_e28781_d_n6;
        locals.var_pds_ini_dn7 = assign20850_e28781_d_n7;
        locals.var_pds_ini_dn10 = assign20850_e28781_d_n10;
        locals.var_pds_ini_dn11 = assign20850_e28781_d_n11;
        locals.var_pds_ini_dn12 = assign20850_e28781_d_n12;
        locals.var_pds_ini_dn17 = assign20850_e28781_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign20860_e28784: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign20860_e28784;
        locals.var_guard640_rv = 0.0;

        let (assign20870_e28792, assign20870_e28792_d_n0, assign20870_e28792_d_n2, assign20870_e28792_d_n6, assign20870_e28792_d_n7, assign20870_e28792_d_n10, assign20870_e28792_d_n11, assign20870_e28792_d_n12, assign20870_e28792_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (0.0 != 0.0)) && (locals.var_guard640 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign20870_e28792;
        locals.var_pds_ini_dn0 = assign20870_e28792_d_n0;
        locals.var_pds_ini_dn2 = assign20870_e28792_d_n2;
        locals.var_pds_ini_dn6 = assign20870_e28792_d_n6;
        locals.var_pds_ini_dn7 = assign20870_e28792_d_n7;
        locals.var_pds_ini_dn10 = assign20870_e28792_d_n10;
        locals.var_pds_ini_dn11 = assign20870_e28792_d_n11;
        locals.var_pds_ini_dn12 = assign20870_e28792_d_n12;
        locals.var_pds_ini_dn17 = assign20870_e28792_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign20880_e28795: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard641 = assign20880_e28795;
        locals.var_guard641_rv = 0.0;

        let (assign20890_e28806, assign20890_e28806_d_n0, assign20890_e28806_d_n2, assign20890_e28806_d_n6, assign20890_e28806_d_n7, assign20890_e28806_d_n10, assign20890_e28806_d_n11, assign20890_e28806_d_n12, assign20890_e28806_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (0.0 != 0.0)) && (locals.var_guard640 == 0.0)) && (locals.var_guard641 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
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

    }

    pub(super) fn stamp_reactive_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20900_e28814, assign20900_e28814_d_n0, assign20900_e28814_d_n2, assign20900_e28814_d_n6, assign20900_e28814_d_n7, assign20900_e28814_d_n10, assign20900_e28814_d_n11, assign20900_e28814_d_n12, assign20900_e28814_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (0.0 != 0.0)) {
        let assign20900_e28812: f64 = (locals.var_ps0__blk606 + locals.var_pds_ini);
        (assign20900_e28812, (locals.var_ps0__blk606_dn0 + locals.var_pds_ini_dn0), (locals.var_ps0__blk606_dn2 + locals.var_pds_ini_dn2), (locals.var_ps0__blk606_dn6 + locals.var_pds_ini_dn6), (locals.var_ps0__blk606_dn7 + locals.var_pds_ini_dn7), (locals.var_ps0__blk606_dn10 + locals.var_pds_ini_dn10), (locals.var_ps0__blk606_dn11 + locals.var_pds_ini_dn11), (locals.var_ps0__blk606_dn12 + locals.var_pds_ini_dn12), (locals.var_ps0__blk606_dn17 + locals.var_pds_ini_dn17),)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign20900_e28814;
        locals.var_psl_lim_dn0 = assign20900_e28814_d_n0;
        locals.var_psl_lim_dn2 = assign20900_e28814_d_n2;
        locals.var_psl_lim_dn6 = assign20900_e28814_d_n6;
        locals.var_psl_lim_dn7 = assign20900_e28814_d_n7;
        locals.var_psl_lim_dn10 = assign20900_e28814_d_n10;
        locals.var_psl_lim_dn11 = assign20900_e28814_d_n11;
        locals.var_psl_lim_dn12 = assign20900_e28814_d_n12;
        locals.var_psl_lim_dn17 = assign20900_e28814_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let assign20910_e28817: f64 = if p.p282 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign20910_e28817;
        locals.var_guard642_rv = 0.0;

        let (assign20920_e28823, assign20920_e28823_d_n0, assign20920_e28823_d_n2, assign20920_e28823_d_n6, assign20920_e28823_d_n7, assign20920_e28823_d_n10, assign20920_e28823_d_n11, assign20920_e28823_d_n12, assign20920_e28823_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) {
        (locals.var_ps0__blk606, locals.var_ps0__blk606_dn0, locals.var_ps0__blk606_dn2, locals.var_ps0__blk606_dn6, locals.var_ps0__blk606_dn7, locals.var_ps0__blk606_dn10, locals.var_ps0__blk606_dn11, locals.var_ps0__blk606_dn12, locals.var_ps0__blk606_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign20920_e28823;
        locals.var_ps0_ini_dn0 = assign20920_e28823_d_n0;
        locals.var_ps0_ini_dn2 = assign20920_e28823_d_n2;
        locals.var_ps0_ini_dn6 = assign20920_e28823_d_n6;
        locals.var_ps0_ini_dn7 = assign20920_e28823_d_n7;
        locals.var_ps0_ini_dn10 = assign20920_e28823_d_n10;
        locals.var_ps0_ini_dn11 = assign20920_e28823_d_n11;
        locals.var_ps0_ini_dn12 = assign20920_e28823_d_n12;
        locals.var_ps0_ini_dn17 = assign20920_e28823_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign20930_e28829, assign20930_e28829_d_n0, assign20930_e28829_d_n2, assign20930_e28829_d_n6, assign20930_e28829_d_n7, assign20930_e28829_d_n10, assign20930_e28829_d_n11, assign20930_e28829_d_n12, assign20930_e28829_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn12, locals.var_dphi_vds_dn17,)
    } else {
        (locals.var_vbcs_cl__blk643, locals.var_vbcs_cl__blk643_dn0, locals.var_vbcs_cl__blk643_dn2, locals.var_vbcs_cl__blk643_dn6, locals.var_vbcs_cl__blk643_dn7, locals.var_vbcs_cl__blk643_dn10, locals.var_vbcs_cl__blk643_dn11, locals.var_vbcs_cl__blk643_dn12, locals.var_vbcs_cl__blk643_dn17,)
    }
};
        locals.var_vbcs_cl__blk643 = assign20930_e28829;
        locals.var_vbcs_cl__blk643_dn0 = assign20930_e28829_d_n0;
        locals.var_vbcs_cl__blk643_dn2 = assign20930_e28829_d_n2;
        locals.var_vbcs_cl__blk643_dn6 = assign20930_e28829_d_n6;
        locals.var_vbcs_cl__blk643_dn7 = assign20930_e28829_d_n7;
        locals.var_vbcs_cl__blk643_dn10 = assign20930_e28829_d_n10;
        locals.var_vbcs_cl__blk643_dn11 = assign20930_e28829_d_n11;
        locals.var_vbcs_cl__blk643_dn12 = assign20930_e28829_d_n12;
        locals.var_vbcs_cl__blk643_dn17 = assign20930_e28829_d_n17;
        locals.var_vbcs_cl__blk643_rv = 0.0;

        let (assign20940_e28843, assign20940_e28843_d_n0, assign20940_e28843_d_n2, assign20940_e28843_d_n6, assign20940_e28843_d_n7, assign20940_e28843_d_n10, assign20940_e28843_d_n11, assign20940_e28843_d_n12, assign20940_e28843_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) {
        let assign20940_e28835: f64 = (locals.var_vfb - locals.var_dvth);
        let assign20940_e28837: f64 = (assign20940_e28835 + locals.var_dppg);
        let assign20940_e28839: f64 = (assign20940_e28837 + locals.var_vbcs_cl__blk643);
        let assign20940_e28841: f64 = (assign20940_e28839 + p.p286);
        (assign20940_e28841, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbcs_cl__blk643_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbcs_cl__blk643_dn2), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbcs_cl__blk643_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbcs_cl__blk643_dn7), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbcs_cl__blk643_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbcs_cl__blk643_dn11), (((-locals.var_dvth_dn12) + locals.var_dppg_dn12) + locals.var_vbcs_cl__blk643_dn12), (((-locals.var_dvth_dn17) + locals.var_dppg_dn17) + locals.var_vbcs_cl__blk643_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign20940_e28843;
        locals.var_vgs_fb_dn0 = assign20940_e28843_d_n0;
        locals.var_vgs_fb_dn2 = assign20940_e28843_d_n2;
        locals.var_vgs_fb_dn6 = assign20940_e28843_d_n6;
        locals.var_vgs_fb_dn7 = assign20940_e28843_d_n7;
        locals.var_vgs_fb_dn10 = assign20940_e28843_d_n10;
        locals.var_vgs_fb_dn11 = assign20940_e28843_d_n11;
        locals.var_vgs_fb_dn12 = assign20940_e28843_d_n12;
        locals.var_vgs_fb_dn17 = assign20940_e28843_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let assign20950_e28846: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard645 = assign20950_e28846;
        locals.var_guard645_rv = 0.0;

        let (assign20960_e28855,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign20960_e28853: f64 = (-1.0);
        (assign20960_e28853,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign20960_e28855;
        locals.var_flg_zone_rv = 0.0;

        let (assign20970_e28871, assign20970_e28871_d_n0, assign20970_e28871_d_n2, assign20970_e28871_d_n6, assign20970_e28871_d_n7, assign20970_e28871_d_n10, assign20970_e28871_d_n11, assign20970_e28871_d_n12, assign20970_e28871_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign20970_e28863: f64 = (2.0 * locals.var_beta_inv);
        let assign20970_e28865: f64 = (-locals.var_vgs_min);
        let assign20970_e28867: f64 = (assign20970_e28865 / locals.var_fac1);
        let assign20970_e28868: f64 = (assign20970_e28867).ln();
        let assign20970_e28869: f64 = (assign20970_e28863 * assign20970_e28868);
        (assign20970_e28869, (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867)), (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867)), (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867)), (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867)), (((2.0 * locals.var_beta_inv_dn10) * assign20970_e28868) + (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867))), (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867)), (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867)), (assign20970_e28863 * ((-((assign20970_e28865 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign20970_e28867)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign20970_e28871;
        locals.var_ps0_min_dn0 = assign20970_e28871_d_n0;
        locals.var_ps0_min_dn2 = assign20970_e28871_d_n2;
        locals.var_ps0_min_dn6 = assign20970_e28871_d_n6;
        locals.var_ps0_min_dn7 = assign20970_e28871_d_n7;
        locals.var_ps0_min_dn10 = assign20970_e28871_d_n10;
        locals.var_ps0_min_dn11 = assign20970_e28871_d_n11;
        locals.var_ps0_min_dn12 = assign20970_e28871_d_n12;
        locals.var_ps0_min_dn17 = assign20970_e28871_d_n17;
        locals.var_ps0_min_rv = 0.0;

        let (assign20980_e28883, assign20980_e28883_d_n0, assign20980_e28883_d_n2, assign20980_e28883_d_n6, assign20980_e28883_d_n7, assign20980_e28883_d_n10, assign20980_e28883_d_n11, assign20980_e28883_d_n12, assign20980_e28883_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign20980_e28880: f64 = (locals.var_vgp__blk608 - locals.var_vbcs_cl__blk643);
        let assign20980_e28881: f64 = (locals.var_beta * assign20980_e28880);
        (assign20980_e28881, (locals.var_beta * (locals.var_vgp__blk608_dn0 - locals.var_vbcs_cl__blk643_dn0)), (locals.var_beta * (locals.var_vgp__blk608_dn2 - locals.var_vbcs_cl__blk643_dn2)), (locals.var_beta * (locals.var_vgp__blk608_dn6 - locals.var_vbcs_cl__blk643_dn6)), (locals.var_beta * (locals.var_vgp__blk608_dn7 - locals.var_vbcs_cl__blk643_dn7)), ((locals.var_beta_dn10 * assign20980_e28880) + (locals.var_beta * (locals.var_vgp__blk608_dn10 - locals.var_vbcs_cl__blk643_dn10))), (locals.var_beta * (locals.var_vgp__blk608_dn11 - locals.var_vbcs_cl__blk643_dn11)), (locals.var_beta * (locals.var_vgp__blk608_dn12 - locals.var_vbcs_cl__blk643_dn12)), (locals.var_beta * (locals.var_vgp__blk608_dn17 - locals.var_vbcs_cl__blk643_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign20980_e28883;
        locals.var_tx_dn0 = assign20980_e28883_d_n0;
        locals.var_tx_dn2 = assign20980_e28883_d_n2;
        locals.var_tx_dn6 = assign20980_e28883_d_n6;
        locals.var_tx_dn7 = assign20980_e28883_d_n7;
        locals.var_tx_dn10 = assign20980_e28883_d_n10;
        locals.var_tx_dn11 = assign20980_e28883_d_n11;
        locals.var_tx_dn12 = assign20980_e28883_d_n12;
        locals.var_tx_dn17 = assign20980_e28883_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign20990_e28895, assign20990_e28895_d_n0, assign20990_e28895_d_n2, assign20990_e28895_d_n6, assign20990_e28895_d_n7, assign20990_e28895_d_n10, assign20990_e28895_d_n11, assign20990_e28895_d_n12, assign20990_e28895_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign20990_e28892: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign20990_e28893: f64 = (1.0 / assign20990_e28892);
        (assign20990_e28893, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign20990_e28892 * assign20990_e28892))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign20990_e28892 * assign20990_e28892))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign20990_e28892 * assign20990_e28892))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign20990_e28892 * assign20990_e28892))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign20990_e28892 * assign20990_e28892))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign20990_e28892 * assign20990_e28892))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign20990_e28892 * assign20990_e28892))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign20990_e28892 * assign20990_e28892))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign20990_e28895;
        locals.var_t1_dn0 = assign20990_e28895_d_n0;
        locals.var_t1_dn2 = assign20990_e28895_d_n2;
        locals.var_t1_dn6 = assign20990_e28895_d_n6;
        locals.var_t1_dn7 = assign20990_e28895_d_n7;
        locals.var_t1_dn10 = assign20990_e28895_d_n10;
        locals.var_t1_dn11 = assign20990_e28895_d_n11;
        locals.var_t1_dn12 = assign20990_e28895_d_n12;
        locals.var_t1_dn17 = assign20990_e28895_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21000_e28905, assign21000_e28905_d_n0, assign21000_e28905_d_n2, assign21000_e28905_d_n6, assign21000_e28905_d_n7, assign21000_e28905_d_n10, assign21000_e28905_d_n11, assign21000_e28905_d_n12, assign21000_e28905_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21000_e28903: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign21000_e28903, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign21000_e28905;
        locals.var_ty_dn0 = assign21000_e28905_d_n0;
        locals.var_ty_dn2 = assign21000_e28905_d_n2;
        locals.var_ty_dn6 = assign21000_e28905_d_n6;
        locals.var_ty_dn7 = assign21000_e28905_d_n7;
        locals.var_ty_dn10 = assign21000_e28905_d_n10;
        locals.var_ty_dn11 = assign21000_e28905_d_n11;
        locals.var_ty_dn12 = assign21000_e28905_d_n12;
        locals.var_ty_dn17 = assign21000_e28905_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign21010_e28919, assign21010_e28919_d_n0, assign21010_e28919_d_n2, assign21010_e28919_d_n6, assign21010_e28919_d_n7, assign21010_e28919_d_n10, assign21010_e28919_d_n11, assign21010_e28919_d_n12, assign21010_e28919_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21010_e28914: f64 = (3.0 * 1.414213562373095);
        let assign21010_e28916: f64 = (assign21010_e28914 * locals.var_ty);
        let assign21010_e28917: f64 = (2.0 + assign21010_e28916);
        (assign21010_e28917, (assign21010_e28914 * locals.var_ty_dn0), (assign21010_e28914 * locals.var_ty_dn2), (assign21010_e28914 * locals.var_ty_dn6), (assign21010_e28914 * locals.var_ty_dn7), (assign21010_e28914 * locals.var_ty_dn10), (assign21010_e28914 * locals.var_ty_dn11), (assign21010_e28914 * locals.var_ty_dn12), (assign21010_e28914 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign21010_e28919;
        locals.var_ac41_dn0 = assign21010_e28919_d_n0;
        locals.var_ac41_dn2 = assign21010_e28919_d_n2;
        locals.var_ac41_dn6 = assign21010_e28919_d_n6;
        locals.var_ac41_dn7 = assign21010_e28919_d_n7;
        locals.var_ac41_dn10 = assign21010_e28919_d_n10;
        locals.var_ac41_dn11 = assign21010_e28919_d_n11;
        locals.var_ac41_dn12 = assign21010_e28919_d_n12;
        locals.var_ac41_dn17 = assign21010_e28919_d_n17;
        locals.var_ac41_rv = 0.0;

        let (assign21020_e28933, assign21020_e28933_d_n0, assign21020_e28933_d_n2, assign21020_e28933_d_n6, assign21020_e28933_d_n7, assign21020_e28933_d_n10, assign21020_e28933_d_n11, assign21020_e28933_d_n12, assign21020_e28933_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21020_e28927: f64 = (8.0 * locals.var_ac41);
        let assign21020_e28929: f64 = (assign21020_e28927 * locals.var_ac41);
        let assign21020_e28931: f64 = (assign21020_e28929 * locals.var_ac41);
        (assign21020_e28931, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign21020_e28927 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign21020_e28929 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign21020_e28933;
        locals.var_ac4_dn0 = assign21020_e28933_d_n0;
        locals.var_ac4_dn2 = assign21020_e28933_d_n2;
        locals.var_ac4_dn6 = assign21020_e28933_d_n6;
        locals.var_ac4_dn7 = assign21020_e28933_d_n7;
        locals.var_ac4_dn10 = assign21020_e28933_d_n10;
        locals.var_ac4_dn11 = assign21020_e28933_d_n11;
        locals.var_ac4_dn12 = assign21020_e28933_d_n12;
        locals.var_ac4_dn17 = assign21020_e28933_d_n17;
        locals.var_ac4_rv = 0.0;

        let (assign21030_e28943, assign21030_e28943_d_n0, assign21030_e28943_d_n2, assign21030_e28943_d_n6, assign21030_e28943_d_n7, assign21030_e28943_d_n10, assign21030_e28943_d_n11, assign21030_e28943_d_n12, assign21030_e28943_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21030_e28941: f64 = (locals.var_tx - 2.0);
        (assign21030_e28941, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign21030_e28943;
        locals.var_t4_dn0 = assign21030_e28943_d_n0;
        locals.var_t4_dn2 = assign21030_e28943_d_n2;
        locals.var_t4_dn6 = assign21030_e28943_d_n6;
        locals.var_t4_dn7 = assign21030_e28943_d_n7;
        locals.var_t4_dn10 = assign21030_e28943_d_n10;
        locals.var_t4_dn11 = assign21030_e28943_d_n11;
        locals.var_t4_dn12 = assign21030_e28943_d_n12;
        locals.var_t4_dn17 = assign21030_e28943_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign21040_e28955, assign21040_e28955_d_n0, assign21040_e28955_d_n2, assign21040_e28955_d_n6, assign21040_e28955_d_n7, assign21040_e28955_d_n10, assign21040_e28955_d_n11, assign21040_e28955_d_n12, assign21040_e28955_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21040_e28951: f64 = (9.0 * locals.var_ty);
        let assign21040_e28953: f64 = (assign21040_e28951 * locals.var_t4);
        (assign21040_e28953, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign21040_e28951 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign21040_e28955;
        locals.var_t5_dn0 = assign21040_e28955_d_n0;
        locals.var_t5_dn2 = assign21040_e28955_d_n2;
        locals.var_t5_dn6 = assign21040_e28955_d_n6;
        locals.var_t5_dn7 = assign21040_e28955_d_n7;
        locals.var_t5_dn10 = assign21040_e28955_d_n10;
        locals.var_t5_dn11 = assign21040_e28955_d_n11;
        locals.var_t5_dn12 = assign21040_e28955_d_n12;
        locals.var_t5_dn17 = assign21040_e28955_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign21050_e28967, assign21050_e28967_d_n0, assign21050_e28967_d_n2, assign21050_e28967_d_n6, assign21050_e28967_d_n7, assign21050_e28967_d_n10, assign21050_e28967_d_n11, assign21050_e28967_d_n12, assign21050_e28967_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21050_e28963: f64 = (7.0 * 1.414213562373095);
        let assign21050_e28965: f64 = (assign21050_e28963 - locals.var_t5);
        (assign21050_e28965, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign21050_e28967;
        locals.var_ac31_dn0 = assign21050_e28967_d_n0;
        locals.var_ac31_dn2 = assign21050_e28967_d_n2;
        locals.var_ac31_dn6 = assign21050_e28967_d_n6;
        locals.var_ac31_dn7 = assign21050_e28967_d_n7;
        locals.var_ac31_dn10 = assign21050_e28967_d_n10;
        locals.var_ac31_dn11 = assign21050_e28967_d_n11;
        locals.var_ac31_dn12 = assign21050_e28967_d_n12;
        locals.var_ac31_dn17 = assign21050_e28967_d_n17;
        locals.var_ac31_rv = 0.0;

        let (assign21060_e28977, assign21060_e28977_d_n0, assign21060_e28977_d_n2, assign21060_e28977_d_n6, assign21060_e28977_d_n7, assign21060_e28977_d_n10, assign21060_e28977_d_n11, assign21060_e28977_d_n12, assign21060_e28977_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21060_e28975: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign21060_e28975, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign21060_e28977;
        locals.var_ac3_dn0 = assign21060_e28977_d_n0;
        locals.var_ac3_dn2 = assign21060_e28977_d_n2;
        locals.var_ac3_dn6 = assign21060_e28977_d_n6;
        locals.var_ac3_dn7 = assign21060_e28977_d_n7;
        locals.var_ac3_dn10 = assign21060_e28977_d_n10;
        locals.var_ac3_dn11 = assign21060_e28977_d_n11;
        locals.var_ac3_dn12 = assign21060_e28977_d_n12;
        locals.var_ac3_dn17 = assign21060_e28977_d_n17;
        locals.var_ac3_rv = 0.0;

        let assign21070_e28981: f64 = (locals.var_ac3 * 1e-8);
        let assign21070_e28982: f64 = if locals.var_ac4 < assign21070_e28981 { 1.0 } else { 0.0 };
        locals.var_guard646 = assign21070_e28982;
        locals.var_guard646_rv = 0.0;

        let (assign21080_e29005, assign21080_e29005_d_n0, assign21080_e29005_d_n2, assign21080_e29005_d_n6, assign21080_e29005_d_n7, assign21080_e29005_d_n10, assign21080_e29005_d_n11, assign21080_e29005_d_n12, assign21080_e29005_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) {
        let assign21080_e28991: f64 = (-7.0);
        let assign21080_e28993: f64 = (assign21080_e28991 * 1.414213562373095);
        let assign21080_e28995: f64 = (assign21080_e28993 + locals.var_ac31);
        let assign21080_e28998: f64 = (0.5 * locals.var_ac4);
        let assign21080_e29000: f64 = (assign21080_e28998 / locals.var_ac31);
        let assign21080_e29001: f64 = (assign21080_e28995 + assign21080_e29000);
        let assign21080_e29003: f64 = (assign21080_e29001 + locals.var_t5);
        (assign21080_e29003, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign21080_e28998 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign21080_e29005;
        locals.var_ac1_dn0 = assign21080_e29005_d_n0;
        locals.var_ac1_dn2 = assign21080_e29005_d_n2;
        locals.var_ac1_dn6 = assign21080_e29005_d_n6;
        locals.var_ac1_dn7 = assign21080_e29005_d_n7;
        locals.var_ac1_dn10 = assign21080_e29005_d_n10;
        locals.var_ac1_dn11 = assign21080_e29005_d_n11;
        locals.var_ac1_dn12 = assign21080_e29005_d_n12;
        locals.var_ac1_dn17 = assign21080_e29005_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign21090_e29019, assign21090_e29019_d_n0, assign21090_e29019_d_n2, assign21090_e29019_d_n6, assign21090_e29019_d_n7, assign21090_e29019_d_n10, assign21090_e29019_d_n11, assign21090_e29019_d_n12, assign21090_e29019_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 == 0.0)) {
        let assign21090_e29016: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign21090_e29017: f64 = (assign21090_e29016).sqrt();
        (assign21090_e29017, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign21090_e29017)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign21090_e29017)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign21090_e29017)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign21090_e29017)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign21090_e29017)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign21090_e29017)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign21090_e29017)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign21090_e29017)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign21090_e29019;
        locals.var_ac2_dn0 = assign21090_e29019_d_n0;
        locals.var_ac2_dn2 = assign21090_e29019_d_n2;
        locals.var_ac2_dn6 = assign21090_e29019_d_n6;
        locals.var_ac2_dn7 = assign21090_e29019_d_n7;
        locals.var_ac2_dn10 = assign21090_e29019_d_n10;
        locals.var_ac2_dn11 = assign21090_e29019_d_n11;
        locals.var_ac2_dn12 = assign21090_e29019_d_n12;
        locals.var_ac2_dn17 = assign21090_e29019_d_n17;
        locals.var_ac2_rv = 0.0;

        let (assign21100_e29037, assign21100_e29037_d_n0, assign21100_e29037_d_n2, assign21100_e29037_d_n6, assign21100_e29037_d_n7, assign21100_e29037_d_n10, assign21100_e29037_d_n11, assign21100_e29037_d_n12, assign21100_e29037_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 == 0.0)) {
        let assign21100_e29029: f64 = (-7.0);
        let assign21100_e29031: f64 = (assign21100_e29029 * 1.414213562373095);
        let assign21100_e29033: f64 = (assign21100_e29031 + locals.var_ac2);
        let assign21100_e29035: f64 = (assign21100_e29033 + locals.var_t5);
        (assign21100_e29035, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign21100_e29037;
        locals.var_ac1_dn0 = assign21100_e29037_d_n0;
        locals.var_ac1_dn2 = assign21100_e29037_d_n2;
        locals.var_ac1_dn6 = assign21100_e29037_d_n6;
        locals.var_ac1_dn7 = assign21100_e29037_d_n7;
        locals.var_ac1_dn10 = assign21100_e29037_d_n10;
        locals.var_ac1_dn11 = assign21100_e29037_d_n11;
        locals.var_ac1_dn12 = assign21100_e29037_d_n12;
        locals.var_ac1_dn17 = assign21100_e29037_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign21110_e29047, assign21110_e29047_d_n0, assign21110_e29047_d_n2, assign21110_e29047_d_n6, assign21110_e29047_d_n7, assign21110_e29047_d_n10, assign21110_e29047_d_n11, assign21110_e29047_d_n12, assign21110_e29047_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21110_e29045: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign21110_e29045, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign21110_e29045 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign21110_e29047;
        locals.var_acd_dn0 = assign21110_e29047_d_n0;
        locals.var_acd_dn2 = assign21110_e29047_d_n2;
        locals.var_acd_dn6 = assign21110_e29047_d_n6;
        locals.var_acd_dn7 = assign21110_e29047_d_n7;
        locals.var_acd_dn10 = assign21110_e29047_d_n10;
        locals.var_acd_dn11 = assign21110_e29047_d_n11;
        locals.var_acd_dn12 = assign21110_e29047_d_n12;
        locals.var_acd_dn17 = assign21110_e29047_d_n17;
        locals.var_acd_rv = 0.0;

        let (assign21120_e29072, assign21120_e29072_d_n0, assign21120_e29072_d_n2, assign21120_e29072_d_n6, assign21120_e29072_d_n7, assign21120_e29072_d_n10, assign21120_e29072_d_n11, assign21120_e29072_d_n12, assign21120_e29072_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21120_e29054: f64 = (-4.0);
        let assign21120_e29056: f64 = (assign21120_e29054 * 1.414213562373095);
        let assign21120_e29059: f64 = (12.0 * locals.var_ty);
        let assign21120_e29060: f64 = (assign21120_e29056 - assign21120_e29059);
        let assign21120_e29063: f64 = (2.0 * locals.var_acd);
        let assign21120_e29064: f64 = (assign21120_e29060 + assign21120_e29063);
        let assign21120_e29067: f64 = (1.414213562373095 * locals.var_acd);
        let assign21120_e29069: f64 = (assign21120_e29067 * locals.var_acd);
        let assign21120_e29070: f64 = (assign21120_e29064 + assign21120_e29069);
        (assign21120_e29070, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign21120_e29067 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign21120_e29072;
        locals.var_acn_dn0 = assign21120_e29072_d_n0;
        locals.var_acn_dn2 = assign21120_e29072_d_n2;
        locals.var_acn_dn6 = assign21120_e29072_d_n6;
        locals.var_acn_dn7 = assign21120_e29072_d_n7;
        locals.var_acn_dn10 = assign21120_e29072_d_n10;
        locals.var_acn_dn11 = assign21120_e29072_d_n11;
        locals.var_acn_dn12 = assign21120_e29072_d_n12;
        locals.var_acn_dn17 = assign21120_e29072_d_n17;
        locals.var_acn_rv = 0.0;

        let (assign21130_e29082, assign21130_e29082_d_n0, assign21130_e29082_d_n2, assign21130_e29082_d_n6, assign21130_e29082_d_n7, assign21130_e29082_d_n10, assign21130_e29082_d_n11, assign21130_e29082_d_n12, assign21130_e29082_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21130_e29080: f64 = (1.0 / locals.var_acd);
        (assign21130_e29080, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21130_e29082;
        locals.var_t1_dn0 = assign21130_e29082_d_n0;
        locals.var_t1_dn2 = assign21130_e29082_d_n2;
        locals.var_t1_dn6 = assign21130_e29082_d_n6;
        locals.var_t1_dn7 = assign21130_e29082_d_n7;
        locals.var_t1_dn10 = assign21130_e29082_d_n10;
        locals.var_t1_dn11 = assign21130_e29082_d_n11;
        locals.var_t1_dn12 = assign21130_e29082_d_n12;
        locals.var_t1_dn17 = assign21130_e29082_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21140_e29092, assign21140_e29092_d_n0, assign21140_e29092_d_n2, assign21140_e29092_d_n6, assign21140_e29092_d_n7, assign21140_e29092_d_n10, assign21140_e29092_d_n11, assign21140_e29092_d_n12, assign21140_e29092_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21140_e29090: f64 = (locals.var_acn * locals.var_t1);
        (assign21140_e29090, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign21140_e29092;
        locals.var_chi_dn0 = assign21140_e29092_d_n0;
        locals.var_chi_dn2 = assign21140_e29092_d_n2;
        locals.var_chi_dn6 = assign21140_e29092_d_n6;
        locals.var_chi_dn7 = assign21140_e29092_d_n7;
        locals.var_chi_dn10 = assign21140_e29092_d_n10;
        locals.var_chi_dn11 = assign21140_e29092_d_n11;
        locals.var_chi_dn12 = assign21140_e29092_d_n12;
        locals.var_chi_dn17 = assign21140_e29092_d_n17;
        locals.var_chi_rv = 0.0;

        let (assign21150_e29104, assign21150_e29104_d_n0, assign21150_e29104_d_n2, assign21150_e29104_d_n6, assign21150_e29104_d_n7, assign21150_e29104_d_n10, assign21150_e29104_d_n11, assign21150_e29104_d_n12, assign21150_e29104_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21150_e29100: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign21150_e29102: f64 = (assign21150_e29100 + locals.var_vbcs_cl__blk643);
        (assign21150_e29102, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl__blk643_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl__blk643_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl__blk643_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl__blk643_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl__blk643_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl__blk643_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl__blk643_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl__blk643_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign21150_e29104;
        locals.var_psa_dn0 = assign21150_e29104_d_n0;
        locals.var_psa_dn2 = assign21150_e29104_d_n2;
        locals.var_psa_dn6 = assign21150_e29104_d_n6;
        locals.var_psa_dn7 = assign21150_e29104_d_n7;
        locals.var_psa_dn10 = assign21150_e29104_d_n10;
        locals.var_psa_dn11 = assign21150_e29104_d_n11;
        locals.var_psa_dn12 = assign21150_e29104_d_n12;
        locals.var_psa_dn17 = assign21150_e29104_d_n17;
        locals.var_psa_rv = 0.0;

        let (assign21160_e29114, assign21160_e29114_d_n0, assign21160_e29114_d_n2, assign21160_e29114_d_n6, assign21160_e29114_d_n7, assign21160_e29114_d_n10, assign21160_e29114_d_n11, assign21160_e29114_d_n12, assign21160_e29114_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21160_e29112: f64 = (locals.var_psa - locals.var_vbcs_cl__blk643);
        (assign21160_e29112, (locals.var_psa_dn0 - locals.var_vbcs_cl__blk643_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl__blk643_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl__blk643_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl__blk643_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl__blk643_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl__blk643_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl__blk643_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl__blk643_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21160_e29114;
        locals.var_t1_dn0 = assign21160_e29114_d_n0;
        locals.var_t1_dn2 = assign21160_e29114_d_n2;
        locals.var_t1_dn6 = assign21160_e29114_d_n6;
        locals.var_t1_dn7 = assign21160_e29114_d_n7;
        locals.var_t1_dn10 = assign21160_e29114_d_n10;
        locals.var_t1_dn11 = assign21160_e29114_d_n11;
        locals.var_t1_dn12 = assign21160_e29114_d_n12;
        locals.var_t1_dn17 = assign21160_e29114_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21170_e29124, assign21170_e29124_d_n0, assign21170_e29124_d_n2, assign21170_e29124_d_n6, assign21170_e29124_d_n7, assign21170_e29124_d_n10, assign21170_e29124_d_n11, assign21170_e29124_d_n12, assign21170_e29124_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21170_e29122: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign21170_e29122, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21170_e29124;
        locals.var_t2_dn0 = assign21170_e29124_d_n0;
        locals.var_t2_dn2 = assign21170_e29124_d_n2;
        locals.var_t2_dn6 = assign21170_e29124_d_n6;
        locals.var_t2_dn7 = assign21170_e29124_d_n7;
        locals.var_t2_dn10 = assign21170_e29124_d_n10;
        locals.var_t2_dn11 = assign21170_e29124_d_n11;
        locals.var_t2_dn12 = assign21170_e29124_d_n12;
        locals.var_t2_dn17 = assign21170_e29124_d_n17;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21180_e29137, assign21180_e29137_d_n0, assign21180_e29137_d_n2, assign21180_e29137_d_n6, assign21180_e29137_d_n7, assign21180_e29137_d_n10, assign21180_e29137_d_n11, assign21180_e29137_d_n12, assign21180_e29137_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21180_e29133: f64 = (locals.var_t2 * locals.var_t2);
        let assign21180_e29134: f64 = (1.0 + assign21180_e29133);
        let assign21180_e29135: f64 = (assign21180_e29134).sqrt();
        (assign21180_e29135, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign21180_e29135)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign21180_e29135)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign21180_e29135)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign21180_e29135)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign21180_e29135)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign21180_e29135)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign21180_e29135)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign21180_e29135)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign21180_e29137;
        locals.var_t3_dn0 = assign21180_e29137_d_n0;
        locals.var_t3_dn2 = assign21180_e29137_d_n2;
        locals.var_t3_dn6 = assign21180_e29137_d_n6;
        locals.var_t3_dn7 = assign21180_e29137_d_n7;
        locals.var_t3_dn10 = assign21180_e29137_d_n10;
        locals.var_t3_dn11 = assign21180_e29137_d_n11;
        locals.var_t3_dn12 = assign21180_e29137_d_n12;
        locals.var_t3_dn17 = assign21180_e29137_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign21190_e29149, assign21190_e29149_d_n0, assign21190_e29149_d_n2, assign21190_e29149_d_n6, assign21190_e29149_d_n7, assign21190_e29149_d_n10, assign21190_e29149_d_n11, assign21190_e29149_d_n12, assign21190_e29149_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign21190_e29145: f64 = (locals.var_t1 / locals.var_t3);
        let assign21190_e29147: f64 = (assign21190_e29145 + locals.var_vbcs_cl__blk643);
        (assign21190_e29147, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl__blk643_dn17),)
    } else {
        (locals.var_ps0__blk606, locals.var_ps0__blk606_dn0, locals.var_ps0__blk606_dn2, locals.var_ps0__blk606_dn6, locals.var_ps0__blk606_dn7, locals.var_ps0__blk606_dn10, locals.var_ps0__blk606_dn11, locals.var_ps0__blk606_dn12, locals.var_ps0__blk606_dn17,)
    }
};
        locals.var_ps0__blk606 = assign21190_e29149;
        locals.var_ps0__blk606_dn0 = assign21190_e29149_d_n0;
        locals.var_ps0__blk606_dn2 = assign21190_e29149_d_n2;
        locals.var_ps0__blk606_dn6 = assign21190_e29149_d_n6;
        locals.var_ps0__blk606_dn7 = assign21190_e29149_d_n7;
        locals.var_ps0__blk606_dn10 = assign21190_e29149_d_n10;
        locals.var_ps0__blk606_dn11 = assign21190_e29149_d_n11;
        locals.var_ps0__blk606_dn12 = assign21190_e29149_d_n12;
        locals.var_ps0__blk606_dn17 = assign21190_e29149_d_n17;
        locals.var_ps0__blk606_rv = 0.0;

        let (assign21200_e29163, assign21200_e29163_d_n0, assign21200_e29163_d_n2, assign21200_e29163_d_n6, assign21200_e29163_d_n7, assign21200_e29163_d_n10, assign21200_e29163_d_n11, assign21200_e29163_d_n12, assign21200_e29163_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21200_e29159: f64 = (locals.var_vbcs_cl__blk643 - p.p287);
        let assign21200_e29160: f64 = (locals.var_beta * assign21200_e29159);
        let assign21200_e29161: f64 = (assign21200_e29160).exp();
        (assign21200_e29161, (assign21200_e29161 * (locals.var_beta * locals.var_vbcs_cl__blk643_dn0)), (assign21200_e29161 * (locals.var_beta * locals.var_vbcs_cl__blk643_dn2)), (assign21200_e29161 * (locals.var_beta * locals.var_vbcs_cl__blk643_dn6)), (assign21200_e29161 * (locals.var_beta * locals.var_vbcs_cl__blk643_dn7)), (assign21200_e29161 * ((locals.var_beta_dn10 * assign21200_e29159) + (locals.var_beta * locals.var_vbcs_cl__blk643_dn10))), (assign21200_e29161 * (locals.var_beta * locals.var_vbcs_cl__blk643_dn11)), (assign21200_e29161 * (locals.var_beta * locals.var_vbcs_cl__blk643_dn12)), (assign21200_e29161 * (locals.var_beta * locals.var_vbcs_cl__blk643_dn17)),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn12, locals.var_exp_bvbsvds_dn17,)
    }
};
        locals.var_exp_bvbsvds = assign21200_e29163;
        locals.var_exp_bvbsvds_dn0 = assign21200_e29163_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign21200_e29163_d_n2;
        locals.var_exp_bvbsvds_dn6 = assign21200_e29163_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign21200_e29163_d_n7;
        locals.var_exp_bvbsvds_dn10 = assign21200_e29163_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign21200_e29163_d_n11;
        locals.var_exp_bvbsvds_dn12 = assign21200_e29163_d_n12;
        locals.var_exp_bvbsvds_dn17 = assign21200_e29163_d_n17;
        locals.var_exp_bvbsvds_rv = 0.0;

        let (assign21210_e29172,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign21210_e29172;
        locals.var_flg_conv_rv = 0.0;

        let (assign21220_e29181, assign21220_e29181_d_n0, assign21220_e29181_d_n2, assign21220_e29181_d_n6, assign21220_e29181_d_n7, assign21220_e29181_d_n10, assign21220_e29181_d_n11, assign21220_e29181_d_n12, assign21220_e29181_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi__blk644, locals.var_phi_s0_soi__blk644_dn0, locals.var_phi_s0_soi__blk644_dn2, locals.var_phi_s0_soi__blk644_dn6, locals.var_phi_s0_soi__blk644_dn7, locals.var_phi_s0_soi__blk644_dn10, locals.var_phi_s0_soi__blk644_dn11, locals.var_phi_s0_soi__blk644_dn12, locals.var_phi_s0_soi__blk644_dn17,)
    }
};
        locals.var_phi_s0_soi__blk644 = assign21220_e29181;
        locals.var_phi_s0_soi__blk644_dn0 = assign21220_e29181_d_n0;
        locals.var_phi_s0_soi__blk644_dn2 = assign21220_e29181_d_n2;
        locals.var_phi_s0_soi__blk644_dn6 = assign21220_e29181_d_n6;
        locals.var_phi_s0_soi__blk644_dn7 = assign21220_e29181_d_n7;
        locals.var_phi_s0_soi__blk644_dn10 = assign21220_e29181_d_n10;
        locals.var_phi_s0_soi__blk644_dn11 = assign21220_e29181_d_n11;
        locals.var_phi_s0_soi__blk644_dn12 = assign21220_e29181_d_n12;
        locals.var_phi_s0_soi__blk644_dn17 = assign21220_e29181_d_n17;
        locals.var_phi_s0_soi__blk644_rv = 0.0;

        let (assign21230_e29198, assign21230_e29198_d_n0, assign21230_e29198_d_n2, assign21230_e29198_d_n6, assign21230_e29198_d_n7, assign21230_e29198_d_n10, assign21230_e29198_d_n11, assign21230_e29198_d_n12, assign21230_e29198_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21230_e29190: f64 = (locals.var_q_nsub * p.p237);
        let assign21230_e29192: f64 = (assign21230_e29190 * p.p237);
        let assign21230_e29194: f64 = (assign21230_e29192 / 2.0);
        let assign21230_e29196: f64 = (assign21230_e29194 / 1.034943e-10);
        (assign21230_e29196, ((((locals.var_q_nsub_dn0 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn12 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn17 * p.p237) * p.p237) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn12, locals.var_dphi_sb_dn17,)
    }
};
        locals.var_dphi_sb = assign21230_e29198;
        locals.var_dphi_sb_dn0 = assign21230_e29198_d_n0;
        locals.var_dphi_sb_dn2 = assign21230_e29198_d_n2;
        locals.var_dphi_sb_dn6 = assign21230_e29198_d_n6;
        locals.var_dphi_sb_dn7 = assign21230_e29198_d_n7;
        locals.var_dphi_sb_dn10 = assign21230_e29198_d_n10;
        locals.var_dphi_sb_dn11 = assign21230_e29198_d_n11;
        locals.var_dphi_sb_dn12 = assign21230_e29198_d_n12;
        locals.var_dphi_sb_dn17 = assign21230_e29198_d_n17;
        locals.var_dphi_sb_rv = 0.0;

        let (assign21240_e29212, assign21240_e29212_d_n0, assign21240_e29212_d_n2, assign21240_e29212_d_n6, assign21240_e29212_d_n7, assign21240_e29212_d_n10, assign21240_e29212_d_n11, assign21240_e29212_d_n12, assign21240_e29212_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21240_e29207: f64 = (2.0 * locals.var_beta);
        let assign21240_e29209: f64 = (assign21240_e29207 * locals.var_dphi_sb);
        let assign21240_e29210: f64 = (assign21240_e29209).sqrt();
        (assign21240_e29210, ((assign21240_e29207 * locals.var_dphi_sb_dn0) / (2.0 * assign21240_e29210)), ((assign21240_e29207 * locals.var_dphi_sb_dn2) / (2.0 * assign21240_e29210)), ((assign21240_e29207 * locals.var_dphi_sb_dn6) / (2.0 * assign21240_e29210)), ((assign21240_e29207 * locals.var_dphi_sb_dn7) / (2.0 * assign21240_e29210)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign21240_e29207 * locals.var_dphi_sb_dn10)) / (2.0 * assign21240_e29210)), ((assign21240_e29207 * locals.var_dphi_sb_dn11) / (2.0 * assign21240_e29210)), ((assign21240_e29207 * locals.var_dphi_sb_dn12) / (2.0 * assign21240_e29210)), ((assign21240_e29207 * locals.var_dphi_sb_dn17) / (2.0 * assign21240_e29210)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign21240_e29212;
        locals.var_t0_dn0 = assign21240_e29212_d_n0;
        locals.var_t0_dn2 = assign21240_e29212_d_n2;
        locals.var_t0_dn6 = assign21240_e29212_d_n6;
        locals.var_t0_dn7 = assign21240_e29212_d_n7;
        locals.var_t0_dn10 = assign21240_e29212_d_n10;
        locals.var_t0_dn11 = assign21240_e29212_d_n11;
        locals.var_t0_dn12 = assign21240_e29212_d_n12;
        locals.var_t0_dn17 = assign21240_e29212_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign21250_e29228, assign21250_e29228_d_n0, assign21250_e29228_d_n2, assign21250_e29228_d_n6, assign21250_e29228_d_n7, assign21250_e29228_d_n10, assign21250_e29228_d_n11, assign21250_e29228_d_n12, assign21250_e29228_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21250_e29220: f64 = (locals.var_t0).exp();
        let assign21250_e29222: f64 = (-locals.var_t0);
        let assign21250_e29223: f64 = (assign21250_e29222).exp();
        let assign21250_e29224: f64 = (assign21250_e29220 + assign21250_e29223);
        let assign21250_e29226: f64 = (assign21250_e29224 / 2.0);
        (assign21250_e29226, (((assign21250_e29220 * locals.var_t0_dn0) + (assign21250_e29223 * (-locals.var_t0_dn0))) / 2.0), (((assign21250_e29220 * locals.var_t0_dn2) + (assign21250_e29223 * (-locals.var_t0_dn2))) / 2.0), (((assign21250_e29220 * locals.var_t0_dn6) + (assign21250_e29223 * (-locals.var_t0_dn6))) / 2.0), (((assign21250_e29220 * locals.var_t0_dn7) + (assign21250_e29223 * (-locals.var_t0_dn7))) / 2.0), (((assign21250_e29220 * locals.var_t0_dn10) + (assign21250_e29223 * (-locals.var_t0_dn10))) / 2.0), (((assign21250_e29220 * locals.var_t0_dn11) + (assign21250_e29223 * (-locals.var_t0_dn11))) / 2.0), (((assign21250_e29220 * locals.var_t0_dn12) + (assign21250_e29223 * (-locals.var_t0_dn12))) / 2.0), (((assign21250_e29220 * locals.var_t0_dn17) + (assign21250_e29223 * (-locals.var_t0_dn17))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21250_e29228;
        locals.var_t1_dn0 = assign21250_e29228_d_n0;
        locals.var_t1_dn2 = assign21250_e29228_d_n2;
        locals.var_t1_dn6 = assign21250_e29228_d_n6;
        locals.var_t1_dn7 = assign21250_e29228_d_n7;
        locals.var_t1_dn10 = assign21250_e29228_d_n10;
        locals.var_t1_dn11 = assign21250_e29228_d_n11;
        locals.var_t1_dn12 = assign21250_e29228_d_n12;
        locals.var_t1_dn17 = assign21250_e29228_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21260_e29240, assign21260_e29240_d_n0, assign21260_e29240_d_n2, assign21260_e29240_d_n6, assign21260_e29240_d_n7, assign21260_e29240_d_n10, assign21260_e29240_d_n11, assign21260_e29240_d_n12, assign21260_e29240_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21260_e29236: f64 = (locals.var_t1).ln();
        let assign21260_e29238: f64 = (assign21260_e29236 / locals.var_dphi_sb);
        (assign21260_e29238, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn12 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn12)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn17 / locals.var_t1) * locals.var_dphi_sb) - (assign21260_e29236 * locals.var_dphi_sb_dn17)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn12, locals.var_c_sb_dn17,)
    }
};
        locals.var_c_sb = assign21260_e29240;
        locals.var_c_sb_dn0 = assign21260_e29240_d_n0;
        locals.var_c_sb_dn2 = assign21260_e29240_d_n2;
        locals.var_c_sb_dn6 = assign21260_e29240_d_n6;
        locals.var_c_sb_dn7 = assign21260_e29240_d_n7;
        locals.var_c_sb_dn10 = assign21260_e29240_d_n10;
        locals.var_c_sb_dn11 = assign21260_e29240_d_n11;
        locals.var_c_sb_dn12 = assign21260_e29240_d_n12;
        locals.var_c_sb_dn17 = assign21260_e29240_d_n17;
        locals.var_c_sb_rv = 0.0;

        let (assign21270_e29249,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign21270_e29249;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign21280_loop_guard: usize = 0;
        while {
            let assign21280_cond_e29259: f64 = (locals.var_lp_s0_max + 1.0);
            let assign21280_cond_e29261: f64 = if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_lp_s0 <= assign21280_cond_e29259)) { 1.0 } else { 0.0 };
            assign21280_cond_e29261 != 0.0
        } {
            assign21280_loop_guard += 1;
            assert!(assign21280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign21280_body0_e29272, assign21280_body0_e29272_d_n0, assign21280_body0_e29272_d_n2, assign21280_body0_e29272_d_n6, assign21280_body0_e29272_d_n7, assign21280_body0_e29272_d_n10, assign21280_body0_e29272_d_n11, assign21280_body0_e29272_d_n12, assign21280_body0_e29272_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21280_body0_e29270: f64 = (locals.var_phi_s0_soi__blk644 - locals.var_vbcs_cl__blk643);
        (assign21280_body0_e29270, (locals.var_phi_s0_soi__blk644_dn0 - locals.var_vbcs_cl__blk643_dn0), (locals.var_phi_s0_soi__blk644_dn2 - locals.var_vbcs_cl__blk643_dn2), (locals.var_phi_s0_soi__blk644_dn6 - locals.var_vbcs_cl__blk643_dn6), (locals.var_phi_s0_soi__blk644_dn7 - locals.var_vbcs_cl__blk643_dn7), (locals.var_phi_s0_soi__blk644_dn10 - locals.var_vbcs_cl__blk643_dn10), (locals.var_phi_s0_soi__blk644_dn11 - locals.var_vbcs_cl__blk643_dn11), (locals.var_phi_s0_soi__blk644_dn12 - locals.var_vbcs_cl__blk643_dn12), (locals.var_phi_s0_soi__blk644_dn17 - locals.var_vbcs_cl__blk643_dn17),)
    } else {
        (locals.var_phi_soi0, locals.var_phi_soi0_dn0, locals.var_phi_soi0_dn2, locals.var_phi_soi0_dn6, locals.var_phi_soi0_dn7, locals.var_phi_soi0_dn10, locals.var_phi_soi0_dn11, locals.var_phi_soi0_dn12, locals.var_phi_soi0_dn17,)
    }
};
            locals.var_phi_soi0 = assign21280_body0_e29272;
            locals.var_phi_soi0_dn0 = assign21280_body0_e29272_d_n0;
            locals.var_phi_soi0_dn2 = assign21280_body0_e29272_d_n2;
            locals.var_phi_soi0_dn6 = assign21280_body0_e29272_d_n6;
            locals.var_phi_soi0_dn7 = assign21280_body0_e29272_d_n7;
            locals.var_phi_soi0_dn10 = assign21280_body0_e29272_d_n10;
            locals.var_phi_soi0_dn11 = assign21280_body0_e29272_d_n11;
            locals.var_phi_soi0_dn12 = assign21280_body0_e29272_d_n12;
            locals.var_phi_soi0_dn17 = assign21280_body0_e29272_d_n17;
            locals.var_phi_soi0_rv = 0.0;
            let (assign21280_body1_e29283, assign21280_body1_e29283_d_n0, assign21280_body1_e29283_d_n2, assign21280_body1_e29283_d_n6, assign21280_body1_e29283_d_n7, assign21280_body1_e29283_d_n10, assign21280_body1_e29283_d_n11, assign21280_body1_e29283_d_n12, assign21280_body1_e29283_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21280_body1_e29281: f64 = (locals.var_beta * locals.var_phi_soi0);
        (assign21280_body1_e29281, (locals.var_beta * locals.var_phi_soi0_dn0), (locals.var_beta * locals.var_phi_soi0_dn2), (locals.var_beta * locals.var_phi_soi0_dn6), (locals.var_beta * locals.var_phi_soi0_dn7), ((locals.var_beta_dn10 * locals.var_phi_soi0) + (locals.var_beta * locals.var_phi_soi0_dn10)), (locals.var_beta * locals.var_phi_soi0_dn11), (locals.var_beta * locals.var_phi_soi0_dn12), (locals.var_beta * locals.var_phi_soi0_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign21280_body1_e29283;
            locals.var_chi_dn0 = assign21280_body1_e29283_d_n0;
            locals.var_chi_dn2 = assign21280_body1_e29283_d_n2;
            locals.var_chi_dn6 = assign21280_body1_e29283_d_n6;
            locals.var_chi_dn7 = assign21280_body1_e29283_d_n7;
            locals.var_chi_dn10 = assign21280_body1_e29283_d_n10;
            locals.var_chi_dn11 = assign21280_body1_e29283_d_n11;
            locals.var_chi_dn12 = assign21280_body1_e29283_d_n12;
            locals.var_chi_dn17 = assign21280_body1_e29283_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign21280_body2_e29296, assign21280_body2_e29296_d_n0, assign21280_body2_e29296_d_n2, assign21280_body2_e29296_d_n6, assign21280_body2_e29296_d_n7, assign21280_body2_e29296_d_n10, assign21280_body2_e29296_d_n11, assign21280_body2_e29296_d_n12, assign21280_body2_e29296_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21280_body2_e29293: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        let assign21280_body2_e29294: f64 = (locals.var_c_sb * assign21280_body2_e29293);
        (assign21280_body2_e29294, ((locals.var_c_sb_dn0 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign21280_body2_e29293) + (locals.var_c_sb * (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign21280_body2_e29296;
            locals.var_ty_dn0 = assign21280_body2_e29296_d_n0;
            locals.var_ty_dn2 = assign21280_body2_e29296_d_n2;
            locals.var_ty_dn6 = assign21280_body2_e29296_d_n6;
            locals.var_ty_dn7 = assign21280_body2_e29296_d_n7;
            locals.var_ty_dn10 = assign21280_body2_e29296_d_n10;
            locals.var_ty_dn11 = assign21280_body2_e29296_d_n11;
            locals.var_ty_dn12 = assign21280_body2_e29296_d_n12;
            locals.var_ty_dn17 = assign21280_body2_e29296_d_n17;
            locals.var_ty_rv = 0.0;
            let assign21280_body3_e29299: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard647 = assign21280_body3_e29299;
            locals.var_guard647_rv = 0.0;
            let (assign21280_body4_e29311, assign21280_body4_e29311_d_n0, assign21280_body4_e29311_d_n2, assign21280_body4_e29311_d_n6, assign21280_body4_e29311_d_n7, assign21280_body4_e29311_d_n10, assign21280_body4_e29311_d_n11, assign21280_body4_e29311_d_n12, assign21280_body4_e29311_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21280_body4_e29309: f64 = (locals.var_ty).exp();
        (assign21280_body4_e29309, (assign21280_body4_e29309 * locals.var_ty_dn0), (assign21280_body4_e29309 * locals.var_ty_dn2), (assign21280_body4_e29309 * locals.var_ty_dn6), (assign21280_body4_e29309 * locals.var_ty_dn7), (assign21280_body4_e29309 * locals.var_ty_dn10), (assign21280_body4_e29309 * locals.var_ty_dn11), (assign21280_body4_e29309 * locals.var_ty_dn12), (assign21280_body4_e29309 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21280_body4_e29311;
            locals.var_t1_dn0 = assign21280_body4_e29311_d_n0;
            locals.var_t1_dn2 = assign21280_body4_e29311_d_n2;
            locals.var_t1_dn6 = assign21280_body4_e29311_d_n6;
            locals.var_t1_dn7 = assign21280_body4_e29311_d_n7;
            locals.var_t1_dn10 = assign21280_body4_e29311_d_n10;
            locals.var_t1_dn11 = assign21280_body4_e29311_d_n11;
            locals.var_t1_dn12 = assign21280_body4_e29311_d_n12;
            locals.var_t1_dn17 = assign21280_body4_e29311_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21280_body5_e29326, assign21280_body5_e29326_d_n0, assign21280_body5_e29326_d_n2, assign21280_body5_e29326_d_n6, assign21280_body5_e29326_d_n7, assign21280_body5_e29326_d_n10, assign21280_body5_e29326_d_n11, assign21280_body5_e29326_d_n12, assign21280_body5_e29326_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21280_body5_e29321: f64 = (-locals.var_c_sb);
        let assign21280_body5_e29323: f64 = (assign21280_body5_e29321 * locals.var_dphi_sb);
        let assign21280_body5_e29324: f64 = (assign21280_body5_e29323).exp();
        (assign21280_body5_e29324, (assign21280_body5_e29324 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn0))), (assign21280_body5_e29324 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn2))), (assign21280_body5_e29324 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn6))), (assign21280_body5_e29324 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn7))), (assign21280_body5_e29324 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn10))), (assign21280_body5_e29324 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn11))), (assign21280_body5_e29324 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn12))), (assign21280_body5_e29324 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign21280_body5_e29321 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21280_body5_e29326;
            locals.var_t0_dn0 = assign21280_body5_e29326_d_n0;
            locals.var_t0_dn2 = assign21280_body5_e29326_d_n2;
            locals.var_t0_dn6 = assign21280_body5_e29326_d_n6;
            locals.var_t0_dn7 = assign21280_body5_e29326_d_n7;
            locals.var_t0_dn10 = assign21280_body5_e29326_d_n10;
            locals.var_t0_dn11 = assign21280_body5_e29326_d_n11;
            locals.var_t0_dn12 = assign21280_body5_e29326_d_n12;
            locals.var_t0_dn17 = assign21280_body5_e29326_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21280_body6_e29339, assign21280_body6_e29339_d_n0, assign21280_body6_e29339_d_n2, assign21280_body6_e29339_d_n6, assign21280_body6_e29339_d_n7, assign21280_body6_e29339_d_n10, assign21280_body6_e29339_d_n11, assign21280_body6_e29339_d_n12, assign21280_body6_e29339_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21280_body6_e29337: f64 = (locals.var_t1 - locals.var_t0);
        (assign21280_body6_e29337, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign21280_body6_e29339;
            locals.var_t2_dn0 = assign21280_body6_e29339_d_n0;
            locals.var_t2_dn2 = assign21280_body6_e29339_d_n2;
            locals.var_t2_dn6 = assign21280_body6_e29339_d_n6;
            locals.var_t2_dn7 = assign21280_body6_e29339_d_n7;
            locals.var_t2_dn10 = assign21280_body6_e29339_d_n10;
            locals.var_t2_dn11 = assign21280_body6_e29339_d_n11;
            locals.var_t2_dn12 = assign21280_body6_e29339_d_n12;
            locals.var_t2_dn17 = assign21280_body6_e29339_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign21280_body7_e29355, assign21280_body7_e29355_d_n0, assign21280_body7_e29355_d_n2, assign21280_body7_e29355_d_n6, assign21280_body7_e29355_d_n7, assign21280_body7_e29355_d_n10, assign21280_body7_e29355_d_n11, assign21280_body7_e29355_d_n12, assign21280_body7_e29355_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21280_body7_e29350: f64 = (1.0 + locals.var_t2);
        let assign21280_body7_e29351: f64 = (assign21280_body7_e29350).ln();
        let assign21280_body7_e29353: f64 = (assign21280_body7_e29351 / locals.var_c_sb);
        (assign21280_body7_e29353, ((((locals.var_t2_dn0 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign21280_body7_e29350) * locals.var_c_sb) - (assign21280_body7_e29351 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign21280_body7_e29355;
            locals.var_phi_soib_dn0 = assign21280_body7_e29355_d_n0;
            locals.var_phi_soib_dn2 = assign21280_body7_e29355_d_n2;
            locals.var_phi_soib_dn6 = assign21280_body7_e29355_d_n6;
            locals.var_phi_soib_dn7 = assign21280_body7_e29355_d_n7;
            locals.var_phi_soib_dn10 = assign21280_body7_e29355_d_n10;
            locals.var_phi_soib_dn11 = assign21280_body7_e29355_d_n11;
            locals.var_phi_soib_dn12 = assign21280_body7_e29355_d_n12;
            locals.var_phi_soib_dn17 = assign21280_body7_e29355_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign21280_body8_e29370, assign21280_body8_e29370_d_n0, assign21280_body8_e29370_d_n2, assign21280_body8_e29370_d_n6, assign21280_body8_e29370_d_n7, assign21280_body8_e29370_d_n10, assign21280_body8_e29370_d_n11, assign21280_body8_e29370_d_n12, assign21280_body8_e29370_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign21280_body8_e29367: f64 = (1.0 + locals.var_t2);
        let assign21280_body8_e29368: f64 = (locals.var_t1 / assign21280_body8_e29367);
        (assign21280_body8_e29368, (((locals.var_t1_dn0 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn0)) / (assign21280_body8_e29367 * assign21280_body8_e29367)), (((locals.var_t1_dn2 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn2)) / (assign21280_body8_e29367 * assign21280_body8_e29367)), (((locals.var_t1_dn6 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn6)) / (assign21280_body8_e29367 * assign21280_body8_e29367)), (((locals.var_t1_dn7 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn7)) / (assign21280_body8_e29367 * assign21280_body8_e29367)), (((locals.var_t1_dn10 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn10)) / (assign21280_body8_e29367 * assign21280_body8_e29367)), (((locals.var_t1_dn11 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn11)) / (assign21280_body8_e29367 * assign21280_body8_e29367)), (((locals.var_t1_dn12 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn12)) / (assign21280_body8_e29367 * assign21280_body8_e29367)), (((locals.var_t1_dn17 * assign21280_body8_e29367) - (locals.var_t1 * locals.var_t2_dn17)) / (assign21280_body8_e29367 * assign21280_body8_e29367)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign21280_body8_e29370;
            locals.var_phi_soib_dpss_dn0 = assign21280_body8_e29370_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign21280_body8_e29370_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign21280_body8_e29370_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign21280_body8_e29370_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign21280_body8_e29370_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign21280_body8_e29370_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign21280_body8_e29370_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign21280_body8_e29370_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign21280_body9_e29384, assign21280_body9_e29384_d_n0, assign21280_body9_e29384_d_n2, assign21280_body9_e29384_d_n6, assign21280_body9_e29384_d_n7, assign21280_body9_e29384_d_n10, assign21280_body9_e29384_d_n11, assign21280_body9_e29384_d_n12, assign21280_body9_e29384_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign21280_body9_e29382: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        (assign21280_body9_e29382, (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign21280_body9_e29384;
            locals.var_phi_soib_dn0 = assign21280_body9_e29384_d_n0;
            locals.var_phi_soib_dn2 = assign21280_body9_e29384_d_n2;
            locals.var_phi_soib_dn6 = assign21280_body9_e29384_d_n6;
            locals.var_phi_soib_dn7 = assign21280_body9_e29384_d_n7;
            locals.var_phi_soib_dn10 = assign21280_body9_e29384_d_n10;
            locals.var_phi_soib_dn11 = assign21280_body9_e29384_d_n11;
            locals.var_phi_soib_dn12 = assign21280_body9_e29384_d_n12;
            locals.var_phi_soib_dn17 = assign21280_body9_e29384_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign21280_body10_e29396, assign21280_body10_e29396_d_n0, assign21280_body10_e29396_d_n2, assign21280_body10_e29396_d_n6, assign21280_body10_e29396_d_n7, assign21280_body10_e29396_d_n10, assign21280_body10_e29396_d_n11, assign21280_body10_e29396_d_n12, assign21280_body10_e29396_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard647 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign21280_body10_e29396;
            locals.var_phi_soib_dpss_dn0 = assign21280_body10_e29396_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign21280_body10_e29396_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign21280_body10_e29396_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign21280_body10_e29396_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign21280_body10_e29396_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign21280_body10_e29396_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign21280_body10_e29396_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign21280_body10_e29396_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign21280_body11_e29407, assign21280_body11_e29407_d_n0, assign21280_body11_e29407_d_n2, assign21280_body11_e29407_d_n6, assign21280_body11_e29407_d_n7, assign21280_body11_e29407_d_n10, assign21280_body11_e29407_d_n11, assign21280_body11_e29407_d_n12, assign21280_body11_e29407_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21280_body11_e29405: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign21280_body11_e29405, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign21280_body11_e29407;
            locals.var_chib_dn0 = assign21280_body11_e29407_d_n0;
            locals.var_chib_dn2 = assign21280_body11_e29407_d_n2;
            locals.var_chib_dn6 = assign21280_body11_e29407_d_n6;
            locals.var_chib_dn7 = assign21280_body11_e29407_d_n7;
            locals.var_chib_dn10 = assign21280_body11_e29407_d_n10;
            locals.var_chib_dn11 = assign21280_body11_e29407_d_n11;
            locals.var_chib_dn12 = assign21280_body11_e29407_d_n12;
            locals.var_chib_dn17 = assign21280_body11_e29407_d_n17;
            locals.var_chib_rv = 0.0;
            let assign21280_body12_e29409: f64 = (locals.var_chi).abs();
            let assign21280_body12_e29411: f64 = if assign21280_body12_e29409 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard648 = assign21280_body12_e29411;
            locals.var_guard648_rv = 0.0;
            let (assign21280_body13_e29429, assign21280_body13_e29429_d_n0, assign21280_body13_e29429_d_n2, assign21280_body13_e29429_d_n6, assign21280_body13_e29429_d_n7, assign21280_body13_e29429_d_n10, assign21280_body13_e29429_d_n11, assign21280_body13_e29429_d_n12, assign21280_body13_e29429_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign21280_body13_e29423: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign21280_body13_e29424: f64 = (1.0 - assign21280_body13_e29423);
        let assign21280_body13_e29426: f64 = (assign21280_body13_e29424 / 2.0);
        let assign21280_body13_e29427: f64 = (assign21280_body13_e29426).sqrt();
        (assign21280_body13_e29427, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign21280_body13_e29427)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign21280_body13_e29427)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign21280_body13_e29427)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign21280_body13_e29427)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign21280_body13_e29427)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign21280_body13_e29427)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign21280_body13_e29427)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign21280_body13_e29427)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21280_body13_e29429;
            locals.var_t0_dn0 = assign21280_body13_e29429_d_n0;
            locals.var_t0_dn2 = assign21280_body13_e29429_d_n2;
            locals.var_t0_dn6 = assign21280_body13_e29429_d_n6;
            locals.var_t0_dn7 = assign21280_body13_e29429_d_n7;
            locals.var_t0_dn10 = assign21280_body13_e29429_d_n10;
            locals.var_t0_dn11 = assign21280_body13_e29429_d_n11;
            locals.var_t0_dn12 = assign21280_body13_e29429_d_n12;
            locals.var_t0_dn17 = assign21280_body13_e29429_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21280_body14_e29442, assign21280_body14_e29442_d_n0, assign21280_body14_e29442_d_n2, assign21280_body14_e29442_d_n6, assign21280_body14_e29442_d_n7, assign21280_body14_e29442_d_n10, assign21280_body14_e29442_d_n11, assign21280_body14_e29442_d_n12, assign21280_body14_e29442_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign21280_body14_e29440: f64 = (locals.var_chi * locals.var_t0);
        (assign21280_body14_e29440, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21280_body14_e29442;
            locals.var_fb_dn0 = assign21280_body14_e29442_d_n0;
            locals.var_fb_dn2 = assign21280_body14_e29442_d_n2;
            locals.var_fb_dn6 = assign21280_body14_e29442_d_n6;
            locals.var_fb_dn7 = assign21280_body14_e29442_d_n7;
            locals.var_fb_dn10 = assign21280_body14_e29442_d_n10;
            locals.var_fb_dn11 = assign21280_body14_e29442_d_n11;
            locals.var_fb_dn12 = assign21280_body14_e29442_d_n12;
            locals.var_fb_dn17 = assign21280_body14_e29442_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21280_body15_e29455, assign21280_body15_e29455_d_n0, assign21280_body15_e29455_d_n2, assign21280_body15_e29455_d_n6, assign21280_body15_e29455_d_n7, assign21280_body15_e29455_d_n10, assign21280_body15_e29455_d_n11, assign21280_body15_e29455_d_n12, assign21280_body15_e29455_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign21280_body15_e29453: f64 = (locals.var_beta * locals.var_t0);
        (assign21280_body15_e29453, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21280_body15_e29455;
            locals.var_fb_dpss_dn0 = assign21280_body15_e29455_d_n0;
            locals.var_fb_dpss_dn2 = assign21280_body15_e29455_d_n2;
            locals.var_fb_dpss_dn6 = assign21280_body15_e29455_d_n6;
            locals.var_fb_dpss_dn7 = assign21280_body15_e29455_d_n7;
            locals.var_fb_dpss_dn10 = assign21280_body15_e29455_d_n10;
            locals.var_fb_dpss_dn11 = assign21280_body15_e29455_d_n11;
            locals.var_fb_dpss_dn12 = assign21280_body15_e29455_d_n12;
            locals.var_fb_dpss_dn17 = assign21280_body15_e29455_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21280_body16_e29458: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard649 = assign21280_body16_e29458;
            locals.var_guard649_rv = 0.0;
            let (assign21280_body17_e29472, assign21280_body17_e29472_d_n0, assign21280_body17_e29472_d_n2, assign21280_body17_e29472_d_n6, assign21280_body17_e29472_d_n7, assign21280_body17_e29472_d_n10, assign21280_body17_e29472_d_n11, assign21280_body17_e29472_d_n12, assign21280_body17_e29472_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21280_body17_e29470: f64 = (-locals.var_fb);
        (assign21280_body17_e29470, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21280_body17_e29472;
            locals.var_fb_dn0 = assign21280_body17_e29472_d_n0;
            locals.var_fb_dn2 = assign21280_body17_e29472_d_n2;
            locals.var_fb_dn6 = assign21280_body17_e29472_d_n6;
            locals.var_fb_dn7 = assign21280_body17_e29472_d_n7;
            locals.var_fb_dn10 = assign21280_body17_e29472_d_n10;
            locals.var_fb_dn11 = assign21280_body17_e29472_d_n11;
            locals.var_fb_dn12 = assign21280_body17_e29472_d_n12;
            locals.var_fb_dn17 = assign21280_body17_e29472_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21280_body18_e29486, assign21280_body18_e29486_d_n0, assign21280_body18_e29486_d_n2, assign21280_body18_e29486_d_n6, assign21280_body18_e29486_d_n7, assign21280_body18_e29486_d_n10, assign21280_body18_e29486_d_n11, assign21280_body18_e29486_d_n12, assign21280_body18_e29486_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign21280_body18_e29484: f64 = (-locals.var_fb_dpss);
        (assign21280_body18_e29484, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21280_body18_e29486;
            locals.var_fb_dpss_dn0 = assign21280_body18_e29486_d_n0;
            locals.var_fb_dpss_dn2 = assign21280_body18_e29486_d_n2;
            locals.var_fb_dpss_dn6 = assign21280_body18_e29486_d_n6;
            locals.var_fb_dpss_dn7 = assign21280_body18_e29486_d_n7;
            locals.var_fb_dpss_dn10 = assign21280_body18_e29486_d_n10;
            locals.var_fb_dpss_dn11 = assign21280_body18_e29486_d_n11;
            locals.var_fb_dpss_dn12 = assign21280_body18_e29486_d_n12;
            locals.var_fb_dpss_dn17 = assign21280_body18_e29486_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21280_body19_e29488: f64 = (locals.var_chi).abs();
            let assign21280_body19_e29490: f64 = if assign21280_body19_e29488 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard650 = assign21280_body19_e29490;
            locals.var_guard650_rv = 0.0;
            let (assign21280_body20_e29526, assign21280_body20_e29526_d_n0, assign21280_body20_e29526_d_n2, assign21280_body20_e29526_d_n6, assign21280_body20_e29526_d_n7, assign21280_body20_e29526_d_n10, assign21280_body20_e29526_d_n11, assign21280_body20_e29526_d_n12, assign21280_body20_e29526_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21280_body20_e29504: f64 = (locals.var_chi * locals.var_chi);
        let assign21280_body20_e29506: f64 = (assign21280_body20_e29504 / 2.0);
        let assign21280_body20_e29510: f64 = (locals.var_chi / 3.0);
        let assign21280_body20_e29514: f64 = (locals.var_chi / 4.0);
        let assign21280_body20_e29518: f64 = (locals.var_chi / 5.0);
        let assign21280_body20_e29519: f64 = (1.0 - assign21280_body20_e29518);
        let assign21280_body20_e29520: f64 = (assign21280_body20_e29514 * assign21280_body20_e29519);
        let assign21280_body20_e29521: f64 = (1.0 - assign21280_body20_e29520);
        let assign21280_body20_e29522: f64 = (assign21280_body20_e29510 * assign21280_body20_e29521);
        let assign21280_body20_e29523: f64 = (1.0 - assign21280_body20_e29522);
        let assign21280_body20_e29524: f64 = (assign21280_body20_e29506 * assign21280_body20_e29523);
        (assign21280_body20_e29524, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn0 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn0 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn2 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn2 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn6 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn6 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn7 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn7 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn10 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn10 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn11 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn11 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn12 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn12 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign21280_body20_e29523) + (assign21280_body20_e29506 * (-(((locals.var_chi_dn17 / 3.0) * assign21280_body20_e29521) + (assign21280_body20_e29510 * (-(((locals.var_chi_dn17 / 4.0) * assign21280_body20_e29519) + (assign21280_body20_e29514 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21280_body20_e29526;
            locals.var_t0_dn0 = assign21280_body20_e29526_d_n0;
            locals.var_t0_dn2 = assign21280_body20_e29526_d_n2;
            locals.var_t0_dn6 = assign21280_body20_e29526_d_n6;
            locals.var_t0_dn7 = assign21280_body20_e29526_d_n7;
            locals.var_t0_dn10 = assign21280_body20_e29526_d_n10;
            locals.var_t0_dn11 = assign21280_body20_e29526_d_n11;
            locals.var_t0_dn12 = assign21280_body20_e29526_d_n12;
            locals.var_t0_dn17 = assign21280_body20_e29526_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21280_body21_e29558, assign21280_body21_e29558_d_n0, assign21280_body21_e29558_d_n2, assign21280_body21_e29558_d_n6, assign21280_body21_e29558_d_n7, assign21280_body21_e29558_d_n10, assign21280_body21_e29558_d_n11, assign21280_body21_e29558_d_n12, assign21280_body21_e29558_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21280_body21_e29542: f64 = (locals.var_chi / 2.0);
        let assign21280_body21_e29546: f64 = (locals.var_chi / 3.0);
        let assign21280_body21_e29550: f64 = (locals.var_chi / 4.0);
        let assign21280_body21_e29551: f64 = (1.0 - assign21280_body21_e29550);
        let assign21280_body21_e29552: f64 = (assign21280_body21_e29546 * assign21280_body21_e29551);
        let assign21280_body21_e29553: f64 = (1.0 - assign21280_body21_e29552);
        let assign21280_body21_e29554: f64 = (assign21280_body21_e29542 * assign21280_body21_e29553);
        let assign21280_body21_e29555: f64 = (1.0 - assign21280_body21_e29554);
        let assign21280_body21_e29556: f64 = (locals.var_chi * assign21280_body21_e29555);
        (assign21280_body21_e29556, ((locals.var_chi_dn0 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn0 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn2 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn6 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn7 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn10 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn11 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn12 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign21280_body21_e29555) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign21280_body21_e29553) + (assign21280_body21_e29542 * (-(((locals.var_chi_dn17 / 3.0) * assign21280_body21_e29551) + (assign21280_body21_e29546 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21280_body21_e29558;
            locals.var_t1_dn0 = assign21280_body21_e29558_d_n0;
            locals.var_t1_dn2 = assign21280_body21_e29558_d_n2;
            locals.var_t1_dn6 = assign21280_body21_e29558_d_n6;
            locals.var_t1_dn7 = assign21280_body21_e29558_d_n7;
            locals.var_t1_dn10 = assign21280_body21_e29558_d_n10;
            locals.var_t1_dn11 = assign21280_body21_e29558_d_n11;
            locals.var_t1_dn12 = assign21280_body21_e29558_d_n12;
            locals.var_t1_dn17 = assign21280_body21_e29558_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21280_body22_e29594, assign21280_body22_e29594_d_n0, assign21280_body22_e29594_d_n2, assign21280_body22_e29594_d_n6, assign21280_body22_e29594_d_n7, assign21280_body22_e29594_d_n10, assign21280_body22_e29594_d_n11, assign21280_body22_e29594_d_n12, assign21280_body22_e29594_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21280_body22_e29572: f64 = (locals.var_chib * locals.var_chib);
        let assign21280_body22_e29574: f64 = (assign21280_body22_e29572 / 2.0);
        let assign21280_body22_e29578: f64 = (locals.var_chib / 3.0);
        let assign21280_body22_e29582: f64 = (locals.var_chib / 4.0);
        let assign21280_body22_e29586: f64 = (locals.var_chib / 5.0);
        let assign21280_body22_e29587: f64 = (1.0 - assign21280_body22_e29586);
        let assign21280_body22_e29588: f64 = (assign21280_body22_e29582 * assign21280_body22_e29587);
        let assign21280_body22_e29589: f64 = (1.0 - assign21280_body22_e29588);
        let assign21280_body22_e29590: f64 = (assign21280_body22_e29578 * assign21280_body22_e29589);
        let assign21280_body22_e29591: f64 = (1.0 - assign21280_body22_e29590);
        let assign21280_body22_e29592: f64 = (assign21280_body22_e29574 * assign21280_body22_e29591);
        (assign21280_body22_e29592, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn0 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn0 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn2 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn2 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn6 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn6 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn7 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn7 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn10 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn10 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn11 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn11 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn12 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn12 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign21280_body22_e29591) + (assign21280_body22_e29574 * (-(((locals.var_chib_dn17 / 3.0) * assign21280_body22_e29589) + (assign21280_body22_e29578 * (-(((locals.var_chib_dn17 / 4.0) * assign21280_body22_e29587) + (assign21280_body22_e29582 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign21280_body22_e29594;
            locals.var_t2_dn0 = assign21280_body22_e29594_d_n0;
            locals.var_t2_dn2 = assign21280_body22_e29594_d_n2;
            locals.var_t2_dn6 = assign21280_body22_e29594_d_n6;
            locals.var_t2_dn7 = assign21280_body22_e29594_d_n7;
            locals.var_t2_dn10 = assign21280_body22_e29594_d_n10;
            locals.var_t2_dn11 = assign21280_body22_e29594_d_n11;
            locals.var_t2_dn12 = assign21280_body22_e29594_d_n12;
            locals.var_t2_dn17 = assign21280_body22_e29594_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign21280_body23_e29626, assign21280_body23_e29626_d_n0, assign21280_body23_e29626_d_n2, assign21280_body23_e29626_d_n6, assign21280_body23_e29626_d_n7, assign21280_body23_e29626_d_n10, assign21280_body23_e29626_d_n11, assign21280_body23_e29626_d_n12, assign21280_body23_e29626_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21280_body23_e29610: f64 = (locals.var_chib / 2.0);
        let assign21280_body23_e29614: f64 = (locals.var_chib / 3.0);
        let assign21280_body23_e29618: f64 = (locals.var_chib / 4.0);
        let assign21280_body23_e29619: f64 = (1.0 - assign21280_body23_e29618);
        let assign21280_body23_e29620: f64 = (assign21280_body23_e29614 * assign21280_body23_e29619);
        let assign21280_body23_e29621: f64 = (1.0 - assign21280_body23_e29620);
        let assign21280_body23_e29622: f64 = (assign21280_body23_e29610 * assign21280_body23_e29621);
        let assign21280_body23_e29623: f64 = (1.0 - assign21280_body23_e29622);
        let assign21280_body23_e29624: f64 = (locals.var_chib * assign21280_body23_e29623);
        (assign21280_body23_e29624, ((locals.var_chib_dn0 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn0 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn2 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn6 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn7 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn10 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn11 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn12 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign21280_body23_e29623) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign21280_body23_e29621) + (assign21280_body23_e29610 * (-(((locals.var_chib_dn17 / 3.0) * assign21280_body23_e29619) + (assign21280_body23_e29614 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign21280_body23_e29626;
            locals.var_t3_dn0 = assign21280_body23_e29626_d_n0;
            locals.var_t3_dn2 = assign21280_body23_e29626_d_n2;
            locals.var_t3_dn6 = assign21280_body23_e29626_d_n6;
            locals.var_t3_dn7 = assign21280_body23_e29626_d_n7;
            locals.var_t3_dn10 = assign21280_body23_e29626_d_n10;
            locals.var_t3_dn11 = assign21280_body23_e29626_d_n11;
            locals.var_t3_dn12 = assign21280_body23_e29626_d_n12;
            locals.var_t3_dn17 = assign21280_body23_e29626_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign21280_body24_e29643, assign21280_body24_e29643_d_n0, assign21280_body24_e29643_d_n2, assign21280_body24_e29643_d_n6, assign21280_body24_e29643_d_n7, assign21280_body24_e29643_d_n10, assign21280_body24_e29643_d_n11, assign21280_body24_e29643_d_n12, assign21280_body24_e29643_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21280_body24_e29640: f64 = (locals.var_t0 - locals.var_t2);
        let assign21280_body24_e29641: f64 = (assign21280_body24_e29640).sqrt();
        (assign21280_body24_e29641, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign21280_body24_e29641)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign21280_body24_e29641)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign21280_body24_e29641)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign21280_body24_e29641)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign21280_body24_e29641)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign21280_body24_e29641)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign21280_body24_e29641)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign21280_body24_e29641)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21280_body24_e29643;
            locals.var_fb_dn0 = assign21280_body24_e29643_d_n0;
            locals.var_fb_dn2 = assign21280_body24_e29643_d_n2;
            locals.var_fb_dn6 = assign21280_body24_e29643_d_n6;
            locals.var_fb_dn7 = assign21280_body24_e29643_d_n7;
            locals.var_fb_dn10 = assign21280_body24_e29643_d_n10;
            locals.var_fb_dn11 = assign21280_body24_e29643_d_n11;
            locals.var_fb_dn12 = assign21280_body24_e29643_d_n12;
            locals.var_fb_dn17 = assign21280_body24_e29643_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21280_body25_e29667, assign21280_body25_e29667_d_n0, assign21280_body25_e29667_d_n2, assign21280_body25_e29667_d_n6, assign21280_body25_e29667_d_n7, assign21280_body25_e29667_d_n10, assign21280_body25_e29667_d_n11, assign21280_body25_e29667_d_n12, assign21280_body25_e29667_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign21280_body25_e29657: f64 = (locals.var_beta * 0.5);
        let assign21280_body25_e29661: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign21280_body25_e29662: f64 = (locals.var_t1 - assign21280_body25_e29661);
        let assign21280_body25_e29663: f64 = (assign21280_body25_e29657 * assign21280_body25_e29662);
        let assign21280_body25_e29665: f64 = (assign21280_body25_e29663 / locals.var_fb);
        (assign21280_body25_e29665, ((((assign21280_body25_e29657 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body25_e29657 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body25_e29657 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body25_e29657 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign21280_body25_e29662) + (assign21280_body25_e29657 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body25_e29657 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body25_e29657 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body25_e29657 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign21280_body25_e29663 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21280_body25_e29667;
            locals.var_fb_dpss_dn0 = assign21280_body25_e29667_d_n0;
            locals.var_fb_dpss_dn2 = assign21280_body25_e29667_d_n2;
            locals.var_fb_dpss_dn6 = assign21280_body25_e29667_d_n6;
            locals.var_fb_dpss_dn7 = assign21280_body25_e29667_d_n7;
            locals.var_fb_dpss_dn10 = assign21280_body25_e29667_d_n10;
            locals.var_fb_dpss_dn11 = assign21280_body25_e29667_d_n11;
            locals.var_fb_dpss_dn12 = assign21280_body25_e29667_d_n12;
            locals.var_fb_dpss_dn17 = assign21280_body25_e29667_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign21280_body26_e29684, assign21280_body26_e29684_d_n0, assign21280_body26_e29684_d_n2, assign21280_body26_e29684_d_n6, assign21280_body26_e29684_d_n7, assign21280_body26_e29684_d_n10, assign21280_body26_e29684_d_n11, assign21280_body26_e29684_d_n12, assign21280_body26_e29684_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign21280_body26_e29681: f64 = (-locals.var_chi);
        let assign21280_body26_e29682: f64 = (assign21280_body26_e29681).exp();
        (assign21280_body26_e29682, (assign21280_body26_e29682 * (-locals.var_chi_dn0)), (assign21280_body26_e29682 * (-locals.var_chi_dn2)), (assign21280_body26_e29682 * (-locals.var_chi_dn6)), (assign21280_body26_e29682 * (-locals.var_chi_dn7)), (assign21280_body26_e29682 * (-locals.var_chi_dn10)), (assign21280_body26_e29682 * (-locals.var_chi_dn11)), (assign21280_body26_e29682 * (-locals.var_chi_dn12)), (assign21280_body26_e29682 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign21280_body26_e29684;
            locals.var_t0_dn0 = assign21280_body26_e29684_d_n0;
            locals.var_t0_dn2 = assign21280_body26_e29684_d_n2;
            locals.var_t0_dn6 = assign21280_body26_e29684_d_n6;
            locals.var_t0_dn7 = assign21280_body26_e29684_d_n7;
            locals.var_t0_dn10 = assign21280_body26_e29684_d_n10;
            locals.var_t0_dn11 = assign21280_body26_e29684_d_n11;
            locals.var_t0_dn12 = assign21280_body26_e29684_d_n12;
            locals.var_t0_dn17 = assign21280_body26_e29684_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign21280_body27_e29701, assign21280_body27_e29701_d_n0, assign21280_body27_e29701_d_n2, assign21280_body27_e29701_d_n6, assign21280_body27_e29701_d_n7, assign21280_body27_e29701_d_n10, assign21280_body27_e29701_d_n11, assign21280_body27_e29701_d_n12, assign21280_body27_e29701_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign21280_body27_e29698: f64 = (-locals.var_chib);
        let assign21280_body27_e29699: f64 = (assign21280_body27_e29698).exp();
        (assign21280_body27_e29699, (assign21280_body27_e29699 * (-locals.var_chib_dn0)), (assign21280_body27_e29699 * (-locals.var_chib_dn2)), (assign21280_body27_e29699 * (-locals.var_chib_dn6)), (assign21280_body27_e29699 * (-locals.var_chib_dn7)), (assign21280_body27_e29699 * (-locals.var_chib_dn10)), (assign21280_body27_e29699 * (-locals.var_chib_dn11)), (assign21280_body27_e29699 * (-locals.var_chib_dn12)), (assign21280_body27_e29699 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign21280_body27_e29701;
            locals.var_t1_dn0 = assign21280_body27_e29701_d_n0;
            locals.var_t1_dn2 = assign21280_body27_e29701_d_n2;
            locals.var_t1_dn6 = assign21280_body27_e29701_d_n6;
            locals.var_t1_dn7 = assign21280_body27_e29701_d_n7;
            locals.var_t1_dn10 = assign21280_body27_e29701_d_n10;
            locals.var_t1_dn11 = assign21280_body27_e29701_d_n11;
            locals.var_t1_dn12 = assign21280_body27_e29701_d_n12;
            locals.var_t1_dn17 = assign21280_body27_e29701_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign21280_body28_e29723, assign21280_body28_e29723_d_n0, assign21280_body28_e29723_d_n2, assign21280_body28_e29723_d_n6, assign21280_body28_e29723_d_n7, assign21280_body28_e29723_d_n10, assign21280_body28_e29723_d_n11, assign21280_body28_e29723_d_n12, assign21280_body28_e29723_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign21280_body28_e29716: f64 = (locals.var_chi - locals.var_chib);
        let assign21280_body28_e29719: f64 = (locals.var_t0 - locals.var_t1);
        let assign21280_body28_e29720: f64 = (assign21280_body28_e29716 + assign21280_body28_e29719);
        let assign21280_body28_e29721: f64 = (assign21280_body28_e29720).sqrt();
        (assign21280_body28_e29721, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign21280_body28_e29721)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign21280_body28_e29721)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign21280_body28_e29721)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign21280_body28_e29721)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign21280_body28_e29721)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign21280_body28_e29721)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign21280_body28_e29721)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign21280_body28_e29721)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign21280_body28_e29723;
            locals.var_fb_dn0 = assign21280_body28_e29723_d_n0;
            locals.var_fb_dn2 = assign21280_body28_e29723_d_n2;
            locals.var_fb_dn6 = assign21280_body28_e29723_d_n6;
            locals.var_fb_dn7 = assign21280_body28_e29723_d_n7;
            locals.var_fb_dn10 = assign21280_body28_e29723_d_n10;
            locals.var_fb_dn11 = assign21280_body28_e29723_d_n11;
            locals.var_fb_dn12 = assign21280_body28_e29723_d_n12;
            locals.var_fb_dn17 = assign21280_body28_e29723_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign21280_body29_e29752, assign21280_body29_e29752_d_n0, assign21280_body29_e29752_d_n2, assign21280_body29_e29752_d_n6, assign21280_body29_e29752_d_n7, assign21280_body29_e29752_d_n10, assign21280_body29_e29752_d_n11, assign21280_body29_e29752_d_n12, assign21280_body29_e29752_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign21280_body29_e29738: f64 = (locals.var_beta * 0.5);
        let assign21280_body29_e29741: f64 = (1.0 - locals.var_t0);
        let assign21280_body29_e29745: f64 = (1.0 - locals.var_t1);
        let assign21280_body29_e29746: f64 = (locals.var_phi_soib_dpss * assign21280_body29_e29745);
        let assign21280_body29_e29747: f64 = (assign21280_body29_e29741 - assign21280_body29_e29746);
        let assign21280_body29_e29748: f64 = (assign21280_body29_e29738 * assign21280_body29_e29747);
        let assign21280_body29_e29750: f64 = (assign21280_body29_e29748 / locals.var_fb);
        (assign21280_body29_e29750, ((((assign21280_body29_e29738 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body29_e29738 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body29_e29738 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body29_e29738 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign21280_body29_e29747) + (assign21280_body29_e29738 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body29_e29738 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body29_e29738 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign21280_body29_e29738 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign21280_body29_e29745) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign21280_body29_e29748 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign21280_body29_e29752;
            locals.var_fb_dpss_dn0 = assign21280_body29_e29752_d_n0;
            locals.var_fb_dpss_dn2 = assign21280_body29_e29752_d_n2;
            locals.var_fb_dpss_dn6 = assign21280_body29_e29752_d_n6;
            locals.var_fb_dpss_dn7 = assign21280_body29_e29752_d_n7;
            locals.var_fb_dpss_dn10 = assign21280_body29_e29752_d_n10;
            locals.var_fb_dpss_dn11 = assign21280_body29_e29752_d_n11;
            locals.var_fb_dpss_dn12 = assign21280_body29_e29752_d_n12;
            locals.var_fb_dpss_dn17 = assign21280_body29_e29752_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign21280_body30_e29759: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard651 = assign21280_body30_e29759;
            locals.var_guard651_rv = 0.0;
            let (assign21280_body31_e29771,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign21280_body31_e29769: f64 = (-1.0);
        (assign21280_body31_e29769,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign21280_body31_e29771;
            locals.var_flg_zone_rv = 0.0;
            let assign21280_body32_e29774: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard652 = assign21280_body32_e29774;
            locals.var_guard652_rv = 0.0;
            let (assign21280_body33_e29786, assign21280_body33_e29786_d_n0, assign21280_body33_e29786_d_n2, assign21280_body33_e29786_d_n6, assign21280_body33_e29786_d_n7, assign21280_body33_e29786_d_n10, assign21280_body33_e29786_d_n11, assign21280_body33_e29786_d_n12, assign21280_body33_e29786_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21280_body33_e29784: f64 = (-locals.var_fb);
        (assign21280_body33_e29784, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21280_body33_e29786;
            locals.var_fs02_dn0 = assign21280_body33_e29786_d_n0;
            locals.var_fs02_dn2 = assign21280_body33_e29786_d_n2;
            locals.var_fs02_dn6 = assign21280_body33_e29786_d_n6;
            locals.var_fs02_dn7 = assign21280_body33_e29786_d_n7;
            locals.var_fs02_dn10 = assign21280_body33_e29786_d_n10;
            locals.var_fs02_dn11 = assign21280_body33_e29786_d_n11;
            locals.var_fs02_dn12 = assign21280_body33_e29786_d_n12;
            locals.var_fs02_dn17 = assign21280_body33_e29786_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21280_body34_e29798, assign21280_body34_e29798_d_n0, assign21280_body34_e29798_d_n2, assign21280_body34_e29798_d_n6, assign21280_body34_e29798_d_n7, assign21280_body34_e29798_d_n10, assign21280_body34_e29798_d_n11, assign21280_body34_e29798_d_n12, assign21280_body34_e29798_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign21280_body34_e29796: f64 = (-locals.var_fb_dpss);
        (assign21280_body34_e29796, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21280_body34_e29798;
            locals.var_fs02_dps0_dn0 = assign21280_body34_e29798_d_n0;
            locals.var_fs02_dps0_dn2 = assign21280_body34_e29798_d_n2;
            locals.var_fs02_dps0_dn6 = assign21280_body34_e29798_d_n6;
            locals.var_fs02_dps0_dn7 = assign21280_body34_e29798_d_n7;
            locals.var_fs02_dps0_dn10 = assign21280_body34_e29798_d_n10;
            locals.var_fs02_dps0_dn11 = assign21280_body34_e29798_d_n11;
            locals.var_fs02_dps0_dn12 = assign21280_body34_e29798_d_n12;
            locals.var_fs02_dps0_dn17 = assign21280_body34_e29798_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign21280_body35_e29801: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard653 = assign21280_body35_e29801;
            locals.var_guard653_rv = 0.0;
            let (assign21280_body36_e29815, assign21280_body36_e29815_d_n0, assign21280_body36_e29815_d_n2, assign21280_body36_e29815_d_n6, assign21280_body36_e29815_d_n7, assign21280_body36_e29815_d_n10, assign21280_body36_e29815_d_n11, assign21280_body36_e29815_d_n12, assign21280_body36_e29815_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21280_body36_e29815;
            locals.var_fs02_dn0 = assign21280_body36_e29815_d_n0;
            locals.var_fs02_dn2 = assign21280_body36_e29815_d_n2;
            locals.var_fs02_dn6 = assign21280_body36_e29815_d_n6;
            locals.var_fs02_dn7 = assign21280_body36_e29815_d_n7;
            locals.var_fs02_dn10 = assign21280_body36_e29815_d_n10;
            locals.var_fs02_dn11 = assign21280_body36_e29815_d_n11;
            locals.var_fs02_dn12 = assign21280_body36_e29815_d_n12;
            locals.var_fs02_dn17 = assign21280_body36_e29815_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21280_body37_e29829, assign21280_body37_e29829_d_n0, assign21280_body37_e29829_d_n2, assign21280_body37_e29829_d_n6, assign21280_body37_e29829_d_n7, assign21280_body37_e29829_d_n10, assign21280_body37_e29829_d_n11, assign21280_body37_e29829_d_n12, assign21280_body37_e29829_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21280_body37_e29829;
            locals.var_fs02_dps0_dn0 = assign21280_body37_e29829_d_n0;
            locals.var_fs02_dps0_dn2 = assign21280_body37_e29829_d_n2;
            locals.var_fs02_dps0_dn6 = assign21280_body37_e29829_d_n6;
            locals.var_fs02_dps0_dn7 = assign21280_body37_e29829_d_n7;
            locals.var_fs02_dps0_dn10 = assign21280_body37_e29829_d_n10;
            locals.var_fs02_dps0_dn11 = assign21280_body37_e29829_d_n11;
            locals.var_fs02_dps0_dn12 = assign21280_body37_e29829_d_n12;
            locals.var_fs02_dps0_dn17 = assign21280_body37_e29829_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign21280_body38_e29848, assign21280_body38_e29848_d_n0, assign21280_body38_e29848_d_n2, assign21280_body38_e29848_d_n6, assign21280_body38_e29848_d_n7, assign21280_body38_e29848_d_n10, assign21280_body38_e29848_d_n11, assign21280_body38_e29848_d_n12, assign21280_body38_e29848_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign21280_body38_e29845: f64 = (locals.var_phi_s0_soi__blk644 - p.p287);
        let assign21280_body38_e29846: f64 = (locals.var_beta * assign21280_body38_e29845);
        (assign21280_body38_e29846, (locals.var_beta * locals.var_phi_s0_soi__blk644_dn0), (locals.var_beta * locals.var_phi_s0_soi__blk644_dn2), (locals.var_beta * locals.var_phi_s0_soi__blk644_dn6), (locals.var_beta * locals.var_phi_s0_soi__blk644_dn7), ((locals.var_beta_dn10 * assign21280_body38_e29845) + (locals.var_beta * locals.var_phi_s0_soi__blk644_dn10)), (locals.var_beta * locals.var_phi_s0_soi__blk644_dn11), (locals.var_beta * locals.var_phi_s0_soi__blk644_dn12), (locals.var_beta * locals.var_phi_s0_soi__blk644_dn17),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn17,)
    }
};
            locals.var_rho = assign21280_body38_e29848;
            locals.var_rho_dn0 = assign21280_body38_e29848_d_n0;
            locals.var_rho_dn2 = assign21280_body38_e29848_d_n2;
            locals.var_rho_dn6 = assign21280_body38_e29848_d_n6;
            locals.var_rho_dn7 = assign21280_body38_e29848_d_n7;
            locals.var_rho_dn10 = assign21280_body38_e29848_d_n10;
            locals.var_rho_dn11 = assign21280_body38_e29848_d_n11;
            locals.var_rho_dn12 = assign21280_body38_e29848_d_n12;
            locals.var_rho_dn17 = assign21280_body38_e29848_d_n17;
            locals.var_rho_rv = 0.0;
            let (assign21280_body39_e29864, assign21280_body39_e29864_d_n0, assign21280_body39_e29864_d_n2, assign21280_body39_e29864_d_n6, assign21280_body39_e29864_d_n7, assign21280_body39_e29864_d_n10, assign21280_body39_e29864_d_n11, assign21280_body39_e29864_d_n12, assign21280_body39_e29864_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign21280_body39_e29862: f64 = (locals.var_rho).exp();
        (assign21280_body39_e29862, (assign21280_body39_e29862 * locals.var_rho_dn0), (assign21280_body39_e29862 * locals.var_rho_dn2), (assign21280_body39_e29862 * locals.var_rho_dn6), (assign21280_body39_e29862 * locals.var_rho_dn7), (assign21280_body39_e29862 * locals.var_rho_dn10), (assign21280_body39_e29862 * locals.var_rho_dn11), (assign21280_body39_e29862 * locals.var_rho_dn12), (assign21280_body39_e29862 * locals.var_rho_dn17),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn12, locals.var_exp_rho_dn17,)
    }
};
            locals.var_exp_rho = assign21280_body39_e29864;
            locals.var_exp_rho_dn0 = assign21280_body39_e29864_d_n0;
            locals.var_exp_rho_dn2 = assign21280_body39_e29864_d_n2;
            locals.var_exp_rho_dn6 = assign21280_body39_e29864_d_n6;
            locals.var_exp_rho_dn7 = assign21280_body39_e29864_d_n7;
            locals.var_exp_rho_dn10 = assign21280_body39_e29864_d_n10;
            locals.var_exp_rho_dn11 = assign21280_body39_e29864_d_n11;
            locals.var_exp_rho_dn12 = assign21280_body39_e29864_d_n12;
            locals.var_exp_rho_dn17 = assign21280_body39_e29864_d_n17;
            locals.var_exp_rho_rv = 0.0;
            let (assign21280_body40_e29887, assign21280_body40_e29887_d_n0, assign21280_body40_e29887_d_n2, assign21280_body40_e29887_d_n6, assign21280_body40_e29887_d_n7, assign21280_body40_e29887_d_n10, assign21280_body40_e29887_d_n11, assign21280_body40_e29887_d_n12, assign21280_body40_e29887_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign21280_body40_e29882: f64 = (locals.var_chi + 1.0);
        let assign21280_body40_e29883: f64 = (locals.var_exp_bvbsvds * assign21280_body40_e29882);
        let assign21280_body40_e29884: f64 = (locals.var_exp_rho - assign21280_body40_e29883);
        let assign21280_body40_e29885: f64 = (locals.var_cnst1soi * assign21280_body40_e29884);
        (assign21280_body40_e29885, ((locals.var_cnst1soi_dn0 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn12 - ((locals.var_exp_bvbsvds_dn12 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign21280_body40_e29884) + (locals.var_cnst1soi * (locals.var_exp_rho_dn17 - ((locals.var_exp_bvbsvds_dn17 * assign21280_body40_e29882) + (locals.var_exp_bvbsvds * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign21280_body40_e29887;
            locals.var_fs01_dn0 = assign21280_body40_e29887_d_n0;
            locals.var_fs01_dn2 = assign21280_body40_e29887_d_n2;
            locals.var_fs01_dn6 = assign21280_body40_e29887_d_n6;
            locals.var_fs01_dn7 = assign21280_body40_e29887_d_n7;
            locals.var_fs01_dn10 = assign21280_body40_e29887_d_n10;
            locals.var_fs01_dn11 = assign21280_body40_e29887_d_n11;
            locals.var_fs01_dn12 = assign21280_body40_e29887_d_n12;
            locals.var_fs01_dn17 = assign21280_body40_e29887_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign21280_body41_e29908, assign21280_body41_e29908_d_n0, assign21280_body41_e29908_d_n2, assign21280_body41_e29908_d_n6, assign21280_body41_e29908_d_n7, assign21280_body41_e29908_d_n10, assign21280_body41_e29908_d_n11, assign21280_body41_e29908_d_n12, assign21280_body41_e29908_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign21280_body41_e29902: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign21280_body41_e29905: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign21280_body41_e29906: f64 = (assign21280_body41_e29902 * assign21280_body41_e29905);
        (assign21280_body41_e29906, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn12 - locals.var_exp_bvbsvds_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign21280_body41_e29905) + (assign21280_body41_e29902 * (locals.var_exp_rho_dn17 - locals.var_exp_bvbsvds_dn17))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign21280_body41_e29908;
            locals.var_fs01_dps0_dn0 = assign21280_body41_e29908_d_n0;
            locals.var_fs01_dps0_dn2 = assign21280_body41_e29908_d_n2;
            locals.var_fs01_dps0_dn6 = assign21280_body41_e29908_d_n6;
            locals.var_fs01_dps0_dn7 = assign21280_body41_e29908_d_n7;
            locals.var_fs01_dps0_dn10 = assign21280_body41_e29908_d_n10;
            locals.var_fs01_dps0_dn11 = assign21280_body41_e29908_d_n11;
            locals.var_fs01_dps0_dn12 = assign21280_body41_e29908_d_n12;
            locals.var_fs01_dps0_dn17 = assign21280_body41_e29908_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign21280_body42_e29928, assign21280_body42_e29928_d_n0, assign21280_body42_e29928_d_n2, assign21280_body42_e29928_d_n6, assign21280_body42_e29928_d_n7, assign21280_body42_e29928_d_n10, assign21280_body42_e29928_d_n11, assign21280_body42_e29928_d_n12, assign21280_body42_e29928_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign21280_body42_e29923: f64 = (locals.var_fb * locals.var_fb);
        let assign21280_body42_e29925: f64 = (assign21280_body42_e29923 + locals.var_fs01);
        let assign21280_body42_e29926: f64 = (assign21280_body42_e29925).sqrt();
        (assign21280_body42_e29926, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign21280_body42_e29926)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign21280_body42_e29926)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign21280_body42_e29926)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign21280_body42_e29926)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign21280_body42_e29926)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign21280_body42_e29926)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign21280_body42_e29926)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fs01_dn17) / (2.0 * assign21280_body42_e29926)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign21280_body42_e29928;
            locals.var_fs02_dn0 = assign21280_body42_e29928_d_n0;
            locals.var_fs02_dn2 = assign21280_body42_e29928_d_n2;
            locals.var_fs02_dn6 = assign21280_body42_e29928_d_n6;
            locals.var_fs02_dn7 = assign21280_body42_e29928_d_n7;
            locals.var_fs02_dn10 = assign21280_body42_e29928_d_n10;
            locals.var_fs02_dn11 = assign21280_body42_e29928_d_n11;
            locals.var_fs02_dn12 = assign21280_body42_e29928_d_n12;
            locals.var_fs02_dn17 = assign21280_body42_e29928_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign21280_body43_e29953, assign21280_body43_e29953_d_n0, assign21280_body43_e29953_d_n2, assign21280_body43_e29953_d_n6, assign21280_body43_e29953_d_n7, assign21280_body43_e29953_d_n10, assign21280_body43_e29953_d_n11, assign21280_body43_e29953_d_n12, assign21280_body43_e29953_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard652 == 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign21280_body43_e29944: f64 = (2.0 * locals.var_fb_dpss);
        let assign21280_body43_e29946: f64 = (assign21280_body43_e29944 * locals.var_fb);
        let assign21280_body43_e29948: f64 = (assign21280_body43_e29946 + locals.var_fs01_dps0);
        let assign21280_body43_e29949: f64 = (0.5 * assign21280_body43_e29948);
        let assign21280_body43_e29951: f64 = (assign21280_body43_e29949 / locals.var_fs02);
        (assign21280_body43_e29951, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign21280_body43_e29944 * locals.var_fb_dn17)) + locals.var_fs01_dps0_dn17)) * locals.var_fs02) - (assign21280_body43_e29949 * locals.var_fs02_dn17)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign21280_body43_e29953;
            locals.var_fs02_dps0_dn0 = assign21280_body43_e29953_d_n0;
            locals.var_fs02_dps0_dn2 = assign21280_body43_e29953_d_n2;
            locals.var_fs02_dps0_dn6 = assign21280_body43_e29953_d_n6;
            locals.var_fs02_dps0_dn7 = assign21280_body43_e29953_d_n7;
            locals.var_fs02_dps0_dn10 = assign21280_body43_e29953_d_n10;
            locals.var_fs02_dps0_dn11 = assign21280_body43_e29953_d_n11;
            locals.var_fs02_dps0_dn12 = assign21280_body43_e29953_d_n12;
            locals.var_fs02_dps0_dn17 = assign21280_body43_e29953_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign21280_body44_e29969, assign21280_body44_e29969_d_n0, assign21280_body44_e29969_d_n2, assign21280_body44_e29969_d_n6, assign21280_body44_e29969_d_n7, assign21280_body44_e29969_d_n10, assign21280_body44_e29969_d_n11, assign21280_body44_e29969_d_n12, assign21280_body44_e29969_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21280_body44_e29961: f64 = (-locals.var_vgp__blk608);
        let assign21280_body44_e29963: f64 = (assign21280_body44_e29961 + locals.var_phi_s0_soi__blk644);
        let assign21280_body44_e29966: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign21280_body44_e29967: f64 = (assign21280_body44_e29963 + assign21280_body44_e29966);
        (assign21280_body44_e29967, (((-locals.var_vgp__blk608_dn0) + locals.var_phi_s0_soi__blk644_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgp__blk608_dn2) + locals.var_phi_s0_soi__blk644_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (((-locals.var_vgp__blk608_dn6) + locals.var_phi_s0_soi__blk644_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgp__blk608_dn7) + locals.var_phi_s0_soi__blk644_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgp__blk608_dn10) + locals.var_phi_s0_soi__blk644_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (((-locals.var_vgp__blk608_dn11) + locals.var_phi_s0_soi__blk644_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (((-locals.var_vgp__blk608_dn12) + locals.var_phi_s0_soi__blk644_dn12) + ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))), (((-locals.var_vgp__blk608_dn17) + locals.var_phi_s0_soi__blk644_dn17) + ((locals.var_fac1_dn17 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn17))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, locals.var_fs0_dn17,)
    }
};
            locals.var_fs0 = assign21280_body44_e29969;
            locals.var_fs0_dn0 = assign21280_body44_e29969_d_n0;
            locals.var_fs0_dn2 = assign21280_body44_e29969_d_n2;
            locals.var_fs0_dn6 = assign21280_body44_e29969_d_n6;
            locals.var_fs0_dn7 = assign21280_body44_e29969_d_n7;
            locals.var_fs0_dn10 = assign21280_body44_e29969_d_n10;
            locals.var_fs0_dn11 = assign21280_body44_e29969_d_n11;
            locals.var_fs0_dn12 = assign21280_body44_e29969_d_n12;
            locals.var_fs0_dn17 = assign21280_body44_e29969_d_n17;
            locals.var_fs0_rv = 0.0;
            let (assign21280_body45_e29982, assign21280_body45_e29982_d_n0, assign21280_body45_e29982_d_n2, assign21280_body45_e29982_d_n6, assign21280_body45_e29982_d_n7, assign21280_body45_e29982_d_n10, assign21280_body45_e29982_d_n11, assign21280_body45_e29982_d_n12, assign21280_body45_e29982_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21280_body45_e29979: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign21280_body45_e29980: f64 = (1.0 + assign21280_body45_e29979);
        (assign21280_body45_e29980, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12)), ((locals.var_fac1_dn17 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn17)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, locals.var_fs0_dps0_dn17,)
    }
};
            locals.var_fs0_dps0 = assign21280_body45_e29982;
            locals.var_fs0_dps0_dn0 = assign21280_body45_e29982_d_n0;
            locals.var_fs0_dps0_dn2 = assign21280_body45_e29982_d_n2;
            locals.var_fs0_dps0_dn6 = assign21280_body45_e29982_d_n6;
            locals.var_fs0_dps0_dn7 = assign21280_body45_e29982_d_n7;
            locals.var_fs0_dps0_dn10 = assign21280_body45_e29982_d_n10;
            locals.var_fs0_dps0_dn11 = assign21280_body45_e29982_d_n11;
            locals.var_fs0_dps0_dn12 = assign21280_body45_e29982_d_n12;
            locals.var_fs0_dps0_dn17 = assign21280_body45_e29982_d_n17;
            locals.var_fs0_dps0_rv = 0.0;
            let assign21280_body46_e29985: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard654 = assign21280_body46_e29985;
            locals.var_guard654_rv = 0.0;
            let (assign21280_body47_e29998,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign21280_body47_e29996: f64 = (locals.var_lp_s0_max + 1.0);
        (assign21280_body47_e29996,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign21280_body47_e29998;
            locals.var_lp_s0_rv = 0.0;
            let (assign21280_body48_e30013, assign21280_body48_e30013_d_n0, assign21280_body48_e30013_d_n2, assign21280_body48_e30013_d_n6, assign21280_body48_e30013_d_n7, assign21280_body48_e30013_d_n10, assign21280_body48_e30013_d_n11, assign21280_body48_e30013_d_n12, assign21280_body48_e30013_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign21280_body48_e30009: f64 = (-locals.var_fs0);
        let assign21280_body48_e30011: f64 = (assign21280_body48_e30009 / locals.var_fs0_dps0);
        (assign21280_body48_e30011, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn17) * locals.var_fs0_dps0) - (assign21280_body48_e30009 * locals.var_fs0_dps0_dn17)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign21280_body48_e30013;
            locals.var_dps0_dn0 = assign21280_body48_e30013_d_n0;
            locals.var_dps0_dn2 = assign21280_body48_e30013_d_n2;
            locals.var_dps0_dn6 = assign21280_body48_e30013_d_n6;
            locals.var_dps0_dn7 = assign21280_body48_e30013_d_n7;
            locals.var_dps0_dn10 = assign21280_body48_e30013_d_n10;
            locals.var_dps0_dn11 = assign21280_body48_e30013_d_n11;
            locals.var_dps0_dn12 = assign21280_body48_e30013_d_n12;
            locals.var_dps0_dn17 = assign21280_body48_e30013_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign21280_body49_e30038, assign21280_body49_e30038_d_n0, assign21280_body49_e30038_d_n2, assign21280_body49_e30038_d_n6, assign21280_body49_e30038_d_n7, assign21280_body49_e30038_d_n10, assign21280_body49_e30038_d_n11, assign21280_body49_e30038_d_n12, assign21280_body49_e30038_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign21280_body49_e30025: f64 = (0.5 * 0.1);
        let assign21280_body49_e30029: f64 = (locals.var_phi_s0_soi__blk644).abs();
        let (assign21280_body49_e30034, assign21280_body49_e30034_d_n0, assign21280_body49_e30034_d_n2, assign21280_body49_e30034_d_n6, assign21280_body49_e30034_d_n7, assign21280_body49_e30034_d_n10, assign21280_body49_e30034_d_n11, assign21280_body49_e30034_d_n12, assign21280_body49_e30034_d_n17,) = {
            if (1.0 >= assign21280_body49_e30029) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign21280_body49_e30033: f64 = (locals.var_phi_s0_soi__blk644).abs();
                (assign21280_body49_e30033, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn0 } else { (-locals.var_phi_s0_soi__blk644_dn0) }, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn2 } else { (-locals.var_phi_s0_soi__blk644_dn2) }, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn6 } else { (-locals.var_phi_s0_soi__blk644_dn6) }, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn7 } else { (-locals.var_phi_s0_soi__blk644_dn7) }, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn10 } else { (-locals.var_phi_s0_soi__blk644_dn10) }, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn11 } else { (-locals.var_phi_s0_soi__blk644_dn11) }, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn12 } else { (-locals.var_phi_s0_soi__blk644_dn12) }, if locals.var_phi_s0_soi__blk644 >= 0.0 { locals.var_phi_s0_soi__blk644_dn17 } else { (-locals.var_phi_s0_soi__blk644_dn17) },)
            }
        };
        let assign21280_body49_e30035: f64 = (1.0 + assign21280_body49_e30034);
        let assign21280_body49_e30036: f64 = (assign21280_body49_e30025 * assign21280_body49_e30035);
        (assign21280_body49_e30036, (assign21280_body49_e30025 * assign21280_body49_e30034_d_n0), (assign21280_body49_e30025 * assign21280_body49_e30034_d_n2), (assign21280_body49_e30025 * assign21280_body49_e30034_d_n6), (assign21280_body49_e30025 * assign21280_body49_e30034_d_n7), (assign21280_body49_e30025 * assign21280_body49_e30034_d_n10), (assign21280_body49_e30025 * assign21280_body49_e30034_d_n11), (assign21280_body49_e30025 * assign21280_body49_e30034_d_n12), (assign21280_body49_e30025 * assign21280_body49_e30034_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign21280_body49_e30038;
            locals.var_dplim_dn0 = assign21280_body49_e30038_d_n0;
            locals.var_dplim_dn2 = assign21280_body49_e30038_d_n2;
            locals.var_dplim_dn6 = assign21280_body49_e30038_d_n6;
            locals.var_dplim_dn7 = assign21280_body49_e30038_d_n7;
            locals.var_dplim_dn10 = assign21280_body49_e30038_d_n10;
            locals.var_dplim_dn11 = assign21280_body49_e30038_d_n11;
            locals.var_dplim_dn12 = assign21280_body49_e30038_d_n12;
            locals.var_dplim_dn17 = assign21280_body49_e30038_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign21280_body50_e30040: f64 = (locals.var_dps0).abs();
            let assign21280_body50_e30042: f64 = if assign21280_body50_e30040 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard655 = assign21280_body50_e30042;
            locals.var_guard655_rv = 0.0;
            let (assign21280_body51_e30064, assign21280_body51_e30064_d_n0, assign21280_body51_e30064_d_n2, assign21280_body51_e30064_d_n6, assign21280_body51_e30064_d_n7, assign21280_body51_e30064_d_n10, assign21280_body51_e30064_d_n11, assign21280_body51_e30064_d_n12, assign21280_body51_e30064_d_n17,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 != 0.0)) {
        let (assign21280_body51_e30061,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign21280_body51_e30060: f64 = (-1.0);
                (assign21280_body51_e30060,)
            }
        };
        let assign21280_body51_e30062: f64 = (locals.var_dplim * assign21280_body51_e30061);
        (assign21280_body51_e30062, (locals.var_dplim_dn0 * assign21280_body51_e30061), (locals.var_dplim_dn2 * assign21280_body51_e30061), (locals.var_dplim_dn6 * assign21280_body51_e30061), (locals.var_dplim_dn7 * assign21280_body51_e30061), (locals.var_dplim_dn10 * assign21280_body51_e30061), (locals.var_dplim_dn11 * assign21280_body51_e30061), (locals.var_dplim_dn12 * assign21280_body51_e30061), (locals.var_dplim_dn17 * assign21280_body51_e30061),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign21280_body51_e30064;
            locals.var_dps0_dn0 = assign21280_body51_e30064_d_n0;
            locals.var_dps0_dn2 = assign21280_body51_e30064_d_n2;
            locals.var_dps0_dn6 = assign21280_body51_e30064_d_n6;
            locals.var_dps0_dn7 = assign21280_body51_e30064_d_n7;
            locals.var_dps0_dn10 = assign21280_body51_e30064_d_n10;
            locals.var_dps0_dn11 = assign21280_body51_e30064_d_n11;
            locals.var_dps0_dn12 = assign21280_body51_e30064_d_n12;
            locals.var_dps0_dn17 = assign21280_body51_e30064_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign21280_body52_e30078, assign21280_body52_e30078_d_n0, assign21280_body52_e30078_d_n2, assign21280_body52_e30078_d_n6, assign21280_body52_e30078_d_n7, assign21280_body52_e30078_d_n10, assign21280_body52_e30078_d_n11, assign21280_body52_e30078_d_n12, assign21280_body52_e30078_d_n17,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign21280_body52_e30076: f64 = (locals.var_phi_s0_soi__blk644 + locals.var_dps0);
        (assign21280_body52_e30076, (locals.var_phi_s0_soi__blk644_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_soi__blk644_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_soi__blk644_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_soi__blk644_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_soi__blk644_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_soi__blk644_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_soi__blk644_dn12 + locals.var_dps0_dn12), (locals.var_phi_s0_soi__blk644_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_phi_s0_soi__blk644, locals.var_phi_s0_soi__blk644_dn0, locals.var_phi_s0_soi__blk644_dn2, locals.var_phi_s0_soi__blk644_dn6, locals.var_phi_s0_soi__blk644_dn7, locals.var_phi_s0_soi__blk644_dn10, locals.var_phi_s0_soi__blk644_dn11, locals.var_phi_s0_soi__blk644_dn12, locals.var_phi_s0_soi__blk644_dn17,)
    }
};
            locals.var_phi_s0_soi__blk644 = assign21280_body52_e30078;
            locals.var_phi_s0_soi__blk644_dn0 = assign21280_body52_e30078_d_n0;
            locals.var_phi_s0_soi__blk644_dn2 = assign21280_body52_e30078_d_n2;
            locals.var_phi_s0_soi__blk644_dn6 = assign21280_body52_e30078_d_n6;
            locals.var_phi_s0_soi__blk644_dn7 = assign21280_body52_e30078_d_n7;
            locals.var_phi_s0_soi__blk644_dn10 = assign21280_body52_e30078_d_n10;
            locals.var_phi_s0_soi__blk644_dn11 = assign21280_body52_e30078_d_n11;
            locals.var_phi_s0_soi__blk644_dn12 = assign21280_body52_e30078_d_n12;
            locals.var_phi_s0_soi__blk644_dn17 = assign21280_body52_e30078_d_n17;
            locals.var_phi_s0_soi__blk644_rv = 0.0;
            let assign21280_body53_e30080: f64 = (locals.var_dps0).abs();
            let assign21280_body53_e30084: f64 = (locals.var_fs0).abs();
            let assign21280_body53_e30087: f64 = if ((assign21280_body53_e30080 <= 5e-12) && (assign21280_body53_e30084 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard656 = assign21280_body53_e30087;
            locals.var_guard656_rv = 0.0;
            let (assign21280_body54_e30101,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard656 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign21280_body54_e30101;
            locals.var_flg_conv_rv = 0.0;
            let (assign21280_body55_e30112,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        let assign21280_body55_e30110: f64 = (locals.var_lp_s0 + 1.0);
        (assign21280_body55_e30110,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign21280_body55_e30112;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign21290_e30121, assign21290_e30121_d_n0, assign21290_e30121_d_n2, assign21290_e30121_d_n6, assign21290_e30121_d_n7, assign21290_e30121_d_n10, assign21290_e30121_d_n11, assign21290_e30121_d_n12, assign21290_e30121_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard645 == 0.0)) {
        (locals.var_phi_s0_soi__blk644, locals.var_phi_s0_soi__blk644_dn0, locals.var_phi_s0_soi__blk644_dn2, locals.var_phi_s0_soi__blk644_dn6, locals.var_phi_s0_soi__blk644_dn7, locals.var_phi_s0_soi__blk644_dn10, locals.var_phi_s0_soi__blk644_dn11, locals.var_phi_s0_soi__blk644_dn12, locals.var_phi_s0_soi__blk644_dn17,)
    } else {
        (locals.var_ps0__blk606, locals.var_ps0__blk606_dn0, locals.var_ps0__blk606_dn2, locals.var_ps0__blk606_dn6, locals.var_ps0__blk606_dn7, locals.var_ps0__blk606_dn10, locals.var_ps0__blk606_dn11, locals.var_ps0__blk606_dn12, locals.var_ps0__blk606_dn17,)
    }
};
        locals.var_ps0__blk606 = assign21290_e30121;
        locals.var_ps0__blk606_dn0 = assign21290_e30121_d_n0;
        locals.var_ps0__blk606_dn2 = assign21290_e30121_d_n2;
        locals.var_ps0__blk606_dn6 = assign21290_e30121_d_n6;
        locals.var_ps0__blk606_dn7 = assign21290_e30121_d_n7;
        locals.var_ps0__blk606_dn10 = assign21290_e30121_d_n10;
        locals.var_ps0__blk606_dn11 = assign21290_e30121_d_n11;
        locals.var_ps0__blk606_dn12 = assign21290_e30121_d_n12;
        locals.var_ps0__blk606_dn17 = assign21290_e30121_d_n17;
        locals.var_ps0__blk606_rv = 0.0;

        let (assign21300_e30130, assign21300_e30130_d_n0, assign21300_e30130_d_n2, assign21300_e30130_d_n6, assign21300_e30130_d_n7, assign21300_e30130_d_n10, assign21300_e30130_d_n11, assign21300_e30130_d_n12, assign21300_e30130_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21300_e30124: f64 = (-locals.var_beta);
        let assign21300_e30127: f64 = (locals.var_ps0__blk606 - locals.var_dphi_vds);
        let assign21300_e30128: f64 = (assign21300_e30124 * assign21300_e30127);
        (assign21300_e30128, (assign21300_e30124 * (locals.var_ps0__blk606_dn0 - locals.var_dphi_vds_dn0)), (assign21300_e30124 * (locals.var_ps0__blk606_dn2 - locals.var_dphi_vds_dn2)), (assign21300_e30124 * (locals.var_ps0__blk606_dn6 - locals.var_dphi_vds_dn6)), (assign21300_e30124 * (locals.var_ps0__blk606_dn7 - locals.var_dphi_vds_dn7)), (((-locals.var_beta_dn10) * assign21300_e30127) + (assign21300_e30124 * (locals.var_ps0__blk606_dn10 - locals.var_dphi_vds_dn10))), (assign21300_e30124 * (locals.var_ps0__blk606_dn11 - locals.var_dphi_vds_dn11)), (assign21300_e30124 * (locals.var_ps0__blk606_dn12 - locals.var_dphi_vds_dn12)), (assign21300_e30124 * (locals.var_ps0__blk606_dn17 - locals.var_dphi_vds_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign21300_e30130;
        locals.var_t5_dn0 = assign21300_e30130_d_n0;
        locals.var_t5_dn2 = assign21300_e30130_d_n2;
        locals.var_t5_dn6 = assign21300_e30130_d_n6;
        locals.var_t5_dn7 = assign21300_e30130_d_n7;
        locals.var_t5_dn10 = assign21300_e30130_d_n10;
        locals.var_t5_dn11 = assign21300_e30130_d_n11;
        locals.var_t5_dn12 = assign21300_e30130_d_n12;
        locals.var_t5_dn17 = assign21300_e30130_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign21310_e30140,) = {
    if (locals.var_guard596 != 0.0) {
        let (assign21310_e30138,) = {
            if (locals.var_t5 >= 0.0) {
                (1.0,)
            } else {
                let assign21310_e30137: f64 = (-1.0);
                (assign21310_e30137,)
            }
        };
        (assign21310_e30138,)
    } else {
        (locals.var_t5sign,)
    }
};
        locals.var_t5sign = assign21310_e30140;
        locals.var_t5sign_rv = 0.0;

        let (assign21320_e30146, assign21320_e30146_d_n0, assign21320_e30146_d_n2, assign21320_e30146_d_n6, assign21320_e30146_d_n7, assign21320_e30146_d_n10, assign21320_e30146_d_n11, assign21320_e30146_d_n12, assign21320_e30146_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21320_e30144: f64 = (locals.var_t5sign * locals.var_t5);
        (assign21320_e30144, (locals.var_t5sign * locals.var_t5_dn0), (locals.var_t5sign * locals.var_t5_dn2), (locals.var_t5sign * locals.var_t5_dn6), (locals.var_t5sign * locals.var_t5_dn7), (locals.var_t5sign * locals.var_t5_dn10), (locals.var_t5sign * locals.var_t5_dn11), (locals.var_t5sign * locals.var_t5_dn12), (locals.var_t5sign * locals.var_t5_dn17),)
    } else {
        (locals.var_t5y, locals.var_t5y_dn0, locals.var_t5y_dn2, locals.var_t5y_dn6, locals.var_t5y_dn7, locals.var_t5y_dn10, locals.var_t5y_dn11, locals.var_t5y_dn12, locals.var_t5y_dn17,)
    }
};
        locals.var_t5y = assign21320_e30146;
        locals.var_t5y_dn0 = assign21320_e30146_d_n0;
        locals.var_t5y_dn2 = assign21320_e30146_d_n2;
        locals.var_t5y_dn6 = assign21320_e30146_d_n6;
        locals.var_t5y_dn7 = assign21320_e30146_d_n7;
        locals.var_t5y_dn10 = assign21320_e30146_d_n10;
        locals.var_t5y_dn11 = assign21320_e30146_d_n11;
        locals.var_t5y_dn12 = assign21320_e30146_d_n12;
        locals.var_t5y_dn17 = assign21320_e30146_d_n17;
        locals.var_t5y_rv = 0.0;

        let (assign21330_e30151, assign21330_e30151_d_n0, assign21330_e30151_d_n2, assign21330_e30151_d_n6, assign21330_e30151_d_n7, assign21330_e30151_d_n10, assign21330_e30151_d_n11, assign21330_e30151_d_n12, assign21330_e30151_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21330_e30149: f64 = (locals.var_t5).exp();
        (assign21330_e30149, (assign21330_e30149 * locals.var_t5_dn0), (assign21330_e30149 * locals.var_t5_dn2), (assign21330_e30149 * locals.var_t5_dn6), (assign21330_e30149 * locals.var_t5_dn7), (assign21330_e30149 * locals.var_t5_dn10), (assign21330_e30149 * locals.var_t5_dn11), (assign21330_e30149 * locals.var_t5_dn12), (assign21330_e30149 * locals.var_t5_dn17),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign21330_e30151;
        locals.var_t6_dn0 = assign21330_e30151_d_n0;
        locals.var_t6_dn2 = assign21330_e30151_d_n2;
        locals.var_t6_dn6 = assign21330_e30151_d_n6;
        locals.var_t6_dn7 = assign21330_e30151_d_n7;
        locals.var_t6_dn10 = assign21330_e30151_d_n10;
        locals.var_t6_dn11 = assign21330_e30151_d_n11;
        locals.var_t6_dn12 = assign21330_e30151_d_n12;
        locals.var_t6_dn17 = assign21330_e30151_d_n17;
        locals.var_t6_rv = 0.0;

        let (assign21340_e30159, assign21340_e30159_d_n0, assign21340_e30159_d_n2, assign21340_e30159_d_n6, assign21340_e30159_d_n7, assign21340_e30159_d_n10, assign21340_e30159_d_n11, assign21340_e30159_d_n12, assign21340_e30159_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21340_e30155: f64 = (locals.var_t6 - 1.0);
        let assign21340_e30157: f64 = (assign21340_e30155 - locals.var_t5);
        (assign21340_e30157, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn11 - locals.var_t5_dn11), (locals.var_t6_dn12 - locals.var_t5_dn12), (locals.var_t6_dn17 - locals.var_t5_dn17),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn17,)
    }
};
        locals.var_t7 = assign21340_e30159;
        locals.var_t7_dn0 = assign21340_e30159_d_n0;
        locals.var_t7_dn2 = assign21340_e30159_d_n2;
        locals.var_t7_dn6 = assign21340_e30159_d_n6;
        locals.var_t7_dn7 = assign21340_e30159_d_n7;
        locals.var_t7_dn10 = assign21340_e30159_d_n10;
        locals.var_t7_dn11 = assign21340_e30159_d_n11;
        locals.var_t7_dn12 = assign21340_e30159_d_n12;
        locals.var_t7_dn17 = assign21340_e30159_d_n17;
        locals.var_t7_rv = 0.0;

        let assign21350_e30162: f64 = if locals.var_t5 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign21350_e30162;
        locals.var_guard657_rv = 0.0;

        let (assign21360_e30172, assign21360_e30172_d_n0, assign21360_e30172_d_n2, assign21360_e30172_d_n6, assign21360_e30172_d_n7, assign21360_e30172_d_n10, assign21360_e30172_d_n11, assign21360_e30172_d_n12, assign21360_e30172_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard657 != 0.0)) {
        let assign21360_e30167: f64 = (-locals.var_cnst0soi);
        let assign21360_e30169: f64 = (locals.var_t7).sqrt();
        let assign21360_e30170: f64 = (assign21360_e30167 * assign21360_e30169);
        (assign21360_e30170, (((-locals.var_cnst0soi_dn0) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn0 / (2.0 * assign21360_e30169)))), (((-locals.var_cnst0soi_dn2) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn2 / (2.0 * assign21360_e30169)))), (((-locals.var_cnst0soi_dn6) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn6 / (2.0 * assign21360_e30169)))), (((-locals.var_cnst0soi_dn7) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn7 / (2.0 * assign21360_e30169)))), (((-locals.var_cnst0soi_dn10) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn10 / (2.0 * assign21360_e30169)))), (((-locals.var_cnst0soi_dn11) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn11 / (2.0 * assign21360_e30169)))), (((-locals.var_cnst0soi_dn12) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn12 / (2.0 * assign21360_e30169)))), (((-locals.var_cnst0soi_dn17) * assign21360_e30169) + (assign21360_e30167 * (locals.var_t7_dn17 / (2.0 * assign21360_e30169)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21360_e30172;
        locals.var_qbu_dn0 = assign21360_e30172_d_n0;
        locals.var_qbu_dn2 = assign21360_e30172_d_n2;
        locals.var_qbu_dn6 = assign21360_e30172_d_n6;
        locals.var_qbu_dn7 = assign21360_e30172_d_n7;
        locals.var_qbu_dn10 = assign21360_e30172_d_n10;
        locals.var_qbu_dn11 = assign21360_e30172_d_n11;
        locals.var_qbu_dn12 = assign21360_e30172_d_n12;
        locals.var_qbu_dn17 = assign21360_e30172_d_n17;
        locals.var_qbu_rv = 0.0;

        let assign21370_e30175: f64 = if locals.var_t5y > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign21370_e30175;
        locals.var_guard658_rv = 0.0;

        let (assign21380_e30187, assign21380_e30187_d_n0, assign21380_e30187_d_n2, assign21380_e30187_d_n6, assign21380_e30187_d_n7, assign21380_e30187_d_n10, assign21380_e30187_d_n11, assign21380_e30187_d_n12, assign21380_e30187_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign21380_e30184: f64 = (locals.var_t7).sqrt();
        let assign21380_e30185: f64 = (locals.var_cnst0soi * assign21380_e30184);
        (assign21380_e30185, ((locals.var_cnst0soi_dn0 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn0 / (2.0 * assign21380_e30184)))), ((locals.var_cnst0soi_dn2 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn2 / (2.0 * assign21380_e30184)))), ((locals.var_cnst0soi_dn6 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn6 / (2.0 * assign21380_e30184)))), ((locals.var_cnst0soi_dn7 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn7 / (2.0 * assign21380_e30184)))), ((locals.var_cnst0soi_dn10 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn10 / (2.0 * assign21380_e30184)))), ((locals.var_cnst0soi_dn11 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn11 / (2.0 * assign21380_e30184)))), ((locals.var_cnst0soi_dn12 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn12 / (2.0 * assign21380_e30184)))), ((locals.var_cnst0soi_dn17 * assign21380_e30184) + (locals.var_cnst0soi * (locals.var_t7_dn17 / (2.0 * assign21380_e30184)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21380_e30187;
        locals.var_qbu_dn0 = assign21380_e30187_d_n0;
        locals.var_qbu_dn2 = assign21380_e30187_d_n2;
        locals.var_qbu_dn6 = assign21380_e30187_d_n6;
        locals.var_qbu_dn7 = assign21380_e30187_d_n7;
        locals.var_qbu_dn10 = assign21380_e30187_d_n10;
        locals.var_qbu_dn11 = assign21380_e30187_d_n11;
        locals.var_qbu_dn12 = assign21380_e30187_d_n12;
        locals.var_qbu_dn17 = assign21380_e30187_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign21390_e30215, assign21390_e30215_d_n0, assign21390_e30215_d_n2, assign21390_e30215_d_n6, assign21390_e30215_d_n7, assign21390_e30215_d_n10, assign21390_e30215_d_n11, assign21390_e30215_d_n12, assign21390_e30215_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 == 0.0)) {
        let assign21390_e30196: f64 = (-locals.var_t5sign);
        let assign21390_e30198: f64 = (assign21390_e30196 * locals.var_t5y);
        let assign21390_e30200: f64 = (assign21390_e30198 * 0.7071067811865475);
        let assign21390_e30204: f64 = (locals.var_t5y * 0.3333333333333333);
        let assign21390_e30208: f64 = (0.25 * locals.var_t5y);
        let assign21390_e30209: f64 = (1.0 + assign21390_e30208);
        let assign21390_e30210: f64 = (assign21390_e30204 * assign21390_e30209);
        let assign21390_e30211: f64 = (1.0 + assign21390_e30210);
        let assign21390_e30212: f64 = (assign21390_e30211).sqrt();
        let assign21390_e30213: f64 = (assign21390_e30200 * assign21390_e30212);
        (assign21390_e30213, ((((assign21390_e30196 * locals.var_t5y_dn0) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn0 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn0))) / (2.0 * assign21390_e30212)))), ((((assign21390_e30196 * locals.var_t5y_dn2) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn2 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn2))) / (2.0 * assign21390_e30212)))), ((((assign21390_e30196 * locals.var_t5y_dn6) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn6 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn6))) / (2.0 * assign21390_e30212)))), ((((assign21390_e30196 * locals.var_t5y_dn7) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn7 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn7))) / (2.0 * assign21390_e30212)))), ((((assign21390_e30196 * locals.var_t5y_dn10) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn10 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn10))) / (2.0 * assign21390_e30212)))), ((((assign21390_e30196 * locals.var_t5y_dn11) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn11 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn11))) / (2.0 * assign21390_e30212)))), ((((assign21390_e30196 * locals.var_t5y_dn12) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn12 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn12))) / (2.0 * assign21390_e30212)))), ((((assign21390_e30196 * locals.var_t5y_dn17) * 0.7071067811865475) * assign21390_e30212) + (assign21390_e30200 * ((((locals.var_t5y_dn17 * 0.3333333333333333) * assign21390_e30209) + (assign21390_e30204 * (0.25 * locals.var_t5y_dn17))) / (2.0 * assign21390_e30212)))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign21390_e30215;
        locals.var_qbu_dn0 = assign21390_e30215_d_n0;
        locals.var_qbu_dn2 = assign21390_e30215_d_n2;
        locals.var_qbu_dn6 = assign21390_e30215_d_n6;
        locals.var_qbu_dn7 = assign21390_e30215_d_n7;
        locals.var_qbu_dn10 = assign21390_e30215_d_n10;
        locals.var_qbu_dn11 = assign21390_e30215_d_n11;
        locals.var_qbu_dn12 = assign21390_e30215_d_n12;
        locals.var_qbu_dn17 = assign21390_e30215_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign21400_e30228, assign21400_e30228_d_n0, assign21400_e30228_d_n2, assign21400_e30228_d_n6, assign21400_e30228_d_n7, assign21400_e30228_d_n10, assign21400_e30228_d_n11, assign21400_e30228_d_n12, assign21400_e30228_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21400_e30219: f64 = (locals.var_qbu * locals.var_qbu);
        let assign21400_e30222: f64 = (4.0 * 1e-6);
        let assign21400_e30224: f64 = (assign21400_e30222 * 1e-6);
        let assign21400_e30225: f64 = (assign21400_e30219 + assign21400_e30224);
        let assign21400_e30226: f64 = (assign21400_e30225).sqrt();
        (assign21400_e30226, (((locals.var_qbu_dn0 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn0)) / (2.0 * assign21400_e30226)), (((locals.var_qbu_dn2 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn2)) / (2.0 * assign21400_e30226)), (((locals.var_qbu_dn6 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn6)) / (2.0 * assign21400_e30226)), (((locals.var_qbu_dn7 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn7)) / (2.0 * assign21400_e30226)), (((locals.var_qbu_dn10 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn10)) / (2.0 * assign21400_e30226)), (((locals.var_qbu_dn11 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn11)) / (2.0 * assign21400_e30226)), (((locals.var_qbu_dn12 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn12)) / (2.0 * assign21400_e30226)), (((locals.var_qbu_dn17 * locals.var_qbu) + (locals.var_qbu * locals.var_qbu_dn17)) / (2.0 * assign21400_e30226)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21400_e30228;
        locals.var_tmf1_dn0 = assign21400_e30228_d_n0;
        locals.var_tmf1_dn2 = assign21400_e30228_d_n2;
        locals.var_tmf1_dn6 = assign21400_e30228_d_n6;
        locals.var_tmf1_dn7 = assign21400_e30228_d_n7;
        locals.var_tmf1_dn10 = assign21400_e30228_d_n10;
        locals.var_tmf1_dn11 = assign21400_e30228_d_n11;
        locals.var_tmf1_dn12 = assign21400_e30228_d_n12;
        locals.var_tmf1_dn17 = assign21400_e30228_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21410_e30240, assign21410_e30240_d_n0, assign21410_e30240_d_n2, assign21410_e30240_d_n6, assign21410_e30240_d_n7, assign21410_e30240_d_n10, assign21410_e30240_d_n11, assign21410_e30240_d_n12, assign21410_e30240_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21410_e30233: f64 = (locals.var_qbu + locals.var_tmf1);
        let assign21410_e30234: f64 = (0.5 * assign21410_e30233);
        let assign21410_e30237: f64 = (1e-10 * 1e-6);
        let assign21410_e30238: f64 = (assign21410_e30234 + assign21410_e30237);
        (assign21410_e30238, (0.5 * (locals.var_qbu_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_qbu_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_qbu_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_qbu_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_qbu_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_qbu_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_qbu_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_qbu_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn12, locals.var_wqbu_dn17,)
    }
};
        locals.var_wqbu = assign21410_e30240;
        locals.var_wqbu_dn0 = assign21410_e30240_d_n0;
        locals.var_wqbu_dn2 = assign21410_e30240_d_n2;
        locals.var_wqbu_dn6 = assign21410_e30240_d_n6;
        locals.var_wqbu_dn7 = assign21410_e30240_d_n7;
        locals.var_wqbu_dn10 = assign21410_e30240_d_n10;
        locals.var_wqbu_dn11 = assign21410_e30240_d_n11;
        locals.var_wqbu_dn12 = assign21410_e30240_d_n12;
        locals.var_wqbu_dn17 = assign21410_e30240_d_n17;
        locals.var_wqbu_rv = 0.0;

        let assign21420_e30243: f64 = if locals.var_wqbu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign21420_e30243;
        locals.var_guard659_rv = 0.0;

        let (assign21430_e30249, assign21430_e30249_d_n0, assign21430_e30249_d_n2, assign21430_e30249_d_n6, assign21430_e30249_d_n7, assign21430_e30249_d_n10, assign21430_e30249_d_n11, assign21430_e30249_d_n12, assign21430_e30249_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard659 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn12, locals.var_wqbu_dn17,)
    }
};
        locals.var_wqbu = assign21430_e30249;
        locals.var_wqbu_dn0 = assign21430_e30249_d_n0;
        locals.var_wqbu_dn2 = assign21430_e30249_d_n2;
        locals.var_wqbu_dn6 = assign21430_e30249_d_n6;
        locals.var_wqbu_dn7 = assign21430_e30249_d_n7;
        locals.var_wqbu_dn10 = assign21430_e30249_d_n10;
        locals.var_wqbu_dn11 = assign21430_e30249_d_n11;
        locals.var_wqbu_dn12 = assign21430_e30249_d_n12;
        locals.var_wqbu_dn17 = assign21430_e30249_d_n17;
        locals.var_wqbu_rv = 0.0;

        let (assign21440_e30257, assign21440_e30257_d_n0, assign21440_e30257_d_n2, assign21440_e30257_d_n6, assign21440_e30257_d_n7, assign21440_e30257_d_n10, assign21440_e30257_d_n11, assign21440_e30257_d_n12, assign21440_e30257_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21440_e30254: f64 = (1.6021918e-19 * locals.var_nsub);
        let assign21440_e30255: f64 = (locals.var_wqbu / assign21440_e30254);
        (assign21440_e30255, (((locals.var_wqbu_dn0 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn0))) / (assign21440_e30254 * assign21440_e30254)), (((locals.var_wqbu_dn2 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn2))) / (assign21440_e30254 * assign21440_e30254)), (((locals.var_wqbu_dn6 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn6))) / (assign21440_e30254 * assign21440_e30254)), (((locals.var_wqbu_dn7 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn7))) / (assign21440_e30254 * assign21440_e30254)), (((locals.var_wqbu_dn10 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn10))) / (assign21440_e30254 * assign21440_e30254)), (((locals.var_wqbu_dn11 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn11))) / (assign21440_e30254 * assign21440_e30254)), (((locals.var_wqbu_dn12 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn12))) / (assign21440_e30254 * assign21440_e30254)), (((locals.var_wqbu_dn17 * assign21440_e30254) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn17))) / (assign21440_e30254 * assign21440_e30254)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn12, locals.var_wdep_dn17,)
    }
};
        locals.var_wdep = assign21440_e30257;
        locals.var_wdep_dn0 = assign21440_e30257_d_n0;
        locals.var_wdep_dn2 = assign21440_e30257_d_n2;
        locals.var_wdep_dn6 = assign21440_e30257_d_n6;
        locals.var_wdep_dn7 = assign21440_e30257_d_n7;
        locals.var_wdep_dn10 = assign21440_e30257_d_n10;
        locals.var_wdep_dn11 = assign21440_e30257_d_n11;
        locals.var_wdep_dn12 = assign21440_e30257_d_n12;
        locals.var_wdep_dn17 = assign21440_e30257_d_n17;
        locals.var_wdep_rv = 0.0;

        let (assign21450_e30263, assign21450_e30263_d_n0, assign21450_e30263_d_n2, assign21450_e30263_d_n6, assign21450_e30263_d_n7, assign21450_e30263_d_n10, assign21450_e30263_d_n11, assign21450_e30263_d_n12, assign21450_e30263_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21450_e30261: f64 = (locals.var_wdep - locals.var_wk_xj);
        (assign21450_e30261, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn12, locals.var_wdep_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21450_e30263;
        locals.var_t1_dn0 = assign21450_e30263_d_n0;
        locals.var_t1_dn2 = assign21450_e30263_d_n2;
        locals.var_t1_dn6 = assign21450_e30263_d_n6;
        locals.var_t1_dn7 = assign21450_e30263_d_n7;
        locals.var_t1_dn10 = assign21450_e30263_d_n10;
        locals.var_t1_dn11 = assign21450_e30263_d_n11;
        locals.var_t1_dn12 = assign21450_e30263_d_n12;
        locals.var_t1_dn17 = assign21450_e30263_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21460_e30269, assign21460_e30269_d_n0, assign21460_e30269_d_n2, assign21460_e30269_d_n6, assign21460_e30269_d_n7, assign21460_e30269_d_n10, assign21460_e30269_d_n11, assign21460_e30269_d_n12, assign21460_e30269_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21460_e30267: f64 = (locals.var_wdep * 0.01);
        (assign21460_e30267, (locals.var_wdep_dn0 * 0.01), (locals.var_wdep_dn2 * 0.01), (locals.var_wdep_dn6 * 0.01), (locals.var_wdep_dn7 * 0.01), (locals.var_wdep_dn10 * 0.01), (locals.var_wdep_dn11 * 0.01), (locals.var_wdep_dn12 * 0.01), (locals.var_wdep_dn17 * 0.01),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn12, locals.var_delta_dn17,)
    }
};
        locals.var_delta = assign21460_e30269;
        locals.var_delta_dn0 = assign21460_e30269_d_n0;
        locals.var_delta_dn2 = assign21460_e30269_d_n2;
        locals.var_delta_dn6 = assign21460_e30269_d_n6;
        locals.var_delta_dn7 = assign21460_e30269_d_n7;
        locals.var_delta_dn10 = assign21460_e30269_d_n10;
        locals.var_delta_dn11 = assign21460_e30269_d_n11;
        locals.var_delta_dn12 = assign21460_e30269_d_n12;
        locals.var_delta_dn17 = assign21460_e30269_d_n17;
        locals.var_delta_rv = 0.0;

        let (assign21470_e30282, assign21470_e30282_d_n0, assign21470_e30282_d_n2, assign21470_e30282_d_n6, assign21470_e30282_d_n7, assign21470_e30282_d_n10, assign21470_e30282_d_n11, assign21470_e30282_d_n12, assign21470_e30282_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21470_e30273: f64 = (locals.var_t1 * locals.var_t1);
        let assign21470_e30276: f64 = (4.0 * locals.var_delta);
        let assign21470_e30278: f64 = (assign21470_e30276 * locals.var_delta);
        let assign21470_e30279: f64 = (assign21470_e30273 + assign21470_e30278);
        let assign21470_e30280: f64 = (assign21470_e30279).sqrt();
        (assign21470_e30280, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn0))) / (2.0 * assign21470_e30280)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn2))) / (2.0 * assign21470_e30280)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn6))) / (2.0 * assign21470_e30280)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn7))) / (2.0 * assign21470_e30280)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn10))) / (2.0 * assign21470_e30280)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn11))) / (2.0 * assign21470_e30280)), ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + (((4.0 * locals.var_delta_dn12) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn12))) / (2.0 * assign21470_e30280)), ((((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) + (((4.0 * locals.var_delta_dn17) * locals.var_delta) + (assign21470_e30276 * locals.var_delta_dn17))) / (2.0 * assign21470_e30280)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21470_e30282;
        locals.var_tmf1_dn0 = assign21470_e30282_d_n0;
        locals.var_tmf1_dn2 = assign21470_e30282_d_n2;
        locals.var_tmf1_dn6 = assign21470_e30282_d_n6;
        locals.var_tmf1_dn7 = assign21470_e30282_d_n7;
        locals.var_tmf1_dn10 = assign21470_e30282_d_n10;
        locals.var_tmf1_dn11 = assign21470_e30282_d_n11;
        locals.var_tmf1_dn12 = assign21470_e30282_d_n12;
        locals.var_tmf1_dn17 = assign21470_e30282_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21480_e30294, assign21480_e30294_d_n0, assign21480_e30294_d_n2, assign21480_e30294_d_n6, assign21480_e30294_d_n7, assign21480_e30294_d_n10, assign21480_e30294_d_n11, assign21480_e30294_d_n12, assign21480_e30294_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21480_e30287: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign21480_e30288: f64 = (0.5 * assign21480_e30287);
        let assign21480_e30291: f64 = (1e-10 * locals.var_delta);
        let assign21480_e30292: f64 = (assign21480_e30288 + assign21480_e30291);
        (assign21480_e30292, ((0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0)) + (1e-10 * locals.var_delta_dn0)), ((0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2)) + (1e-10 * locals.var_delta_dn2)), ((0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6)) + (1e-10 * locals.var_delta_dn6)), ((0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7)) + (1e-10 * locals.var_delta_dn7)), ((0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10)) + (1e-10 * locals.var_delta_dn10)), ((0.5 * (locals.var_t1_dn11 + locals.var_tmf1_dn11)) + (1e-10 * locals.var_delta_dn11)), ((0.5 * (locals.var_t1_dn12 + locals.var_tmf1_dn12)) + (1e-10 * locals.var_delta_dn12)), ((0.5 * (locals.var_t1_dn17 + locals.var_tmf1_dn17)) + (1e-10 * locals.var_delta_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21480_e30294;
        locals.var_t2_dn0 = assign21480_e30294_d_n0;
        locals.var_t2_dn2 = assign21480_e30294_d_n2;
        locals.var_t2_dn6 = assign21480_e30294_d_n6;
        locals.var_t2_dn7 = assign21480_e30294_d_n7;
        locals.var_t2_dn10 = assign21480_e30294_d_n10;
        locals.var_t2_dn11 = assign21480_e30294_d_n11;
        locals.var_t2_dn12 = assign21480_e30294_d_n12;
        locals.var_t2_dn17 = assign21480_e30294_d_n17;
        locals.var_t2_rv = 0.0;

        let assign21490_e30297: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign21490_e30297;
        locals.var_guard660_rv = 0.0;

        let (assign21500_e30303, assign21500_e30303_d_n0, assign21500_e30303_d_n2, assign21500_e30303_d_n6, assign21500_e30303_d_n7, assign21500_e30303_d_n10, assign21500_e30303_d_n11, assign21500_e30303_d_n12, assign21500_e30303_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard660 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign21500_e30303;
        locals.var_t2_dn0 = assign21500_e30303_d_n0;
        locals.var_t2_dn2 = assign21500_e30303_d_n2;
        locals.var_t2_dn6 = assign21500_e30303_d_n6;
        locals.var_t2_dn7 = assign21500_e30303_d_n7;
        locals.var_t2_dn10 = assign21500_e30303_d_n10;
        locals.var_t2_dn11 = assign21500_e30303_d_n11;
        locals.var_t2_dn12 = assign21500_e30303_d_n12;
        locals.var_t2_dn17 = assign21500_e30303_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign21510_e30313, assign21510_e30313_d_n0, assign21510_e30313_d_n2, assign21510_e30313_d_n6, assign21510_e30313_d_n7, assign21510_e30313_d_n10, assign21510_e30313_d_n11, assign21510_e30313_d_n12, assign21510_e30313_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21510_e30307: f64 = (locals.var_t2 / locals.var_wdep);
        let assign21510_e30309: f64 = (assign21510_e30307 * locals.var_t2);
        let assign21510_e30311: f64 = (assign21510_e30309 / locals.var_wdep);
        (assign21510_e30311, ((((((((locals.var_t2_dn0 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn0)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn0)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn0)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn2 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn2)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn2)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn2)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn6 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn6)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn6)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn6)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn7 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn7)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn7)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn7)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn10 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn10)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn10)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn10)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn11 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn11)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn11)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn11)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn12 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn12)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn12)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn12)) / (locals.var_wdep * locals.var_wdep)), ((((((((locals.var_t2_dn17 * locals.var_wdep) - (locals.var_t2 * locals.var_wdep_dn17)) / (locals.var_wdep * locals.var_wdep)) * locals.var_t2) + (assign21510_e30307 * locals.var_t2_dn17)) * locals.var_wdep) - (assign21510_e30309 * locals.var_wdep_dn17)) / (locals.var_wdep * locals.var_wdep)),)
    } else {
        (locals.var_wfactor, locals.var_wfactor_dn0, locals.var_wfactor_dn2, locals.var_wfactor_dn6, locals.var_wfactor_dn7, locals.var_wfactor_dn10, locals.var_wfactor_dn11, locals.var_wfactor_dn12, locals.var_wfactor_dn17,)
    }
};
        locals.var_wfactor = assign21510_e30313;
        locals.var_wfactor_dn0 = assign21510_e30313_d_n0;
        locals.var_wfactor_dn2 = assign21510_e30313_d_n2;
        locals.var_wfactor_dn6 = assign21510_e30313_d_n6;
        locals.var_wfactor_dn7 = assign21510_e30313_d_n7;
        locals.var_wfactor_dn10 = assign21510_e30313_d_n10;
        locals.var_wfactor_dn11 = assign21510_e30313_d_n11;
        locals.var_wfactor_dn12 = assign21510_e30313_d_n12;
        locals.var_wfactor_dn17 = assign21510_e30313_d_n17;
        locals.var_wfactor_rv = 0.0;

        let (assign21520_e30323, assign21520_e30323_d_n0, assign21520_e30323_d_n2, assign21520_e30323_d_n6, assign21520_e30323_d_n7, assign21520_e30323_d_n10, assign21520_e30323_d_n11, assign21520_e30323_d_n12, assign21520_e30323_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21520_e30317: f64 = (locals.var_ps0__blk606 - locals.var_dphi_vds);
        let assign21520_e30319: f64 = (assign21520_e30317 * locals.var_wfactor);
        let assign21520_e30321: f64 = (assign21520_e30319 + locals.var_dphi_vds);
        (assign21520_e30321, ((((locals.var_ps0__blk606_dn0 - locals.var_dphi_vds_dn0) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn0)) + locals.var_dphi_vds_dn0), ((((locals.var_ps0__blk606_dn2 - locals.var_dphi_vds_dn2) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn2)) + locals.var_dphi_vds_dn2), ((((locals.var_ps0__blk606_dn6 - locals.var_dphi_vds_dn6) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn6)) + locals.var_dphi_vds_dn6), ((((locals.var_ps0__blk606_dn7 - locals.var_dphi_vds_dn7) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn7)) + locals.var_dphi_vds_dn7), ((((locals.var_ps0__blk606_dn10 - locals.var_dphi_vds_dn10) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn10)) + locals.var_dphi_vds_dn10), ((((locals.var_ps0__blk606_dn11 - locals.var_dphi_vds_dn11) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn11)) + locals.var_dphi_vds_dn11), ((((locals.var_ps0__blk606_dn12 - locals.var_dphi_vds_dn12) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn12)) + locals.var_dphi_vds_dn12), ((((locals.var_ps0__blk606_dn17 - locals.var_dphi_vds_dn17) * locals.var_wfactor) + (assign21520_e30317 * locals.var_wfactor_dn17)) + locals.var_dphi_vds_dn17),)
    } else {
        (locals.var_phim, locals.var_phim_dn0, locals.var_phim_dn2, locals.var_phim_dn6, locals.var_phim_dn7, locals.var_phim_dn10, locals.var_phim_dn11, locals.var_phim_dn12, locals.var_phim_dn17,)
    }
};
        locals.var_phim = assign21520_e30323;
        locals.var_phim_dn0 = assign21520_e30323_d_n0;
        locals.var_phim_dn2 = assign21520_e30323_d_n2;
        locals.var_phim_dn6 = assign21520_e30323_d_n6;
        locals.var_phim_dn7 = assign21520_e30323_d_n7;
        locals.var_phim_dn10 = assign21520_e30323_d_n10;
        locals.var_phim_dn11 = assign21520_e30323_d_n11;
        locals.var_phim_dn12 = assign21520_e30323_d_n12;
        locals.var_phim_dn17 = assign21520_e30323_d_n17;
        locals.var_phim_rv = 0.0;

        let (assign21530_e30337, assign21530_e30337_d_n0, assign21530_e30337_d_n2, assign21530_e30337_d_n6, assign21530_e30337_d_n7, assign21530_e30337_d_n10, assign21530_e30337_d_n11, assign21530_e30337_d_n12, assign21530_e30337_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21530_e30327: f64 = (locals.var_beta * locals.var_phim);
        let assign21530_e30328: f64 = (assign21530_e30327).exp();
        let assign21530_e30332: f64 = (locals.var_phim - locals.var_vds);
        let assign21530_e30333: f64 = (locals.var_beta * assign21530_e30332);
        let assign21530_e30334: f64 = (assign21530_e30333).exp();
        let assign21530_e30335: f64 = (assign21530_e30328 - assign21530_e30334);
        (assign21530_e30335, ((assign21530_e30328 * (locals.var_beta * locals.var_phim_dn0)) - (assign21530_e30334 * (locals.var_beta * (locals.var_phim_dn0 - locals.var_vds_dn0)))), ((assign21530_e30328 * (locals.var_beta * locals.var_phim_dn2)) - (assign21530_e30334 * (locals.var_beta * (locals.var_phim_dn2 - locals.var_vds_dn2)))), ((assign21530_e30328 * (locals.var_beta * locals.var_phim_dn6)) - (assign21530_e30334 * (locals.var_beta * (locals.var_phim_dn6 - locals.var_vds_dn6)))), ((assign21530_e30328 * (locals.var_beta * locals.var_phim_dn7)) - (assign21530_e30334 * (locals.var_beta * (locals.var_phim_dn7 - locals.var_vds_dn7)))), ((assign21530_e30328 * ((locals.var_beta_dn10 * locals.var_phim) + (locals.var_beta * locals.var_phim_dn10))) - (assign21530_e30334 * ((locals.var_beta_dn10 * assign21530_e30332) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_vds_dn10))))), ((assign21530_e30328 * (locals.var_beta * locals.var_phim_dn11)) - (assign21530_e30334 * (locals.var_beta * (locals.var_phim_dn11 - locals.var_vds_dn11)))), ((assign21530_e30328 * (locals.var_beta * locals.var_phim_dn12)) - (assign21530_e30334 * (locals.var_beta * (locals.var_phim_dn12 - locals.var_vds_dn12)))), ((assign21530_e30328 * (locals.var_beta * locals.var_phim_dn17)) - (assign21530_e30334 * (locals.var_beta * (locals.var_phim_dn17 - locals.var_vds_dn17)))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign21530_e30337;
        locals.var_ty_dn0 = assign21530_e30337_d_n0;
        locals.var_ty_dn2 = assign21530_e30337_d_n2;
        locals.var_ty_dn6 = assign21530_e30337_d_n6;
        locals.var_ty_dn7 = assign21530_e30337_d_n7;
        locals.var_ty_dn10 = assign21530_e30337_d_n10;
        locals.var_ty_dn11 = assign21530_e30337_d_n11;
        locals.var_ty_dn12 = assign21530_e30337_d_n12;
        locals.var_ty_dn17 = assign21530_e30337_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign21540_e30348,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21540_e30341: f64 = (2.0 * 1.6021918e-19);
        let assign21540_e30343: f64 = (assign21540_e30341 * locals.var_uc_wk_njunc);
        let assign21540_e30345: f64 = (assign21540_e30343 * 1.034943e-10);
        let assign21540_e30346: f64 = (assign21540_e30345).sqrt();
        (assign21540_e30346,)
    } else {
        (locals.var_conpt00,)
    }
};
        locals.var_conpt00 = assign21540_e30348;
        locals.var_conpt00_rv = 0.0;

        let (assign21550_e30355, assign21550_e30355_d_n10,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21550_e30352: f64 = (locals.var_beta_inv).sqrt();
        let assign21550_e30353: f64 = (locals.var_conpt00 * assign21550_e30352);
        (assign21550_e30353, (locals.var_conpt00 * (locals.var_beta_inv_dn10 / (2.0 * assign21550_e30352))),)
    } else {
        (locals.var_conpt0, locals.var_conpt0_dn10,)
    }
};
        locals.var_conpt0 = assign21550_e30355;
        locals.var_conpt0_dn10 = assign21550_e30355_d_n10;
        locals.var_conpt0_rv = 0.0;

        let (assign21560_e30363, assign21560_e30363_d_n0, assign21560_e30363_d_n2, assign21560_e30363_d_n6, assign21560_e30363_d_n7, assign21560_e30363_d_n10, assign21560_e30363_d_n11, assign21560_e30363_d_n12, assign21560_e30363_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21560_e30360: f64 = (locals.var_phim - locals.var_dphi_vds);
        let assign21560_e30361: f64 = (locals.var_beta * assign21560_e30360);
        (assign21560_e30361, (locals.var_beta * (locals.var_phim_dn0 - locals.var_dphi_vds_dn0)), (locals.var_beta * (locals.var_phim_dn2 - locals.var_dphi_vds_dn2)), (locals.var_beta * (locals.var_phim_dn6 - locals.var_dphi_vds_dn6)), (locals.var_beta * (locals.var_phim_dn7 - locals.var_dphi_vds_dn7)), ((locals.var_beta_dn10 * assign21560_e30360) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_dphi_vds_dn10))), (locals.var_beta * (locals.var_phim_dn11 - locals.var_dphi_vds_dn11)), (locals.var_beta * (locals.var_phim_dn12 - locals.var_dphi_vds_dn12)), (locals.var_beta * (locals.var_phim_dn17 - locals.var_dphi_vds_dn17)),)
    } else {
        (locals.var_t1w__blk605, locals.var_t1w__blk605_dn0, locals.var_t1w__blk605_dn2, locals.var_t1w__blk605_dn6, locals.var_t1w__blk605_dn7, locals.var_t1w__blk605_dn10, locals.var_t1w__blk605_dn11, locals.var_t1w__blk605_dn12, locals.var_t1w__blk605_dn17,)
    }
};
        locals.var_t1w__blk605 = assign21560_e30363;
        locals.var_t1w__blk605_dn0 = assign21560_e30363_d_n0;
        locals.var_t1w__blk605_dn2 = assign21560_e30363_d_n2;
        locals.var_t1w__blk605_dn6 = assign21560_e30363_d_n6;
        locals.var_t1w__blk605_dn7 = assign21560_e30363_d_n7;
        locals.var_t1w__blk605_dn10 = assign21560_e30363_d_n10;
        locals.var_t1w__blk605_dn11 = assign21560_e30363_d_n11;
        locals.var_t1w__blk605_dn12 = assign21560_e30363_d_n12;
        locals.var_t1w__blk605_dn17 = assign21560_e30363_d_n17;
        locals.var_t1w__blk605_rv = 0.0;

        let assign21570_e30368: f64 = (0.2 * locals.var_beta);
        let assign21570_e30369: f64 = assign21570_e30368;
        let assign21570_e30373: f64 = (0.2 * locals.var_beta);
        let assign21570_e30376: f64 = if ((locals.var_t1w__blk605 < assign21570_e30369) && (assign21570_e30373 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard661 = assign21570_e30376;
        locals.var_guard661_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        locals: &mut StampLocals,
    ) {
        let (assign21580_e30388, assign21580_e30388_d_n0, assign21580_e30388_d_n2, assign21580_e30388_d_n6, assign21580_e30388_d_n7, assign21580_e30388_d_n10, assign21580_e30388_d_n11, assign21580_e30388_d_n12, assign21580_e30388_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21580_e30383: f64 = (0.2 * locals.var_beta);
        let assign21580_e30384: f64 = assign21580_e30383;
        let assign21580_e30386: f64 = (assign21580_e30384 - locals.var_t1w__blk605);
        (assign21580_e30386, (-locals.var_t1w__blk605_dn0), (-locals.var_t1w__blk605_dn2), (-locals.var_t1w__blk605_dn6), (-locals.var_t1w__blk605_dn7), ((0.2 * locals.var_beta_dn10) - locals.var_t1w__blk605_dn10), (-locals.var_t1w__blk605_dn11), (-locals.var_t1w__blk605_dn12), (-locals.var_t1w__blk605_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign21580_e30388;
        locals.var_tmf1_dn0 = assign21580_e30388_d_n0;
        locals.var_tmf1_dn2 = assign21580_e30388_d_n2;
        locals.var_tmf1_dn6 = assign21580_e30388_d_n6;
        locals.var_tmf1_dn7 = assign21580_e30388_d_n7;
        locals.var_tmf1_dn10 = assign21580_e30388_d_n10;
        locals.var_tmf1_dn11 = assign21580_e30388_d_n11;
        locals.var_tmf1_dn12 = assign21580_e30388_d_n12;
        locals.var_tmf1_dn17 = assign21580_e30388_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign21590_e30396, assign21590_e30396_d_n0, assign21590_e30396_d_n2, assign21590_e30396_d_n6, assign21590_e30396_d_n7, assign21590_e30396_d_n10, assign21590_e30396_d_n11, assign21590_e30396_d_n12, assign21590_e30396_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21590_e30394: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign21590_e30394, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign21590_e30396;
        locals.var_x2_dn0 = assign21590_e30396_d_n0;
        locals.var_x2_dn2 = assign21590_e30396_d_n2;
        locals.var_x2_dn6 = assign21590_e30396_d_n6;
        locals.var_x2_dn7 = assign21590_e30396_d_n7;
        locals.var_x2_dn10 = assign21590_e30396_d_n10;
        locals.var_x2_dn11 = assign21590_e30396_d_n11;
        locals.var_x2_dn12 = assign21590_e30396_d_n12;
        locals.var_x2_dn17 = assign21590_e30396_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign21600_e30408, assign21600_e30408_d_n0, assign21600_e30408_d_n2, assign21600_e30408_d_n6, assign21600_e30408_d_n7, assign21600_e30408_d_n10, assign21600_e30408_d_n11, assign21600_e30408_d_n12, assign21600_e30408_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21600_e30402: f64 = (0.2 * locals.var_beta);
        let assign21600_e30405: f64 = (0.2 * locals.var_beta);
        let assign21600_e30406: f64 = (assign21600_e30402 * assign21600_e30405);
        (assign21600_e30406, 0.0, 0.0, 0.0, 0.0, (((0.2 * locals.var_beta_dn10) * assign21600_e30405) + (assign21600_e30402 * (0.2 * locals.var_beta_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign21600_e30408;
        locals.var_xmax2_dn0 = assign21600_e30408_d_n0;
        locals.var_xmax2_dn2 = assign21600_e30408_d_n2;
        locals.var_xmax2_dn6 = assign21600_e30408_d_n6;
        locals.var_xmax2_dn7 = assign21600_e30408_d_n7;
        locals.var_xmax2_dn10 = assign21600_e30408_d_n10;
        locals.var_xmax2_dn11 = assign21600_e30408_d_n11;
        locals.var_xmax2_dn12 = assign21600_e30408_d_n12;
        locals.var_xmax2_dn17 = assign21600_e30408_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign21610_e30414, assign21610_e30414_d_n0, assign21610_e30414_d_n2, assign21610_e30414_d_n6, assign21610_e30414_d_n7, assign21610_e30414_d_n10, assign21610_e30414_d_n11, assign21610_e30414_d_n12, assign21610_e30414_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign21610_e30414;
        locals.var_xp_dn0 = assign21610_e30414_d_n0;
        locals.var_xp_dn2 = assign21610_e30414_d_n2;
        locals.var_xp_dn6 = assign21610_e30414_d_n6;
        locals.var_xp_dn7 = assign21610_e30414_d_n7;
        locals.var_xp_dn10 = assign21610_e30414_d_n10;
        locals.var_xp_dn11 = assign21610_e30414_d_n11;
        locals.var_xp_dn12 = assign21610_e30414_d_n12;
        locals.var_xp_dn17 = assign21610_e30414_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign21620_e30420, assign21620_e30420_d_n0, assign21620_e30420_d_n2, assign21620_e30420_d_n6, assign21620_e30420_d_n7, assign21620_e30420_d_n10, assign21620_e30420_d_n11, assign21620_e30420_d_n12, assign21620_e30420_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign21620_e30420;
        locals.var_xmp_dn0 = assign21620_e30420_d_n0;
        locals.var_xmp_dn2 = assign21620_e30420_d_n2;
        locals.var_xmp_dn6 = assign21620_e30420_d_n6;
        locals.var_xmp_dn7 = assign21620_e30420_d_n7;
        locals.var_xmp_dn10 = assign21620_e30420_d_n10;
        locals.var_xmp_dn11 = assign21620_e30420_d_n11;
        locals.var_xmp_dn12 = assign21620_e30420_d_n12;
        locals.var_xmp_dn17 = assign21620_e30420_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign21630_e30426,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign21630_e30426;
        locals.var_m0_rv = 0.0;

        let (assign21640_e30432,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21640_e30432;
        locals.var_mm_rv = 0.0;

        let (assign21650_e30438, assign21650_e30438_d_n0, assign21650_e30438_d_n2, assign21650_e30438_d_n6, assign21650_e30438_d_n7, assign21650_e30438_d_n10, assign21650_e30438_d_n11, assign21650_e30438_d_n12, assign21650_e30438_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign21650_e30438;
        locals.var_arg_dn0 = assign21650_e30438_d_n0;
        locals.var_arg_dn2 = assign21650_e30438_d_n2;
        locals.var_arg_dn6 = assign21650_e30438_d_n6;
        locals.var_arg_dn7 = assign21650_e30438_d_n7;
        locals.var_arg_dn10 = assign21650_e30438_d_n10;
        locals.var_arg_dn11 = assign21650_e30438_d_n11;
        locals.var_arg_dn12 = assign21650_e30438_d_n12;
        locals.var_arg_dn17 = assign21650_e30438_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign21660_e30444, assign21660_e30444_d_n0, assign21660_e30444_d_n2, assign21660_e30444_d_n6, assign21660_e30444_d_n7, assign21660_e30444_d_n10, assign21660_e30444_d_n11, assign21660_e30444_d_n12, assign21660_e30444_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21660_e30444;
        locals.var_dnm_dn0 = assign21660_e30444_d_n0;
        locals.var_dnm_dn2 = assign21660_e30444_d_n2;
        locals.var_dnm_dn6 = assign21660_e30444_d_n6;
        locals.var_dnm_dn7 = assign21660_e30444_d_n7;
        locals.var_dnm_dn10 = assign21660_e30444_d_n10;
        locals.var_dnm_dn11 = assign21660_e30444_d_n11;
        locals.var_dnm_dn12 = assign21660_e30444_d_n12;
        locals.var_dnm_dn17 = assign21660_e30444_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21670_e30452, assign21670_e30452_d_n0, assign21670_e30452_d_n2, assign21670_e30452_d_n6, assign21670_e30452_d_n7, assign21670_e30452_d_n10, assign21670_e30452_d_n11, assign21670_e30452_d_n12, assign21670_e30452_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21670_e30450: f64 = (locals.var_xp * locals.var_x2);
        (assign21670_e30450, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign21670_e30452;
        locals.var_xp_dn0 = assign21670_e30452_d_n0;
        locals.var_xp_dn2 = assign21670_e30452_d_n2;
        locals.var_xp_dn6 = assign21670_e30452_d_n6;
        locals.var_xp_dn7 = assign21670_e30452_d_n7;
        locals.var_xp_dn10 = assign21670_e30452_d_n10;
        locals.var_xp_dn11 = assign21670_e30452_d_n11;
        locals.var_xp_dn12 = assign21670_e30452_d_n12;
        locals.var_xp_dn17 = assign21670_e30452_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign21680_e30460, assign21680_e30460_d_n0, assign21680_e30460_d_n2, assign21680_e30460_d_n6, assign21680_e30460_d_n7, assign21680_e30460_d_n10, assign21680_e30460_d_n11, assign21680_e30460_d_n12, assign21680_e30460_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21680_e30458: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign21680_e30458, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign21680_e30460;
        locals.var_xmp_dn0 = assign21680_e30460_d_n0;
        locals.var_xmp_dn2 = assign21680_e30460_d_n2;
        locals.var_xmp_dn6 = assign21680_e30460_d_n6;
        locals.var_xmp_dn7 = assign21680_e30460_d_n7;
        locals.var_xmp_dn10 = assign21680_e30460_d_n10;
        locals.var_xmp_dn11 = assign21680_e30460_d_n11;
        locals.var_xmp_dn12 = assign21680_e30460_d_n12;
        locals.var_xmp_dn17 = assign21680_e30460_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign21690_e30468, assign21690_e30468_d_n0, assign21690_e30468_d_n2, assign21690_e30468_d_n6, assign21690_e30468_d_n7, assign21690_e30468_d_n10, assign21690_e30468_d_n11, assign21690_e30468_d_n12, assign21690_e30468_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21690_e30466: f64 = (locals.var_xp + locals.var_xmp);
        (assign21690_e30466, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign21690_e30468;
        locals.var_arg_dn0 = assign21690_e30468_d_n0;
        locals.var_arg_dn2 = assign21690_e30468_d_n2;
        locals.var_arg_dn6 = assign21690_e30468_d_n6;
        locals.var_arg_dn7 = assign21690_e30468_d_n7;
        locals.var_arg_dn10 = assign21690_e30468_d_n10;
        locals.var_arg_dn11 = assign21690_e30468_d_n11;
        locals.var_arg_dn12 = assign21690_e30468_d_n12;
        locals.var_arg_dn17 = assign21690_e30468_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign21700_e30474, assign21700_e30474_d_n0, assign21700_e30474_d_n2, assign21700_e30474_d_n6, assign21700_e30474_d_n7, assign21700_e30474_d_n10, assign21700_e30474_d_n11, assign21700_e30474_d_n12, assign21700_e30474_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21700_e30474;
        locals.var_dnm_dn0 = assign21700_e30474_d_n0;
        locals.var_dnm_dn2 = assign21700_e30474_d_n2;
        locals.var_dnm_dn6 = assign21700_e30474_d_n6;
        locals.var_dnm_dn7 = assign21700_e30474_d_n7;
        locals.var_dnm_dn10 = assign21700_e30474_d_n10;
        locals.var_dnm_dn11 = assign21700_e30474_d_n11;
        locals.var_dnm_dn12 = assign21700_e30474_d_n12;
        locals.var_dnm_dn17 = assign21700_e30474_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign21710_e30489: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard662 = assign21710_e30489;
        locals.var_guard662_rv = 0.0;

        let assign21720_e30492: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign21720_e30492;
        locals.var_guard663_rv = 0.0;

        let (assign21730_e30502,) = {
    if ((((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21730_e30502;
        locals.var_mm_rv = 0.0;

        let assign21740_e30505: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign21740_e30505;
        locals.var_guard664_rv = 0.0;

        let (assign21750_e30518,) = {
    if (((((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21750_e30518;
        locals.var_mm_rv = 0.0;

        let assign21760_e30521: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign21760_e30521;
        locals.var_guard665_rv = 0.0;

        let (assign21770_e30537,) = {
    if ((((((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 == 0.0)) && (locals.var_guard665 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21770_e30537;
        locals.var_mm_rv = 0.0;

        let assign21780_e30540: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign21780_e30540;
        locals.var_guard666_rv = 0.0;

        let (assign21790_e30559,) = {
    if (((((((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard664 == 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign21790_e30559;
        locals.var_mm_rv = 0.0;

        let (assign21800_e30567,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign21800_e30567;
        locals.var_m0_rv = 0.0;

        let mut assign21810_loop_guard: usize = 0;
        while {
            let assign21810_cond_e30576: f64 = if ((((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign21810_cond_e30576 != 0.0
        } {
            assign21810_loop_guard += 1;
            assert!(assign21810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign21810_body0_e30585, assign21810_body0_e30585_d_n0, assign21810_body0_e30585_d_n2, assign21810_body0_e30585_d_n6, assign21810_body0_e30585_d_n7, assign21810_body0_e30585_d_n10, assign21810_body0_e30585_d_n11, assign21810_body0_e30585_d_n12, assign21810_body0_e30585_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign21810_body0_e30583: f64 = (locals.var_dnm).sqrt();
        (assign21810_body0_e30583, (locals.var_dnm_dn0 / (2.0 * assign21810_body0_e30583)), (locals.var_dnm_dn2 / (2.0 * assign21810_body0_e30583)), (locals.var_dnm_dn6 / (2.0 * assign21810_body0_e30583)), (locals.var_dnm_dn7 / (2.0 * assign21810_body0_e30583)), (locals.var_dnm_dn10 / (2.0 * assign21810_body0_e30583)), (locals.var_dnm_dn11 / (2.0 * assign21810_body0_e30583)), (locals.var_dnm_dn12 / (2.0 * assign21810_body0_e30583)), (locals.var_dnm_dn17 / (2.0 * assign21810_body0_e30583)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign21810_body0_e30585;
            locals.var_dnm_dn0 = assign21810_body0_e30585_d_n0;
            locals.var_dnm_dn2 = assign21810_body0_e30585_d_n2;
            locals.var_dnm_dn6 = assign21810_body0_e30585_d_n6;
            locals.var_dnm_dn7 = assign21810_body0_e30585_d_n7;
            locals.var_dnm_dn10 = assign21810_body0_e30585_d_n10;
            locals.var_dnm_dn11 = assign21810_body0_e30585_d_n11;
            locals.var_dnm_dn12 = assign21810_body0_e30585_d_n12;
            locals.var_dnm_dn17 = assign21810_body0_e30585_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign21810_body1_e30595,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign21810_body1_e30593: f64 = (locals.var_m0 + 1.0);
        (assign21810_body1_e30593,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign21810_body1_e30595;
            locals.var_m0_rv = 0.0;
        }

        let (assign21820_e30610, assign21820_e30610_d_n0, assign21820_e30610_d_n2, assign21820_e30610_d_n6, assign21820_e30610_d_n7, assign21820_e30610_d_n10, assign21820_e30610_d_n11, assign21820_e30610_d_n12, assign21820_e30610_d_n17,) = {
    if (((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 == 0.0)) {
        let assign21820_e30606: f64 = 2.0;
        let assign21820_e30607: f64 = (1.0 / assign21820_e30606);
        let assign21820_e30608: f64 = (locals.var_dnm).powf(assign21820_e30607);
        (assign21820_e30608, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn0)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn2)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn6)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn7)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn10)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn11)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn12)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign21820_e30607) as f64).is_finite() && ((assign21820_e30607) as f64).fract() == 0.0 { if assign21820_e30607 == 0.0 { 0.0 } else { (assign21820_e30607 * ((locals.var_dnm).powf(assign21820_e30607 - 1.0) * locals.var_dnm_dn17)) } } else { (assign21820_e30608 * (assign21820_e30607 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21820_e30610;
        locals.var_dnm_dn0 = assign21820_e30610_d_n0;
        locals.var_dnm_dn2 = assign21820_e30610_d_n2;
        locals.var_dnm_dn6 = assign21820_e30610_d_n6;
        locals.var_dnm_dn7 = assign21820_e30610_d_n7;
        locals.var_dnm_dn10 = assign21820_e30610_d_n10;
        locals.var_dnm_dn11 = assign21820_e30610_d_n11;
        locals.var_dnm_dn12 = assign21820_e30610_d_n12;
        locals.var_dnm_dn17 = assign21820_e30610_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21830_e30618, assign21830_e30618_d_n0, assign21830_e30618_d_n2, assign21830_e30618_d_n6, assign21830_e30618_d_n7, assign21830_e30618_d_n10, assign21830_e30618_d_n11, assign21830_e30618_d_n12, assign21830_e30618_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21830_e30616: f64 = (1.0 / locals.var_dnm);
        (assign21830_e30616, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign21830_e30618;
        locals.var_dnm_dn0 = assign21830_e30618_d_n0;
        locals.var_dnm_dn2 = assign21830_e30618_d_n2;
        locals.var_dnm_dn6 = assign21830_e30618_d_n6;
        locals.var_dnm_dn7 = assign21830_e30618_d_n7;
        locals.var_dnm_dn10 = assign21830_e30618_d_n10;
        locals.var_dnm_dn11 = assign21830_e30618_d_n11;
        locals.var_dnm_dn12 = assign21830_e30618_d_n12;
        locals.var_dnm_dn17 = assign21830_e30618_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign21840_e30630, assign21840_e30630_d_n0, assign21840_e30630_d_n2, assign21840_e30630_d_n6, assign21840_e30630_d_n7, assign21840_e30630_d_n10, assign21840_e30630_d_n11, assign21840_e30630_d_n12, assign21840_e30630_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21840_e30625: f64 = (0.2 * locals.var_beta);
        let assign21840_e30626: f64 = (locals.var_tmf1 * assign21840_e30625);
        let assign21840_e30628: f64 = (assign21840_e30626 * locals.var_dnm);
        (assign21840_e30628, (((locals.var_tmf1_dn0 * assign21840_e30625) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign21840_e30625) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * assign21840_e30625) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign21840_e30625) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign21840_e30625) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn10))) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign21840_e30625) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * assign21840_e30625) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * assign21840_e30625) * locals.var_dnm) + (assign21840_e30626 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign21840_e30630;
        locals.var_tmf0_dn0 = assign21840_e30630_d_n0;
        locals.var_tmf0_dn2 = assign21840_e30630_d_n2;
        locals.var_tmf0_dn6 = assign21840_e30630_d_n6;
        locals.var_tmf0_dn7 = assign21840_e30630_d_n7;
        locals.var_tmf0_dn10 = assign21840_e30630_d_n10;
        locals.var_tmf0_dn11 = assign21840_e30630_d_n11;
        locals.var_tmf0_dn12 = assign21840_e30630_d_n12;
        locals.var_tmf0_dn17 = assign21840_e30630_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign21850_e30642, assign21850_e30642_d_n0, assign21850_e30642_d_n2, assign21850_e30642_d_n6, assign21850_e30642_d_n7, assign21850_e30642_d_n10, assign21850_e30642_d_n11, assign21850_e30642_d_n12, assign21850_e30642_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign21850_e30637: f64 = (0.2 * locals.var_beta);
        let assign21850_e30638: f64 = assign21850_e30637;
        let assign21850_e30640: f64 = (assign21850_e30638 - locals.var_tmf0);
        (assign21850_e30640, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), ((0.2 * locals.var_beta_dn10) - locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21850_e30642;
        locals.var_t1_dn0 = assign21850_e30642_d_n0;
        locals.var_t1_dn2 = assign21850_e30642_d_n2;
        locals.var_t1_dn6 = assign21850_e30642_d_n6;
        locals.var_t1_dn7 = assign21850_e30642_d_n7;
        locals.var_t1_dn10 = assign21850_e30642_d_n10;
        locals.var_t1_dn11 = assign21850_e30642_d_n11;
        locals.var_t1_dn12 = assign21850_e30642_d_n12;
        locals.var_t1_dn17 = assign21850_e30642_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21860_e30649, assign21860_e30649_d_n0, assign21860_e30649_d_n2, assign21860_e30649_d_n6, assign21860_e30649_d_n7, assign21860_e30649_d_n10, assign21860_e30649_d_n11, assign21860_e30649_d_n12, assign21860_e30649_d_n17,) = {
    if ((locals.var_guard596 != 0.0) && (locals.var_guard661 == 0.0)) {
        (locals.var_t1w__blk605, locals.var_t1w__blk605_dn0, locals.var_t1w__blk605_dn2, locals.var_t1w__blk605_dn6, locals.var_t1w__blk605_dn7, locals.var_t1w__blk605_dn10, locals.var_t1w__blk605_dn11, locals.var_t1w__blk605_dn12, locals.var_t1w__blk605_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign21860_e30649;
        locals.var_t1_dn0 = assign21860_e30649_d_n0;
        locals.var_t1_dn2 = assign21860_e30649_d_n2;
        locals.var_t1_dn6 = assign21860_e30649_d_n6;
        locals.var_t1_dn7 = assign21860_e30649_d_n7;
        locals.var_t1_dn10 = assign21860_e30649_d_n10;
        locals.var_t1_dn11 = assign21860_e30649_d_n11;
        locals.var_t1_dn12 = assign21860_e30649_d_n12;
        locals.var_t1_dn17 = assign21860_e30649_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign21870_e30658, assign21870_e30658_d_n0, assign21870_e30658_d_n2, assign21870_e30658_d_n6, assign21870_e30658_d_n7, assign21870_e30658_d_n10, assign21870_e30658_d_n11, assign21870_e30658_d_n12, assign21870_e30658_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21870_e30654: f64 = (10.0 * 2.220446049250313e-16);
        let assign21870_e30655: f64 = (locals.var_t1 + assign21870_e30654);
        let assign21870_e30656: f64 = (assign21870_e30655).sqrt();
        (assign21870_e30656, (locals.var_t1_dn0 / (2.0 * assign21870_e30656)), (locals.var_t1_dn2 / (2.0 * assign21870_e30656)), (locals.var_t1_dn6 / (2.0 * assign21870_e30656)), (locals.var_t1_dn7 / (2.0 * assign21870_e30656)), (locals.var_t1_dn10 / (2.0 * assign21870_e30656)), (locals.var_t1_dn11 / (2.0 * assign21870_e30656)), (locals.var_t1_dn12 / (2.0 * assign21870_e30656)), (locals.var_t1_dn17 / (2.0 * assign21870_e30656)),)
    } else {
        (locals.var_sq1npt, locals.var_sq1npt_dn0, locals.var_sq1npt_dn2, locals.var_sq1npt_dn6, locals.var_sq1npt_dn7, locals.var_sq1npt_dn10, locals.var_sq1npt_dn11, locals.var_sq1npt_dn12, locals.var_sq1npt_dn17,)
    }
};
        locals.var_sq1npt = assign21870_e30658;
        locals.var_sq1npt_dn0 = assign21870_e30658_d_n0;
        locals.var_sq1npt_dn2 = assign21870_e30658_d_n2;
        locals.var_sq1npt_dn6 = assign21870_e30658_d_n6;
        locals.var_sq1npt_dn7 = assign21870_e30658_d_n7;
        locals.var_sq1npt_dn10 = assign21870_e30658_d_n10;
        locals.var_sq1npt_dn11 = assign21870_e30658_d_n11;
        locals.var_sq1npt_dn12 = assign21870_e30658_d_n12;
        locals.var_sq1npt_dn17 = assign21870_e30658_d_n17;
        locals.var_sq1npt_rv = 0.0;

        let (assign21880_e30664, assign21880_e30664_d_n0, assign21880_e30664_d_n2, assign21880_e30664_d_n6, assign21880_e30664_d_n7, assign21880_e30664_d_n10, assign21880_e30664_d_n11, assign21880_e30664_d_n12, assign21880_e30664_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21880_e30662: f64 = (locals.var_conpt0 * locals.var_sq1npt);
        (assign21880_e30662, (locals.var_conpt0 * locals.var_sq1npt_dn0), (locals.var_conpt0 * locals.var_sq1npt_dn2), (locals.var_conpt0 * locals.var_sq1npt_dn6), (locals.var_conpt0 * locals.var_sq1npt_dn7), ((locals.var_conpt0_dn10 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn10)), (locals.var_conpt0 * locals.var_sq1npt_dn11), (locals.var_conpt0 * locals.var_sq1npt_dn12), (locals.var_conpt0 * locals.var_sq1npt_dn17),)
    } else {
        (locals.var_qn0npt, locals.var_qn0npt_dn0, locals.var_qn0npt_dn2, locals.var_qn0npt_dn6, locals.var_qn0npt_dn7, locals.var_qn0npt_dn10, locals.var_qn0npt_dn11, locals.var_qn0npt_dn12, locals.var_qn0npt_dn17,)
    }
};
        locals.var_qn0npt = assign21880_e30664;
        locals.var_qn0npt_dn0 = assign21880_e30664_d_n0;
        locals.var_qn0npt_dn2 = assign21880_e30664_d_n2;
        locals.var_qn0npt_dn6 = assign21880_e30664_d_n6;
        locals.var_qn0npt_dn7 = assign21880_e30664_d_n7;
        locals.var_qn0npt_dn10 = assign21880_e30664_d_n10;
        locals.var_qn0npt_dn11 = assign21880_e30664_d_n11;
        locals.var_qn0npt_dn12 = assign21880_e30664_d_n12;
        locals.var_qn0npt_dn17 = assign21880_e30664_d_n17;
        locals.var_qn0npt_rv = 0.0;

        let (assign21890_e30674, assign21890_e30674_d_n0, assign21890_e30674_d_n2, assign21890_e30674_d_n6, assign21890_e30674_d_n7, assign21890_e30674_d_n10, assign21890_e30674_d_n11, assign21890_e30674_d_n12, assign21890_e30674_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21890_e30668: f64 = (2.0 * locals.var_beta_inv);
        let assign21890_e30670: f64 = (assign21890_e30668 / locals.var_leff__blk603);
        let assign21890_e30672: f64 = (assign21890_e30670 * locals.var_qn0npt);
        (assign21890_e30672, (assign21890_e30670 * locals.var_qn0npt_dn0), (assign21890_e30670 * locals.var_qn0npt_dn2), (assign21890_e30670 * locals.var_qn0npt_dn6), (assign21890_e30670 * locals.var_qn0npt_dn7), ((((2.0 * locals.var_beta_inv_dn10) / locals.var_leff__blk603) * locals.var_qn0npt) + (assign21890_e30670 * locals.var_qn0npt_dn10)), (assign21890_e30670 * locals.var_qn0npt_dn11), (assign21890_e30670 * locals.var_qn0npt_dn12), (assign21890_e30670 * locals.var_qn0npt_dn17),)
    } else {
        (locals.var_wk_jnpt_a, locals.var_wk_jnpt_a_dn0, locals.var_wk_jnpt_a_dn2, locals.var_wk_jnpt_a_dn6, locals.var_wk_jnpt_a_dn7, locals.var_wk_jnpt_a_dn10, locals.var_wk_jnpt_a_dn11, locals.var_wk_jnpt_a_dn12, locals.var_wk_jnpt_a_dn17,)
    }
};
        locals.var_wk_jnpt_a = assign21890_e30674;
        locals.var_wk_jnpt_a_dn0 = assign21890_e30674_d_n0;
        locals.var_wk_jnpt_a_dn2 = assign21890_e30674_d_n2;
        locals.var_wk_jnpt_a_dn6 = assign21890_e30674_d_n6;
        locals.var_wk_jnpt_a_dn7 = assign21890_e30674_d_n7;
        locals.var_wk_jnpt_a_dn10 = assign21890_e30674_d_n10;
        locals.var_wk_jnpt_a_dn11 = assign21890_e30674_d_n11;
        locals.var_wk_jnpt_a_dn12 = assign21890_e30674_d_n12;
        locals.var_wk_jnpt_a_dn17 = assign21890_e30674_d_n17;
        locals.var_wk_jnpt_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21900_e30684, assign21900_e30684_d_n0, assign21900_e30684_d_n2, assign21900_e30684_d_n6, assign21900_e30684_d_n7, assign21900_e30684_d_n10, assign21900_e30684_d_n11, assign21900_e30684_d_n12, assign21900_e30684_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21900_e30678: f64 = (locals.var_wk_jnpt_a * locals.var_wk_mu);
        let assign21900_e30680: f64 = (assign21900_e30678 * locals.var_weff_nf);
        let assign21900_e30682: f64 = (assign21900_e30680 * locals.var_ty);
        (assign21900_e30682, ((((locals.var_wk_jnpt_a_dn0 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn0)), ((((locals.var_wk_jnpt_a_dn2 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn2)), ((((locals.var_wk_jnpt_a_dn6 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn6)), ((((locals.var_wk_jnpt_a_dn7 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn7)), ((((locals.var_wk_jnpt_a_dn10 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn10)), ((((locals.var_wk_jnpt_a_dn11 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn11)), ((((locals.var_wk_jnpt_a_dn12 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn12)), ((((locals.var_wk_jnpt_a_dn17 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21900_e30680 * locals.var_ty_dn17)),)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn10, locals.var_idspt1_dn11, locals.var_idspt1_dn12, locals.var_idspt1_dn17,)
    }
};
        locals.var_idspt1 = assign21900_e30684;
        locals.var_idspt1_dn0 = assign21900_e30684_d_n0;
        locals.var_idspt1_dn2 = assign21900_e30684_d_n2;
        locals.var_idspt1_dn6 = assign21900_e30684_d_n6;
        locals.var_idspt1_dn7 = assign21900_e30684_d_n7;
        locals.var_idspt1_dn10 = assign21900_e30684_d_n10;
        locals.var_idspt1_dn11 = assign21900_e30684_d_n11;
        locals.var_idspt1_dn12 = assign21900_e30684_d_n12;
        locals.var_idspt1_dn17 = assign21900_e30684_d_n17;
        locals.var_idspt1_rv = 0.0;

        let (assign21910_e30690, assign21910_e30690_d_n0, assign21910_e30690_d_n2, assign21910_e30690_d_n6, assign21910_e30690_d_n7, assign21910_e30690_d_n10, assign21910_e30690_d_n11, assign21910_e30690_d_n12, assign21910_e30690_d_n17,) = {
    if (locals.var_guard596 != 0.0) {
        let assign21910_e30688: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign21910_e30688, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn11 + locals.var_idspt1_dn11), (locals.var_idsorg_dn12 + locals.var_idspt1_dn12), (locals.var_idsorg_dn17 + locals.var_idspt1_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign21910_e30690;
        locals.var_ids_dn0 = assign21910_e30690_d_n0;
        locals.var_ids_dn2 = assign21910_e30690_d_n2;
        locals.var_ids_dn6 = assign21910_e30690_d_n6;
        locals.var_ids_dn7 = assign21910_e30690_d_n7;
        locals.var_ids_dn10 = assign21910_e30690_d_n10;
        locals.var_ids_dn11 = assign21910_e30690_d_n11;
        locals.var_ids_dn12 = assign21910_e30690_d_n12;
        locals.var_ids_dn17 = assign21910_e30690_d_n17;
        locals.var_ids_rv = 0.0;

        let assign21920_e30693: f64 = (locals.var_idspt0 + locals.var_idspt1);
        locals.var_idspt = assign21920_e30693;
        locals.var_idspt_dn0 = (locals.var_idspt0_dn0 + locals.var_idspt1_dn0);
        locals.var_idspt_dn2 = (locals.var_idspt0_dn2 + locals.var_idspt1_dn2);
        locals.var_idspt_dn6 = (locals.var_idspt0_dn6 + locals.var_idspt1_dn6);
        locals.var_idspt_dn7 = (locals.var_idspt0_dn7 + locals.var_idspt1_dn7);
        locals.var_idspt_dn10 = (locals.var_idspt0_dn10 + locals.var_idspt1_dn10);
        locals.var_idspt_dn11 = (locals.var_idspt0_dn11 + locals.var_idspt1_dn11);
        locals.var_idspt_dn12 = (locals.var_idspt0_dn12 + locals.var_idspt1_dn12);
        locals.var_idspt_dn17 = (locals.var_idspt0_dn17 + locals.var_idspt1_dn17);
        locals.var_idspt_rv = 0.0;

        let assign21930_e30700: f64 = if ((p.p43 == 1.0) || (p.p45 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard667 = assign21930_e30700;
        locals.var_guard667_rv = 0.0;

        let assign21940_e30707: f64 = if ((locals.var_flg_noqi == 1.0) || (p.p25 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard680 = assign21940_e30707;
        locals.var_guard680_rv = 0.0;

        let (assign21950_e30713, assign21950_e30713_d_n0, assign21950_e30713_d_n2, assign21950_e30713_d_n6, assign21950_e30713_d_n7, assign21950_e30713_d_n10, assign21950_e30713_d_n11, assign21950_e30713_d_n12, assign21950_e30713_d_n17,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard680 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign21950_e30713;
        locals.var_isub_dn0 = assign21950_e30713_d_n0;
        locals.var_isub_dn2 = assign21950_e30713_d_n2;
        locals.var_isub_dn6 = assign21950_e30713_d_n6;
        locals.var_isub_dn7 = assign21950_e30713_d_n7;
        locals.var_isub_dn10 = assign21950_e30713_d_n10;
        locals.var_isub_dn11 = assign21950_e30713_d_n11;
        locals.var_isub_dn12 = assign21950_e30713_d_n12;
        locals.var_isub_dn17 = assign21950_e30713_d_n17;
        locals.var_isub_rv = 0.0;

        let assign21960_e30720: f64 = if ((p.p117 <= 0.0) || (locals.var_mks_vmax <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard681 = assign21960_e30720;
        locals.var_guard681_rv = 0.0;

        let (assign21970_e30729, assign21970_e30729_d_n0, assign21970_e30729_d_n2, assign21970_e30729_d_n6, assign21970_e30729_d_n7, assign21970_e30729_d_n10, assign21970_e30729_d_n11, assign21970_e30729_d_n12, assign21970_e30729_d_n17,) = {
    if (((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign21970_e30729;
        locals.var_isub_dn0 = assign21970_e30729_d_n0;
        locals.var_isub_dn2 = assign21970_e30729_d_n2;
        locals.var_isub_dn6 = assign21970_e30729_d_n6;
        locals.var_isub_dn7 = assign21970_e30729_d_n7;
        locals.var_isub_dn10 = assign21970_e30729_d_n10;
        locals.var_isub_dn11 = assign21970_e30729_d_n11;
        locals.var_isub_dn12 = assign21970_e30729_d_n12;
        locals.var_isub_dn17 = assign21970_e30729_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign21980_e30747, assign21980_e30747_d_n0, assign21980_e30747_d_n2, assign21980_e30747_d_n6, assign21980_e30747_d_n7, assign21980_e30747_d_n10, assign21980_e30747_d_n11, assign21980_e30747_d_n12, assign21980_e30747_d_n17,) = {
    if (((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) {
        let assign21980_e30739: f64 = (locals.var_vgsz - locals.var_vfbsub0);
        let assign21980_e30741: f64 = (assign21980_e30739 + locals.var_dvth);
        let assign21980_e30743: f64 = (assign21980_e30741 - locals.var_dppg);
        let assign21980_e30745: f64 = (assign21980_e30743 + p.p48);
        (assign21980_e30745, ((locals.var_vgsz_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_vgsz_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    }
};
        locals.var_vgpsub = assign21980_e30747;
        locals.var_vgpsub_dn0 = assign21980_e30747_d_n0;
        locals.var_vgpsub_dn2 = assign21980_e30747_d_n2;
        locals.var_vgpsub_dn6 = assign21980_e30747_d_n6;
        locals.var_vgpsub_dn7 = assign21980_e30747_d_n7;
        locals.var_vgpsub_dn10 = assign21980_e30747_d_n10;
        locals.var_vgpsub_dn11 = assign21980_e30747_d_n11;
        locals.var_vgpsub_dn12 = assign21980_e30747_d_n12;
        locals.var_vgpsub_dn17 = assign21980_e30747_d_n17;
        locals.var_vgpsub_rv = 0.0;

        let assign21990_e30750: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign21990_e30750;
        locals.var_guard682_rv = 0.0;

        let (assign22000_e30762, assign22000_e30762_d_n0, assign22000_e30762_d_n2, assign22000_e30762_d_n6, assign22000_e30762_d_n7, assign22000_e30762_d_n10, assign22000_e30762_d_n11, assign22000_e30762_d_n12, assign22000_e30762_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_t1__blk668, locals.var_t1__blk668_dn0, locals.var_t1__blk668_dn2, locals.var_t1__blk668_dn6, locals.var_t1__blk668_dn7, locals.var_t1__blk668_dn10, locals.var_t1__blk668_dn11, locals.var_t1__blk668_dn12, locals.var_t1__blk668_dn17,)
    }
};
        locals.var_t1__blk668 = assign22000_e30762;
        locals.var_t1__blk668_dn0 = assign22000_e30762_d_n0;
        locals.var_t1__blk668_dn2 = assign22000_e30762_d_n2;
        locals.var_t1__blk668_dn6 = assign22000_e30762_d_n6;
        locals.var_t1__blk668_dn7 = assign22000_e30762_d_n7;
        locals.var_t1__blk668_dn10 = assign22000_e30762_d_n10;
        locals.var_t1__blk668_dn11 = assign22000_e30762_d_n11;
        locals.var_t1__blk668_dn12 = assign22000_e30762_d_n12;
        locals.var_t1__blk668_dn17 = assign22000_e30762_d_n17;
        locals.var_t1__blk668_rv = 0.0;

        let (assign22010_e30776, assign22010_e30776_d_n0, assign22010_e30776_d_n2, assign22010_e30776_d_n6, assign22010_e30776_d_n7, assign22010_e30776_d_n10, assign22010_e30776_d_n11, assign22010_e30776_d_n12, assign22010_e30776_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22010_e30774: f64 = (locals.var_c_fox * locals.var_c_fox);
        (assign22010_e30774, ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)), ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)), ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)), ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)), ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)), ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)), ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)), ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_t7__blk675, locals.var_t7__blk675_dn0, locals.var_t7__blk675_dn2, locals.var_t7__blk675_dn6, locals.var_t7__blk675_dn7, locals.var_t7__blk675_dn10, locals.var_t7__blk675_dn11, locals.var_t7__blk675_dn12, locals.var_t7__blk675_dn17,)
    }
};
        locals.var_t7__blk675 = assign22010_e30776;
        locals.var_t7__blk675_dn0 = assign22010_e30776_d_n0;
        locals.var_t7__blk675_dn2 = assign22010_e30776_d_n2;
        locals.var_t7__blk675_dn6 = assign22010_e30776_d_n6;
        locals.var_t7__blk675_dn7 = assign22010_e30776_d_n7;
        locals.var_t7__blk675_dn10 = assign22010_e30776_d_n10;
        locals.var_t7__blk675_dn11 = assign22010_e30776_d_n11;
        locals.var_t7__blk675_dn12 = assign22010_e30776_d_n12;
        locals.var_t7__blk675_dn17 = assign22010_e30776_d_n17;
        locals.var_t7__blk675_rv = 0.0;

        let (assign22020_e30788, assign22020_e30788_d_n0, assign22020_e30788_d_n2, assign22020_e30788_d_n6, assign22020_e30788_d_n7, assign22020_e30788_d_n10, assign22020_e30788_d_n11, assign22020_e30788_d_n12, assign22020_e30788_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        (locals.var_qnsub_esi, locals.var_qnsub_esi_dn0, locals.var_qnsub_esi_dn2, locals.var_qnsub_esi_dn6, locals.var_qnsub_esi_dn7, locals.var_qnsub_esi_dn10, locals.var_qnsub_esi_dn11, locals.var_qnsub_esi_dn12, locals.var_qnsub_esi_dn17,)
    } else {
        (locals.var_t8__blk676, locals.var_t8__blk676_dn0, locals.var_t8__blk676_dn2, locals.var_t8__blk676_dn6, locals.var_t8__blk676_dn7, locals.var_t8__blk676_dn10, locals.var_t8__blk676_dn11, locals.var_t8__blk676_dn12, locals.var_t8__blk676_dn17,)
    }
};
        locals.var_t8__blk676 = assign22020_e30788;
        locals.var_t8__blk676_dn0 = assign22020_e30788_d_n0;
        locals.var_t8__blk676_dn2 = assign22020_e30788_d_n2;
        locals.var_t8__blk676_dn6 = assign22020_e30788_d_n6;
        locals.var_t8__blk676_dn7 = assign22020_e30788_d_n7;
        locals.var_t8__blk676_dn10 = assign22020_e30788_d_n10;
        locals.var_t8__blk676_dn11 = assign22020_e30788_d_n11;
        locals.var_t8__blk676_dn12 = assign22020_e30788_d_n12;
        locals.var_t8__blk676_dn17 = assign22020_e30788_d_n17;
        locals.var_t8__blk676_rv = 0.0;

        let (assign22030_e30802, assign22030_e30802_d_n0, assign22030_e30802_d_n2, assign22030_e30802_d_n6, assign22030_e30802_d_n7, assign22030_e30802_d_n10, assign22030_e30802_d_n11, assign22030_e30802_d_n12, assign22030_e30802_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22030_e30800: f64 = (locals.var_t8__blk676 / locals.var_t7__blk675);
        (assign22030_e30800, (((locals.var_t8__blk676_dn0 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn0)) / (locals.var_t7__blk675 * locals.var_t7__blk675)), (((locals.var_t8__blk676_dn2 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn2)) / (locals.var_t7__blk675 * locals.var_t7__blk675)), (((locals.var_t8__blk676_dn6 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn6)) / (locals.var_t7__blk675 * locals.var_t7__blk675)), (((locals.var_t8__blk676_dn7 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn7)) / (locals.var_t7__blk675 * locals.var_t7__blk675)), (((locals.var_t8__blk676_dn10 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn10)) / (locals.var_t7__blk675 * locals.var_t7__blk675)), (((locals.var_t8__blk676_dn11 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn11)) / (locals.var_t7__blk675 * locals.var_t7__blk675)), (((locals.var_t8__blk676_dn12 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn12)) / (locals.var_t7__blk675 * locals.var_t7__blk675)), (((locals.var_t8__blk676_dn17 * locals.var_t7__blk675) - (locals.var_t8__blk676 * locals.var_t7__blk675_dn17)) / (locals.var_t7__blk675 * locals.var_t7__blk675)),)
    } else {
        (locals.var_t3__blk670, locals.var_t3__blk670_dn0, locals.var_t3__blk670_dn2, locals.var_t3__blk670_dn6, locals.var_t3__blk670_dn7, locals.var_t3__blk670_dn10, locals.var_t3__blk670_dn11, locals.var_t3__blk670_dn12, locals.var_t3__blk670_dn17,)
    }
};
        locals.var_t3__blk670 = assign22030_e30802;
        locals.var_t3__blk670_dn0 = assign22030_e30802_d_n0;
        locals.var_t3__blk670_dn2 = assign22030_e30802_d_n2;
        locals.var_t3__blk670_dn6 = assign22030_e30802_d_n6;
        locals.var_t3__blk670_dn7 = assign22030_e30802_d_n7;
        locals.var_t3__blk670_dn10 = assign22030_e30802_d_n10;
        locals.var_t3__blk670_dn11 = assign22030_e30802_d_n11;
        locals.var_t3__blk670_dn12 = assign22030_e30802_d_n12;
        locals.var_t3__blk670_dn17 = assign22030_e30802_d_n17;
        locals.var_t3__blk670_rv = 0.0;

        let (assign22040_e30816, assign22040_e30816_d_n0, assign22040_e30816_d_n2, assign22040_e30816_d_n6, assign22040_e30816_d_n7, assign22040_e30816_d_n10, assign22040_e30816_d_n11, assign22040_e30816_d_n12, assign22040_e30816_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22040_e30814: f64 = (2.0 / locals.var_t8__blk676);
        (assign22040_e30814, (-((2.0 * locals.var_t8__blk676_dn0) / (locals.var_t8__blk676 * locals.var_t8__blk676))), (-((2.0 * locals.var_t8__blk676_dn2) / (locals.var_t8__blk676 * locals.var_t8__blk676))), (-((2.0 * locals.var_t8__blk676_dn6) / (locals.var_t8__blk676 * locals.var_t8__blk676))), (-((2.0 * locals.var_t8__blk676_dn7) / (locals.var_t8__blk676 * locals.var_t8__blk676))), (-((2.0 * locals.var_t8__blk676_dn10) / (locals.var_t8__blk676 * locals.var_t8__blk676))), (-((2.0 * locals.var_t8__blk676_dn11) / (locals.var_t8__blk676 * locals.var_t8__blk676))), (-((2.0 * locals.var_t8__blk676_dn12) / (locals.var_t8__blk676 * locals.var_t8__blk676))), (-((2.0 * locals.var_t8__blk676_dn17) / (locals.var_t8__blk676 * locals.var_t8__blk676))),)
    } else {
        (locals.var_t9__blk677, locals.var_t9__blk677_dn0, locals.var_t9__blk677_dn2, locals.var_t9__blk677_dn6, locals.var_t9__blk677_dn7, locals.var_t9__blk677_dn10, locals.var_t9__blk677_dn11, locals.var_t9__blk677_dn12, locals.var_t9__blk677_dn17,)
    }
};
        locals.var_t9__blk677 = assign22040_e30816;
        locals.var_t9__blk677_dn0 = assign22040_e30816_d_n0;
        locals.var_t9__blk677_dn2 = assign22040_e30816_d_n2;
        locals.var_t9__blk677_dn6 = assign22040_e30816_d_n6;
        locals.var_t9__blk677_dn7 = assign22040_e30816_d_n7;
        locals.var_t9__blk677_dn10 = assign22040_e30816_d_n10;
        locals.var_t9__blk677_dn11 = assign22040_e30816_d_n11;
        locals.var_t9__blk677_dn12 = assign22040_e30816_d_n12;
        locals.var_t9__blk677_dn17 = assign22040_e30816_d_n17;
        locals.var_t9__blk677_rv = 0.0;

        let (assign22050_e30830, assign22050_e30830_d_n0, assign22050_e30830_d_n2, assign22050_e30830_d_n6, assign22050_e30830_d_n7, assign22050_e30830_d_n10, assign22050_e30830_d_n11, assign22050_e30830_d_n12, assign22050_e30830_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22050_e30828: f64 = (locals.var_t9__blk677 * locals.var_t7__blk675);
        (assign22050_e30828, ((locals.var_t9__blk677_dn0 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn0)), ((locals.var_t9__blk677_dn2 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn2)), ((locals.var_t9__blk677_dn6 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn6)), ((locals.var_t9__blk677_dn7 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn7)), ((locals.var_t9__blk677_dn10 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn10)), ((locals.var_t9__blk677_dn11 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn11)), ((locals.var_t9__blk677_dn12 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn12)), ((locals.var_t9__blk677_dn17 * locals.var_t7__blk675) + (locals.var_t9__blk677 * locals.var_t7__blk675_dn17)),)
    } else {
        (locals.var_t4__blk671, locals.var_t4__blk671_dn0, locals.var_t4__blk671_dn2, locals.var_t4__blk671_dn6, locals.var_t4__blk671_dn7, locals.var_t4__blk671_dn10, locals.var_t4__blk671_dn11, locals.var_t4__blk671_dn12, locals.var_t4__blk671_dn17,)
    }
};
        locals.var_t4__blk671 = assign22050_e30830;
        locals.var_t4__blk671_dn0 = assign22050_e30830_d_n0;
        locals.var_t4__blk671_dn2 = assign22050_e30830_d_n2;
        locals.var_t4__blk671_dn6 = assign22050_e30830_d_n6;
        locals.var_t4__blk671_dn7 = assign22050_e30830_d_n7;
        locals.var_t4__blk671_dn10 = assign22050_e30830_d_n10;
        locals.var_t4__blk671_dn11 = assign22050_e30830_d_n11;
        locals.var_t4__blk671_dn12 = assign22050_e30830_d_n12;
        locals.var_t4__blk671_dn17 = assign22050_e30830_d_n17;
        locals.var_t4__blk671_rv = 0.0;

        let (assign22060_e30848, assign22060_e30848_d_n0, assign22060_e30848_d_n2, assign22060_e30848_d_n6, assign22060_e30848_d_n7, assign22060_e30848_d_n10, assign22060_e30848_d_n11, assign22060_e30848_d_n12, assign22060_e30848_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22060_e30842: f64 = (locals.var_t1__blk668 - locals.var_beta_inv);
        let assign22060_e30845: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign22060_e30846: f64 = (assign22060_e30842 - assign22060_e30845);
        (assign22060_e30846, (locals.var_t1__blk668_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk668_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk668_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk668_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk668_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk668_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk668_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk668_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk672, locals.var_t5__blk672_dn0, locals.var_t5__blk672_dn2, locals.var_t5__blk672_dn6, locals.var_t5__blk672_dn7, locals.var_t5__blk672_dn10, locals.var_t5__blk672_dn11, locals.var_t5__blk672_dn12, locals.var_t5__blk672_dn17,)
    }
};
        locals.var_t5__blk672 = assign22060_e30848;
        locals.var_t5__blk672_dn0 = assign22060_e30848_d_n0;
        locals.var_t5__blk672_dn2 = assign22060_e30848_d_n2;
        locals.var_t5__blk672_dn6 = assign22060_e30848_d_n6;
        locals.var_t5__blk672_dn7 = assign22060_e30848_d_n7;
        locals.var_t5__blk672_dn10 = assign22060_e30848_d_n10;
        locals.var_t5__blk672_dn11 = assign22060_e30848_d_n11;
        locals.var_t5__blk672_dn12 = assign22060_e30848_d_n12;
        locals.var_t5__blk672_dn17 = assign22060_e30848_d_n17;
        locals.var_t5__blk672_rv = 0.0;

        let (assign22070_e30864, assign22070_e30864_d_n0, assign22070_e30864_d_n2, assign22070_e30864_d_n6, assign22070_e30864_d_n7, assign22070_e30864_d_n10, assign22070_e30864_d_n11, assign22070_e30864_d_n12, assign22070_e30864_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22070_e30860: f64 = (p.p49 * locals.var_qhs);
        let assign22070_e30862: f64 = (assign22070_e30860 / locals.var_c_soi);
        (assign22070_e30862, ((p.p49 * locals.var_qhs_dn0) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn2) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn6) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn7) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn10) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn11) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn12) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn17) / locals.var_c_soi),)
    } else {
        (locals.var_dvbssub, locals.var_dvbssub_dn0, locals.var_dvbssub_dn2, locals.var_dvbssub_dn6, locals.var_dvbssub_dn7, locals.var_dvbssub_dn10, locals.var_dvbssub_dn11, locals.var_dvbssub_dn12, locals.var_dvbssub_dn17,)
    }
};
        locals.var_dvbssub = assign22070_e30864;
        locals.var_dvbssub_dn0 = assign22070_e30864_d_n0;
        locals.var_dvbssub_dn2 = assign22070_e30864_d_n2;
        locals.var_dvbssub_dn6 = assign22070_e30864_d_n6;
        locals.var_dvbssub_dn7 = assign22070_e30864_d_n7;
        locals.var_dvbssub_dn10 = assign22070_e30864_d_n10;
        locals.var_dvbssub_dn11 = assign22070_e30864_d_n11;
        locals.var_dvbssub_dn12 = assign22070_e30864_d_n12;
        locals.var_dvbssub_dn17 = assign22070_e30864_d_n17;
        locals.var_dvbssub_rv = 0.0;

        let (assign22080_e30880, assign22080_e30880_d_n0, assign22080_e30880_d_n2, assign22080_e30880_d_n6, assign22080_e30880_d_n7, assign22080_e30880_d_n10, assign22080_e30880_d_n11, assign22080_e30880_d_n12, assign22080_e30880_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22080_e30877: f64 = (locals.var_xvbs * locals.var_dvbssub);
        let assign22080_e30878: f64 = (locals.var_t5__blk672 - assign22080_e30877);
        (assign22080_e30878, (locals.var_t5__blk672_dn0 - (locals.var_xvbs * locals.var_dvbssub_dn0)), (locals.var_t5__blk672_dn2 - (locals.var_xvbs * locals.var_dvbssub_dn2)), (locals.var_t5__blk672_dn6 - (locals.var_xvbs * locals.var_dvbssub_dn6)), (locals.var_t5__blk672_dn7 - (locals.var_xvbs * locals.var_dvbssub_dn7)), (locals.var_t5__blk672_dn10 - (locals.var_xvbs * locals.var_dvbssub_dn10)), (locals.var_t5__blk672_dn11 - (locals.var_xvbs * locals.var_dvbssub_dn11)), (locals.var_t5__blk672_dn12 - (locals.var_xvbs * locals.var_dvbssub_dn12)), (locals.var_t5__blk672_dn17 - (locals.var_xvbs * locals.var_dvbssub_dn17)),)
    } else {
        (locals.var_t5__blk672, locals.var_t5__blk672_dn0, locals.var_t5__blk672_dn2, locals.var_t5__blk672_dn6, locals.var_t5__blk672_dn7, locals.var_t5__blk672_dn10, locals.var_t5__blk672_dn11, locals.var_t5__blk672_dn12, locals.var_t5__blk672_dn17,)
    }
};
        locals.var_t5__blk672 = assign22080_e30880;
        locals.var_t5__blk672_dn0 = assign22080_e30880_d_n0;
        locals.var_t5__blk672_dn2 = assign22080_e30880_d_n2;
        locals.var_t5__blk672_dn6 = assign22080_e30880_d_n6;
        locals.var_t5__blk672_dn7 = assign22080_e30880_d_n7;
        locals.var_t5__blk672_dn10 = assign22080_e30880_d_n10;
        locals.var_t5__blk672_dn11 = assign22080_e30880_d_n11;
        locals.var_t5__blk672_dn12 = assign22080_e30880_d_n12;
        locals.var_t5__blk672_dn17 = assign22080_e30880_d_n17;
        locals.var_t5__blk672_rv = 0.0;

        let (assign22090_e30896, assign22090_e30896_d_n0, assign22090_e30896_d_n2, assign22090_e30896_d_n6, assign22090_e30896_d_n7, assign22090_e30896_d_n10, assign22090_e30896_d_n11, assign22090_e30896_d_n12, assign22090_e30896_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22090_e30893: f64 = (locals.var_t4__blk671 * locals.var_t5__blk672);
        let assign22090_e30894: f64 = (1.0 + assign22090_e30893);
        (assign22090_e30894, ((locals.var_t4__blk671_dn0 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn0)), ((locals.var_t4__blk671_dn2 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn2)), ((locals.var_t4__blk671_dn6 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn6)), ((locals.var_t4__blk671_dn7 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn7)), ((locals.var_t4__blk671_dn10 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn10)), ((locals.var_t4__blk671_dn11 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn11)), ((locals.var_t4__blk671_dn12 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn12)), ((locals.var_t4__blk671_dn17 * locals.var_t5__blk672) + (locals.var_t4__blk671 * locals.var_t5__blk672_dn17)),)
    } else {
        (locals.var_t6w__blk674, locals.var_t6w__blk674_dn0, locals.var_t6w__blk674_dn2, locals.var_t6w__blk674_dn6, locals.var_t6w__blk674_dn7, locals.var_t6w__blk674_dn10, locals.var_t6w__blk674_dn11, locals.var_t6w__blk674_dn12, locals.var_t6w__blk674_dn17,)
    }
};
        locals.var_t6w__blk674 = assign22090_e30896;
        locals.var_t6w__blk674_dn0 = assign22090_e30896_d_n0;
        locals.var_t6w__blk674_dn2 = assign22090_e30896_d_n2;
        locals.var_t6w__blk674_dn6 = assign22090_e30896_d_n6;
        locals.var_t6w__blk674_dn7 = assign22090_e30896_d_n7;
        locals.var_t6w__blk674_dn10 = assign22090_e30896_d_n10;
        locals.var_t6w__blk674_dn11 = assign22090_e30896_d_n11;
        locals.var_t6w__blk674_dn12 = assign22090_e30896_d_n12;
        locals.var_t6w__blk674_dn17 = assign22090_e30896_d_n17;
        locals.var_t6w__blk674_rv = 0.0;

        let (assign22100_e30917, assign22100_e30917_d_n0, assign22100_e30917_d_n2, assign22100_e30917_d_n6, assign22100_e30917_d_n7, assign22100_e30917_d_n10, assign22100_e30917_d_n11, assign22100_e30917_d_n12, assign22100_e30917_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22100_e30908: f64 = (locals.var_t6w__blk674 * locals.var_t6w__blk674);
        let assign22100_e30911: f64 = (4.0 * 0.001);
        let assign22100_e30913: f64 = (assign22100_e30911 * 0.001);
        let assign22100_e30914: f64 = (assign22100_e30908 + assign22100_e30913);
        let assign22100_e30915: f64 = (assign22100_e30914).sqrt();
        (assign22100_e30915, (((locals.var_t6w__blk674_dn0 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn0)) / (2.0 * assign22100_e30915)), (((locals.var_t6w__blk674_dn2 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn2)) / (2.0 * assign22100_e30915)), (((locals.var_t6w__blk674_dn6 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn6)) / (2.0 * assign22100_e30915)), (((locals.var_t6w__blk674_dn7 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn7)) / (2.0 * assign22100_e30915)), (((locals.var_t6w__blk674_dn10 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn10)) / (2.0 * assign22100_e30915)), (((locals.var_t6w__blk674_dn11 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn11)) / (2.0 * assign22100_e30915)), (((locals.var_t6w__blk674_dn12 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn12)) / (2.0 * assign22100_e30915)), (((locals.var_t6w__blk674_dn17 * locals.var_t6w__blk674) + (locals.var_t6w__blk674 * locals.var_t6w__blk674_dn17)) / (2.0 * assign22100_e30915)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22100_e30917;
        locals.var_tmf1_dn0 = assign22100_e30917_d_n0;
        locals.var_tmf1_dn2 = assign22100_e30917_d_n2;
        locals.var_tmf1_dn6 = assign22100_e30917_d_n6;
        locals.var_tmf1_dn7 = assign22100_e30917_d_n7;
        locals.var_tmf1_dn10 = assign22100_e30917_d_n10;
        locals.var_tmf1_dn11 = assign22100_e30917_d_n11;
        locals.var_tmf1_dn12 = assign22100_e30917_d_n12;
        locals.var_tmf1_dn17 = assign22100_e30917_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22110_e30937, assign22110_e30937_d_n0, assign22110_e30937_d_n2, assign22110_e30937_d_n6, assign22110_e30937_d_n7, assign22110_e30937_d_n10, assign22110_e30937_d_n11, assign22110_e30937_d_n12, assign22110_e30937_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22110_e30930: f64 = (locals.var_t6w__blk674 + locals.var_tmf1);
        let assign22110_e30931: f64 = (0.5 * assign22110_e30930);
        let assign22110_e30934: f64 = (1e-10 * 0.001);
        let assign22110_e30935: f64 = (assign22110_e30931 + assign22110_e30934);
        (assign22110_e30935, (0.5 * (locals.var_t6w__blk674_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk674_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk674_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk674_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk674_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk674_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk674_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk674_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk673, locals.var_t6__blk673_dn0, locals.var_t6__blk673_dn2, locals.var_t6__blk673_dn6, locals.var_t6__blk673_dn7, locals.var_t6__blk673_dn10, locals.var_t6__blk673_dn11, locals.var_t6__blk673_dn12, locals.var_t6__blk673_dn17,)
    }
};
        locals.var_t6__blk673 = assign22110_e30937;
        locals.var_t6__blk673_dn0 = assign22110_e30937_d_n0;
        locals.var_t6__blk673_dn2 = assign22110_e30937_d_n2;
        locals.var_t6__blk673_dn6 = assign22110_e30937_d_n6;
        locals.var_t6__blk673_dn7 = assign22110_e30937_d_n7;
        locals.var_t6__blk673_dn10 = assign22110_e30937_d_n10;
        locals.var_t6__blk673_dn11 = assign22110_e30937_d_n11;
        locals.var_t6__blk673_dn12 = assign22110_e30937_d_n12;
        locals.var_t6__blk673_dn17 = assign22110_e30937_d_n17;
        locals.var_t6__blk673_rv = 0.0;

        let assign22120_e30940: f64 = if locals.var_t6__blk673 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign22120_e30940;
        locals.var_guard683_rv = 0.0;

        let (assign22130_e30954, assign22130_e30954_d_n0, assign22130_e30954_d_n2, assign22130_e30954_d_n6, assign22130_e30954_d_n7, assign22130_e30954_d_n10, assign22130_e30954_d_n11, assign22130_e30954_d_n12, assign22130_e30954_d_n17,) = {
    if (((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk673, locals.var_t6__blk673_dn0, locals.var_t6__blk673_dn2, locals.var_t6__blk673_dn6, locals.var_t6__blk673_dn7, locals.var_t6__blk673_dn10, locals.var_t6__blk673_dn11, locals.var_t6__blk673_dn12, locals.var_t6__blk673_dn17,)
    }
};
        locals.var_t6__blk673 = assign22130_e30954;
        locals.var_t6__blk673_dn0 = assign22130_e30954_d_n0;
        locals.var_t6__blk673_dn2 = assign22130_e30954_d_n2;
        locals.var_t6__blk673_dn6 = assign22130_e30954_d_n6;
        locals.var_t6__blk673_dn7 = assign22130_e30954_d_n7;
        locals.var_t6__blk673_dn10 = assign22130_e30954_d_n10;
        locals.var_t6__blk673_dn11 = assign22130_e30954_d_n11;
        locals.var_t6__blk673_dn12 = assign22130_e30954_d_n12;
        locals.var_t6__blk673_dn17 = assign22130_e30954_d_n17;
        locals.var_t6__blk673_rv = 0.0;

        let (assign22140_e30968, assign22140_e30968_d_n0, assign22140_e30968_d_n2, assign22140_e30968_d_n6, assign22140_e30968_d_n7, assign22140_e30968_d_n10, assign22140_e30968_d_n11, assign22140_e30968_d_n12, assign22140_e30968_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22140_e30966: f64 = (locals.var_t6__blk673 + 1e-50);
        (assign22140_e30966, locals.var_t6__blk673_dn0, locals.var_t6__blk673_dn2, locals.var_t6__blk673_dn6, locals.var_t6__blk673_dn7, locals.var_t6__blk673_dn10, locals.var_t6__blk673_dn11, locals.var_t6__blk673_dn12, locals.var_t6__blk673_dn17,)
    } else {
        (locals.var_t6__blk673, locals.var_t6__blk673_dn0, locals.var_t6__blk673_dn2, locals.var_t6__blk673_dn6, locals.var_t6__blk673_dn7, locals.var_t6__blk673_dn10, locals.var_t6__blk673_dn11, locals.var_t6__blk673_dn12, locals.var_t6__blk673_dn17,)
    }
};
        locals.var_t6__blk673 = assign22140_e30968;
        locals.var_t6__blk673_dn0 = assign22140_e30968_d_n0;
        locals.var_t6__blk673_dn2 = assign22140_e30968_d_n2;
        locals.var_t6__blk673_dn6 = assign22140_e30968_d_n6;
        locals.var_t6__blk673_dn7 = assign22140_e30968_d_n7;
        locals.var_t6__blk673_dn10 = assign22140_e30968_d_n10;
        locals.var_t6__blk673_dn11 = assign22140_e30968_d_n11;
        locals.var_t6__blk673_dn12 = assign22140_e30968_d_n12;
        locals.var_t6__blk673_dn17 = assign22140_e30968_d_n17;
        locals.var_t6__blk673_rv = 0.0;

        let (assign22150_e30981, assign22150_e30981_d_n0, assign22150_e30981_d_n2, assign22150_e30981_d_n6, assign22150_e30981_d_n7, assign22150_e30981_d_n10, assign22150_e30981_d_n11, assign22150_e30981_d_n12, assign22150_e30981_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22150_e30979: f64 = (locals.var_t6__blk673).sqrt();
        (assign22150_e30979, (locals.var_t6__blk673_dn0 / (2.0 * assign22150_e30979)), (locals.var_t6__blk673_dn2 / (2.0 * assign22150_e30979)), (locals.var_t6__blk673_dn6 / (2.0 * assign22150_e30979)), (locals.var_t6__blk673_dn7 / (2.0 * assign22150_e30979)), (locals.var_t6__blk673_dn10 / (2.0 * assign22150_e30979)), (locals.var_t6__blk673_dn11 / (2.0 * assign22150_e30979)), (locals.var_t6__blk673_dn12 / (2.0 * assign22150_e30979)), (locals.var_t6__blk673_dn17 / (2.0 * assign22150_e30979)),)
    } else {
        (locals.var_t6__blk673, locals.var_t6__blk673_dn0, locals.var_t6__blk673_dn2, locals.var_t6__blk673_dn6, locals.var_t6__blk673_dn7, locals.var_t6__blk673_dn10, locals.var_t6__blk673_dn11, locals.var_t6__blk673_dn12, locals.var_t6__blk673_dn17,)
    }
};
        locals.var_t6__blk673 = assign22150_e30981;
        locals.var_t6__blk673_dn0 = assign22150_e30981_d_n0;
        locals.var_t6__blk673_dn2 = assign22150_e30981_d_n2;
        locals.var_t6__blk673_dn6 = assign22150_e30981_d_n6;
        locals.var_t6__blk673_dn7 = assign22150_e30981_d_n7;
        locals.var_t6__blk673_dn10 = assign22150_e30981_d_n10;
        locals.var_t6__blk673_dn11 = assign22150_e30981_d_n11;
        locals.var_t6__blk673_dn12 = assign22150_e30981_d_n12;
        locals.var_t6__blk673_dn17 = assign22150_e30981_d_n17;
        locals.var_t6__blk673_rv = 0.0;

        let (assign22160_e31001, assign22160_e31001_d_n0, assign22160_e31001_d_n2, assign22160_e31001_d_n6, assign22160_e31001_d_n7, assign22160_e31001_d_n10, assign22160_e31001_d_n11, assign22160_e31001_d_n12, assign22160_e31001_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22160_e30993: f64 = (locals.var_t1__blk668 * locals.var_uc_svgs);
        let assign22160_e30997: f64 = (1.0 - locals.var_t6__blk673);
        let assign22160_e30998: f64 = (locals.var_t3__blk670 * assign22160_e30997);
        let assign22160_e30999: f64 = (assign22160_e30993 + assign22160_e30998);
        (assign22160_e30999, ((locals.var_t1__blk668_dn0 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn0 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn0)))), ((locals.var_t1__blk668_dn2 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn2 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn2)))), ((locals.var_t1__blk668_dn6 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn6 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn6)))), ((locals.var_t1__blk668_dn7 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn7 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn7)))), ((locals.var_t1__blk668_dn10 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn10 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn10)))), ((locals.var_t1__blk668_dn11 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn11 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn11)))), ((locals.var_t1__blk668_dn12 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn12 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn12)))), ((locals.var_t1__blk668_dn17 * locals.var_uc_svgs) + ((locals.var_t3__blk670_dn17 * assign22160_e30997) + (locals.var_t3__blk670 * (-locals.var_t6__blk673_dn17)))),)
    } else {
        (locals.var_psislsat__blk678, locals.var_psislsat__blk678_dn0, locals.var_psislsat__blk678_dn2, locals.var_psislsat__blk678_dn6, locals.var_psislsat__blk678_dn7, locals.var_psislsat__blk678_dn10, locals.var_psislsat__blk678_dn11, locals.var_psislsat__blk678_dn12, locals.var_psislsat__blk678_dn17,)
    }
};
        locals.var_psislsat__blk678 = assign22160_e31001;
        locals.var_psislsat__blk678_dn0 = assign22160_e31001_d_n0;
        locals.var_psislsat__blk678_dn2 = assign22160_e31001_d_n2;
        locals.var_psislsat__blk678_dn6 = assign22160_e31001_d_n6;
        locals.var_psislsat__blk678_dn7 = assign22160_e31001_d_n7;
        locals.var_psislsat__blk678_dn10 = assign22160_e31001_d_n10;
        locals.var_psislsat__blk678_dn11 = assign22160_e31001_d_n11;
        locals.var_psislsat__blk678_dn12 = assign22160_e31001_d_n12;
        locals.var_psislsat__blk678_dn17 = assign22160_e31001_d_n17;
        locals.var_psislsat__blk678_rv = 0.0;

        let (assign22170_e31023, assign22170_e31023_d_n0, assign22170_e31023_d_n2, assign22170_e31023_d_n6, assign22170_e31023_d_n7, assign22170_e31023_d_n10, assign22170_e31023_d_n11, assign22170_e31023_d_n12, assign22170_e31023_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22170_e31013: f64 = (p.p122 * locals.var_vdsz);
        let assign22170_e31015: f64 = (assign22170_e31013 + locals.var_ps0z);
        let assign22170_e31018: f64 = (locals.var_xgate * locals.var_zvgs);
        let assign22170_e31020: f64 = (assign22170_e31018 * locals.var_psislsat__blk678);
        let assign22170_e31021: f64 = (assign22170_e31015 - assign22170_e31020);
        (assign22170_e31021, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0z_dn0) - (assign22170_e31018 * locals.var_psislsat__blk678_dn0)), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0z_dn2) - (assign22170_e31018 * locals.var_psislsat__blk678_dn2)), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0z_dn6) - (assign22170_e31018 * locals.var_psislsat__blk678_dn6)), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0z_dn7) - (assign22170_e31018 * locals.var_psislsat__blk678_dn7)), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0z_dn10) - (assign22170_e31018 * locals.var_psislsat__blk678_dn10)), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0z_dn11) - (assign22170_e31018 * locals.var_psislsat__blk678_dn11)), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0z_dn12) - (assign22170_e31018 * locals.var_psislsat__blk678_dn12)), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0z_dn17) - (assign22170_e31018 * locals.var_psislsat__blk678_dn17)),)
    } else {
        (locals.var_psisubsat__blk679, locals.var_psisubsat__blk679_dn0, locals.var_psisubsat__blk679_dn2, locals.var_psisubsat__blk679_dn6, locals.var_psisubsat__blk679_dn7, locals.var_psisubsat__blk679_dn10, locals.var_psisubsat__blk679_dn11, locals.var_psisubsat__blk679_dn12, locals.var_psisubsat__blk679_dn17,)
    }
};
        locals.var_psisubsat__blk679 = assign22170_e31023;
        locals.var_psisubsat__blk679_dn0 = assign22170_e31023_d_n0;
        locals.var_psisubsat__blk679_dn2 = assign22170_e31023_d_n2;
        locals.var_psisubsat__blk679_dn6 = assign22170_e31023_d_n6;
        locals.var_psisubsat__blk679_dn7 = assign22170_e31023_d_n7;
        locals.var_psisubsat__blk679_dn10 = assign22170_e31023_d_n10;
        locals.var_psisubsat__blk679_dn11 = assign22170_e31023_d_n11;
        locals.var_psisubsat__blk679_dn12 = assign22170_e31023_d_n12;
        locals.var_psisubsat__blk679_dn17 = assign22170_e31023_d_n17;
        locals.var_psisubsat__blk679_rv = 0.0;

        let (assign22180_e31044, assign22180_e31044_d_n0, assign22180_e31044_d_n2, assign22180_e31044_d_n6, assign22180_e31044_d_n7, assign22180_e31044_d_n10, assign22180_e31044_d_n11, assign22180_e31044_d_n12, assign22180_e31044_d_n17,) = {
    if ((((locals.var_guard667 != 0.0) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign22180_e31035: f64 = (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679);
        let assign22180_e31038: f64 = (4.0 * 0.01);
        let assign22180_e31040: f64 = (assign22180_e31038 * 0.01);
        let assign22180_e31041: f64 = (assign22180_e31035 + assign22180_e31040);
        let assign22180_e31042: f64 = (assign22180_e31041).sqrt();
        (assign22180_e31042, (((locals.var_psisubsat__blk679_dn0 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn0)) / (2.0 * assign22180_e31042)), (((locals.var_psisubsat__blk679_dn2 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn2)) / (2.0 * assign22180_e31042)), (((locals.var_psisubsat__blk679_dn6 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn6)) / (2.0 * assign22180_e31042)), (((locals.var_psisubsat__blk679_dn7 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn7)) / (2.0 * assign22180_e31042)), (((locals.var_psisubsat__blk679_dn10 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn10)) / (2.0 * assign22180_e31042)), (((locals.var_psisubsat__blk679_dn11 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn11)) / (2.0 * assign22180_e31042)), (((locals.var_psisubsat__blk679_dn12 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn12)) / (2.0 * assign22180_e31042)), (((locals.var_psisubsat__blk679_dn17 * locals.var_psisubsat__blk679) + (locals.var_psisubsat__blk679 * locals.var_psisubsat__blk679_dn17)) / (2.0 * assign22180_e31042)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22180_e31044;
        locals.var_tmf1_dn0 = assign22180_e31044_d_n0;
        locals.var_tmf1_dn2 = assign22180_e31044_d_n2;
        locals.var_tmf1_dn6 = assign22180_e31044_d_n6;
        locals.var_tmf1_dn7 = assign22180_e31044_d_n7;
        locals.var_tmf1_dn10 = assign22180_e31044_d_n10;
        locals.var_tmf1_dn11 = assign22180_e31044_d_n11;
        locals.var_tmf1_dn12 = assign22180_e31044_d_n12;
        locals.var_tmf1_dn17 = assign22180_e31044_d_n17;
        locals.var_tmf1_rv = 0.0;

    }
}
