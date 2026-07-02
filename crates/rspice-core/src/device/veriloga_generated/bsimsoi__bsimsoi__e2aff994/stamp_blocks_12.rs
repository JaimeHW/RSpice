#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_192(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign58190_e94980, assign58190_e94980_d_n3, assign58190_e94980_d_n4, assign58190_e94980_d_n5, assign58190_e94980_d_n6, assign58190_e94980_d_n7, assign58190_e94980_d_n8, assign58190_e94980_d_n9, assign58190_e94980_d_n10, assign58190_e94980_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58190_e94977: f64 = (2.0 * locals.var_invgamg2);
        let assign58190_e94978: f64 = (locals.var_t6 + assign58190_e94977);
        (assign58190_e94978, locals.var_t6_dn3, (locals.var_t6_dn4 + (2.0 * locals.var_invgamg2_dn4)), (locals.var_t6_dn5 + (2.0 * locals.var_invgamg2_dn5)), locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign58190_e94980;
        locals.var_t7_dn3 = assign58190_e94980_d_n3;
        locals.var_t7_dn4 = assign58190_e94980_d_n4;
        locals.var_t7_dn5 = assign58190_e94980_d_n5;
        locals.var_t7_dn6 = assign58190_e94980_d_n6;
        locals.var_t7_dn7 = assign58190_e94980_d_n7;
        locals.var_t7_dn8 = assign58190_e94980_d_n8;
        locals.var_t7_dn9 = assign58190_e94980_d_n9;
        locals.var_t7_dn10 = assign58190_e94980_d_n10;
        locals.var_t7_dn11 = assign58190_e94980_d_n11;

        let (assign58200_e94991, assign58200_e94991_d_n3, assign58200_e94991_d_n4, assign58200_e94991_d_n5, assign58200_e94991_d_n6, assign58200_e94991_d_n7, assign58200_e94991_d_n8, assign58200_e94991_d_n9, assign58200_e94991_d_n10, assign58200_e94991_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58200_e94987: f64 = (0.3333333333333333 * locals.var_dqsd2);
        let assign58200_e94989: f64 = (assign58200_e94987 * locals.var_t5);
        (assign58200_e94989, (((0.3333333333333333 * locals.var_dqsd2_dn3) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn3)), (((0.3333333333333333 * locals.var_dqsd2_dn4) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn4)), (((0.3333333333333333 * locals.var_dqsd2_dn5) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn5)), (((0.3333333333333333 * locals.var_dqsd2_dn6) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn6)), (((0.3333333333333333 * locals.var_dqsd2_dn7) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn7)), (((0.3333333333333333 * locals.var_dqsd2_dn8) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn8)), (((0.3333333333333333 * locals.var_dqsd2_dn9) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn9)), (((0.3333333333333333 * locals.var_dqsd2_dn10) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn10)), (((0.3333333333333333 * locals.var_dqsd2_dn11) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign58200_e94991;
        locals.var_t8_dn3 = assign58200_e94991_d_n3;
        locals.var_t8_dn4 = assign58200_e94991_d_n4;
        locals.var_t8_dn5 = assign58200_e94991_d_n5;
        locals.var_t8_dn6 = assign58200_e94991_d_n6;
        locals.var_t8_dn7 = assign58200_e94991_d_n7;
        locals.var_t8_dn8 = assign58200_e94991_d_n8;
        locals.var_t8_dn9 = assign58200_e94991_d_n9;
        locals.var_t8_dn10 = assign58200_e94991_d_n10;
        locals.var_t8_dn11 = assign58200_e94991_d_n11;

        let (assign58210_e95010, assign58210_e95010_d_n3, assign58210_e95010_d_n4, assign58210_e95010_d_n5, assign58210_e95010_d_n6, assign58210_e95010_d_n7, assign58210_e95010_d_n8, assign58210_e95010_d_n9, assign58210_e95010_d_n10, assign58210_e95010_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58210_e94999: f64 = (2.0 * locals.var_tempd);
        let assign58210_e95001: f64 = (assign58210_e94999 - 1.0);
        let assign58210_e95002: f64 = (locals.var_sid * assign58210_e95001);
        let assign58210_e95005: f64 = (2.0 * locals.var_tempd);
        let assign58210_e95007: f64 = (assign58210_e95005 + 1.0);
        let assign58210_e95008: f64 = (assign58210_e95002 / assign58210_e95007);
        (assign58210_e95008, (((((locals.var_sid_dn3 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn3))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn3))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn4 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn4))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn4))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn5 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn5))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn5))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn6 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn6))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn6))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn7 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn7))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn7))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn8 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn8))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn8))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn9 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn9))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn9))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn10 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn10))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn10))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn11 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn11))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn11))) / (assign58210_e95007 * assign58210_e95007)),)
    } else {
        (locals.var_dqgeff, locals.var_dqgeff_dn3, locals.var_dqgeff_dn4, locals.var_dqgeff_dn5, locals.var_dqgeff_dn6, locals.var_dqgeff_dn7, locals.var_dqgeff_dn8, locals.var_dqgeff_dn9, locals.var_dqgeff_dn10, locals.var_dqgeff_dn11,)
    }
};
        locals.var_dqgeff = assign58210_e95010;
        locals.var_dqgeff_dn3 = assign58210_e95010_d_n3;
        locals.var_dqgeff_dn4 = assign58210_e95010_d_n4;
        locals.var_dqgeff_dn5 = assign58210_e95010_d_n5;
        locals.var_dqgeff_dn6 = assign58210_e95010_d_n6;
        locals.var_dqgeff_dn7 = assign58210_e95010_d_n7;
        locals.var_dqgeff_dn8 = assign58210_e95010_d_n8;
        locals.var_dqgeff_dn9 = assign58210_e95010_d_n9;
        locals.var_dqgeff_dn10 = assign58210_e95010_d_n10;
        locals.var_dqgeff_dn11 = assign58210_e95010_d_n11;

        let (assign58220_e95027, assign58220_e95027_d_n3, assign58220_e95027_d_n4, assign58220_e95027_d_n5, assign58220_e95027_d_n6, assign58220_e95027_d_n7, assign58220_e95027_d_n8, assign58220_e95027_d_n9, assign58220_e95027_d_n10, assign58220_e95027_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58220_e95019: f64 = (locals.var_nq - 1.0);
        let assign58220_e95020: f64 = (2.0 * assign58220_e95019);
        let assign58220_e95022: f64 = (assign58220_e95020 * locals.var_qdeff);
        let assign58220_e95023: f64 = (locals.var_vgpqm - assign58220_e95022);
        let assign58220_e95025: f64 = (assign58220_e95023 + locals.var_dqgeff);
        (assign58220_e95025, ((locals.var_vgpqm_dn3 - (((2.0 * locals.var_nq_dn3) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn3))) + locals.var_dqgeff_dn3), ((locals.var_vgpqm_dn4 - (((2.0 * locals.var_nq_dn4) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn4))) + locals.var_dqgeff_dn4), ((locals.var_vgpqm_dn5 - (((2.0 * locals.var_nq_dn5) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn5))) + locals.var_dqgeff_dn5), ((locals.var_vgpqm_dn6 - (((2.0 * locals.var_nq_dn6) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn6))) + locals.var_dqgeff_dn6), ((locals.var_vgpqm_dn7 - (((2.0 * locals.var_nq_dn7) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn7))) + locals.var_dqgeff_dn7), ((locals.var_vgpqm_dn8 - (((2.0 * locals.var_nq_dn8) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn8))) + locals.var_dqgeff_dn8), ((locals.var_vgpqm_dn9 - (((2.0 * locals.var_nq_dn9) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn9))) + locals.var_dqgeff_dn9), ((locals.var_vgpqm_dn10 - (((2.0 * locals.var_nq_dn10) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn10))) + locals.var_dqgeff_dn10), ((locals.var_vgpqm_dn11 - (((2.0 * locals.var_nq_dn11) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn11))) + locals.var_dqgeff_dn11),)
    } else {
        (locals.var_qbeff, locals.var_qbeff_dn3, locals.var_qbeff_dn4, locals.var_qbeff_dn5, locals.var_qbeff_dn6, locals.var_qbeff_dn7, locals.var_qbeff_dn8, locals.var_qbeff_dn9, locals.var_qbeff_dn10, locals.var_qbeff_dn11,)
    }
};
        locals.var_qbeff = assign58220_e95027;
        locals.var_qbeff_dn3 = assign58220_e95027_d_n3;
        locals.var_qbeff_dn4 = assign58220_e95027_d_n4;
        locals.var_qbeff_dn5 = assign58220_e95027_d_n5;
        locals.var_qbeff_dn6 = assign58220_e95027_d_n6;
        locals.var_qbeff_dn7 = assign58220_e95027_d_n7;
        locals.var_qbeff_dn8 = assign58220_e95027_d_n8;
        locals.var_qbeff_dn9 = assign58220_e95027_d_n9;
        locals.var_qbeff_dn10 = assign58220_e95027_d_n10;
        locals.var_qbeff_dn11 = assign58220_e95027_d_n11;

        let (assign58230_e95054, assign58230_e95054_d_n3, assign58230_e95054_d_n4, assign58230_e95054_d_n5, assign58230_e95054_d_n6, assign58230_e95054_d_n7, assign58230_e95054_d_n8, assign58230_e95054_d_n9, assign58230_e95054_d_n10, assign58230_e95054_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58230_e95035: f64 = (locals.var_t1 + locals.var_t2);
        let assign58230_e95038: f64 = (locals.var_t4 * locals.var_t7);
        let assign58230_e95042: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign58230_e95044: f64 = (assign58230_e95042 + locals.var_t8);
        let assign58230_e95045: f64 = (locals.var_nq * assign58230_e95044);
        let assign58230_e95046: f64 = (assign58230_e95038 - assign58230_e95045);
        let assign58230_e95047: f64 = (assign58230_e95035 + assign58230_e95046);
        let assign58230_e95048: f64 = (locals.var_inv_mdl * assign58230_e95047);
        let assign58230_e95051: f64 = (locals.var_mdl_less_1 * locals.var_qbeff);
        let assign58230_e95052: f64 = (assign58230_e95048 + assign58230_e95051);
        (assign58230_e95052, (((locals.var_inv_mdl_dn3 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn3 + locals.var_t2_dn3) + (((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) - ((locals.var_nq_dn3 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn3 + locals.var_qdeff_dn3) + locals.var_t8_dn3))))))) + ((locals.var_mdl_less_1_dn3 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn3))), (((locals.var_inv_mdl_dn4 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn4 + locals.var_t2_dn4) + (((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) - ((locals.var_nq_dn4 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn4 + locals.var_qdeff_dn4) + locals.var_t8_dn4))))))) + ((locals.var_mdl_less_1_dn4 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn4))), (((locals.var_inv_mdl_dn5 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn5 + locals.var_t2_dn5) + (((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) - ((locals.var_nq_dn5 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn5 + locals.var_qdeff_dn5) + locals.var_t8_dn5))))))) + ((locals.var_mdl_less_1_dn5 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn5))), (((locals.var_inv_mdl_dn6 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn6 + locals.var_t2_dn6) + (((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) - ((locals.var_nq_dn6 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn6 + locals.var_qdeff_dn6) + locals.var_t8_dn6))))))) + ((locals.var_mdl_less_1_dn6 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn6))), (((locals.var_inv_mdl_dn7 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn7 + locals.var_t2_dn7) + (((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) - ((locals.var_nq_dn7 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn7 + locals.var_qdeff_dn7) + locals.var_t8_dn7))))))) + ((locals.var_mdl_less_1_dn7 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn7))), (((locals.var_inv_mdl_dn8 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn8 + locals.var_t2_dn8) + (((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) - ((locals.var_nq_dn8 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn8 + locals.var_qdeff_dn8) + locals.var_t8_dn8))))))) + ((locals.var_mdl_less_1_dn8 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn8))), (((locals.var_inv_mdl_dn9 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn9 + locals.var_t2_dn9) + (((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) - ((locals.var_nq_dn9 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn9 + locals.var_qdeff_dn9) + locals.var_t8_dn9))))))) + ((locals.var_mdl_less_1_dn9 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn9))), (((locals.var_inv_mdl_dn10 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn10 + locals.var_t2_dn10) + (((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) - ((locals.var_nq_dn10 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn10 + locals.var_qdeff_dn10) + locals.var_t8_dn10))))))) + ((locals.var_mdl_less_1_dn10 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn10))), (((locals.var_inv_mdl_dn11 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn11 + locals.var_t2_dn11) + (((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) - ((locals.var_nq_dn11 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn11 + locals.var_qdeff_dn11) + locals.var_t8_dn11))))))) + ((locals.var_mdl_less_1_dn11 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn11))),)
    } else {
        (locals.var_qb_1, locals.var_qb_1_dn3, locals.var_qb_1_dn4, locals.var_qb_1_dn5, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, locals.var_qb_1_dn9, locals.var_qb_1_dn10, locals.var_qb_1_dn11,)
    }
};
        locals.var_qb_1 = assign58230_e95054;
        locals.var_qb_1_dn3 = assign58230_e95054_d_n3;
        locals.var_qb_1_dn4 = assign58230_e95054_d_n4;
        locals.var_qb_1_dn5 = assign58230_e95054_d_n5;
        locals.var_qb_1_dn6 = assign58230_e95054_d_n6;
        locals.var_qb_1_dn7 = assign58230_e95054_d_n7;
        locals.var_qb_1_dn8 = assign58230_e95054_d_n8;
        locals.var_qb_1_dn9 = assign58230_e95054_d_n9;
        locals.var_qb_1_dn10 = assign58230_e95054_d_n10;
        locals.var_qb_1_dn11 = assign58230_e95054_d_n11;

        let (assign58240_e95063, assign58240_e95063_d_n3, assign58240_e95063_d_n4, assign58240_e95063_d_n5, assign58240_e95063_d_n6, assign58240_e95063_d_n7, assign58240_e95063_d_n8, assign58240_e95063_d_n9, assign58240_e95063_d_n10, assign58240_e95063_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58240_e95061: f64 = (locals.var_qs_1 + locals.var_qdeff);
        (assign58240_e95061, (locals.var_qs_1_dn3 + locals.var_qdeff_dn3), (locals.var_qs_1_dn4 + locals.var_qdeff_dn4), (locals.var_qs_1_dn5 + locals.var_qdeff_dn5), (locals.var_qs_1_dn6 + locals.var_qdeff_dn6), (locals.var_qs_1_dn7 + locals.var_qdeff_dn7), (locals.var_qs_1_dn8 + locals.var_qdeff_dn8), (locals.var_qs_1_dn9 + locals.var_qdeff_dn9), (locals.var_qs_1_dn10 + locals.var_qdeff_dn10), (locals.var_qs_1_dn11 + locals.var_qdeff_dn11),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign58240_e95063;
        locals.var_t9_dn3 = assign58240_e95063_d_n3;
        locals.var_t9_dn4 = assign58240_e95063_d_n4;
        locals.var_t9_dn5 = assign58240_e95063_d_n5;
        locals.var_t9_dn6 = assign58240_e95063_d_n6;
        locals.var_t9_dn7 = assign58240_e95063_d_n7;
        locals.var_t9_dn8 = assign58240_e95063_d_n8;
        locals.var_t9_dn9 = assign58240_e95063_d_n9;
        locals.var_t9_dn10 = assign58240_e95063_d_n10;
        locals.var_t9_dn11 = assign58240_e95063_d_n11;

        let (assign58250_e95074, assign58250_e95074_d_n3, assign58250_e95074_d_n4, assign58250_e95074_d_n5, assign58250_e95074_d_n6, assign58250_e95074_d_n7, assign58250_e95074_d_n8, assign58250_e95074_d_n9, assign58250_e95074_d_n10, assign58250_e95074_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58250_e95070: f64 = (locals.var_dqsd2 * locals.var_t5);
        let assign58250_e95072: f64 = (assign58250_e95070 * locals.var_t5);
        (assign58250_e95072, ((((locals.var_dqsd2_dn3 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn3)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn3)), ((((locals.var_dqsd2_dn4 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn4)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn4)), ((((locals.var_dqsd2_dn5 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn5)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn5)), ((((locals.var_dqsd2_dn6 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn6)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn6)), ((((locals.var_dqsd2_dn7 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn7)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn7)), ((((locals.var_dqsd2_dn8 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn8)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn8)), ((((locals.var_dqsd2_dn9 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn9)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn9)), ((((locals.var_dqsd2_dn10 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn10)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn10)), ((((locals.var_dqsd2_dn11 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn11)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign58250_e95074;
        locals.var_t10_dn3 = assign58250_e95074_d_n3;
        locals.var_t10_dn4 = assign58250_e95074_d_n4;
        locals.var_t10_dn5 = assign58250_e95074_d_n5;
        locals.var_t10_dn6 = assign58250_e95074_d_n6;
        locals.var_t10_dn7 = assign58250_e95074_d_n7;
        locals.var_t10_dn8 = assign58250_e95074_d_n8;
        locals.var_t10_dn9 = assign58250_e95074_d_n9;
        locals.var_t10_dn10 = assign58250_e95074_d_n10;
        locals.var_t10_dn11 = assign58250_e95074_d_n11;

        let (assign58260_e95099, assign58260_e95099_d_n3, assign58260_e95099_d_n4, assign58260_e95099_d_n5, assign58260_e95099_d_n6, assign58260_e95099_d_n7, assign58260_e95099_d_n8, assign58260_e95099_d_n9, assign58260_e95099_d_n10, assign58260_e95099_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58260_e95081: f64 = (locals.var_nq * locals.var_inv_mdl);
        let assign58260_e95085: f64 = (0.3333333333333333 * locals.var_dqsd2);
        let assign58260_e95087: f64 = (assign58260_e95085 * locals.var_t5);
        let assign58260_e95088: f64 = (locals.var_t9 + assign58260_e95087);
        let assign58260_e95089: f64 = (assign58260_e95081 * assign58260_e95088);
        let assign58260_e95092: f64 = (2.0 * locals.var_nq);
        let assign58260_e95094: f64 = (assign58260_e95092 * locals.var_mdl_less_1);
        let assign58260_e95096: f64 = (assign58260_e95094 * locals.var_qdeff);
        let assign58260_e95097: f64 = (assign58260_e95089 + assign58260_e95096);
        (assign58260_e95097, (((((locals.var_nq_dn3 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn3)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn3 + (((0.3333333333333333 * locals.var_dqsd2_dn3) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn3))))) + (((((2.0 * locals.var_nq_dn3) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn3)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn3))), (((((locals.var_nq_dn4 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn4)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn4 + (((0.3333333333333333 * locals.var_dqsd2_dn4) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn4))))) + (((((2.0 * locals.var_nq_dn4) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn4)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn4))), (((((locals.var_nq_dn5 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn5)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn5 + (((0.3333333333333333 * locals.var_dqsd2_dn5) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn5))))) + (((((2.0 * locals.var_nq_dn5) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn5)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn5))), (((((locals.var_nq_dn6 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn6)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn6 + (((0.3333333333333333 * locals.var_dqsd2_dn6) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn6))))) + (((((2.0 * locals.var_nq_dn6) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn6)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn6))), (((((locals.var_nq_dn7 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn7)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn7 + (((0.3333333333333333 * locals.var_dqsd2_dn7) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn7))))) + (((((2.0 * locals.var_nq_dn7) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn7)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn7))), (((((locals.var_nq_dn8 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn8)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn8 + (((0.3333333333333333 * locals.var_dqsd2_dn8) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn8))))) + (((((2.0 * locals.var_nq_dn8) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn8)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn8))), (((((locals.var_nq_dn9 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn9)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn9 + (((0.3333333333333333 * locals.var_dqsd2_dn9) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn9))))) + (((((2.0 * locals.var_nq_dn9) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn9)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn9))), (((((locals.var_nq_dn10 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn10)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn10 + (((0.3333333333333333 * locals.var_dqsd2_dn10) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn10))))) + (((((2.0 * locals.var_nq_dn10) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn10)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn10))), (((((locals.var_nq_dn11 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn11)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn11 + (((0.3333333333333333 * locals.var_dqsd2_dn11) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn11))))) + (((((2.0 * locals.var_nq_dn11) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn11)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn11))),)
    } else {
        (locals.var_qi_1, locals.var_qi_1_dn3, locals.var_qi_1_dn4, locals.var_qi_1_dn5, locals.var_qi_1_dn6, locals.var_qi_1_dn7, locals.var_qi_1_dn8, locals.var_qi_1_dn9, locals.var_qi_1_dn10, locals.var_qi_1_dn11,)
    }
};
        locals.var_qi_1 = assign58260_e95099;
        locals.var_qi_1_dn3 = assign58260_e95099_d_n3;
        locals.var_qi_1_dn4 = assign58260_e95099_d_n4;
        locals.var_qi_1_dn5 = assign58260_e95099_d_n5;
        locals.var_qi_1_dn6 = assign58260_e95099_d_n6;
        locals.var_qi_1_dn7 = assign58260_e95099_d_n7;
        locals.var_qi_1_dn8 = assign58260_e95099_d_n8;
        locals.var_qi_1_dn9 = assign58260_e95099_d_n9;
        locals.var_qi_1_dn10 = assign58260_e95099_d_n10;
        locals.var_qi_1_dn11 = assign58260_e95099_d_n11;

        let (assign58270_e95126, assign58270_e95126_d_n3, assign58270_e95126_d_n4, assign58270_e95126_d_n5, assign58270_e95126_d_n6, assign58270_e95126_d_n7, assign58270_e95126_d_n8, assign58270_e95126_d_n9, assign58270_e95126_d_n10, assign58270_e95126_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58270_e95106: f64 = (locals.var_nq * locals.var_inv_mdl_2);
        let assign58270_e95109: f64 = (0.5 * locals.var_t9);
        let assign58270_e95112: f64 = (locals.var_dqsd / 6.0);
        let assign58270_e95116: f64 = (locals.var_dqsd * locals.var_t5);
        let assign58270_e95117: f64 = (1.0 - assign58270_e95116);
        let assign58270_e95120: f64 = (0.2 * locals.var_t10);
        let assign58270_e95121: f64 = (assign58270_e95117 - assign58270_e95120);
        let assign58270_e95122: f64 = (assign58270_e95112 * assign58270_e95121);
        let assign58270_e95123: f64 = (assign58270_e95109 - assign58270_e95122);
        let assign58270_e95124: f64 = (assign58270_e95106 * assign58270_e95123);
        (assign58270_e95124, ((((locals.var_nq_dn3 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn3)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn3) - (((locals.var_dqsd_dn3 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn3 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn3))) - (0.2 * locals.var_t10_dn3))))))), ((((locals.var_nq_dn4 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn4)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn4) - (((locals.var_dqsd_dn4 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn4 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn4))) - (0.2 * locals.var_t10_dn4))))))), ((((locals.var_nq_dn5 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn5)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn5) - (((locals.var_dqsd_dn5 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn5 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn5))) - (0.2 * locals.var_t10_dn5))))))), ((((locals.var_nq_dn6 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn6)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn6) - (((locals.var_dqsd_dn6 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn6 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn6))) - (0.2 * locals.var_t10_dn6))))))), ((((locals.var_nq_dn7 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn7)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn7) - (((locals.var_dqsd_dn7 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn7 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn7))) - (0.2 * locals.var_t10_dn7))))))), ((((locals.var_nq_dn8 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn8)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn8) - (((locals.var_dqsd_dn8 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn8 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn8))) - (0.2 * locals.var_t10_dn8))))))), ((((locals.var_nq_dn9 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn9)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn9) - (((locals.var_dqsd_dn9 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn9 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn9))) - (0.2 * locals.var_t10_dn9))))))), ((((locals.var_nq_dn10 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn10)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn10) - (((locals.var_dqsd_dn10 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn10 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn10))) - (0.2 * locals.var_t10_dn10))))))), ((((locals.var_nq_dn11 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn11)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn11) - (((locals.var_dqsd_dn11 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn11 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn11))) - (0.2 * locals.var_t10_dn11))))))),)
    } else {
        (locals.var_qd1, locals.var_qd1_dn3, locals.var_qd1_dn4, locals.var_qd1_dn5, locals.var_qd1_dn6, locals.var_qd1_dn7, locals.var_qd1_dn8, locals.var_qd1_dn9, locals.var_qd1_dn10, locals.var_qd1_dn11,)
    }
};
        locals.var_qd1 = assign58270_e95126;
        locals.var_qd1_dn3 = assign58270_e95126_d_n3;
        locals.var_qd1_dn4 = assign58270_e95126_d_n4;
        locals.var_qd1_dn5 = assign58270_e95126_d_n5;
        locals.var_qd1_dn6 = assign58270_e95126_d_n6;
        locals.var_qd1_dn7 = assign58270_e95126_d_n7;
        locals.var_qd1_dn8 = assign58270_e95126_d_n8;
        locals.var_qd1_dn9 = assign58270_e95126_d_n9;
        locals.var_qd1_dn10 = assign58270_e95126_d_n10;
        locals.var_qd1_dn11 = assign58270_e95126_d_n11;

        let (assign58280_e95139, assign58280_e95139_d_n3, assign58280_e95139_d_n4, assign58280_e95139_d_n5, assign58280_e95139_d_n6, assign58280_e95139_d_n7, assign58280_e95139_d_n8, assign58280_e95139_d_n9, assign58280_e95139_d_n10, assign58280_e95139_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58280_e95134: f64 = (locals.var_mdl - locals.var_inv_mdl);
        let assign58280_e95135: f64 = (locals.var_nq * assign58280_e95134);
        let assign58280_e95137: f64 = (assign58280_e95135 * locals.var_qdeff);
        (assign58280_e95137, ((((locals.var_nq_dn3 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn3 - locals.var_inv_mdl_dn3))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn3)), ((((locals.var_nq_dn4 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn4 - locals.var_inv_mdl_dn4))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn4)), ((((locals.var_nq_dn5 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn5 - locals.var_inv_mdl_dn5))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn5)), ((((locals.var_nq_dn6 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn6 - locals.var_inv_mdl_dn6))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn6)), ((((locals.var_nq_dn7 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn7 - locals.var_inv_mdl_dn7))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn7)), ((((locals.var_nq_dn8 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn8 - locals.var_inv_mdl_dn8))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn8)), ((((locals.var_nq_dn9 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn9 - locals.var_inv_mdl_dn9))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn9)), ((((locals.var_nq_dn10 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn10 - locals.var_inv_mdl_dn10))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn10)), ((((locals.var_nq_dn11 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn11 - locals.var_inv_mdl_dn11))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn11)),)
    } else {
        (locals.var_qd2, locals.var_qd2_dn3, locals.var_qd2_dn4, locals.var_qd2_dn5, locals.var_qd2_dn6, locals.var_qd2_dn7, locals.var_qd2_dn8, locals.var_qd2_dn9, locals.var_qd2_dn10, locals.var_qd2_dn11,)
    }
};
        locals.var_qd2 = assign58280_e95139;
        locals.var_qd2_dn3 = assign58280_e95139_d_n3;
        locals.var_qd2_dn4 = assign58280_e95139_d_n4;
        locals.var_qd2_dn5 = assign58280_e95139_d_n5;
        locals.var_qd2_dn6 = assign58280_e95139_d_n6;
        locals.var_qd2_dn7 = assign58280_e95139_d_n7;
        locals.var_qd2_dn8 = assign58280_e95139_d_n8;
        locals.var_qd2_dn9 = assign58280_e95139_d_n9;
        locals.var_qd2_dn10 = assign58280_e95139_d_n10;
        locals.var_qd2_dn11 = assign58280_e95139_d_n11;

        let (assign58290_e95148, assign58290_e95148_d_n3, assign58290_e95148_d_n4, assign58290_e95148_d_n5, assign58290_e95148_d_n6, assign58290_e95148_d_n7, assign58290_e95148_d_n8, assign58290_e95148_d_n9, assign58290_e95148_d_n10, assign58290_e95148_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58290_e95146: f64 = (locals.var_qd1 + locals.var_qd2);
        (assign58290_e95146, (locals.var_qd1_dn3 + locals.var_qd2_dn3), (locals.var_qd1_dn4 + locals.var_qd2_dn4), (locals.var_qd1_dn5 + locals.var_qd2_dn5), (locals.var_qd1_dn6 + locals.var_qd2_dn6), (locals.var_qd1_dn7 + locals.var_qd2_dn7), (locals.var_qd1_dn8 + locals.var_qd2_dn8), (locals.var_qd1_dn9 + locals.var_qd2_dn9), (locals.var_qd1_dn10 + locals.var_qd2_dn10), (locals.var_qd1_dn11 + locals.var_qd2_dn11),)
    } else {
        (locals.var_qd, locals.var_qd_dn3, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11,)
    }
};
        locals.var_qd = assign58290_e95148;
        locals.var_qd_dn3 = assign58290_e95148_d_n3;
        locals.var_qd_dn4 = assign58290_e95148_d_n4;
        locals.var_qd_dn5 = assign58290_e95148_d_n5;
        locals.var_qd_dn6 = assign58290_e95148_d_n6;
        locals.var_qd_dn7 = assign58290_e95148_d_n7;
        locals.var_qd_dn8 = assign58290_e95148_d_n8;
        locals.var_qd_dn9 = assign58290_e95148_d_n9;
        locals.var_qd_dn10 = assign58290_e95148_d_n10;
        locals.var_qd_dn11 = assign58290_e95148_d_n11;

        let (assign58300_e95163, assign58300_e95163_d_n4, assign58300_e95163_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58300_e95156: f64 = (8.8541878128e-12 * p.p110);
        let assign58300_e95158: f64 = (assign58300_e95156 / locals.var_bsimbulktoxp);
        let assign58300_e95159: f64 = (p.p1380 * assign58300_e95158);
        let assign58300_e95161: f64 = (assign58300_e95159 * locals.var_vt);
        (assign58300_e95161, (assign58300_e95159 * locals.var_vt_dn4), (assign58300_e95159 * locals.var_vt_dn5),)
    } else {
        (locals.var_wlcox, locals.var_wlcox_dn4, locals.var_wlcox_dn5,)
    }
};
        locals.var_wlcox = assign58300_e95163;
        locals.var_wlcox_dn4 = assign58300_e95163_d_n4;
        locals.var_wlcox_dn5 = assign58300_e95163_d_n5;

        let (assign58310_e95172, assign58310_e95172_d_n3, assign58310_e95172_d_n4, assign58310_e95172_d_n5, assign58310_e95172_d_n6, assign58310_e95172_d_n7, assign58310_e95172_d_n8, assign58310_e95172_d_n9, assign58310_e95172_d_n10, assign58310_e95172_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58310_e95170: f64 = (locals.var_wlcox * locals.var_qb_1);
        (assign58310_e95170, (locals.var_wlcox * locals.var_qb_1_dn3), ((locals.var_wlcox_dn4 * locals.var_qb_1) + (locals.var_wlcox * locals.var_qb_1_dn4)), ((locals.var_wlcox_dn5 * locals.var_qb_1) + (locals.var_wlcox * locals.var_qb_1_dn5)), (locals.var_wlcox * locals.var_qb_1_dn6), (locals.var_wlcox * locals.var_qb_1_dn7), (locals.var_wlcox * locals.var_qb_1_dn8), (locals.var_wlcox * locals.var_qb_1_dn9), (locals.var_wlcox * locals.var_qb_1_dn10), (locals.var_wlcox * locals.var_qb_1_dn11),)
    } else {
        (locals.var_qbi_agbcp2, locals.var_qbi_agbcp2_dn3, locals.var_qbi_agbcp2_dn4, locals.var_qbi_agbcp2_dn5, locals.var_qbi_agbcp2_dn6, locals.var_qbi_agbcp2_dn7, locals.var_qbi_agbcp2_dn8, locals.var_qbi_agbcp2_dn9, locals.var_qbi_agbcp2_dn10, locals.var_qbi_agbcp2_dn11,)
    }
};
        locals.var_qbi_agbcp2 = assign58310_e95172;
        locals.var_qbi_agbcp2_dn3 = assign58310_e95172_d_n3;
        locals.var_qbi_agbcp2_dn4 = assign58310_e95172_d_n4;
        locals.var_qbi_agbcp2_dn5 = assign58310_e95172_d_n5;
        locals.var_qbi_agbcp2_dn6 = assign58310_e95172_d_n6;
        locals.var_qbi_agbcp2_dn7 = assign58310_e95172_d_n7;
        locals.var_qbi_agbcp2_dn8 = assign58310_e95172_d_n8;
        locals.var_qbi_agbcp2_dn9 = assign58310_e95172_d_n9;
        locals.var_qbi_agbcp2_dn10 = assign58310_e95172_d_n10;
        locals.var_qbi_agbcp2_dn11 = assign58310_e95172_d_n11;

        let (assign58320_e95181, assign58320_e95181_d_n3, assign58320_e95181_d_n4, assign58320_e95181_d_n5, assign58320_e95181_d_n6, assign58320_e95181_d_n7, assign58320_e95181_d_n8, assign58320_e95181_d_n9, assign58320_e95181_d_n10, assign58320_e95181_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58320_e95179: f64 = (locals.var_wlcox * locals.var_qd);
        (assign58320_e95179, (locals.var_wlcox * locals.var_qd_dn3), ((locals.var_wlcox_dn4 * locals.var_qd) + (locals.var_wlcox * locals.var_qd_dn4)), ((locals.var_wlcox_dn5 * locals.var_qd) + (locals.var_wlcox * locals.var_qd_dn5)), (locals.var_wlcox * locals.var_qd_dn6), (locals.var_wlcox * locals.var_qd_dn7), (locals.var_wlcox * locals.var_qd_dn8), (locals.var_wlcox * locals.var_qd_dn9), (locals.var_wlcox * locals.var_qd_dn10), (locals.var_wlcox * locals.var_qd_dn11),)
    } else {
        (locals.var_qdi_agbcp2, locals.var_qdi_agbcp2_dn3, locals.var_qdi_agbcp2_dn4, locals.var_qdi_agbcp2_dn5, locals.var_qdi_agbcp2_dn6, locals.var_qdi_agbcp2_dn7, locals.var_qdi_agbcp2_dn8, locals.var_qdi_agbcp2_dn9, locals.var_qdi_agbcp2_dn10, locals.var_qdi_agbcp2_dn11,)
    }
};
        locals.var_qdi_agbcp2 = assign58320_e95181;
        locals.var_qdi_agbcp2_dn3 = assign58320_e95181_d_n3;
        locals.var_qdi_agbcp2_dn4 = assign58320_e95181_d_n4;
        locals.var_qdi_agbcp2_dn5 = assign58320_e95181_d_n5;
        locals.var_qdi_agbcp2_dn6 = assign58320_e95181_d_n6;
        locals.var_qdi_agbcp2_dn7 = assign58320_e95181_d_n7;
        locals.var_qdi_agbcp2_dn8 = assign58320_e95181_d_n8;
        locals.var_qdi_agbcp2_dn9 = assign58320_e95181_d_n9;
        locals.var_qdi_agbcp2_dn10 = assign58320_e95181_d_n10;
        locals.var_qdi_agbcp2_dn11 = assign58320_e95181_d_n11;

        let (assign58330_e95190, assign58330_e95190_d_n3, assign58330_e95190_d_n4, assign58330_e95190_d_n5, assign58330_e95190_d_n6, assign58330_e95190_d_n7, assign58330_e95190_d_n8, assign58330_e95190_d_n9, assign58330_e95190_d_n10, assign58330_e95190_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58330_e95188: f64 = (locals.var_wlcox * locals.var_qi_1);
        (assign58330_e95188, (locals.var_wlcox * locals.var_qi_1_dn3), ((locals.var_wlcox_dn4 * locals.var_qi_1) + (locals.var_wlcox * locals.var_qi_1_dn4)), ((locals.var_wlcox_dn5 * locals.var_qi_1) + (locals.var_wlcox * locals.var_qi_1_dn5)), (locals.var_wlcox * locals.var_qi_1_dn6), (locals.var_wlcox * locals.var_qi_1_dn7), (locals.var_wlcox * locals.var_qi_1_dn8), (locals.var_wlcox * locals.var_qi_1_dn9), (locals.var_wlcox * locals.var_qi_1_dn10), (locals.var_wlcox * locals.var_qi_1_dn11),)
    } else {
        (locals.var_qi_agbcp2, locals.var_qi_agbcp2_dn3, locals.var_qi_agbcp2_dn4, locals.var_qi_agbcp2_dn5, locals.var_qi_agbcp2_dn6, locals.var_qi_agbcp2_dn7, locals.var_qi_agbcp2_dn8, locals.var_qi_agbcp2_dn9, locals.var_qi_agbcp2_dn10, locals.var_qi_agbcp2_dn11,)
    }
};
        locals.var_qi_agbcp2 = assign58330_e95190;
        locals.var_qi_agbcp2_dn3 = assign58330_e95190_d_n3;
        locals.var_qi_agbcp2_dn4 = assign58330_e95190_d_n4;
        locals.var_qi_agbcp2_dn5 = assign58330_e95190_d_n5;
        locals.var_qi_agbcp2_dn6 = assign58330_e95190_d_n6;
        locals.var_qi_agbcp2_dn7 = assign58330_e95190_d_n7;
        locals.var_qi_agbcp2_dn8 = assign58330_e95190_d_n8;
        locals.var_qi_agbcp2_dn9 = assign58330_e95190_d_n9;
        locals.var_qi_agbcp2_dn10 = assign58330_e95190_d_n10;
        locals.var_qi_agbcp2_dn11 = assign58330_e95190_d_n11;

        let (assign58340_e95198, assign58340_e95198_d_n3, assign58340_e95198_d_n4, assign58340_e95198_d_n5, assign58340_e95198_d_n6, assign58340_e95198_d_n7, assign58340_e95198_d_n8, assign58340_e95198_d_n9, assign58340_e95198_d_n10, assign58340_e95198_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbi_agbcp2, locals.var_qbi_agbcp2_dn3, locals.var_qbi_agbcp2_dn4, locals.var_qbi_agbcp2_dn5, locals.var_qbi_agbcp2_dn6, locals.var_qbi_agbcp2_dn7, locals.var_qbi_agbcp2_dn8, locals.var_qbi_agbcp2_dn9, locals.var_qbi_agbcp2_dn10, locals.var_qbi_agbcp2_dn11,)
    }
};
        locals.var_qbi_agbcp2 = assign58340_e95198;
        locals.var_qbi_agbcp2_dn3 = assign58340_e95198_d_n3;
        locals.var_qbi_agbcp2_dn4 = assign58340_e95198_d_n4;
        locals.var_qbi_agbcp2_dn5 = assign58340_e95198_d_n5;
        locals.var_qbi_agbcp2_dn6 = assign58340_e95198_d_n6;
        locals.var_qbi_agbcp2_dn7 = assign58340_e95198_d_n7;
        locals.var_qbi_agbcp2_dn8 = assign58340_e95198_d_n8;
        locals.var_qbi_agbcp2_dn9 = assign58340_e95198_d_n9;
        locals.var_qbi_agbcp2_dn10 = assign58340_e95198_d_n10;
        locals.var_qbi_agbcp2_dn11 = assign58340_e95198_d_n11;

        let (assign58350_e95206, assign58350_e95206_d_n3, assign58350_e95206_d_n4, assign58350_e95206_d_n5, assign58350_e95206_d_n6, assign58350_e95206_d_n7, assign58350_e95206_d_n8, assign58350_e95206_d_n9, assign58350_e95206_d_n10, assign58350_e95206_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdi_agbcp2, locals.var_qdi_agbcp2_dn3, locals.var_qdi_agbcp2_dn4, locals.var_qdi_agbcp2_dn5, locals.var_qdi_agbcp2_dn6, locals.var_qdi_agbcp2_dn7, locals.var_qdi_agbcp2_dn8, locals.var_qdi_agbcp2_dn9, locals.var_qdi_agbcp2_dn10, locals.var_qdi_agbcp2_dn11,)
    }
};
        locals.var_qdi_agbcp2 = assign58350_e95206;
        locals.var_qdi_agbcp2_dn3 = assign58350_e95206_d_n3;
        locals.var_qdi_agbcp2_dn4 = assign58350_e95206_d_n4;
        locals.var_qdi_agbcp2_dn5 = assign58350_e95206_d_n5;
        locals.var_qdi_agbcp2_dn6 = assign58350_e95206_d_n6;
        locals.var_qdi_agbcp2_dn7 = assign58350_e95206_d_n7;
        locals.var_qdi_agbcp2_dn8 = assign58350_e95206_d_n8;
        locals.var_qdi_agbcp2_dn9 = assign58350_e95206_d_n9;
        locals.var_qdi_agbcp2_dn10 = assign58350_e95206_d_n10;
        locals.var_qdi_agbcp2_dn11 = assign58350_e95206_d_n11;

        let (assign58360_e95214, assign58360_e95214_d_n3, assign58360_e95214_d_n4, assign58360_e95214_d_n5, assign58360_e95214_d_n6, assign58360_e95214_d_n7, assign58360_e95214_d_n8, assign58360_e95214_d_n9, assign58360_e95214_d_n10, assign58360_e95214_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi_agbcp2, locals.var_qi_agbcp2_dn3, locals.var_qi_agbcp2_dn4, locals.var_qi_agbcp2_dn5, locals.var_qi_agbcp2_dn6, locals.var_qi_agbcp2_dn7, locals.var_qi_agbcp2_dn8, locals.var_qi_agbcp2_dn9, locals.var_qi_agbcp2_dn10, locals.var_qi_agbcp2_dn11,)
    }
};
        locals.var_qi_agbcp2 = assign58360_e95214;
        locals.var_qi_agbcp2_dn3 = assign58360_e95214_d_n3;
        locals.var_qi_agbcp2_dn4 = assign58360_e95214_d_n4;
        locals.var_qi_agbcp2_dn5 = assign58360_e95214_d_n5;
        locals.var_qi_agbcp2_dn6 = assign58360_e95214_d_n6;
        locals.var_qi_agbcp2_dn7 = assign58360_e95214_d_n7;
        locals.var_qi_agbcp2_dn8 = assign58360_e95214_d_n8;
        locals.var_qi_agbcp2_dn9 = assign58360_e95214_d_n9;
        locals.var_qi_agbcp2_dn10 = assign58360_e95214_d_n10;
        locals.var_qi_agbcp2_dn11 = assign58360_e95214_d_n11;

        let (assign58450_e95288, assign58450_e95288_d_n3, assign58450_e95288_d_n4, assign58450_e95288_d_n5, assign58450_e95288_d_n6, assign58450_e95288_d_n7, assign58450_e95288_d_n8, assign58450_e95288_d_n9, assign58450_e95288_d_n10, assign58450_e95288_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58450_e95281: f64 = (-locals.var_qdi);
        let assign58450_e95284: f64 = (p.p45 * locals.var_qdi_agbcp2);
        let assign58450_e95285: f64 = (assign58450_e95281 + assign58450_e95284);
        let assign58450_e95286: f64 = (-assign58450_e95285);
        (assign58450_e95286, (-((-locals.var_qdi_dn3) + (p.p45 * locals.var_qdi_agbcp2_dn3))), (-((-locals.var_qdi_dn4) + (p.p45 * locals.var_qdi_agbcp2_dn4))), (-((-locals.var_qdi_dn5) + (p.p45 * locals.var_qdi_agbcp2_dn5))), (-((-locals.var_qdi_dn6) + (p.p45 * locals.var_qdi_agbcp2_dn6))), (-((-locals.var_qdi_dn7) + (p.p45 * locals.var_qdi_agbcp2_dn7))), (-((-locals.var_qdi_dn8) + (p.p45 * locals.var_qdi_agbcp2_dn8))), (-((-locals.var_qdi_dn9) + (p.p45 * locals.var_qdi_agbcp2_dn9))), (-((-locals.var_qdi_dn10) + (p.p45 * locals.var_qdi_agbcp2_dn10))), (-((-locals.var_qdi_dn11) + (p.p45 * locals.var_qdi_agbcp2_dn11))),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8, locals.var_qdi_dn9, locals.var_qdi_dn10, locals.var_qdi_dn11,)
    }
};
        locals.var_qdi = assign58450_e95288;
        locals.var_qdi_dn3 = assign58450_e95288_d_n3;
        locals.var_qdi_dn4 = assign58450_e95288_d_n4;
        locals.var_qdi_dn5 = assign58450_e95288_d_n5;
        locals.var_qdi_dn6 = assign58450_e95288_d_n6;
        locals.var_qdi_dn7 = assign58450_e95288_d_n7;
        locals.var_qdi_dn8 = assign58450_e95288_d_n8;
        locals.var_qdi_dn9 = assign58450_e95288_d_n9;
        locals.var_qdi_dn10 = assign58450_e95288_d_n10;
        locals.var_qdi_dn11 = assign58450_e95288_d_n11;

        let (assign58460_e95303, assign58460_e95303_d_n3, assign58460_e95303_d_n4, assign58460_e95303_d_n5, assign58460_e95303_d_n6, assign58460_e95303_d_n7, assign58460_e95303_d_n8, assign58460_e95303_d_n9, assign58460_e95303_d_n10, assign58460_e95303_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58460_e95292: f64 = (-locals.var_qsi);
        let assign58460_e95295: f64 = (p.p45 * locals.var_qi_agbcp2);
        let assign58460_e95296: f64 = (assign58460_e95292 + assign58460_e95295);
        let assign58460_e95299: f64 = (p.p45 * locals.var_qdi_agbcp2);
        let assign58460_e95300: f64 = (assign58460_e95296 - assign58460_e95299);
        let assign58460_e95301: f64 = (-assign58460_e95300);
        (assign58460_e95301, (-(((-locals.var_qsi_dn3) + (p.p45 * locals.var_qi_agbcp2_dn3)) - (p.p45 * locals.var_qdi_agbcp2_dn3))), (-(((-locals.var_qsi_dn4) + (p.p45 * locals.var_qi_agbcp2_dn4)) - (p.p45 * locals.var_qdi_agbcp2_dn4))), (-(((-locals.var_qsi_dn5) + (p.p45 * locals.var_qi_agbcp2_dn5)) - (p.p45 * locals.var_qdi_agbcp2_dn5))), (-(((-locals.var_qsi_dn6) + (p.p45 * locals.var_qi_agbcp2_dn6)) - (p.p45 * locals.var_qdi_agbcp2_dn6))), (-(((-locals.var_qsi_dn7) + (p.p45 * locals.var_qi_agbcp2_dn7)) - (p.p45 * locals.var_qdi_agbcp2_dn7))), (-(((-locals.var_qsi_dn8) + (p.p45 * locals.var_qi_agbcp2_dn8)) - (p.p45 * locals.var_qdi_agbcp2_dn8))), (-(((-locals.var_qsi_dn9) + (p.p45 * locals.var_qi_agbcp2_dn9)) - (p.p45 * locals.var_qdi_agbcp2_dn9))), (-(((-locals.var_qsi_dn10) + (p.p45 * locals.var_qi_agbcp2_dn10)) - (p.p45 * locals.var_qdi_agbcp2_dn10))), (-(((-locals.var_qsi_dn11) + (p.p45 * locals.var_qi_agbcp2_dn11)) - (p.p45 * locals.var_qdi_agbcp2_dn11))),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8, locals.var_qsi_dn9, locals.var_qsi_dn10, locals.var_qsi_dn11,)
    }
};
        locals.var_qsi = assign58460_e95303;
        locals.var_qsi_dn3 = assign58460_e95303_d_n3;
        locals.var_qsi_dn4 = assign58460_e95303_d_n4;
        locals.var_qsi_dn5 = assign58460_e95303_d_n5;
        locals.var_qsi_dn6 = assign58460_e95303_d_n6;
        locals.var_qsi_dn7 = assign58460_e95303_d_n7;
        locals.var_qsi_dn8 = assign58460_e95303_d_n8;
        locals.var_qsi_dn9 = assign58460_e95303_d_n9;
        locals.var_qsi_dn10 = assign58460_e95303_d_n10;
        locals.var_qsi_dn11 = assign58460_e95303_d_n11;

        let (assign58470_e95314, assign58470_e95314_d_n3, assign58470_e95314_d_n4, assign58470_e95314_d_n5, assign58470_e95314_d_n6, assign58470_e95314_d_n7, assign58470_e95314_d_n8, assign58470_e95314_d_n9, assign58470_e95314_d_n10, assign58470_e95314_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58470_e95307: f64 = (-locals.var_qbi);
        let assign58470_e95310: f64 = (p.p45 * locals.var_qbi_agbcp2);
        let assign58470_e95311: f64 = (assign58470_e95307 + assign58470_e95310);
        let assign58470_e95312: f64 = (-assign58470_e95311);
        (assign58470_e95312, (-((-locals.var_qbi_dn3) + (p.p45 * locals.var_qbi_agbcp2_dn3))), (-((-locals.var_qbi_dn4) + (p.p45 * locals.var_qbi_agbcp2_dn4))), (-((-locals.var_qbi_dn5) + (p.p45 * locals.var_qbi_agbcp2_dn5))), (-((-locals.var_qbi_dn6) + (p.p45 * locals.var_qbi_agbcp2_dn6))), (-((-locals.var_qbi_dn7) + (p.p45 * locals.var_qbi_agbcp2_dn7))), (-((-locals.var_qbi_dn8) + (p.p45 * locals.var_qbi_agbcp2_dn8))), (-((-locals.var_qbi_dn9) + (p.p45 * locals.var_qbi_agbcp2_dn9))), (-((-locals.var_qbi_dn10) + (p.p45 * locals.var_qbi_agbcp2_dn10))), (-((-locals.var_qbi_dn11) + (p.p45 * locals.var_qbi_agbcp2_dn11))),)
    } else {
        (locals.var_qbi, locals.var_qbi_dn3, locals.var_qbi_dn4, locals.var_qbi_dn5, locals.var_qbi_dn6, locals.var_qbi_dn7, locals.var_qbi_dn8, locals.var_qbi_dn9, locals.var_qbi_dn10, locals.var_qbi_dn11,)
    }
};
        locals.var_qbi = assign58470_e95314;
        locals.var_qbi_dn3 = assign58470_e95314_d_n3;
        locals.var_qbi_dn4 = assign58470_e95314_d_n4;
        locals.var_qbi_dn5 = assign58470_e95314_d_n5;
        locals.var_qbi_dn6 = assign58470_e95314_d_n6;
        locals.var_qbi_dn7 = assign58470_e95314_d_n7;
        locals.var_qbi_dn8 = assign58470_e95314_d_n8;
        locals.var_qbi_dn9 = assign58470_e95314_d_n9;
        locals.var_qbi_dn10 = assign58470_e95314_d_n10;
        locals.var_qbi_dn11 = assign58470_e95314_d_n11;

        let (assign58480_e95324, assign58480_e95324_d_n3, assign58480_e95324_d_n4, assign58480_e95324_d_n5, assign58480_e95324_d_n6, assign58480_e95324_d_n7, assign58480_e95324_d_n8, assign58480_e95324_d_n9, assign58480_e95324_d_n10, assign58480_e95324_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58480_e95319: f64 = (locals.var_qbi + locals.var_qsi);
        let assign58480_e95321: f64 = (assign58480_e95319 + locals.var_qdi);
        let assign58480_e95322: f64 = (-assign58480_e95321);
        (assign58480_e95322, (-((locals.var_qbi_dn3 + locals.var_qsi_dn3) + locals.var_qdi_dn3)), (-((locals.var_qbi_dn4 + locals.var_qsi_dn4) + locals.var_qdi_dn4)), (-((locals.var_qbi_dn5 + locals.var_qsi_dn5) + locals.var_qdi_dn5)), (-((locals.var_qbi_dn6 + locals.var_qsi_dn6) + locals.var_qdi_dn6)), (-((locals.var_qbi_dn7 + locals.var_qsi_dn7) + locals.var_qdi_dn7)), (-((locals.var_qbi_dn8 + locals.var_qsi_dn8) + locals.var_qdi_dn8)), (-((locals.var_qbi_dn9 + locals.var_qsi_dn9) + locals.var_qdi_dn9)), (-((locals.var_qbi_dn10 + locals.var_qsi_dn10) + locals.var_qdi_dn10)), (-((locals.var_qbi_dn11 + locals.var_qsi_dn11) + locals.var_qdi_dn11)),)
    } else {
        (locals.var_qgi, locals.var_qgi_dn3, locals.var_qgi_dn4, locals.var_qgi_dn5, locals.var_qgi_dn6, locals.var_qgi_dn7, locals.var_qgi_dn8, locals.var_qgi_dn9, locals.var_qgi_dn10, locals.var_qgi_dn11,)
    }
};
        locals.var_qgi = assign58480_e95324;
        locals.var_qgi_dn3 = assign58480_e95324_d_n3;
        locals.var_qgi_dn4 = assign58480_e95324_d_n4;
        locals.var_qgi_dn5 = assign58480_e95324_d_n5;
        locals.var_qgi_dn6 = assign58480_e95324_d_n6;
        locals.var_qgi_dn7 = assign58480_e95324_d_n7;
        locals.var_qgi_dn8 = assign58480_e95324_d_n8;
        locals.var_qgi_dn9 = assign58480_e95324_d_n9;
        locals.var_qgi_dn10 = assign58480_e95324_d_n10;
        locals.var_qgi_dn11 = assign58480_e95324_d_n11;

        let assign58490_e95327: f64 = if (!param_given[867]) { 1.0 } else { 0.0 };
        locals.var_guard861 = assign58490_e95327;

        let (assign58500_e95351,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard861 != 0.0)) {
        let assign58500_e95334: f64 = (2.0 * p.p110);
        let assign58500_e95336: f64 = (assign58500_e95334 * 8.8541878128e-12);
        let assign58500_e95338: f64 = (assign58500_e95336 / 3.141592653589793);
        let assign58500_e95343: f64 = (4e-7 / p.p76);
        let assign58500_e95344: f64 = (1.0 + assign58500_e95343);
        let assign58500_e95345: f64 = (p.p871 * assign58500_e95344);
        let assign58500_e95347: f64 = (assign58500_e95345).max(1e-38);
        let assign58500_e95348: f64 = (assign58500_e95347).ln();
        let assign58500_e95349: f64 = (assign58500_e95338 * assign58500_e95348);
        (assign58500_e95349,)
    } else {
        (locals.var_cf_i,)
    }
};
        locals.var_cf_i = assign58500_e95351;

        let (assign58510_e95358,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58510_e95356: f64 = (p.p872 + locals.var_cf_i);
        (assign58510_e95356,)
    } else {
        (locals.var_cgsof,)
    }
};
        locals.var_cgsof = assign58510_e95358;

        let (assign58520_e95365,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58520_e95363: f64 = (p.p873 + locals.var_cf_i);
        (assign58520_e95363,)
    } else {
        (locals.var_cgdof,)
    }
};
        locals.var_cgdof = assign58520_e95365;

        let assign58530_e95368: f64 = if p.p32 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard862 = assign58530_e95368;

    }

    pub(super) fn stamp_transient_block_193(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (assign58540_e95382, assign58540_e95382_d_n3, assign58540_e95382_d_n4, assign58540_e95382_d_n5, assign58540_e95382_d_n6, assign58540_e95382_d_n7, assign58540_e95382_d_n8, assign58540_e95382_d_n9, assign58540_e95382_d_n10, assign58540_e95382_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 != 0.0)) {
        let assign58540_e95374: f64 = (-locals.var_wact);
        let assign58540_e95376: f64 = (assign58540_e95374 * p.p2);
        let assign58540_e95378: f64 = (assign58540_e95376 * locals.var_cgsof);
        let assign58540_e95380: f64 = (assign58540_e95378 * locals.var_vgs_ov_noswap);
        (assign58540_e95380, 0.0, 0.0, 0.0, 0.0, (assign58540_e95378 * locals.var_vgs_ov_noswap_dn7), 0.0, (assign58540_e95378 * locals.var_vgs_ov_noswap_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qovs, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11,)
    }
};
        locals.var_qovs = assign58540_e95382;
        locals.var_qovs_dn3 = assign58540_e95382_d_n3;
        locals.var_qovs_dn4 = assign58540_e95382_d_n4;
        locals.var_qovs_dn5 = assign58540_e95382_d_n5;
        locals.var_qovs_dn6 = assign58540_e95382_d_n6;
        locals.var_qovs_dn7 = assign58540_e95382_d_n7;
        locals.var_qovs_dn8 = assign58540_e95382_d_n8;
        locals.var_qovs_dn9 = assign58540_e95382_d_n9;
        locals.var_qovs_dn10 = assign58540_e95382_d_n10;
        locals.var_qovs_dn11 = assign58540_e95382_d_n11;

        let (assign58550_e95396, assign58550_e95396_d_n3, assign58550_e95396_d_n4, assign58550_e95396_d_n5, assign58550_e95396_d_n6, assign58550_e95396_d_n7, assign58550_e95396_d_n8, assign58550_e95396_d_n9, assign58550_e95396_d_n10, assign58550_e95396_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 != 0.0)) {
        let assign58550_e95388: f64 = (-locals.var_wact);
        let assign58550_e95390: f64 = (assign58550_e95388 * p.p2);
        let assign58550_e95392: f64 = (assign58550_e95390 * locals.var_cgdof);
        let assign58550_e95394: f64 = (assign58550_e95392 * locals.var_vgd_ov_noswap);
        (assign58550_e95394, 0.0, 0.0, 0.0, (assign58550_e95392 * locals.var_vgd_ov_noswap_dn6), 0.0, 0.0, (assign58550_e95392 * locals.var_vgd_ov_noswap_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qovd, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11,)
    }
};
        locals.var_qovd = assign58550_e95396;
        locals.var_qovd_dn3 = assign58550_e95396_d_n3;
        locals.var_qovd_dn4 = assign58550_e95396_d_n4;
        locals.var_qovd_dn5 = assign58550_e95396_d_n5;
        locals.var_qovd_dn6 = assign58550_e95396_d_n6;
        locals.var_qovd_dn7 = assign58550_e95396_d_n7;
        locals.var_qovd_dn8 = assign58550_e95396_d_n8;
        locals.var_qovd_dn9 = assign58550_e95396_d_n9;
        locals.var_qovd_dn10 = assign58550_e95396_d_n10;
        locals.var_qovd_dn11 = assign58550_e95396_d_n11;

        let (assign58560_e95419, assign58560_e95419_d_n3, assign58560_e95419_d_n4, assign58560_e95419_d_n5, assign58560_e95419_d_n6, assign58560_e95419_d_n7, assign58560_e95419_d_n8, assign58560_e95419_d_n9, assign58560_e95419_d_n10, assign58560_e95419_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58560_e95404: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58560_e95406: f64 = (assign58560_e95404 + 0.02);
        let assign58560_e95409: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58560_e95411: f64 = (assign58560_e95409 + 0.02);
        let assign58560_e95412: f64 = (assign58560_e95406 * assign58560_e95411);
        let assign58560_e95415: f64 = (4.0 * 0.02);
        let assign58560_e95416: f64 = (assign58560_e95412 + assign58560_e95415);
        let assign58560_e95417: f64 = (assign58560_e95416).sqrt();
        (assign58560_e95417, 0.0, ((((-locals.var_vfbsdr_dn4) * assign58560_e95411) + (assign58560_e95406 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign58560_e95417)), ((((-locals.var_vfbsdr_dn5) * assign58560_e95411) + (assign58560_e95406 * (-locals.var_vfbsdr_dn5))) / (2.0 * assign58560_e95417)), 0.0, (((locals.var_vgs_ov_noswap_dn7 * assign58560_e95411) + (assign58560_e95406 * locals.var_vgs_ov_noswap_dn7)) / (2.0 * assign58560_e95417)), 0.0, (((locals.var_vgs_ov_noswap_dn9 * assign58560_e95411) + (assign58560_e95406 * locals.var_vgs_ov_noswap_dn9)) / (2.0 * assign58560_e95417)), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58560_e95419;
        locals.var_t0_dn3 = assign58560_e95419_d_n3;
        locals.var_t0_dn4 = assign58560_e95419_d_n4;
        locals.var_t0_dn5 = assign58560_e95419_d_n5;
        locals.var_t0_dn6 = assign58560_e95419_d_n6;
        locals.var_t0_dn7 = assign58560_e95419_d_n7;
        locals.var_t0_dn8 = assign58560_e95419_d_n8;
        locals.var_t0_dn9 = assign58560_e95419_d_n9;
        locals.var_t0_dn10 = assign58560_e95419_d_n10;
        locals.var_t0_dn11 = assign58560_e95419_d_n11;

        let (assign58570_e95435, assign58570_e95435_d_n3, assign58570_e95435_d_n4, assign58570_e95435_d_n5, assign58570_e95435_d_n6, assign58570_e95435_d_n7, assign58570_e95435_d_n8, assign58570_e95435_d_n9, assign58570_e95435_d_n10, assign58570_e95435_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58570_e95428: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58570_e95430: f64 = (assign58570_e95428 + 0.02);
        let assign58570_e95432: f64 = (assign58570_e95430 - locals.var_t0);
        let assign58570_e95433: f64 = (0.5 * assign58570_e95432);
        (assign58570_e95433, (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * ((-locals.var_vfbsdr_dn5) - locals.var_t0_dn5)), (0.5 * (-locals.var_t0_dn6)), (0.5 * (locals.var_vgs_ov_noswap_dn7 - locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (locals.var_vgs_ov_noswap_dn9 - locals.var_t0_dn9)), (0.5 * (-locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)),)
    } else {
        (locals.var_vgsov, locals.var_vgsov_dn3, locals.var_vgsov_dn4, locals.var_vgsov_dn5, locals.var_vgsov_dn6, locals.var_vgsov_dn7, locals.var_vgsov_dn8, locals.var_vgsov_dn9, locals.var_vgsov_dn10, locals.var_vgsov_dn11,)
    }
};
        locals.var_vgsov = assign58570_e95435;
        locals.var_vgsov_dn3 = assign58570_e95435_d_n3;
        locals.var_vgsov_dn4 = assign58570_e95435_d_n4;
        locals.var_vgsov_dn5 = assign58570_e95435_d_n5;
        locals.var_vgsov_dn6 = assign58570_e95435_d_n6;
        locals.var_vgsov_dn7 = assign58570_e95435_d_n7;
        locals.var_vgsov_dn8 = assign58570_e95435_d_n8;
        locals.var_vgsov_dn9 = assign58570_e95435_d_n9;
        locals.var_vgsov_dn10 = assign58570_e95435_d_n10;
        locals.var_vgsov_dn11 = assign58570_e95435_d_n11;

        let (assign58580_e95456, assign58580_e95456_d_n3, assign58580_e95456_d_n4, assign58580_e95456_d_n5, assign58580_e95456_d_n6, assign58580_e95456_d_n7, assign58580_e95456_d_n8, assign58580_e95456_d_n9, assign58580_e95456_d_n10, assign58580_e95456_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58580_e95444: f64 = (-locals.var_vgsov);
        let assign58580_e95446: f64 = (assign58580_e95444 / p.p893);
        let assign58580_e95448: f64 = (assign58580_e95446).powf(p.p894);
        let assign58580_e95449: f64 = (1.0 + assign58580_e95448);
        let assign58580_e95452: f64 = (1.0 / p.p894);
        let assign58580_e95453: f64 = (assign58580_e95449).powf(assign58580_e95452);
        let assign58580_e95454: f64 = (locals.var_vgsov / assign58580_e95453);
        (assign58580_e95454, (((locals.var_vgsov_dn3 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn3) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn3) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn3) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn3) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn4 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn4) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn4) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn4) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn4) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn5 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn5) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn5) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn5) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn5) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn6 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn6) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn6) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn6) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn6) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn7 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn7) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn7) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn7) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn7) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn8 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn8) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn8) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn8) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn8) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn9 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn9) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn9) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn9) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn9) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn10 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn10) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn10) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn10) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn10) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn11 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn11) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn11) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn11) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn11) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign58580_e95456;
        locals.var_t6_dn3 = assign58580_e95456_d_n3;
        locals.var_t6_dn4 = assign58580_e95456_d_n4;
        locals.var_t6_dn5 = assign58580_e95456_d_n5;
        locals.var_t6_dn6 = assign58580_e95456_d_n6;
        locals.var_t6_dn7 = assign58580_e95456_d_n7;
        locals.var_t6_dn8 = assign58580_e95456_d_n8;
        locals.var_t6_dn9 = assign58580_e95456_d_n9;
        locals.var_t6_dn10 = assign58580_e95456_d_n10;
        locals.var_t6_dn11 = assign58580_e95456_d_n11;

        let (assign58590_e95471, assign58590_e95471_d_n3, assign58590_e95471_d_n4, assign58590_e95471_d_n5, assign58590_e95471_d_n6, assign58590_e95471_d_n7, assign58590_e95471_d_n8, assign58590_e95471_d_n9, assign58590_e95471_d_n10, assign58590_e95471_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58590_e95465: f64 = (4.0 * locals.var_t6);
        let assign58590_e95467: f64 = (assign58590_e95465 / locals.var_ckappas_i);
        let assign58590_e95468: f64 = (1.0 - assign58590_e95467);
        let assign58590_e95469: f64 = (assign58590_e95468).sqrt();
        (assign58590_e95469, ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58590_e95471;
        locals.var_t1_dn3 = assign58590_e95471_d_n3;
        locals.var_t1_dn4 = assign58590_e95471_d_n4;
        locals.var_t1_dn5 = assign58590_e95471_d_n5;
        locals.var_t1_dn6 = assign58590_e95471_d_n6;
        locals.var_t1_dn7 = assign58590_e95471_d_n7;
        locals.var_t1_dn8 = assign58590_e95471_d_n8;
        locals.var_t1_dn9 = assign58590_e95471_d_n9;
        locals.var_t1_dn10 = assign58590_e95471_d_n10;
        locals.var_t1_dn11 = assign58590_e95471_d_n11;

        let (assign58600_e95503, assign58600_e95503_d_n3, assign58600_e95503_d_n4, assign58600_e95503_d_n5, assign58600_e95503_d_n6, assign58600_e95503_d_n7, assign58600_e95503_d_n8, assign58600_e95503_d_n9, assign58600_e95503_d_n10, assign58600_e95503_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58600_e95478: f64 = (-locals.var_wact);
        let assign58600_e95480: f64 = (assign58600_e95478 * p.p2);
        let assign58600_e95483: f64 = (locals.var_cgsof * locals.var_vgs_ov_noswap);
        let assign58600_e95487: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58600_e95489: f64 = (assign58600_e95487 - locals.var_vgsov);
        let assign58600_e95492: f64 = (0.5 * locals.var_ckappas_i);
        let assign58600_e95494: f64 = (-1.0);
        let assign58600_e95496: f64 = (assign58600_e95494 + locals.var_t1);
        let assign58600_e95497: f64 = (assign58600_e95492 * assign58600_e95496);
        let assign58600_e95498: f64 = (assign58600_e95489 - assign58600_e95497);
        let assign58600_e95499: f64 = (locals.var_cgsl_i * assign58600_e95498);
        let assign58600_e95500: f64 = (assign58600_e95483 + assign58600_e95499);
        let assign58600_e95501: f64 = (assign58600_e95480 * assign58600_e95500);
        (assign58600_e95501, (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn3) - (assign58600_e95492 * locals.var_t1_dn3)))), (assign58600_e95480 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgsov_dn4) - (assign58600_e95492 * locals.var_t1_dn4)))), (assign58600_e95480 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn5) - locals.var_vgsov_dn5) - (assign58600_e95492 * locals.var_t1_dn5)))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn6) - (assign58600_e95492 * locals.var_t1_dn6)))), (assign58600_e95480 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn7) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn7 - locals.var_vgsov_dn7) - (assign58600_e95492 * locals.var_t1_dn7))))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn8) - (assign58600_e95492 * locals.var_t1_dn8)))), (assign58600_e95480 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn9) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn9 - locals.var_vgsov_dn9) - (assign58600_e95492 * locals.var_t1_dn9))))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn10) - (assign58600_e95492 * locals.var_t1_dn10)))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn11) - (assign58600_e95492 * locals.var_t1_dn11)))),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11,)
    }
};
        locals.var_qovs = assign58600_e95503;
        locals.var_qovs_dn3 = assign58600_e95503_d_n3;
        locals.var_qovs_dn4 = assign58600_e95503_d_n4;
        locals.var_qovs_dn5 = assign58600_e95503_d_n5;
        locals.var_qovs_dn6 = assign58600_e95503_d_n6;
        locals.var_qovs_dn7 = assign58600_e95503_d_n7;
        locals.var_qovs_dn8 = assign58600_e95503_d_n8;
        locals.var_qovs_dn9 = assign58600_e95503_d_n9;
        locals.var_qovs_dn10 = assign58600_e95503_d_n10;
        locals.var_qovs_dn11 = assign58600_e95503_d_n11;

        let (assign58610_e95526, assign58610_e95526_d_n3, assign58610_e95526_d_n4, assign58610_e95526_d_n5, assign58610_e95526_d_n6, assign58610_e95526_d_n7, assign58610_e95526_d_n8, assign58610_e95526_d_n9, assign58610_e95526_d_n10, assign58610_e95526_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58610_e95511: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58610_e95513: f64 = (assign58610_e95511 + 0.02);
        let assign58610_e95516: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58610_e95518: f64 = (assign58610_e95516 + 0.02);
        let assign58610_e95519: f64 = (assign58610_e95513 * assign58610_e95518);
        let assign58610_e95522: f64 = (4.0 * 0.02);
        let assign58610_e95523: f64 = (assign58610_e95519 + assign58610_e95522);
        let assign58610_e95524: f64 = (assign58610_e95523).sqrt();
        (assign58610_e95524, 0.0, ((((-locals.var_vfbsdr_dn4) * assign58610_e95518) + (assign58610_e95513 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign58610_e95524)), ((((-locals.var_vfbsdr_dn5) * assign58610_e95518) + (assign58610_e95513 * (-locals.var_vfbsdr_dn5))) / (2.0 * assign58610_e95524)), (((locals.var_vgd_ov_noswap_dn6 * assign58610_e95518) + (assign58610_e95513 * locals.var_vgd_ov_noswap_dn6)) / (2.0 * assign58610_e95524)), 0.0, 0.0, (((locals.var_vgd_ov_noswap_dn9 * assign58610_e95518) + (assign58610_e95513 * locals.var_vgd_ov_noswap_dn9)) / (2.0 * assign58610_e95524)), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58610_e95526;
        locals.var_t0_dn3 = assign58610_e95526_d_n3;
        locals.var_t0_dn4 = assign58610_e95526_d_n4;
        locals.var_t0_dn5 = assign58610_e95526_d_n5;
        locals.var_t0_dn6 = assign58610_e95526_d_n6;
        locals.var_t0_dn7 = assign58610_e95526_d_n7;
        locals.var_t0_dn8 = assign58610_e95526_d_n8;
        locals.var_t0_dn9 = assign58610_e95526_d_n9;
        locals.var_t0_dn10 = assign58610_e95526_d_n10;
        locals.var_t0_dn11 = assign58610_e95526_d_n11;

        let (assign58620_e95542, assign58620_e95542_d_n3, assign58620_e95542_d_n4, assign58620_e95542_d_n5, assign58620_e95542_d_n6, assign58620_e95542_d_n7, assign58620_e95542_d_n8, assign58620_e95542_d_n9, assign58620_e95542_d_n10, assign58620_e95542_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58620_e95535: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58620_e95537: f64 = (assign58620_e95535 + 0.02);
        let assign58620_e95539: f64 = (assign58620_e95537 - locals.var_t0);
        let assign58620_e95540: f64 = (0.5 * assign58620_e95539);
        (assign58620_e95540, (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * ((-locals.var_vfbsdr_dn5) - locals.var_t0_dn5)), (0.5 * (locals.var_vgd_ov_noswap_dn6 - locals.var_t0_dn6)), (0.5 * (-locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (locals.var_vgd_ov_noswap_dn9 - locals.var_t0_dn9)), (0.5 * (-locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)),)
    } else {
        (locals.var_vgdov, locals.var_vgdov_dn3, locals.var_vgdov_dn4, locals.var_vgdov_dn5, locals.var_vgdov_dn6, locals.var_vgdov_dn7, locals.var_vgdov_dn8, locals.var_vgdov_dn9, locals.var_vgdov_dn10, locals.var_vgdov_dn11,)
    }
};
        locals.var_vgdov = assign58620_e95542;
        locals.var_vgdov_dn3 = assign58620_e95542_d_n3;
        locals.var_vgdov_dn4 = assign58620_e95542_d_n4;
        locals.var_vgdov_dn5 = assign58620_e95542_d_n5;
        locals.var_vgdov_dn6 = assign58620_e95542_d_n6;
        locals.var_vgdov_dn7 = assign58620_e95542_d_n7;
        locals.var_vgdov_dn8 = assign58620_e95542_d_n8;
        locals.var_vgdov_dn9 = assign58620_e95542_d_n9;
        locals.var_vgdov_dn10 = assign58620_e95542_d_n10;
        locals.var_vgdov_dn11 = assign58620_e95542_d_n11;

        let (assign58630_e95563, assign58630_e95563_d_n3, assign58630_e95563_d_n4, assign58630_e95563_d_n5, assign58630_e95563_d_n6, assign58630_e95563_d_n7, assign58630_e95563_d_n8, assign58630_e95563_d_n9, assign58630_e95563_d_n10, assign58630_e95563_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58630_e95551: f64 = (-locals.var_vgdov);
        let assign58630_e95553: f64 = (assign58630_e95551 / p.p891);
        let assign58630_e95555: f64 = (assign58630_e95553).powf(p.p892);
        let assign58630_e95556: f64 = (1.0 + assign58630_e95555);
        let assign58630_e95559: f64 = (1.0 / p.p892);
        let assign58630_e95560: f64 = (assign58630_e95556).powf(assign58630_e95559);
        let assign58630_e95561: f64 = (locals.var_vgdov / assign58630_e95560);
        (assign58630_e95561, (((locals.var_vgdov_dn3 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn3) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn3) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn3) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn3) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn4 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn4) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn4) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn4) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn4) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn5 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn5) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn5) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn5) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn5) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn6 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn6) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn6) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn6) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn6) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn7 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn7) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn7) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn7) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn7) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn8 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn8) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn8) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn8) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn8) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn9 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn9) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn9) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn9) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn9) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn10 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn10) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn10) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn10) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn10) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn11 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn11) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn11) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn11) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn11) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign58630_e95563;
        locals.var_t6_dn3 = assign58630_e95563_d_n3;
        locals.var_t6_dn4 = assign58630_e95563_d_n4;
        locals.var_t6_dn5 = assign58630_e95563_d_n5;
        locals.var_t6_dn6 = assign58630_e95563_d_n6;
        locals.var_t6_dn7 = assign58630_e95563_d_n7;
        locals.var_t6_dn8 = assign58630_e95563_d_n8;
        locals.var_t6_dn9 = assign58630_e95563_d_n9;
        locals.var_t6_dn10 = assign58630_e95563_d_n10;
        locals.var_t6_dn11 = assign58630_e95563_d_n11;

        let (assign58640_e95578, assign58640_e95578_d_n3, assign58640_e95578_d_n4, assign58640_e95578_d_n5, assign58640_e95578_d_n6, assign58640_e95578_d_n7, assign58640_e95578_d_n8, assign58640_e95578_d_n9, assign58640_e95578_d_n10, assign58640_e95578_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58640_e95572: f64 = (4.0 * locals.var_t6);
        let assign58640_e95574: f64 = (assign58640_e95572 / locals.var_ckappad_i);
        let assign58640_e95575: f64 = (1.0 - assign58640_e95574);
        let assign58640_e95576: f64 = (assign58640_e95575).sqrt();
        (assign58640_e95576, ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign58640_e95578;
        locals.var_t2_dn3 = assign58640_e95578_d_n3;
        locals.var_t2_dn4 = assign58640_e95578_d_n4;
        locals.var_t2_dn5 = assign58640_e95578_d_n5;
        locals.var_t2_dn6 = assign58640_e95578_d_n6;
        locals.var_t2_dn7 = assign58640_e95578_d_n7;
        locals.var_t2_dn8 = assign58640_e95578_d_n8;
        locals.var_t2_dn9 = assign58640_e95578_d_n9;
        locals.var_t2_dn10 = assign58640_e95578_d_n10;
        locals.var_t2_dn11 = assign58640_e95578_d_n11;

        let (assign58650_e95610, assign58650_e95610_d_n3, assign58650_e95610_d_n4, assign58650_e95610_d_n5, assign58650_e95610_d_n6, assign58650_e95610_d_n7, assign58650_e95610_d_n8, assign58650_e95610_d_n9, assign58650_e95610_d_n10, assign58650_e95610_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58650_e95585: f64 = (-locals.var_wact);
        let assign58650_e95587: f64 = (assign58650_e95585 * p.p2);
        let assign58650_e95590: f64 = (locals.var_cgdof * locals.var_vgd_ov_noswap);
        let assign58650_e95594: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58650_e95596: f64 = (assign58650_e95594 - locals.var_vgdov);
        let assign58650_e95599: f64 = (0.5 * locals.var_ckappad_i);
        let assign58650_e95601: f64 = (-1.0);
        let assign58650_e95603: f64 = (assign58650_e95601 + locals.var_t2);
        let assign58650_e95604: f64 = (assign58650_e95599 * assign58650_e95603);
        let assign58650_e95605: f64 = (assign58650_e95596 - assign58650_e95604);
        let assign58650_e95606: f64 = (locals.var_cgdl_i * assign58650_e95605);
        let assign58650_e95607: f64 = (assign58650_e95590 + assign58650_e95606);
        let assign58650_e95608: f64 = (assign58650_e95587 * assign58650_e95607);
        (assign58650_e95608, (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn3) - (assign58650_e95599 * locals.var_t2_dn3)))), (assign58650_e95587 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgdov_dn4) - (assign58650_e95599 * locals.var_t2_dn4)))), (assign58650_e95587 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn5) - locals.var_vgdov_dn5) - (assign58650_e95599 * locals.var_t2_dn5)))), (assign58650_e95587 * ((locals.var_cgdof * locals.var_vgd_ov_noswap_dn6) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswap_dn6 - locals.var_vgdov_dn6) - (assign58650_e95599 * locals.var_t2_dn6))))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn7) - (assign58650_e95599 * locals.var_t2_dn7)))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn8) - (assign58650_e95599 * locals.var_t2_dn8)))), (assign58650_e95587 * ((locals.var_cgdof * locals.var_vgd_ov_noswap_dn9) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswap_dn9 - locals.var_vgdov_dn9) - (assign58650_e95599 * locals.var_t2_dn9))))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn10) - (assign58650_e95599 * locals.var_t2_dn10)))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn11) - (assign58650_e95599 * locals.var_t2_dn11)))),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11,)
    }
};
        locals.var_qovd = assign58650_e95610;
        locals.var_qovd_dn3 = assign58650_e95610_d_n3;
        locals.var_qovd_dn4 = assign58650_e95610_d_n4;
        locals.var_qovd_dn5 = assign58650_e95610_d_n5;
        locals.var_qovd_dn6 = assign58650_e95610_d_n6;
        locals.var_qovd_dn7 = assign58650_e95610_d_n7;
        locals.var_qovd_dn8 = assign58650_e95610_d_n8;
        locals.var_qovd_dn9 = assign58650_e95610_d_n9;
        locals.var_qovd_dn10 = assign58650_e95610_d_n10;
        locals.var_qovd_dn11 = assign58650_e95610_d_n11;

        let (assign58660_e95624, assign58660_e95624_d_n9, assign58660_e95624_d_n10,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58660_e95614: f64 = (-locals.var_devsign);
        let assign58660_e95616: f64 = (assign58660_e95614 * p.p2);
        let assign58660_e95618: f64 = (assign58660_e95616 * locals.var_lact);
        let assign58660_e95620: f64 = (assign58660_e95618 * p.p874);
        let assign58660_e95622: f64 = (assign58660_e95620 * (nv9 - nv10));
        (assign58660_e95622, assign58660_e95620, (-assign58660_e95620),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn9, locals.var_qovb_dn10,)
    }
};
        locals.var_qovb = assign58660_e95624;
        locals.var_qovb_dn9 = assign58660_e95624_d_n9;
        locals.var_qovb_dn10 = assign58660_e95624_d_n10;

        let (assign58670_e95634, assign58670_e95634_d_n3, assign58670_e95634_d_n4, assign58670_e95634_d_n5, assign58670_e95634_d_n6, assign58670_e95634_d_n7, assign58670_e95634_d_n8, assign58670_e95634_d_n9, assign58670_e95634_d_n10, assign58670_e95634_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58670_e95629: f64 = (locals.var_qovs + locals.var_qovd);
        let assign58670_e95631: f64 = (assign58670_e95629 + locals.var_qovb);
        let assign58670_e95632: f64 = (-assign58670_e95631);
        (assign58670_e95632, (-(locals.var_qovs_dn3 + locals.var_qovd_dn3)), (-(locals.var_qovs_dn4 + locals.var_qovd_dn4)), (-(locals.var_qovs_dn5 + locals.var_qovd_dn5)), (-(locals.var_qovs_dn6 + locals.var_qovd_dn6)), (-(locals.var_qovs_dn7 + locals.var_qovd_dn7)), (-(locals.var_qovs_dn8 + locals.var_qovd_dn8)), (-((locals.var_qovs_dn9 + locals.var_qovd_dn9) + locals.var_qovb_dn9)), (-((locals.var_qovs_dn10 + locals.var_qovd_dn10) + locals.var_qovb_dn10)), (-(locals.var_qovs_dn11 + locals.var_qovd_dn11)),)
    } else {
        (locals.var_qovg, locals.var_qovg_dn3, locals.var_qovg_dn4, locals.var_qovg_dn5, locals.var_qovg_dn6, locals.var_qovg_dn7, locals.var_qovg_dn8, locals.var_qovg_dn9, locals.var_qovg_dn10, locals.var_qovg_dn11,)
    }
};
        locals.var_qovg = assign58670_e95634;
        locals.var_qovg_dn3 = assign58670_e95634_d_n3;
        locals.var_qovg_dn4 = assign58670_e95634_d_n4;
        locals.var_qovg_dn5 = assign58670_e95634_d_n5;
        locals.var_qovg_dn6 = assign58670_e95634_d_n6;
        locals.var_qovg_dn7 = assign58670_e95634_d_n7;
        locals.var_qovg_dn8 = assign58670_e95634_d_n8;
        locals.var_qovg_dn9 = assign58670_e95634_d_n9;
        locals.var_qovg_dn10 = assign58670_e95634_d_n10;
        locals.var_qovg_dn11 = assign58670_e95634_d_n11;

        let (assign58680_e95645,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58680_e95640: f64 = (2.0 * locals.var_dlcv);
        let assign58680_e95641: f64 = (locals.var_lnew - assign58680_e95640);
        let assign58680_e95643: f64 = (assign58680_e95641 - p.p1394);
        (assign58680_e95643,)
    } else {
        (locals.var_leffcvb,)
    }
};
        locals.var_leffcvb = assign58680_e95645;

        let (assign58690_e95654,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58690_e95651: f64 = (2.0 * p.p1393);
        let assign58690_e95652: f64 = (locals.var_leffcvb + assign58690_e95651);
        (assign58690_e95652,)
    } else {
        (locals.var_leffcvbg,)
    }
};
        locals.var_leffcvbg = assign58690_e95654;

        let assign58700_e95657: f64 = if locals.var_nsub_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard863 = assign58700_e95657;

        let (assign58710_e95669, assign58710_e95669_d_n3, assign58710_e95669_d_n4, assign58710_e95669_d_n5, assign58710_e95669_d_n6, assign58710_e95669_d_n7, assign58710_e95669_d_n8, assign58710_e95669_d_n9, assign58710_e95669_d_n10, assign58710_e95669_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 != 0.0)) {
        let assign58710_e95664: f64 = (locals.var_ndep_i / locals.var_nsub_i);
        let assign58710_e95666: f64 = (assign58710_e95664).max(1e-38);
        let assign58710_e95667: f64 = (assign58710_e95666).ln();
        (assign58710_e95667, (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn3 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn4 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn5 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn6 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn7 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn8 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn9 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn10 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn11 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58710_e95669;
        locals.var_t0_dn3 = assign58710_e95669_d_n3;
        locals.var_t0_dn4 = assign58710_e95669_d_n4;
        locals.var_t0_dn5 = assign58710_e95669_d_n5;
        locals.var_t0_dn6 = assign58710_e95669_d_n6;
        locals.var_t0_dn7 = assign58710_e95669_d_n7;
        locals.var_t0_dn8 = assign58710_e95669_d_n8;
        locals.var_t0_dn9 = assign58710_e95669_d_n9;
        locals.var_t0_dn10 = assign58710_e95669_d_n10;
        locals.var_t0_dn11 = assign58710_e95669_d_n11;

        let (assign58720_e95681, assign58720_e95681_d_n3, assign58720_e95681_d_n4, assign58720_e95681_d_n5, assign58720_e95681_d_n6, assign58720_e95681_d_n7, assign58720_e95681_d_n8, assign58720_e95681_d_n9, assign58720_e95681_d_n10, assign58720_e95681_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 != 0.0)) {
        let assign58720_e95675: f64 = (-locals.var_devsign);
        let assign58720_e95677: f64 = (assign58720_e95675 * locals.var_vtm);
        let assign58720_e95679: f64 = (assign58720_e95677 * locals.var_t0);
        (assign58720_e95679, (assign58720_e95677 * locals.var_t0_dn3), (((assign58720_e95675 * locals.var_vtm_dn4) * locals.var_t0) + (assign58720_e95677 * locals.var_t0_dn4)), (((assign58720_e95675 * locals.var_vtm_dn5) * locals.var_t0) + (assign58720_e95677 * locals.var_t0_dn5)), (assign58720_e95677 * locals.var_t0_dn6), (assign58720_e95677 * locals.var_t0_dn7), (assign58720_e95677 * locals.var_t0_dn8), (assign58720_e95677 * locals.var_t0_dn9), (assign58720_e95677 * locals.var_t0_dn10), (assign58720_e95677 * locals.var_t0_dn11),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11,)
    }
};
        locals.var_vfbb = assign58720_e95681;
        locals.var_vfbb_dn3 = assign58720_e95681_d_n3;
        locals.var_vfbb_dn4 = assign58720_e95681_d_n4;
        locals.var_vfbb_dn5 = assign58720_e95681_d_n5;
        locals.var_vfbb_dn6 = assign58720_e95681_d_n6;
        locals.var_vfbb_dn7 = assign58720_e95681_d_n7;
        locals.var_vfbb_dn8 = assign58720_e95681_d_n8;
        locals.var_vfbb_dn9 = assign58720_e95681_d_n9;
        locals.var_vfbb_dn10 = assign58720_e95681_d_n10;
        locals.var_vfbb_dn11 = assign58720_e95681_d_n11;

        let (assign58730_e95699, assign58730_e95699_d_n3, assign58730_e95699_d_n4, assign58730_e95699_d_n5, assign58730_e95699_d_n6, assign58730_e95699_d_n7, assign58730_e95699_d_n8, assign58730_e95699_d_n9, assign58730_e95699_d_n10, assign58730_e95699_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 == 0.0)) {
        let assign58730_e95688: f64 = (-locals.var_ndep_i);
        let assign58730_e95690: f64 = (assign58730_e95688 * locals.var_nsub_i);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_ni;
        let assign58730_e95692: f64 = (assign58730_e95690 * __rspice_inv_cse_0);
        let assign58730_e95694: f64 = (assign58730_e95692 * __rspice_inv_cse_0);
        let assign58730_e95696: f64 = (assign58730_e95694).max(1e-38);
        let assign58730_e95697: f64 = (assign58730_e95696).ln();
        (assign58730_e95697, (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn3) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn4) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn5) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn6) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn7) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn8) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn9) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn10) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn11) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58730_e95699;
        locals.var_t0_dn3 = assign58730_e95699_d_n3;
        locals.var_t0_dn4 = assign58730_e95699_d_n4;
        locals.var_t0_dn5 = assign58730_e95699_d_n5;
        locals.var_t0_dn6 = assign58730_e95699_d_n6;
        locals.var_t0_dn7 = assign58730_e95699_d_n7;
        locals.var_t0_dn8 = assign58730_e95699_d_n8;
        locals.var_t0_dn9 = assign58730_e95699_d_n9;
        locals.var_t0_dn10 = assign58730_e95699_d_n10;
        locals.var_t0_dn11 = assign58730_e95699_d_n11;

        let (assign58740_e95712, assign58740_e95712_d_n3, assign58740_e95712_d_n4, assign58740_e95712_d_n5, assign58740_e95712_d_n6, assign58740_e95712_d_n7, assign58740_e95712_d_n8, assign58740_e95712_d_n9, assign58740_e95712_d_n10, assign58740_e95712_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 == 0.0)) {
        let assign58740_e95706: f64 = (-locals.var_devsign);
        let assign58740_e95708: f64 = (assign58740_e95706 * locals.var_vtm);
        let assign58740_e95710: f64 = (assign58740_e95708 * locals.var_t0);
        (assign58740_e95710, (assign58740_e95708 * locals.var_t0_dn3), (((assign58740_e95706 * locals.var_vtm_dn4) * locals.var_t0) + (assign58740_e95708 * locals.var_t0_dn4)), (((assign58740_e95706 * locals.var_vtm_dn5) * locals.var_t0) + (assign58740_e95708 * locals.var_t0_dn5)), (assign58740_e95708 * locals.var_t0_dn6), (assign58740_e95708 * locals.var_t0_dn7), (assign58740_e95708 * locals.var_t0_dn8), (assign58740_e95708 * locals.var_t0_dn9), (assign58740_e95708 * locals.var_t0_dn10), (assign58740_e95708 * locals.var_t0_dn11),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11,)
    }
};
        locals.var_vfbb = assign58740_e95712;
        locals.var_vfbb_dn3 = assign58740_e95712_d_n3;
        locals.var_vfbb_dn4 = assign58740_e95712_d_n4;
        locals.var_vfbb_dn5 = assign58740_e95712_d_n5;
        locals.var_vfbb_dn6 = assign58740_e95712_d_n6;
        locals.var_vfbb_dn7 = assign58740_e95712_d_n7;
        locals.var_vfbb_dn8 = assign58740_e95712_d_n8;
        locals.var_vfbb_dn9 = assign58740_e95712_d_n9;
        locals.var_vfbb_dn10 = assign58740_e95712_d_n10;
        locals.var_vfbb_dn11 = assign58740_e95712_d_n11;

        let (assign58750_e95719, assign58750_e95719_d_n3, assign58750_e95719_d_n4, assign58750_e95719_d_n5, assign58750_e95719_d_n6, assign58750_e95719_d_n7, assign58750_e95719_d_n8, assign58750_e95719_d_n9, assign58750_e95719_d_n10, assign58750_e95719_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58750_e95717: f64 = (locals.var_ves - locals.var_vfbb);
        (assign58750_e95717, (locals.var_ves_dn3 - locals.var_vfbb_dn3), (-locals.var_vfbb_dn4), (-locals.var_vfbb_dn5), (locals.var_ves_dn6 - locals.var_vfbb_dn6), (locals.var_ves_dn7 - locals.var_vfbb_dn7), (-locals.var_vfbb_dn8), (-locals.var_vfbb_dn9), (locals.var_ves_dn10 - locals.var_vfbb_dn10), (-locals.var_vfbb_dn11),)
    } else {
        (locals.var_vesfb, locals.var_vesfb_dn3, locals.var_vesfb_dn4, locals.var_vesfb_dn5, locals.var_vesfb_dn6, locals.var_vesfb_dn7, locals.var_vesfb_dn8, locals.var_vesfb_dn9, locals.var_vesfb_dn10, locals.var_vesfb_dn11,)
    }
};
        locals.var_vesfb = assign58750_e95719;
        locals.var_vesfb_dn3 = assign58750_e95719_d_n3;
        locals.var_vesfb_dn4 = assign58750_e95719_d_n4;
        locals.var_vesfb_dn5 = assign58750_e95719_d_n5;
        locals.var_vesfb_dn6 = assign58750_e95719_d_n6;
        locals.var_vesfb_dn7 = assign58750_e95719_d_n7;
        locals.var_vesfb_dn8 = assign58750_e95719_d_n8;
        locals.var_vesfb_dn9 = assign58750_e95719_d_n9;
        locals.var_vesfb_dn10 = assign58750_e95719_d_n10;
        locals.var_vesfb_dn11 = assign58750_e95719_d_n11;

        let (assign58760_e95726,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58760_e95724: f64 = (3.453133e-11 / p.p75);
        (assign58760_e95724,)
    } else {
        (locals.var_cbox_1,)
    }
};
        locals.var_cbox_1 = assign58760_e95726;

        let (assign58770_e95745,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58770_e95731: f64 = (locals.var_kb1_i * p.p1388);
        let assign58770_e95733: f64 = (assign58770_e95731 * locals.var_cbox_1);
        let assign58770_e95736: f64 = (locals.var_wact / p.p1373);
        let assign58770_e95738: f64 = (assign58770_e95736 * p.p2);
        let assign58770_e95740: f64 = (assign58770_e95738 * locals.var_leffcvbg);
        let assign58770_e95742: f64 = (assign58770_e95740 + p.p1382);
        let assign58770_e95743: f64 = (assign58770_e95733 * assign58770_e95742);
        (assign58770_e95743,)
    } else {
        (locals.var_cboxwl,)
    }
};
        locals.var_cboxwl = assign58770_e95745;

        let (assign58780_e95754, assign58780_e95754_d_n3, assign58780_e95754_d_n4, assign58780_e95754_d_n5, assign58780_e95754_d_n6, assign58780_e95754_d_n7, assign58780_e95754_d_n8, assign58780_e95754_d_n9, assign58780_e95754_d_n10, assign58780_e95754_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58780_e95751: f64 = (locals.var_vesfb - locals.var_vbs);
        let assign58780_e95752: f64 = (locals.var_cboxwl * assign58780_e95751);
        (assign58780_e95752, (locals.var_cboxwl * locals.var_vesfb_dn3), (locals.var_cboxwl * locals.var_vesfb_dn4), (locals.var_cboxwl * locals.var_vesfb_dn5), (locals.var_cboxwl * (locals.var_vesfb_dn6 - locals.var_vbs_dn6)), (locals.var_cboxwl * (locals.var_vesfb_dn7 - locals.var_vbs_dn7)), (locals.var_cboxwl * locals.var_vesfb_dn8), (locals.var_cboxwl * locals.var_vesfb_dn9), (locals.var_cboxwl * (locals.var_vesfb_dn10 - locals.var_vbs_dn10)), (locals.var_cboxwl * locals.var_vesfb_dn11),)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11,)
    }
};
        locals.var_qe1 = assign58780_e95754;
        locals.var_qe1_dn3 = assign58780_e95754_d_n3;
        locals.var_qe1_dn4 = assign58780_e95754_d_n4;
        locals.var_qe1_dn5 = assign58780_e95754_d_n5;
        locals.var_qe1_dn6 = assign58780_e95754_d_n6;
        locals.var_qe1_dn7 = assign58780_e95754_d_n7;
        locals.var_qe1_dn8 = assign58780_e95754_d_n8;
        locals.var_qe1_dn9 = assign58780_e95754_d_n9;
        locals.var_qe1_dn10 = assign58780_e95754_d_n10;
        locals.var_qe1_dn11 = assign58780_e95754_d_n11;

    }

    pub(super) fn stamp_transient_block_194(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign58790_e95759, assign58790_e95759_d_n3, assign58790_e95759_d_n4, assign58790_e95759_d_n5, assign58790_e95759_d_n6, assign58790_e95759_d_n7, assign58790_e95759_d_n8, assign58790_e95759_d_n9, assign58790_e95759_d_n10, assign58790_e95759_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11,)
    }
};
        locals.var_qsub = assign58790_e95759;
        locals.var_qsub_dn3 = assign58790_e95759_d_n3;
        locals.var_qsub_dn4 = assign58790_e95759_d_n4;
        locals.var_qsub_dn5 = assign58790_e95759_d_n5;
        locals.var_qsub_dn6 = assign58790_e95759_d_n6;
        locals.var_qsub_dn7 = assign58790_e95759_d_n7;
        locals.var_qsub_dn8 = assign58790_e95759_d_n8;
        locals.var_qsub_dn9 = assign58790_e95759_d_n9;
        locals.var_qsub_dn10 = assign58790_e95759_d_n10;
        locals.var_qsub_dn11 = assign58790_e95759_d_n11;

        let (assign58800_e95775, assign58800_e95775_d_n3, assign58800_e95775_d_n4, assign58800_e95775_d_n5, assign58800_e95775_d_n6, assign58800_e95775_d_n7, assign58800_e95775_d_n8, assign58800_e95775_d_n9, assign58800_e95775_d_n10, assign58800_e95775_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58800_e95767: f64 = (p.p74 / p.p75);
        let assign58800_e95768: f64 = (1.0 + assign58800_e95767);
        let assign58800_e95769: f64 = (p.p871 * assign58800_e95768);
        let assign58800_e95771: f64 = (assign58800_e95769).max(1e-38);
        let assign58800_e95772: f64 = (assign58800_e95771).ln();
        let assign58800_e95773: f64 = (p.p1395 * assign58800_e95772);
        (assign58800_e95773, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58800_e95775;
        locals.var_t0_dn3 = assign58800_e95775_d_n3;
        locals.var_t0_dn4 = assign58800_e95775_d_n4;
        locals.var_t0_dn5 = assign58800_e95775_d_n5;
        locals.var_t0_dn6 = assign58800_e95775_d_n6;
        locals.var_t0_dn7 = assign58800_e95775_d_n7;
        locals.var_t0_dn8 = assign58800_e95775_d_n8;
        locals.var_t0_dn9 = assign58800_e95775_d_n9;
        locals.var_t0_dn10 = assign58800_e95775_d_n10;
        locals.var_t0_dn11 = assign58800_e95775_d_n11;

        let (assign58810_e95782, assign58810_e95782_d_n3, assign58810_e95782_d_n4, assign58810_e95782_d_n5, assign58810_e95782_d_n6, assign58810_e95782_d_n7, assign58810_e95782_d_n8, assign58810_e95782_d_n9, assign58810_e95782_d_n10, assign58810_e95782_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58810_e95780: f64 = (p.p19 - p.p1);
        (assign58810_e95780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58810_e95782;
        locals.var_t1_dn3 = assign58810_e95782_d_n3;
        locals.var_t1_dn4 = assign58810_e95782_d_n4;
        locals.var_t1_dn5 = assign58810_e95782_d_n5;
        locals.var_t1_dn6 = assign58810_e95782_d_n6;
        locals.var_t1_dn7 = assign58810_e95782_d_n7;
        locals.var_t1_dn8 = assign58810_e95782_d_n8;
        locals.var_t1_dn9 = assign58810_e95782_d_n9;
        locals.var_t1_dn10 = assign58810_e95782_d_n10;
        locals.var_t1_dn11 = assign58810_e95782_d_n11;

        let assign58820_e95785: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard864 = assign58820_e95785;

        let (assign58830_e95794, assign58830_e95794_d_n3, assign58830_e95794_d_n4, assign58830_e95794_d_n5, assign58830_e95794_d_n6, assign58830_e95794_d_n7, assign58830_e95794_d_n8, assign58830_e95794_d_n9, assign58830_e95794_d_n10, assign58830_e95794_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard864 != 0.0)) {
        let assign58830_e95792: f64 = (locals.var_t0 * locals.var_t1);
        (assign58830_e95792, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)),)
    } else {
        (locals.var_csesw, locals.var_csesw_dn3, locals.var_csesw_dn4, locals.var_csesw_dn5, locals.var_csesw_dn6, locals.var_csesw_dn7, locals.var_csesw_dn8, locals.var_csesw_dn9, locals.var_csesw_dn10, locals.var_csesw_dn11,)
    }
};
        locals.var_csesw = assign58830_e95794;
        locals.var_csesw_dn3 = assign58830_e95794_d_n3;
        locals.var_csesw_dn4 = assign58830_e95794_d_n4;
        locals.var_csesw_dn5 = assign58830_e95794_d_n5;
        locals.var_csesw_dn6 = assign58830_e95794_d_n6;
        locals.var_csesw_dn7 = assign58830_e95794_d_n7;
        locals.var_csesw_dn8 = assign58830_e95794_d_n8;
        locals.var_csesw_dn9 = assign58830_e95794_d_n9;
        locals.var_csesw_dn10 = assign58830_e95794_d_n10;
        locals.var_csesw_dn11 = assign58830_e95794_d_n11;

        let (assign58840_e95802, assign58840_e95802_d_n3, assign58840_e95802_d_n4, assign58840_e95802_d_n5, assign58840_e95802_d_n6, assign58840_e95802_d_n7, assign58840_e95802_d_n8, assign58840_e95802_d_n9, assign58840_e95802_d_n10, assign58840_e95802_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard864 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_csesw, locals.var_csesw_dn3, locals.var_csesw_dn4, locals.var_csesw_dn5, locals.var_csesw_dn6, locals.var_csesw_dn7, locals.var_csesw_dn8, locals.var_csesw_dn9, locals.var_csesw_dn10, locals.var_csesw_dn11,)
    }
};
        locals.var_csesw = assign58840_e95802;
        locals.var_csesw_dn3 = assign58840_e95802_d_n3;
        locals.var_csesw_dn4 = assign58840_e95802_d_n4;
        locals.var_csesw_dn5 = assign58840_e95802_d_n5;
        locals.var_csesw_dn6 = assign58840_e95802_d_n6;
        locals.var_csesw_dn7 = assign58840_e95802_d_n7;
        locals.var_csesw_dn8 = assign58840_e95802_d_n8;
        locals.var_csesw_dn9 = assign58840_e95802_d_n9;
        locals.var_csesw_dn10 = assign58840_e95802_d_n10;
        locals.var_csesw_dn11 = assign58840_e95802_d_n11;

        let (assign58850_e95809, assign58850_e95809_d_n3, assign58850_e95809_d_n4, assign58850_e95809_d_n5, assign58850_e95809_d_n6, assign58850_e95809_d_n7, assign58850_e95809_d_n8, assign58850_e95809_d_n9, assign58850_e95809_d_n10, assign58850_e95809_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58850_e95807: f64 = (p.p20 - p.p1);
        (assign58850_e95807, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58850_e95809;
        locals.var_t1_dn3 = assign58850_e95809_d_n3;
        locals.var_t1_dn4 = assign58850_e95809_d_n4;
        locals.var_t1_dn5 = assign58850_e95809_d_n5;
        locals.var_t1_dn6 = assign58850_e95809_d_n6;
        locals.var_t1_dn7 = assign58850_e95809_d_n7;
        locals.var_t1_dn8 = assign58850_e95809_d_n8;
        locals.var_t1_dn9 = assign58850_e95809_d_n9;
        locals.var_t1_dn10 = assign58850_e95809_d_n10;
        locals.var_t1_dn11 = assign58850_e95809_d_n11;

        let assign58860_e95812: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard865 = assign58860_e95812;

        let (assign58870_e95821, assign58870_e95821_d_n3, assign58870_e95821_d_n4, assign58870_e95821_d_n5, assign58870_e95821_d_n6, assign58870_e95821_d_n7, assign58870_e95821_d_n8, assign58870_e95821_d_n9, assign58870_e95821_d_n10, assign58870_e95821_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard865 != 0.0)) {
        let assign58870_e95819: f64 = (locals.var_t0 * locals.var_t1);
        (assign58870_e95819, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)),)
    } else {
        (locals.var_cdesw, locals.var_cdesw_dn3, locals.var_cdesw_dn4, locals.var_cdesw_dn5, locals.var_cdesw_dn6, locals.var_cdesw_dn7, locals.var_cdesw_dn8, locals.var_cdesw_dn9, locals.var_cdesw_dn10, locals.var_cdesw_dn11,)
    }
};
        locals.var_cdesw = assign58870_e95821;
        locals.var_cdesw_dn3 = assign58870_e95821_d_n3;
        locals.var_cdesw_dn4 = assign58870_e95821_d_n4;
        locals.var_cdesw_dn5 = assign58870_e95821_d_n5;
        locals.var_cdesw_dn6 = assign58870_e95821_d_n6;
        locals.var_cdesw_dn7 = assign58870_e95821_d_n7;
        locals.var_cdesw_dn8 = assign58870_e95821_d_n8;
        locals.var_cdesw_dn9 = assign58870_e95821_d_n9;
        locals.var_cdesw_dn10 = assign58870_e95821_d_n10;
        locals.var_cdesw_dn11 = assign58870_e95821_d_n11;

        let (assign58880_e95829, assign58880_e95829_d_n3, assign58880_e95829_d_n4, assign58880_e95829_d_n5, assign58880_e95829_d_n6, assign58880_e95829_d_n7, assign58880_e95829_d_n8, assign58880_e95829_d_n9, assign58880_e95829_d_n10, assign58880_e95829_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard865 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdesw, locals.var_cdesw_dn3, locals.var_cdesw_dn4, locals.var_cdesw_dn5, locals.var_cdesw_dn6, locals.var_cdesw_dn7, locals.var_cdesw_dn8, locals.var_cdesw_dn9, locals.var_cdesw_dn10, locals.var_cdesw_dn11,)
    }
};
        locals.var_cdesw = assign58880_e95829;
        locals.var_cdesw_dn3 = assign58880_e95829_d_n3;
        locals.var_cdesw_dn4 = assign58880_e95829_d_n4;
        locals.var_cdesw_dn5 = assign58880_e95829_d_n5;
        locals.var_cdesw_dn6 = assign58880_e95829_d_n6;
        locals.var_cdesw_dn7 = assign58880_e95829_d_n7;
        locals.var_cdesw_dn8 = assign58880_e95829_d_n8;
        locals.var_cdesw_dn9 = assign58880_e95829_d_n9;
        locals.var_cdesw_dn10 = assign58880_e95829_d_n10;
        locals.var_cdesw_dn11 = assign58880_e95829_d_n11;

        let (assign58890_e95836,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58890_e95834: f64 = (locals.var_cbox_1 * p.p17);
        (assign58890_e95834,)
    } else {
        (locals.var_csbox,)
    }
};
        locals.var_csbox = assign58890_e95836;

        let (assign58900_e95843,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58900_e95841: f64 = (p.p1396 * p.p17);
        (assign58900_e95841,)
    } else {
        (locals.var_csmin,)
    }
};
        locals.var_csmin = assign58900_e95843;

        let (assign58910_e95850,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58910_e95848: f64 = (locals.var_cbox_1 * p.p18);
        (assign58910_e95848,)
    } else {
        (locals.var_cdbox,)
    }
};
        locals.var_cdbox = assign58910_e95850;

        let (assign58920_e95857,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58920_e95855: f64 = (p.p1396 * p.p18);
        (assign58920_e95855,)
    } else {
        (locals.var_cdmin,)
    }
};
        locals.var_cdmin = assign58920_e95857;

        let (assign58930_e95865, assign58930_e95865_d_n3, assign58930_e95865_d_n4, assign58930_e95865_d_n5, assign58930_e95865_d_n6, assign58930_e95865_d_n7, assign58930_e95865_d_n8, assign58930_e95865_d_n9, assign58930_e95865_d_n10, assign58930_e95865_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58930_e95861: f64 = (-locals.var_devsign);
        let assign58930_e95863: f64 = (assign58930_e95861 * locals.var_ves_1);
        (assign58930_e95863, (assign58930_e95861 * locals.var_ves_1_dn3), 0.0, 0.0, (assign58930_e95861 * locals.var_ves_1_dn6), (assign58930_e95861 * locals.var_ves_1_dn7), 0.0, 0.0, (assign58930_e95861 * locals.var_ves_1_dn10), 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign58930_e95865;
        locals.var_t10_dn3 = assign58930_e95865_d_n3;
        locals.var_t10_dn4 = assign58930_e95865_d_n4;
        locals.var_t10_dn5 = assign58930_e95865_d_n5;
        locals.var_t10_dn6 = assign58930_e95865_d_n6;
        locals.var_t10_dn7 = assign58930_e95865_d_n7;
        locals.var_t10_dn8 = assign58930_e95865_d_n8;
        locals.var_t10_dn9 = assign58930_e95865_d_n9;
        locals.var_t10_dn10 = assign58930_e95865_d_n10;
        locals.var_t10_dn11 = assign58930_e95865_d_n11;

        let (assign58940_e95873, assign58940_e95873_d_n3, assign58940_e95873_d_n4, assign58940_e95873_d_n5, assign58940_e95873_d_n6, assign58940_e95873_d_n7, assign58940_e95873_d_n8, assign58940_e95873_d_n9, assign58940_e95873_d_n10, assign58940_e95873_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58940_e95869: f64 = (-locals.var_devsign);
        let assign58940_e95871: f64 = (assign58940_e95869 * locals.var_ved);
        (assign58940_e95871, (assign58940_e95869 * locals.var_ved_dn3), 0.0, 0.0, (assign58940_e95869 * locals.var_ved_dn6), (assign58940_e95869 * locals.var_ved_dn7), 0.0, 0.0, (assign58940_e95869 * locals.var_ved_dn10), 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign58940_e95873;
        locals.var_t11_dn3 = assign58940_e95873_d_n3;
        locals.var_t11_dn4 = assign58940_e95873_d_n4;
        locals.var_t11_dn5 = assign58940_e95873_d_n5;
        locals.var_t11_dn6 = assign58940_e95873_d_n6;
        locals.var_t11_dn7 = assign58940_e95873_d_n7;
        locals.var_t11_dn8 = assign58940_e95873_d_n8;
        locals.var_t11_dn9 = assign58940_e95873_d_n9;
        locals.var_t11_dn10 = assign58940_e95873_d_n10;
        locals.var_t11_dn11 = assign58940_e95873_d_n11;

        let assign58950_e95876: f64 = if p.p1396 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard866 = assign58950_e95876;

        let (assign58960_e95890, assign58960_e95890_d_n3, assign58960_e95890_d_n4, assign58960_e95890_d_n5, assign58960_e95890_d_n6, assign58960_e95890_d_n7, assign58960_e95890_d_n8, assign58960_e95890_d_n9, assign58960_e95890_d_n10, assign58960_e95890_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58960_e95882: f64 = (-0.5);
        let assign58960_e95885: f64 = (locals.var_cdbox - locals.var_cdmin);
        let assign58960_e95886: f64 = (assign58960_e95882 * assign58960_e95885);
        let assign58960_e95888: f64 = (assign58960_e95886 / p.p1399);
        (assign58960_e95888, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58960_e95890;
        locals.var_t1_dn3 = assign58960_e95890_d_n3;
        locals.var_t1_dn4 = assign58960_e95890_d_n4;
        locals.var_t1_dn5 = assign58960_e95890_d_n5;
        locals.var_t1_dn6 = assign58960_e95890_d_n6;
        locals.var_t1_dn7 = assign58960_e95890_d_n7;
        locals.var_t1_dn8 = assign58960_e95890_d_n8;
        locals.var_t1_dn9 = assign58960_e95890_d_n9;
        locals.var_t1_dn10 = assign58960_e95890_d_n10;
        locals.var_t1_dn11 = assign58960_e95890_d_n11;

        let (assign58970_e95906, assign58970_e95906_d_n3, assign58970_e95906_d_n4, assign58970_e95906_d_n5, assign58970_e95906_d_n6, assign58970_e95906_d_n7, assign58970_e95906_d_n8, assign58970_e95906_d_n9, assign58970_e95906_d_n10, assign58970_e95906_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58970_e95896: f64 = (-p.p1399);
        let assign58970_e95898: f64 = (assign58970_e95896 * locals.var_t11);
        let assign58970_e95900: f64 = (assign58970_e95898 + p.p1400);
        let assign58970_e95901: f64 = (assign58970_e95900).cosh();
        let assign58970_e95903: f64 = (assign58970_e95901).max(1e-38);
        let assign58970_e95904: f64 = (assign58970_e95903).ln();
        (assign58970_e95904, (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn3)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn4)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn5)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn6)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn7)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn8)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn9)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn10)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn11)) } else { 0.0 } / assign58970_e95903),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign58970_e95906;
        locals.var_t2_dn3 = assign58970_e95906_d_n3;
        locals.var_t2_dn4 = assign58970_e95906_d_n4;
        locals.var_t2_dn5 = assign58970_e95906_d_n5;
        locals.var_t2_dn6 = assign58970_e95906_d_n6;
        locals.var_t2_dn7 = assign58970_e95906_d_n7;
        locals.var_t2_dn8 = assign58970_e95906_d_n8;
        locals.var_t2_dn9 = assign58970_e95906_d_n9;
        locals.var_t2_dn10 = assign58970_e95906_d_n10;
        locals.var_t2_dn11 = assign58970_e95906_d_n11;

        let (assign58980_e95919, assign58980_e95919_d_n3, assign58980_e95919_d_n4, assign58980_e95919_d_n5, assign58980_e95919_d_n6, assign58980_e95919_d_n7, assign58980_e95919_d_n8, assign58980_e95919_d_n9, assign58980_e95919_d_n10, assign58980_e95919_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58980_e95914: f64 = (locals.var_cdbox + locals.var_cdmin);
        let assign58980_e95915: f64 = (0.5 * assign58980_e95914);
        let assign58980_e95917: f64 = (assign58980_e95915 * locals.var_t11);
        (assign58980_e95917, (assign58980_e95915 * locals.var_t11_dn3), (assign58980_e95915 * locals.var_t11_dn4), (assign58980_e95915 * locals.var_t11_dn5), (assign58980_e95915 * locals.var_t11_dn6), (assign58980_e95915 * locals.var_t11_dn7), (assign58980_e95915 * locals.var_t11_dn8), (assign58980_e95915 * locals.var_t11_dn9), (assign58980_e95915 * locals.var_t11_dn10), (assign58980_e95915 * locals.var_t11_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign58980_e95919;
        locals.var_t3_dn3 = assign58980_e95919_d_n3;
        locals.var_t3_dn4 = assign58980_e95919_d_n4;
        locals.var_t3_dn5 = assign58980_e95919_d_n5;
        locals.var_t3_dn6 = assign58980_e95919_d_n6;
        locals.var_t3_dn7 = assign58980_e95919_d_n7;
        locals.var_t3_dn8 = assign58980_e95919_d_n8;
        locals.var_t3_dn9 = assign58980_e95919_d_n9;
        locals.var_t3_dn10 = assign58980_e95919_d_n10;
        locals.var_t3_dn11 = assign58980_e95919_d_n11;

        let (assign58990_e95930, assign58990_e95930_d_n3, assign58990_e95930_d_n4, assign58990_e95930_d_n5, assign58990_e95930_d_n6, assign58990_e95930_d_n7, assign58990_e95930_d_n8, assign58990_e95930_d_n9, assign58990_e95930_d_n10, assign58990_e95930_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58990_e95926: f64 = (locals.var_t1 * locals.var_t2);
        let assign58990_e95928: f64 = (assign58990_e95926 + locals.var_t3);
        (assign58990_e95928, (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) + locals.var_t3_dn3), (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) + locals.var_t3_dn4), (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) + locals.var_t3_dn5), (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) + locals.var_t3_dn6), (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) + locals.var_t3_dn7), (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) + locals.var_t3_dn8), (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) + locals.var_t3_dn9), (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) + locals.var_t3_dn10), (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) + locals.var_t3_dn11),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign58990_e95930;
        locals.var_qde_dn3 = assign58990_e95930_d_n3;
        locals.var_qde_dn4 = assign58990_e95930_d_n4;
        locals.var_qde_dn5 = assign58990_e95930_d_n5;
        locals.var_qde_dn6 = assign58990_e95930_d_n6;
        locals.var_qde_dn7 = assign58990_e95930_d_n7;
        locals.var_qde_dn8 = assign58990_e95930_d_n8;
        locals.var_qde_dn9 = assign58990_e95930_d_n9;
        locals.var_qde_dn10 = assign58990_e95930_d_n10;
        locals.var_qde_dn11 = assign58990_e95930_d_n11;

        let (assign59000_e95944, assign59000_e95944_d_n3, assign59000_e95944_d_n4, assign59000_e95944_d_n5, assign59000_e95944_d_n6, assign59000_e95944_d_n7, assign59000_e95944_d_n8, assign59000_e95944_d_n9, assign59000_e95944_d_n10, assign59000_e95944_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59000_e95936: f64 = (-0.5);
        let assign59000_e95939: f64 = (locals.var_csbox - locals.var_csmin);
        let assign59000_e95940: f64 = (assign59000_e95936 * assign59000_e95939);
        let assign59000_e95942: f64 = (assign59000_e95940 / p.p1397);
        (assign59000_e95942, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59000_e95944;
        locals.var_t1_dn3 = assign59000_e95944_d_n3;
        locals.var_t1_dn4 = assign59000_e95944_d_n4;
        locals.var_t1_dn5 = assign59000_e95944_d_n5;
        locals.var_t1_dn6 = assign59000_e95944_d_n6;
        locals.var_t1_dn7 = assign59000_e95944_d_n7;
        locals.var_t1_dn8 = assign59000_e95944_d_n8;
        locals.var_t1_dn9 = assign59000_e95944_d_n9;
        locals.var_t1_dn10 = assign59000_e95944_d_n10;
        locals.var_t1_dn11 = assign59000_e95944_d_n11;

        let (assign59010_e95960, assign59010_e95960_d_n3, assign59010_e95960_d_n4, assign59010_e95960_d_n5, assign59010_e95960_d_n6, assign59010_e95960_d_n7, assign59010_e95960_d_n8, assign59010_e95960_d_n9, assign59010_e95960_d_n10, assign59010_e95960_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59010_e95950: f64 = (-p.p1397);
        let assign59010_e95952: f64 = (assign59010_e95950 * locals.var_t10);
        let assign59010_e95954: f64 = (assign59010_e95952 + p.p1398);
        let assign59010_e95955: f64 = (assign59010_e95954).cosh();
        let assign59010_e95957: f64 = (assign59010_e95955).max(1e-38);
        let assign59010_e95958: f64 = (assign59010_e95957).ln();
        (assign59010_e95958, (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn3)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn4)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn5)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn6)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn7)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn8)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn9)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn10)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn11)) } else { 0.0 } / assign59010_e95957),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59010_e95960;
        locals.var_t2_dn3 = assign59010_e95960_d_n3;
        locals.var_t2_dn4 = assign59010_e95960_d_n4;
        locals.var_t2_dn5 = assign59010_e95960_d_n5;
        locals.var_t2_dn6 = assign59010_e95960_d_n6;
        locals.var_t2_dn7 = assign59010_e95960_d_n7;
        locals.var_t2_dn8 = assign59010_e95960_d_n8;
        locals.var_t2_dn9 = assign59010_e95960_d_n9;
        locals.var_t2_dn10 = assign59010_e95960_d_n10;
        locals.var_t2_dn11 = assign59010_e95960_d_n11;

        let (assign59020_e95973, assign59020_e95973_d_n3, assign59020_e95973_d_n4, assign59020_e95973_d_n5, assign59020_e95973_d_n6, assign59020_e95973_d_n7, assign59020_e95973_d_n8, assign59020_e95973_d_n9, assign59020_e95973_d_n10, assign59020_e95973_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59020_e95968: f64 = (locals.var_csbox + locals.var_csmin);
        let assign59020_e95969: f64 = (0.5 * assign59020_e95968);
        let assign59020_e95971: f64 = (assign59020_e95969 * locals.var_t10);
        (assign59020_e95971, (assign59020_e95969 * locals.var_t10_dn3), (assign59020_e95969 * locals.var_t10_dn4), (assign59020_e95969 * locals.var_t10_dn5), (assign59020_e95969 * locals.var_t10_dn6), (assign59020_e95969 * locals.var_t10_dn7), (assign59020_e95969 * locals.var_t10_dn8), (assign59020_e95969 * locals.var_t10_dn9), (assign59020_e95969 * locals.var_t10_dn10), (assign59020_e95969 * locals.var_t10_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59020_e95973;
        locals.var_t3_dn3 = assign59020_e95973_d_n3;
        locals.var_t3_dn4 = assign59020_e95973_d_n4;
        locals.var_t3_dn5 = assign59020_e95973_d_n5;
        locals.var_t3_dn6 = assign59020_e95973_d_n6;
        locals.var_t3_dn7 = assign59020_e95973_d_n7;
        locals.var_t3_dn8 = assign59020_e95973_d_n8;
        locals.var_t3_dn9 = assign59020_e95973_d_n9;
        locals.var_t3_dn10 = assign59020_e95973_d_n10;
        locals.var_t3_dn11 = assign59020_e95973_d_n11;

        let (assign59030_e95984, assign59030_e95984_d_n3, assign59030_e95984_d_n4, assign59030_e95984_d_n5, assign59030_e95984_d_n6, assign59030_e95984_d_n7, assign59030_e95984_d_n8, assign59030_e95984_d_n9, assign59030_e95984_d_n10, assign59030_e95984_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59030_e95980: f64 = (locals.var_t1 * locals.var_t2);
        let assign59030_e95982: f64 = (assign59030_e95980 + locals.var_t3);
        (assign59030_e95982, (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) + locals.var_t3_dn3), (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) + locals.var_t3_dn4), (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) + locals.var_t3_dn5), (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) + locals.var_t3_dn6), (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) + locals.var_t3_dn7), (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) + locals.var_t3_dn8), (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) + locals.var_t3_dn9), (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) + locals.var_t3_dn10), (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) + locals.var_t3_dn11),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign59030_e95984;
        locals.var_qse_dn3 = assign59030_e95984_d_n3;
        locals.var_qse_dn4 = assign59030_e95984_d_n4;
        locals.var_qse_dn5 = assign59030_e95984_d_n5;
        locals.var_qse_dn6 = assign59030_e95984_d_n6;
        locals.var_qse_dn7 = assign59030_e95984_d_n7;
        locals.var_qse_dn8 = assign59030_e95984_d_n8;
        locals.var_qse_dn9 = assign59030_e95984_d_n9;
        locals.var_qse_dn10 = assign59030_e95984_d_n10;
        locals.var_qse_dn11 = assign59030_e95984_d_n11;

        let (assign59040_e95994, assign59040_e95994_d_n3, assign59040_e95994_d_n4, assign59040_e95994_d_n5, assign59040_e95994_d_n6, assign59040_e95994_d_n7, assign59040_e95994_d_n8, assign59040_e95994_d_n9, assign59040_e95994_d_n10, assign59040_e95994_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 == 0.0)) {
        let assign59040_e95992: f64 = (locals.var_csbox * locals.var_t10);
        (assign59040_e95992, (locals.var_csbox * locals.var_t10_dn3), (locals.var_csbox * locals.var_t10_dn4), (locals.var_csbox * locals.var_t10_dn5), (locals.var_csbox * locals.var_t10_dn6), (locals.var_csbox * locals.var_t10_dn7), (locals.var_csbox * locals.var_t10_dn8), (locals.var_csbox * locals.var_t10_dn9), (locals.var_csbox * locals.var_t10_dn10), (locals.var_csbox * locals.var_t10_dn11),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign59040_e95994;
        locals.var_qse_dn3 = assign59040_e95994_d_n3;
        locals.var_qse_dn4 = assign59040_e95994_d_n4;
        locals.var_qse_dn5 = assign59040_e95994_d_n5;
        locals.var_qse_dn6 = assign59040_e95994_d_n6;
        locals.var_qse_dn7 = assign59040_e95994_d_n7;
        locals.var_qse_dn8 = assign59040_e95994_d_n8;
        locals.var_qse_dn9 = assign59040_e95994_d_n9;
        locals.var_qse_dn10 = assign59040_e95994_d_n10;
        locals.var_qse_dn11 = assign59040_e95994_d_n11;

        let (assign59050_e96004, assign59050_e96004_d_n3, assign59050_e96004_d_n4, assign59050_e96004_d_n5, assign59050_e96004_d_n6, assign59050_e96004_d_n7, assign59050_e96004_d_n8, assign59050_e96004_d_n9, assign59050_e96004_d_n10, assign59050_e96004_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 == 0.0)) {
        let assign59050_e96002: f64 = (locals.var_cdbox * locals.var_t11);
        (assign59050_e96002, (locals.var_cdbox * locals.var_t11_dn3), (locals.var_cdbox * locals.var_t11_dn4), (locals.var_cdbox * locals.var_t11_dn5), (locals.var_cdbox * locals.var_t11_dn6), (locals.var_cdbox * locals.var_t11_dn7), (locals.var_cdbox * locals.var_t11_dn8), (locals.var_cdbox * locals.var_t11_dn9), (locals.var_cdbox * locals.var_t11_dn10), (locals.var_cdbox * locals.var_t11_dn11),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign59050_e96004;
        locals.var_qde_dn3 = assign59050_e96004_d_n3;
        locals.var_qde_dn4 = assign59050_e96004_d_n4;
        locals.var_qde_dn5 = assign59050_e96004_d_n5;
        locals.var_qde_dn6 = assign59050_e96004_d_n6;
        locals.var_qde_dn7 = assign59050_e96004_d_n7;
        locals.var_qde_dn8 = assign59050_e96004_d_n8;
        locals.var_qde_dn9 = assign59050_e96004_d_n9;
        locals.var_qde_dn10 = assign59050_e96004_d_n10;
        locals.var_qde_dn11 = assign59050_e96004_d_n11;

        let (assign59060_e96013, assign59060_e96013_d_n3, assign59060_e96013_d_n4, assign59060_e96013_d_n5, assign59060_e96013_d_n6, assign59060_e96013_d_n7, assign59060_e96013_d_n8, assign59060_e96013_d_n9, assign59060_e96013_d_n10, assign59060_e96013_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign59060_e96010: f64 = (locals.var_csesw * locals.var_t10);
        let assign59060_e96011: f64 = (locals.var_qse + assign59060_e96010);
        (assign59060_e96011, (locals.var_qse_dn3 + ((locals.var_csesw_dn3 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn3))), (locals.var_qse_dn4 + ((locals.var_csesw_dn4 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn4))), (locals.var_qse_dn5 + ((locals.var_csesw_dn5 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn5))), (locals.var_qse_dn6 + ((locals.var_csesw_dn6 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn6))), (locals.var_qse_dn7 + ((locals.var_csesw_dn7 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn7))), (locals.var_qse_dn8 + ((locals.var_csesw_dn8 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn8))), (locals.var_qse_dn9 + ((locals.var_csesw_dn9 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn9))), (locals.var_qse_dn10 + ((locals.var_csesw_dn10 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn10))), (locals.var_qse_dn11 + ((locals.var_csesw_dn11 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn11))),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign59060_e96013;
        locals.var_qse_dn3 = assign59060_e96013_d_n3;
        locals.var_qse_dn4 = assign59060_e96013_d_n4;
        locals.var_qse_dn5 = assign59060_e96013_d_n5;
        locals.var_qse_dn6 = assign59060_e96013_d_n6;
        locals.var_qse_dn7 = assign59060_e96013_d_n7;
        locals.var_qse_dn8 = assign59060_e96013_d_n8;
        locals.var_qse_dn9 = assign59060_e96013_d_n9;
        locals.var_qse_dn10 = assign59060_e96013_d_n10;
        locals.var_qse_dn11 = assign59060_e96013_d_n11;

        let (assign59070_e96022, assign59070_e96022_d_n3, assign59070_e96022_d_n4, assign59070_e96022_d_n5, assign59070_e96022_d_n6, assign59070_e96022_d_n7, assign59070_e96022_d_n8, assign59070_e96022_d_n9, assign59070_e96022_d_n10, assign59070_e96022_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign59070_e96019: f64 = (locals.var_cdesw * locals.var_t11);
        let assign59070_e96020: f64 = (locals.var_qde + assign59070_e96019);
        (assign59070_e96020, (locals.var_qde_dn3 + ((locals.var_cdesw_dn3 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn3))), (locals.var_qde_dn4 + ((locals.var_cdesw_dn4 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn4))), (locals.var_qde_dn5 + ((locals.var_cdesw_dn5 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn5))), (locals.var_qde_dn6 + ((locals.var_cdesw_dn6 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn6))), (locals.var_qde_dn7 + ((locals.var_cdesw_dn7 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn7))), (locals.var_qde_dn8 + ((locals.var_cdesw_dn8 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn8))), (locals.var_qde_dn9 + ((locals.var_cdesw_dn9 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn9))), (locals.var_qde_dn10 + ((locals.var_cdesw_dn10 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn10))), (locals.var_qde_dn11 + ((locals.var_cdesw_dn11 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn11))),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign59070_e96022;
        locals.var_qde_dn3 = assign59070_e96022_d_n3;
        locals.var_qde_dn4 = assign59070_e96022_d_n4;
        locals.var_qde_dn5 = assign59070_e96022_d_n5;
        locals.var_qde_dn6 = assign59070_e96022_d_n6;
        locals.var_qde_dn7 = assign59070_e96022_d_n7;
        locals.var_qde_dn8 = assign59070_e96022_d_n8;
        locals.var_qde_dn9 = assign59070_e96022_d_n9;
        locals.var_qde_dn10 = assign59070_e96022_d_n10;
        locals.var_qde_dn11 = assign59070_e96022_d_n11;

        let assign59080_e96025: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard867 = assign59080_e96025;

    }

    pub(super) fn stamp_transient_block_195(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign59090_e96037, assign59090_e96037_d_n3, assign59090_e96037_d_n4, assign59090_e96037_d_n5, assign59090_e96037_d_n6, assign59090_e96037_d_n7, assign59090_e96037_d_n8, assign59090_e96037_d_n9, assign59090_e96037_d_n10, assign59090_e96037_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59090_e96032: f64 = (locals.var_ndepedge_i / locals.var_ni);
        let assign59090_e96034: f64 = (assign59090_e96032).max(1e-38);
        let assign59090_e96035: f64 = (assign59090_e96034).ln();
        (assign59090_e96035, (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034),)
    } else {
        (locals.var_phib_edge, locals.var_phib_edge_dn3, locals.var_phib_edge_dn4, locals.var_phib_edge_dn5, locals.var_phib_edge_dn6, locals.var_phib_edge_dn7, locals.var_phib_edge_dn8, locals.var_phib_edge_dn9, locals.var_phib_edge_dn10, locals.var_phib_edge_dn11,)
    }
};
        locals.var_phib_edge = assign59090_e96037;
        locals.var_phib_edge_dn3 = assign59090_e96037_d_n3;
        locals.var_phib_edge_dn4 = assign59090_e96037_d_n4;
        locals.var_phib_edge_dn5 = assign59090_e96037_d_n5;
        locals.var_phib_edge_dn6 = assign59090_e96037_d_n6;
        locals.var_phib_edge_dn7 = assign59090_e96037_d_n7;
        locals.var_phib_edge_dn8 = assign59090_e96037_d_n8;
        locals.var_phib_edge_dn9 = assign59090_e96037_d_n9;
        locals.var_phib_edge_dn10 = assign59090_e96037_d_n10;
        locals.var_phib_edge_dn11 = assign59090_e96037_d_n11;

        let (assign59100_e96052, assign59100_e96052_d_n3, assign59100_e96052_d_n4, assign59100_e96052_d_n5, assign59100_e96052_d_n6, assign59100_e96052_d_n7, assign59100_e96052_d_n8, assign59100_e96052_d_n9, assign59100_e96052_d_n10, assign59100_e96052_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59100_e96045: f64 = (locals.var_vt * locals.var_phib_edge);
        let assign59100_e96046: f64 = (0.4 + assign59100_e96045);
        let assign59100_e96048: f64 = (assign59100_e96046 + locals.var_phin_i);
        let assign59100_e96050: f64 = (assign59100_e96048).max(0.4);
        (assign59100_e96050, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn3) } else { 0.0 }, if assign59100_e96048 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib_edge) + (locals.var_vt * locals.var_phib_edge_dn4)) } else { 0.0 }, if assign59100_e96048 >= 0.4 { ((locals.var_vt_dn5 * locals.var_phib_edge) + (locals.var_vt * locals.var_phib_edge_dn5)) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn6) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn7) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn8) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn9) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn10) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn11) } else { 0.0 },)
    } else {
        (locals.var_phist, locals.var_phist_dn3, locals.var_phist_dn4, locals.var_phist_dn5, locals.var_phist_dn6, locals.var_phist_dn7, locals.var_phist_dn8, locals.var_phist_dn9, locals.var_phist_dn10, locals.var_phist_dn11,)
    }
};
        locals.var_phist = assign59100_e96052;
        locals.var_phist_dn3 = assign59100_e96052_d_n3;
        locals.var_phist_dn4 = assign59100_e96052_d_n4;
        locals.var_phist_dn5 = assign59100_e96052_d_n5;
        locals.var_phist_dn6 = assign59100_e96052_d_n6;
        locals.var_phist_dn7 = assign59100_e96052_d_n7;
        locals.var_phist_dn8 = assign59100_e96052_d_n8;
        locals.var_phist_dn9 = assign59100_e96052_d_n9;
        locals.var_phist_dn10 = assign59100_e96052_d_n10;
        locals.var_phist_dn11 = assign59100_e96052_d_n11;

        let (assign59110_e96066, assign59110_e96066_d_n3, assign59110_e96066_d_n4, assign59110_e96066_d_n5, assign59110_e96066_d_n6, assign59110_e96066_d_n7, assign59110_e96066_d_n8, assign59110_e96066_d_n9, assign59110_e96066_d_n10, assign59110_e96066_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59110_e96059: f64 = (2.0 * locals.var_epssi);
        let assign59110_e96062: f64 = (1.602176462e-19 * locals.var_ndepedge_i);
        let assign59110_e96063: f64 = (assign59110_e96059 / assign59110_e96062);
        let assign59110_e96064: f64 = (assign59110_e96063).sqrt();
        (assign59110_e96064, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1dep, locals.var_t1dep_dn3, locals.var_t1dep_dn4, locals.var_t1dep_dn5, locals.var_t1dep_dn6, locals.var_t1dep_dn7, locals.var_t1dep_dn8, locals.var_t1dep_dn9, locals.var_t1dep_dn10, locals.var_t1dep_dn11,)
    }
};
        locals.var_t1dep = assign59110_e96066;
        locals.var_t1dep_dn3 = assign59110_e96066_d_n3;
        locals.var_t1dep_dn4 = assign59110_e96066_d_n4;
        locals.var_t1dep_dn5 = assign59110_e96066_d_n5;
        locals.var_t1dep_dn6 = assign59110_e96066_d_n6;
        locals.var_t1dep_dn7 = assign59110_e96066_d_n7;
        locals.var_t1dep_dn8 = assign59110_e96066_d_n8;
        locals.var_t1dep_dn9 = assign59110_e96066_d_n9;
        locals.var_t1dep_dn10 = assign59110_e96066_d_n10;
        locals.var_t1dep_dn11 = assign59110_e96066_d_n11;

        let (assign59120_e96106, assign59120_e96106_d_n4, assign59120_e96106_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59120_e96077: f64 = (locals.var_tratio - 1.0);
        let assign59120_e96078: f64 = (locals.var_tnfactoredge_i * assign59120_e96077);
        let assign59120_e96079: f64 = (1.0 + assign59120_e96078);
        let assign59120_e96084: f64 = (locals.var_tratio - 1.0);
        let assign59120_e96085: f64 = (locals.var_tnfactoredge_i * assign59120_e96084);
        let assign59120_e96086: f64 = (1.0 + assign59120_e96085);
        let assign59120_e96091: f64 = (locals.var_tratio - 1.0);
        let assign59120_e96092: f64 = (locals.var_tnfactoredge_i * assign59120_e96091);
        let assign59120_e96093: f64 = (1.0 + assign59120_e96092);
        let assign59120_e96094: f64 = (assign59120_e96086 * assign59120_e96093);
        let assign59120_e96097: f64 = (4.0 * 0.001);
        let assign59120_e96099: f64 = (assign59120_e96097 * 0.001);
        let assign59120_e96100: f64 = (assign59120_e96094 + assign59120_e96099);
        let assign59120_e96101: f64 = (assign59120_e96100).sqrt();
        let assign59120_e96102: f64 = (assign59120_e96079 + assign59120_e96101);
        let assign59120_e96103: f64 = (0.5 * assign59120_e96102);
        let assign59120_e96104: f64 = (locals.var_nfactoredge_i * assign59120_e96103);
        (assign59120_e96104, (locals.var_nfactoredge_i * (0.5 * ((locals.var_tnfactoredge_i * locals.var_tratio_dn4) + ((((locals.var_tnfactoredge_i * locals.var_tratio_dn4) * assign59120_e96093) + (assign59120_e96086 * (locals.var_tnfactoredge_i * locals.var_tratio_dn4))) / (2.0 * assign59120_e96101))))), (locals.var_nfactoredge_i * (0.5 * ((locals.var_tnfactoredge_i * locals.var_tratio_dn5) + ((((locals.var_tnfactoredge_i * locals.var_tratio_dn5) * assign59120_e96093) + (assign59120_e96086 * (locals.var_tnfactoredge_i * locals.var_tratio_dn5))) / (2.0 * assign59120_e96101))))),)
    } else {
        (locals.var_nfactoredge_t, locals.var_nfactoredge_t_dn4, locals.var_nfactoredge_t_dn5,)
    }
};
        locals.var_nfactoredge_t = assign59120_e96106;
        locals.var_nfactoredge_t_dn4 = assign59120_e96106_d_n4;
        locals.var_nfactoredge_t_dn5 = assign59120_e96106_d_n5;

        let (assign59130_e96121, assign59130_e96121_d_n3, assign59130_e96121_d_n4, assign59130_e96121_d_n5, assign59130_e96121_d_n6, assign59130_e96121_d_n7, assign59130_e96121_d_n8, assign59130_e96121_d_n9, assign59130_e96121_d_n10, assign59130_e96121_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59130_e96116: f64 = (locals.var_tratio - 1.0);
        let assign59130_e96117: f64 = (locals.var_teta0edge_i * assign59130_e96116);
        let assign59130_e96118: f64 = (1.0 + assign59130_e96117);
        let assign59130_e96119: f64 = (locals.var_eta0edge_i * assign59130_e96118);
        (assign59130_e96119, (locals.var_eta0edge_i_dn3 * assign59130_e96118), ((locals.var_eta0edge_i_dn4 * assign59130_e96118) + (locals.var_eta0edge_i * (locals.var_teta0edge_i * locals.var_tratio_dn4))), ((locals.var_eta0edge_i_dn5 * assign59130_e96118) + (locals.var_eta0edge_i * (locals.var_teta0edge_i * locals.var_tratio_dn5))), (locals.var_eta0edge_i_dn6 * assign59130_e96118), (locals.var_eta0edge_i_dn7 * assign59130_e96118), (locals.var_eta0edge_i_dn8 * assign59130_e96118), (locals.var_eta0edge_i_dn9 * assign59130_e96118), (locals.var_eta0edge_i_dn10 * assign59130_e96118), (locals.var_eta0edge_i_dn11 * assign59130_e96118),)
    } else {
        (locals.var_eta0edge_t, locals.var_eta0edge_t_dn3, locals.var_eta0edge_t_dn4, locals.var_eta0edge_t_dn5, locals.var_eta0edge_t_dn6, locals.var_eta0edge_t_dn7, locals.var_eta0edge_t_dn8, locals.var_eta0edge_t_dn9, locals.var_eta0edge_t_dn10, locals.var_eta0edge_t_dn11,)
    }
};
        locals.var_eta0edge_t = assign59130_e96121;
        locals.var_eta0edge_t_dn3 = assign59130_e96121_d_n3;
        locals.var_eta0edge_t_dn4 = assign59130_e96121_d_n4;
        locals.var_eta0edge_t_dn5 = assign59130_e96121_d_n5;
        locals.var_eta0edge_t_dn6 = assign59130_e96121_d_n6;
        locals.var_eta0edge_t_dn7 = assign59130_e96121_d_n7;
        locals.var_eta0edge_t_dn8 = assign59130_e96121_d_n8;
        locals.var_eta0edge_t_dn9 = assign59130_e96121_d_n9;
        locals.var_eta0edge_t_dn10 = assign59130_e96121_d_n10;
        locals.var_eta0edge_t_dn11 = assign59130_e96121_d_n11;

        let (assign59140_e96153, assign59140_e96153_d_n3, assign59140_e96153_d_n4, assign59140_e96153_d_n5, assign59140_e96153_d_n6, assign59140_e96153_d_n7, assign59140_e96153_d_n8, assign59140_e96153_d_n9, assign59140_e96153_d_n10, assign59140_e96153_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59140_e96129: f64 = (locals.var_phist - locals.var_vbsx);
        let assign59140_e96131: f64 = (assign59140_e96129 + 0.05);
        let assign59140_e96134: f64 = (locals.var_phist - locals.var_vbsx);
        let assign59140_e96136: f64 = (assign59140_e96134 - 0.05);
        let assign59140_e96139: f64 = (locals.var_phist - locals.var_vbsx);
        let assign59140_e96141: f64 = (assign59140_e96139 - 0.05);
        let assign59140_e96142: f64 = (assign59140_e96136 * assign59140_e96141);
        let assign59140_e96145: f64 = (0.25 * 0.1);
        let assign59140_e96147: f64 = (assign59140_e96145 * 0.1);
        let assign59140_e96148: f64 = (assign59140_e96142 + assign59140_e96147);
        let assign59140_e96149: f64 = (assign59140_e96148).sqrt();
        let assign59140_e96150: f64 = (assign59140_e96131 + assign59140_e96149);
        let assign59140_e96151: f64 = (0.5 * assign59140_e96150);
        (assign59140_e96151, (0.5 * ((locals.var_phist_dn3 - locals.var_vbsx_dn3) + ((((locals.var_phist_dn3 - locals.var_vbsx_dn3) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn4 - locals.var_vbsx_dn4) + ((((locals.var_phist_dn4 - locals.var_vbsx_dn4) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn5 - locals.var_vbsx_dn5) + ((((locals.var_phist_dn5 - locals.var_vbsx_dn5) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn6 - locals.var_vbsx_dn6) + ((((locals.var_phist_dn6 - locals.var_vbsx_dn6) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn7 - locals.var_vbsx_dn7) + ((((locals.var_phist_dn7 - locals.var_vbsx_dn7) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn8 - locals.var_vbsx_dn8) + ((((locals.var_phist_dn8 - locals.var_vbsx_dn8) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn9 - locals.var_vbsx_dn9) + ((((locals.var_phist_dn9 - locals.var_vbsx_dn9) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn10 - locals.var_vbsx_dn10) + ((((locals.var_phist_dn10 - locals.var_vbsx_dn10) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn11 - locals.var_vbsx_dn11) + ((((locals.var_phist_dn11 - locals.var_vbsx_dn11) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (2.0 * assign59140_e96149)))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11,)
    }
};
        locals.var_phistvbs = assign59140_e96153;
        locals.var_phistvbs_dn3 = assign59140_e96153_d_n3;
        locals.var_phistvbs_dn4 = assign59140_e96153_d_n4;
        locals.var_phistvbs_dn5 = assign59140_e96153_d_n5;
        locals.var_phistvbs_dn6 = assign59140_e96153_d_n6;
        locals.var_phistvbs_dn7 = assign59140_e96153_d_n7;
        locals.var_phistvbs_dn8 = assign59140_e96153_d_n8;
        locals.var_phistvbs_dn9 = assign59140_e96153_d_n9;
        locals.var_phistvbs_dn10 = assign59140_e96153_d_n10;
        locals.var_phistvbs_dn11 = assign59140_e96153_d_n11;

        let (assign59150_e96161, assign59150_e96161_d_n3, assign59150_e96161_d_n4, assign59150_e96161_d_n5, assign59150_e96161_d_n6, assign59150_e96161_d_n7, assign59150_e96161_d_n8, assign59150_e96161_d_n9, assign59150_e96161_d_n10, assign59150_e96161_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59150_e96159: f64 = (locals.var_phistvbs).sqrt();
        (assign59150_e96159, (locals.var_phistvbs_dn3 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn4 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn5 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn6 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn7 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn8 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn9 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn10 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn11 / (2.0 * assign59150_e96159)),)
    } else {
        (locals.var_sqrtphistvbs, locals.var_sqrtphistvbs_dn3, locals.var_sqrtphistvbs_dn4, locals.var_sqrtphistvbs_dn5, locals.var_sqrtphistvbs_dn6, locals.var_sqrtphistvbs_dn7, locals.var_sqrtphistvbs_dn8, locals.var_sqrtphistvbs_dn9, locals.var_sqrtphistvbs_dn10, locals.var_sqrtphistvbs_dn11,)
    }
};
        locals.var_sqrtphistvbs = assign59150_e96161;
        locals.var_sqrtphistvbs_dn3 = assign59150_e96161_d_n3;
        locals.var_sqrtphistvbs_dn4 = assign59150_e96161_d_n4;
        locals.var_sqrtphistvbs_dn5 = assign59150_e96161_d_n5;
        locals.var_sqrtphistvbs_dn6 = assign59150_e96161_d_n6;
        locals.var_sqrtphistvbs_dn7 = assign59150_e96161_d_n7;
        locals.var_sqrtphistvbs_dn8 = assign59150_e96161_d_n8;
        locals.var_sqrtphistvbs_dn9 = assign59150_e96161_d_n9;
        locals.var_sqrtphistvbs_dn10 = assign59150_e96161_d_n10;
        locals.var_sqrtphistvbs_dn11 = assign59150_e96161_d_n11;

        let (assign59160_e96170, assign59160_e96170_d_n3, assign59160_e96170_d_n4, assign59160_e96170_d_n5, assign59160_e96170_d_n6, assign59160_e96170_d_n7, assign59160_e96170_d_n8, assign59160_e96170_d_n9, assign59160_e96170_d_n10, assign59160_e96170_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59160_e96168: f64 = (locals.var_t1dep * locals.var_sqrtphistvbs);
        (assign59160_e96168, ((locals.var_t1dep_dn3 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn3)), ((locals.var_t1dep_dn4 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn4)), ((locals.var_t1dep_dn5 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn5)), ((locals.var_t1dep_dn6 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn6)), ((locals.var_t1dep_dn7 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn7)), ((locals.var_t1dep_dn8 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn8)), ((locals.var_t1dep_dn9 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn9)), ((locals.var_t1dep_dn10 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn10)), ((locals.var_t1dep_dn11 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn11)),)
    } else {
        (locals.var_xdep, locals.var_xdep_dn3, locals.var_xdep_dn4, locals.var_xdep_dn5, locals.var_xdep_dn6, locals.var_xdep_dn7, locals.var_xdep_dn8, locals.var_xdep_dn9, locals.var_xdep_dn10, locals.var_xdep_dn11,)
    }
};
        locals.var_xdep = assign59160_e96170;
        locals.var_xdep_dn3 = assign59160_e96170_d_n3;
        locals.var_xdep_dn4 = assign59160_e96170_d_n4;
        locals.var_xdep_dn5 = assign59160_e96170_d_n5;
        locals.var_xdep_dn6 = assign59160_e96170_d_n6;
        locals.var_xdep_dn7 = assign59160_e96170_d_n7;
        locals.var_xdep_dn8 = assign59160_e96170_d_n8;
        locals.var_xdep_dn9 = assign59160_e96170_d_n9;
        locals.var_xdep_dn10 = assign59160_e96170_d_n10;
        locals.var_xdep_dn11 = assign59160_e96170_d_n11;

        let (assign59170_e96179, assign59170_e96179_d_n3, assign59170_e96179_d_n4, assign59170_e96179_d_n5, assign59170_e96179_d_n6, assign59170_e96179_d_n7, assign59170_e96179_d_n8, assign59170_e96179_d_n9, assign59170_e96179_d_n10, assign59170_e96179_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59170_e96177: f64 = (locals.var_epssi / locals.var_xdep);
        (assign59170_e96177, (-((locals.var_epssi * locals.var_xdep_dn3) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn4) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn5) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn6) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn7) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn8) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn9) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn10) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn11) / (locals.var_xdep * locals.var_xdep))),)
    } else {
        (locals.var_cdep, locals.var_cdep_dn3, locals.var_cdep_dn4, locals.var_cdep_dn5, locals.var_cdep_dn6, locals.var_cdep_dn7, locals.var_cdep_dn8, locals.var_cdep_dn9, locals.var_cdep_dn10, locals.var_cdep_dn11,)
    }
};
        locals.var_cdep = assign59170_e96179;
        locals.var_cdep_dn3 = assign59170_e96179_d_n3;
        locals.var_cdep_dn4 = assign59170_e96179_d_n4;
        locals.var_cdep_dn5 = assign59170_e96179_d_n5;
        locals.var_cdep_dn6 = assign59170_e96179_d_n6;
        locals.var_cdep_dn7 = assign59170_e96179_d_n7;
        locals.var_cdep_dn8 = assign59170_e96179_d_n8;
        locals.var_cdep_dn9 = assign59170_e96179_d_n9;
        locals.var_cdep_dn10 = assign59170_e96179_d_n10;
        locals.var_cdep_dn11 = assign59170_e96179_d_n11;

        let (assign59180_e96196, assign59180_e96196_d_n3, assign59180_e96196_d_n4, assign59180_e96196_d_n5, assign59180_e96196_d_n6, assign59180_e96196_d_n7, assign59180_e96196_d_n8, assign59180_e96196_d_n9, assign59180_e96196_d_n10, assign59180_e96196_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59180_e96186: f64 = (locals.var_citedge_i + locals.var_nfactoredge_t);
        let assign59180_e96189: f64 = (locals.var_cdscdedge_a * locals.var_vdsx);
        let assign59180_e96190: f64 = (assign59180_e96186 + assign59180_e96189);
        let assign59180_e96193: f64 = (locals.var_cdscbedge_i * locals.var_vbsx);
        let assign59180_e96194: f64 = (assign59180_e96190 - assign59180_e96193);
        (assign59180_e96194, (((locals.var_cdscdedge_a_dn3 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn3)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn3)), ((locals.var_nfactoredge_t_dn4 + ((locals.var_cdscdedge_a_dn4 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn4))) - (locals.var_cdscbedge_i * locals.var_vbsx_dn4)), ((locals.var_nfactoredge_t_dn5 + ((locals.var_cdscdedge_a_dn5 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn5))) - (locals.var_cdscbedge_i * locals.var_vbsx_dn5)), (((locals.var_cdscdedge_a_dn6 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn6)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn6)), (((locals.var_cdscdedge_a_dn7 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn7)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn7)), (((locals.var_cdscdedge_a_dn8 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn8)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn8)), (((locals.var_cdscdedge_a_dn9 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn9)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn9)), (((locals.var_cdscdedge_a_dn10 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn10)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn10)), (((locals.var_cdscdedge_a_dn11 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn11)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn11)),)
    } else {
        (locals.var_cdsc, locals.var_cdsc_dn3, locals.var_cdsc_dn4, locals.var_cdsc_dn5, locals.var_cdsc_dn6, locals.var_cdsc_dn7, locals.var_cdsc_dn8, locals.var_cdsc_dn9, locals.var_cdsc_dn10, locals.var_cdsc_dn11,)
    }
};
        locals.var_cdsc = assign59180_e96196;
        locals.var_cdsc_dn3 = assign59180_e96196_d_n3;
        locals.var_cdsc_dn4 = assign59180_e96196_d_n4;
        locals.var_cdsc_dn5 = assign59180_e96196_d_n5;
        locals.var_cdsc_dn6 = assign59180_e96196_d_n6;
        locals.var_cdsc_dn7 = assign59180_e96196_d_n7;
        locals.var_cdsc_dn8 = assign59180_e96196_d_n8;
        locals.var_cdsc_dn9 = assign59180_e96196_d_n9;
        locals.var_cdsc_dn10 = assign59180_e96196_d_n10;
        locals.var_cdsc_dn11 = assign59180_e96196_d_n11;

        let (assign59190_e96207, assign59190_e96207_d_n3, assign59190_e96207_d_n4, assign59190_e96207_d_n5, assign59190_e96207_d_n6, assign59190_e96207_d_n7, assign59190_e96207_d_n8, assign59190_e96207_d_n9, assign59190_e96207_d_n10, assign59190_e96207_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59190_e96204: f64 = (locals.var_cdsc / locals.var_cox);
        let assign59190_e96205: f64 = (1.0 + assign59190_e96204);
        (assign59190_e96205, (locals.var_cdsc_dn3 / locals.var_cox), (locals.var_cdsc_dn4 / locals.var_cox), (locals.var_cdsc_dn5 / locals.var_cox), (locals.var_cdsc_dn6 / locals.var_cox), (locals.var_cdsc_dn7 / locals.var_cox), (locals.var_cdsc_dn8 / locals.var_cox), (locals.var_cdsc_dn9 / locals.var_cox), (locals.var_cdsc_dn10 / locals.var_cox), (locals.var_cdsc_dn11 / locals.var_cox),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59190_e96207;
        locals.var_t1_dn3 = assign59190_e96207_d_n3;
        locals.var_t1_dn4 = assign59190_e96207_d_n4;
        locals.var_t1_dn5 = assign59190_e96207_d_n5;
        locals.var_t1_dn6 = assign59190_e96207_d_n6;
        locals.var_t1_dn7 = assign59190_e96207_d_n7;
        locals.var_t1_dn8 = assign59190_e96207_d_n8;
        locals.var_t1_dn9 = assign59190_e96207_d_n9;
        locals.var_t1_dn10 = assign59190_e96207_d_n10;
        locals.var_t1_dn11 = assign59190_e96207_d_n11;

        let (assign59200_e96233, assign59200_e96233_d_n3, assign59200_e96233_d_n4, assign59200_e96233_d_n5, assign59200_e96233_d_n6, assign59200_e96233_d_n7, assign59200_e96233_d_n8, assign59200_e96233_d_n9, assign59200_e96233_d_n10, assign59200_e96233_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59200_e96215: f64 = (locals.var_t1 + 1.0);
        let assign59200_e96218: f64 = (locals.var_t1 - 1.0);
        let assign59200_e96221: f64 = (locals.var_t1 - 1.0);
        let assign59200_e96222: f64 = (assign59200_e96218 * assign59200_e96221);
        let assign59200_e96225: f64 = (0.25 * 0.05);
        let assign59200_e96227: f64 = (assign59200_e96225 * 0.05);
        let assign59200_e96228: f64 = (assign59200_e96222 + assign59200_e96227);
        let assign59200_e96229: f64 = (assign59200_e96228).sqrt();
        let assign59200_e96230: f64 = (assign59200_e96215 + assign59200_e96229);
        let assign59200_e96231: f64 = (0.5 * assign59200_e96230);
        (assign59200_e96231, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn3)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn4)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn5)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn6)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn7)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn8)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn9)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn10)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn11)) / (2.0 * assign59200_e96229)))),)
    } else {
        (locals.var_n, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11,)
    }
};
        locals.var_n = assign59200_e96233;
        locals.var_n_dn3 = assign59200_e96233_d_n3;
        locals.var_n_dn4 = assign59200_e96233_d_n4;
        locals.var_n_dn5 = assign59200_e96233_d_n5;
        locals.var_n_dn6 = assign59200_e96233_d_n6;
        locals.var_n_dn7 = assign59200_e96233_d_n7;
        locals.var_n_dn8 = assign59200_e96233_d_n8;
        locals.var_n_dn9 = assign59200_e96233_d_n9;
        locals.var_n_dn10 = assign59200_e96233_d_n10;
        locals.var_n_dn11 = assign59200_e96233_d_n11;

        let (assign59210_e96242, assign59210_e96242_d_n3, assign59210_e96242_d_n4, assign59210_e96242_d_n5, assign59210_e96242_d_n6, assign59210_e96242_d_n7, assign59210_e96242_d_n8, assign59210_e96242_d_n9, assign59210_e96242_d_n10, assign59210_e96242_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59210_e96240: f64 = (locals.var_n * locals.var_vt);
        (assign59210_e96240, (locals.var_n_dn3 * locals.var_vt), ((locals.var_n_dn4 * locals.var_vt) + (locals.var_n * locals.var_vt_dn4)), ((locals.var_n_dn5 * locals.var_vt) + (locals.var_n * locals.var_vt_dn5)), (locals.var_n_dn6 * locals.var_vt), (locals.var_n_dn7 * locals.var_vt), (locals.var_n_dn8 * locals.var_vt), (locals.var_n_dn9 * locals.var_vt), (locals.var_n_dn10 * locals.var_vt), (locals.var_n_dn11 * locals.var_vt),)
    } else {
        (locals.var_nvt, locals.var_nvt_dn3, locals.var_nvt_dn4, locals.var_nvt_dn5, locals.var_nvt_dn6, locals.var_nvt_dn7, locals.var_nvt_dn8, locals.var_nvt_dn9, locals.var_nvt_dn10, locals.var_nvt_dn11,)
    }
};
        locals.var_nvt = assign59210_e96242;
        locals.var_nvt_dn3 = assign59210_e96242_d_n3;
        locals.var_nvt_dn4 = assign59210_e96242_d_n4;
        locals.var_nvt_dn5 = assign59210_e96242_d_n5;
        locals.var_nvt_dn6 = assign59210_e96242_d_n6;
        locals.var_nvt_dn7 = assign59210_e96242_d_n7;
        locals.var_nvt_dn8 = assign59210_e96242_d_n8;
        locals.var_nvt_dn9 = assign59210_e96242_d_n9;
        locals.var_nvt_dn10 = assign59210_e96242_d_n10;
        locals.var_nvt_dn11 = assign59210_e96242_d_n11;

        let (assign59220_e96251, assign59220_e96251_d_n3, assign59220_e96251_d_n4, assign59220_e96251_d_n5, assign59220_e96251_d_n6, assign59220_e96251_d_n7, assign59220_e96251_d_n8, assign59220_e96251_d_n9, assign59220_e96251_d_n10, assign59220_e96251_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59220_e96249: f64 = (1.0 / locals.var_nvt);
        (assign59220_e96249, (-(locals.var_nvt_dn3 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn4 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn5 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn6 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn7 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn8 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn9 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn10 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn11 / (locals.var_nvt * locals.var_nvt))),)
    } else {
        (locals.var_inv_nvt, locals.var_inv_nvt_dn3, locals.var_inv_nvt_dn4, locals.var_inv_nvt_dn5, locals.var_inv_nvt_dn6, locals.var_inv_nvt_dn7, locals.var_inv_nvt_dn8, locals.var_inv_nvt_dn9, locals.var_inv_nvt_dn10, locals.var_inv_nvt_dn11,)
    }
};
        locals.var_inv_nvt = assign59220_e96251;
        locals.var_inv_nvt_dn3 = assign59220_e96251_d_n3;
        locals.var_inv_nvt_dn4 = assign59220_e96251_d_n4;
        locals.var_inv_nvt_dn5 = assign59220_e96251_d_n5;
        locals.var_inv_nvt_dn6 = assign59220_e96251_d_n6;
        locals.var_inv_nvt_dn7 = assign59220_e96251_d_n7;
        locals.var_inv_nvt_dn8 = assign59220_e96251_d_n8;
        locals.var_inv_nvt_dn9 = assign59220_e96251_d_n9;
        locals.var_inv_nvt_dn10 = assign59220_e96251_d_n10;
        locals.var_inv_nvt_dn11 = assign59220_e96251_d_n11;

        let (assign59230_e96260, assign59230_e96260_d_n3, assign59230_e96260_d_n4, assign59230_e96260_d_n5, assign59230_e96260_d_n6, assign59230_e96260_d_n7, assign59230_e96260_d_n8, assign59230_e96260_d_n9, assign59230_e96260_d_n10, assign59230_e96260_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59230_e96258: f64 = (locals.var_vg * locals.var_inv_nvt);
        (assign59230_e96258, (locals.var_vg * locals.var_inv_nvt_dn3), (locals.var_vg * locals.var_inv_nvt_dn4), (locals.var_vg * locals.var_inv_nvt_dn5), (locals.var_vg * locals.var_inv_nvt_dn6), (locals.var_vg * locals.var_inv_nvt_dn7), ((locals.var_vg_dn8 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn8)), (locals.var_vg * locals.var_inv_nvt_dn9), ((locals.var_vg_dn10 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn10)), (locals.var_vg * locals.var_inv_nvt_dn11),)
    } else {
        (locals.var_vg_1, locals.var_vg_1_dn3, locals.var_vg_1_dn4, locals.var_vg_1_dn5, locals.var_vg_1_dn6, locals.var_vg_1_dn7, locals.var_vg_1_dn8, locals.var_vg_1_dn9, locals.var_vg_1_dn10, locals.var_vg_1_dn11,)
    }
};
        locals.var_vg_1 = assign59230_e96260;
        locals.var_vg_1_dn3 = assign59230_e96260_d_n3;
        locals.var_vg_1_dn4 = assign59230_e96260_d_n4;
        locals.var_vg_1_dn5 = assign59230_e96260_d_n5;
        locals.var_vg_1_dn6 = assign59230_e96260_d_n6;
        locals.var_vg_1_dn7 = assign59230_e96260_d_n7;
        locals.var_vg_1_dn8 = assign59230_e96260_d_n8;
        locals.var_vg_1_dn9 = assign59230_e96260_d_n9;
        locals.var_vg_1_dn10 = assign59230_e96260_d_n10;
        locals.var_vg_1_dn11 = assign59230_e96260_d_n11;

        let (assign59240_e96269, assign59240_e96269_d_n3, assign59240_e96269_d_n4, assign59240_e96269_d_n5, assign59240_e96269_d_n6, assign59240_e96269_d_n7, assign59240_e96269_d_n8, assign59240_e96269_d_n9, assign59240_e96269_d_n10, assign59240_e96269_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59240_e96267: f64 = (locals.var_vs * locals.var_inv_nvt);
        (assign59240_e96267, (locals.var_vs * locals.var_inv_nvt_dn3), (locals.var_vs * locals.var_inv_nvt_dn4), (locals.var_vs * locals.var_inv_nvt_dn5), ((locals.var_vs_dn6 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn6)), ((locals.var_vs_dn7 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn7)), (locals.var_vs * locals.var_inv_nvt_dn8), (locals.var_vs * locals.var_inv_nvt_dn9), ((locals.var_vs_dn10 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn10)), (locals.var_vs * locals.var_inv_nvt_dn11),)
    } else {
        (locals.var_vs_1, locals.var_vs_1_dn3, locals.var_vs_1_dn4, locals.var_vs_1_dn5, locals.var_vs_1_dn6, locals.var_vs_1_dn7, locals.var_vs_1_dn8, locals.var_vs_1_dn9, locals.var_vs_1_dn10, locals.var_vs_1_dn11,)
    }
};
        locals.var_vs_1 = assign59240_e96269;
        locals.var_vs_1_dn3 = assign59240_e96269_d_n3;
        locals.var_vs_1_dn4 = assign59240_e96269_d_n4;
        locals.var_vs_1_dn5 = assign59240_e96269_d_n5;
        locals.var_vs_1_dn6 = assign59240_e96269_d_n6;
        locals.var_vs_1_dn7 = assign59240_e96269_d_n7;
        locals.var_vs_1_dn8 = assign59240_e96269_d_n8;
        locals.var_vs_1_dn9 = assign59240_e96269_d_n9;
        locals.var_vs_1_dn10 = assign59240_e96269_d_n10;
        locals.var_vs_1_dn11 = assign59240_e96269_d_n11;

        let (assign59250_e96278, assign59250_e96278_d_n3, assign59250_e96278_d_n4, assign59250_e96278_d_n5, assign59250_e96278_d_n6, assign59250_e96278_d_n7, assign59250_e96278_d_n8, assign59250_e96278_d_n9, assign59250_e96278_d_n10, assign59250_e96278_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59250_e96276: f64 = (locals.var_vfb_i * locals.var_inv_nvt);
        (assign59250_e96276, ((locals.var_vfb_i_dn3 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn3)), ((locals.var_vfb_i_dn4 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn4)), ((locals.var_vfb_i_dn5 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn5)), ((locals.var_vfb_i_dn6 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn6)), ((locals.var_vfb_i_dn7 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn7)), ((locals.var_vfb_i_dn8 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn8)), ((locals.var_vfb_i_dn9 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn9)), ((locals.var_vfb_i_dn10 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn10)), ((locals.var_vfb_i_dn11 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn11)),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11,)
    }
};
        locals.var_vfb = assign59250_e96278;
        locals.var_vfb_dn3 = assign59250_e96278_d_n3;
        locals.var_vfb_dn4 = assign59250_e96278_d_n4;
        locals.var_vfb_dn5 = assign59250_e96278_d_n5;
        locals.var_vfb_dn6 = assign59250_e96278_d_n6;
        locals.var_vfb_dn7 = assign59250_e96278_d_n7;
        locals.var_vfb_dn8 = assign59250_e96278_d_n8;
        locals.var_vfb_dn9 = assign59250_e96278_d_n9;
        locals.var_vfb_dn10 = assign59250_e96278_d_n10;
        locals.var_vfb_dn11 = assign59250_e96278_d_n11;

        let (assign59260_e96292, assign59260_e96292_d_n3, assign59260_e96292_d_n4, assign59260_e96292_d_n5, assign59260_e96292_d_n6, assign59260_e96292_d_n7, assign59260_e96292_d_n8, assign59260_e96292_d_n9, assign59260_e96292_d_n10, assign59260_e96292_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59260_e96286: f64 = (locals.var_etabedge_i * locals.var_vbsx);
        let assign59260_e96287: f64 = (locals.var_eta0edge_t + assign59260_e96286);
        let assign59260_e96288: f64 = (-assign59260_e96287);
        let assign59260_e96290: f64 = (assign59260_e96288 * locals.var_vdsx);
        (assign59260_e96290, (((-(locals.var_eta0edge_t_dn3 + (locals.var_etabedge_i * locals.var_vbsx_dn3))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn3)), (((-(locals.var_eta0edge_t_dn4 + (locals.var_etabedge_i * locals.var_vbsx_dn4))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn4)), (((-(locals.var_eta0edge_t_dn5 + (locals.var_etabedge_i * locals.var_vbsx_dn5))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn5)), (((-(locals.var_eta0edge_t_dn6 + (locals.var_etabedge_i * locals.var_vbsx_dn6))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn6)), (((-(locals.var_eta0edge_t_dn7 + (locals.var_etabedge_i * locals.var_vbsx_dn7))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn7)), (((-(locals.var_eta0edge_t_dn8 + (locals.var_etabedge_i * locals.var_vbsx_dn8))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn8)), (((-(locals.var_eta0edge_t_dn9 + (locals.var_etabedge_i * locals.var_vbsx_dn9))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn9)), (((-(locals.var_eta0edge_t_dn10 + (locals.var_etabedge_i * locals.var_vbsx_dn10))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn10)), (((-(locals.var_eta0edge_t_dn11 + (locals.var_etabedge_i * locals.var_vbsx_dn11))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn11)),)
    } else {
        (locals.var_dvth_dibl_1, locals.var_dvth_dibl_1_dn3, locals.var_dvth_dibl_1_dn4, locals.var_dvth_dibl_1_dn5, locals.var_dvth_dibl_1_dn6, locals.var_dvth_dibl_1_dn7, locals.var_dvth_dibl_1_dn8, locals.var_dvth_dibl_1_dn9, locals.var_dvth_dibl_1_dn10, locals.var_dvth_dibl_1_dn11,)
    }
};
        locals.var_dvth_dibl_1 = assign59260_e96292;
        locals.var_dvth_dibl_1_dn3 = assign59260_e96292_d_n3;
        locals.var_dvth_dibl_1_dn4 = assign59260_e96292_d_n4;
        locals.var_dvth_dibl_1_dn5 = assign59260_e96292_d_n5;
        locals.var_dvth_dibl_1_dn6 = assign59260_e96292_d_n6;
        locals.var_dvth_dibl_1_dn7 = assign59260_e96292_d_n7;
        locals.var_dvth_dibl_1_dn8 = assign59260_e96292_d_n8;
        locals.var_dvth_dibl_1_dn9 = assign59260_e96292_d_n9;
        locals.var_dvth_dibl_1_dn10 = assign59260_e96292_d_n10;
        locals.var_dvth_dibl_1_dn11 = assign59260_e96292_d_n11;

        let (assign59270_e96313, assign59270_e96313_d_n3, assign59270_e96313_d_n4, assign59270_e96313_d_n5, assign59270_e96313_d_n6, assign59270_e96313_d_n7, assign59270_e96313_d_n8, assign59270_e96313_d_n9, assign59270_e96313_d_n10, assign59270_e96313_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59270_e96300: f64 = (locals.var_kt1ledge_i / locals.var_leff);
        let assign59270_e96301: f64 = (locals.var_kt1edge_i + assign59270_e96300);
        let assign59270_e96304: f64 = (locals.var_kt2edge_i * locals.var_vbsx);
        let assign59270_e96305: f64 = (assign59270_e96301 + assign59270_e96304);
        let assign59270_e96308: f64 = (locals.var_tratio).powf(locals.var_kt1expedge_i);
        let assign59270_e96310: f64 = (assign59270_e96308 - 1.0);
        let assign59270_e96311: f64 = (assign59270_e96305 * assign59270_e96310);
        (assign59270_e96311, ((locals.var_kt2edge_i * locals.var_vbsx_dn3) * assign59270_e96310), (((locals.var_kt2edge_i * locals.var_vbsx_dn4) * assign59270_e96310) + (assign59270_e96305 * if 0.0 == 0.0 && ((locals.var_kt1expedge_i) as f64).is_finite() && ((locals.var_kt1expedge_i) as f64).fract() == 0.0 { if locals.var_kt1expedge_i == 0.0 { 0.0 } else { (locals.var_kt1expedge_i * ((locals.var_tratio).powf(locals.var_kt1expedge_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign59270_e96308 * (locals.var_kt1expedge_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (((locals.var_kt2edge_i * locals.var_vbsx_dn5) * assign59270_e96310) + (assign59270_e96305 * if 0.0 == 0.0 && ((locals.var_kt1expedge_i) as f64).is_finite() && ((locals.var_kt1expedge_i) as f64).fract() == 0.0 { if locals.var_kt1expedge_i == 0.0 { 0.0 } else { (locals.var_kt1expedge_i * ((locals.var_tratio).powf(locals.var_kt1expedge_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign59270_e96308 * (locals.var_kt1expedge_i * (locals.var_tratio_dn5 / locals.var_tratio))) })), ((locals.var_kt2edge_i * locals.var_vbsx_dn6) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn7) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn8) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn9) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn10) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn11) * assign59270_e96310),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11,)
    }
};
        locals.var_dvth_temp = assign59270_e96313;
        locals.var_dvth_temp_dn3 = assign59270_e96313_d_n3;
        locals.var_dvth_temp_dn4 = assign59270_e96313_d_n4;
        locals.var_dvth_temp_dn5 = assign59270_e96313_d_n5;
        locals.var_dvth_temp_dn6 = assign59270_e96313_d_n6;
        locals.var_dvth_temp_dn7 = assign59270_e96313_d_n7;
        locals.var_dvth_temp_dn8 = assign59270_e96313_d_n8;
        locals.var_dvth_temp_dn9 = assign59270_e96313_d_n9;
        locals.var_dvth_temp_dn10 = assign59270_e96313_d_n10;
        locals.var_dvth_temp_dn11 = assign59270_e96313_d_n11;

        let (assign59280_e96326, assign59280_e96326_d_n3, assign59280_e96326_d_n4, assign59280_e96326_d_n5, assign59280_e96326_d_n6, assign59280_e96326_d_n7, assign59280_e96326_d_n8, assign59280_e96326_d_n9, assign59280_e96326_d_n10, assign59280_e96326_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59280_e96322: f64 = (p.p1264 * locals.var_vbsx);
        let assign59280_e96323: f64 = (1.0 + assign59280_e96322);
        let assign59280_e96324: f64 = (locals.var_litl * assign59280_e96323);
        (assign59280_e96324, (locals.var_litl * (p.p1264 * locals.var_vbsx_dn3)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn4)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn5)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn6)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn7)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn8)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn9)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn10)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn11)),)
    } else {
        (locals.var_litl_edge, locals.var_litl_edge_dn3, locals.var_litl_edge_dn4, locals.var_litl_edge_dn5, locals.var_litl_edge_dn6, locals.var_litl_edge_dn7, locals.var_litl_edge_dn8, locals.var_litl_edge_dn9, locals.var_litl_edge_dn10, locals.var_litl_edge_dn11,)
    }
};
        locals.var_litl_edge = assign59280_e96326;
        locals.var_litl_edge_dn3 = assign59280_e96326_d_n3;
        locals.var_litl_edge_dn4 = assign59280_e96326_d_n4;
        locals.var_litl_edge_dn5 = assign59280_e96326_d_n5;
        locals.var_litl_edge_dn6 = assign59280_e96326_d_n6;
        locals.var_litl_edge_dn7 = assign59280_e96326_d_n7;
        locals.var_litl_edge_dn8 = assign59280_e96326_d_n8;
        locals.var_litl_edge_dn9 = assign59280_e96326_d_n9;
        locals.var_litl_edge_dn10 = assign59280_e96326_d_n10;
        locals.var_litl_edge_dn11 = assign59280_e96326_d_n11;

        let assign59290_e96329: f64 = if locals.var_litl_edge > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard868 = assign59290_e96329;

        let (assign59300_e96342, assign59300_e96342_d_n3, assign59300_e96342_d_n4, assign59300_e96342_d_n5, assign59300_e96342_d_n6, assign59300_e96342_d_n7, assign59300_e96342_d_n8, assign59300_e96342_d_n9, assign59300_e96342_d_n10, assign59300_e96342_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 != 0.0)) {
        let assign59300_e96338: f64 = (p.p1263 * locals.var_leff);
        let assign59300_e96340: f64 = (assign59300_e96338 / locals.var_litl_edge);
        (assign59300_e96340, (-((assign59300_e96338 * locals.var_litl_edge_dn3) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn4) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn5) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn6) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn7) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn8) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn9) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn10) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn11) / (locals.var_litl_edge * locals.var_litl_edge))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign59300_e96342;
        locals.var_t0_dn3 = assign59300_e96342_d_n3;
        locals.var_t0_dn4 = assign59300_e96342_d_n4;
        locals.var_t0_dn5 = assign59300_e96342_d_n5;
        locals.var_t0_dn6 = assign59300_e96342_d_n6;
        locals.var_t0_dn7 = assign59300_e96342_d_n7;
        locals.var_t0_dn8 = assign59300_e96342_d_n8;
        locals.var_t0_dn9 = assign59300_e96342_d_n9;
        locals.var_t0_dn10 = assign59300_e96342_d_n10;
        locals.var_t0_dn11 = assign59300_e96342_d_n11;

        let assign59310_e96345: f64 = if locals.var_t0 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard869 = assign59310_e96345;

        let (assign59320_e96363, assign59320_e96363_d_n3, assign59320_e96363_d_n4, assign59320_e96363_d_n5, assign59320_e96363_d_n6, assign59320_e96363_d_n7, assign59320_e96363_d_n8, assign59320_e96363_d_n9, assign59320_e96363_d_n10, assign59320_e96363_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_guard869 != 0.0)) {
        let assign59320_e96356: f64 = (0.5 * p.p1262);
        let assign59320_e96358: f64 = (locals.var_t0).cosh();
        let assign59320_e96360: f64 = (assign59320_e96358 - 1.0);
        let assign59320_e96361: f64 = (assign59320_e96356 / assign59320_e96360);
        (assign59320_e96361, (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn3)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn4)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn5)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn6)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn7)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn8)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn9)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn10)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn11)) / (assign59320_e96360 * assign59320_e96360))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11,)
    }
};
        locals.var_theta_sce_edge = assign59320_e96363;
        locals.var_theta_sce_edge_dn3 = assign59320_e96363_d_n3;
        locals.var_theta_sce_edge_dn4 = assign59320_e96363_d_n4;
        locals.var_theta_sce_edge_dn5 = assign59320_e96363_d_n5;
        locals.var_theta_sce_edge_dn6 = assign59320_e96363_d_n6;
        locals.var_theta_sce_edge_dn7 = assign59320_e96363_d_n7;
        locals.var_theta_sce_edge_dn8 = assign59320_e96363_d_n8;
        locals.var_theta_sce_edge_dn9 = assign59320_e96363_d_n9;
        locals.var_theta_sce_edge_dn10 = assign59320_e96363_d_n10;
        locals.var_theta_sce_edge_dn11 = assign59320_e96363_d_n11;

        let (assign59330_e96379, assign59330_e96379_d_n3, assign59330_e96379_d_n4, assign59330_e96379_d_n5, assign59330_e96379_d_n6, assign59330_e96379_d_n7, assign59330_e96379_d_n8, assign59330_e96379_d_n9, assign59330_e96379_d_n10, assign59330_e96379_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_guard869 == 0.0)) {
        let assign59330_e96375: f64 = (-locals.var_t0);
        let assign59330_e96376: f64 = { let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign59330_e96377: f64 = (p.p1262 * assign59330_e96376);
        (assign59330_e96377, (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn3))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11,)
    }
};
        locals.var_theta_sce_edge = assign59330_e96379;
        locals.var_theta_sce_edge_dn3 = assign59330_e96379_d_n3;
        locals.var_theta_sce_edge_dn4 = assign59330_e96379_d_n4;
        locals.var_theta_sce_edge_dn5 = assign59330_e96379_d_n5;
        locals.var_theta_sce_edge_dn6 = assign59330_e96379_d_n6;
        locals.var_theta_sce_edge_dn7 = assign59330_e96379_d_n7;
        locals.var_theta_sce_edge_dn8 = assign59330_e96379_d_n8;
        locals.var_theta_sce_edge_dn9 = assign59330_e96379_d_n9;
        locals.var_theta_sce_edge_dn10 = assign59330_e96379_d_n10;
        locals.var_theta_sce_edge_dn11 = assign59330_e96379_d_n11;

    }

    pub(super) fn stamp_transient_block_196(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign59340_e96389, assign59340_e96389_d_n3, assign59340_e96389_d_n4, assign59340_e96389_d_n5, assign59340_e96389_d_n6, assign59340_e96389_d_n7, assign59340_e96389_d_n8, assign59340_e96389_d_n9, assign59340_e96389_d_n10, assign59340_e96389_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11,)
    }
};
        locals.var_theta_sce_edge = assign59340_e96389;
        locals.var_theta_sce_edge_dn3 = assign59340_e96389_d_n3;
        locals.var_theta_sce_edge_dn4 = assign59340_e96389_d_n4;
        locals.var_theta_sce_edge_dn5 = assign59340_e96389_d_n5;
        locals.var_theta_sce_edge_dn6 = assign59340_e96389_d_n6;
        locals.var_theta_sce_edge_dn7 = assign59340_e96389_d_n7;
        locals.var_theta_sce_edge_dn8 = assign59340_e96389_d_n8;
        locals.var_theta_sce_edge_dn9 = assign59340_e96389_d_n9;
        locals.var_theta_sce_edge_dn10 = assign59340_e96389_d_n10;
        locals.var_theta_sce_edge_dn11 = assign59340_e96389_d_n11;

        let (assign59350_e96400, assign59350_e96400_d_n3, assign59350_e96400_d_n4, assign59350_e96400_d_n5, assign59350_e96400_d_n6, assign59350_e96400_d_n7, assign59350_e96400_d_n8, assign59350_e96400_d_n9, assign59350_e96400_d_n10, assign59350_e96400_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59350_e96397: f64 = (locals.var_vbi_edge - locals.var_phist);
        let assign59350_e96398: f64 = (locals.var_theta_sce_edge * assign59350_e96397);
        (assign59350_e96398, ((locals.var_theta_sce_edge_dn3 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn3 - locals.var_phist_dn3))), ((locals.var_theta_sce_edge_dn4 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn4 - locals.var_phist_dn4))), ((locals.var_theta_sce_edge_dn5 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn5 - locals.var_phist_dn5))), ((locals.var_theta_sce_edge_dn6 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn6 - locals.var_phist_dn6))), ((locals.var_theta_sce_edge_dn7 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn7 - locals.var_phist_dn7))), ((locals.var_theta_sce_edge_dn8 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn8 - locals.var_phist_dn8))), ((locals.var_theta_sce_edge_dn9 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn9 - locals.var_phist_dn9))), ((locals.var_theta_sce_edge_dn10 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn10 - locals.var_phist_dn10))), ((locals.var_theta_sce_edge_dn11 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn11 - locals.var_phist_dn11))),)
    } else {
        (locals.var_dvth_sce, locals.var_dvth_sce_dn3, locals.var_dvth_sce_dn4, locals.var_dvth_sce_dn5, locals.var_dvth_sce_dn6, locals.var_dvth_sce_dn7, locals.var_dvth_sce_dn8, locals.var_dvth_sce_dn9, locals.var_dvth_sce_dn10, locals.var_dvth_sce_dn11,)
    }
};
        locals.var_dvth_sce = assign59350_e96400;
        locals.var_dvth_sce_dn3 = assign59350_e96400_d_n3;
        locals.var_dvth_sce_dn4 = assign59350_e96400_d_n4;
        locals.var_dvth_sce_dn5 = assign59350_e96400_d_n5;
        locals.var_dvth_sce_dn6 = assign59350_e96400_d_n6;
        locals.var_dvth_sce_dn7 = assign59350_e96400_d_n7;
        locals.var_dvth_sce_dn8 = assign59350_e96400_d_n8;
        locals.var_dvth_sce_dn9 = assign59350_e96400_d_n9;
        locals.var_dvth_sce_dn10 = assign59350_e96400_d_n10;
        locals.var_dvth_sce_dn11 = assign59350_e96400_d_n11;

        let (assign59360_e96421, assign59360_e96421_d_n3, assign59360_e96421_d_n4, assign59360_e96421_d_n5, assign59360_e96421_d_n6, assign59360_e96421_d_n7, assign59360_e96421_d_n8, assign59360_e96421_d_n9, assign59360_e96421_d_n10, assign59360_e96421_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59360_e96407: f64 = (locals.var_dvth_dibl_1 - locals.var_dvth_temp);
        let assign59360_e96409: f64 = (assign59360_e96407 + locals.var_dvth_sce);
        let assign59360_e96411: f64 = (assign59360_e96409 + p.p1151);
        let assign59360_e96413: f64 = (assign59360_e96411 + locals.var_vth0_stress_edge);
        let assign59360_e96416: f64 = (locals.var_k2edge_i * locals.var_vbsx);
        let assign59360_e96417: f64 = (assign59360_e96413 - assign59360_e96416);
        let assign59360_e96419: f64 = (assign59360_e96417 + locals.var_vth0_well_edge);
        (assign59360_e96419, (((((locals.var_dvth_dibl_1_dn3 - locals.var_dvth_temp_dn3) + locals.var_dvth_sce_dn3) + locals.var_vth0_stress_edge_dn3) - ((locals.var_k2edge_i_dn3 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn3))) + locals.var_vth0_well_edge_dn3), (((((locals.var_dvth_dibl_1_dn4 - locals.var_dvth_temp_dn4) + locals.var_dvth_sce_dn4) + locals.var_vth0_stress_edge_dn4) - ((locals.var_k2edge_i_dn4 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn4))) + locals.var_vth0_well_edge_dn4), (((((locals.var_dvth_dibl_1_dn5 - locals.var_dvth_temp_dn5) + locals.var_dvth_sce_dn5) + locals.var_vth0_stress_edge_dn5) - ((locals.var_k2edge_i_dn5 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn5))) + locals.var_vth0_well_edge_dn5), (((((locals.var_dvth_dibl_1_dn6 - locals.var_dvth_temp_dn6) + locals.var_dvth_sce_dn6) + locals.var_vth0_stress_edge_dn6) - ((locals.var_k2edge_i_dn6 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn6))) + locals.var_vth0_well_edge_dn6), (((((locals.var_dvth_dibl_1_dn7 - locals.var_dvth_temp_dn7) + locals.var_dvth_sce_dn7) + locals.var_vth0_stress_edge_dn7) - ((locals.var_k2edge_i_dn7 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn7))) + locals.var_vth0_well_edge_dn7), (((((locals.var_dvth_dibl_1_dn8 - locals.var_dvth_temp_dn8) + locals.var_dvth_sce_dn8) + locals.var_vth0_stress_edge_dn8) - ((locals.var_k2edge_i_dn8 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn8))) + locals.var_vth0_well_edge_dn8), (((((locals.var_dvth_dibl_1_dn9 - locals.var_dvth_temp_dn9) + locals.var_dvth_sce_dn9) + locals.var_vth0_stress_edge_dn9) - ((locals.var_k2edge_i_dn9 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn9))) + locals.var_vth0_well_edge_dn9), (((((locals.var_dvth_dibl_1_dn10 - locals.var_dvth_temp_dn10) + locals.var_dvth_sce_dn10) + locals.var_vth0_stress_edge_dn10) - ((locals.var_k2edge_i_dn10 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn10))) + locals.var_vth0_well_edge_dn10), (((((locals.var_dvth_dibl_1_dn11 - locals.var_dvth_temp_dn11) + locals.var_dvth_sce_dn11) + locals.var_vth0_stress_edge_dn11) - ((locals.var_k2edge_i_dn11 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn11))) + locals.var_vth0_well_edge_dn11),)
    } else {
        (locals.var_vth_shift, locals.var_vth_shift_dn3, locals.var_vth_shift_dn4, locals.var_vth_shift_dn5, locals.var_vth_shift_dn6, locals.var_vth_shift_dn7, locals.var_vth_shift_dn8, locals.var_vth_shift_dn9, locals.var_vth_shift_dn10, locals.var_vth_shift_dn11,)
    }
};
        locals.var_vth_shift = assign59360_e96421;
        locals.var_vth_shift_dn3 = assign59360_e96421_d_n3;
        locals.var_vth_shift_dn4 = assign59360_e96421_d_n4;
        locals.var_vth_shift_dn5 = assign59360_e96421_d_n5;
        locals.var_vth_shift_dn6 = assign59360_e96421_d_n6;
        locals.var_vth_shift_dn7 = assign59360_e96421_d_n7;
        locals.var_vth_shift_dn8 = assign59360_e96421_d_n8;
        locals.var_vth_shift_dn9 = assign59360_e96421_d_n9;
        locals.var_vth_shift_dn10 = assign59360_e96421_d_n10;
        locals.var_vth_shift_dn11 = assign59360_e96421_d_n11;

        let (assign59370_e96434, assign59370_e96434_d_n3, assign59370_e96434_d_n4, assign59370_e96434_d_n5, assign59370_e96434_d_n6, assign59370_e96434_d_n7, assign59370_e96434_d_n8, assign59370_e96434_d_n9, assign59370_e96434_d_n10, assign59370_e96434_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59370_e96428: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign59370_e96431: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign59370_e96432: f64 = (assign59370_e96428 - assign59370_e96431);
        (assign59370_e96432, ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3))), ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4))), ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5))), ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6))), ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7))), ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8))), ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9))), ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10))), ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11))),)
    } else {
        (locals.var_vgfb, locals.var_vgfb_dn3, locals.var_vgfb_dn4, locals.var_vgfb_dn5, locals.var_vgfb_dn6, locals.var_vgfb_dn7, locals.var_vgfb_dn8, locals.var_vgfb_dn9, locals.var_vgfb_dn10, locals.var_vgfb_dn11,)
    }
};
        locals.var_vgfb = assign59370_e96434;
        locals.var_vgfb_dn3 = assign59370_e96434_d_n3;
        locals.var_vgfb_dn4 = assign59370_e96434_d_n4;
        locals.var_vgfb_dn5 = assign59370_e96434_d_n5;
        locals.var_vgfb_dn6 = assign59370_e96434_d_n6;
        locals.var_vgfb_dn7 = assign59370_e96434_d_n7;
        locals.var_vgfb_dn8 = assign59370_e96434_d_n8;
        locals.var_vgfb_dn9 = assign59370_e96434_d_n9;
        locals.var_vgfb_dn10 = assign59370_e96434_d_n10;
        locals.var_vgfb_dn11 = assign59370_e96434_d_n11;

        let (assign59380_e96450,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59380_e96444: f64 = (-p.p1150);
        let assign59380_e96445: f64 = (locals.var_leff).powf(assign59380_e96444);
        let assign59380_e96446: f64 = (p.p1149 * assign59380_e96445);
        let assign59380_e96447: f64 = (1.0 + assign59380_e96446);
        let assign59380_e96448: f64 = (p.p1148 * assign59380_e96447);
        (assign59380_e96448,)
    } else {
        (locals.var_dgammaedge_i,)
    }
};
        locals.var_dgammaedge_i = assign59380_e96450;

        let (assign59390_e96468, assign59390_e96468_d_n3, assign59390_e96468_d_n4, assign59390_e96468_d_n5, assign59390_e96468_d_n6, assign59390_e96468_d_n7, assign59390_e96468_d_n8, assign59390_e96468_d_n9, assign59390_e96468_d_n10, assign59390_e96468_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59390_e96457: f64 = (2.0 * 1.602176462e-19);
        let assign59390_e96459: f64 = (assign59390_e96457 * locals.var_epssi);
        let assign59390_e96461: f64 = (assign59390_e96459 * locals.var_ndepedge_i);
        let assign59390_e96463: f64 = (assign59390_e96461 * locals.var_inv_nvt);
        let assign59390_e96464: f64 = (assign59390_e96463).sqrt();
        let assign59390_e96466: f64 = (assign59390_e96464 / locals.var_cox);
        (assign59390_e96466, (((assign59390_e96461 * locals.var_inv_nvt_dn3) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn4) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn5) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn6) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn7) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn8) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn9) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn10) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn11) / (2.0 * assign59390_e96464)) / locals.var_cox),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11,)
    }
};
        locals.var_gam_edge = assign59390_e96468;
        locals.var_gam_edge_dn3 = assign59390_e96468_d_n3;
        locals.var_gam_edge_dn4 = assign59390_e96468_d_n4;
        locals.var_gam_edge_dn5 = assign59390_e96468_d_n5;
        locals.var_gam_edge_dn6 = assign59390_e96468_d_n6;
        locals.var_gam_edge_dn7 = assign59390_e96468_d_n7;
        locals.var_gam_edge_dn8 = assign59390_e96468_d_n8;
        locals.var_gam_edge_dn9 = assign59390_e96468_d_n9;
        locals.var_gam_edge_dn10 = assign59390_e96468_d_n10;
        locals.var_gam_edge_dn11 = assign59390_e96468_d_n11;

        let (assign59400_e96479, assign59400_e96479_d_n3, assign59400_e96479_d_n4, assign59400_e96479_d_n5, assign59400_e96479_d_n6, assign59400_e96479_d_n7, assign59400_e96479_d_n8, assign59400_e96479_d_n9, assign59400_e96479_d_n10, assign59400_e96479_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59400_e96476: f64 = (1.0 + locals.var_dgammaedge_i);
        let assign59400_e96477: f64 = (locals.var_gam_edge * assign59400_e96476);
        (assign59400_e96477, (locals.var_gam_edge_dn3 * assign59400_e96476), (locals.var_gam_edge_dn4 * assign59400_e96476), (locals.var_gam_edge_dn5 * assign59400_e96476), (locals.var_gam_edge_dn6 * assign59400_e96476), (locals.var_gam_edge_dn7 * assign59400_e96476), (locals.var_gam_edge_dn8 * assign59400_e96476), (locals.var_gam_edge_dn9 * assign59400_e96476), (locals.var_gam_edge_dn10 * assign59400_e96476), (locals.var_gam_edge_dn11 * assign59400_e96476),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11,)
    }
};
        locals.var_gam_edge = assign59400_e96479;
        locals.var_gam_edge_dn3 = assign59400_e96479_d_n3;
        locals.var_gam_edge_dn4 = assign59400_e96479_d_n4;
        locals.var_gam_edge_dn5 = assign59400_e96479_d_n5;
        locals.var_gam_edge_dn6 = assign59400_e96479_d_n6;
        locals.var_gam_edge_dn7 = assign59400_e96479_d_n7;
        locals.var_gam_edge_dn8 = assign59400_e96479_d_n8;
        locals.var_gam_edge_dn9 = assign59400_e96479_d_n9;
        locals.var_gam_edge_dn10 = assign59400_e96479_d_n10;
        locals.var_gam_edge_dn11 = assign59400_e96479_d_n11;

        let (assign59410_e96488, assign59410_e96488_d_n3, assign59410_e96488_d_n4, assign59410_e96488_d_n5, assign59410_e96488_d_n6, assign59410_e96488_d_n7, assign59410_e96488_d_n8, assign59410_e96488_d_n9, assign59410_e96488_d_n10, assign59410_e96488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59410_e96486: f64 = (locals.var_phib_edge / locals.var_n);
        (assign59410_e96486, (((locals.var_phib_edge_dn3 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn3)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn4 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn4)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn5 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn5)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn6 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn6)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn7 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn7)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn8 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn8)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn9 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn9)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn10 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn10)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn11 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn11)) / (locals.var_n * locals.var_n)),)
    } else {
        (locals.var_phib_n_edge, locals.var_phib_n_edge_dn3, locals.var_phib_n_edge_dn4, locals.var_phib_n_edge_dn5, locals.var_phib_n_edge_dn6, locals.var_phib_n_edge_dn7, locals.var_phib_n_edge_dn8, locals.var_phib_n_edge_dn9, locals.var_phib_n_edge_dn10, locals.var_phib_n_edge_dn11,)
    }
};
        locals.var_phib_n_edge = assign59410_e96488;
        locals.var_phib_n_edge_dn3 = assign59410_e96488_d_n3;
        locals.var_phib_n_edge_dn4 = assign59410_e96488_d_n4;
        locals.var_phib_n_edge_dn5 = assign59410_e96488_d_n5;
        locals.var_phib_n_edge_dn6 = assign59410_e96488_d_n6;
        locals.var_phib_n_edge_dn7 = assign59410_e96488_d_n7;
        locals.var_phib_n_edge_dn8 = assign59410_e96488_d_n8;
        locals.var_phib_n_edge_dn9 = assign59410_e96488_d_n9;
        locals.var_phib_n_edge_dn10 = assign59410_e96488_d_n10;
        locals.var_phib_n_edge_dn11 = assign59410_e96488_d_n11;

        let (assign59420_e96497, assign59420_e96497_d_n3, assign59420_e96497_d_n4, assign59420_e96497_d_n5, assign59420_e96497_d_n6, assign59420_e96497_d_n7, assign59420_e96497_d_n8, assign59420_e96497_d_n9, assign59420_e96497_d_n10, assign59420_e96497_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59420_e96495: f64 = 1.0;
        (assign59420_e96495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59420_e96497;
        locals.var_t1_dn3 = assign59420_e96497_d_n3;
        locals.var_t1_dn4 = assign59420_e96497_d_n4;
        locals.var_t1_dn5 = assign59420_e96497_d_n5;
        locals.var_t1_dn6 = assign59420_e96497_d_n6;
        locals.var_t1_dn7 = assign59420_e96497_d_n7;
        locals.var_t1_dn8 = assign59420_e96497_d_n8;
        locals.var_t1_dn9 = assign59420_e96497_d_n9;
        locals.var_t1_dn10 = assign59420_e96497_d_n10;
        locals.var_t1_dn11 = assign59420_e96497_d_n11;

        let (assign59430_e96506, assign59430_e96506_d_n3, assign59430_e96506_d_n4, assign59430_e96506_d_n5, assign59430_e96506_d_n6, assign59430_e96506_d_n7, assign59430_e96506_d_n8, assign59430_e96506_d_n9, assign59430_e96506_d_n10, assign59430_e96506_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59430_e96504: f64 = (locals.var_vgfb / locals.var_t1);
        (assign59430_e96504, (((locals.var_vgfb_dn3 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn4 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn5 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn6 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn7 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn8 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn9 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn10 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn11 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11,)
    }
};
        locals.var_vgfbpd = assign59430_e96506;
        locals.var_vgfbpd_dn3 = assign59430_e96506_d_n3;
        locals.var_vgfbpd_dn4 = assign59430_e96506_d_n4;
        locals.var_vgfbpd_dn5 = assign59430_e96506_d_n5;
        locals.var_vgfbpd_dn6 = assign59430_e96506_d_n6;
        locals.var_vgfbpd_dn7 = assign59430_e96506_d_n7;
        locals.var_vgfbpd_dn8 = assign59430_e96506_d_n8;
        locals.var_vgfbpd_dn9 = assign59430_e96506_d_n9;
        locals.var_vgfbpd_dn10 = assign59430_e96506_d_n10;
        locals.var_vgfbpd_dn11 = assign59430_e96506_d_n11;

        let (assign59440_e96515, assign59440_e96515_d_n3, assign59440_e96515_d_n4, assign59440_e96515_d_n5, assign59440_e96515_d_n6, assign59440_e96515_d_n7, assign59440_e96515_d_n8, assign59440_e96515_d_n9, assign59440_e96515_d_n10, assign59440_e96515_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59440_e96513: f64 = (locals.var_gam_edge / locals.var_t1);
        (assign59440_e96513, (((locals.var_gam_edge_dn3 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn4 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn5 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn6 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn7 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn8 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn9 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn10 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn11 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11,)
    }
};
        locals.var_gammapd = assign59440_e96515;
        locals.var_gammapd_dn3 = assign59440_e96515_d_n3;
        locals.var_gammapd_dn4 = assign59440_e96515_d_n4;
        locals.var_gammapd_dn5 = assign59440_e96515_d_n5;
        locals.var_gammapd_dn6 = assign59440_e96515_d_n6;
        locals.var_gammapd_dn7 = assign59440_e96515_d_n7;
        locals.var_gammapd_dn8 = assign59440_e96515_d_n8;
        locals.var_gammapd_dn9 = assign59440_e96515_d_n9;
        locals.var_gammapd_dn10 = assign59440_e96515_d_n10;
        locals.var_gammapd_dn11 = assign59440_e96515_d_n11;

        let (assign59450_e96532, assign59450_e96532_d_n3, assign59450_e96532_d_n4, assign59450_e96532_d_n5, assign59450_e96532_d_n6, assign59450_e96532_d_n7, assign59450_e96532_d_n8, assign59450_e96532_d_n9, assign59450_e96532_d_n10, assign59450_e96532_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59450_e96522: f64 = (0.5 * locals.var_vgfbpd);
        let assign59450_e96527: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign59450_e96528: f64 = (1.0 + assign59450_e96527);
        let assign59450_e96529: f64 = (3.0 * assign59450_e96528);
        let assign59450_e96530: f64 = (assign59450_e96522 - assign59450_e96529);
        (assign59450_e96530, ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59450_e96532;
        locals.var_t1_dn3 = assign59450_e96532_d_n3;
        locals.var_t1_dn4 = assign59450_e96532_d_n4;
        locals.var_t1_dn5 = assign59450_e96532_d_n5;
        locals.var_t1_dn6 = assign59450_e96532_d_n6;
        locals.var_t1_dn7 = assign59450_e96532_d_n7;
        locals.var_t1_dn8 = assign59450_e96532_d_n8;
        locals.var_t1_dn9 = assign59450_e96532_d_n9;
        locals.var_t1_dn10 = assign59450_e96532_d_n10;
        locals.var_t1_dn11 = assign59450_e96532_d_n11;

        let (assign59460_e96548, assign59460_e96548_d_n3, assign59460_e96548_d_n4, assign59460_e96548_d_n5, assign59460_e96548_d_n6, assign59460_e96548_d_n7, assign59460_e96548_d_n8, assign59460_e96548_d_n9, assign59460_e96548_d_n10, assign59460_e96548_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59460_e96540: f64 = (locals.var_t1 * locals.var_t1);
        let assign59460_e96543: f64 = (6.0 * locals.var_vgfbpd);
        let assign59460_e96544: f64 = (assign59460_e96540 + assign59460_e96543);
        let assign59460_e96545: f64 = (assign59460_e96544).sqrt();
        let assign59460_e96546: f64 = (locals.var_t1 + assign59460_e96545);
        (assign59460_e96546, (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign59460_e96545))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59460_e96548;
        locals.var_t2_dn3 = assign59460_e96548_d_n3;
        locals.var_t2_dn4 = assign59460_e96548_d_n4;
        locals.var_t2_dn5 = assign59460_e96548_d_n5;
        locals.var_t2_dn6 = assign59460_e96548_d_n6;
        locals.var_t2_dn7 = assign59460_e96548_d_n7;
        locals.var_t2_dn8 = assign59460_e96548_d_n8;
        locals.var_t2_dn9 = assign59460_e96548_d_n9;
        locals.var_t2_dn10 = assign59460_e96548_d_n10;
        locals.var_t2_dn11 = assign59460_e96548_d_n11;

        let assign59470_e96551: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard870 = assign59470_e96551;

        let (assign59480_e96564, assign59480_e96564_d_n3, assign59480_e96564_d_n4, assign59480_e96564_d_n5, assign59480_e96564_d_n6, assign59480_e96564_d_n7, assign59480_e96564_d_n8, assign59480_e96564_d_n9, assign59480_e96564_d_n10, assign59480_e96564_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign59480_e96560: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign59480_e96562: f64 = (assign59480_e96560 / locals.var_gammapd);
        (assign59480_e96562, ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59480_e96564;
        locals.var_t3_dn3 = assign59480_e96564_d_n3;
        locals.var_t3_dn4 = assign59480_e96564_d_n4;
        locals.var_t3_dn5 = assign59480_e96564_d_n5;
        locals.var_t3_dn6 = assign59480_e96564_d_n6;
        locals.var_t3_dn7 = assign59480_e96564_d_n7;
        locals.var_t3_dn8 = assign59480_e96564_d_n8;
        locals.var_t3_dn9 = assign59480_e96564_d_n9;
        locals.var_t3_dn10 = assign59480_e96564_d_n10;
        locals.var_t3_dn11 = assign59480_e96564_d_n11;

        let (assign59490_e96583, assign59490_e96583_d_n3, assign59490_e96583_d_n4, assign59490_e96583_d_n5, assign59490_e96583_d_n6, assign59490_e96583_d_n7, assign59490_e96583_d_n8, assign59490_e96583_d_n9, assign59490_e96583_d_n10, assign59490_e96583_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign59490_e96573: f64 = (1.0 - locals.var_t2);
        let assign59490_e96576: f64 = (locals.var_t3 * locals.var_t3);
        let assign59490_e96577: f64 = (assign59490_e96573 + assign59490_e96576);
        let assign59490_e96579: f64 = (assign59490_e96577).max(1e-38);
        let assign59490_e96580: f64 = (assign59490_e96579).ln();
        let assign59490_e96581: f64 = (-assign59490_e96580);
        (assign59490_e96581, (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign59490_e96579)),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign59490_e96583;
        locals.var_psip_dn3 = assign59490_e96583_d_n3;
        locals.var_psip_dn4 = assign59490_e96583_d_n4;
        locals.var_psip_dn5 = assign59490_e96583_d_n5;
        locals.var_psip_dn6 = assign59490_e96583_d_n6;
        locals.var_psip_dn7 = assign59490_e96583_d_n7;
        locals.var_psip_dn8 = assign59490_e96583_d_n8;
        locals.var_psip_dn9 = assign59490_e96583_d_n9;
        locals.var_psip_dn10 = assign59490_e96583_d_n10;
        locals.var_psip_dn11 = assign59490_e96583_d_n11;

        let (assign59500_e96595, assign59500_e96595_d_n3, assign59500_e96595_d_n4, assign59500_e96595_d_n5, assign59500_e96595_d_n6, assign59500_e96595_d_n7, assign59500_e96595_d_n8, assign59500_e96595_d_n9, assign59500_e96595_d_n10, assign59500_e96595_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59500_e96592: f64 = (-locals.var_t2);
        let assign59500_e96593: f64 = { let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59500_e96593, ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59500_e96595;
        locals.var_t3_dn3 = assign59500_e96595_d_n3;
        locals.var_t3_dn4 = assign59500_e96595_d_n4;
        locals.var_t3_dn5 = assign59500_e96595_d_n5;
        locals.var_t3_dn6 = assign59500_e96595_d_n6;
        locals.var_t3_dn7 = assign59500_e96595_d_n7;
        locals.var_t3_dn8 = assign59500_e96595_d_n8;
        locals.var_t3_dn9 = assign59500_e96595_d_n9;
        locals.var_t3_dn10 = assign59500_e96595_d_n10;
        locals.var_t3_dn11 = assign59500_e96595_d_n11;

        let (assign59510_e96607, assign59510_e96607_d_n3, assign59510_e96607_d_n4, assign59510_e96607_d_n5, assign59510_e96607_d_n6, assign59510_e96607_d_n7, assign59510_e96607_d_n8, assign59510_e96607_d_n9, assign59510_e96607_d_n10, assign59510_e96607_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59510_e96605: f64 = (0.5 * locals.var_gammapd);
        (assign59510_e96605, (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59510_e96607;
        locals.var_t1_dn3 = assign59510_e96607_d_n3;
        locals.var_t1_dn4 = assign59510_e96607_d_n4;
        locals.var_t1_dn5 = assign59510_e96607_d_n5;
        locals.var_t1_dn6 = assign59510_e96607_d_n6;
        locals.var_t1_dn7 = assign59510_e96607_d_n7;
        locals.var_t1_dn8 = assign59510_e96607_d_n8;
        locals.var_t1_dn9 = assign59510_e96607_d_n9;
        locals.var_t1_dn10 = assign59510_e96607_d_n10;
        locals.var_t1_dn11 = assign59510_e96607_d_n11;

        let (assign59520_e96628, assign59520_e96628_d_n3, assign59520_e96628_d_n4, assign59520_e96628_d_n5, assign59520_e96628_d_n6, assign59520_e96628_d_n7, assign59520_e96628_d_n8, assign59520_e96628_d_n9, assign59520_e96628_d_n10, assign59520_e96628_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59520_e96617: f64 = (locals.var_vgfbpd - 1.0);
        let assign59520_e96619: f64 = (assign59520_e96617 + locals.var_t3);
        let assign59520_e96622: f64 = (locals.var_t1 * locals.var_t1);
        let assign59520_e96623: f64 = (assign59520_e96619 + assign59520_e96622);
        let assign59520_e96624: f64 = (assign59520_e96623).sqrt();
        let assign59520_e96626: f64 = (assign59520_e96624 - locals.var_t1);
        (assign59520_e96626, ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59520_e96628;
        locals.var_t2_dn3 = assign59520_e96628_d_n3;
        locals.var_t2_dn4 = assign59520_e96628_d_n4;
        locals.var_t2_dn5 = assign59520_e96628_d_n5;
        locals.var_t2_dn6 = assign59520_e96628_d_n6;
        locals.var_t2_dn7 = assign59520_e96628_d_n7;
        locals.var_t2_dn8 = assign59520_e96628_d_n8;
        locals.var_t2_dn9 = assign59520_e96628_d_n9;
        locals.var_t2_dn10 = assign59520_e96628_d_n10;
        locals.var_t2_dn11 = assign59520_e96628_d_n11;

        let (assign59530_e96644, assign59530_e96644_d_n3, assign59530_e96644_d_n4, assign59530_e96644_d_n5, assign59530_e96644_d_n6, assign59530_e96644_d_n7, assign59530_e96644_d_n8, assign59530_e96644_d_n9, assign59530_e96644_d_n10, assign59530_e96644_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59530_e96638: f64 = (locals.var_t2 * locals.var_t2);
        let assign59530_e96640: f64 = (assign59530_e96638 + 1.0);
        let assign59530_e96642: f64 = (assign59530_e96640 - locals.var_t3);
        (assign59530_e96642, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign59530_e96644;
        locals.var_psip_dn3 = assign59530_e96644_d_n3;
        locals.var_psip_dn4 = assign59530_e96644_d_n4;
        locals.var_psip_dn5 = assign59530_e96644_d_n5;
        locals.var_psip_dn6 = assign59530_e96644_d_n6;
        locals.var_psip_dn7 = assign59530_e96644_d_n7;
        locals.var_psip_dn8 = assign59530_e96644_d_n8;
        locals.var_psip_dn9 = assign59530_e96644_d_n9;
        locals.var_psip_dn10 = assign59530_e96644_d_n10;
        locals.var_psip_dn11 = assign59530_e96644_d_n11;

        let (assign59540_e96670, assign59540_e96670_d_n3, assign59540_e96670_d_n4, assign59540_e96670_d_n5, assign59540_e96670_d_n6, assign59540_e96670_d_n7, assign59540_e96670_d_n8, assign59540_e96670_d_n9, assign59540_e96670_d_n10, assign59540_e96670_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59540_e96652: f64 = (locals.var_psip + 1.0);
        let assign59540_e96655: f64 = (locals.var_psip - 1.0);
        let assign59540_e96658: f64 = (locals.var_psip - 1.0);
        let assign59540_e96659: f64 = (assign59540_e96655 * assign59540_e96658);
        let assign59540_e96662: f64 = (0.25 * 2.0);
        let assign59540_e96664: f64 = (assign59540_e96662 * 2.0);
        let assign59540_e96665: f64 = (assign59540_e96659 + assign59540_e96664);
        let assign59540_e96666: f64 = (assign59540_e96665).sqrt();
        let assign59540_e96667: f64 = (assign59540_e96652 + assign59540_e96666);
        let assign59540_e96668: f64 = (0.5 * assign59540_e96667);
        (assign59540_e96668, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn3)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn4)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn5)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn6)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn7)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn8)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn9)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn10)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn11)) / (2.0 * assign59540_e96666)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59540_e96670;
        locals.var_t8_dn3 = assign59540_e96670_d_n3;
        locals.var_t8_dn4 = assign59540_e96670_d_n4;
        locals.var_t8_dn5 = assign59540_e96670_d_n5;
        locals.var_t8_dn6 = assign59540_e96670_d_n6;
        locals.var_t8_dn7 = assign59540_e96670_d_n7;
        locals.var_t8_dn8 = assign59540_e96670_d_n8;
        locals.var_t8_dn9 = assign59540_e96670_d_n9;
        locals.var_t8_dn10 = assign59540_e96670_d_n10;
        locals.var_t8_dn11 = assign59540_e96670_d_n11;

        let (assign59550_e96678, assign59550_e96678_d_n3, assign59550_e96678_d_n4, assign59550_e96678_d_n5, assign59550_e96678_d_n6, assign59550_e96678_d_n7, assign59550_e96678_d_n8, assign59550_e96678_d_n9, assign59550_e96678_d_n10, assign59550_e96678_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59550_e96676: f64 = (locals.var_t8).sqrt();
        (assign59550_e96676, (locals.var_t8_dn3 / (2.0 * assign59550_e96676)), (locals.var_t8_dn4 / (2.0 * assign59550_e96676)), (locals.var_t8_dn5 / (2.0 * assign59550_e96676)), (locals.var_t8_dn6 / (2.0 * assign59550_e96676)), (locals.var_t8_dn7 / (2.0 * assign59550_e96676)), (locals.var_t8_dn8 / (2.0 * assign59550_e96676)), (locals.var_t8_dn9 / (2.0 * assign59550_e96676)), (locals.var_t8_dn10 / (2.0 * assign59550_e96676)), (locals.var_t8_dn11 / (2.0 * assign59550_e96676)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign59550_e96678;
        locals.var_sqrtpsip_dn3 = assign59550_e96678_d_n3;
        locals.var_sqrtpsip_dn4 = assign59550_e96678_d_n4;
        locals.var_sqrtpsip_dn5 = assign59550_e96678_d_n5;
        locals.var_sqrtpsip_dn6 = assign59550_e96678_d_n6;
        locals.var_sqrtpsip_dn7 = assign59550_e96678_d_n7;
        locals.var_sqrtpsip_dn8 = assign59550_e96678_d_n8;
        locals.var_sqrtpsip_dn9 = assign59550_e96678_d_n9;
        locals.var_sqrtpsip_dn10 = assign59550_e96678_d_n10;
        locals.var_sqrtpsip_dn11 = assign59550_e96678_d_n11;

        let (assign59560_e96693, assign59560_e96693_d_n3, assign59560_e96693_d_n4, assign59560_e96693_d_n5, assign59560_e96693_d_n6, assign59560_e96693_d_n7, assign59560_e96693_d_n8, assign59560_e96693_d_n9, assign59560_e96693_d_n10, assign59560_e96693_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59560_e96687: f64 = (2.0 * locals.var_sqrtpsip);
        let assign59560_e96688: f64 = (locals.var_gam_edge / assign59560_e96687);
        let assign59560_e96689: f64 = (1.0 + assign59560_e96688);
        let assign59560_e96691: f64 = (assign59560_e96689 / locals.var_gam_edge);
        (assign59560_e96691, ((((((locals.var_gam_edge_dn3 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign59560_e96693;
        locals.var_t0_dn3 = assign59560_e96693_d_n3;
        locals.var_t0_dn4 = assign59560_e96693_d_n4;
        locals.var_t0_dn5 = assign59560_e96693_d_n5;
        locals.var_t0_dn6 = assign59560_e96693_d_n6;
        locals.var_t0_dn7 = assign59560_e96693_d_n7;
        locals.var_t0_dn8 = assign59560_e96693_d_n8;
        locals.var_t0_dn9 = assign59560_e96693_d_n9;
        locals.var_t0_dn10 = assign59560_e96693_d_n10;
        locals.var_t0_dn11 = assign59560_e96693_d_n11;

        let (assign59570_e96706, assign59570_e96706_d_n3, assign59570_e96706_d_n4, assign59570_e96706_d_n5, assign59570_e96706_d_n6, assign59570_e96706_d_n7, assign59570_e96706_d_n8, assign59570_e96706_d_n9, assign59570_e96706_d_n10, assign59570_e96706_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59570_e96701: f64 = (2.0 * locals.var_phib_n_edge);
        let assign59570_e96702: f64 = (locals.var_psip - assign59570_e96701);
        let assign59570_e96704: f64 = (assign59570_e96702 - locals.var_vs_1);
        (assign59570_e96704, ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vs_1_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vs_1_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vs_1_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vs_1_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vs_1_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vs_1_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vs_1_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vs_1_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vs_1_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59570_e96706;
        locals.var_t1_dn3 = assign59570_e96706_d_n3;
        locals.var_t1_dn4 = assign59570_e96706_d_n4;
        locals.var_t1_dn5 = assign59570_e96706_d_n5;
        locals.var_t1_dn6 = assign59570_e96706_d_n6;
        locals.var_t1_dn7 = assign59570_e96706_d_n7;
        locals.var_t1_dn8 = assign59570_e96706_d_n8;
        locals.var_t1_dn9 = assign59570_e96706_d_n9;
        locals.var_t1_dn10 = assign59570_e96706_d_n10;
        locals.var_t1_dn11 = assign59570_e96706_d_n11;

        let (assign59580_e96722, assign59580_e96722_d_n3, assign59580_e96722_d_n4, assign59580_e96722_d_n5, assign59580_e96722_d_n6, assign59580_e96722_d_n7, assign59580_e96722_d_n8, assign59580_e96722_d_n9, assign59580_e96722_d_n10, assign59580_e96722_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59580_e96714: f64 = (4.0 * locals.var_t0);
        let assign59580_e96716: f64 = (assign59580_e96714 * locals.var_sqrtpsip);
        let assign59580_e96718: f64 = (assign59580_e96716).max(1e-38);
        let assign59580_e96719: f64 = (assign59580_e96718).ln();
        let assign59580_e96720: f64 = (locals.var_t1 - assign59580_e96719);
        (assign59580_e96720, (locals.var_t1_dn3 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn4 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn5 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn6 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn7 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn8 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn9 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn10 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn11 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign59580_e96718)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59580_e96722;
        locals.var_t2_dn3 = assign59580_e96722_d_n3;
        locals.var_t2_dn4 = assign59580_e96722_d_n4;
        locals.var_t2_dn5 = assign59580_e96722_d_n5;
        locals.var_t2_dn6 = assign59580_e96722_d_n6;
        locals.var_t2_dn7 = assign59580_e96722_d_n7;
        locals.var_t2_dn8 = assign59580_e96722_d_n8;
        locals.var_t2_dn9 = assign59580_e96722_d_n9;
        locals.var_t2_dn10 = assign59580_e96722_d_n10;
        locals.var_t2_dn11 = assign59580_e96722_d_n11;

    }

    pub(super) fn stamp_transient_block_197(
        locals: &mut StampLocals,
    ) {
        let (assign59590_e96742, assign59590_e96742_d_n3, assign59590_e96742_d_n4, assign59590_e96742_d_n5, assign59590_e96742_d_n6, assign59590_e96742_d_n7, assign59590_e96742_d_n8, assign59590_e96742_d_n9, assign59590_e96742_d_n10, assign59590_e96742_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59590_e96730: f64 = (locals.var_t2 - 0.201491);
        let assign59590_e96734: f64 = (locals.var_t2 + 0.402982);
        let assign59590_e96735: f64 = (locals.var_t2 * assign59590_e96734);
        let assign59590_e96737: f64 = (assign59590_e96735 + 2.446562);
        let assign59590_e96738: f64 = (assign59590_e96737).sqrt();
        let assign59590_e96739: f64 = (assign59590_e96730 - assign59590_e96738);
        let assign59590_e96740: f64 = (0.5 * assign59590_e96739);
        (assign59590_e96740, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign59590_e96738)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59590_e96742;
        locals.var_t8_dn3 = assign59590_e96742_d_n3;
        locals.var_t8_dn4 = assign59590_e96742_d_n4;
        locals.var_t8_dn5 = assign59590_e96742_d_n5;
        locals.var_t8_dn6 = assign59590_e96742_d_n6;
        locals.var_t8_dn7 = assign59590_e96742_d_n7;
        locals.var_t8_dn8 = assign59590_e96742_d_n8;
        locals.var_t8_dn9 = assign59590_e96742_d_n9;
        locals.var_t8_dn10 = assign59590_e96742_d_n10;
        locals.var_t8_dn11 = assign59590_e96742_d_n11;

        let (assign59600_e96749, assign59600_e96749_d_n3, assign59600_e96749_d_n4, assign59600_e96749_d_n5, assign59600_e96749_d_n6, assign59600_e96749_d_n7, assign59600_e96749_d_n8, assign59600_e96749_d_n9, assign59600_e96749_d_n10, assign59600_e96749_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign59600_e96749;
        locals.var_sqrtpsisa_dn3 = assign59600_e96749_d_n3;
        locals.var_sqrtpsisa_dn4 = assign59600_e96749_d_n4;
        locals.var_sqrtpsisa_dn5 = assign59600_e96749_d_n5;
        locals.var_sqrtpsisa_dn6 = assign59600_e96749_d_n6;
        locals.var_sqrtpsisa_dn7 = assign59600_e96749_d_n7;
        locals.var_sqrtpsisa_dn8 = assign59600_e96749_d_n8;
        locals.var_sqrtpsisa_dn9 = assign59600_e96749_d_n9;
        locals.var_sqrtpsisa_dn10 = assign59600_e96749_d_n10;
        locals.var_sqrtpsisa_dn11 = assign59600_e96749_d_n11;

        let assign59610_e96752: f64 = (-68.0);
        let assign59610_e96753: f64 = if locals.var_t8 <= assign59610_e96752 { 1.0 } else { 0.0 };
        locals.var_guard871 = assign59610_e96753;

        let (assign59620_e96763, assign59620_e96763_d_n3, assign59620_e96763_d_n4, assign59620_e96763_d_n5, assign59620_e96763_d_n6, assign59620_e96763_d_n7, assign59620_e96763_d_n8, assign59620_e96763_d_n9, assign59620_e96763_d_n10, assign59620_e96763_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign59620_e96761: f64 = (-100.0);
        (assign59620_e96761, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59620_e96763;
        locals.var_t4_dn3 = assign59620_e96763_d_n3;
        locals.var_t4_dn4 = assign59620_e96763_d_n4;
        locals.var_t4_dn5 = assign59620_e96763_d_n5;
        locals.var_t4_dn6 = assign59620_e96763_d_n6;
        locals.var_t4_dn7 = assign59620_e96763_d_n7;
        locals.var_t4_dn8 = assign59620_e96763_d_n8;
        locals.var_t4_dn9 = assign59620_e96763_d_n9;
        locals.var_t4_dn10 = assign59620_e96763_d_n10;
        locals.var_t4_dn11 = assign59620_e96763_d_n11;

        let (assign59630_e96772, assign59630_e96772_d_n3, assign59630_e96772_d_n4, assign59630_e96772_d_n5, assign59630_e96772_d_n6, assign59630_e96772_d_n7, assign59630_e96772_d_n8, assign59630_e96772_d_n9, assign59630_e96772_d_n10, assign59630_e96772_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59630_e96772;
        locals.var_t5_dn3 = assign59630_e96772_d_n3;
        locals.var_t5_dn4 = assign59630_e96772_d_n4;
        locals.var_t5_dn5 = assign59630_e96772_d_n5;
        locals.var_t5_dn6 = assign59630_e96772_d_n6;
        locals.var_t5_dn7 = assign59630_e96772_d_n7;
        locals.var_t5_dn8 = assign59630_e96772_d_n8;
        locals.var_t5_dn9 = assign59630_e96772_d_n9;
        locals.var_t5_dn10 = assign59630_e96772_d_n10;
        locals.var_t5_dn11 = assign59630_e96772_d_n11;

        let assign59640_e96777: f64 = (0.5 * locals.var_t5);
        let assign59640_e96778: f64 = (locals.var_t4 - assign59640_e96777);
        let assign59640_e96779: f64 = if locals.var_t8 < assign59640_e96778 { 1.0 } else { 0.0 };
        locals.var_guard872 = assign59640_e96779;

        let (assign59650_e96791, assign59650_e96791_d_n3, assign59650_e96791_d_n4, assign59650_e96791_d_n5, assign59650_e96791_d_n6, assign59650_e96791_d_n7, assign59650_e96791_d_n8, assign59650_e96791_d_n9, assign59650_e96791_d_n10, assign59650_e96791_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign59650_e96789: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59650_e96789, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59650_e96791;
        locals.var_t3_dn3 = assign59650_e96791_d_n3;
        locals.var_t3_dn4 = assign59650_e96791_d_n4;
        locals.var_t3_dn5 = assign59650_e96791_d_n5;
        locals.var_t3_dn6 = assign59650_e96791_d_n6;
        locals.var_t3_dn7 = assign59650_e96791_d_n7;
        locals.var_t3_dn8 = assign59650_e96791_d_n8;
        locals.var_t3_dn9 = assign59650_e96791_d_n9;
        locals.var_t3_dn10 = assign59650_e96791_d_n10;
        locals.var_t3_dn11 = assign59650_e96791_d_n11;

        let assign59660_e96796: f64 = (0.5 * locals.var_t5);
        let assign59660_e96797: f64 = (locals.var_t4 + assign59660_e96796);
        let assign59660_e96798: f64 = if locals.var_t8 > assign59660_e96797 { 1.0 } else { 0.0 };
        locals.var_guard873 = assign59660_e96798;

        let (assign59670_e96813, assign59670_e96813_d_n3, assign59670_e96813_d_n4, assign59670_e96813_d_n5, assign59670_e96813_d_n6, assign59670_e96813_d_n7, assign59670_e96813_d_n8, assign59670_e96813_d_n9, assign59670_e96813_d_n10, assign59670_e96813_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign59670_e96811: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59670_e96811, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59670_e96813;
        locals.var_t3_dn3 = assign59670_e96813_d_n3;
        locals.var_t3_dn4 = assign59670_e96813_d_n4;
        locals.var_t3_dn5 = assign59670_e96813_d_n5;
        locals.var_t3_dn6 = assign59670_e96813_d_n6;
        locals.var_t3_dn7 = assign59670_e96813_d_n7;
        locals.var_t3_dn8 = assign59670_e96813_d_n8;
        locals.var_t3_dn9 = assign59670_e96813_d_n9;
        locals.var_t3_dn10 = assign59670_e96813_d_n10;
        locals.var_t3_dn11 = assign59670_e96813_d_n11;

        let (assign59680_e96832, assign59680_e96832_d_n3, assign59680_e96832_d_n4, assign59680_e96832_d_n5, assign59680_e96832_d_n6, assign59680_e96832_d_n7, assign59680_e96832_d_n8, assign59680_e96832_d_n9, assign59680_e96832_d_n10, assign59680_e96832_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign59680_e96828: f64 = (locals.var_t8 - locals.var_t4);
        let assign59680_e96830: f64 = (assign59680_e96828 / locals.var_t5);
        (assign59680_e96830, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59680_e96832;
        locals.var_t2_dn3 = assign59680_e96832_d_n3;
        locals.var_t2_dn4 = assign59680_e96832_d_n4;
        locals.var_t2_dn5 = assign59680_e96832_d_n5;
        locals.var_t2_dn6 = assign59680_e96832_d_n6;
        locals.var_t2_dn7 = assign59680_e96832_d_n7;
        locals.var_t2_dn8 = assign59680_e96832_d_n8;
        locals.var_t2_dn9 = assign59680_e96832_d_n9;
        locals.var_t2_dn10 = assign59680_e96832_d_n10;
        locals.var_t2_dn11 = assign59680_e96832_d_n11;

        let (assign59690_e96849, assign59690_e96849_d_n3, assign59690_e96849_d_n4, assign59690_e96849_d_n5, assign59690_e96849_d_n6, assign59690_e96849_d_n7, assign59690_e96849_d_n8, assign59690_e96849_d_n9, assign59690_e96849_d_n10, assign59690_e96849_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign59690_e96847: f64 = (locals.var_t2 * locals.var_t2);
        (assign59690_e96847, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign59690_e96849;
        locals.var_t6_dn3 = assign59690_e96849_d_n3;
        locals.var_t6_dn4 = assign59690_e96849_d_n4;
        locals.var_t6_dn5 = assign59690_e96849_d_n5;
        locals.var_t6_dn6 = assign59690_e96849_d_n6;
        locals.var_t6_dn7 = assign59690_e96849_d_n7;
        locals.var_t6_dn8 = assign59690_e96849_d_n8;
        locals.var_t6_dn9 = assign59690_e96849_d_n9;
        locals.var_t6_dn10 = assign59690_e96849_d_n10;
        locals.var_t6_dn11 = assign59690_e96849_d_n11;

        let (assign59700_e96887, assign59700_e96887_d_n3, assign59700_e96887_d_n4, assign59700_e96887_d_n5, assign59700_e96887_d_n6, assign59700_e96887_d_n7, assign59700_e96887_d_n8, assign59700_e96887_d_n9, assign59700_e96887_d_n10, assign59700_e96887_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign59700_e96866: f64 = (5.0 / 64.0);
        let assign59700_e96869: f64 = (0.5 * locals.var_t2);
        let assign59700_e96870: f64 = (assign59700_e96866 + assign59700_e96869);
        let assign59700_e96874: f64 = (15.0 / 16.0);
        let assign59700_e96878: f64 = (1.25 - locals.var_t6);
        let assign59700_e96879: f64 = (locals.var_t6 * assign59700_e96878);
        let assign59700_e96880: f64 = (assign59700_e96874 - assign59700_e96879);
        let assign59700_e96881: f64 = (locals.var_t6 * assign59700_e96880);
        let assign59700_e96882: f64 = (assign59700_e96870 + assign59700_e96881);
        let assign59700_e96883: f64 = (locals.var_t5 * assign59700_e96882);
        let assign59700_e96884: f64 = (locals.var_t4 + assign59700_e96883);
        let assign59700_e96885: f64 = { let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59700_e96885, ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59700_e96887;
        locals.var_t3_dn3 = assign59700_e96887_d_n3;
        locals.var_t3_dn4 = assign59700_e96887_d_n4;
        locals.var_t3_dn5 = assign59700_e96887_d_n5;
        locals.var_t3_dn6 = assign59700_e96887_d_n6;
        locals.var_t3_dn7 = assign59700_e96887_d_n7;
        locals.var_t3_dn8 = assign59700_e96887_d_n8;
        locals.var_t3_dn9 = assign59700_e96887_d_n9;
        locals.var_t3_dn10 = assign59700_e96887_d_n10;
        locals.var_t3_dn11 = assign59700_e96887_d_n11;

        let (assign59710_e96919, assign59710_e96919_d_n3, assign59710_e96919_d_n4, assign59710_e96919_d_n5, assign59710_e96919_d_n6, assign59710_e96919_d_n7, assign59710_e96919_d_n8, assign59710_e96919_d_n9, assign59710_e96919_d_n10, assign59710_e96919_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign59710_e96897: f64 = (1.0 + locals.var_t1);
        let assign59710_e96899: f64 = (assign59710_e96897 - locals.var_t8);
        let assign59710_e96902: f64 = (2.0 * locals.var_t0);
        let assign59710_e96905: f64 = (locals.var_t3 * 2.0);
        let assign59710_e96907: f64 = (assign59710_e96905 * locals.var_t0);
        let assign59710_e96910: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign59710_e96911: f64 = (assign59710_e96907 + assign59710_e96910);
        let assign59710_e96912: f64 = (assign59710_e96902 * assign59710_e96911);
        let assign59710_e96914: f64 = (assign59710_e96912).max(1e-38);
        let assign59710_e96915: f64 = (assign59710_e96914).ln();
        let assign59710_e96916: f64 = (assign59710_e96899 - assign59710_e96915);
        let assign59710_e96917: f64 = (locals.var_t3 * assign59710_e96916);
        (assign59710_e96917, ((locals.var_t3_dn3 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn4 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn5 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn6 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn7 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn8 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn9 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn10 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn11 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59710_e96914)))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11,)
    }
};
        locals.var_qs_edge = assign59710_e96919;
        locals.var_qs_edge_dn3 = assign59710_e96919_d_n3;
        locals.var_qs_edge_dn4 = assign59710_e96919_d_n4;
        locals.var_qs_edge_dn5 = assign59710_e96919_d_n5;
        locals.var_qs_edge_dn6 = assign59710_e96919_d_n6;
        locals.var_qs_edge_dn7 = assign59710_e96919_d_n7;
        locals.var_qs_edge_dn8 = assign59710_e96919_d_n8;
        locals.var_qs_edge_dn9 = assign59710_e96919_d_n9;
        locals.var_qs_edge_dn10 = assign59710_e96919_d_n10;
        locals.var_qs_edge_dn11 = assign59710_e96919_d_n11;

        let (assign59720_e96930, assign59720_e96930_d_n3, assign59720_e96930_d_n4, assign59720_e96930_d_n5, assign59720_e96930_d_n6, assign59720_e96930_d_n7, assign59720_e96930_d_n8, assign59720_e96930_d_n9, assign59720_e96930_d_n10, assign59720_e96930_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59720_e96928: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59720_e96928, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59720_e96930;
        locals.var_t3_dn3 = assign59720_e96930_d_n3;
        locals.var_t3_dn4 = assign59720_e96930_d_n4;
        locals.var_t3_dn5 = assign59720_e96930_d_n5;
        locals.var_t3_dn6 = assign59720_e96930_d_n6;
        locals.var_t3_dn7 = assign59720_e96930_d_n7;
        locals.var_t3_dn8 = assign59720_e96930_d_n8;
        locals.var_t3_dn9 = assign59720_e96930_d_n9;
        locals.var_t3_dn10 = assign59720_e96930_d_n10;
        locals.var_t3_dn11 = assign59720_e96930_d_n11;

        let (assign59730_e96942, assign59730_e96942_d_n3, assign59730_e96942_d_n4, assign59730_e96942_d_n5, assign59730_e96942_d_n6, assign59730_e96942_d_n7, assign59730_e96942_d_n8, assign59730_e96942_d_n9, assign59730_e96942_d_n10, assign59730_e96942_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59730_e96940: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign59730_e96940, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign59730_e96942;
        locals.var_sqrtpsisainv_dn3 = assign59730_e96942_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign59730_e96942_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign59730_e96942_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign59730_e96942_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign59730_e96942_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign59730_e96942_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign59730_e96942_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign59730_e96942_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign59730_e96942_d_n11;

        let (assign59740_e96975, assign59740_e96975_d_n3, assign59740_e96975_d_n4, assign59740_e96975_d_n5, assign59740_e96975_d_n6, assign59740_e96975_d_n7, assign59740_e96975_d_n8, assign59740_e96975_d_n9, assign59740_e96975_d_n10, assign59740_e96975_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59740_e96952: f64 = (2.0 * locals.var_t3);
        let assign59740_e96955: f64 = (locals.var_t3 * 2.0);
        let assign59740_e96957: f64 = (assign59740_e96955 * locals.var_t0);
        let assign59740_e96960: f64 = (locals.var_t3 * 2.0);
        let assign59740_e96962: f64 = (assign59740_e96960 * locals.var_t0);
        let assign59740_e96965: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign59740_e96966: f64 = (assign59740_e96962 + assign59740_e96965);
        let assign59740_e96967: f64 = (assign59740_e96957 * assign59740_e96966);
        let assign59740_e96969: f64 = (assign59740_e96967).max(1e-38);
        let assign59740_e96970: f64 = (assign59740_e96969).ln();
        let assign59740_e96971: f64 = (assign59740_e96952 + assign59740_e96970);
        let assign59740_e96973: f64 = (assign59740_e96971 - locals.var_t1);
        (assign59740_e96973, (((2.0 * locals.var_t3_dn3) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn3)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn4)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn5)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn6)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn7)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn8)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn9)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn10)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn11)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59740_e96975;
        locals.var_t4_dn3 = assign59740_e96975_d_n3;
        locals.var_t4_dn4 = assign59740_e96975_d_n4;
        locals.var_t4_dn5 = assign59740_e96975_d_n5;
        locals.var_t4_dn6 = assign59740_e96975_d_n6;
        locals.var_t4_dn7 = assign59740_e96975_d_n7;
        locals.var_t4_dn8 = assign59740_e96975_d_n8;
        locals.var_t4_dn9 = assign59740_e96975_d_n9;
        locals.var_t4_dn10 = assign59740_e96975_d_n10;
        locals.var_t4_dn11 = assign59740_e96975_d_n11;

        let (assign59750_e96999, assign59750_e96999_d_n3, assign59750_e96999_d_n4, assign59750_e96999_d_n5, assign59750_e96999_d_n6, assign59750_e96999_d_n7, assign59750_e96999_d_n8, assign59750_e96999_d_n9, assign59750_e96999_d_n10, assign59750_e96999_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59750_e96986: f64 = (1.0 / locals.var_t3);
        let assign59750_e96987: f64 = (2.0 + assign59750_e96986);
        let assign59750_e96990: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59750_e96993: f64 = (locals.var_t0 * locals.var_t3);
        let assign59750_e96995: f64 = (assign59750_e96993 + locals.var_sqrtpsisa);
        let assign59750_e96996: f64 = (assign59750_e96990 / assign59750_e96995);
        let assign59750_e96997: f64 = (assign59750_e96987 + assign59750_e96996);
        (assign59750_e96997, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59750_e96995 * assign59750_e96995))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59750_e96999;
        locals.var_t5_dn3 = assign59750_e96999_d_n3;
        locals.var_t5_dn4 = assign59750_e96999_d_n4;
        locals.var_t5_dn5 = assign59750_e96999_d_n5;
        locals.var_t5_dn6 = assign59750_e96999_d_n6;
        locals.var_t5_dn7 = assign59750_e96999_d_n7;
        locals.var_t5_dn8 = assign59750_e96999_d_n8;
        locals.var_t5_dn9 = assign59750_e96999_d_n9;
        locals.var_t5_dn10 = assign59750_e96999_d_n10;
        locals.var_t5_dn11 = assign59750_e96999_d_n11;

        let (assign59760_e97013, assign59760_e97013_d_n3, assign59760_e97013_d_n4, assign59760_e97013_d_n5, assign59760_e97013_d_n6, assign59760_e97013_d_n7, assign59760_e97013_d_n8, assign59760_e97013_d_n9, assign59760_e97013_d_n10, assign59760_e97013_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59760_e97010: f64 = (locals.var_t4 / locals.var_t5);
        let assign59760_e97011: f64 = (locals.var_t3 - assign59760_e97010);
        (assign59760_e97011, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59760_e97013;
        locals.var_t3_dn3 = assign59760_e97013_d_n3;
        locals.var_t3_dn4 = assign59760_e97013_d_n4;
        locals.var_t3_dn5 = assign59760_e97013_d_n5;
        locals.var_t3_dn6 = assign59760_e97013_d_n6;
        locals.var_t3_dn7 = assign59760_e97013_d_n7;
        locals.var_t3_dn8 = assign59760_e97013_d_n8;
        locals.var_t3_dn9 = assign59760_e97013_d_n9;
        locals.var_t3_dn10 = assign59760_e97013_d_n10;
        locals.var_t3_dn11 = assign59760_e97013_d_n11;

        let (assign59770_e97046, assign59770_e97046_d_n3, assign59770_e97046_d_n4, assign59770_e97046_d_n5, assign59770_e97046_d_n6, assign59770_e97046_d_n7, assign59770_e97046_d_n8, assign59770_e97046_d_n9, assign59770_e97046_d_n10, assign59770_e97046_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59770_e97023: f64 = (2.0 * locals.var_t3);
        let assign59770_e97026: f64 = (locals.var_t3 * 2.0);
        let assign59770_e97028: f64 = (assign59770_e97026 * locals.var_t0);
        let assign59770_e97031: f64 = (locals.var_t3 * 2.0);
        let assign59770_e97033: f64 = (assign59770_e97031 * locals.var_t0);
        let assign59770_e97036: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign59770_e97037: f64 = (assign59770_e97033 + assign59770_e97036);
        let assign59770_e97038: f64 = (assign59770_e97028 * assign59770_e97037);
        let assign59770_e97040: f64 = (assign59770_e97038).max(1e-38);
        let assign59770_e97041: f64 = (assign59770_e97040).ln();
        let assign59770_e97042: f64 = (assign59770_e97023 + assign59770_e97041);
        let assign59770_e97044: f64 = (assign59770_e97042 - locals.var_t1);
        (assign59770_e97044, (((2.0 * locals.var_t3_dn3) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn3)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn4)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn5)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn6)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn7)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn8)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn9)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn10)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn11)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59770_e97046;
        locals.var_t4_dn3 = assign59770_e97046_d_n3;
        locals.var_t4_dn4 = assign59770_e97046_d_n4;
        locals.var_t4_dn5 = assign59770_e97046_d_n5;
        locals.var_t4_dn6 = assign59770_e97046_d_n6;
        locals.var_t4_dn7 = assign59770_e97046_d_n7;
        locals.var_t4_dn8 = assign59770_e97046_d_n8;
        locals.var_t4_dn9 = assign59770_e97046_d_n9;
        locals.var_t4_dn10 = assign59770_e97046_d_n10;
        locals.var_t4_dn11 = assign59770_e97046_d_n11;

        let (assign59780_e97070, assign59780_e97070_d_n3, assign59780_e97070_d_n4, assign59780_e97070_d_n5, assign59780_e97070_d_n6, assign59780_e97070_d_n7, assign59780_e97070_d_n8, assign59780_e97070_d_n9, assign59780_e97070_d_n10, assign59780_e97070_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59780_e97057: f64 = (1.0 / locals.var_t3);
        let assign59780_e97058: f64 = (2.0 + assign59780_e97057);
        let assign59780_e97061: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59780_e97064: f64 = (locals.var_t0 * locals.var_t3);
        let assign59780_e97066: f64 = (assign59780_e97064 + locals.var_sqrtpsisa);
        let assign59780_e97067: f64 = (assign59780_e97061 / assign59780_e97066);
        let assign59780_e97068: f64 = (assign59780_e97058 + assign59780_e97067);
        (assign59780_e97068, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59780_e97066 * assign59780_e97066))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59780_e97070;
        locals.var_t5_dn3 = assign59780_e97070_d_n3;
        locals.var_t5_dn4 = assign59780_e97070_d_n4;
        locals.var_t5_dn5 = assign59780_e97070_d_n5;
        locals.var_t5_dn6 = assign59780_e97070_d_n6;
        locals.var_t5_dn7 = assign59780_e97070_d_n7;
        locals.var_t5_dn8 = assign59780_e97070_d_n8;
        locals.var_t5_dn9 = assign59780_e97070_d_n9;
        locals.var_t5_dn10 = assign59780_e97070_d_n10;
        locals.var_t5_dn11 = assign59780_e97070_d_n11;

        let (assign59790_e97098, assign59790_e97098_d_n3, assign59790_e97098_d_n4, assign59790_e97098_d_n5, assign59790_e97098_d_n6, assign59790_e97098_d_n7, assign59790_e97098_d_n8, assign59790_e97098_d_n9, assign59790_e97098_d_n10, assign59790_e97098_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59790_e97080: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59790_e97083: f64 = (locals.var_t0 * locals.var_t3);
        let assign59790_e97085: f64 = (assign59790_e97083 + locals.var_sqrtpsisa);
        let assign59790_e97086: f64 = (assign59790_e97080 / assign59790_e97085);
        let assign59790_e97089: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59790_e97092: f64 = (locals.var_t0 * locals.var_t3);
        let assign59790_e97094: f64 = (assign59790_e97092 + locals.var_sqrtpsisa);
        let assign59790_e97095: f64 = (assign59790_e97089 / assign59790_e97094);
        let assign59790_e97096: f64 = (assign59790_e97086 * assign59790_e97095);
        (assign59790_e97096, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59790_e97094 * assign59790_e97094)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign59790_e97098;
        locals.var_t6_dn3 = assign59790_e97098_d_n3;
        locals.var_t6_dn4 = assign59790_e97098_d_n4;
        locals.var_t6_dn5 = assign59790_e97098_d_n5;
        locals.var_t6_dn6 = assign59790_e97098_d_n6;
        locals.var_t6_dn7 = assign59790_e97098_d_n7;
        locals.var_t6_dn8 = assign59790_e97098_d_n8;
        locals.var_t6_dn9 = assign59790_e97098_d_n9;
        locals.var_t6_dn10 = assign59790_e97098_d_n10;
        locals.var_t6_dn11 = assign59790_e97098_d_n11;

        let (assign59800_e97131, assign59800_e97131_d_n3, assign59800_e97131_d_n4, assign59800_e97131_d_n5, assign59800_e97131_d_n6, assign59800_e97131_d_n7, assign59800_e97131_d_n8, assign59800_e97131_d_n9, assign59800_e97131_d_n10, assign59800_e97131_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign59800_e97108: f64 = (1.0 * __rspice_inv_cse_0);
        let assign59800_e97111: f64 = (1.0 * __rspice_inv_cse_0);
        let assign59800_e97112: f64 = (assign59800_e97108 * assign59800_e97111);
        let assign59800_e97113: f64 = (-assign59800_e97112);
        let assign59800_e97117: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign59800_e97119: f64 = (assign59800_e97117 * locals.var_sqrtpsisa);
        let assign59800_e97122: f64 = (locals.var_t0 * locals.var_t3);
        let assign59800_e97124: f64 = (assign59800_e97122 + locals.var_sqrtpsisa);
        let assign59800_e97125: f64 = (assign59800_e97119 * assign59800_e97124);
        let assign59800_e97126: f64 = (1.0 / assign59800_e97125);
        let assign59800_e97127: f64 = (assign59800_e97113 - assign59800_e97126);
        let assign59800_e97129: f64 = (assign59800_e97127 - locals.var_t6);
        (assign59800_e97129, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn3)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn4)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn5)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn6)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn7)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn8)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn9)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn10)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn11)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign59800_e97131;
        locals.var_t7_dn3 = assign59800_e97131_d_n3;
        locals.var_t7_dn4 = assign59800_e97131_d_n4;
        locals.var_t7_dn5 = assign59800_e97131_d_n5;
        locals.var_t7_dn6 = assign59800_e97131_d_n6;
        locals.var_t7_dn7 = assign59800_e97131_d_n7;
        locals.var_t7_dn8 = assign59800_e97131_d_n8;
        locals.var_t7_dn9 = assign59800_e97131_d_n9;
        locals.var_t7_dn10 = assign59800_e97131_d_n10;
        locals.var_t7_dn11 = assign59800_e97131_d_n11;

        let (assign59810_e97157, assign59810_e97157_d_n3, assign59810_e97157_d_n4, assign59810_e97157_d_n5, assign59810_e97157_d_n6, assign59810_e97157_d_n7, assign59810_e97157_d_n8, assign59810_e97157_d_n9, assign59810_e97157_d_n10, assign59810_e97157_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59810_e97142: f64 = (locals.var_t4 / locals.var_t5);
        let assign59810_e97146: f64 = (locals.var_t4 * locals.var_t7);
        let assign59810_e97149: f64 = (2.0 * locals.var_t5);
        let assign59810_e97151: f64 = (assign59810_e97149 * locals.var_t5);
        let assign59810_e97152: f64 = (assign59810_e97146 / assign59810_e97151);
        let assign59810_e97153: f64 = (1.0 + assign59810_e97152);
        let assign59810_e97154: f64 = (assign59810_e97142 * assign59810_e97153);
        let assign59810_e97155: f64 = (locals.var_t3 - assign59810_e97154);
        (assign59810_e97155, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn3)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn4)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn5)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn6)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn7)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn8)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn9)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn10)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn11)))) / (assign59810_e97151 * assign59810_e97151))))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11,)
    }
};
        locals.var_qs_edge = assign59810_e97157;
        locals.var_qs_edge_dn3 = assign59810_e97157_d_n3;
        locals.var_qs_edge_dn4 = assign59810_e97157_d_n4;
        locals.var_qs_edge_dn5 = assign59810_e97157_d_n5;
        locals.var_qs_edge_dn6 = assign59810_e97157_d_n6;
        locals.var_qs_edge_dn7 = assign59810_e97157_d_n7;
        locals.var_qs_edge_dn8 = assign59810_e97157_d_n8;
        locals.var_qs_edge_dn9 = assign59810_e97157_d_n9;
        locals.var_qs_edge_dn10 = assign59810_e97157_d_n10;
        locals.var_qs_edge_dn11 = assign59810_e97157_d_n11;

        let (assign59820_e97172, assign59820_e97172_d_n3, assign59820_e97172_d_n4, assign59820_e97172_d_n5, assign59820_e97172_d_n6, assign59820_e97172_d_n7, assign59820_e97172_d_n8, assign59820_e97172_d_n9, assign59820_e97172_d_n10, assign59820_e97172_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59820_e97164: f64 = (2.0 * locals.var_nvt);
        let assign59820_e97166: f64 = (assign59820_e97164 * locals.var_qs_edge);
        let assign59820_e97169: f64 = (2.0 * locals.var_nvt);
        let assign59820_e97170: f64 = (assign59820_e97166 + assign59820_e97169);
        (assign59820_e97170, ((((2.0 * locals.var_nvt_dn3) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn3)) + (2.0 * locals.var_nvt_dn3)), ((((2.0 * locals.var_nvt_dn4) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn4)) + (2.0 * locals.var_nvt_dn4)), ((((2.0 * locals.var_nvt_dn5) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn5)) + (2.0 * locals.var_nvt_dn5)), ((((2.0 * locals.var_nvt_dn6) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn6)) + (2.0 * locals.var_nvt_dn6)), ((((2.0 * locals.var_nvt_dn7) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn7)) + (2.0 * locals.var_nvt_dn7)), ((((2.0 * locals.var_nvt_dn8) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn8)) + (2.0 * locals.var_nvt_dn8)), ((((2.0 * locals.var_nvt_dn9) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn9)) + (2.0 * locals.var_nvt_dn9)), ((((2.0 * locals.var_nvt_dn10) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn10)) + (2.0 * locals.var_nvt_dn10)), ((((2.0 * locals.var_nvt_dn11) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn11)) + (2.0 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11,)
    }
};
        locals.var_vdsatedge = assign59820_e97172;
        locals.var_vdsatedge_dn3 = assign59820_e97172_d_n3;
        locals.var_vdsatedge_dn4 = assign59820_e97172_d_n4;
        locals.var_vdsatedge_dn5 = assign59820_e97172_d_n5;
        locals.var_vdsatedge_dn6 = assign59820_e97172_d_n6;
        locals.var_vdsatedge_dn7 = assign59820_e97172_d_n7;
        locals.var_vdsatedge_dn8 = assign59820_e97172_d_n8;
        locals.var_vdsatedge_dn9 = assign59820_e97172_d_n9;
        locals.var_vdsatedge_dn10 = assign59820_e97172_d_n10;
        locals.var_vdsatedge_dn11 = assign59820_e97172_d_n11;

    }

    pub(super) fn stamp_transient_block_198(
        locals: &mut StampLocals,
    ) {
        let (assign59830_e97179, assign59830_e97179_d_n3, assign59830_e97179_d_n4, assign59830_e97179_d_n5, assign59830_e97179_d_n6, assign59830_e97179_d_n7, assign59830_e97179_d_n8, assign59830_e97179_d_n9, assign59830_e97179_d_n10, assign59830_e97179_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11,)
    }
};
        locals.var_vdsatedge_1 = assign59830_e97179;
        locals.var_vdsatedge_1_dn3 = assign59830_e97179_d_n3;
        locals.var_vdsatedge_1_dn4 = assign59830_e97179_d_n4;
        locals.var_vdsatedge_1_dn5 = assign59830_e97179_d_n5;
        locals.var_vdsatedge_1_dn6 = assign59830_e97179_d_n6;
        locals.var_vdsatedge_1_dn7 = assign59830_e97179_d_n7;
        locals.var_vdsatedge_1_dn8 = assign59830_e97179_d_n8;
        locals.var_vdsatedge_1_dn9 = assign59830_e97179_d_n9;
        locals.var_vdsatedge_1_dn10 = assign59830_e97179_d_n10;
        locals.var_vdsatedge_1_dn11 = assign59830_e97179_d_n11;

        let (assign59840_e97188, assign59840_e97188_d_n3, assign59840_e97188_d_n4, assign59840_e97188_d_n5, assign59840_e97188_d_n6, assign59840_e97188_d_n7, assign59840_e97188_d_n8, assign59840_e97188_d_n9, assign59840_e97188_d_n10, assign59840_e97188_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59840_e97186: f64 = (locals.var_vdsatedge_1 + locals.var_vs);
        (assign59840_e97186, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, (locals.var_vdsatedge_1_dn6 + locals.var_vs_dn6), (locals.var_vdsatedge_1_dn7 + locals.var_vs_dn7), locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, (locals.var_vdsatedge_1_dn10 + locals.var_vs_dn10), locals.var_vdsatedge_1_dn11,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11,)
    }
};
        locals.var_vdsatedge_1 = assign59840_e97188;
        locals.var_vdsatedge_1_dn3 = assign59840_e97188_d_n3;
        locals.var_vdsatedge_1_dn4 = assign59840_e97188_d_n4;
        locals.var_vdsatedge_1_dn5 = assign59840_e97188_d_n5;
        locals.var_vdsatedge_1_dn6 = assign59840_e97188_d_n6;
        locals.var_vdsatedge_1_dn7 = assign59840_e97188_d_n7;
        locals.var_vdsatedge_1_dn8 = assign59840_e97188_d_n8;
        locals.var_vdsatedge_1_dn9 = assign59840_e97188_d_n9;
        locals.var_vdsatedge_1_dn10 = assign59840_e97188_d_n10;
        locals.var_vdsatedge_1_dn11 = assign59840_e97188_d_n11;

        let (assign59850_e97220, assign59850_e97220_d_n3, assign59850_e97220_d_n4, assign59850_e97220_d_n5, assign59850_e97220_d_n6, assign59850_e97220_d_n7, assign59850_e97220_d_n8, assign59850_e97220_d_n9, assign59850_e97220_d_n10, assign59850_e97220_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59850_e97196: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign59850_e97198: f64 = assign59850_e97196;
        let assign59850_e97201: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign59850_e97203: f64 = assign59850_e97201;
        let assign59850_e97206: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign59850_e97208: f64 = assign59850_e97206;
        let assign59850_e97209: f64 = (assign59850_e97203 * assign59850_e97208);
        let assign59850_e97212: f64 = (0.25 * 0.001);
        let assign59850_e97214: f64 = (assign59850_e97212 * 0.001);
        let assign59850_e97215: f64 = (assign59850_e97209 + assign59850_e97214);
        let assign59850_e97216: f64 = (assign59850_e97215).sqrt();
        let assign59850_e97217: f64 = (assign59850_e97198 + assign59850_e97216);
        let assign59850_e97218: f64 = (0.5 * assign59850_e97217);
        (assign59850_e97218, (0.5 * (locals.var_vdsatedge_1_dn3 + (((locals.var_vdsatedge_1_dn3 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn3)) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn4 + (((locals.var_vdsatedge_1_dn4 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn4)) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn5 + (((locals.var_vdsatedge_1_dn5 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn5)) / (2.0 * assign59850_e97216)))), (0.5 * ((locals.var_vdsatedge_1_dn6 - locals.var_vs_dn6) + ((((locals.var_vdsatedge_1_dn6 - locals.var_vs_dn6) * assign59850_e97208) + (assign59850_e97203 * (locals.var_vdsatedge_1_dn6 - locals.var_vs_dn6))) / (2.0 * assign59850_e97216)))), (0.5 * ((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) + ((((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) * assign59850_e97208) + (assign59850_e97203 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn8 + (((locals.var_vdsatedge_1_dn8 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn8)) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn9 + (((locals.var_vdsatedge_1_dn9 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn9)) / (2.0 * assign59850_e97216)))), (0.5 * ((locals.var_vdsatedge_1_dn10 - locals.var_vs_dn10) + ((((locals.var_vdsatedge_1_dn10 - locals.var_vs_dn10) * assign59850_e97208) + (assign59850_e97203 * (locals.var_vdsatedge_1_dn10 - locals.var_vs_dn10))) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn11 + (((locals.var_vdsatedge_1_dn11 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn11)) / (2.0 * assign59850_e97216)))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11,)
    }
};
        locals.var_vdssate = assign59850_e97220;
        locals.var_vdssate_dn3 = assign59850_e97220_d_n3;
        locals.var_vdssate_dn4 = assign59850_e97220_d_n4;
        locals.var_vdssate_dn5 = assign59850_e97220_d_n5;
        locals.var_vdssate_dn6 = assign59850_e97220_d_n6;
        locals.var_vdssate_dn7 = assign59850_e97220_d_n7;
        locals.var_vdssate_dn8 = assign59850_e97220_d_n8;
        locals.var_vdssate_dn9 = assign59850_e97220_d_n9;
        locals.var_vdssate_dn10 = assign59850_e97220_d_n10;
        locals.var_vdssate_dn11 = assign59850_e97220_d_n11;

        let (assign59860_e97235, assign59860_e97235_d_n3, assign59860_e97235_d_n4, assign59860_e97235_d_n5, assign59860_e97235_d_n6, assign59860_e97235_d_n7, assign59860_e97235_d_n8, assign59860_e97235_d_n9, assign59860_e97235_d_n10, assign59860_e97235_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59860_e97227: f64 = (locals.var_vds / locals.var_vdssate);
        let assign59860_e97229: f64 = (assign59860_e97227 + 1e-6);
        let assign59860_e97232: f64 = (1.0 / locals.var_delta_t);
        let assign59860_e97233: f64 = (assign59860_e97229).powf(assign59860_e97232);
        (assign59860_e97233, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn5) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn5) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((locals.var_vds_dn6 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn6)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((locals.var_vds_dn6 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn6)) / (locals.var_vdssate * locals.var_vdssate)) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((locals.var_vds_dn10 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn10)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((locals.var_vds_dn10 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn10)) / (locals.var_vdssate * locals.var_vdssate)) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn11) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn11) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign59860_e97235;
        locals.var_t7_dn3 = assign59860_e97235_d_n3;
        locals.var_t7_dn4 = assign59860_e97235_d_n4;
        locals.var_t7_dn5 = assign59860_e97235_d_n5;
        locals.var_t7_dn6 = assign59860_e97235_d_n6;
        locals.var_t7_dn7 = assign59860_e97235_d_n7;
        locals.var_t7_dn8 = assign59860_e97235_d_n8;
        locals.var_t7_dn9 = assign59860_e97235_d_n9;
        locals.var_t7_dn10 = assign59860_e97235_d_n10;
        locals.var_t7_dn11 = assign59860_e97235_d_n11;

        let (assign59870_e97247, assign59870_e97247_d_n3, assign59870_e97247_d_n4, assign59870_e97247_d_n5, assign59870_e97247_d_n6, assign59870_e97247_d_n7, assign59870_e97247_d_n8, assign59870_e97247_d_n9, assign59870_e97247_d_n10, assign59870_e97247_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59870_e97242: f64 = (1.0 + locals.var_t7);
        let assign59870_e97244: f64 = (-locals.var_delta_t);
        let assign59870_e97245: f64 = (assign59870_e97242).powf(assign59870_e97244);
        (assign59870_e97245, if (-locals.var_delta_t_dn3) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn3)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn3) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn3 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn4)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn4) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn4 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn5)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn5) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn5 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn6)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn6) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn6 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn7)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn7) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn7 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn8)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn8) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn8 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn9)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn9) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn9 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn10)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn10) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn10 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn11)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn11) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn11 / assign59870_e97242)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59870_e97247;
        locals.var_t8_dn3 = assign59870_e97247_d_n3;
        locals.var_t8_dn4 = assign59870_e97247_d_n4;
        locals.var_t8_dn5 = assign59870_e97247_d_n5;
        locals.var_t8_dn6 = assign59870_e97247_d_n6;
        locals.var_t8_dn7 = assign59870_e97247_d_n7;
        locals.var_t8_dn8 = assign59870_e97247_d_n8;
        locals.var_t8_dn9 = assign59870_e97247_d_n9;
        locals.var_t8_dn10 = assign59870_e97247_d_n10;
        locals.var_t8_dn11 = assign59870_e97247_d_n11;

        let (assign59880_e97256, assign59880_e97256_d_n3, assign59880_e97256_d_n4, assign59880_e97256_d_n5, assign59880_e97256_d_n6, assign59880_e97256_d_n7, assign59880_e97256_d_n8, assign59880_e97256_d_n9, assign59880_e97256_d_n10, assign59880_e97256_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59880_e97254: f64 = (locals.var_vds * locals.var_t8);
        (assign59880_e97254, (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), (locals.var_vds * locals.var_t8_dn5), ((locals.var_vds_dn6 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn6)), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), ((locals.var_vds_dn10 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn10)), (locals.var_vds * locals.var_t8_dn11),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11,)
    }
};
        locals.var_vdseff = assign59880_e97256;
        locals.var_vdseff_dn3 = assign59880_e97256_d_n3;
        locals.var_vdseff_dn4 = assign59880_e97256_d_n4;
        locals.var_vdseff_dn5 = assign59880_e97256_d_n5;
        locals.var_vdseff_dn6 = assign59880_e97256_d_n6;
        locals.var_vdseff_dn7 = assign59880_e97256_d_n7;
        locals.var_vdseff_dn8 = assign59880_e97256_d_n8;
        locals.var_vdseff_dn9 = assign59880_e97256_d_n9;
        locals.var_vdseff_dn10 = assign59880_e97256_d_n10;
        locals.var_vdseff_dn11 = assign59880_e97256_d_n11;

        let (assign59890_e97267, assign59890_e97267_d_n3, assign59890_e97267_d_n4, assign59890_e97267_d_n5, assign59890_e97267_d_n6, assign59890_e97267_d_n7, assign59890_e97267_d_n8, assign59890_e97267_d_n9, assign59890_e97267_d_n10, assign59890_e97267_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59890_e97263: f64 = (locals.var_vdseff + locals.var_vs);
        let assign59890_e97265: f64 = (assign59890_e97263 * locals.var_inv_nvt);
        (assign59890_e97265, ((locals.var_vdseff_dn3 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn3)), ((locals.var_vdseff_dn4 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn4)), ((locals.var_vdseff_dn5 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn5)), (((locals.var_vdseff_dn6 + locals.var_vs_dn6) * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn6)), (((locals.var_vdseff_dn7 + locals.var_vs_dn7) * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn7)), ((locals.var_vdseff_dn8 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn8)), ((locals.var_vdseff_dn9 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn9)), (((locals.var_vdseff_dn10 + locals.var_vs_dn10) * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn10)), ((locals.var_vdseff_dn11 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn11)),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11,)
    }
};
        locals.var_vdeff = assign59890_e97267;
        locals.var_vdeff_dn3 = assign59890_e97267_d_n3;
        locals.var_vdeff_dn4 = assign59890_e97267_d_n4;
        locals.var_vdeff_dn5 = assign59890_e97267_d_n5;
        locals.var_vdeff_dn6 = assign59890_e97267_d_n6;
        locals.var_vdeff_dn7 = assign59890_e97267_d_n7;
        locals.var_vdeff_dn8 = assign59890_e97267_d_n8;
        locals.var_vdeff_dn9 = assign59890_e97267_d_n9;
        locals.var_vdeff_dn10 = assign59890_e97267_d_n10;
        locals.var_vdeff_dn11 = assign59890_e97267_d_n11;

        let (assign59900_e97293, assign59900_e97293_d_n3, assign59900_e97293_d_n4, assign59900_e97293_d_n5, assign59900_e97293_d_n6, assign59900_e97293_d_n7, assign59900_e97293_d_n8, assign59900_e97293_d_n9, assign59900_e97293_d_n10, assign59900_e97293_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59900_e97275: f64 = (locals.var_psip + 1.0);
        let assign59900_e97278: f64 = (locals.var_psip - 1.0);
        let assign59900_e97281: f64 = (locals.var_psip - 1.0);
        let assign59900_e97282: f64 = (assign59900_e97278 * assign59900_e97281);
        let assign59900_e97285: f64 = (0.25 * 2.0);
        let assign59900_e97287: f64 = (assign59900_e97285 * 2.0);
        let assign59900_e97288: f64 = (assign59900_e97282 + assign59900_e97287);
        let assign59900_e97289: f64 = (assign59900_e97288).sqrt();
        let assign59900_e97290: f64 = (assign59900_e97275 + assign59900_e97289);
        let assign59900_e97291: f64 = (0.5 * assign59900_e97290);
        (assign59900_e97291, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn3)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn4)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn5)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn6)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn7)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn8)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn9)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn10)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn11)) / (2.0 * assign59900_e97289)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59900_e97293;
        locals.var_t8_dn3 = assign59900_e97293_d_n3;
        locals.var_t8_dn4 = assign59900_e97293_d_n4;
        locals.var_t8_dn5 = assign59900_e97293_d_n5;
        locals.var_t8_dn6 = assign59900_e97293_d_n6;
        locals.var_t8_dn7 = assign59900_e97293_d_n7;
        locals.var_t8_dn8 = assign59900_e97293_d_n8;
        locals.var_t8_dn9 = assign59900_e97293_d_n9;
        locals.var_t8_dn10 = assign59900_e97293_d_n10;
        locals.var_t8_dn11 = assign59900_e97293_d_n11;

        let (assign59910_e97301, assign59910_e97301_d_n3, assign59910_e97301_d_n4, assign59910_e97301_d_n5, assign59910_e97301_d_n6, assign59910_e97301_d_n7, assign59910_e97301_d_n8, assign59910_e97301_d_n9, assign59910_e97301_d_n10, assign59910_e97301_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59910_e97299: f64 = (locals.var_t8).sqrt();
        (assign59910_e97299, (locals.var_t8_dn3 / (2.0 * assign59910_e97299)), (locals.var_t8_dn4 / (2.0 * assign59910_e97299)), (locals.var_t8_dn5 / (2.0 * assign59910_e97299)), (locals.var_t8_dn6 / (2.0 * assign59910_e97299)), (locals.var_t8_dn7 / (2.0 * assign59910_e97299)), (locals.var_t8_dn8 / (2.0 * assign59910_e97299)), (locals.var_t8_dn9 / (2.0 * assign59910_e97299)), (locals.var_t8_dn10 / (2.0 * assign59910_e97299)), (locals.var_t8_dn11 / (2.0 * assign59910_e97299)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign59910_e97301;
        locals.var_sqrtpsip_dn3 = assign59910_e97301_d_n3;
        locals.var_sqrtpsip_dn4 = assign59910_e97301_d_n4;
        locals.var_sqrtpsip_dn5 = assign59910_e97301_d_n5;
        locals.var_sqrtpsip_dn6 = assign59910_e97301_d_n6;
        locals.var_sqrtpsip_dn7 = assign59910_e97301_d_n7;
        locals.var_sqrtpsip_dn8 = assign59910_e97301_d_n8;
        locals.var_sqrtpsip_dn9 = assign59910_e97301_d_n9;
        locals.var_sqrtpsip_dn10 = assign59910_e97301_d_n10;
        locals.var_sqrtpsip_dn11 = assign59910_e97301_d_n11;

        let (assign59920_e97316, assign59920_e97316_d_n3, assign59920_e97316_d_n4, assign59920_e97316_d_n5, assign59920_e97316_d_n6, assign59920_e97316_d_n7, assign59920_e97316_d_n8, assign59920_e97316_d_n9, assign59920_e97316_d_n10, assign59920_e97316_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59920_e97310: f64 = (2.0 * locals.var_sqrtpsip);
        let assign59920_e97311: f64 = (locals.var_gam_edge / assign59920_e97310);
        let assign59920_e97312: f64 = (1.0 + assign59920_e97311);
        let assign59920_e97314: f64 = (assign59920_e97312 / locals.var_gam_edge);
        (assign59920_e97314, ((((((locals.var_gam_edge_dn3 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign59920_e97316;
        locals.var_t0_dn3 = assign59920_e97316_d_n3;
        locals.var_t0_dn4 = assign59920_e97316_d_n4;
        locals.var_t0_dn5 = assign59920_e97316_d_n5;
        locals.var_t0_dn6 = assign59920_e97316_d_n6;
        locals.var_t0_dn7 = assign59920_e97316_d_n7;
        locals.var_t0_dn8 = assign59920_e97316_d_n8;
        locals.var_t0_dn9 = assign59920_e97316_d_n9;
        locals.var_t0_dn10 = assign59920_e97316_d_n10;
        locals.var_t0_dn11 = assign59920_e97316_d_n11;

        let (assign59930_e97329, assign59930_e97329_d_n3, assign59930_e97329_d_n4, assign59930_e97329_d_n5, assign59930_e97329_d_n6, assign59930_e97329_d_n7, assign59930_e97329_d_n8, assign59930_e97329_d_n9, assign59930_e97329_d_n10, assign59930_e97329_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59930_e97324: f64 = (2.0 * locals.var_phib_n_edge);
        let assign59930_e97325: f64 = (locals.var_psip - assign59930_e97324);
        let assign59930_e97327: f64 = (assign59930_e97325 - locals.var_vdeff);
        (assign59930_e97327, ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59930_e97329;
        locals.var_t1_dn3 = assign59930_e97329_d_n3;
        locals.var_t1_dn4 = assign59930_e97329_d_n4;
        locals.var_t1_dn5 = assign59930_e97329_d_n5;
        locals.var_t1_dn6 = assign59930_e97329_d_n6;
        locals.var_t1_dn7 = assign59930_e97329_d_n7;
        locals.var_t1_dn8 = assign59930_e97329_d_n8;
        locals.var_t1_dn9 = assign59930_e97329_d_n9;
        locals.var_t1_dn10 = assign59930_e97329_d_n10;
        locals.var_t1_dn11 = assign59930_e97329_d_n11;

        let (assign59940_e97345, assign59940_e97345_d_n3, assign59940_e97345_d_n4, assign59940_e97345_d_n5, assign59940_e97345_d_n6, assign59940_e97345_d_n7, assign59940_e97345_d_n8, assign59940_e97345_d_n9, assign59940_e97345_d_n10, assign59940_e97345_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59940_e97337: f64 = (4.0 * locals.var_t0);
        let assign59940_e97339: f64 = (assign59940_e97337 * locals.var_sqrtpsip);
        let assign59940_e97341: f64 = (assign59940_e97339).max(1e-38);
        let assign59940_e97342: f64 = (assign59940_e97341).ln();
        let assign59940_e97343: f64 = (locals.var_t1 - assign59940_e97342);
        (assign59940_e97343, (locals.var_t1_dn3 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn4 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn5 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn6 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn7 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn8 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn9 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn10 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn11 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign59940_e97341)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59940_e97345;
        locals.var_t2_dn3 = assign59940_e97345_d_n3;
        locals.var_t2_dn4 = assign59940_e97345_d_n4;
        locals.var_t2_dn5 = assign59940_e97345_d_n5;
        locals.var_t2_dn6 = assign59940_e97345_d_n6;
        locals.var_t2_dn7 = assign59940_e97345_d_n7;
        locals.var_t2_dn8 = assign59940_e97345_d_n8;
        locals.var_t2_dn9 = assign59940_e97345_d_n9;
        locals.var_t2_dn10 = assign59940_e97345_d_n10;
        locals.var_t2_dn11 = assign59940_e97345_d_n11;

        let (assign59950_e97365, assign59950_e97365_d_n3, assign59950_e97365_d_n4, assign59950_e97365_d_n5, assign59950_e97365_d_n6, assign59950_e97365_d_n7, assign59950_e97365_d_n8, assign59950_e97365_d_n9, assign59950_e97365_d_n10, assign59950_e97365_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59950_e97353: f64 = (locals.var_t2 - 0.201491);
        let assign59950_e97357: f64 = (locals.var_t2 + 0.402982);
        let assign59950_e97358: f64 = (locals.var_t2 * assign59950_e97357);
        let assign59950_e97360: f64 = (assign59950_e97358 + 2.446562);
        let assign59950_e97361: f64 = (assign59950_e97360).sqrt();
        let assign59950_e97362: f64 = (assign59950_e97353 - assign59950_e97361);
        let assign59950_e97363: f64 = (0.5 * assign59950_e97362);
        (assign59950_e97363, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign59950_e97361)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59950_e97365;
        locals.var_t8_dn3 = assign59950_e97365_d_n3;
        locals.var_t8_dn4 = assign59950_e97365_d_n4;
        locals.var_t8_dn5 = assign59950_e97365_d_n5;
        locals.var_t8_dn6 = assign59950_e97365_d_n6;
        locals.var_t8_dn7 = assign59950_e97365_d_n7;
        locals.var_t8_dn8 = assign59950_e97365_d_n8;
        locals.var_t8_dn9 = assign59950_e97365_d_n9;
        locals.var_t8_dn10 = assign59950_e97365_d_n10;
        locals.var_t8_dn11 = assign59950_e97365_d_n11;

        let (assign59960_e97372, assign59960_e97372_d_n3, assign59960_e97372_d_n4, assign59960_e97372_d_n5, assign59960_e97372_d_n6, assign59960_e97372_d_n7, assign59960_e97372_d_n8, assign59960_e97372_d_n9, assign59960_e97372_d_n10, assign59960_e97372_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign59960_e97372;
        locals.var_sqrtpsisa_dn3 = assign59960_e97372_d_n3;
        locals.var_sqrtpsisa_dn4 = assign59960_e97372_d_n4;
        locals.var_sqrtpsisa_dn5 = assign59960_e97372_d_n5;
        locals.var_sqrtpsisa_dn6 = assign59960_e97372_d_n6;
        locals.var_sqrtpsisa_dn7 = assign59960_e97372_d_n7;
        locals.var_sqrtpsisa_dn8 = assign59960_e97372_d_n8;
        locals.var_sqrtpsisa_dn9 = assign59960_e97372_d_n9;
        locals.var_sqrtpsisa_dn10 = assign59960_e97372_d_n10;
        locals.var_sqrtpsisa_dn11 = assign59960_e97372_d_n11;

        let assign59970_e97375: f64 = (-68.0);
        let assign59970_e97376: f64 = if locals.var_t8 <= assign59970_e97375 { 1.0 } else { 0.0 };
        locals.var_guard874 = assign59970_e97376;

        let (assign59980_e97386, assign59980_e97386_d_n3, assign59980_e97386_d_n4, assign59980_e97386_d_n5, assign59980_e97386_d_n6, assign59980_e97386_d_n7, assign59980_e97386_d_n8, assign59980_e97386_d_n9, assign59980_e97386_d_n10, assign59980_e97386_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign59980_e97384: f64 = (-100.0);
        (assign59980_e97384, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59980_e97386;
        locals.var_t4_dn3 = assign59980_e97386_d_n3;
        locals.var_t4_dn4 = assign59980_e97386_d_n4;
        locals.var_t4_dn5 = assign59980_e97386_d_n5;
        locals.var_t4_dn6 = assign59980_e97386_d_n6;
        locals.var_t4_dn7 = assign59980_e97386_d_n7;
        locals.var_t4_dn8 = assign59980_e97386_d_n8;
        locals.var_t4_dn9 = assign59980_e97386_d_n9;
        locals.var_t4_dn10 = assign59980_e97386_d_n10;
        locals.var_t4_dn11 = assign59980_e97386_d_n11;

        let (assign59990_e97395, assign59990_e97395_d_n3, assign59990_e97395_d_n4, assign59990_e97395_d_n5, assign59990_e97395_d_n6, assign59990_e97395_d_n7, assign59990_e97395_d_n8, assign59990_e97395_d_n9, assign59990_e97395_d_n10, assign59990_e97395_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59990_e97395;
        locals.var_t5_dn3 = assign59990_e97395_d_n3;
        locals.var_t5_dn4 = assign59990_e97395_d_n4;
        locals.var_t5_dn5 = assign59990_e97395_d_n5;
        locals.var_t5_dn6 = assign59990_e97395_d_n6;
        locals.var_t5_dn7 = assign59990_e97395_d_n7;
        locals.var_t5_dn8 = assign59990_e97395_d_n8;
        locals.var_t5_dn9 = assign59990_e97395_d_n9;
        locals.var_t5_dn10 = assign59990_e97395_d_n10;
        locals.var_t5_dn11 = assign59990_e97395_d_n11;

        let assign60000_e97400: f64 = (0.5 * locals.var_t5);
        let assign60000_e97401: f64 = (locals.var_t4 - assign60000_e97400);
        let assign60000_e97402: f64 = if locals.var_t8 < assign60000_e97401 { 1.0 } else { 0.0 };
        locals.var_guard875 = assign60000_e97402;

        let (assign60010_e97414, assign60010_e97414_d_n3, assign60010_e97414_d_n4, assign60010_e97414_d_n5, assign60010_e97414_d_n6, assign60010_e97414_d_n7, assign60010_e97414_d_n8, assign60010_e97414_d_n9, assign60010_e97414_d_n10, assign60010_e97414_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign60010_e97412: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60010_e97412, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60010_e97414;
        locals.var_t3_dn3 = assign60010_e97414_d_n3;
        locals.var_t3_dn4 = assign60010_e97414_d_n4;
        locals.var_t3_dn5 = assign60010_e97414_d_n5;
        locals.var_t3_dn6 = assign60010_e97414_d_n6;
        locals.var_t3_dn7 = assign60010_e97414_d_n7;
        locals.var_t3_dn8 = assign60010_e97414_d_n8;
        locals.var_t3_dn9 = assign60010_e97414_d_n9;
        locals.var_t3_dn10 = assign60010_e97414_d_n10;
        locals.var_t3_dn11 = assign60010_e97414_d_n11;

        let assign60020_e97419: f64 = (0.5 * locals.var_t5);
        let assign60020_e97420: f64 = (locals.var_t4 + assign60020_e97419);
        let assign60020_e97421: f64 = if locals.var_t8 > assign60020_e97420 { 1.0 } else { 0.0 };
        locals.var_guard876 = assign60020_e97421;

        let (assign60030_e97436, assign60030_e97436_d_n3, assign60030_e97436_d_n4, assign60030_e97436_d_n5, assign60030_e97436_d_n6, assign60030_e97436_d_n7, assign60030_e97436_d_n8, assign60030_e97436_d_n9, assign60030_e97436_d_n10, assign60030_e97436_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 != 0.0)) {
        let assign60030_e97434: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60030_e97434, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60030_e97436;
        locals.var_t3_dn3 = assign60030_e97436_d_n3;
        locals.var_t3_dn4 = assign60030_e97436_d_n4;
        locals.var_t3_dn5 = assign60030_e97436_d_n5;
        locals.var_t3_dn6 = assign60030_e97436_d_n6;
        locals.var_t3_dn7 = assign60030_e97436_d_n7;
        locals.var_t3_dn8 = assign60030_e97436_d_n8;
        locals.var_t3_dn9 = assign60030_e97436_d_n9;
        locals.var_t3_dn10 = assign60030_e97436_d_n10;
        locals.var_t3_dn11 = assign60030_e97436_d_n11;

        let (assign60040_e97455, assign60040_e97455_d_n3, assign60040_e97455_d_n4, assign60040_e97455_d_n5, assign60040_e97455_d_n6, assign60040_e97455_d_n7, assign60040_e97455_d_n8, assign60040_e97455_d_n9, assign60040_e97455_d_n10, assign60040_e97455_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign60040_e97451: f64 = (locals.var_t8 - locals.var_t4);
        let assign60040_e97453: f64 = (assign60040_e97451 / locals.var_t5);
        (assign60040_e97453, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign60040_e97455;
        locals.var_t2_dn3 = assign60040_e97455_d_n3;
        locals.var_t2_dn4 = assign60040_e97455_d_n4;
        locals.var_t2_dn5 = assign60040_e97455_d_n5;
        locals.var_t2_dn6 = assign60040_e97455_d_n6;
        locals.var_t2_dn7 = assign60040_e97455_d_n7;
        locals.var_t2_dn8 = assign60040_e97455_d_n8;
        locals.var_t2_dn9 = assign60040_e97455_d_n9;
        locals.var_t2_dn10 = assign60040_e97455_d_n10;
        locals.var_t2_dn11 = assign60040_e97455_d_n11;

        let (assign60050_e97472, assign60050_e97472_d_n3, assign60050_e97472_d_n4, assign60050_e97472_d_n5, assign60050_e97472_d_n6, assign60050_e97472_d_n7, assign60050_e97472_d_n8, assign60050_e97472_d_n9, assign60050_e97472_d_n10, assign60050_e97472_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign60050_e97470: f64 = (locals.var_t2 * locals.var_t2);
        (assign60050_e97470, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign60050_e97472;
        locals.var_t6_dn3 = assign60050_e97472_d_n3;
        locals.var_t6_dn4 = assign60050_e97472_d_n4;
        locals.var_t6_dn5 = assign60050_e97472_d_n5;
        locals.var_t6_dn6 = assign60050_e97472_d_n6;
        locals.var_t6_dn7 = assign60050_e97472_d_n7;
        locals.var_t6_dn8 = assign60050_e97472_d_n8;
        locals.var_t6_dn9 = assign60050_e97472_d_n9;
        locals.var_t6_dn10 = assign60050_e97472_d_n10;
        locals.var_t6_dn11 = assign60050_e97472_d_n11;

        let (assign60060_e97510, assign60060_e97510_d_n3, assign60060_e97510_d_n4, assign60060_e97510_d_n5, assign60060_e97510_d_n6, assign60060_e97510_d_n7, assign60060_e97510_d_n8, assign60060_e97510_d_n9, assign60060_e97510_d_n10, assign60060_e97510_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign60060_e97489: f64 = (5.0 / 64.0);
        let assign60060_e97492: f64 = (0.5 * locals.var_t2);
        let assign60060_e97493: f64 = (assign60060_e97489 + assign60060_e97492);
        let assign60060_e97497: f64 = (15.0 / 16.0);
        let assign60060_e97501: f64 = (1.25 - locals.var_t6);
        let assign60060_e97502: f64 = (locals.var_t6 * assign60060_e97501);
        let assign60060_e97503: f64 = (assign60060_e97497 - assign60060_e97502);
        let assign60060_e97504: f64 = (locals.var_t6 * assign60060_e97503);
        let assign60060_e97505: f64 = (assign60060_e97493 + assign60060_e97504);
        let assign60060_e97506: f64 = (locals.var_t5 * assign60060_e97505);
        let assign60060_e97507: f64 = (locals.var_t4 + assign60060_e97506);
        let assign60060_e97508: f64 = { let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60060_e97508, ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60060_e97510;
        locals.var_t3_dn3 = assign60060_e97510_d_n3;
        locals.var_t3_dn4 = assign60060_e97510_d_n4;
        locals.var_t3_dn5 = assign60060_e97510_d_n5;
        locals.var_t3_dn6 = assign60060_e97510_d_n6;
        locals.var_t3_dn7 = assign60060_e97510_d_n7;
        locals.var_t3_dn8 = assign60060_e97510_d_n8;
        locals.var_t3_dn9 = assign60060_e97510_d_n9;
        locals.var_t3_dn10 = assign60060_e97510_d_n10;
        locals.var_t3_dn11 = assign60060_e97510_d_n11;

        let (assign60070_e97542, assign60070_e97542_d_n3, assign60070_e97542_d_n4, assign60070_e97542_d_n5, assign60070_e97542_d_n6, assign60070_e97542_d_n7, assign60070_e97542_d_n8, assign60070_e97542_d_n9, assign60070_e97542_d_n10, assign60070_e97542_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign60070_e97520: f64 = (1.0 + locals.var_t1);
        let assign60070_e97522: f64 = (assign60070_e97520 - locals.var_t8);
        let assign60070_e97525: f64 = (2.0 * locals.var_t0);
        let assign60070_e97528: f64 = (locals.var_t3 * 2.0);
        let assign60070_e97530: f64 = (assign60070_e97528 * locals.var_t0);
        let assign60070_e97533: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign60070_e97534: f64 = (assign60070_e97530 + assign60070_e97533);
        let assign60070_e97535: f64 = (assign60070_e97525 * assign60070_e97534);
        let assign60070_e97537: f64 = (assign60070_e97535).max(1e-38);
        let assign60070_e97538: f64 = (assign60070_e97537).ln();
        let assign60070_e97539: f64 = (assign60070_e97522 - assign60070_e97538);
        let assign60070_e97540: f64 = (locals.var_t3 * assign60070_e97539);
        (assign60070_e97540, ((locals.var_t3_dn3 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn4 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn5 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn6 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn7 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn8 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn9 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn10 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn11 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60070_e97537)))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11,)
    }
};
        locals.var_qdeff_edge = assign60070_e97542;
        locals.var_qdeff_edge_dn3 = assign60070_e97542_d_n3;
        locals.var_qdeff_edge_dn4 = assign60070_e97542_d_n4;
        locals.var_qdeff_edge_dn5 = assign60070_e97542_d_n5;
        locals.var_qdeff_edge_dn6 = assign60070_e97542_d_n6;
        locals.var_qdeff_edge_dn7 = assign60070_e97542_d_n7;
        locals.var_qdeff_edge_dn8 = assign60070_e97542_d_n8;
        locals.var_qdeff_edge_dn9 = assign60070_e97542_d_n9;
        locals.var_qdeff_edge_dn10 = assign60070_e97542_d_n10;
        locals.var_qdeff_edge_dn11 = assign60070_e97542_d_n11;

    }

    pub(super) fn stamp_transient_block_199(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign60080_e97553, assign60080_e97553_d_n3, assign60080_e97553_d_n4, assign60080_e97553_d_n5, assign60080_e97553_d_n6, assign60080_e97553_d_n7, assign60080_e97553_d_n8, assign60080_e97553_d_n9, assign60080_e97553_d_n10, assign60080_e97553_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60080_e97551: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60080_e97551, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60080_e97553;
        locals.var_t3_dn3 = assign60080_e97553_d_n3;
        locals.var_t3_dn4 = assign60080_e97553_d_n4;
        locals.var_t3_dn5 = assign60080_e97553_d_n5;
        locals.var_t3_dn6 = assign60080_e97553_d_n6;
        locals.var_t3_dn7 = assign60080_e97553_d_n7;
        locals.var_t3_dn8 = assign60080_e97553_d_n8;
        locals.var_t3_dn9 = assign60080_e97553_d_n9;
        locals.var_t3_dn10 = assign60080_e97553_d_n10;
        locals.var_t3_dn11 = assign60080_e97553_d_n11;

        let (assign60090_e97565, assign60090_e97565_d_n3, assign60090_e97565_d_n4, assign60090_e97565_d_n5, assign60090_e97565_d_n6, assign60090_e97565_d_n7, assign60090_e97565_d_n8, assign60090_e97565_d_n9, assign60090_e97565_d_n10, assign60090_e97565_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60090_e97563: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign60090_e97563, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign60090_e97565;
        locals.var_sqrtpsisainv_dn3 = assign60090_e97565_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign60090_e97565_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign60090_e97565_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign60090_e97565_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign60090_e97565_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign60090_e97565_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign60090_e97565_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign60090_e97565_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign60090_e97565_d_n11;

        let (assign60100_e97598, assign60100_e97598_d_n3, assign60100_e97598_d_n4, assign60100_e97598_d_n5, assign60100_e97598_d_n6, assign60100_e97598_d_n7, assign60100_e97598_d_n8, assign60100_e97598_d_n9, assign60100_e97598_d_n10, assign60100_e97598_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60100_e97575: f64 = (2.0 * locals.var_t3);
        let assign60100_e97578: f64 = (locals.var_t3 * 2.0);
        let assign60100_e97580: f64 = (assign60100_e97578 * locals.var_t0);
        let assign60100_e97583: f64 = (locals.var_t3 * 2.0);
        let assign60100_e97585: f64 = (assign60100_e97583 * locals.var_t0);
        let assign60100_e97588: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign60100_e97589: f64 = (assign60100_e97585 + assign60100_e97588);
        let assign60100_e97590: f64 = (assign60100_e97580 * assign60100_e97589);
        let assign60100_e97592: f64 = (assign60100_e97590).max(1e-38);
        let assign60100_e97593: f64 = (assign60100_e97592).ln();
        let assign60100_e97594: f64 = (assign60100_e97575 + assign60100_e97593);
        let assign60100_e97596: f64 = (assign60100_e97594 - locals.var_t1);
        (assign60100_e97596, (((2.0 * locals.var_t3_dn3) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn3)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn4)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn5)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn6)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn7)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn8)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn9)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn10)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn11)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign60100_e97598;
        locals.var_t4_dn3 = assign60100_e97598_d_n3;
        locals.var_t4_dn4 = assign60100_e97598_d_n4;
        locals.var_t4_dn5 = assign60100_e97598_d_n5;
        locals.var_t4_dn6 = assign60100_e97598_d_n6;
        locals.var_t4_dn7 = assign60100_e97598_d_n7;
        locals.var_t4_dn8 = assign60100_e97598_d_n8;
        locals.var_t4_dn9 = assign60100_e97598_d_n9;
        locals.var_t4_dn10 = assign60100_e97598_d_n10;
        locals.var_t4_dn11 = assign60100_e97598_d_n11;

        let (assign60110_e97622, assign60110_e97622_d_n3, assign60110_e97622_d_n4, assign60110_e97622_d_n5, assign60110_e97622_d_n6, assign60110_e97622_d_n7, assign60110_e97622_d_n8, assign60110_e97622_d_n9, assign60110_e97622_d_n10, assign60110_e97622_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60110_e97609: f64 = (1.0 / locals.var_t3);
        let assign60110_e97610: f64 = (2.0 + assign60110_e97609);
        let assign60110_e97613: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60110_e97616: f64 = (locals.var_t0 * locals.var_t3);
        let assign60110_e97618: f64 = (assign60110_e97616 + locals.var_sqrtpsisa);
        let assign60110_e97619: f64 = (assign60110_e97613 / assign60110_e97618);
        let assign60110_e97620: f64 = (assign60110_e97610 + assign60110_e97619);
        (assign60110_e97620, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60110_e97618 * assign60110_e97618))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign60110_e97622;
        locals.var_t5_dn3 = assign60110_e97622_d_n3;
        locals.var_t5_dn4 = assign60110_e97622_d_n4;
        locals.var_t5_dn5 = assign60110_e97622_d_n5;
        locals.var_t5_dn6 = assign60110_e97622_d_n6;
        locals.var_t5_dn7 = assign60110_e97622_d_n7;
        locals.var_t5_dn8 = assign60110_e97622_d_n8;
        locals.var_t5_dn9 = assign60110_e97622_d_n9;
        locals.var_t5_dn10 = assign60110_e97622_d_n10;
        locals.var_t5_dn11 = assign60110_e97622_d_n11;

        let (assign60120_e97636, assign60120_e97636_d_n3, assign60120_e97636_d_n4, assign60120_e97636_d_n5, assign60120_e97636_d_n6, assign60120_e97636_d_n7, assign60120_e97636_d_n8, assign60120_e97636_d_n9, assign60120_e97636_d_n10, assign60120_e97636_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60120_e97633: f64 = (locals.var_t4 / locals.var_t5);
        let assign60120_e97634: f64 = (locals.var_t3 - assign60120_e97633);
        (assign60120_e97634, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60120_e97636;
        locals.var_t3_dn3 = assign60120_e97636_d_n3;
        locals.var_t3_dn4 = assign60120_e97636_d_n4;
        locals.var_t3_dn5 = assign60120_e97636_d_n5;
        locals.var_t3_dn6 = assign60120_e97636_d_n6;
        locals.var_t3_dn7 = assign60120_e97636_d_n7;
        locals.var_t3_dn8 = assign60120_e97636_d_n8;
        locals.var_t3_dn9 = assign60120_e97636_d_n9;
        locals.var_t3_dn10 = assign60120_e97636_d_n10;
        locals.var_t3_dn11 = assign60120_e97636_d_n11;

        let (assign60130_e97669, assign60130_e97669_d_n3, assign60130_e97669_d_n4, assign60130_e97669_d_n5, assign60130_e97669_d_n6, assign60130_e97669_d_n7, assign60130_e97669_d_n8, assign60130_e97669_d_n9, assign60130_e97669_d_n10, assign60130_e97669_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60130_e97646: f64 = (2.0 * locals.var_t3);
        let assign60130_e97649: f64 = (locals.var_t3 * 2.0);
        let assign60130_e97651: f64 = (assign60130_e97649 * locals.var_t0);
        let assign60130_e97654: f64 = (locals.var_t3 * 2.0);
        let assign60130_e97656: f64 = (assign60130_e97654 * locals.var_t0);
        let assign60130_e97659: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign60130_e97660: f64 = (assign60130_e97656 + assign60130_e97659);
        let assign60130_e97661: f64 = (assign60130_e97651 * assign60130_e97660);
        let assign60130_e97663: f64 = (assign60130_e97661).max(1e-38);
        let assign60130_e97664: f64 = (assign60130_e97663).ln();
        let assign60130_e97665: f64 = (assign60130_e97646 + assign60130_e97664);
        let assign60130_e97667: f64 = (assign60130_e97665 - locals.var_t1);
        (assign60130_e97667, (((2.0 * locals.var_t3_dn3) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn3)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn4)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn5)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn6)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn7)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn8)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn9)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn10)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn11)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign60130_e97669;
        locals.var_t4_dn3 = assign60130_e97669_d_n3;
        locals.var_t4_dn4 = assign60130_e97669_d_n4;
        locals.var_t4_dn5 = assign60130_e97669_d_n5;
        locals.var_t4_dn6 = assign60130_e97669_d_n6;
        locals.var_t4_dn7 = assign60130_e97669_d_n7;
        locals.var_t4_dn8 = assign60130_e97669_d_n8;
        locals.var_t4_dn9 = assign60130_e97669_d_n9;
        locals.var_t4_dn10 = assign60130_e97669_d_n10;
        locals.var_t4_dn11 = assign60130_e97669_d_n11;

        let (assign60140_e97693, assign60140_e97693_d_n3, assign60140_e97693_d_n4, assign60140_e97693_d_n5, assign60140_e97693_d_n6, assign60140_e97693_d_n7, assign60140_e97693_d_n8, assign60140_e97693_d_n9, assign60140_e97693_d_n10, assign60140_e97693_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60140_e97680: f64 = (1.0 / locals.var_t3);
        let assign60140_e97681: f64 = (2.0 + assign60140_e97680);
        let assign60140_e97684: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60140_e97687: f64 = (locals.var_t0 * locals.var_t3);
        let assign60140_e97689: f64 = (assign60140_e97687 + locals.var_sqrtpsisa);
        let assign60140_e97690: f64 = (assign60140_e97684 / assign60140_e97689);
        let assign60140_e97691: f64 = (assign60140_e97681 + assign60140_e97690);
        (assign60140_e97691, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60140_e97689 * assign60140_e97689))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign60140_e97693;
        locals.var_t5_dn3 = assign60140_e97693_d_n3;
        locals.var_t5_dn4 = assign60140_e97693_d_n4;
        locals.var_t5_dn5 = assign60140_e97693_d_n5;
        locals.var_t5_dn6 = assign60140_e97693_d_n6;
        locals.var_t5_dn7 = assign60140_e97693_d_n7;
        locals.var_t5_dn8 = assign60140_e97693_d_n8;
        locals.var_t5_dn9 = assign60140_e97693_d_n9;
        locals.var_t5_dn10 = assign60140_e97693_d_n10;
        locals.var_t5_dn11 = assign60140_e97693_d_n11;

        let (assign60150_e97721, assign60150_e97721_d_n3, assign60150_e97721_d_n4, assign60150_e97721_d_n5, assign60150_e97721_d_n6, assign60150_e97721_d_n7, assign60150_e97721_d_n8, assign60150_e97721_d_n9, assign60150_e97721_d_n10, assign60150_e97721_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60150_e97703: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60150_e97706: f64 = (locals.var_t0 * locals.var_t3);
        let assign60150_e97708: f64 = (assign60150_e97706 + locals.var_sqrtpsisa);
        let assign60150_e97709: f64 = (assign60150_e97703 / assign60150_e97708);
        let assign60150_e97712: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60150_e97715: f64 = (locals.var_t0 * locals.var_t3);
        let assign60150_e97717: f64 = (assign60150_e97715 + locals.var_sqrtpsisa);
        let assign60150_e97718: f64 = (assign60150_e97712 / assign60150_e97717);
        let assign60150_e97719: f64 = (assign60150_e97709 * assign60150_e97718);
        (assign60150_e97719, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60150_e97717 * assign60150_e97717)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign60150_e97721;
        locals.var_t6_dn3 = assign60150_e97721_d_n3;
        locals.var_t6_dn4 = assign60150_e97721_d_n4;
        locals.var_t6_dn5 = assign60150_e97721_d_n5;
        locals.var_t6_dn6 = assign60150_e97721_d_n6;
        locals.var_t6_dn7 = assign60150_e97721_d_n7;
        locals.var_t6_dn8 = assign60150_e97721_d_n8;
        locals.var_t6_dn9 = assign60150_e97721_d_n9;
        locals.var_t6_dn10 = assign60150_e97721_d_n10;
        locals.var_t6_dn11 = assign60150_e97721_d_n11;

        let (assign60160_e97754, assign60160_e97754_d_n3, assign60160_e97754_d_n4, assign60160_e97754_d_n5, assign60160_e97754_d_n6, assign60160_e97754_d_n7, assign60160_e97754_d_n8, assign60160_e97754_d_n9, assign60160_e97754_d_n10, assign60160_e97754_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign60160_e97731: f64 = (1.0 * __rspice_inv_cse_0);
        let assign60160_e97734: f64 = (1.0 * __rspice_inv_cse_0);
        let assign60160_e97735: f64 = (assign60160_e97731 * assign60160_e97734);
        let assign60160_e97736: f64 = (-assign60160_e97735);
        let assign60160_e97740: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign60160_e97742: f64 = (assign60160_e97740 * locals.var_sqrtpsisa);
        let assign60160_e97745: f64 = (locals.var_t0 * locals.var_t3);
        let assign60160_e97747: f64 = (assign60160_e97745 + locals.var_sqrtpsisa);
        let assign60160_e97748: f64 = (assign60160_e97742 * assign60160_e97747);
        let assign60160_e97749: f64 = (1.0 / assign60160_e97748);
        let assign60160_e97750: f64 = (assign60160_e97736 - assign60160_e97749);
        let assign60160_e97752: f64 = (assign60160_e97750 - locals.var_t6);
        (assign60160_e97752, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn3)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn4)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn5)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn6)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn7)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn8)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn9)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn10)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn11)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign60160_e97754;
        locals.var_t7_dn3 = assign60160_e97754_d_n3;
        locals.var_t7_dn4 = assign60160_e97754_d_n4;
        locals.var_t7_dn5 = assign60160_e97754_d_n5;
        locals.var_t7_dn6 = assign60160_e97754_d_n6;
        locals.var_t7_dn7 = assign60160_e97754_d_n7;
        locals.var_t7_dn8 = assign60160_e97754_d_n8;
        locals.var_t7_dn9 = assign60160_e97754_d_n9;
        locals.var_t7_dn10 = assign60160_e97754_d_n10;
        locals.var_t7_dn11 = assign60160_e97754_d_n11;

        let (assign60170_e97780, assign60170_e97780_d_n3, assign60170_e97780_d_n4, assign60170_e97780_d_n5, assign60170_e97780_d_n6, assign60170_e97780_d_n7, assign60170_e97780_d_n8, assign60170_e97780_d_n9, assign60170_e97780_d_n10, assign60170_e97780_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60170_e97765: f64 = (locals.var_t4 / locals.var_t5);
        let assign60170_e97769: f64 = (locals.var_t4 * locals.var_t7);
        let assign60170_e97772: f64 = (2.0 * locals.var_t5);
        let assign60170_e97774: f64 = (assign60170_e97772 * locals.var_t5);
        let assign60170_e97775: f64 = (assign60170_e97769 / assign60170_e97774);
        let assign60170_e97776: f64 = (1.0 + assign60170_e97775);
        let assign60170_e97777: f64 = (assign60170_e97765 * assign60170_e97776);
        let assign60170_e97778: f64 = (locals.var_t3 - assign60170_e97777);
        (assign60170_e97778, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn3)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn4)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn5)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn6)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn7)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn8)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn9)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn10)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn11)))) / (assign60170_e97774 * assign60170_e97774))))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11,)
    }
};
        locals.var_qdeff_edge = assign60170_e97780;
        locals.var_qdeff_edge_dn3 = assign60170_e97780_d_n3;
        locals.var_qdeff_edge_dn4 = assign60170_e97780_d_n4;
        locals.var_qdeff_edge_dn5 = assign60170_e97780_d_n5;
        locals.var_qdeff_edge_dn6 = assign60170_e97780_d_n6;
        locals.var_qdeff_edge_dn7 = assign60170_e97780_d_n7;
        locals.var_qdeff_edge_dn8 = assign60170_e97780_d_n8;
        locals.var_qdeff_edge_dn9 = assign60170_e97780_d_n9;
        locals.var_qdeff_edge_dn10 = assign60170_e97780_d_n10;
        locals.var_qdeff_edge_dn11 = assign60170_e97780_d_n11;

        let (assign60180_e97806, assign60180_e97806_d_n3, assign60180_e97806_d_n4, assign60180_e97806_d_n5, assign60180_e97806_d_n6, assign60180_e97806_d_n7, assign60180_e97806_d_n8, assign60180_e97806_d_n9, assign60180_e97806_d_n10, assign60180_e97806_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60180_e97788: f64 = (locals.var_psip + 1.0);
        let assign60180_e97791: f64 = (locals.var_psip - 1.0);
        let assign60180_e97794: f64 = (locals.var_psip - 1.0);
        let assign60180_e97795: f64 = (assign60180_e97791 * assign60180_e97794);
        let assign60180_e97798: f64 = (0.25 * 2.0);
        let assign60180_e97800: f64 = (assign60180_e97798 * 2.0);
        let assign60180_e97801: f64 = (assign60180_e97795 + assign60180_e97800);
        let assign60180_e97802: f64 = (assign60180_e97801).sqrt();
        let assign60180_e97803: f64 = (assign60180_e97788 + assign60180_e97802);
        let assign60180_e97804: f64 = (0.5 * assign60180_e97803);
        (assign60180_e97804, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn3)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn4)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn5)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn6)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn7)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn8)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn9)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn10)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn11)) / (2.0 * assign60180_e97802)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11,)
    }
};
        locals.var_psipclamp = assign60180_e97806;
        locals.var_psipclamp_dn3 = assign60180_e97806_d_n3;
        locals.var_psipclamp_dn4 = assign60180_e97806_d_n4;
        locals.var_psipclamp_dn5 = assign60180_e97806_d_n5;
        locals.var_psipclamp_dn6 = assign60180_e97806_d_n6;
        locals.var_psipclamp_dn7 = assign60180_e97806_d_n7;
        locals.var_psipclamp_dn8 = assign60180_e97806_d_n8;
        locals.var_psipclamp_dn9 = assign60180_e97806_d_n9;
        locals.var_psipclamp_dn10 = assign60180_e97806_d_n10;
        locals.var_psipclamp_dn11 = assign60180_e97806_d_n11;

        let (assign60190_e97814, assign60190_e97814_d_n3, assign60190_e97814_d_n4, assign60190_e97814_d_n5, assign60190_e97814_d_n6, assign60190_e97814_d_n7, assign60190_e97814_d_n8, assign60190_e97814_d_n9, assign60190_e97814_d_n10, assign60190_e97814_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60190_e97812: f64 = (locals.var_psipclamp).sqrt();
        (assign60190_e97812, (locals.var_psipclamp_dn3 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn4 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn5 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn6 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn7 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn8 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn9 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn10 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn11 / (2.0 * assign60190_e97812)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign60190_e97814;
        locals.var_sqrtpsip_dn3 = assign60190_e97814_d_n3;
        locals.var_sqrtpsip_dn4 = assign60190_e97814_d_n4;
        locals.var_sqrtpsip_dn5 = assign60190_e97814_d_n5;
        locals.var_sqrtpsip_dn6 = assign60190_e97814_d_n6;
        locals.var_sqrtpsip_dn7 = assign60190_e97814_d_n7;
        locals.var_sqrtpsip_dn8 = assign60190_e97814_d_n8;
        locals.var_sqrtpsip_dn9 = assign60190_e97814_d_n9;
        locals.var_sqrtpsip_dn10 = assign60190_e97814_d_n10;
        locals.var_sqrtpsip_dn11 = assign60190_e97814_d_n11;

        let (assign60200_e97827, assign60200_e97827_d_n3, assign60200_e97827_d_n4, assign60200_e97827_d_n5, assign60200_e97827_d_n6, assign60200_e97827_d_n7, assign60200_e97827_d_n8, assign60200_e97827_d_n9, assign60200_e97827_d_n10, assign60200_e97827_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60200_e97821: f64 = (locals.var_psip - locals.var_qs_edge);
        let assign60200_e97823: f64 = (assign60200_e97821 - locals.var_qdeff_edge);
        let assign60200_e97825: f64 = (assign60200_e97823 - 1.0);
        (assign60200_e97825, ((locals.var_psip_dn3 - locals.var_qs_edge_dn3) - locals.var_qdeff_edge_dn3), ((locals.var_psip_dn4 - locals.var_qs_edge_dn4) - locals.var_qdeff_edge_dn4), ((locals.var_psip_dn5 - locals.var_qs_edge_dn5) - locals.var_qdeff_edge_dn5), ((locals.var_psip_dn6 - locals.var_qs_edge_dn6) - locals.var_qdeff_edge_dn6), ((locals.var_psip_dn7 - locals.var_qs_edge_dn7) - locals.var_qdeff_edge_dn7), ((locals.var_psip_dn8 - locals.var_qs_edge_dn8) - locals.var_qdeff_edge_dn8), ((locals.var_psip_dn9 - locals.var_qs_edge_dn9) - locals.var_qdeff_edge_dn9), ((locals.var_psip_dn10 - locals.var_qs_edge_dn10) - locals.var_qdeff_edge_dn10), ((locals.var_psip_dn11 - locals.var_qs_edge_dn11) - locals.var_qdeff_edge_dn11),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign60200_e97827;
        locals.var_psiavg_dn3 = assign60200_e97827_d_n3;
        locals.var_psiavg_dn4 = assign60200_e97827_d_n4;
        locals.var_psiavg_dn5 = assign60200_e97827_d_n5;
        locals.var_psiavg_dn6 = assign60200_e97827_d_n6;
        locals.var_psiavg_dn7 = assign60200_e97827_d_n7;
        locals.var_psiavg_dn8 = assign60200_e97827_d_n8;
        locals.var_psiavg_dn9 = assign60200_e97827_d_n9;
        locals.var_psiavg_dn10 = assign60200_e97827_d_n10;
        locals.var_psiavg_dn11 = assign60200_e97827_d_n11;

        let (assign60210_e97853, assign60210_e97853_d_n3, assign60210_e97853_d_n4, assign60210_e97853_d_n5, assign60210_e97853_d_n6, assign60210_e97853_d_n7, assign60210_e97853_d_n8, assign60210_e97853_d_n9, assign60210_e97853_d_n10, assign60210_e97853_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60210_e97835: f64 = (locals.var_psiavg + 1.0);
        let assign60210_e97838: f64 = (locals.var_psiavg - 1.0);
        let assign60210_e97841: f64 = (locals.var_psiavg - 1.0);
        let assign60210_e97842: f64 = (assign60210_e97838 * assign60210_e97841);
        let assign60210_e97845: f64 = (0.25 * 2.0);
        let assign60210_e97847: f64 = (assign60210_e97845 * 2.0);
        let assign60210_e97848: f64 = (assign60210_e97842 + assign60210_e97847);
        let assign60210_e97849: f64 = (assign60210_e97848).sqrt();
        let assign60210_e97850: f64 = (assign60210_e97835 + assign60210_e97849);
        let assign60210_e97851: f64 = (0.5 * assign60210_e97850);
        (assign60210_e97851, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn3)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn4)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn5)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn6)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn7)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn8)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn9)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn10)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn11)) / (2.0 * assign60210_e97849)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign60210_e97853;
        locals.var_t0_dn3 = assign60210_e97853_d_n3;
        locals.var_t0_dn4 = assign60210_e97853_d_n4;
        locals.var_t0_dn5 = assign60210_e97853_d_n5;
        locals.var_t0_dn6 = assign60210_e97853_d_n6;
        locals.var_t0_dn7 = assign60210_e97853_d_n7;
        locals.var_t0_dn8 = assign60210_e97853_d_n8;
        locals.var_t0_dn9 = assign60210_e97853_d_n9;
        locals.var_t0_dn10 = assign60210_e97853_d_n10;
        locals.var_t0_dn11 = assign60210_e97853_d_n11;

        let (assign60220_e97861, assign60220_e97861_d_n3, assign60220_e97861_d_n4, assign60220_e97861_d_n5, assign60220_e97861_d_n6, assign60220_e97861_d_n7, assign60220_e97861_d_n8, assign60220_e97861_d_n9, assign60220_e97861_d_n10, assign60220_e97861_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60220_e97859: f64 = (locals.var_t0).sqrt();
        (assign60220_e97859, (locals.var_t0_dn3 / (2.0 * assign60220_e97859)), (locals.var_t0_dn4 / (2.0 * assign60220_e97859)), (locals.var_t0_dn5 / (2.0 * assign60220_e97859)), (locals.var_t0_dn6 / (2.0 * assign60220_e97859)), (locals.var_t0_dn7 / (2.0 * assign60220_e97859)), (locals.var_t0_dn8 / (2.0 * assign60220_e97859)), (locals.var_t0_dn9 / (2.0 * assign60220_e97859)), (locals.var_t0_dn10 / (2.0 * assign60220_e97859)), (locals.var_t0_dn11 / (2.0 * assign60220_e97859)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign60220_e97861;
        locals.var_t2_dn3 = assign60220_e97861_d_n3;
        locals.var_t2_dn4 = assign60220_e97861_d_n4;
        locals.var_t2_dn5 = assign60220_e97861_d_n5;
        locals.var_t2_dn6 = assign60220_e97861_d_n6;
        locals.var_t2_dn7 = assign60220_e97861_d_n7;
        locals.var_t2_dn8 = assign60220_e97861_d_n8;
        locals.var_t2_dn9 = assign60220_e97861_d_n9;
        locals.var_t2_dn10 = assign60220_e97861_d_n10;
        locals.var_t2_dn11 = assign60220_e97861_d_n11;

        let (assign60230_e97874, assign60230_e97874_d_n3, assign60230_e97874_d_n4, assign60230_e97874_d_n5, assign60230_e97874_d_n6, assign60230_e97874_d_n7, assign60230_e97874_d_n8, assign60230_e97874_d_n9, assign60230_e97874_d_n10, assign60230_e97874_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60230_e97870: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign60230_e97871: f64 = (locals.var_gam_edge / assign60230_e97870);
        let assign60230_e97872: f64 = (1.0 + assign60230_e97871);
        (assign60230_e97872, (((locals.var_gam_edge_dn3 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn4 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn5 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn6 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn7 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn8 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn9 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn10 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn11 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign60230_e97870 * assign60230_e97870)),)
    } else {
        (locals.var_nq_edge, locals.var_nq_edge_dn3, locals.var_nq_edge_dn4, locals.var_nq_edge_dn5, locals.var_nq_edge_dn6, locals.var_nq_edge_dn7, locals.var_nq_edge_dn8, locals.var_nq_edge_dn9, locals.var_nq_edge_dn10, locals.var_nq_edge_dn11,)
    }
};
        locals.var_nq_edge = assign60230_e97874;
        locals.var_nq_edge_dn3 = assign60230_e97874_d_n3;
        locals.var_nq_edge_dn4 = assign60230_e97874_d_n4;
        locals.var_nq_edge_dn5 = assign60230_e97874_d_n5;
        locals.var_nq_edge_dn6 = assign60230_e97874_d_n6;
        locals.var_nq_edge_dn7 = assign60230_e97874_d_n7;
        locals.var_nq_edge_dn8 = assign60230_e97874_d_n8;
        locals.var_nq_edge_dn9 = assign60230_e97874_d_n9;
        locals.var_nq_edge_dn10 = assign60230_e97874_d_n10;
        locals.var_nq_edge_dn11 = assign60230_e97874_d_n11;

        let (assign60240_e97909, assign60240_e97909_d_n3, assign60240_e97909_d_n4, assign60240_e97909_d_n5, assign60240_e97909_d_n6, assign60240_e97909_d_n7, assign60240_e97909_d_n8, assign60240_e97909_d_n9, assign60240_e97909_d_n10, assign60240_e97909_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60240_e97881: f64 = (2.0 * p.p2);
        let assign60240_e97883: f64 = (assign60240_e97881 * locals.var_nq_edge);
        let assign60240_e97885: f64 = (assign60240_e97883 * locals.var_ueff);
        let assign60240_e97887: f64 = (assign60240_e97885 * p.p1147);
        let assign60240_e97889: f64 = (assign60240_e97887 / locals.var_leff);
        let assign60240_e97891: f64 = (assign60240_e97889 * locals.var_cox);
        let assign60240_e97893: f64 = (assign60240_e97891 * locals.var_nvt);
        let assign60240_e97895: f64 = (assign60240_e97893 * locals.var_nvt);
        let assign60240_e97898: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign60240_e97901: f64 = (1.0 + locals.var_qs_edge);
        let assign60240_e97903: f64 = (assign60240_e97901 + locals.var_qdeff_edge);
        let assign60240_e97904: f64 = (assign60240_e97898 * assign60240_e97903);
        let assign60240_e97905: f64 = (assign60240_e97895 * assign60240_e97904);
        let assign60240_e97907: f64 = (assign60240_e97905 * locals.var_moc);
        (assign60240_e97907, ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn3) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn3)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn3)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn3 + locals.var_qdeff_edge_dn3))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn3)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn4) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn4)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn4)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn4 + locals.var_qdeff_edge_dn4))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn4)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn5) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn5)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn5)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn5 + locals.var_qdeff_edge_dn5))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn5)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn6) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn6)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn6)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn6 + locals.var_qdeff_edge_dn6))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn6)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn7) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn7)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn7)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn7 + locals.var_qdeff_edge_dn7))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn7)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn8) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn8)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn8)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn8 + locals.var_qdeff_edge_dn8))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn8)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn9) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn9)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn9)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn9 + locals.var_qdeff_edge_dn9))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn9)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn10) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn10)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn10)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn10 + locals.var_qdeff_edge_dn10))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn10)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn11) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn11)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn11)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn11 + locals.var_qdeff_edge_dn11))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn11)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn3, locals.var_ids_edge_dn4, locals.var_ids_edge_dn5, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9, locals.var_ids_edge_dn10, locals.var_ids_edge_dn11,)
    }
};
        locals.var_ids_edge = assign60240_e97909;
        locals.var_ids_edge_dn3 = assign60240_e97909_d_n3;
        locals.var_ids_edge_dn4 = assign60240_e97909_d_n4;
        locals.var_ids_edge_dn5 = assign60240_e97909_d_n5;
        locals.var_ids_edge_dn6 = assign60240_e97909_d_n6;
        locals.var_ids_edge_dn7 = assign60240_e97909_d_n7;
        locals.var_ids_edge_dn8 = assign60240_e97909_d_n8;
        locals.var_ids_edge_dn9 = assign60240_e97909_d_n9;
        locals.var_ids_edge_dn10 = assign60240_e97909_d_n10;
        locals.var_ids_edge_dn11 = assign60240_e97909_d_n11;

        let (assign60250_e97918, assign60250_e97918_d_n3, assign60250_e97918_d_n4, assign60250_e97918_d_n5, assign60250_e97918_d_n6, assign60250_e97918_d_n7, assign60250_e97918_d_n8, assign60250_e97918_d_n9, assign60250_e97918_d_n10, assign60250_e97918_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60250_e97916: f64 = (locals.var_ids_edge + locals.var_ids);
        (assign60250_e97916, (locals.var_ids_edge_dn3 + locals.var_ids_dn3), (locals.var_ids_edge_dn4 + locals.var_ids_dn4), (locals.var_ids_edge_dn5 + locals.var_ids_dn5), (locals.var_ids_edge_dn6 + locals.var_ids_dn6), (locals.var_ids_edge_dn7 + locals.var_ids_dn7), (locals.var_ids_edge_dn8 + locals.var_ids_dn8), (locals.var_ids_edge_dn9 + locals.var_ids_dn9), (locals.var_ids_edge_dn10 + locals.var_ids_dn10), (locals.var_ids_edge_dn11 + locals.var_ids_dn11),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign60250_e97918;
        locals.var_ids_dn3 = assign60250_e97918_d_n3;
        locals.var_ids_dn4 = assign60250_e97918_d_n4;
        locals.var_ids_dn5 = assign60250_e97918_d_n5;
        locals.var_ids_dn6 = assign60250_e97918_d_n6;
        locals.var_ids_dn7 = assign60250_e97918_d_n7;
        locals.var_ids_dn8 = assign60250_e97918_d_n8;
        locals.var_ids_dn9 = assign60250_e97918_d_n9;
        locals.var_ids_dn10 = assign60250_e97918_d_n10;
        locals.var_ids_dn11 = assign60250_e97918_d_n11;

        let (assign60260_e97927,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60260_e97925: f64 = (p.p1012 * p.p1316);
        (assign60260_e97925,)
    } else {
        (locals.var_noia_edge,)
    }
};
        locals.var_noia_edge = assign60260_e97927;

        let (assign60270_e97936,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60270_e97934: f64 = (p.p1013 * p.p1316);
        (assign60270_e97934,)
    } else {
        (locals.var_noib_edge,)
    }
};
        locals.var_noib_edge = assign60270_e97936;

        let (assign60280_e97945,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60280_e97943: f64 = (p.p1014 * p.p1316);
        (assign60280_e97943,)
    } else {
        (locals.var_noic_edge,)
    }
};
        locals.var_noic_edge = assign60280_e97945;

        let (assign60290_e97956,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60290_e97953: f64 = (2.0 * locals.var_lintnoi_i);
        let assign60290_e97954: f64 = (locals.var_leff - assign60290_e97953);
        (assign60290_e97954,)
    } else {
        (locals.var_leffnoi_edge,)
    }
};
        locals.var_leffnoi_edge = assign60290_e97956;

        let (assign60300_e97965,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60300_e97963: f64 = (locals.var_leffnoi_edge * locals.var_leffnoi_edge);
        (assign60300_e97963,)
    } else {
        (locals.var_leffnoisq_edge,)
    }
};
        locals.var_leffnoisq_edge = assign60300_e97965;

        let (assign60310_e97980, assign60310_e97980_d_n3, assign60310_e97980_d_n4, assign60310_e97980_d_n5, assign60310_e97980_d_n6, assign60310_e97980_d_n7, assign60310_e97980_d_n8, assign60310_e97980_d_n9, assign60310_e97980_d_n10, assign60310_e97980_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60310_e97972: f64 = (locals.var_vt / 1.602176462e-19);
        let assign60310_e97975: f64 = (locals.var_cox + locals.var_cdep);
        let assign60310_e97977: f64 = (assign60310_e97975 + locals.var_citedge_i);
        let assign60310_e97978: f64 = (assign60310_e97972 * assign60310_e97977);
        (assign60310_e97978, (assign60310_e97972 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.602176462e-19) * assign60310_e97977) + (assign60310_e97972 * locals.var_cdep_dn4)), (((locals.var_vt_dn5 / 1.602176462e-19) * assign60310_e97977) + (assign60310_e97972 * locals.var_cdep_dn5)), (assign60310_e97972 * locals.var_cdep_dn6), (assign60310_e97972 * locals.var_cdep_dn7), (assign60310_e97972 * locals.var_cdep_dn8), (assign60310_e97972 * locals.var_cdep_dn9), (assign60310_e97972 * locals.var_cdep_dn10), (assign60310_e97972 * locals.var_cdep_dn11),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11,)
    }
};
        locals.var_nstar = assign60310_e97980;
        locals.var_nstar_dn3 = assign60310_e97980_d_n3;
        locals.var_nstar_dn4 = assign60310_e97980_d_n4;
        locals.var_nstar_dn5 = assign60310_e97980_d_n5;
        locals.var_nstar_dn6 = assign60310_e97980_d_n6;
        locals.var_nstar_dn7 = assign60310_e97980_d_n7;
        locals.var_nstar_dn8 = assign60310_e97980_d_n8;
        locals.var_nstar_dn9 = assign60310_e97980_d_n9;
        locals.var_nstar_dn10 = assign60310_e97980_d_n10;
        locals.var_nstar_dn11 = assign60310_e97980_d_n11;

    }

    pub(super) fn stamp_transient_block_200(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign60320_e97997, assign60320_e97997_d_n3, assign60320_e97997_d_n4, assign60320_e97997_d_n5, assign60320_e97997_d_n6, assign60320_e97997_d_n7, assign60320_e97997_d_n8, assign60320_e97997_d_n9, assign60320_e97997_d_n10, assign60320_e97997_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60320_e97987: f64 = (2.0 * locals.var_nq_edge);
        let assign60320_e97989: f64 = (assign60320_e97987 * locals.var_cox);
        let assign60320_e97991: f64 = (assign60320_e97989 * locals.var_vt);
        let assign60320_e97993: f64 = (assign60320_e97991 * locals.var_qdeff_edge);
        let assign60320_e97995: f64 = (assign60320_e97993 / 1.602176462e-19);
        (assign60320_e97995, ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn3)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign60320_e97989 * locals.var_vt_dn4)) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn4)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) + (assign60320_e97989 * locals.var_vt_dn5)) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn5)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn6)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn7)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn8)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn9)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn10)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11,)
    }
};
        locals.var_nl = assign60320_e97997;
        locals.var_nl_dn3 = assign60320_e97997_d_n3;
        locals.var_nl_dn4 = assign60320_e97997_d_n4;
        locals.var_nl_dn5 = assign60320_e97997_d_n5;
        locals.var_nl_dn6 = assign60320_e97997_d_n6;
        locals.var_nl_dn7 = assign60320_e97997_d_n7;
        locals.var_nl_dn8 = assign60320_e97997_d_n8;
        locals.var_nl_dn9 = assign60320_e97997_d_n9;
        locals.var_nl_dn10 = assign60320_e97997_d_n10;
        locals.var_nl_dn11 = assign60320_e97997_d_n11;

        let (assign60330_e98015, assign60330_e98015_d_n3, assign60330_e98015_d_n4, assign60330_e98015_d_n5, assign60330_e98015_d_n6, assign60330_e98015_d_n7, assign60330_e98015_d_n8, assign60330_e98015_d_n9, assign60330_e98015_d_n10, assign60330_e98015_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60330_e98004: f64 = (1.602176462e-19 * 1.602176462e-19);
        let assign60330_e98006: f64 = (assign60330_e98004 * 1.602176462e-19);
        let assign60330_e98008: f64 = (assign60330_e98006 * locals.var_vt);
        let assign60330_e98010: f64 = (locals.var_ids_edge).abs();
        let assign60330_e98011: f64 = (assign60330_e98008 * assign60330_e98010);
        let assign60330_e98013: f64 = (assign60330_e98011 * locals.var_ueff);
        (assign60330_e98013, (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn3 } else { (-locals.var_ids_edge_dn3) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn3)), (((((assign60330_e98006 * locals.var_vt_dn4) * assign60330_e98010) + (assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn4 } else { (-locals.var_ids_edge_dn4) })) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn4)), (((((assign60330_e98006 * locals.var_vt_dn5) * assign60330_e98010) + (assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn5 } else { (-locals.var_ids_edge_dn5) })) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn5)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn6 } else { (-locals.var_ids_edge_dn6) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn6)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn7 } else { (-locals.var_ids_edge_dn7) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn7)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn8 } else { (-locals.var_ids_edge_dn8) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn8)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn9 } else { (-locals.var_ids_edge_dn9) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn9)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn10 } else { (-locals.var_ids_edge_dn10) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn10)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn11 } else { (-locals.var_ids_edge_dn11) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn11)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11,)
    }
};
        locals.var_t0a = assign60330_e98015;
        locals.var_t0a_dn3 = assign60330_e98015_d_n3;
        locals.var_t0a_dn4 = assign60330_e98015_d_n4;
        locals.var_t0a_dn5 = assign60330_e98015_d_n5;
        locals.var_t0a_dn6 = assign60330_e98015_d_n6;
        locals.var_t0a_dn7 = assign60330_e98015_d_n7;
        locals.var_t0a_dn8 = assign60330_e98015_d_n8;
        locals.var_t0a_dn9 = assign60330_e98015_d_n9;
        locals.var_t0a_dn10 = assign60330_e98015_d_n10;
        locals.var_t0a_dn11 = assign60330_e98015_d_n11;

        let (assign60340_e98028, assign60340_e98028_d_n3, assign60340_e98028_d_n4, assign60340_e98028_d_n5, assign60340_e98028_d_n6, assign60340_e98028_d_n7, assign60340_e98028_d_n8, assign60340_e98028_d_n9, assign60340_e98028_d_n10, assign60340_e98028_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60340_e98022: f64 = (1.602176462e-19 * locals.var_vt);
        let assign60340_e98024: f64 = (assign60340_e98022 * locals.var_ids_edge);
        let assign60340_e98026: f64 = (assign60340_e98024 * locals.var_ids_edge);
        (assign60340_e98026, (((assign60340_e98022 * locals.var_ids_edge_dn3) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn3)), (((((1.602176462e-19 * locals.var_vt_dn4) * locals.var_ids_edge) + (assign60340_e98022 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn4)), (((((1.602176462e-19 * locals.var_vt_dn5) * locals.var_ids_edge) + (assign60340_e98022 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn5)), (((assign60340_e98022 * locals.var_ids_edge_dn6) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn6)), (((assign60340_e98022 * locals.var_ids_edge_dn7) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn7)), (((assign60340_e98022 * locals.var_ids_edge_dn8) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn8)), (((assign60340_e98022 * locals.var_ids_edge_dn9) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn9)), (((assign60340_e98022 * locals.var_ids_edge_dn10) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn10)), (((assign60340_e98022 * locals.var_ids_edge_dn11) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn11)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11,)
    }
};
        locals.var_t0b = assign60340_e98028;
        locals.var_t0b_dn3 = assign60340_e98028_d_n3;
        locals.var_t0b_dn4 = assign60340_e98028_d_n4;
        locals.var_t0b_dn5 = assign60340_e98028_d_n5;
        locals.var_t0b_dn6 = assign60340_e98028_d_n6;
        locals.var_t0b_dn7 = assign60340_e98028_d_n7;
        locals.var_t0b_dn8 = assign60340_e98028_d_n8;
        locals.var_t0b_dn9 = assign60340_e98028_d_n9;
        locals.var_t0b_dn10 = assign60340_e98028_d_n10;
        locals.var_t0b_dn11 = assign60340_e98028_d_n11;

        let (assign60350_e98045, assign60350_e98045_d_n3, assign60350_e98045_d_n4, assign60350_e98045_d_n5, assign60350_e98045_d_n6, assign60350_e98045_d_n7, assign60350_e98045_d_n8, assign60350_e98045_d_n9, assign60350_e98045_d_n10, assign60350_e98045_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60350_e98036: f64 = (locals.var_noib_edge * locals.var_nl);
        let assign60350_e98037: f64 = (locals.var_noia_edge + assign60350_e98036);
        let assign60350_e98040: f64 = (locals.var_noic_edge * locals.var_nl);
        let assign60350_e98042: f64 = (assign60350_e98040 * locals.var_nl);
        let assign60350_e98043: f64 = (assign60350_e98037 + assign60350_e98042);
        (assign60350_e98043, ((locals.var_noib_edge * locals.var_nl_dn3) + (((locals.var_noic_edge * locals.var_nl_dn3) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn3))), ((locals.var_noib_edge * locals.var_nl_dn4) + (((locals.var_noic_edge * locals.var_nl_dn4) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn4))), ((locals.var_noib_edge * locals.var_nl_dn5) + (((locals.var_noic_edge * locals.var_nl_dn5) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn5))), ((locals.var_noib_edge * locals.var_nl_dn6) + (((locals.var_noic_edge * locals.var_nl_dn6) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn6))), ((locals.var_noib_edge * locals.var_nl_dn7) + (((locals.var_noic_edge * locals.var_nl_dn7) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn7))), ((locals.var_noib_edge * locals.var_nl_dn8) + (((locals.var_noic_edge * locals.var_nl_dn8) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn8))), ((locals.var_noib_edge * locals.var_nl_dn9) + (((locals.var_noic_edge * locals.var_nl_dn9) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn9))), ((locals.var_noib_edge * locals.var_nl_dn10) + (((locals.var_noic_edge * locals.var_nl_dn10) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn10))), ((locals.var_noib_edge * locals.var_nl_dn11) + (((locals.var_noic_edge * locals.var_nl_dn11) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn11))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11,)
    }
};
        locals.var_t0c = assign60350_e98045;
        locals.var_t0c_dn3 = assign60350_e98045_d_n3;
        locals.var_t0c_dn4 = assign60350_e98045_d_n4;
        locals.var_t0c_dn5 = assign60350_e98045_d_n5;
        locals.var_t0c_dn6 = assign60350_e98045_d_n6;
        locals.var_t0c_dn7 = assign60350_e98045_d_n7;
        locals.var_t0c_dn8 = assign60350_e98045_d_n8;
        locals.var_t0c_dn9 = assign60350_e98045_d_n9;
        locals.var_t0c_dn10 = assign60350_e98045_d_n10;
        locals.var_t0c_dn11 = assign60350_e98045_d_n11;

        let (assign60360_e98058, assign60360_e98058_d_n3, assign60360_e98058_d_n4, assign60360_e98058_d_n5, assign60360_e98058_d_n6, assign60360_e98058_d_n7, assign60360_e98058_d_n8, assign60360_e98058_d_n9, assign60360_e98058_d_n10, assign60360_e98058_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60360_e98052: f64 = (locals.var_nl + locals.var_nstar);
        let assign60360_e98055: f64 = (locals.var_nl + locals.var_nstar);
        let assign60360_e98056: f64 = (assign60360_e98052 * assign60360_e98055);
        (assign60360_e98056, (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn11 + locals.var_nstar_dn11))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11,)
    }
};
        locals.var_t0d = assign60360_e98058;
        locals.var_t0d_dn3 = assign60360_e98058_d_n3;
        locals.var_t0d_dn4 = assign60360_e98058_d_n4;
        locals.var_t0d_dn5 = assign60360_e98058_d_n5;
        locals.var_t0d_dn6 = assign60360_e98058_d_n6;
        locals.var_t0d_dn7 = assign60360_e98058_d_n7;
        locals.var_t0d_dn8 = assign60360_e98058_d_n8;
        locals.var_t0d_dn9 = assign60360_e98058_d_n9;
        locals.var_t0d_dn10 = assign60360_e98058_d_n10;
        locals.var_t0d_dn11 = assign60360_e98058_d_n11;

        let (assign60370_e98069, assign60370_e98069_d_n4, assign60370_e98069_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60370_e98065: f64 = (locals.var_noia_edge * 1.602176462e-19);
        let assign60370_e98067: f64 = (assign60370_e98065 * locals.var_vt);
        (assign60370_e98067, (assign60370_e98065 * locals.var_vt_dn4), (assign60370_e98065 * locals.var_vt_dn5),)
    } else {
        (locals.var_t0e, locals.var_t0e_dn4, locals.var_t0e_dn5,)
    }
};
        locals.var_t0e = assign60370_e98069;
        locals.var_t0e_dn4 = assign60370_e98069_d_n4;
        locals.var_t0e_dn5 = assign60370_e98069_d_n5;

        let (assign60380_e98086, assign60380_e98086_d_n3, assign60380_e98086_d_n4, assign60380_e98086_d_n5, assign60380_e98086_d_n6, assign60380_e98086_d_n7, assign60380_e98086_d_n8, assign60380_e98086_d_n9, assign60380_e98086_d_n10, assign60380_e98086_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60380_e98076: f64 = (2.0 * locals.var_nq_edge);
        let assign60380_e98078: f64 = (assign60380_e98076 * locals.var_cox);
        let assign60380_e98080: f64 = (assign60380_e98078 * locals.var_vt);
        let assign60380_e98082: f64 = (assign60380_e98080 * locals.var_qs_edge);
        let assign60380_e98084: f64 = (assign60380_e98082 / 1.602176462e-19);
        (assign60380_e98084, ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn3)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign60380_e98078 * locals.var_vt_dn4)) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn4)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) + (assign60380_e98078 * locals.var_vt_dn5)) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn5)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn6)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn7)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn8)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn9)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn10)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11,)
    }
};
        locals.var_n0 = assign60380_e98086;
        locals.var_n0_dn3 = assign60380_e98086_d_n3;
        locals.var_n0_dn4 = assign60380_e98086_d_n4;
        locals.var_n0_dn5 = assign60380_e98086_d_n5;
        locals.var_n0_dn6 = assign60380_e98086_d_n6;
        locals.var_n0_dn7 = assign60380_e98086_d_n7;
        locals.var_n0_dn8 = assign60380_e98086_d_n8;
        locals.var_n0_dn9 = assign60380_e98086_d_n9;
        locals.var_n0_dn10 = assign60380_e98086_d_n10;
        locals.var_n0_dn11 = assign60380_e98086_d_n11;

        let (assign60390_e98104, assign60390_e98104_d_n3, assign60390_e98104_d_n4, assign60390_e98104_d_n5, assign60390_e98104_d_n6, assign60390_e98104_d_n7, assign60390_e98104_d_n8, assign60390_e98104_d_n9, assign60390_e98104_d_n10, assign60390_e98104_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60390_e98094: f64 = (locals.var_n0 + locals.var_nstar);
        let assign60390_e98097: f64 = (locals.var_nl + locals.var_nstar);
        let assign60390_e98098: f64 = (assign60390_e98094 / assign60390_e98097);
        let assign60390_e98100: f64 = (assign60390_e98098).max(1e-38);
        let assign60390_e98101: f64 = (assign60390_e98100).ln();
        let assign60390_e98102: f64 = (locals.var_noia_edge * assign60390_e98101);
        (assign60390_e98102, (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign60390_e98104;
        locals.var_t1_dn3 = assign60390_e98104_d_n3;
        locals.var_t1_dn4 = assign60390_e98104_d_n4;
        locals.var_t1_dn5 = assign60390_e98104_d_n5;
        locals.var_t1_dn6 = assign60390_e98104_d_n6;
        locals.var_t1_dn7 = assign60390_e98104_d_n7;
        locals.var_t1_dn8 = assign60390_e98104_d_n8;
        locals.var_t1_dn9 = assign60390_e98104_d_n9;
        locals.var_t1_dn10 = assign60390_e98104_d_n10;
        locals.var_t1_dn11 = assign60390_e98104_d_n11;

        let (assign60400_e98115, assign60400_e98115_d_n3, assign60400_e98115_d_n4, assign60400_e98115_d_n5, assign60400_e98115_d_n6, assign60400_e98115_d_n7, assign60400_e98115_d_n8, assign60400_e98115_d_n9, assign60400_e98115_d_n10, assign60400_e98115_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60400_e98112: f64 = (locals.var_n0 - locals.var_nl);
        let assign60400_e98113: f64 = (locals.var_noib_edge * assign60400_e98112);
        (assign60400_e98113, (locals.var_noib_edge * (locals.var_n0_dn3 - locals.var_nl_dn3)), (locals.var_noib_edge * (locals.var_n0_dn4 - locals.var_nl_dn4)), (locals.var_noib_edge * (locals.var_n0_dn5 - locals.var_nl_dn5)), (locals.var_noib_edge * (locals.var_n0_dn6 - locals.var_nl_dn6)), (locals.var_noib_edge * (locals.var_n0_dn7 - locals.var_nl_dn7)), (locals.var_noib_edge * (locals.var_n0_dn8 - locals.var_nl_dn8)), (locals.var_noib_edge * (locals.var_n0_dn9 - locals.var_nl_dn9)), (locals.var_noib_edge * (locals.var_n0_dn10 - locals.var_nl_dn10)), (locals.var_noib_edge * (locals.var_n0_dn11 - locals.var_nl_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign60400_e98115;
        locals.var_t2_dn3 = assign60400_e98115_d_n3;
        locals.var_t2_dn4 = assign60400_e98115_d_n4;
        locals.var_t2_dn5 = assign60400_e98115_d_n5;
        locals.var_t2_dn6 = assign60400_e98115_d_n6;
        locals.var_t2_dn7 = assign60400_e98115_d_n7;
        locals.var_t2_dn8 = assign60400_e98115_d_n8;
        locals.var_t2_dn9 = assign60400_e98115_d_n9;
        locals.var_t2_dn10 = assign60400_e98115_d_n10;
        locals.var_t2_dn11 = assign60400_e98115_d_n11;

        let (assign60410_e98132, assign60410_e98132_d_n3, assign60410_e98132_d_n4, assign60410_e98132_d_n5, assign60410_e98132_d_n6, assign60410_e98132_d_n7, assign60410_e98132_d_n8, assign60410_e98132_d_n9, assign60410_e98132_d_n10, assign60410_e98132_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60410_e98122: f64 = (0.5 * locals.var_noic_edge);
        let assign60410_e98125: f64 = (locals.var_n0 * locals.var_n0);
        let assign60410_e98128: f64 = (locals.var_nl * locals.var_nl);
        let assign60410_e98129: f64 = (assign60410_e98125 - assign60410_e98128);
        let assign60410_e98130: f64 = (assign60410_e98122 * assign60410_e98129);
        (assign60410_e98130, (assign60410_e98122 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign60410_e98122 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign60410_e98122 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign60410_e98122 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign60410_e98122 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign60410_e98122 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign60410_e98122 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign60410_e98122 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign60410_e98122 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60410_e98132;
        locals.var_t3_dn3 = assign60410_e98132_d_n3;
        locals.var_t3_dn4 = assign60410_e98132_d_n4;
        locals.var_t3_dn5 = assign60410_e98132_d_n5;
        locals.var_t3_dn6 = assign60410_e98132_d_n6;
        locals.var_t3_dn7 = assign60410_e98132_d_n7;
        locals.var_t3_dn8 = assign60410_e98132_d_n8;
        locals.var_t3_dn9 = assign60410_e98132_d_n9;
        locals.var_t3_dn10 = assign60410_e98132_d_n10;
        locals.var_t3_dn11 = assign60410_e98132_d_n11;

        let (assign60420_e98145, assign60420_e98145_d_n3, assign60420_e98145_d_n4, assign60420_e98145_d_n5, assign60420_e98145_d_n6, assign60420_e98145_d_n7, assign60420_e98145_d_n8, assign60420_e98145_d_n9, assign60420_e98145_d_n10, assign60420_e98145_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60420_e98139: f64 = (10000000000.0 * locals.var_leffnoisq_edge);
        let assign60420_e98141: f64 = (assign60420_e98139 * p.p1147);
        let assign60420_e98143: f64 = (assign60420_e98141 * p.p2);
        (assign60420_e98143, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign60420_e98145;
        locals.var_t4_dn3 = assign60420_e98145_d_n3;
        locals.var_t4_dn4 = assign60420_e98145_d_n4;
        locals.var_t4_dn5 = assign60420_e98145_d_n5;
        locals.var_t4_dn6 = assign60420_e98145_d_n6;
        locals.var_t4_dn7 = assign60420_e98145_d_n7;
        locals.var_t4_dn8 = assign60420_e98145_d_n8;
        locals.var_t4_dn9 = assign60420_e98145_d_n9;
        locals.var_t4_dn10 = assign60420_e98145_d_n10;
        locals.var_t4_dn11 = assign60420_e98145_d_n11;

        let (assign60430_e98170, assign60430_e98170_d_n3, assign60430_e98170_d_n4, assign60430_e98170_d_n5, assign60430_e98170_d_n6, assign60430_e98170_d_n7, assign60430_e98170_d_n8, assign60430_e98170_d_n9, assign60430_e98170_d_n10, assign60430_e98170_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60430_e98152: f64 = (locals.var_t0a / locals.var_t0);
        let assign60430_e98155: f64 = (locals.var_t1 + locals.var_t2);
        let assign60430_e98157: f64 = (assign60430_e98155 + locals.var_t3);
        let assign60430_e98158: f64 = (assign60430_e98152 * assign60430_e98157);
        let assign60430_e98161: f64 = (locals.var_t0b / locals.var_t4);
        let assign60430_e98163: f64 = (assign60430_e98161 * locals.var_delclm);
        let assign60430_e98165: f64 = (assign60430_e98163 * locals.var_t0c);
        let assign60430_e98167: f64 = (assign60430_e98165 / locals.var_t0d);
        let assign60430_e98168: f64 = (assign60430_e98158 + assign60430_e98167);
        (assign60430_e98168, ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11,)
    }
};
        locals.var_ssi = assign60430_e98170;
        locals.var_ssi_dn3 = assign60430_e98170_d_n3;
        locals.var_ssi_dn4 = assign60430_e98170_d_n4;
        locals.var_ssi_dn5 = assign60430_e98170_d_n5;
        locals.var_ssi_dn6 = assign60430_e98170_d_n6;
        locals.var_ssi_dn7 = assign60430_e98170_d_n7;
        locals.var_ssi_dn8 = assign60430_e98170_d_n8;
        locals.var_ssi_dn9 = assign60430_e98170_d_n9;
        locals.var_ssi_dn10 = assign60430_e98170_d_n10;
        locals.var_ssi_dn11 = assign60430_e98170_d_n11;

        let (assign60440_e98187, assign60440_e98187_d_n3, assign60440_e98187_d_n4, assign60440_e98187_d_n5, assign60440_e98187_d_n6, assign60440_e98187_d_n7, assign60440_e98187_d_n8, assign60440_e98187_d_n9, assign60440_e98187_d_n10, assign60440_e98187_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60440_e98177: f64 = (p.p1147 * p.p2);
        let assign60440_e98179: f64 = (assign60440_e98177 * locals.var_leffnoi_edge);
        let assign60440_e98181: f64 = (assign60440_e98179 * 10000000000.0);
        let assign60440_e98183: f64 = (assign60440_e98181 * locals.var_nstar);
        let assign60440_e98185: f64 = (assign60440_e98183 * locals.var_nstar);
        (assign60440_e98185, (((assign60440_e98181 * locals.var_nstar_dn3) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn3)), (((assign60440_e98181 * locals.var_nstar_dn4) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn4)), (((assign60440_e98181 * locals.var_nstar_dn5) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn5)), (((assign60440_e98181 * locals.var_nstar_dn6) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn6)), (((assign60440_e98181 * locals.var_nstar_dn7) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn7)), (((assign60440_e98181 * locals.var_nstar_dn8) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn8)), (((assign60440_e98181 * locals.var_nstar_dn9) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn9)), (((assign60440_e98181 * locals.var_nstar_dn10) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn10)), (((assign60440_e98181 * locals.var_nstar_dn11) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign60440_e98187;
        locals.var_t5_dn3 = assign60440_e98187_d_n3;
        locals.var_t5_dn4 = assign60440_e98187_d_n4;
        locals.var_t5_dn5 = assign60440_e98187_d_n5;
        locals.var_t5_dn6 = assign60440_e98187_d_n6;
        locals.var_t5_dn7 = assign60440_e98187_d_n7;
        locals.var_t5_dn8 = assign60440_e98187_d_n8;
        locals.var_t5_dn9 = assign60440_e98187_d_n9;
        locals.var_t5_dn10 = assign60440_e98187_d_n10;
        locals.var_t5_dn11 = assign60440_e98187_d_n11;

        let (assign60450_e98200, assign60450_e98200_d_n3, assign60450_e98200_d_n4, assign60450_e98200_d_n5, assign60450_e98200_d_n6, assign60450_e98200_d_n7, assign60450_e98200_d_n8, assign60450_e98200_d_n9, assign60450_e98200_d_n10, assign60450_e98200_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60450_e98194: f64 = (locals.var_t0e / locals.var_t5);
        let assign60450_e98196: f64 = (assign60450_e98194 * locals.var_ids_edge);
        let assign60450_e98198: f64 = (assign60450_e98196 * locals.var_ids_edge);
        (assign60450_e98198, (((((-((locals.var_t0e * locals.var_t5_dn3) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn3)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn5)), (((((-((locals.var_t0e * locals.var_t5_dn6) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn6)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn6)), (((((-((locals.var_t0e * locals.var_t5_dn7) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn7)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn7)), (((((-((locals.var_t0e * locals.var_t5_dn8) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn8)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn8)), (((((-((locals.var_t0e * locals.var_t5_dn9) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn9)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn9)), (((((-((locals.var_t0e * locals.var_t5_dn10) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn10)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn10)), (((((-((locals.var_t0e * locals.var_t5_dn11) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn11)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn11)),)
    } else {
        (locals.var_swi, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11,)
    }
};
        locals.var_swi = assign60450_e98200;
        locals.var_swi_dn3 = assign60450_e98200_d_n3;
        locals.var_swi_dn4 = assign60450_e98200_d_n4;
        locals.var_swi_dn5 = assign60450_e98200_d_n5;
        locals.var_swi_dn6 = assign60450_e98200_d_n6;
        locals.var_swi_dn7 = assign60450_e98200_d_n7;
        locals.var_swi_dn8 = assign60450_e98200_d_n8;
        locals.var_swi_dn9 = assign60450_e98200_d_n9;
        locals.var_swi_dn10 = assign60450_e98200_d_n10;
        locals.var_swi_dn11 = assign60450_e98200_d_n11;

        let (assign60460_e98209, assign60460_e98209_d_n3, assign60460_e98209_d_n4, assign60460_e98209_d_n5, assign60460_e98209_d_n6, assign60460_e98209_d_n7, assign60460_e98209_d_n8, assign60460_e98209_d_n9, assign60460_e98209_d_n10, assign60460_e98209_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60460_e98207: f64 = (locals.var_swi + locals.var_ssi);
        (assign60460_e98207, (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign60460_e98209;
        locals.var_t6_dn3 = assign60460_e98209_d_n3;
        locals.var_t6_dn4 = assign60460_e98209_d_n4;
        locals.var_t6_dn5 = assign60460_e98209_d_n5;
        locals.var_t6_dn6 = assign60460_e98209_d_n6;
        locals.var_t6_dn7 = assign60460_e98209_d_n7;
        locals.var_t6_dn8 = assign60460_e98209_d_n8;
        locals.var_t6_dn9 = assign60460_e98209_d_n9;
        locals.var_t6_dn10 = assign60460_e98209_d_n10;
        locals.var_t6_dn11 = assign60460_e98209_d_n11;

        let assign60470_e98212: f64 = if locals.var_t6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign60470_e98212;

        let (assign60480_e98225, assign60480_e98225_d_n3, assign60480_e98225_d_n4, assign60480_e98225_d_n5, assign60480_e98225_d_n6, assign60480_e98225_d_n7, assign60480_e98225_d_n8, assign60480_e98225_d_n9, assign60480_e98225_d_n10, assign60480_e98225_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign60480_e98221: f64 = (locals.var_ssi * locals.var_swi);
        let assign60480_e98223: f64 = (assign60480_e98221 / locals.var_t6);
        (assign60480_e98223, (((((locals.var_ssi_dn3 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn3)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn4 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn4)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn5 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn5)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn6 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn6)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn7 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn7)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn8 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn8)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn9 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn9)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn10 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn10)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn11 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn11)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign60480_e98225;
        locals.var_t7_dn3 = assign60480_e98225_d_n3;
        locals.var_t7_dn4 = assign60480_e98225_d_n4;
        locals.var_t7_dn5 = assign60480_e98225_d_n5;
        locals.var_t7_dn6 = assign60480_e98225_d_n6;
        locals.var_t7_dn7 = assign60480_e98225_d_n7;
        locals.var_t7_dn8 = assign60480_e98225_d_n8;
        locals.var_t7_dn9 = assign60480_e98225_d_n9;
        locals.var_t7_dn10 = assign60480_e98225_d_n10;
        locals.var_t7_dn11 = assign60480_e98225_d_n11;

        let (assign60490_e98242, assign60490_e98242_d_n3, assign60490_e98242_d_n4, assign60490_e98242_d_n5, assign60490_e98242_d_n6, assign60490_e98242_d_n7, assign60490_e98242_d_n8, assign60490_e98242_d_n9, assign60490_e98242_d_n10, assign60490_e98242_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign60490_e98236: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign60490_e98238: f64 = (assign60490_e98236).powf(p.p1318);
        let assign60490_e98239: f64 = (p.p1317 * assign60490_e98238);
        let assign60490_e98240: f64 = (1.0 + assign60490_e98239);
        (assign60490_e98240, (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) / assign60490_e98236))) }),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign60490_e98242;
        locals.var_t8_dn3 = assign60490_e98242_d_n3;
        locals.var_t8_dn4 = assign60490_e98242_d_n4;
        locals.var_t8_dn5 = assign60490_e98242_d_n5;
        locals.var_t8_dn6 = assign60490_e98242_d_n6;
        locals.var_t8_dn7 = assign60490_e98242_d_n7;
        locals.var_t8_dn8 = assign60490_e98242_d_n8;
        locals.var_t8_dn9 = assign60490_e98242_d_n9;
        locals.var_t8_dn10 = assign60490_e98242_d_n10;
        locals.var_t8_dn11 = assign60490_e98242_d_n11;

        let (assign60520_e98276, assign60520_e98276_d_n3, assign60520_e98276_d_n4, assign60520_e98276_d_n5, assign60520_e98276_d_n6, assign60520_e98276_d_n7, assign60520_e98276_d_n8, assign60520_e98276_d_n9, assign60520_e98276_d_n10, assign60520_e98276_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60520_e98269: f64 = (locals.var_qbi + locals.var_qovb);
        let assign60520_e98271: f64 = (assign60520_e98269 + locals.var_qbsj);
        let assign60520_e98273: f64 = (assign60520_e98271 + locals.var_qbdj);
        let assign60520_e98274: f64 = (locals.var_devsign * assign60520_e98273);
        (assign60520_e98274, (locals.var_devsign * ((locals.var_qbi_dn3 + locals.var_qbsj_dn3) + locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qbi_dn4 + locals.var_qbsj_dn4) + locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qbi_dn5 + locals.var_qbsj_dn5) + locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qbi_dn6 + locals.var_qbsj_dn6) + locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qbi_dn7 + locals.var_qbsj_dn7) + locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qbi_dn8 + locals.var_qbsj_dn8) + locals.var_qbdj_dn8)), (locals.var_devsign * (((locals.var_qbi_dn9 + locals.var_qovb_dn9) + locals.var_qbsj_dn9) + locals.var_qbdj_dn9)), (locals.var_devsign * (((locals.var_qbi_dn10 + locals.var_qovb_dn10) + locals.var_qbsj_dn10) + locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qbi_dn11 + locals.var_qbsj_dn11) + locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qb_2, locals.var_qb_2_dn3, locals.var_qb_2_dn4, locals.var_qb_2_dn5, locals.var_qb_2_dn6, locals.var_qb_2_dn7, locals.var_qb_2_dn8, locals.var_qb_2_dn9, locals.var_qb_2_dn10, locals.var_qb_2_dn11,)
    }
};
        locals.var_qb_2 = assign60520_e98276;
        locals.var_qb_2_dn3 = assign60520_e98276_d_n3;
        locals.var_qb_2_dn4 = assign60520_e98276_d_n4;
        locals.var_qb_2_dn5 = assign60520_e98276_d_n5;
        locals.var_qb_2_dn6 = assign60520_e98276_d_n6;
        locals.var_qb_2_dn7 = assign60520_e98276_d_n7;
        locals.var_qb_2_dn8 = assign60520_e98276_d_n8;
        locals.var_qb_2_dn9 = assign60520_e98276_d_n9;
        locals.var_qb_2_dn10 = assign60520_e98276_d_n10;
        locals.var_qb_2_dn11 = assign60520_e98276_d_n11;

        let assign60530_e98279: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard878 = assign60530_e98279;

        let (assign60540_e98288, assign60540_e98288_d_n3, assign60540_e98288_d_n4, assign60540_e98288_d_n5, assign60540_e98288_d_n6, assign60540_e98288_d_n7, assign60540_e98288_d_n8, assign60540_e98288_d_n9, assign60540_e98288_d_n10, assign60540_e98288_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60540_e98286: f64 = (locals.var_devsign * locals.var_qsi);
        (assign60540_e98286, (locals.var_devsign * locals.var_qsi_dn3), (locals.var_devsign * locals.var_qsi_dn4), (locals.var_devsign * locals.var_qsi_dn5), (locals.var_devsign * locals.var_qsi_dn6), (locals.var_devsign * locals.var_qsi_dn7), (locals.var_devsign * locals.var_qsi_dn8), (locals.var_devsign * locals.var_qsi_dn9), (locals.var_devsign * locals.var_qsi_dn10), (locals.var_devsign * locals.var_qsi_dn11),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11,)
    }
};
        locals.var_qsi_1 = assign60540_e98288;
        locals.var_qsi_1_dn3 = assign60540_e98288_d_n3;
        locals.var_qsi_1_dn4 = assign60540_e98288_d_n4;
        locals.var_qsi_1_dn5 = assign60540_e98288_d_n5;
        locals.var_qsi_1_dn6 = assign60540_e98288_d_n6;
        locals.var_qsi_1_dn7 = assign60540_e98288_d_n7;
        locals.var_qsi_1_dn8 = assign60540_e98288_d_n8;
        locals.var_qsi_1_dn9 = assign60540_e98288_d_n9;
        locals.var_qsi_1_dn10 = assign60540_e98288_d_n10;
        locals.var_qsi_1_dn11 = assign60540_e98288_d_n11;

        let (assign60570_e98315, assign60570_e98315_d_n3, assign60570_e98315_d_n4, assign60570_e98315_d_n5, assign60570_e98315_d_n6, assign60570_e98315_d_n7, assign60570_e98315_d_n8, assign60570_e98315_d_n9, assign60570_e98315_d_n10, assign60570_e98315_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60570_e98313: f64 = (locals.var_devsign * locals.var_qdi);
        (assign60570_e98313, (locals.var_devsign * locals.var_qdi_dn3), (locals.var_devsign * locals.var_qdi_dn4), (locals.var_devsign * locals.var_qdi_dn5), (locals.var_devsign * locals.var_qdi_dn6), (locals.var_devsign * locals.var_qdi_dn7), (locals.var_devsign * locals.var_qdi_dn8), (locals.var_devsign * locals.var_qdi_dn9), (locals.var_devsign * locals.var_qdi_dn10), (locals.var_devsign * locals.var_qdi_dn11),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11,)
    }
};
        locals.var_qdi_1 = assign60570_e98315;
        locals.var_qdi_1_dn3 = assign60570_e98315_d_n3;
        locals.var_qdi_1_dn4 = assign60570_e98315_d_n4;
        locals.var_qdi_1_dn5 = assign60570_e98315_d_n5;
        locals.var_qdi_1_dn6 = assign60570_e98315_d_n6;
        locals.var_qdi_1_dn7 = assign60570_e98315_d_n7;
        locals.var_qdi_1_dn8 = assign60570_e98315_d_n8;
        locals.var_qdi_1_dn9 = assign60570_e98315_d_n9;
        locals.var_qdi_1_dn10 = assign60570_e98315_d_n10;
        locals.var_qdi_1_dn11 = assign60570_e98315_d_n11;

        let (assign60600_e98346, assign60600_e98346_d_n3, assign60600_e98346_d_n4, assign60600_e98346_d_n5, assign60600_e98346_d_n6, assign60600_e98346_d_n7, assign60600_e98346_d_n8, assign60600_e98346_d_n9, assign60600_e98346_d_n10, assign60600_e98346_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60600_e98341: f64 = (locals.var_qsi + locals.var_qovs);
        let assign60600_e98343: f64 = (assign60600_e98341 - locals.var_qbsj);
        let assign60600_e98344: f64 = (locals.var_devsign * assign60600_e98343);
        (assign60600_e98344, (locals.var_devsign * ((locals.var_qsi_dn3 + locals.var_qovs_dn3) - locals.var_qbsj_dn3)), (locals.var_devsign * ((locals.var_qsi_dn4 + locals.var_qovs_dn4) - locals.var_qbsj_dn4)), (locals.var_devsign * ((locals.var_qsi_dn5 + locals.var_qovs_dn5) - locals.var_qbsj_dn5)), (locals.var_devsign * ((locals.var_qsi_dn6 + locals.var_qovs_dn6) - locals.var_qbsj_dn6)), (locals.var_devsign * ((locals.var_qsi_dn7 + locals.var_qovs_dn7) - locals.var_qbsj_dn7)), (locals.var_devsign * ((locals.var_qsi_dn8 + locals.var_qovs_dn8) - locals.var_qbsj_dn8)), (locals.var_devsign * ((locals.var_qsi_dn9 + locals.var_qovs_dn9) - locals.var_qbsj_dn9)), (locals.var_devsign * ((locals.var_qsi_dn10 + locals.var_qovs_dn10) - locals.var_qbsj_dn10)), (locals.var_devsign * ((locals.var_qsi_dn11 + locals.var_qovs_dn11) - locals.var_qbsj_dn11)),)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign60600_e98346;
        locals.var_qs_2_dn3 = assign60600_e98346_d_n3;
        locals.var_qs_2_dn4 = assign60600_e98346_d_n4;
        locals.var_qs_2_dn5 = assign60600_e98346_d_n5;
        locals.var_qs_2_dn6 = assign60600_e98346_d_n6;
        locals.var_qs_2_dn7 = assign60600_e98346_d_n7;
        locals.var_qs_2_dn8 = assign60600_e98346_d_n8;
        locals.var_qs_2_dn9 = assign60600_e98346_d_n9;
        locals.var_qs_2_dn10 = assign60600_e98346_d_n10;
        locals.var_qs_2_dn11 = assign60600_e98346_d_n11;

        let (assign60610_e98359, assign60610_e98359_d_n3, assign60610_e98359_d_n4, assign60610_e98359_d_n5, assign60610_e98359_d_n6, assign60610_e98359_d_n7, assign60610_e98359_d_n8, assign60610_e98359_d_n9, assign60610_e98359_d_n10, assign60610_e98359_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60610_e98354: f64 = (locals.var_qdi + locals.var_qovd);
        let assign60610_e98356: f64 = (assign60610_e98354 - locals.var_qbdj);
        let assign60610_e98357: f64 = (locals.var_devsign * assign60610_e98356);
        (assign60610_e98357, (locals.var_devsign * ((locals.var_qdi_dn3 + locals.var_qovd_dn3) - locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qdi_dn4 + locals.var_qovd_dn4) - locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qdi_dn5 + locals.var_qovd_dn5) - locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qdi_dn6 + locals.var_qovd_dn6) - locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qdi_dn7 + locals.var_qovd_dn7) - locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qdi_dn8 + locals.var_qovd_dn8) - locals.var_qbdj_dn8)), (locals.var_devsign * ((locals.var_qdi_dn9 + locals.var_qovd_dn9) - locals.var_qbdj_dn9)), (locals.var_devsign * ((locals.var_qdi_dn10 + locals.var_qovd_dn10) - locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qdi_dn11 + locals.var_qovd_dn11) - locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign60610_e98359;
        locals.var_qd_1_dn3 = assign60610_e98359_d_n3;
        locals.var_qd_1_dn4 = assign60610_e98359_d_n4;
        locals.var_qd_1_dn5 = assign60610_e98359_d_n5;
        locals.var_qd_1_dn6 = assign60610_e98359_d_n6;
        locals.var_qd_1_dn7 = assign60610_e98359_d_n7;
        locals.var_qd_1_dn8 = assign60610_e98359_d_n8;
        locals.var_qd_1_dn9 = assign60610_e98359_d_n9;
        locals.var_qd_1_dn10 = assign60610_e98359_d_n10;
        locals.var_qd_1_dn11 = assign60610_e98359_d_n11;

        let (assign60620_e98369, assign60620_e98369_d_n3, assign60620_e98369_d_n4, assign60620_e98369_d_n5, assign60620_e98369_d_n6, assign60620_e98369_d_n7, assign60620_e98369_d_n8, assign60620_e98369_d_n9, assign60620_e98369_d_n10, assign60620_e98369_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60620_e98367: f64 = (locals.var_devsign * locals.var_qdi);
        (assign60620_e98367, (locals.var_devsign * locals.var_qdi_dn3), (locals.var_devsign * locals.var_qdi_dn4), (locals.var_devsign * locals.var_qdi_dn5), (locals.var_devsign * locals.var_qdi_dn6), (locals.var_devsign * locals.var_qdi_dn7), (locals.var_devsign * locals.var_qdi_dn8), (locals.var_devsign * locals.var_qdi_dn9), (locals.var_devsign * locals.var_qdi_dn10), (locals.var_devsign * locals.var_qdi_dn11),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11,)
    }
};
        locals.var_qsi_1 = assign60620_e98369;
        locals.var_qsi_1_dn3 = assign60620_e98369_d_n3;
        locals.var_qsi_1_dn4 = assign60620_e98369_d_n4;
        locals.var_qsi_1_dn5 = assign60620_e98369_d_n5;
        locals.var_qsi_1_dn6 = assign60620_e98369_d_n6;
        locals.var_qsi_1_dn7 = assign60620_e98369_d_n7;
        locals.var_qsi_1_dn8 = assign60620_e98369_d_n8;
        locals.var_qsi_1_dn9 = assign60620_e98369_d_n9;
        locals.var_qsi_1_dn10 = assign60620_e98369_d_n10;
        locals.var_qsi_1_dn11 = assign60620_e98369_d_n11;

    }

    pub(super) fn stamp_transient_block_201(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign60650_e98399, assign60650_e98399_d_n3, assign60650_e98399_d_n4, assign60650_e98399_d_n5, assign60650_e98399_d_n6, assign60650_e98399_d_n7, assign60650_e98399_d_n8, assign60650_e98399_d_n9, assign60650_e98399_d_n10, assign60650_e98399_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60650_e98397: f64 = (locals.var_devsign * locals.var_qsi);
        (assign60650_e98397, (locals.var_devsign * locals.var_qsi_dn3), (locals.var_devsign * locals.var_qsi_dn4), (locals.var_devsign * locals.var_qsi_dn5), (locals.var_devsign * locals.var_qsi_dn6), (locals.var_devsign * locals.var_qsi_dn7), (locals.var_devsign * locals.var_qsi_dn8), (locals.var_devsign * locals.var_qsi_dn9), (locals.var_devsign * locals.var_qsi_dn10), (locals.var_devsign * locals.var_qsi_dn11),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11,)
    }
};
        locals.var_qdi_1 = assign60650_e98399;
        locals.var_qdi_1_dn3 = assign60650_e98399_d_n3;
        locals.var_qdi_1_dn4 = assign60650_e98399_d_n4;
        locals.var_qdi_1_dn5 = assign60650_e98399_d_n5;
        locals.var_qdi_1_dn6 = assign60650_e98399_d_n6;
        locals.var_qdi_1_dn7 = assign60650_e98399_d_n7;
        locals.var_qdi_1_dn8 = assign60650_e98399_d_n8;
        locals.var_qdi_1_dn9 = assign60650_e98399_d_n9;
        locals.var_qdi_1_dn10 = assign60650_e98399_d_n10;
        locals.var_qdi_1_dn11 = assign60650_e98399_d_n11;

        let (assign60680_e98433, assign60680_e98433_d_n3, assign60680_e98433_d_n4, assign60680_e98433_d_n5, assign60680_e98433_d_n6, assign60680_e98433_d_n7, assign60680_e98433_d_n8, assign60680_e98433_d_n9, assign60680_e98433_d_n10, assign60680_e98433_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60680_e98428: f64 = (locals.var_qdi + locals.var_qovs);
        let assign60680_e98430: f64 = (assign60680_e98428 - locals.var_qbsj);
        let assign60680_e98431: f64 = (locals.var_devsign * assign60680_e98430);
        (assign60680_e98431, (locals.var_devsign * ((locals.var_qdi_dn3 + locals.var_qovs_dn3) - locals.var_qbsj_dn3)), (locals.var_devsign * ((locals.var_qdi_dn4 + locals.var_qovs_dn4) - locals.var_qbsj_dn4)), (locals.var_devsign * ((locals.var_qdi_dn5 + locals.var_qovs_dn5) - locals.var_qbsj_dn5)), (locals.var_devsign * ((locals.var_qdi_dn6 + locals.var_qovs_dn6) - locals.var_qbsj_dn6)), (locals.var_devsign * ((locals.var_qdi_dn7 + locals.var_qovs_dn7) - locals.var_qbsj_dn7)), (locals.var_devsign * ((locals.var_qdi_dn8 + locals.var_qovs_dn8) - locals.var_qbsj_dn8)), (locals.var_devsign * ((locals.var_qdi_dn9 + locals.var_qovs_dn9) - locals.var_qbsj_dn9)), (locals.var_devsign * ((locals.var_qdi_dn10 + locals.var_qovs_dn10) - locals.var_qbsj_dn10)), (locals.var_devsign * ((locals.var_qdi_dn11 + locals.var_qovs_dn11) - locals.var_qbsj_dn11)),)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign60680_e98433;
        locals.var_qs_2_dn3 = assign60680_e98433_d_n3;
        locals.var_qs_2_dn4 = assign60680_e98433_d_n4;
        locals.var_qs_2_dn5 = assign60680_e98433_d_n5;
        locals.var_qs_2_dn6 = assign60680_e98433_d_n6;
        locals.var_qs_2_dn7 = assign60680_e98433_d_n7;
        locals.var_qs_2_dn8 = assign60680_e98433_d_n8;
        locals.var_qs_2_dn9 = assign60680_e98433_d_n9;
        locals.var_qs_2_dn10 = assign60680_e98433_d_n10;
        locals.var_qs_2_dn11 = assign60680_e98433_d_n11;

        let (assign60690_e98447, assign60690_e98447_d_n3, assign60690_e98447_d_n4, assign60690_e98447_d_n5, assign60690_e98447_d_n6, assign60690_e98447_d_n7, assign60690_e98447_d_n8, assign60690_e98447_d_n9, assign60690_e98447_d_n10, assign60690_e98447_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60690_e98442: f64 = (locals.var_qsi + locals.var_qovd);
        let assign60690_e98444: f64 = (assign60690_e98442 - locals.var_qbdj);
        let assign60690_e98445: f64 = (locals.var_devsign * assign60690_e98444);
        (assign60690_e98445, (locals.var_devsign * ((locals.var_qsi_dn3 + locals.var_qovd_dn3) - locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qsi_dn4 + locals.var_qovd_dn4) - locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qsi_dn5 + locals.var_qovd_dn5) - locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qsi_dn6 + locals.var_qovd_dn6) - locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qsi_dn7 + locals.var_qovd_dn7) - locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qsi_dn8 + locals.var_qovd_dn8) - locals.var_qbdj_dn8)), (locals.var_devsign * ((locals.var_qsi_dn9 + locals.var_qovd_dn9) - locals.var_qbdj_dn9)), (locals.var_devsign * ((locals.var_qsi_dn10 + locals.var_qovd_dn10) - locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qsi_dn11 + locals.var_qovd_dn11) - locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign60690_e98447;
        locals.var_qd_1_dn3 = assign60690_e98447_d_n3;
        locals.var_qd_1_dn4 = assign60690_e98447_d_n4;
        locals.var_qd_1_dn5 = assign60690_e98447_d_n5;
        locals.var_qd_1_dn6 = assign60690_e98447_d_n6;
        locals.var_qd_1_dn7 = assign60690_e98447_d_n7;
        locals.var_qd_1_dn8 = assign60690_e98447_d_n8;
        locals.var_qd_1_dn9 = assign60690_e98447_d_n9;
        locals.var_qd_1_dn10 = assign60690_e98447_d_n10;
        locals.var_qd_1_dn11 = assign60690_e98447_d_n11;

        let (assign60700_e98456, assign60700_e98456_d_n3, assign60700_e98456_d_n4, assign60700_e98456_d_n5, assign60700_e98456_d_n6, assign60700_e98456_d_n7, assign60700_e98456_d_n8, assign60700_e98456_d_n9, assign60700_e98456_d_n10, assign60700_e98456_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60700_e98453: f64 = (locals.var_qgi + locals.var_qovg);
        let assign60700_e98454: f64 = (locals.var_devsign * assign60700_e98453);
        (assign60700_e98454, (locals.var_devsign * (locals.var_qgi_dn3 + locals.var_qovg_dn3)), (locals.var_devsign * (locals.var_qgi_dn4 + locals.var_qovg_dn4)), (locals.var_devsign * (locals.var_qgi_dn5 + locals.var_qovg_dn5)), (locals.var_devsign * (locals.var_qgi_dn6 + locals.var_qovg_dn6)), (locals.var_devsign * (locals.var_qgi_dn7 + locals.var_qovg_dn7)), (locals.var_devsign * (locals.var_qgi_dn8 + locals.var_qovg_dn8)), (locals.var_devsign * (locals.var_qgi_dn9 + locals.var_qovg_dn9)), (locals.var_devsign * (locals.var_qgi_dn10 + locals.var_qovg_dn10)), (locals.var_devsign * (locals.var_qgi_dn11 + locals.var_qovg_dn11)),)
    } else {
        (locals.var_qg, locals.var_qg_dn3, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11,)
    }
};
        locals.var_qg = assign60700_e98456;
        locals.var_qg_dn3 = assign60700_e98456_d_n3;
        locals.var_qg_dn4 = assign60700_e98456_d_n4;
        locals.var_qg_dn5 = assign60700_e98456_d_n5;
        locals.var_qg_dn6 = assign60700_e98456_d_n6;
        locals.var_qg_dn7 = assign60700_e98456_d_n7;
        locals.var_qg_dn8 = assign60700_e98456_d_n8;
        locals.var_qg_dn9 = assign60700_e98456_d_n9;
        locals.var_qg_dn10 = assign60700_e98456_d_n10;
        locals.var_qg_dn11 = assign60700_e98456_d_n11;

        locals.var_weff_1 = locals.var_weff;

        locals.var_leff_1 = locals.var_leff;

        let assign61440_e98842: f64 = if ((p.p33 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard888 = assign61440_e98842;

        let (assign61450_e98848, assign61450_e98848_d_n3, assign61450_e98848_d_n4, assign61450_e98848_d_n5, assign61450_e98848_d_n6, assign61450_e98848_d_n7, assign61450_e98848_d_n8, assign61450_e98848_d_n9, assign61450_e98848_d_n10, assign61450_e98848_d_n11,) = {
    if (locals.var_guard888 != 0.0) {
        let assign61450_e98846: f64 = (1.0 / locals.var_rdrain);
        (assign61450_e98846, (-(locals.var_rdrain_dn3 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn4 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn5 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn6 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn7 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn8 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn9 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn10 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn11 / (locals.var_rdrain * locals.var_rdrain))),)
    } else {
        (locals.var_gdpr, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11,)
    }
};
        locals.var_gdpr = assign61450_e98848;
        locals.var_gdpr_dn3 = assign61450_e98848_d_n3;
        locals.var_gdpr_dn4 = assign61450_e98848_d_n4;
        locals.var_gdpr_dn5 = assign61450_e98848_d_n5;
        locals.var_gdpr_dn6 = assign61450_e98848_d_n6;
        locals.var_gdpr_dn7 = assign61450_e98848_d_n7;
        locals.var_gdpr_dn8 = assign61450_e98848_d_n8;
        locals.var_gdpr_dn9 = assign61450_e98848_d_n9;
        locals.var_gdpr_dn10 = assign61450_e98848_d_n10;
        locals.var_gdpr_dn11 = assign61450_e98848_d_n11;

        let assign61470_e98862: f64 = if ((p.p33 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard890 = assign61470_e98862;

        let (assign61480_e98868, assign61480_e98868_d_n3, assign61480_e98868_d_n4, assign61480_e98868_d_n5, assign61480_e98868_d_n6, assign61480_e98868_d_n7, assign61480_e98868_d_n8, assign61480_e98868_d_n9, assign61480_e98868_d_n10, assign61480_e98868_d_n11,) = {
    if (locals.var_guard890 != 0.0) {
        let assign61480_e98866: f64 = (1.0 / locals.var_rsource);
        (assign61480_e98866, (-(locals.var_rsource_dn3 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn4 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn5 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn6 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn7 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn8 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn9 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn10 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn11 / (locals.var_rsource * locals.var_rsource))),)
    } else {
        (locals.var_gspr, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11,)
    }
};
        locals.var_gspr = assign61480_e98868;
        locals.var_gspr_dn3 = assign61480_e98868_d_n3;
        locals.var_gspr_dn4 = assign61480_e98868_d_n4;
        locals.var_gspr_dn5 = assign61480_e98868_d_n5;
        locals.var_gspr_dn6 = assign61480_e98868_d_n6;
        locals.var_gspr_dn7 = assign61480_e98868_d_n7;
        locals.var_gspr_dn8 = assign61480_e98868_d_n8;
        locals.var_gspr_dn9 = assign61480_e98868_d_n9;
        locals.var_gspr_dn10 = assign61480_e98868_d_n10;
        locals.var_gspr_dn11 = assign61480_e98868_d_n11;

        let assign61510_e98885: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard893 = assign61510_e98885;

        let (assign61520_e98895, assign61520_e98895_d_n0, assign61520_e98895_d_n2, assign61520_e98895_d_n3, assign61520_e98895_d_n4, assign61520_e98895_d_n5, assign61520_e98895_d_n6, assign61520_e98895_d_n7, assign61520_e98895_d_n8, assign61520_e98895_d_n9, assign61520_e98895_d_n10, assign61520_e98895_d_n11,) = {
    if (locals.var_guard893 != 0.0) {
        let assign61520_e98889: f64 = (locals.var_devsign * locals.var_sigvds);
        let assign61520_e98891: f64 = (assign61520_e98889 * locals.var_ids);
        let assign61520_e98893: f64 = (assign61520_e98891 * (nv6 - nv7));
        (assign61520_e98893, 0.0, 0.0, ((assign61520_e98889 * locals.var_ids_dn3) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn4) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn5) * (nv6 - nv7)), (((assign61520_e98889 * locals.var_ids_dn6) * (nv6 - nv7)) + assign61520_e98891), (((assign61520_e98889 * locals.var_ids_dn7) * (nv6 - nv7)) + (-assign61520_e98891)), ((assign61520_e98889 * locals.var_ids_dn8) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn9) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn10) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn11) * (nv6 - nv7)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61520_e98895;
        locals.var_pdiss_dn0 = assign61520_e98895_d_n0;
        locals.var_pdiss_dn2 = assign61520_e98895_d_n2;
        locals.var_pdiss_dn3 = assign61520_e98895_d_n3;
        locals.var_pdiss_dn4 = assign61520_e98895_d_n4;
        locals.var_pdiss_dn5 = assign61520_e98895_d_n5;
        locals.var_pdiss_dn6 = assign61520_e98895_d_n6;
        locals.var_pdiss_dn7 = assign61520_e98895_d_n7;
        locals.var_pdiss_dn8 = assign61520_e98895_d_n8;
        locals.var_pdiss_dn9 = assign61520_e98895_d_n9;
        locals.var_pdiss_dn10 = assign61520_e98895_d_n10;
        locals.var_pdiss_dn11 = assign61520_e98895_d_n11;

        let assign61530_e98902: f64 = if ((p.p33 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard894 = assign61530_e98902;

        let (assign61540_e98914, assign61540_e98914_d_n0, assign61540_e98914_d_n2, assign61540_e98914_d_n3, assign61540_e98914_d_n4, assign61540_e98914_d_n5, assign61540_e98914_d_n6, assign61540_e98914_d_n7, assign61540_e98914_d_n8, assign61540_e98914_d_n9, assign61540_e98914_d_n10, assign61540_e98914_d_n11,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard894 != 0.0)) {
        let assign61540_e98909: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign61540_e98911: f64 = (assign61540_e98909 / locals.var_rdrain);
        let assign61540_e98912: f64 = (locals.var_pdiss + assign61540_e98911);
        (assign61540_e98912, (locals.var_pdiss_dn0 + (((nv0 - nv6) + (nv0 - nv6)) / locals.var_rdrain)), locals.var_pdiss_dn2, (locals.var_pdiss_dn3 + (-((assign61540_e98909 * locals.var_rdrain_dn3) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn4 + (-((assign61540_e98909 * locals.var_rdrain_dn4) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn5 + (-((assign61540_e98909 * locals.var_rdrain_dn5) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn6 + (((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_rdrain) - (assign61540_e98909 * locals.var_rdrain_dn6)) / (locals.var_rdrain * locals.var_rdrain))), (locals.var_pdiss_dn7 + (-((assign61540_e98909 * locals.var_rdrain_dn7) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn8 + (-((assign61540_e98909 * locals.var_rdrain_dn8) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn9 + (-((assign61540_e98909 * locals.var_rdrain_dn9) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn10 + (-((assign61540_e98909 * locals.var_rdrain_dn10) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn11 + (-((assign61540_e98909 * locals.var_rdrain_dn11) / (locals.var_rdrain * locals.var_rdrain)))),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61540_e98914;
        locals.var_pdiss_dn0 = assign61540_e98914_d_n0;
        locals.var_pdiss_dn2 = assign61540_e98914_d_n2;
        locals.var_pdiss_dn3 = assign61540_e98914_d_n3;
        locals.var_pdiss_dn4 = assign61540_e98914_d_n4;
        locals.var_pdiss_dn5 = assign61540_e98914_d_n5;
        locals.var_pdiss_dn6 = assign61540_e98914_d_n6;
        locals.var_pdiss_dn7 = assign61540_e98914_d_n7;
        locals.var_pdiss_dn8 = assign61540_e98914_d_n8;
        locals.var_pdiss_dn9 = assign61540_e98914_d_n9;
        locals.var_pdiss_dn10 = assign61540_e98914_d_n10;
        locals.var_pdiss_dn11 = assign61540_e98914_d_n11;

        let assign61550_e98921: f64 = if ((p.p33 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard895 = assign61550_e98921;

        let (assign61560_e98933, assign61560_e98933_d_n0, assign61560_e98933_d_n2, assign61560_e98933_d_n3, assign61560_e98933_d_n4, assign61560_e98933_d_n5, assign61560_e98933_d_n6, assign61560_e98933_d_n7, assign61560_e98933_d_n8, assign61560_e98933_d_n9, assign61560_e98933_d_n10, assign61560_e98933_d_n11,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard895 != 0.0)) {
        let assign61560_e98928: f64 = ((nv2 - nv7) * (nv2 - nv7));
        let assign61560_e98930: f64 = (assign61560_e98928 / locals.var_rsource);
        let assign61560_e98931: f64 = (locals.var_pdiss + assign61560_e98930);
        (assign61560_e98931, locals.var_pdiss_dn0, (locals.var_pdiss_dn2 + (((nv2 - nv7) + (nv2 - nv7)) / locals.var_rsource)), (locals.var_pdiss_dn3 + (-((assign61560_e98928 * locals.var_rsource_dn3) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn4 + (-((assign61560_e98928 * locals.var_rsource_dn4) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn5 + (-((assign61560_e98928 * locals.var_rsource_dn5) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn6 + (-((assign61560_e98928 * locals.var_rsource_dn6) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn7 + (((((-(nv2 - nv7)) + (-(nv2 - nv7))) * locals.var_rsource) - (assign61560_e98928 * locals.var_rsource_dn7)) / (locals.var_rsource * locals.var_rsource))), (locals.var_pdiss_dn8 + (-((assign61560_e98928 * locals.var_rsource_dn8) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn9 + (-((assign61560_e98928 * locals.var_rsource_dn9) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn10 + (-((assign61560_e98928 * locals.var_rsource_dn10) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn11 + (-((assign61560_e98928 * locals.var_rsource_dn11) / (locals.var_rsource * locals.var_rsource)))),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61560_e98933;
        locals.var_pdiss_dn0 = assign61560_e98933_d_n0;
        locals.var_pdiss_dn2 = assign61560_e98933_d_n2;
        locals.var_pdiss_dn3 = assign61560_e98933_d_n3;
        locals.var_pdiss_dn4 = assign61560_e98933_d_n4;
        locals.var_pdiss_dn5 = assign61560_e98933_d_n5;
        locals.var_pdiss_dn6 = assign61560_e98933_d_n6;
        locals.var_pdiss_dn7 = assign61560_e98933_d_n7;
        locals.var_pdiss_dn8 = assign61560_e98933_d_n8;
        locals.var_pdiss_dn9 = assign61560_e98933_d_n9;
        locals.var_pdiss_dn10 = assign61560_e98933_d_n10;
        locals.var_pdiss_dn11 = assign61560_e98933_d_n11;

        let assign61570_e98938: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard896 = assign61570_e98938;

        let assign61580_e98940: f64 = 1.0;
        locals.var_guard897 = assign61580_e98940;

        let assign61610_e98950: f64 = (p.p1359 * p.p1358);
        locals.var_rbodyext = assign61610_e98950;

        let assign61620_e98958: f64 = if ((p.p43 == 0.0) || (!true)) { 1.0 } else { 0.0 };
        locals.var_guard900 = assign61620_e98958;

        let assign61630_e98963: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard901 = assign61630_e98963;

        let assign61640_e98966: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard902 = assign61640_e98966;

        let (assign61650_e98994,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) {
        let assign61650_e98976: f64 = (p.p1357 * p.p1356);
        let assign61650_e98978: f64 = (assign61650_e98976 * p.p1360);
        let assign61650_e98981: f64 = (2.0 * p.p1356);
        let assign61650_e98984: f64 = (p.p1360 * locals.var_leff_1);
        let assign61650_e98985: f64 = (assign61650_e98981 + assign61650_e98984);
        let assign61650_e98986: f64 = (assign61650_e98978 / assign61650_e98985);
        let assign61650_e98988: f64 = (assign61650_e98986 * locals.var_weff_1);
        let assign61650_e98990: f64 = (assign61650_e98988 / p.p1373);
        let assign61650_e98992: f64 = (assign61650_e98990 / p.p2);
        (assign61650_e98992,)
    } else {
        (locals.var_rbodyint,)
    }
};
        locals.var_rbodyint = assign61650_e98994;

        let assign61660_e98997: f64 = if locals.var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard903 = assign61660_e98997;

        let assign61670_e99000: f64 = if locals.var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard904 = assign61670_e99000;

        let (assign61680_e99016, assign61680_e99016_d_n3, assign61680_e99016_d_n4, assign61680_e99016_d_n5, assign61680_e99016_d_n6, assign61680_e99016_d_n7, assign61680_e99016_d_n8, assign61680_e99016_d_n9, assign61680_e99016_d_n10, assign61680_e99016_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 != 0.0)) && (locals.var_guard904 != 0.0)) {
        let assign61680_e99014: f64 = (1.0 / 0.001);
        (assign61680_e99014, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61680_e99016;
        locals.var_t0_dn3 = assign61680_e99016_d_n3;
        locals.var_t0_dn4 = assign61680_e99016_d_n4;
        locals.var_t0_dn5 = assign61680_e99016_d_n5;
        locals.var_t0_dn6 = assign61680_e99016_d_n6;
        locals.var_t0_dn7 = assign61680_e99016_d_n7;
        locals.var_t0_dn8 = assign61680_e99016_d_n8;
        locals.var_t0_dn9 = assign61680_e99016_d_n9;
        locals.var_t0_dn10 = assign61680_e99016_d_n10;
        locals.var_t0_dn11 = assign61680_e99016_d_n11;

        let (assign61690_e99033, assign61690_e99033_d_n3, assign61690_e99033_d_n4, assign61690_e99033_d_n5, assign61690_e99033_d_n6, assign61690_e99033_d_n7, assign61690_e99033_d_n8, assign61690_e99033_d_n9, assign61690_e99033_d_n10, assign61690_e99033_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 != 0.0)) && (locals.var_guard904 == 0.0)) {
        let assign61690_e99031: f64 = (1.0 / locals.var_rbodyext);
        (assign61690_e99031, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61690_e99033;
        locals.var_t0_dn3 = assign61690_e99033_d_n3;
        locals.var_t0_dn4 = assign61690_e99033_d_n4;
        locals.var_t0_dn5 = assign61690_e99033_d_n5;
        locals.var_t0_dn6 = assign61690_e99033_d_n6;
        locals.var_t0_dn7 = assign61690_e99033_d_n7;
        locals.var_t0_dn8 = assign61690_e99033_d_n8;
        locals.var_t0_dn9 = assign61690_e99033_d_n9;
        locals.var_t0_dn10 = assign61690_e99033_d_n10;
        locals.var_t0_dn11 = assign61690_e99033_d_n11;

        let (assign61720_e99077, assign61720_e99077_d_n4, assign61720_e99077_d_n5,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61720_e99074: f64 = (locals.var_tratio).powf(locals.var_ubte_i);
        let assign61720_e99075: f64 = (locals.var_ub_i * assign61720_e99074);
        (assign61720_e99075, (locals.var_ub_i * if 0.0 == 0.0 && ((locals.var_ubte_i) as f64).is_finite() && ((locals.var_ubte_i) as f64).fract() == 0.0 { if locals.var_ubte_i == 0.0 { 0.0 } else { (locals.var_ubte_i * ((locals.var_tratio).powf(locals.var_ubte_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign61720_e99074 * (locals.var_ubte_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), (locals.var_ub_i * if 0.0 == 0.0 && ((locals.var_ubte_i) as f64).is_finite() && ((locals.var_ubte_i) as f64).fract() == 0.0 { if locals.var_ubte_i == 0.0 { 0.0 } else { (locals.var_ubte_i * ((locals.var_tratio).powf(locals.var_ubte_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign61720_e99074 * (locals.var_ubte_i * (locals.var_tratio_dn5 / locals.var_tratio))) }),)
    } else {
        (locals.var_ub_t, locals.var_ub_t_dn4, locals.var_ub_t_dn5,)
    }
};
        locals.var_ub_t = assign61720_e99077;
        locals.var_ub_t_dn4 = assign61720_e99077_d_n4;
        locals.var_ub_t_dn5 = assign61720_e99077_d_n5;

        let (assign61730_e99095, assign61730_e99095_d_n3, assign61730_e99095_d_n4, assign61730_e99095_d_n5, assign61730_e99095_d_n6, assign61730_e99095_d_n7, assign61730_e99095_d_n8, assign61730_e99095_d_n9, assign61730_e99095_d_n10, assign61730_e99095_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61730_e99088: f64 = (locals.var_qbi + locals.var_qbsj);
        let assign61730_e99090: f64 = (assign61730_e99088 + locals.var_qbdj);
        let assign61730_e99091: f64 = (-assign61730_e99090);
        let assign61730_e99093: f64 = (assign61730_e99091 + locals.var_qsub);
        (assign61730_e99093, ((-((locals.var_qbi_dn3 + locals.var_qbsj_dn3) + locals.var_qbdj_dn3)) + locals.var_qsub_dn3), ((-((locals.var_qbi_dn4 + locals.var_qbsj_dn4) + locals.var_qbdj_dn4)) + locals.var_qsub_dn4), ((-((locals.var_qbi_dn5 + locals.var_qbsj_dn5) + locals.var_qbdj_dn5)) + locals.var_qsub_dn5), ((-((locals.var_qbi_dn6 + locals.var_qbsj_dn6) + locals.var_qbdj_dn6)) + locals.var_qsub_dn6), ((-((locals.var_qbi_dn7 + locals.var_qbsj_dn7) + locals.var_qbdj_dn7)) + locals.var_qsub_dn7), ((-((locals.var_qbi_dn8 + locals.var_qbsj_dn8) + locals.var_qbdj_dn8)) + locals.var_qsub_dn8), ((-((locals.var_qbi_dn9 + locals.var_qbsj_dn9) + locals.var_qbdj_dn9)) + locals.var_qsub_dn9), ((-((locals.var_qbi_dn10 + locals.var_qbsj_dn10) + locals.var_qbdj_dn10)) + locals.var_qsub_dn10), ((-((locals.var_qbi_dn11 + locals.var_qbsj_dn11) + locals.var_qbdj_dn11)) + locals.var_qsub_dn11),)
    } else {
        (locals.var_qb1, locals.var_qb1_dn3, locals.var_qb1_dn4, locals.var_qb1_dn5, locals.var_qb1_dn6, locals.var_qb1_dn7, locals.var_qb1_dn8, locals.var_qb1_dn9, locals.var_qb1_dn10, locals.var_qb1_dn11,)
    }
};
        locals.var_qb1 = assign61730_e99095;
        locals.var_qb1_dn3 = assign61730_e99095_d_n3;
        locals.var_qb1_dn4 = assign61730_e99095_d_n4;
        locals.var_qb1_dn5 = assign61730_e99095_d_n5;
        locals.var_qb1_dn6 = assign61730_e99095_d_n6;
        locals.var_qb1_dn7 = assign61730_e99095_d_n7;
        locals.var_qb1_dn8 = assign61730_e99095_d_n8;
        locals.var_qb1_dn9 = assign61730_e99095_d_n9;
        locals.var_qb1_dn10 = assign61730_e99095_d_n10;
        locals.var_qb1_dn11 = assign61730_e99095_d_n11;

        let (assign61740_e99116, assign61740_e99116_d_n3, assign61740_e99116_d_n4, assign61740_e99116_d_n5, assign61740_e99116_d_n6, assign61740_e99116_d_n7, assign61740_e99116_d_n8, assign61740_e99116_d_n9, assign61740_e99116_d_n10, assign61740_e99116_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61740_e99106: f64 = (1.602176462e-19 * locals.var_neff_i);
        let assign61740_e99108: f64 = (assign61740_e99106 * p.p74);
        let assign61740_e99110: f64 = (assign61740_e99108 * locals.var_weff_1);
        let assign61740_e99112: f64 = (assign61740_e99110 * locals.var_leff_1);
        let assign61740_e99114: f64 = (assign61740_e99112 - locals.var_qb1);
        (assign61740_e99114, (-locals.var_qb1_dn3), (-locals.var_qb1_dn4), (-locals.var_qb1_dn5), (-locals.var_qb1_dn6), (-locals.var_qb1_dn7), (-locals.var_qb1_dn8), (-locals.var_qb1_dn9), (-locals.var_qb1_dn10), (-locals.var_qb1_dn11),)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11,)
    }
};
        locals.var_qbody = assign61740_e99116;
        locals.var_qbody_dn3 = assign61740_e99116_d_n3;
        locals.var_qbody_dn4 = assign61740_e99116_d_n4;
        locals.var_qbody_dn5 = assign61740_e99116_d_n5;
        locals.var_qbody_dn6 = assign61740_e99116_d_n6;
        locals.var_qbody_dn7 = assign61740_e99116_d_n7;
        locals.var_qbody_dn8 = assign61740_e99116_d_n8;
        locals.var_qbody_dn9 = assign61740_e99116_d_n9;
        locals.var_qbody_dn10 = assign61740_e99116_d_n10;
        locals.var_qbody_dn11 = assign61740_e99116_d_n11;

        let (assign61750_e99129, assign61750_e99129_d_n3, assign61750_e99129_d_n4, assign61750_e99129_d_n5, assign61750_e99129_d_n6, assign61750_e99129_d_n7, assign61750_e99129_d_n8, assign61750_e99129_d_n9, assign61750_e99129_d_n10, assign61750_e99129_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61750_e99127: f64 = (locals.var_ub_t * locals.var_qbody);
        (assign61750_e99127, (locals.var_ub_t * locals.var_qbody_dn3), ((locals.var_ub_t_dn4 * locals.var_qbody) + (locals.var_ub_t * locals.var_qbody_dn4)), ((locals.var_ub_t_dn5 * locals.var_qbody) + (locals.var_ub_t * locals.var_qbody_dn5)), (locals.var_ub_t * locals.var_qbody_dn6), (locals.var_ub_t * locals.var_qbody_dn7), (locals.var_ub_t * locals.var_qbody_dn8), (locals.var_ub_t * locals.var_qbody_dn9), (locals.var_ub_t * locals.var_qbody_dn10), (locals.var_ub_t * locals.var_qbody_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61750_e99129;
        locals.var_t0_dn3 = assign61750_e99129_d_n3;
        locals.var_t0_dn4 = assign61750_e99129_d_n4;
        locals.var_t0_dn5 = assign61750_e99129_d_n5;
        locals.var_t0_dn6 = assign61750_e99129_d_n6;
        locals.var_t0_dn7 = assign61750_e99129_d_n7;
        locals.var_t0_dn8 = assign61750_e99129_d_n8;
        locals.var_t0_dn9 = assign61750_e99129_d_n9;
        locals.var_t0_dn10 = assign61750_e99129_d_n10;
        locals.var_t0_dn11 = assign61750_e99129_d_n11;

        let (assign61760_e99142, assign61760_e99142_d_n3, assign61760_e99142_d_n4, assign61760_e99142_d_n5, assign61760_e99142_d_n6, assign61760_e99142_d_n7, assign61760_e99142_d_n8, assign61760_e99142_d_n9, assign61760_e99142_d_n10, assign61760_e99142_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61760_e99140: f64 = (locals.var_weff_1 * locals.var_weff_1);
        (assign61760_e99140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign61760_e99142;
        locals.var_t1_dn3 = assign61760_e99142_d_n3;
        locals.var_t1_dn4 = assign61760_e99142_d_n4;
        locals.var_t1_dn5 = assign61760_e99142_d_n5;
        locals.var_t1_dn6 = assign61760_e99142_d_n6;
        locals.var_t1_dn7 = assign61760_e99142_d_n7;
        locals.var_t1_dn8 = assign61760_e99142_d_n8;
        locals.var_t1_dn9 = assign61760_e99142_d_n9;
        locals.var_t1_dn10 = assign61760_e99142_d_n10;
        locals.var_t1_dn11 = assign61760_e99142_d_n11;

        let (assign61770_e99157,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61770_e99153: f64 = (p.p2 * locals.var_t0);
        let assign61770_e99155: f64 = (assign61770_e99153 / locals.var_t1);
        (assign61770_e99155,)
    } else {
        (locals.var_gbodyint,)
    }
};
        locals.var_gbodyint = assign61770_e99157;

        let (assign61780_e99170,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61780_e99168: f64 = (1.0 / locals.var_gbodyint);
        (assign61780_e99168,)
    } else {
        (locals.var_rbodyint,)
    }
};
        locals.var_rbodyint = assign61780_e99170;

        let assign61790_e99173: f64 = if locals.var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard905 = assign61790_e99173;

        let assign61800_e99176: f64 = if locals.var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard906 = assign61800_e99176;

        let (assign61810_e99193, assign61810_e99193_d_n3, assign61810_e99193_d_n4, assign61810_e99193_d_n5, assign61810_e99193_d_n6, assign61810_e99193_d_n7, assign61810_e99193_d_n8, assign61810_e99193_d_n9, assign61810_e99193_d_n10, assign61810_e99193_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 != 0.0)) && (locals.var_guard906 != 0.0)) {
        let assign61810_e99191: f64 = (1.0 / 0.001);
        (assign61810_e99191, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61810_e99193;
        locals.var_t0_dn3 = assign61810_e99193_d_n3;
        locals.var_t0_dn4 = assign61810_e99193_d_n4;
        locals.var_t0_dn5 = assign61810_e99193_d_n5;
        locals.var_t0_dn6 = assign61810_e99193_d_n6;
        locals.var_t0_dn7 = assign61810_e99193_d_n7;
        locals.var_t0_dn8 = assign61810_e99193_d_n8;
        locals.var_t0_dn9 = assign61810_e99193_d_n9;
        locals.var_t0_dn10 = assign61810_e99193_d_n10;
        locals.var_t0_dn11 = assign61810_e99193_d_n11;

        let (assign61820_e99211, assign61820_e99211_d_n3, assign61820_e99211_d_n4, assign61820_e99211_d_n5, assign61820_e99211_d_n6, assign61820_e99211_d_n7, assign61820_e99211_d_n8, assign61820_e99211_d_n9, assign61820_e99211_d_n10, assign61820_e99211_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 != 0.0)) && (locals.var_guard906 == 0.0)) {
        let assign61820_e99209: f64 = (1.0 / locals.var_rbodyext);
        (assign61820_e99209, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61820_e99211;
        locals.var_t0_dn3 = assign61820_e99211_d_n3;
        locals.var_t0_dn4 = assign61820_e99211_d_n4;
        locals.var_t0_dn5 = assign61820_e99211_d_n5;
        locals.var_t0_dn6 = assign61820_e99211_d_n6;
        locals.var_t0_dn7 = assign61820_e99211_d_n7;
        locals.var_t0_dn8 = assign61820_e99211_d_n8;
        locals.var_t0_dn9 = assign61820_e99211_d_n9;
        locals.var_t0_dn10 = assign61820_e99211_d_n10;
        locals.var_t0_dn11 = assign61820_e99211_d_n11;

        let assign61870_e99254: f64 = if p.p1374 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard908 = assign61870_e99254;

        let (assign61880_e99260, assign61880_e99260_d_n3, assign61880_e99260_d_n4, assign61880_e99260_d_n5, assign61880_e99260_d_n6, assign61880_e99260_d_n7, assign61880_e99260_d_n8, assign61880_e99260_d_n9, assign61880_e99260_d_n10, assign61880_e99260_d_n11,) = {
    if (locals.var_guard908 != 0.0) {
        let assign61880_e99258: f64 = (1.0 / 0.001);
        (assign61880_e99258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61880_e99260;
        locals.var_t0_dn3 = assign61880_e99260_d_n3;
        locals.var_t0_dn4 = assign61880_e99260_d_n4;
        locals.var_t0_dn5 = assign61880_e99260_d_n5;
        locals.var_t0_dn6 = assign61880_e99260_d_n6;
        locals.var_t0_dn7 = assign61880_e99260_d_n7;
        locals.var_t0_dn8 = assign61880_e99260_d_n8;
        locals.var_t0_dn9 = assign61880_e99260_d_n9;
        locals.var_t0_dn10 = assign61880_e99260_d_n10;
        locals.var_t0_dn11 = assign61880_e99260_d_n11;

    }

    pub(super) fn stamp_reactive_block_0(
        locals: &mut StampLocals,
    ) {
        locals.var_sp_vfb = 0.0;
        locals.var_sp_vfb_dn3 = 0.0;
        locals.var_sp_vfb_dn4 = 0.0;
        locals.var_sp_vfb_dn5 = 0.0;
        locals.var_sp_vfb_dn6 = 0.0;
        locals.var_sp_vfb_dn7 = 0.0;
        locals.var_sp_vfb_dn8 = 0.0;
        locals.var_sp_vfb_dn9 = 0.0;
        locals.var_sp_vfb_dn10 = 0.0;
        locals.var_sp_vfb_dn11 = 0.0;
        locals.var_sp_vfb_rv = 0.0;

        locals.var_sp_s_xbar = 0.0;
        locals.var_sp_s_xbar_dn3 = 0.0;
        locals.var_sp_s_xbar_dn4 = 0.0;
        locals.var_sp_s_xbar_dn5 = 0.0;
        locals.var_sp_s_xbar_dn6 = 0.0;
        locals.var_sp_s_xbar_dn7 = 0.0;
        locals.var_sp_s_xbar_dn8 = 0.0;
        locals.var_sp_s_xbar_dn9 = 0.0;
        locals.var_sp_s_xbar_dn10 = 0.0;
        locals.var_sp_s_xbar_dn11 = 0.0;
        locals.var_sp_s_xbar_rv = 0.0;

        locals.var_psip = 0.0;
        locals.var_psip_dn3 = 0.0;
        locals.var_psip_dn4 = 0.0;
        locals.var_psip_dn5 = 0.0;
        locals.var_psip_dn6 = 0.0;
        locals.var_psip_dn7 = 0.0;
        locals.var_psip_dn8 = 0.0;
        locals.var_psip_dn9 = 0.0;
        locals.var_psip_dn10 = 0.0;
        locals.var_psip_dn11 = 0.0;
        locals.var_psip_rv = 0.0;

        locals.var_inv_gam = 0.0;
        locals.var_inv_gam_dn3 = 0.0;
        locals.var_inv_gam_dn4 = 0.0;
        locals.var_inv_gam_dn5 = 0.0;
        locals.var_inv_gam_dn6 = 0.0;
        locals.var_inv_gam_dn7 = 0.0;
        locals.var_inv_gam_dn8 = 0.0;
        locals.var_inv_gam_dn9 = 0.0;
        locals.var_inv_gam_dn10 = 0.0;
        locals.var_inv_gam_dn11 = 0.0;
        locals.var_inv_gam_rv = 0.0;

        locals.var_vdssat = 0.0;
        locals.var_vdssat_dn3 = 0.0;
        locals.var_vdssat_dn4 = 0.0;
        locals.var_vdssat_dn5 = 0.0;
        locals.var_vdssat_dn6 = 0.0;
        locals.var_vdssat_dn7 = 0.0;
        locals.var_vdssat_dn8 = 0.0;
        locals.var_vdssat_dn9 = 0.0;
        locals.var_vdssat_dn10 = 0.0;
        locals.var_vdssat_dn11 = 0.0;
        locals.var_vdssat_rv = 0.0;

        locals.var_vfb2 = 0.0;
        locals.var_vfb2_dn3 = 0.0;
        locals.var_vfb2_dn4 = 0.0;
        locals.var_vfb2_dn5 = 0.0;
        locals.var_vfb2_dn6 = 0.0;
        locals.var_vfb2_dn7 = 0.0;
        locals.var_vfb2_dn8 = 0.0;
        locals.var_vfb2_dn9 = 0.0;
        locals.var_vfb2_dn10 = 0.0;
        locals.var_vfb2_dn11 = 0.0;
        locals.var_vfb2_rv = 0.0;

        locals.var_cdscdedger_i = 0.0;
        locals.var_cdscdedger_i_dn3 = 0.0;
        locals.var_cdscdedger_i_dn4 = 0.0;
        locals.var_cdscdedger_i_dn5 = 0.0;
        locals.var_cdscdedger_i_dn6 = 0.0;
        locals.var_cdscdedger_i_dn7 = 0.0;
        locals.var_cdscdedger_i_dn8 = 0.0;
        locals.var_cdscdedger_i_dn9 = 0.0;
        locals.var_cdscdedger_i_dn10 = 0.0;
        locals.var_cdscdedger_i_dn11 = 0.0;
        locals.var_cdscdedger_i_rv = 0.0;

        locals.var_cdscdr_i = 0.0;
        locals.var_cdscdr_i_dn3 = 0.0;
        locals.var_cdscdr_i_dn4 = 0.0;
        locals.var_cdscdr_i_dn5 = 0.0;
        locals.var_cdscdr_i_dn6 = 0.0;
        locals.var_cdscdr_i_dn7 = 0.0;
        locals.var_cdscdr_i_dn8 = 0.0;
        locals.var_cdscdr_i_dn9 = 0.0;
        locals.var_cdscdr_i_dn10 = 0.0;
        locals.var_cdscdr_i_dn11 = 0.0;
        locals.var_cdscdr_i_rv = 0.0;

        locals.var_l_wln1 = 0.0;
        locals.var_l_wln1_rv = 0.0;

        locals.var_ptwgr_i = 0.0;
        locals.var_ptwgr_i_dn3 = 0.0;
        locals.var_ptwgr_i_dn4 = 0.0;
        locals.var_ptwgr_i_dn5 = 0.0;
        locals.var_ptwgr_i_dn6 = 0.0;
        locals.var_ptwgr_i_dn7 = 0.0;
        locals.var_ptwgr_i_dn8 = 0.0;
        locals.var_ptwgr_i_dn9 = 0.0;
        locals.var_ptwgr_i_dn10 = 0.0;
        locals.var_ptwgr_i_dn11 = 0.0;
        locals.var_ptwgr_i_rv = 0.0;

        locals.var_uar_i = 0.0;
        locals.var_uar_i_dn3 = 0.0;
        locals.var_uar_i_dn4 = 0.0;
        locals.var_uar_i_dn5 = 0.0;
        locals.var_uar_i_dn6 = 0.0;
        locals.var_uar_i_dn7 = 0.0;
        locals.var_uar_i_dn8 = 0.0;
        locals.var_uar_i_dn9 = 0.0;
        locals.var_uar_i_dn10 = 0.0;
        locals.var_uar_i_dn11 = 0.0;
        locals.var_uar_i_rv = 0.0;

        locals.var_ucsr_i = 0.0;
        locals.var_ucsr_i_rv = 0.0;

        locals.var_ud_a = 0.0;
        locals.var_ud_a_dn3 = 0.0;
        locals.var_ud_a_dn4 = 0.0;
        locals.var_ud_a_dn5 = 0.0;
        locals.var_ud_a_dn6 = 0.0;
        locals.var_ud_a_dn7 = 0.0;
        locals.var_ud_a_dn8 = 0.0;
        locals.var_ud_a_dn9 = 0.0;
        locals.var_ud_a_dn10 = 0.0;
        locals.var_ud_a_dn11 = 0.0;
        locals.var_ud_a_rv = 0.0;

        locals.var_w_wwn1 = 0.0;
        locals.var_w_wwn1_rv = 0.0;

        locals.var_inv_sa = 0.0;
        locals.var_inv_sa_dn3 = 0.0;
        locals.var_inv_sa_dn4 = 0.0;
        locals.var_inv_sa_dn5 = 0.0;
        locals.var_inv_sa_dn6 = 0.0;
        locals.var_inv_sa_dn7 = 0.0;
        locals.var_inv_sa_dn8 = 0.0;
        locals.var_inv_sa_dn9 = 0.0;
        locals.var_inv_sa_dn10 = 0.0;
        locals.var_inv_sa_dn11 = 0.0;
        locals.var_inv_sa_rv = 0.0;

        locals.var_eta_stress = 0.0;
        locals.var_eta_stress_dn3 = 0.0;
        locals.var_eta_stress_dn4 = 0.0;
        locals.var_eta_stress_dn5 = 0.0;
        locals.var_eta_stress_dn6 = 0.0;
        locals.var_eta_stress_dn7 = 0.0;
        locals.var_eta_stress_dn8 = 0.0;
        locals.var_eta_stress_dn9 = 0.0;
        locals.var_eta_stress_dn10 = 0.0;
        locals.var_eta_stress_dn11 = 0.0;
        locals.var_eta_stress_rv = 0.0;

        locals.var_m0_i = 0.0;
        locals.var_m0_i_rv = 0.0;

        locals.var_m0_t = 0.0;
        locals.var_m0_t_dn4 = 0.0;
        locals.var_m0_t_dn5 = 0.0;
        locals.var_m0_t_rv = 0.0;

        locals.var_eta0edge_i = 0.0;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_rv = 0.0;

        locals.var_kt2edge_i = 0.0;
        locals.var_kt2edge_i_rv = 0.0;

        locals.var_k2edge_i = 0.0;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_rv = 0.0;

        locals.var_mnud1 = 0.0;
        locals.var_mnud1_dn3 = 0.0;
        locals.var_mnud1_dn4 = 0.0;
        locals.var_mnud1_dn5 = 0.0;
        locals.var_mnud1_dn6 = 0.0;
        locals.var_mnud1_dn7 = 0.0;
        locals.var_mnud1_dn8 = 0.0;
        locals.var_mnud1_dn9 = 0.0;
        locals.var_mnud1_dn10 = 0.0;
        locals.var_mnud1_dn11 = 0.0;
        locals.var_mnud1_rv = 0.0;

        locals.var_c0si_i = 0.0;
        locals.var_c0si_i_rv = 0.0;

        locals.var_c0sisat1_i = 0.0;
        locals.var_c0sisat1_i_rv = 0.0;

        locals.var_eta0r_i = 0.0;
        locals.var_eta0r_i_dn3 = 0.0;
        locals.var_eta0r_i_dn4 = 0.0;
        locals.var_eta0r_i_dn5 = 0.0;
        locals.var_eta0r_i_dn6 = 0.0;
        locals.var_eta0r_i_dn7 = 0.0;
        locals.var_eta0r_i_dn8 = 0.0;
        locals.var_eta0r_i_dn9 = 0.0;
        locals.var_eta0r_i_dn10 = 0.0;
        locals.var_eta0r_i_dn11 = 0.0;
        locals.var_eta0r_i_rv = 0.0;

        locals.var_pclmr_i = 0.0;
        locals.var_pclmr_i_dn3 = 0.0;
        locals.var_pclmr_i_dn4 = 0.0;
        locals.var_pclmr_i_dn5 = 0.0;
        locals.var_pclmr_i_dn6 = 0.0;
        locals.var_pclmr_i_dn7 = 0.0;
        locals.var_pclmr_i_dn8 = 0.0;
        locals.var_pclmr_i_dn9 = 0.0;
        locals.var_pclmr_i_dn10 = 0.0;
        locals.var_pclmr_i_dn11 = 0.0;
        locals.var_pclmr_i_rv = 0.0;

        locals.var_ptwgr_t = 0.0;
        locals.var_ptwgr_t_dn3 = 0.0;
        locals.var_ptwgr_t_dn4 = 0.0;
        locals.var_ptwgr_t_dn5 = 0.0;
        locals.var_ptwgr_t_dn6 = 0.0;
        locals.var_ptwgr_t_dn7 = 0.0;
        locals.var_ptwgr_t_dn8 = 0.0;
        locals.var_ptwgr_t_dn9 = 0.0;
        locals.var_ptwgr_t_dn10 = 0.0;
        locals.var_ptwgr_t_dn11 = 0.0;
        locals.var_ptwgr_t_rv = 0.0;

        locals.var_uar_t = 0.0;
        locals.var_uar_t_dn3 = 0.0;
        locals.var_uar_t_dn4 = 0.0;
        locals.var_uar_t_dn5 = 0.0;
        locals.var_uar_t_dn6 = 0.0;
        locals.var_uar_t_dn7 = 0.0;
        locals.var_uar_t_dn8 = 0.0;
        locals.var_uar_t_dn9 = 0.0;
        locals.var_uar_t_dn10 = 0.0;
        locals.var_uar_t_dn11 = 0.0;
        locals.var_uar_t_rv = 0.0;

        locals.var_ucsr_t = 0.0;
        locals.var_ucsr_t_dn4 = 0.0;
        locals.var_ucsr_t_dn5 = 0.0;
        locals.var_ucsr_t_rv = 0.0;

        locals.var_vsatr_i = 0.0;
        locals.var_vsatr_i_dn3 = 0.0;
        locals.var_vsatr_i_dn4 = 0.0;
        locals.var_vsatr_i_dn5 = 0.0;
        locals.var_vsatr_i_dn6 = 0.0;
        locals.var_vsatr_i_dn7 = 0.0;
        locals.var_vsatr_i_dn8 = 0.0;
        locals.var_vsatr_i_dn9 = 0.0;
        locals.var_vsatr_i_dn10 = 0.0;
        locals.var_vsatr_i_dn11 = 0.0;
        locals.var_vsatr_i_rv = 0.0;

        locals.var_local_sca = 0.0;
        locals.var_local_sca_dn3 = 0.0;
        locals.var_local_sca_dn4 = 0.0;
        locals.var_local_sca_dn5 = 0.0;
        locals.var_local_sca_dn6 = 0.0;
        locals.var_local_sca_dn7 = 0.0;
        locals.var_local_sca_dn8 = 0.0;
        locals.var_local_sca_dn9 = 0.0;
        locals.var_local_sca_dn10 = 0.0;
        locals.var_local_sca_dn11 = 0.0;
        locals.var_local_sca_rv = 0.0;

        locals.var_inv_sb = 0.0;
        locals.var_inv_sb_dn3 = 0.0;
        locals.var_inv_sb_dn4 = 0.0;
        locals.var_inv_sb_dn5 = 0.0;
        locals.var_inv_sb_dn6 = 0.0;
        locals.var_inv_sb_dn7 = 0.0;
        locals.var_inv_sb_dn8 = 0.0;
        locals.var_inv_sb_dn9 = 0.0;
        locals.var_inv_sb_dn10 = 0.0;
        locals.var_inv_sb_dn11 = 0.0;
        locals.var_inv_sb_rv = 0.0;

        locals.var_k01_i = 0.0;
        locals.var_k01_i_rv = 0.0;

        locals.var_citedge_i = 0.0;
        locals.var_citedge_i_rv = 0.0;

        locals.var_etabedge_i = 0.0;
        locals.var_etabedge_i_rv = 0.0;

        locals.var_kt1expedge_i = 0.0;
        locals.var_kt1expedge_i_rv = 0.0;

        locals.var_kvth0edge_i = 0.0;
        locals.var_kvth0edge_i_rv = 0.0;

        locals.var_c0_i = 0.0;
        locals.var_c0_i_rv = 0.0;

        locals.var_c0si1_i = 0.0;
        locals.var_c0si1_i_rv = 0.0;

        locals.var_c0sisat_t = 0.0;
        locals.var_c0sisat_t_dn4 = 0.0;
        locals.var_c0sisat_t_dn5 = 0.0;
        locals.var_c0sisat_t_rv = 0.0;

        locals.var_eta0r_t = 0.0;
        locals.var_eta0r_t_dn3 = 0.0;
        locals.var_eta0r_t_dn4 = 0.0;
        locals.var_eta0r_t_dn5 = 0.0;
        locals.var_eta0r_t_dn6 = 0.0;
        locals.var_eta0r_t_dn7 = 0.0;
        locals.var_eta0r_t_dn8 = 0.0;
        locals.var_eta0r_t_dn9 = 0.0;
        locals.var_eta0r_t_dn10 = 0.0;
        locals.var_eta0r_t_dn11 = 0.0;
        locals.var_eta0r_t_rv = 0.0;

        locals.var_pdiblcr_i = 0.0;
        locals.var_pdiblcr_i_dn3 = 0.0;
        locals.var_pdiblcr_i_dn4 = 0.0;
        locals.var_pdiblcr_i_dn5 = 0.0;
        locals.var_pdiblcr_i_dn6 = 0.0;
        locals.var_pdiblcr_i_dn7 = 0.0;
        locals.var_pdiblcr_i_dn8 = 0.0;
        locals.var_pdiblcr_i_dn9 = 0.0;
        locals.var_pdiblcr_i_dn10 = 0.0;
        locals.var_pdiblcr_i_dn11 = 0.0;
        locals.var_pdiblcr_i_rv = 0.0;

        locals.var_u0r_i = 0.0;
        locals.var_u0r_i_rv = 0.0;

        locals.var_ucr_i = 0.0;
        locals.var_ucr_i_dn3 = 0.0;
        locals.var_ucr_i_dn4 = 0.0;
        locals.var_ucr_i_dn5 = 0.0;
        locals.var_ucr_i_dn6 = 0.0;
        locals.var_ucr_i_dn7 = 0.0;
        locals.var_ucr_i_dn8 = 0.0;
        locals.var_ucr_i_dn9 = 0.0;
        locals.var_ucr_i_dn10 = 0.0;
        locals.var_ucr_i_dn11 = 0.0;
        locals.var_ucr_i_rv = 0.0;

        locals.var_udr_i = 0.0;
        locals.var_udr_i_dn3 = 0.0;
        locals.var_udr_i_dn4 = 0.0;
        locals.var_udr_i_dn5 = 0.0;
        locals.var_udr_i_dn6 = 0.0;
        locals.var_udr_i_dn7 = 0.0;
        locals.var_udr_i_dn8 = 0.0;
        locals.var_udr_i_dn9 = 0.0;
        locals.var_udr_i_dn10 = 0.0;
        locals.var_udr_i_dn11 = 0.0;
        locals.var_udr_i_rv = 0.0;

        locals.var_vsatr_t = 0.0;
        locals.var_vsatr_t_dn3 = 0.0;
        locals.var_vsatr_t_dn4 = 0.0;
        locals.var_vsatr_t_dn5 = 0.0;
        locals.var_vsatr_t_dn6 = 0.0;
        locals.var_vsatr_t_dn7 = 0.0;
        locals.var_vsatr_t_dn8 = 0.0;
        locals.var_vsatr_t_dn9 = 0.0;
        locals.var_vsatr_t_dn10 = 0.0;
        locals.var_vsatr_t_dn11 = 0.0;
        locals.var_vsatr_t_rv = 0.0;

        locals.var_local_scb = 0.0;
        locals.var_local_scb_dn3 = 0.0;
        locals.var_local_scb_dn4 = 0.0;
        locals.var_local_scb_dn5 = 0.0;
        locals.var_local_scb_dn6 = 0.0;
        locals.var_local_scb_dn7 = 0.0;
        locals.var_local_scb_dn8 = 0.0;
        locals.var_local_scb_dn9 = 0.0;
        locals.var_local_scb_dn10 = 0.0;
        locals.var_local_scb_dn11 = 0.0;
        locals.var_local_scb_rv = 0.0;

        locals.var_vth0_stress_edge = 0.0;
        locals.var_vth0_stress_edge_dn3 = 0.0;
        locals.var_vth0_stress_edge_dn4 = 0.0;
        locals.var_vth0_stress_edge_dn5 = 0.0;
        locals.var_vth0_stress_edge_dn6 = 0.0;
        locals.var_vth0_stress_edge_dn7 = 0.0;
        locals.var_vth0_stress_edge_dn8 = 0.0;
        locals.var_vth0_stress_edge_dn9 = 0.0;
        locals.var_vth0_stress_edge_dn10 = 0.0;
        locals.var_vth0_stress_edge_dn11 = 0.0;
        locals.var_vth0_stress_edge_rv = 0.0;

        locals.var_eta_stress_edge = 0.0;
        locals.var_eta_stress_edge_dn3 = 0.0;
        locals.var_eta_stress_edge_dn4 = 0.0;
        locals.var_eta_stress_edge_dn5 = 0.0;
        locals.var_eta_stress_edge_dn6 = 0.0;
        locals.var_eta_stress_edge_dn7 = 0.0;
        locals.var_eta_stress_edge_dn8 = 0.0;
        locals.var_eta_stress_edge_dn9 = 0.0;
        locals.var_eta_stress_edge_dn10 = 0.0;
        locals.var_eta_stress_edge_dn11 = 0.0;
        locals.var_eta_stress_edge_rv = 0.0;

        locals.var_m01_i = 0.0;
        locals.var_m01_i_rv = 0.0;

        locals.var_cdscdedge_i = 0.0;
        locals.var_cdscdedge_i_rv = 0.0;

        locals.var_kt1edge_i = 0.0;
        locals.var_kt1edge_i_rv = 0.0;

        locals.var_tnfactoredge_i = 0.0;
        locals.var_tnfactoredge_i_rv = 0.0;

        locals.var_stk2edge_i = 0.0;
        locals.var_stk2edge_i_rv = 0.0;

        locals.var_c01_i = 0.0;
        locals.var_c01_i_rv = 0.0;

        locals.var_c0si_t = 0.0;
        locals.var_c0si_t_dn4 = 0.0;
        locals.var_c0si_t_dn5 = 0.0;
        locals.var_c0si_t_rv = 0.0;

        locals.var_l_lln1 = 0.0;
        locals.var_l_lln1_rv = 0.0;

        locals.var_psatr_i = 0.0;
        locals.var_psatr_i_rv = 0.0;

        locals.var_u0r_t = 0.0;
        locals.var_u0r_t_dn4 = 0.0;
        locals.var_u0r_t_dn5 = 0.0;
        locals.var_u0r_t_rv = 0.0;

        locals.var_ucr_t = 0.0;
        locals.var_ucr_t_dn3 = 0.0;
        locals.var_ucr_t_dn4 = 0.0;
        locals.var_ucr_t_dn5 = 0.0;
        locals.var_ucr_t_dn6 = 0.0;
        locals.var_ucr_t_dn7 = 0.0;
        locals.var_ucr_t_dn8 = 0.0;
        locals.var_ucr_t_dn9 = 0.0;
        locals.var_ucr_t_dn10 = 0.0;
        locals.var_ucr_t_dn11 = 0.0;
        locals.var_ucr_t_rv = 0.0;

        locals.var_udr_t = 0.0;
        locals.var_udr_t_dn3 = 0.0;
        locals.var_udr_t_dn4 = 0.0;
        locals.var_udr_t_dn5 = 0.0;
        locals.var_udr_t_dn6 = 0.0;
        locals.var_udr_t_dn7 = 0.0;
        locals.var_udr_t_dn8 = 0.0;
        locals.var_udr_t_dn9 = 0.0;
        locals.var_udr_t_dn10 = 0.0;
        locals.var_udr_t_dn11 = 0.0;
        locals.var_udr_t_rv = 0.0;

        locals.var_w_lwn1 = 0.0;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_local_scc = 0.0;
        locals.var_local_scc_dn3 = 0.0;
        locals.var_local_scc_dn4 = 0.0;
        locals.var_local_scc_dn5 = 0.0;
        locals.var_local_scc_dn6 = 0.0;
        locals.var_local_scc_dn7 = 0.0;
        locals.var_local_scc_dn8 = 0.0;
        locals.var_local_scc_dn9 = 0.0;
        locals.var_local_scc_dn10 = 0.0;
        locals.var_local_scc_dn11 = 0.0;
        locals.var_local_scc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_k2_stress_edge = 0.0;
        locals.var_k2_stress_edge_dn3 = 0.0;
        locals.var_k2_stress_edge_dn4 = 0.0;
        locals.var_k2_stress_edge_dn5 = 0.0;
        locals.var_k2_stress_edge_dn6 = 0.0;
        locals.var_k2_stress_edge_dn7 = 0.0;
        locals.var_k2_stress_edge_dn8 = 0.0;
        locals.var_k2_stress_edge_dn9 = 0.0;
        locals.var_k2_stress_edge_dn10 = 0.0;
        locals.var_k2_stress_edge_dn11 = 0.0;
        locals.var_k2_stress_edge_rv = 0.0;

        locals.var_k0_i = 0.0;
        locals.var_k0_i_rv = 0.0;

        locals.var_k0_t = 0.0;
        locals.var_k0_t_dn4 = 0.0;
        locals.var_k0_t_dn5 = 0.0;
        locals.var_k0_t_rv = 0.0;

        locals.var_cdscbedge_i = 0.0;
        locals.var_cdscbedge_i_rv = 0.0;

        locals.var_kt1ledge_i = 0.0;
        locals.var_kt1ledge_i_rv = 0.0;

        locals.var_teta0edge_i = 0.0;
        locals.var_teta0edge_i_rv = 0.0;

        locals.var_steta0edge_i = 0.0;
        locals.var_steta0edge_i_rv = 0.0;

        locals.var_c0_t = 0.0;
        locals.var_c0_t_dn4 = 0.0;
        locals.var_c0_t_dn5 = 0.0;
        locals.var_c0_t_rv = 0.0;

        locals.var_c0sisat_i = 0.0;
        locals.var_c0sisat_i_rv = 0.0;

        locals.var_k2edgewe_i = 0.0;
        locals.var_k2edgewe_i_rv = 0.0;

        locals.var_kvth0edgewe_i = 0.0;
        locals.var_kvth0edgewe_i_rv = 0.0;

        locals.var_temp_adeff = 0.0;
        locals.var_temp_adeff_dn3 = 0.0;
        locals.var_temp_adeff_dn4 = 0.0;
        locals.var_temp_adeff_dn5 = 0.0;
        locals.var_temp_adeff_dn6 = 0.0;
        locals.var_temp_adeff_dn7 = 0.0;
        locals.var_temp_adeff_dn8 = 0.0;
        locals.var_temp_adeff_dn9 = 0.0;
        locals.var_temp_adeff_dn10 = 0.0;
        locals.var_temp_adeff_dn11 = 0.0;
        locals.var_temp_adeff_rv = 0.0;

        locals.var_temp_aseff = 0.0;
        locals.var_temp_aseff_dn3 = 0.0;
        locals.var_temp_aseff_dn4 = 0.0;
        locals.var_temp_aseff_dn5 = 0.0;
        locals.var_temp_aseff_dn6 = 0.0;
        locals.var_temp_aseff_dn7 = 0.0;
        locals.var_temp_aseff_dn8 = 0.0;
        locals.var_temp_aseff_dn9 = 0.0;
        locals.var_temp_aseff_dn10 = 0.0;
        locals.var_temp_aseff_dn11 = 0.0;
        locals.var_temp_aseff_rv = 0.0;

        locals.var_temp_pdeff = 0.0;
        locals.var_temp_pdeff_dn3 = 0.0;
        locals.var_temp_pdeff_dn4 = 0.0;
        locals.var_temp_pdeff_dn5 = 0.0;
        locals.var_temp_pdeff_dn6 = 0.0;
        locals.var_temp_pdeff_dn7 = 0.0;
        locals.var_temp_pdeff_dn8 = 0.0;
        locals.var_temp_pdeff_dn9 = 0.0;
        locals.var_temp_pdeff_dn10 = 0.0;
        locals.var_temp_pdeff_dn11 = 0.0;
        locals.var_temp_pdeff_rv = 0.0;

        locals.var_temp_pseff = 0.0;
        locals.var_temp_pseff_dn3 = 0.0;
        locals.var_temp_pseff_dn4 = 0.0;
        locals.var_temp_pseff_dn5 = 0.0;
        locals.var_temp_pseff_dn6 = 0.0;
        locals.var_temp_pseff_dn7 = 0.0;
        locals.var_temp_pseff_dn8 = 0.0;
        locals.var_temp_pseff_dn9 = 0.0;
        locals.var_temp_pseff_dn10 = 0.0;
        locals.var_temp_pseff_dn11 = 0.0;
        locals.var_temp_pseff_rv = 0.0;

        locals.var_abulkiv = 1.0;
        locals.var_abulkiv_dn3 = 0.0;
        locals.var_abulkiv_dn4 = 0.0;
        locals.var_abulkiv_dn5 = 0.0;
        locals.var_abulkiv_dn6 = 0.0;
        locals.var_abulkiv_dn7 = 0.0;
        locals.var_abulkiv_dn8 = 0.0;
        locals.var_abulkiv_dn9 = 0.0;
        locals.var_abulkiv_dn10 = 0.0;
        locals.var_abulkiv_dn11 = 0.0;
        locals.var_abulkiv_rv = 0.0;

        locals.var_abulkcv = 1.0;
        locals.var_abulkcv_dn3 = 0.0;
        locals.var_abulkcv_dn4 = 0.0;
        locals.var_abulkcv_dn5 = 0.0;
        locals.var_abulkcv_dn6 = 0.0;
        locals.var_abulkcv_dn7 = 0.0;
        locals.var_abulkcv_dn8 = 0.0;
        locals.var_abulkcv_dn9 = 0.0;
        locals.var_abulkcv_dn10 = 0.0;
        locals.var_abulkcv_dn11 = 0.0;
        locals.var_abulkcv_rv = 0.0;

        locals.var_eta_p = 1.0;
        locals.var_eta_p_dn3 = 0.0;
        locals.var_eta_p_dn4 = 0.0;
        locals.var_eta_p_dn5 = 0.0;
        locals.var_eta_p_dn6 = 0.0;
        locals.var_eta_p_dn7 = 0.0;
        locals.var_eta_p_dn8 = 0.0;
        locals.var_eta_p_dn9 = 0.0;
        locals.var_eta_p_dn10 = 0.0;
        locals.var_eta_p_dn11 = 0.0;
        locals.var_eta_p_rv = 0.0;

        locals.var_ddl = 0.0;
        locals.var_ddl_dn3 = 0.0;
        locals.var_ddl_dn4 = 0.0;
        locals.var_ddl_dn5 = 0.0;
        locals.var_ddl_dn6 = 0.0;
        locals.var_ddl_dn7 = 0.0;
        locals.var_ddl_dn8 = 0.0;
        locals.var_ddl_dn9 = 0.0;
        locals.var_ddl_dn10 = 0.0;
        locals.var_ddl_dn11 = 0.0;
        locals.var_ddl_rv = 0.0;

        locals.var_dmob = 0.0;
        locals.var_dmob_dn3 = 0.0;
        locals.var_dmob_dn4 = 0.0;
        locals.var_dmob_dn5 = 0.0;
        locals.var_dmob_dn6 = 0.0;
        locals.var_dmob_dn7 = 0.0;
        locals.var_dmob_dn8 = 0.0;
        locals.var_dmob_dn9 = 0.0;
        locals.var_dmob_dn10 = 0.0;
        locals.var_dmob_dn11 = 0.0;
        locals.var_dmob_rv = 0.0;

        locals.var_dr = 0.0;
        locals.var_dr_dn3 = 0.0;
        locals.var_dr_dn4 = 0.0;
        locals.var_dr_dn5 = 0.0;
        locals.var_dr_dn6 = 0.0;
        locals.var_dr_dn7 = 0.0;
        locals.var_dr_dn8 = 0.0;
        locals.var_dr_dn9 = 0.0;
        locals.var_dr_dn10 = 0.0;
        locals.var_dr_dn11 = 0.0;
        locals.var_dr_rv = 0.0;

        locals.var_dvsat = 0.0;
        locals.var_dvsat_dn3 = 0.0;
        locals.var_dvsat_dn4 = 0.0;
        locals.var_dvsat_dn5 = 0.0;
        locals.var_dvsat_dn6 = 0.0;
        locals.var_dvsat_dn7 = 0.0;
        locals.var_dvsat_dn8 = 0.0;
        locals.var_dvsat_dn9 = 0.0;
        locals.var_dvsat_dn10 = 0.0;
        locals.var_dvsat_dn11 = 0.0;
        locals.var_dvsat_rv = 0.0;

        locals.var_dvsatinv = 0.0;
        locals.var_dvsatinv_dn3 = 0.0;
        locals.var_dvsatinv_dn4 = 0.0;
        locals.var_dvsatinv_dn5 = 0.0;
        locals.var_dvsatinv_dn6 = 0.0;
        locals.var_dvsatinv_dn7 = 0.0;
        locals.var_dvsatinv_dn8 = 0.0;
        locals.var_dvsatinv_dn9 = 0.0;
        locals.var_dvsatinv_dn10 = 0.0;
        locals.var_dvsatinv_dn11 = 0.0;
        locals.var_dvsatinv_rv = 0.0;

        locals.var_ibddif = 0.0;
        locals.var_ibddif_dn3 = 0.0;
        locals.var_ibddif_dn4 = 0.0;
        locals.var_ibddif_dn5 = 0.0;
        locals.var_ibddif_dn6 = 0.0;
        locals.var_ibddif_dn7 = 0.0;
        locals.var_ibddif_dn8 = 0.0;
        locals.var_ibddif_dn9 = 0.0;
        locals.var_ibddif_dn10 = 0.0;
        locals.var_ibddif_dn11 = 0.0;
        locals.var_ibddif_rv = 0.0;

        locals.var_ibsdif = 0.0;
        locals.var_ibsdif_dn3 = 0.0;
        locals.var_ibsdif_dn4 = 0.0;
        locals.var_ibsdif_dn5 = 0.0;
        locals.var_ibsdif_dn6 = 0.0;
        locals.var_ibsdif_dn7 = 0.0;
        locals.var_ibsdif_dn8 = 0.0;
        locals.var_ibsdif_dn9 = 0.0;
        locals.var_ibsdif_dn10 = 0.0;
        locals.var_ibsdif_dn11 = 0.0;
        locals.var_ibsdif_rv = 0.0;

        locals.var_mnud = 0.0;
        locals.var_mnud_dn3 = 0.0;
        locals.var_mnud_dn4 = 0.0;
        locals.var_mnud_dn5 = 0.0;
        locals.var_mnud_dn6 = 0.0;
        locals.var_mnud_dn7 = 0.0;
        locals.var_mnud_dn8 = 0.0;
        locals.var_mnud_dn9 = 0.0;
        locals.var_mnud_dn10 = 0.0;
        locals.var_mnud_dn11 = 0.0;
        locals.var_mnud_rv = 0.0;

        locals.var_moc = 0.0;
        locals.var_moc_dn3 = 0.0;
        locals.var_moc_dn4 = 0.0;
        locals.var_moc_dn5 = 0.0;
        locals.var_moc_dn6 = 0.0;
        locals.var_moc_dn7 = 0.0;
        locals.var_moc_dn8 = 0.0;
        locals.var_moc_dn9 = 0.0;
        locals.var_moc_dn10 = 0.0;
        locals.var_moc_dn11 = 0.0;
        locals.var_moc_rv = 0.0;

        locals.var_mscbe = 0.0;
        locals.var_mscbe_dn3 = 0.0;
        locals.var_mscbe_dn4 = 0.0;
        locals.var_mscbe_dn5 = 0.0;
        locals.var_mscbe_dn6 = 0.0;
        locals.var_mscbe_dn7 = 0.0;
        locals.var_mscbe_dn8 = 0.0;
        locals.var_mscbe_dn9 = 0.0;
        locals.var_mscbe_dn10 = 0.0;
        locals.var_mscbe_dn11 = 0.0;
        locals.var_mscbe_rv = 0.0;

        locals.var_nsat = 0.0;
        locals.var_nsat_dn3 = 0.0;
        locals.var_nsat_dn4 = 0.0;
        locals.var_nsat_dn5 = 0.0;
        locals.var_nsat_dn6 = 0.0;
        locals.var_nsat_dn7 = 0.0;
        locals.var_nsat_dn8 = 0.0;
        locals.var_nsat_dn9 = 0.0;
        locals.var_nsat_dn10 = 0.0;
        locals.var_nsat_dn11 = 0.0;
        locals.var_nsat_rv = 0.0;

        locals.var_rdrain = 0.0;
        locals.var_rdrain_dn3 = 0.0;
        locals.var_rdrain_dn4 = 0.0;
        locals.var_rdrain_dn5 = 0.0;
        locals.var_rdrain_dn6 = 0.0;
        locals.var_rdrain_dn7 = 0.0;
        locals.var_rdrain_dn8 = 0.0;
        locals.var_rdrain_dn9 = 0.0;
        locals.var_rdrain_dn10 = 0.0;
        locals.var_rdrain_dn11 = 0.0;
        locals.var_rdrain_rv = 0.0;

        locals.var_rsource = 0.0;
        locals.var_rsource_dn3 = 0.0;
        locals.var_rsource_dn4 = 0.0;
        locals.var_rsource_dn5 = 0.0;
        locals.var_rsource_dn6 = 0.0;
        locals.var_rsource_dn7 = 0.0;
        locals.var_rsource_dn8 = 0.0;
        locals.var_rsource_dn9 = 0.0;
        locals.var_rsource_dn10 = 0.0;
        locals.var_rsource_dn11 = 0.0;
        locals.var_rsource_rv = 0.0;

        locals.var_vdseff = 0.0;
        locals.var_vdseff_dn3 = 0.0;
        locals.var_vdseff_dn4 = 0.0;
        locals.var_vdseff_dn5 = 0.0;
        locals.var_vdseff_dn6 = 0.0;
        locals.var_vdseff_dn7 = 0.0;
        locals.var_vdseff_dn8 = 0.0;
        locals.var_vdseff_dn9 = 0.0;
        locals.var_vdseff_dn10 = 0.0;
        locals.var_vdseff_dn11 = 0.0;
        locals.var_vdseff_rv = 0.0;

        locals.var_diffvds = 0.0;
        locals.var_diffvds_dn3 = 0.0;
        locals.var_diffvds_dn4 = 0.0;
        locals.var_diffvds_dn5 = 0.0;
        locals.var_diffvds_dn6 = 0.0;
        locals.var_diffvds_dn7 = 0.0;
        locals.var_diffvds_dn8 = 0.0;
        locals.var_diffvds_dn9 = 0.0;
        locals.var_diffvds_dn10 = 0.0;
        locals.var_diffvds_dn11 = 0.0;
        locals.var_diffvds_rv = 0.0;

        locals.var_dps = 0.0;
        locals.var_dps_dn3 = 0.0;
        locals.var_dps_dn4 = 0.0;
        locals.var_dps_dn5 = 0.0;
        locals.var_dps_dn6 = 0.0;
        locals.var_dps_dn7 = 0.0;
        locals.var_dps_dn8 = 0.0;
        locals.var_dps_dn9 = 0.0;
        locals.var_dps_dn10 = 0.0;
        locals.var_dps_dn11 = 0.0;
        locals.var_dps_rv = 0.0;

        locals.var_qia = 0.0;
        locals.var_qia_dn3 = 0.0;
        locals.var_qia_dn4 = 0.0;
        locals.var_qia_dn5 = 0.0;
        locals.var_qia_dn6 = 0.0;
        locals.var_qia_dn7 = 0.0;
        locals.var_qia_dn8 = 0.0;
        locals.var_qia_dn9 = 0.0;
        locals.var_qia_dn10 = 0.0;
        locals.var_qia_dn11 = 0.0;
        locals.var_qia_rv = 0.0;

        locals.var_qid = 0.0;
        locals.var_qid_dn3 = 0.0;
        locals.var_qid_dn4 = 0.0;
        locals.var_qid_dn5 = 0.0;
        locals.var_qid_dn6 = 0.0;
        locals.var_qid_dn7 = 0.0;
        locals.var_qid_dn8 = 0.0;
        locals.var_qid_dn9 = 0.0;
        locals.var_qid_dn10 = 0.0;
        locals.var_qid_dn11 = 0.0;
        locals.var_qid_rv = 0.0;

        locals.var_qim1 = 0.0;
        locals.var_qim1_dn3 = 0.0;
        locals.var_qim1_dn4 = 0.0;
        locals.var_qim1_dn5 = 0.0;
        locals.var_qim1_dn6 = 0.0;
        locals.var_qim1_dn7 = 0.0;
        locals.var_qim1_dn8 = 0.0;
        locals.var_qim1_dn9 = 0.0;
        locals.var_qim1_dn10 = 0.0;
        locals.var_qim1_dn11 = 0.0;
        locals.var_qim1_rv = 0.0;

        locals.var_rdsi = 0.0;
        locals.var_rdsi_dn3 = 0.0;
        locals.var_rdsi_dn4 = 0.0;
        locals.var_rdsi_dn5 = 0.0;
        locals.var_rdsi_dn6 = 0.0;
        locals.var_rdsi_dn7 = 0.0;
        locals.var_rdsi_dn8 = 0.0;
        locals.var_rdsi_dn9 = 0.0;
        locals.var_rdsi_dn10 = 0.0;
        locals.var_rdsi_dn11 = 0.0;
        locals.var_rdsi_rv = 0.0;

        let assign1090_e2343: f64 = (1.3806503e-23 / 1.602176462e-19);
        locals.var_kboq = assign1090_e2343;
        locals.var_kboq_rv = 0.0;

        locals.var_qdeff = 0.0;
        locals.var_qdeff_dn3 = 0.0;
        locals.var_qdeff_dn4 = 0.0;
        locals.var_qdeff_dn5 = 0.0;
        locals.var_qdeff_dn6 = 0.0;
        locals.var_qdeff_dn7 = 0.0;
        locals.var_qdeff_dn8 = 0.0;
        locals.var_qdeff_dn9 = 0.0;
        locals.var_qdeff_dn10 = 0.0;
        locals.var_qdeff_dn11 = 0.0;
        locals.var_qdeff_rv = 0.0;

        locals.var_qs_1 = 0.0;
        locals.var_qs_1_dn3 = 0.0;
        locals.var_qs_1_dn4 = 0.0;
        locals.var_qs_1_dn5 = 0.0;
        locals.var_qs_1_dn6 = 0.0;
        locals.var_qs_1_dn7 = 0.0;
        locals.var_qs_1_dn8 = 0.0;
        locals.var_qs_1_dn9 = 0.0;
        locals.var_qs_1_dn10 = 0.0;
        locals.var_qs_1_dn11 = 0.0;
        locals.var_qs_1_rv = 0.0;

        locals.var_x7_d = 0.0;
        locals.var_x7_d_dn3 = 0.0;
        locals.var_x7_d_dn4 = 0.0;
        locals.var_x7_d_dn5 = 0.0;
        locals.var_x7_d_dn6 = 0.0;
        locals.var_x7_d_dn7 = 0.0;
        locals.var_x7_d_dn8 = 0.0;
        locals.var_x7_d_dn9 = 0.0;
        locals.var_x7_d_dn10 = 0.0;
        locals.var_x7_d_dn11 = 0.0;
        locals.var_x7_d_rv = 0.0;

        locals.var_x7_s = 0.0;
        locals.var_x7_s_dn3 = 0.0;
        locals.var_x7_s_dn4 = 0.0;
        locals.var_x7_s_dn5 = 0.0;
        locals.var_x7_s_dn6 = 0.0;
        locals.var_x7_s_dn7 = 0.0;
        locals.var_x7_s_dn8 = 0.0;
        locals.var_x7_s_dn9 = 0.0;
        locals.var_x7_s_dn10 = 0.0;
        locals.var_x7_s_dn11 = 0.0;
        locals.var_x7_s_rv = 0.0;

        locals.var_ln_t1_t2 = 0.0;
        locals.var_ln_t1_t2_dn3 = 0.0;
        locals.var_ln_t1_t2_dn4 = 0.0;
        locals.var_ln_t1_t2_dn5 = 0.0;
        locals.var_ln_t1_t2_dn6 = 0.0;
        locals.var_ln_t1_t2_dn7 = 0.0;
        locals.var_ln_t1_t2_dn8 = 0.0;
        locals.var_ln_t1_t2_dn9 = 0.0;
        locals.var_ln_t1_t2_dn10 = 0.0;
        locals.var_ln_t1_t2_dn11 = 0.0;
        locals.var_ln_t1_t2_rv = 0.0;

        locals.var_alpha_dd = 0.0;
        locals.var_alpha_dd_dn3 = 0.0;
        locals.var_alpha_dd_dn4 = 0.0;
        locals.var_alpha_dd_dn5 = 0.0;
        locals.var_alpha_dd_dn6 = 0.0;
        locals.var_alpha_dd_dn7 = 0.0;
        locals.var_alpha_dd_dn8 = 0.0;
        locals.var_alpha_dd_dn9 = 0.0;
        locals.var_alpha_dd_dn10 = 0.0;
        locals.var_alpha_dd_dn11 = 0.0;
        locals.var_alpha_dd_rv = 0.0;

        locals.var_qim = 0.0;
        locals.var_qim_dn3 = 0.0;
        locals.var_qim_dn4 = 0.0;
        locals.var_qim_dn5 = 0.0;
        locals.var_qim_dn6 = 0.0;
        locals.var_qim_dn7 = 0.0;
        locals.var_qim_dn8 = 0.0;
        locals.var_qim_dn9 = 0.0;
        locals.var_qim_dn10 = 0.0;
        locals.var_qim_dn11 = 0.0;
        locals.var_qim_rv = 0.0;

        locals.var_h_fact = 0.0;
        locals.var_h_fact_dn3 = 0.0;
        locals.var_h_fact_dn4 = 0.0;
        locals.var_h_fact_dn5 = 0.0;
        locals.var_h_fact_dn6 = 0.0;
        locals.var_h_fact_dn7 = 0.0;
        locals.var_h_fact_dn8 = 0.0;
        locals.var_h_fact_dn9 = 0.0;
        locals.var_h_fact_dn10 = 0.0;
        locals.var_h_fact_dn11 = 0.0;
        locals.var_h_fact_rv = 0.0;

        locals.var_nq_edge = 0.0;
        locals.var_nq_edge_dn3 = 0.0;
        locals.var_nq_edge_dn4 = 0.0;
        locals.var_nq_edge_dn5 = 0.0;
        locals.var_nq_edge_dn6 = 0.0;
        locals.var_nq_edge_dn7 = 0.0;
        locals.var_nq_edge_dn8 = 0.0;
        locals.var_nq_edge_dn9 = 0.0;
        locals.var_nq_edge_dn10 = 0.0;
        locals.var_nq_edge_dn11 = 0.0;
        locals.var_nq_edge_rv = 0.0;

        locals.var_qdeff_edge = 0.0;
        locals.var_qdeff_edge_dn3 = 0.0;
        locals.var_qdeff_edge_dn4 = 0.0;
        locals.var_qdeff_edge_dn5 = 0.0;
        locals.var_qdeff_edge_dn6 = 0.0;
        locals.var_qdeff_edge_dn7 = 0.0;
        locals.var_qdeff_edge_dn8 = 0.0;
        locals.var_qdeff_edge_dn9 = 0.0;
        locals.var_qdeff_edge_dn10 = 0.0;
        locals.var_qdeff_edge_dn11 = 0.0;
        locals.var_qdeff_edge_rv = 0.0;

        locals.var_qs_edge = 0.0;
        locals.var_qs_edge_dn3 = 0.0;
        locals.var_qs_edge_dn4 = 0.0;
        locals.var_qs_edge_dn5 = 0.0;
        locals.var_qs_edge_dn6 = 0.0;
        locals.var_qs_edge_dn7 = 0.0;
        locals.var_qs_edge_dn8 = 0.0;
        locals.var_qs_edge_dn9 = 0.0;
        locals.var_qs_edge_dn10 = 0.0;
        locals.var_qs_edge_dn11 = 0.0;
        locals.var_qs_edge_rv = 0.0;

        let assign1210_e2357: f64 = if p.p30 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign1210_e2357;
        locals.var_guard1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign1220_e2361,) = {
    if (locals.var_guard1 != 0.0) {
        (1.0,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign1220_e2361;
        locals.var_devsign_rv = 0.0;

        let (assign1230_e2367,) = {
    if (locals.var_guard1 == 0.0) {
        let assign1230_e2365: f64 = (-1.0);
        (assign1230_e2365,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign1230_e2367;
        locals.var_devsign_rv = 0.0;

        let assign1240_e2370: f64 = (p.p109 * 8.8541878128e-12);
        locals.var_epssi = assign1240_e2370;
        locals.var_epssi_rv = 0.0;

        let assign1250_e2373: f64 = (p.p110 * 8.8541878128e-12);
        locals.var_epsox = assign1250_e2373;
        locals.var_epsox_rv = 0.0;

        let assign1260_e2376: f64 = (p.p110 * 8.8541878128e-12);
        let assign1260_e2378: f64 = (assign1260_e2376 / p.p76);
        locals.var_cox = assign1260_e2378;
        locals.var_cox_rv = 0.0;

        let assign1270_e2381: f64 = (p.p109 / p.p110);
        locals.var_epsratio = assign1270_e2381;
        locals.var_epsratio_rv = 0.0;

        let assign1280_e2384: f64 = if (!param_given[77]) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign1280_e2384;
        locals.var_guard2_rv = 0.0;

        let (assign1290_e2394,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1290_e2388: f64 = (p.p76 * p.p110);
        let assign1290_e2390: f64 = (assign1290_e2388 / 3.9);
        let assign1290_e2392: f64 = (assign1290_e2390 - p.p78);
        (assign1290_e2392,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1290_e2394;
        locals.var_bsimbulktoxp_rv = 0.0;

        let (assign1300_e2399,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p77,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1300_e2399;
        locals.var_bsimbulktoxp_rv = 0.0;

        let assign1310_e2402: f64 = (p.p0 * p.p49);
        locals.var_l_mult = assign1310_e2402;
        locals.var_l_mult_rv = 0.0;

        let assign1320_e2405: f64 = (p.p1 * p.p50);
        locals.var_w_mult = assign1320_e2405;
        locals.var_w_mult_rv = 0.0;

        let assign1330_e2408: f64 = (locals.var_l_mult + p.p51);
        locals.var_lnew = assign1330_e2408;
        locals.var_lnew_rv = 0.0;

        let assign1350_e2414: f64 = (locals.var_w_mult / p.p2);
        locals.var_w_by_nf = assign1350_e2414;
        locals.var_w_by_nf_rv = 0.0;

        let assign1360_e2417: f64 = (locals.var_w_by_nf + p.p53);
        locals.var_wnew = assign1360_e2417;
        locals.var_wnew_rv = 0.0;

        let assign1380_e2423: f64 = (-p.p58);
        let assign1380_e2424: f64 = (locals.var_lnew).powf(assign1380_e2423);
        locals.var_l_lln = assign1380_e2424;
        locals.var_l_lln_rv = 0.0;

        let assign1390_e2427: f64 = (-p.p59);
        let assign1390_e2428: f64 = (locals.var_wnew).powf(assign1390_e2427);
        locals.var_w_lwn = assign1390_e2428;
        locals.var_w_lwn_rv = 0.0;

        let assign1400_e2431: f64 = (locals.var_l_lln * locals.var_w_lwn);
        locals.var_lw_lln_lwn = assign1400_e2431;
        locals.var_lw_lln_lwn_rv = 0.0;

        let assign1410_e2435: f64 = (p.p55 * locals.var_l_lln);
        let assign1410_e2436: f64 = (p.p54 + assign1410_e2435);
        let assign1410_e2439: f64 = (p.p56 * locals.var_w_lwn);
        let assign1410_e2440: f64 = (assign1410_e2436 + assign1410_e2439);
        let assign1410_e2443: f64 = (p.p57 * locals.var_lw_lln_lwn);
        let assign1410_e2444: f64 = (assign1410_e2440 + assign1410_e2443);
        locals.var_dliv = assign1410_e2444;
        locals.var_dliv_rv = 0.0;

        let assign1420_e2447: f64 = (-p.p64);
        let assign1420_e2448: f64 = (locals.var_lnew).powf(assign1420_e2447);
        locals.var_l_wln = assign1420_e2448;
        locals.var_l_wln_rv = 0.0;

        let assign1430_e2451: f64 = (-p.p65);
        let assign1430_e2452: f64 = (locals.var_wnew).powf(assign1430_e2451);
        locals.var_w_wwn = assign1430_e2452;
        locals.var_w_wwn_rv = 0.0;

        let assign1440_e2455: f64 = (locals.var_l_wln * locals.var_w_wwn);
        locals.var_lw_wln_wwn = assign1440_e2455;
        locals.var_lw_wln_wwn_rv = 0.0;

        let assign1450_e2459: f64 = (p.p61 * locals.var_l_wln);
        let assign1450_e2460: f64 = (p.p60 + assign1450_e2459);
        let assign1450_e2463: f64 = (p.p62 * locals.var_w_wwn);
        let assign1450_e2464: f64 = (assign1450_e2460 + assign1450_e2463);
        let assign1450_e2467: f64 = (p.p63 * locals.var_lw_wln_wwn);
        let assign1450_e2468: f64 = (assign1450_e2464 + assign1450_e2467);
        locals.var_dwiv = assign1450_e2468;
        locals.var_dwiv_rv = 0.0;

        let assign1460_e2472: f64 = (2.0 * locals.var_dliv);
        let assign1460_e2473: f64 = (locals.var_lnew - assign1460_e2472);
        locals.var_leff = assign1460_e2473;
        locals.var_leff_rv = 0.0;

        let assign1490_e2483: f64 = (p.p1375 * p.p1376);
        let assign1490_e2484: f64 = (locals.var_wnew - assign1490_e2483);
        let assign1490_e2487: f64 = (2.0 - p.p1375);
        let assign1490_e2489: f64 = (assign1490_e2487 * locals.var_dwiv);
        let assign1490_e2490: f64 = (assign1490_e2484 - assign1490_e2489);
        locals.var_weff = assign1490_e2490;
        locals.var_weff_rv = 0.0;

        let assign1520_e2500: f64 = (p.p67 * locals.var_l_lln);
        let assign1520_e2501: f64 = (p.p66 + assign1520_e2500);
        let assign1520_e2504: f64 = (p.p68 * locals.var_w_lwn);
        let assign1520_e2505: f64 = (assign1520_e2501 + assign1520_e2504);
        let assign1520_e2508: f64 = (p.p69 * locals.var_lw_lln_lwn);
        let assign1520_e2509: f64 = (assign1520_e2505 + assign1520_e2508);
        locals.var_dlcv = assign1520_e2509;
        locals.var_dlcv_rv = 0.0;

        let assign1530_e2513: f64 = (p.p71 * locals.var_l_wln);
        let assign1530_e2514: f64 = (p.p70 + assign1530_e2513);
        let assign1530_e2517: f64 = (p.p72 * locals.var_w_wwn);
        let assign1530_e2518: f64 = (assign1530_e2514 + assign1530_e2517);
        let assign1530_e2521: f64 = (p.p73 * locals.var_lw_wln_wwn);
        let assign1530_e2522: f64 = (assign1530_e2518 + assign1530_e2521);
        locals.var_dwcv = assign1530_e2522;
        locals.var_dwcv_rv = 0.0;

        let assign1540_e2526: f64 = (2.0 * locals.var_dlcv);
        let assign1540_e2527: f64 = (locals.var_lnew - assign1540_e2526);
        locals.var_lact = assign1540_e2527;
        locals.var_lact_rv = 0.0;

        let assign1570_e2537: f64 = (p.p1375 * p.p1376);
        let assign1570_e2538: f64 = (locals.var_wnew - assign1570_e2537);
        let assign1570_e2541: f64 = (2.0 - p.p1375);
        let assign1570_e2543: f64 = (assign1570_e2541 * locals.var_dwcv);
        let assign1570_e2544: f64 = (assign1570_e2538 - assign1570_e2543);
        locals.var_wact = assign1570_e2544;
        locals.var_wact_rv = 0.0;

        let assign1600_e2555: f64 = (locals.var_lnew).powf(p.p64);
        let assign1600_e2556: f64 = (p.p71 / assign1600_e2555);
        let assign1600_e2557: f64 = (p.p927 + assign1600_e2556);
        let assign1600_e2561: f64 = (locals.var_wnew).powf(p.p65);
        let assign1600_e2562: f64 = (p.p72 / assign1600_e2561);
        let assign1600_e2563: f64 = (assign1600_e2557 + assign1600_e2562);
        let assign1600_e2567: f64 = (locals.var_lnew).powf(p.p64);
        let assign1600_e2568: f64 = (p.p73 / assign1600_e2567);
        let assign1600_e2571: f64 = (locals.var_wnew).powf(p.p65);
        let assign1600_e2572: f64 = (assign1600_e2568 / assign1600_e2571);
        let assign1600_e2573: f64 = (assign1600_e2563 + assign1600_e2572);
        locals.var_dwj = assign1600_e2573;
        locals.var_dwj_rv = 0.0;

        let assign1610_e2577: f64 = (2.0 * locals.var_dwj);
        let assign1610_e2578: f64 = (locals.var_wnew - assign1610_e2577);
        locals.var_weffcj = assign1610_e2578;
        locals.var_weffcj_rv = 0.0;

        let assign1630_e2584: f64 = (1e-6 / locals.var_leff);
        locals.var_inv_l = assign1630_e2584;
        locals.var_inv_l_rv = 0.0;

        let assign1640_e2587: f64 = (1e-6 / locals.var_weff);
        locals.var_inv_w = assign1640_e2587;
        locals.var_inv_w_rv = 0.0;

        let assign1650_e2590: f64 = (1e-6 / locals.var_lact);
        locals.var_inv_lact = assign1650_e2590;
        locals.var_inv_lact_rv = 0.0;

        let assign1660_e2593: f64 = (1e-6 / locals.var_wact);
        locals.var_inv_wact = assign1660_e2593;
        locals.var_inv_wact_rv = 0.0;

        let assign1670_e2596: f64 = (1e-6 / p.p48);
        locals.var_inv_llong = assign1670_e2596;
        locals.var_inv_llong_rv = 0.0;

        let assign1680_e2599: f64 = (1e-6 / p.p52);
        locals.var_inv_wwide = assign1680_e2599;
        locals.var_inv_wwide_rv = 0.0;

        let assign1690_e2602: f64 = (locals.var_inv_l * locals.var_inv_w);
        locals.var_inv_wl = assign1690_e2602;
        locals.var_inv_wl_rv = 0.0;

        locals.var_l_lln1 = locals.var_l_lln;
        locals.var_l_lln1_rv = 0.0;

        locals.var_l_wln1 = locals.var_l_wln;
        locals.var_l_wln1_rv = 0.0;

        let assign1720_e2607: f64 = if p.p1026 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1720_e2607;
        locals.var_guard14_rv = 0.0;

        let assign1730_e2610: f64 = (-locals.var_lnew);
        let assign1730_e2611: f64 = if p.p1026 <= assign1730_e2610 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1730_e2611;
        locals.var_guard15_rv = 0.0;

        let (assign1740_e2623,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1740_e2618: f64 = (locals.var_lnew + p.p1026);
        let assign1740_e2620: f64 = (-p.p58);
        let assign1740_e2621: f64 = (assign1740_e2618).powf(assign1740_e2620);
        (assign1740_e2621,)
    } else {
        (locals.var_l_lln1,)
    }
};
        locals.var_l_lln1 = assign1740_e2623;
        locals.var_l_lln1_rv = 0.0;

        let (assign1750_e2635,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1750_e2630: f64 = (locals.var_lnew + p.p1026);
        let assign1750_e2632: f64 = (-p.p64);
        let assign1750_e2633: f64 = (assign1750_e2630).powf(assign1750_e2632);
        (assign1750_e2633,)
    } else {
        (locals.var_l_wln1,)
    }
};
        locals.var_l_wln1 = assign1750_e2635;
        locals.var_l_wln1_rv = 0.0;

        locals.var_w_lwn1 = locals.var_w_lwn;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_w_wwn1 = locals.var_w_wwn;
        locals.var_w_wwn1_rv = 0.0;

        let assign1780_e2640: f64 = if p.p1027 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1780_e2640;
        locals.var_guard16_rv = 0.0;

        let assign1790_e2643: f64 = (-locals.var_wnew);
        let assign1790_e2644: f64 = if p.p1027 <= assign1790_e2643 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1790_e2644;
        locals.var_guard17_rv = 0.0;

        let (assign1800_e2656,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1800_e2651: f64 = (locals.var_wnew + p.p1027);
        let assign1800_e2653: f64 = (-p.p59);
        let assign1800_e2654: f64 = (assign1800_e2651).powf(assign1800_e2653);
        (assign1800_e2654,)
    } else {
        (locals.var_w_lwn1,)
    }
};
        locals.var_w_lwn1 = assign1800_e2656;
        locals.var_w_lwn1_rv = 0.0;

        let (assign1810_e2668,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1810_e2663: f64 = (locals.var_wnew + p.p1027);
        let assign1810_e2665: f64 = (-p.p65);
        let assign1810_e2666: f64 = (assign1810_e2663).powf(assign1810_e2665);
        (assign1810_e2666,)
    } else {
        (locals.var_w_wwn1,)
    }
};
        locals.var_w_wwn1 = assign1810_e2668;
        locals.var_w_wwn1_rv = 0.0;

        let assign1820_e2671: f64 = (locals.var_l_lln1 * locals.var_w_lwn1);
        locals.var_lw_lln_lwn1 = assign1820_e2671;
        locals.var_lw_lln_lwn1_rv = 0.0;

        let assign1830_e2675: f64 = (p.p55 * locals.var_l_lln1);
        let assign1830_e2676: f64 = (p.p54 + assign1830_e2675);
        let assign1830_e2679: f64 = (p.p56 * locals.var_w_lwn1);
        let assign1830_e2680: f64 = (assign1830_e2676 + assign1830_e2679);
        let assign1830_e2683: f64 = (p.p57 * locals.var_lw_lln_lwn1);
        let assign1830_e2684: f64 = (assign1830_e2680 + assign1830_e2683);
        locals.var_dlb = assign1830_e2684;
        locals.var_dlb_rv = 0.0;

        let assign1840_e2687: f64 = (locals.var_l_wln1 * locals.var_w_wwn1);
        locals.var_lw_wln_wwn1 = assign1840_e2687;
        locals.var_lw_wln_wwn1_rv = 0.0;

        let assign1850_e2691: f64 = (p.p61 * locals.var_l_wln1);
        let assign1850_e2692: f64 = (p.p60 + assign1850_e2691);
        let assign1850_e2695: f64 = (p.p62 * locals.var_w_wwn1);
        let assign1850_e2696: f64 = (assign1850_e2692 + assign1850_e2695);
        let assign1850_e2699: f64 = (p.p63 * locals.var_lw_wln_wwn1);
        let assign1850_e2700: f64 = (assign1850_e2696 + assign1850_e2699);
        locals.var_dwb = assign1850_e2700;
        locals.var_dwb_rv = 0.0;

        let assign1860_e2704: f64 = (2.0 * locals.var_dlb);
        let assign1860_e2705: f64 = (locals.var_lnew - assign1860_e2704);
        let assign1860_e2707: f64 = (assign1860_e2705 + p.p1026);
        locals.var_leff1 = assign1860_e2707;
        locals.var_leff1_rv = 0.0;

        let assign1880_e2714: f64 = (2.0 * locals.var_dwb);
        let assign1880_e2715: f64 = (locals.var_wnew - assign1880_e2714);
        let assign1880_e2717: f64 = (assign1880_e2715 + p.p1027);
        locals.var_weff1 = assign1880_e2717;
        locals.var_weff1_rv = 0.0;

        let assign1900_e2723: f64 = if p.p1025 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1900_e2723;
        locals.var_guard20_rv = 0.0;

        let (assign1910_e2729,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1910_e2727: f64 = (1e-6 / locals.var_leff1);
        (assign1910_e2727,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1910_e2729;
        locals.var_bin_l_rv = 0.0;

        let (assign1920_e2735,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1920_e2733: f64 = (1e-6 / locals.var_weff1);
        (assign1920_e2733,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1920_e2735;
        locals.var_bin_w_rv = 0.0;

        let (assign1930_e2742,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1930_e2740: f64 = (1.0 / locals.var_leff1);
        (assign1930_e2740,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1930_e2742;
        locals.var_bin_l_rv = 0.0;

        let (assign1940_e2749,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1940_e2747: f64 = (1.0 / locals.var_weff1);
        (assign1940_e2747,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1940_e2749;
        locals.var_bin_w_rv = 0.0;

        let assign1950_e2752: f64 = (locals.var_bin_l * locals.var_bin_w);
        locals.var_bin_wl = assign1950_e2752;
        locals.var_bin_wl_rv = 0.0;

        let assign1960_e2756: f64 = (locals.var_bin_l * p.p116);
        let assign1960_e2757: f64 = (p.p115 + assign1960_e2756);
        let assign1960_e2760: f64 = (locals.var_bin_w * p.p117);
        let assign1960_e2761: f64 = (assign1960_e2757 + assign1960_e2760);
        let assign1960_e2764: f64 = (locals.var_bin_wl * p.p118);
        let assign1960_e2765: f64 = (assign1960_e2761 + assign1960_e2764);
        locals.var_vfb_i = assign1960_e2765;
        locals.var_vfb_i_dn3 = 0.0;
        locals.var_vfb_i_dn4 = 0.0;
        locals.var_vfb_i_dn5 = 0.0;
        locals.var_vfb_i_dn6 = 0.0;
        locals.var_vfb_i_dn7 = 0.0;
        locals.var_vfb_i_dn8 = 0.0;
        locals.var_vfb_i_dn9 = 0.0;
        locals.var_vfb_i_dn10 = 0.0;
        locals.var_vfb_i_dn11 = 0.0;
        locals.var_vfb_i_rv = 0.0;

        let assign1970_e2769: f64 = (locals.var_bin_l * p.p120);
        let assign1970_e2770: f64 = (p.p119 + assign1970_e2769);
        let assign1970_e2773: f64 = (locals.var_bin_w * p.p121);
        let assign1970_e2774: f64 = (assign1970_e2770 + assign1970_e2773);
        let assign1970_e2777: f64 = (locals.var_bin_wl * p.p122);
        let assign1970_e2778: f64 = (assign1970_e2774 + assign1970_e2777);
        locals.var_vfbb_i = assign1970_e2778;
        locals.var_vfbb_i_rv = 0.0;

        let assign1980_e2782: f64 = (locals.var_bin_l * p.p130);
        let assign1980_e2783: f64 = (p.p129 + assign1980_e2782);
        let assign1980_e2786: f64 = (locals.var_bin_w * p.p131);
        let assign1980_e2787: f64 = (assign1980_e2783 + assign1980_e2786);
        let assign1980_e2790: f64 = (locals.var_bin_wl * p.p132);
        let assign1980_e2791: f64 = (assign1980_e2787 + assign1980_e2790);
        locals.var_vfbcv_i = assign1980_e2791;
        locals.var_vfbcv_i_dn3 = 0.0;
        locals.var_vfbcv_i_dn4 = 0.0;
        locals.var_vfbcv_i_dn5 = 0.0;
        locals.var_vfbcv_i_dn6 = 0.0;
        locals.var_vfbcv_i_dn7 = 0.0;
        locals.var_vfbcv_i_dn8 = 0.0;
        locals.var_vfbcv_i_dn9 = 0.0;
        locals.var_vfbcv_i_dn10 = 0.0;
        locals.var_vfbcv_i_dn11 = 0.0;
        locals.var_vfbcv_i_rv = 0.0;

        let assign1990_e2795: f64 = (locals.var_bin_l * p.p143);
        let assign1990_e2796: f64 = (p.p142 + assign1990_e2795);
        let assign1990_e2799: f64 = (locals.var_bin_w * p.p144);
        let assign1990_e2800: f64 = (assign1990_e2796 + assign1990_e2799);
        let assign1990_e2803: f64 = (locals.var_bin_wl * p.p145);
        let assign1990_e2804: f64 = (assign1990_e2800 + assign1990_e2803);
        locals.var_nsd_i = assign1990_e2804;
        locals.var_nsd_i_rv = 0.0;

        let assign2000_e2808: f64 = (locals.var_bin_l * p.p88);
        let assign2000_e2809: f64 = (p.p79 + assign2000_e2808);
        let assign2000_e2812: f64 = (locals.var_bin_w * p.p89);
        let assign2000_e2813: f64 = (assign2000_e2809 + assign2000_e2812);
        let assign2000_e2816: f64 = (locals.var_bin_wl * p.p90);
        let assign2000_e2817: f64 = (assign2000_e2813 + assign2000_e2816);
        locals.var_ndep_i = assign2000_e2817;
        locals.var_ndep_i_dn3 = 0.0;
        locals.var_ndep_i_dn4 = 0.0;
        locals.var_ndep_i_dn5 = 0.0;
        locals.var_ndep_i_dn6 = 0.0;
        locals.var_ndep_i_dn7 = 0.0;
        locals.var_ndep_i_dn8 = 0.0;
        locals.var_ndep_i_dn9 = 0.0;
        locals.var_ndep_i_dn10 = 0.0;
        locals.var_ndep_i_dn11 = 0.0;
        locals.var_ndep_i_rv = 0.0;

        let assign2010_e2821: f64 = (locals.var_bin_l * p.p100);
        let assign2010_e2822: f64 = (p.p91 + assign2010_e2821);
        let assign2010_e2825: f64 = (locals.var_bin_w * p.p101);
        let assign2010_e2826: f64 = (assign2010_e2822 + assign2010_e2825);
        let assign2010_e2829: f64 = (locals.var_bin_wl * p.p102);
        let assign2010_e2830: f64 = (assign2010_e2826 + assign2010_e2829);
        locals.var_ndepcv_i = assign2010_e2830;
        locals.var_ndepcv_i_dn3 = 0.0;
        locals.var_ndepcv_i_dn4 = 0.0;
        locals.var_ndepcv_i_dn5 = 0.0;
        locals.var_ndepcv_i_dn6 = 0.0;
        locals.var_ndepcv_i_dn7 = 0.0;
        locals.var_ndepcv_i_dn8 = 0.0;
        locals.var_ndepcv_i_dn9 = 0.0;
        locals.var_ndepcv_i_dn10 = 0.0;
        locals.var_ndepcv_i_dn11 = 0.0;
        locals.var_ndepcv_i_rv = 0.0;

        let assign2020_e2834: f64 = (locals.var_bin_l * p.p104);
        let assign2020_e2835: f64 = (p.p103 + assign2020_e2834);
        let assign2020_e2838: f64 = (locals.var_bin_w * p.p105);
        let assign2020_e2839: f64 = (assign2020_e2835 + assign2020_e2838);
        let assign2020_e2842: f64 = (locals.var_bin_wl * p.p106);
        let assign2020_e2843: f64 = (assign2020_e2839 + assign2020_e2842);
        locals.var_ngate_i = assign2020_e2843;
        locals.var_ngate_i_rv = 0.0;

        let assign2030_e2847: f64 = (locals.var_bin_l * p.p233);
        let assign2030_e2848: f64 = (p.p232 + assign2030_e2847);
        let assign2030_e2851: f64 = (locals.var_bin_w * p.p234);
        let assign2030_e2852: f64 = (assign2030_e2848 + assign2030_e2851);
        let assign2030_e2855: f64 = (locals.var_bin_wl * p.p235);
        let assign2030_e2856: f64 = (assign2030_e2852 + assign2030_e2855);
        locals.var_cit_i = assign2030_e2856;
        locals.var_cit_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2040_e2860: f64 = (locals.var_bin_l * p.p243);
        let assign2040_e2861: f64 = (p.p236 + assign2040_e2860);
        let assign2040_e2864: f64 = (locals.var_bin_w * p.p244);
        let assign2040_e2865: f64 = (assign2040_e2861 + assign2040_e2864);
        let assign2040_e2868: f64 = (locals.var_bin_wl * p.p245);
        let assign2040_e2869: f64 = (assign2040_e2865 + assign2040_e2868);
        locals.var_nfactor_i = assign2040_e2869;
        locals.var_nfactor_i_dn3 = 0.0;
        locals.var_nfactor_i_dn4 = 0.0;
        locals.var_nfactor_i_dn5 = 0.0;
        locals.var_nfactor_i_dn6 = 0.0;
        locals.var_nfactor_i_dn7 = 0.0;
        locals.var_nfactor_i_dn8 = 0.0;
        locals.var_nfactor_i_dn9 = 0.0;
        locals.var_nfactor_i_dn10 = 0.0;
        locals.var_nfactor_i_dn11 = 0.0;
        locals.var_nfactor_i_rv = 0.0;

        let assign2050_e2873: f64 = (p.p247 * locals.var_bin_l);
        let assign2050_e2874: f64 = (p.p246 + assign2050_e2873);
        let assign2050_e2877: f64 = (p.p248 * locals.var_bin_w);
        let assign2050_e2878: f64 = (assign2050_e2874 + assign2050_e2877);
        let assign2050_e2881: f64 = (p.p249 * locals.var_bin_wl);
        let assign2050_e2882: f64 = (assign2050_e2878 + assign2050_e2881);
        locals.var_ascl_i = assign2050_e2882;
        locals.var_ascl_i_rv = 0.0;

        let assign2060_e2886: f64 = (p.p251 * locals.var_bin_l);
        let assign2060_e2887: f64 = (p.p250 + assign2060_e2886);
        let assign2060_e2890: f64 = (p.p252 * locals.var_bin_w);
        let assign2060_e2891: f64 = (assign2060_e2887 + assign2060_e2890);
        let assign2060_e2894: f64 = (p.p253 * locals.var_bin_wl);
        let assign2060_e2895: f64 = (assign2060_e2891 + assign2060_e2894);
        locals.var_bscl_i = assign2060_e2895;
        locals.var_bscl_i_rv = 0.0;

        let assign2070_e2899: f64 = (p.p171 * locals.var_bin_l);
        let assign2070_e2900: f64 = (p.p170 + assign2070_e2899);
        let assign2070_e2903: f64 = (p.p172 * locals.var_bin_w);
        let assign2070_e2904: f64 = (assign2070_e2900 + assign2070_e2903);
        let assign2070_e2907: f64 = (p.p173 * locals.var_bin_wl);
        let assign2070_e2908: f64 = (assign2070_e2904 + assign2070_e2907);
        locals.var_dvbd0_i = assign2070_e2908;
        locals.var_dvbd0_i_rv = 0.0;

        let assign2080_e2912: f64 = (p.p175 * locals.var_bin_l);
        let assign2080_e2913: f64 = (p.p174 + assign2080_e2912);
        let assign2080_e2916: f64 = (p.p176 * locals.var_bin_w);
        let assign2080_e2917: f64 = (assign2080_e2913 + assign2080_e2916);
        let assign2080_e2920: f64 = (p.p177 * locals.var_bin_wl);
        let assign2080_e2921: f64 = (assign2080_e2917 + assign2080_e2920);
        locals.var_dvbd1_i = assign2080_e2921;
        locals.var_dvbd1_i_rv = 0.0;

        let assign2090_e2925: f64 = (p.p179 * locals.var_bin_l);
        let assign2090_e2926: f64 = (p.p178 + assign2090_e2925);
        let assign2090_e2929: f64 = (p.p180 * locals.var_bin_w);
        let assign2090_e2930: f64 = (assign2090_e2926 + assign2090_e2929);
        let assign2090_e2933: f64 = (p.p181 * locals.var_bin_wl);
        let assign2090_e2934: f64 = (assign2090_e2930 + assign2090_e2933);
        locals.var_vsce_i = assign2090_e2934;
        locals.var_vsce_i_rv = 0.0;

        let assign2100_e2938: f64 = (p.p187 * locals.var_bin_l);
        let assign2100_e2939: f64 = (p.p186 + assign2100_e2938);
        let assign2100_e2942: f64 = (p.p188 * locals.var_bin_w);
        let assign2100_e2943: f64 = (assign2100_e2939 + assign2100_e2942);
        let assign2100_e2946: f64 = (p.p189 * locals.var_bin_wl);
        let assign2100_e2947: f64 = (assign2100_e2943 + assign2100_e2946);
        locals.var_cdsbs_i = assign2100_e2947;
        locals.var_cdsbs_i_rv = 0.0;

        let assign2110_e2951: f64 = (p.p183 * locals.var_bin_l);
        let assign2110_e2952: f64 = (p.p182 + assign2110_e2951);
        let assign2110_e2955: f64 = (p.p184 * locals.var_bin_w);
        let assign2110_e2956: f64 = (assign2110_e2952 + assign2110_e2955);
        let assign2110_e2959: f64 = (p.p185 * locals.var_bin_wl);
        let assign2110_e2960: f64 = (assign2110_e2956 + assign2110_e2959);
        locals.var_cdsbs1_i = assign2110_e2960;
        locals.var_cdsbs1_i_rv = 0.0;

        let assign2120_e2964: f64 = (p.p255 * locals.var_bin_l);
        let assign2120_e2965: f64 = (p.p254 + assign2120_e2964);
        let assign2120_e2968: f64 = (p.p256 * locals.var_bin_w);
        let assign2120_e2969: f64 = (assign2120_e2965 + assign2120_e2968);
        let assign2120_e2972: f64 = (p.p257 * locals.var_bin_wl);
        let assign2120_e2973: f64 = (assign2120_e2969 + assign2120_e2972);
        locals.var_dvt1_i = assign2120_e2973;
        locals.var_dvt1_i_rv = 0.0;

        let assign2130_e2977: f64 = (locals.var_bin_l * p.p259);
        let assign2130_e2978: f64 = (p.p258 + assign2130_e2977);
        let assign2130_e2981: f64 = (locals.var_bin_w * p.p260);
        let assign2130_e2982: f64 = (assign2130_e2978 + assign2130_e2981);
        let assign2130_e2985: f64 = (locals.var_bin_wl * p.p261);
        let assign2130_e2986: f64 = (assign2130_e2982 + assign2130_e2985);
        locals.var_cdscd_i = assign2130_e2986;
        locals.var_cdscd_i_dn3 = 0.0;
        locals.var_cdscd_i_dn4 = 0.0;
        locals.var_cdscd_i_dn5 = 0.0;
        locals.var_cdscd_i_dn6 = 0.0;
        locals.var_cdscd_i_dn7 = 0.0;
        locals.var_cdscd_i_dn8 = 0.0;
        locals.var_cdscd_i_dn9 = 0.0;
        locals.var_cdscd_i_dn10 = 0.0;
        locals.var_cdscd_i_dn11 = 0.0;
        locals.var_cdscd_i_rv = 0.0;

        let assign2140_e2990: f64 = (locals.var_bin_l * p.p263);
        let assign2140_e2991: f64 = (p.p262 + assign2140_e2990);
        let assign2140_e2994: f64 = (locals.var_bin_w * p.p264);
        let assign2140_e2995: f64 = (assign2140_e2991 + assign2140_e2994);
        let assign2140_e2998: f64 = (locals.var_bin_wl * p.p265);
        let assign2140_e2999: f64 = (assign2140_e2995 + assign2140_e2998);
        locals.var_cdsc_i = assign2140_e2999;
        locals.var_cdsc_i_rv = 0.0;

        let assign2150_e3003: f64 = (locals.var_bin_l * p.p1165);
        let assign2150_e3004: f64 = (p.p1164 + assign2150_e3003);
        let assign2150_e3007: f64 = (locals.var_bin_w * p.p1166);
        let assign2150_e3008: f64 = (assign2150_e3004 + assign2150_e3007);
        let assign2150_e3011: f64 = (locals.var_bin_wl * p.p1167);
        let assign2150_e3012: f64 = (assign2150_e3008 + assign2150_e3011);
        locals.var_cdscedge_i = assign2150_e3012;
        locals.var_cdscedge_i_rv = 0.0;

        let assign2160_e3016: f64 = (locals.var_bin_l * p.p1192);
        let assign2160_e3017: f64 = (p.p1191 + assign2160_e3016);
        let assign2160_e3020: f64 = (locals.var_bin_w * p.p1193);
        let assign2160_e3021: f64 = (assign2160_e3017 + assign2160_e3020);
        let assign2160_e3024: f64 = (locals.var_bin_wl * p.p1194);
        let assign2160_e3025: f64 = (assign2160_e3021 + assign2160_e3024);
        locals.var_cbcbedge_i = assign2160_e3025;
        locals.var_cbcbedge_i_rv = 0.0;

        let assign2170_e3029: f64 = (locals.var_bin_l * p.p291);
        let assign2170_e3030: f64 = (p.p288 + assign2170_e3029);
        let assign2170_e3033: f64 = (locals.var_bin_w * p.p292);
        let assign2170_e3034: f64 = (assign2170_e3030 + assign2170_e3033);
        let assign2170_e3037: f64 = (locals.var_bin_wl * p.p293);
        let assign2170_e3038: f64 = (assign2170_e3034 + assign2170_e3037);
        locals.var_cdscb_i = assign2170_e3038;
        locals.var_cdscb_i_rv = 0.0;

        let assign2180_e3042: f64 = (locals.var_bin_l * p.p271);
        let assign2180_e3043: f64 = (p.p270 + assign2180_e3042);
        let assign2180_e3046: f64 = (locals.var_bin_w * p.p272);
        let assign2180_e3047: f64 = (assign2180_e3043 + assign2180_e3046);
        let assign2180_e3050: f64 = (locals.var_bin_wl * p.p273);
        let assign2180_e3051: f64 = (assign2180_e3047 + assign2180_e3050);
        locals.var_csecse_i = assign2180_e3051;
        locals.var_csecse_i_rv = 0.0;

        let assign2190_e3055: f64 = (locals.var_bin_l * p.p1177);
        let assign2190_e3056: f64 = (p.p1176 + assign2190_e3055);
        let assign2190_e3059: f64 = (locals.var_bin_w * p.p1178);
        let assign2190_e3060: f64 = (assign2190_e3056 + assign2190_e3059);
        let assign2190_e3063: f64 = (locals.var_bin_wl * p.p1179);
        let assign2190_e3064: f64 = (assign2190_e3060 + assign2190_e3063);
        locals.var_csecseedge_i = assign2190_e3064;
        locals.var_csecseedge_i_rv = 0.0;

        let assign2200_e3068: f64 = (locals.var_bin_l * p.p276);
        let assign2200_e3069: f64 = (p.p275 + assign2200_e3068);
        let assign2200_e3072: f64 = (locals.var_bin_w * p.p277);
        let assign2200_e3073: f64 = (assign2200_e3069 + assign2200_e3072);
        let assign2200_e3076: f64 = (locals.var_bin_wl * p.p278);
        let assign2200_e3077: f64 = (assign2200_e3073 + assign2200_e3076);
        locals.var_cbcb_i = assign2200_e3077;
        locals.var_cbcb_i_rv = 0.0;

        let assign2210_e3081: f64 = (locals.var_bin_l * p.p147);
        let assign2210_e3082: f64 = (p.p146 + assign2210_e3081);
        let assign2210_e3085: f64 = (locals.var_bin_w * p.p148);
        let assign2210_e3086: f64 = (assign2210_e3082 + assign2210_e3085);
        let assign2210_e3089: f64 = (locals.var_bin_wl * p.p149);
        let assign2210_e3090: f64 = (assign2210_e3086 + assign2210_e3089);
        locals.var_dvtp0_i = assign2210_e3090;
        locals.var_dvtp0_i_rv = 0.0;

        let assign2220_e3094: f64 = (locals.var_bin_l * p.p1239);
        let assign2220_e3095: f64 = (p.p1238 + assign2220_e3094);
        let assign2220_e3098: f64 = (locals.var_bin_w * p.p1240);
        let assign2220_e3099: f64 = (assign2220_e3095 + assign2220_e3098);
        let assign2220_e3102: f64 = (locals.var_bin_wl * p.p1241);
        let assign2220_e3103: f64 = (assign2220_e3099 + assign2220_e3102);
        locals.var_dvtp0edge_i = assign2220_e3103;
        locals.var_dvtp0edge_i_rv = 0.0;

        let assign2230_e3107: f64 = (locals.var_bin_l * p.p151);
        let assign2230_e3108: f64 = (p.p150 + assign2230_e3107);
        let assign2230_e3111: f64 = (locals.var_bin_w * p.p152);
        let assign2230_e3112: f64 = (assign2230_e3108 + assign2230_e3111);
        let assign2230_e3115: f64 = (locals.var_bin_wl * p.p153);
        let assign2230_e3116: f64 = (assign2230_e3112 + assign2230_e3115);
        locals.var_dvtp1_i = assign2230_e3116;
        locals.var_dvtp1_i_rv = 0.0;

        let assign2240_e3120: f64 = (locals.var_bin_l * p.p1243);
        let assign2240_e3121: f64 = (p.p1242 + assign2240_e3120);
        let assign2240_e3124: f64 = (locals.var_bin_w * p.p1244);
        let assign2240_e3125: f64 = (assign2240_e3121 + assign2240_e3124);
        let assign2240_e3128: f64 = (locals.var_bin_wl * p.p1245);
        let assign2240_e3129: f64 = (assign2240_e3125 + assign2240_e3128);
        locals.var_dvtp1edge_i = assign2240_e3129;
        locals.var_dvtp1edge_i_rv = 0.0;

        let assign2250_e3133: f64 = (locals.var_bin_l * p.p155);
        let assign2250_e3134: f64 = (p.p154 + assign2250_e3133);
        let assign2250_e3137: f64 = (locals.var_bin_w * p.p156);
        let assign2250_e3138: f64 = (assign2250_e3134 + assign2250_e3137);
        let assign2250_e3141: f64 = (locals.var_bin_wl * p.p157);
        let assign2250_e3142: f64 = (assign2250_e3138 + assign2250_e3141);
        locals.var_dvtp2_i = assign2250_e3142;
        locals.var_dvtp2_i_rv = 0.0;

        let assign2260_e3146: f64 = (locals.var_bin_l * p.p159);
        let assign2260_e3147: f64 = (p.p158 + assign2260_e3146);
        let assign2260_e3150: f64 = (locals.var_bin_w * p.p160);
        let assign2260_e3151: f64 = (assign2260_e3147 + assign2260_e3150);
        let assign2260_e3154: f64 = (locals.var_bin_wl * p.p161);
        let assign2260_e3155: f64 = (assign2260_e3151 + assign2260_e3154);
        locals.var_dvtp3_i = assign2260_e3155;
        locals.var_dvtp3_i_rv = 0.0;

        let assign2270_e3159: f64 = (locals.var_bin_l * p.p163);
        let assign2270_e3160: f64 = (p.p162 + assign2270_e3159);
        let assign2270_e3163: f64 = (locals.var_bin_w * p.p164);
        let assign2270_e3164: f64 = (assign2270_e3160 + assign2270_e3163);
        let assign2270_e3167: f64 = (locals.var_bin_wl * p.p165);
        let assign2270_e3168: f64 = (assign2270_e3164 + assign2270_e3167);
        locals.var_dvtp4_i = assign2270_e3168;
        locals.var_dvtp4_i_rv = 0.0;

        let assign2280_e3172: f64 = (locals.var_bin_l * p.p167);
        let assign2280_e3173: f64 = (p.p166 + assign2280_e3172);
        let assign2280_e3176: f64 = (locals.var_bin_w * p.p168);
        let assign2280_e3177: f64 = (assign2280_e3173 + assign2280_e3176);
        let assign2280_e3180: f64 = (locals.var_bin_wl * p.p169);
        let assign2280_e3181: f64 = (assign2280_e3177 + assign2280_e3180);
        locals.var_dvtp5_i = assign2280_e3181;
        locals.var_dvtp5_i_rv = 0.0;

        let assign2290_e3185: f64 = (locals.var_bin_l * p.p1247);
        let assign2290_e3186: f64 = (p.p1246 + assign2290_e3185);
        let assign2290_e3189: f64 = (locals.var_bin_w * p.p1248);
        let assign2290_e3190: f64 = (assign2290_e3186 + assign2290_e3189);
        let assign2290_e3193: f64 = (locals.var_bin_wl * p.p1249);
        let assign2290_e3194: f64 = (assign2290_e3190 + assign2290_e3193);
        locals.var_dvtp2edge_i = assign2290_e3194;
        locals.var_dvtp2edge_i_rv = 0.0;

        let assign2300_e3198: f64 = (locals.var_bin_l * p.p1251);
        let assign2300_e3199: f64 = (p.p1250 + assign2300_e3198);
        let assign2300_e3202: f64 = (locals.var_bin_w * p.p1252);
        let assign2300_e3203: f64 = (assign2300_e3199 + assign2300_e3202);
        let assign2300_e3206: f64 = (locals.var_bin_wl * p.p1253);
        let assign2300_e3207: f64 = (assign2300_e3203 + assign2300_e3206);
        locals.var_dvtp3edge_i = assign2300_e3207;
        locals.var_dvtp3edge_i_rv = 0.0;

        let assign2310_e3211: f64 = (locals.var_bin_l * p.p1255);
        let assign2310_e3212: f64 = (p.p1254 + assign2310_e3211);
        let assign2310_e3215: f64 = (locals.var_bin_w * p.p1256);
        let assign2310_e3216: f64 = (assign2310_e3212 + assign2310_e3215);
        let assign2310_e3219: f64 = (locals.var_bin_wl * p.p1257);
        let assign2310_e3220: f64 = (assign2310_e3216 + assign2310_e3219);
        locals.var_dvtp4edge_i = assign2310_e3220;
        locals.var_dvtp4edge_i_rv = 0.0;

        let assign2320_e3224: f64 = (locals.var_bin_l * p.p1259);
        let assign2320_e3225: f64 = (p.p1258 + assign2320_e3224);
        let assign2320_e3228: f64 = (locals.var_bin_w * p.p1260);
        let assign2320_e3229: f64 = (assign2320_e3225 + assign2320_e3228);
        let assign2320_e3232: f64 = (locals.var_bin_wl * p.p1261);
        let assign2320_e3233: f64 = (assign2320_e3229 + assign2320_e3232);
        locals.var_dvtp5edge_i = assign2320_e3233;
        locals.var_dvtp5edge_i_rv = 0.0;

        let assign2330_e3237: f64 = (locals.var_bin_l * p.p225);
        let assign2330_e3238: f64 = (p.p218 + assign2330_e3237);
        let assign2330_e3241: f64 = (locals.var_bin_w * p.p226);
        let assign2330_e3242: f64 = (assign2330_e3238 + assign2330_e3241);
        let assign2330_e3245: f64 = (locals.var_bin_wl * p.p227);
        let assign2330_e3246: f64 = (assign2330_e3242 + assign2330_e3245);
        locals.var_k2_i = assign2330_e3246;
        locals.var_k2_i_dn3 = 0.0;
        locals.var_k2_i_dn4 = 0.0;
        locals.var_k2_i_dn5 = 0.0;
        locals.var_k2_i_dn6 = 0.0;
        locals.var_k2_i_dn7 = 0.0;
        locals.var_k2_i_dn8 = 0.0;
        locals.var_k2_i_dn9 = 0.0;
        locals.var_k2_i_dn10 = 0.0;
        locals.var_k2_i_dn11 = 0.0;
        locals.var_k2_i_rv = 0.0;

        let assign2340_e3250: f64 = (locals.var_bin_l * p.p215);
        let assign2340_e3251: f64 = (p.p208 + assign2340_e3250);
        let assign2340_e3254: f64 = (locals.var_bin_w * p.p216);
        let assign2340_e3255: f64 = (assign2340_e3251 + assign2340_e3254);
        let assign2340_e3258: f64 = (locals.var_bin_wl * p.p217);
        let assign2340_e3259: f64 = (assign2340_e3255 + assign2340_e3258);
        locals.var_k1_i = assign2340_e3259;
        locals.var_k1_i_dn3 = 0.0;
        locals.var_k1_i_dn4 = 0.0;
        locals.var_k1_i_dn5 = 0.0;
        locals.var_k1_i_dn6 = 0.0;
        locals.var_k1_i_dn7 = 0.0;
        locals.var_k1_i_dn8 = 0.0;
        locals.var_k1_i_dn9 = 0.0;
        locals.var_k1_i_dn10 = 0.0;
        locals.var_k1_i_dn11 = 0.0;
        locals.var_k1_i_rv = 0.0;

        let assign2350_e3263: f64 = (locals.var_bin_l * p.p1203);
        let assign2350_e3264: f64 = (p.p1196 + assign2350_e3263);
        let assign2350_e3267: f64 = (locals.var_bin_w * p.p1204);
        let assign2350_e3268: f64 = (assign2350_e3264 + assign2350_e3267);
        let assign2350_e3271: f64 = (locals.var_bin_wl * p.p1205);
        let assign2350_e3272: f64 = (assign2350_e3268 + assign2350_e3271);
        locals.var_k1edge_i = assign2350_e3272;
        locals.var_k1edge_i_dn3 = 0.0;
        locals.var_k1edge_i_dn4 = 0.0;
        locals.var_k1edge_i_dn5 = 0.0;
        locals.var_k1edge_i_dn6 = 0.0;
        locals.var_k1edge_i_dn7 = 0.0;
        locals.var_k1edge_i_dn8 = 0.0;
        locals.var_k1edge_i_dn9 = 0.0;
        locals.var_k1edge_i_dn10 = 0.0;
        locals.var_k1edge_i_dn11 = 0.0;
        locals.var_k1edge_i_rv = 0.0;

        let assign2360_e3276: f64 = (locals.var_bin_l * p.p112);
        let assign2360_e3277: f64 = (p.p111 + assign2360_e3276);
        let assign2360_e3280: f64 = (locals.var_bin_w * p.p113);
        let assign2360_e3281: f64 = (assign2360_e3277 + assign2360_e3280);
        let assign2360_e3284: f64 = (locals.var_bin_wl * p.p114);
        let assign2360_e3285: f64 = (assign2360_e3281 + assign2360_e3284);
        locals.var_xj_i = assign2360_e3285;
        locals.var_xj_i_rv = 0.0;

        let assign2370_e3289: f64 = (locals.var_bin_l * p.p191);
        let assign2370_e3290: f64 = (p.p190 + assign2370_e3289);
        let assign2370_e3293: f64 = (locals.var_bin_w * p.p192);
        let assign2370_e3294: f64 = (assign2370_e3290 + assign2370_e3293);
        let assign2370_e3297: f64 = (locals.var_bin_wl * p.p193);
        let assign2370_e3298: f64 = (assign2370_e3294 + assign2370_e3297);
        locals.var_phin_i = assign2370_e3298;
        locals.var_phin_i_rv = 0.0;

        let assign2380_e3302: f64 = (locals.var_bin_l * p.p195);
        let assign2380_e3303: f64 = (p.p194 + assign2380_e3302);
        let assign2380_e3306: f64 = (locals.var_bin_w * p.p196);
        let assign2380_e3307: f64 = (assign2380_e3303 + assign2380_e3306);
        let assign2380_e3310: f64 = (locals.var_bin_wl * p.p197);
        let assign2380_e3311: f64 = (assign2380_e3307 + assign2380_e3310);
        locals.var_eta0_i = assign2380_e3311;
        locals.var_eta0_i_dn3 = 0.0;
        locals.var_eta0_i_dn4 = 0.0;
        locals.var_eta0_i_dn5 = 0.0;
        locals.var_eta0_i_dn6 = 0.0;
        locals.var_eta0_i_dn7 = 0.0;
        locals.var_eta0_i_dn8 = 0.0;
        locals.var_eta0_i_dn9 = 0.0;
        locals.var_eta0_i_dn10 = 0.0;
        locals.var_eta0_i_dn11 = 0.0;
        locals.var_eta0_i_rv = 0.0;

        let assign2390_e3315: f64 = (locals.var_bin_l * p.p205);
        let assign2390_e3316: f64 = (p.p203 + assign2390_e3315);
        let assign2390_e3319: f64 = (locals.var_bin_w * p.p206);
        let assign2390_e3320: f64 = (assign2390_e3316 + assign2390_e3319);
        let assign2390_e3323: f64 = (locals.var_bin_wl * p.p207);
        let assign2390_e3324: f64 = (assign2390_e3320 + assign2390_e3323);
        locals.var_etab_i = assign2390_e3324;
        locals.var_etab_i_rv = 0.0;

        let assign2400_e3328: f64 = (locals.var_bin_l * p.p310);
        let assign2400_e3329: f64 = (p.p309 + assign2400_e3328);
        let assign2400_e3332: f64 = (locals.var_bin_w * p.p311);
        let assign2400_e3333: f64 = (assign2400_e3329 + assign2400_e3332);
        let assign2400_e3336: f64 = (locals.var_bin_wl * p.p312);
        let assign2400_e3337: f64 = (assign2400_e3333 + assign2400_e3336);
        locals.var_delta_i = assign2400_e3337;
        locals.var_delta_i_dn3 = 0.0;
        locals.var_delta_i_dn4 = 0.0;
        locals.var_delta_i_dn5 = 0.0;
        locals.var_delta_i_dn6 = 0.0;
        locals.var_delta_i_dn7 = 0.0;
        locals.var_delta_i_dn8 = 0.0;
        locals.var_delta_i_dn9 = 0.0;
        locals.var_delta_i_dn10 = 0.0;
        locals.var_delta_i_dn11 = 0.0;
        locals.var_delta_i_rv = 0.0;

        let assign2410_e3341: f64 = (locals.var_bin_l * p.p340);
        let assign2410_e3342: f64 = (p.p337 + assign2410_e3341);
        let assign2410_e3345: f64 = (locals.var_bin_w * p.p341);
        let assign2410_e3346: f64 = (assign2410_e3342 + assign2410_e3345);
        let assign2410_e3349: f64 = (locals.var_bin_wl * p.p342);
        let assign2410_e3350: f64 = (assign2410_e3346 + assign2410_e3349);
        locals.var_u0_i = assign2410_e3350;
        locals.var_u0_i_rv = 0.0;

        let assign2420_e3354: f64 = (locals.var_bin_l * p.p355);
        let assign2420_e3355: f64 = (p.p348 + assign2420_e3354);
        let assign2420_e3358: f64 = (locals.var_bin_w * p.p356);
        let assign2420_e3359: f64 = (assign2420_e3355 + assign2420_e3358);
        let assign2420_e3362: f64 = (locals.var_bin_wl * p.p357);
        let assign2420_e3363: f64 = (assign2420_e3359 + assign2420_e3362);
        locals.var_ua_i = assign2420_e3363;
        locals.var_ua_i_dn3 = 0.0;
        locals.var_ua_i_dn4 = 0.0;
        locals.var_ua_i_dn5 = 0.0;
        locals.var_ua_i_dn6 = 0.0;
        locals.var_ua_i_dn7 = 0.0;
        locals.var_ua_i_dn8 = 0.0;
        locals.var_ua_i_dn9 = 0.0;
        locals.var_ua_i_dn10 = 0.0;
        locals.var_ua_i_dn11 = 0.0;
        locals.var_ua_i_rv = 0.0;

        let assign2430_e3367: f64 = (locals.var_bin_l * p.p375);
        let assign2430_e3368: f64 = (p.p372 + assign2430_e3367);
        let assign2430_e3371: f64 = (locals.var_bin_w * p.p376);
        let assign2430_e3372: f64 = (assign2430_e3368 + assign2430_e3371);
        let assign2430_e3375: f64 = (locals.var_bin_wl * p.p377);
        let assign2430_e3376: f64 = (assign2430_e3372 + assign2430_e3375);
        locals.var_ud_i = assign2430_e3376;
        locals.var_ud_i_dn3 = 0.0;
        locals.var_ud_i_dn4 = 0.0;
        locals.var_ud_i_dn5 = 0.0;
        locals.var_ud_i_dn6 = 0.0;
        locals.var_ud_i_dn7 = 0.0;
        locals.var_ud_i_dn8 = 0.0;
        locals.var_ud_i_dn9 = 0.0;
        locals.var_ud_i_dn10 = 0.0;
        locals.var_ud_i_dn11 = 0.0;
        locals.var_ud_i_rv = 0.0;

        let assign2440_e3380: f64 = (locals.var_bin_l * p.p363);
        let assign2440_e3381: f64 = (p.p362 + assign2440_e3380);
        let assign2440_e3384: f64 = (locals.var_bin_w * p.p364);
        let assign2440_e3385: f64 = (assign2440_e3381 + assign2440_e3384);
        let assign2440_e3388: f64 = (locals.var_bin_wl * p.p365);
        let assign2440_e3389: f64 = (assign2440_e3385 + assign2440_e3388);
        locals.var_eu_i = assign2440_e3389;
        locals.var_eu_i_dn3 = 0.0;
        locals.var_eu_i_dn4 = 0.0;
        locals.var_eu_i_dn5 = 0.0;
        locals.var_eu_i_dn6 = 0.0;
        locals.var_eu_i_dn7 = 0.0;
        locals.var_eu_i_dn8 = 0.0;
        locals.var_eu_i_dn9 = 0.0;
        locals.var_eu_i_dn10 = 0.0;
        locals.var_eu_i_dn11 = 0.0;
        locals.var_eu_i_rv = 0.0;

        let assign2450_e3393: f64 = (locals.var_bin_l * p.p383);
        let assign2450_e3394: f64 = (p.p382 + assign2450_e3393);
        let assign2450_e3397: f64 = (locals.var_bin_w * p.p384);
        let assign2450_e3398: f64 = (assign2450_e3394 + assign2450_e3397);
        let assign2450_e3401: f64 = (locals.var_bin_wl * p.p385);
        let assign2450_e3402: f64 = (assign2450_e3398 + assign2450_e3401);
        locals.var_ucs_i = assign2450_e3402;
        locals.var_ucs_i_rv = 0.0;

        let assign2460_e3406: f64 = (locals.var_bin_l * p.p397);
        let assign2460_e3407: f64 = (p.p390 + assign2460_e3406);
        let assign2460_e3410: f64 = (locals.var_bin_w * p.p398);
        let assign2460_e3411: f64 = (assign2460_e3407 + assign2460_e3410);
        let assign2460_e3414: f64 = (locals.var_bin_wl * p.p399);
        let assign2460_e3415: f64 = (assign2460_e3411 + assign2460_e3414);
        locals.var_uc_i = assign2460_e3415;
        locals.var_uc_i_dn3 = 0.0;
        locals.var_uc_i_dn4 = 0.0;
        locals.var_uc_i_dn5 = 0.0;
        locals.var_uc_i_dn6 = 0.0;
        locals.var_uc_i_dn7 = 0.0;
        locals.var_uc_i_dn8 = 0.0;
        locals.var_uc_i_dn9 = 0.0;
        locals.var_uc_i_dn10 = 0.0;
        locals.var_uc_i_dn11 = 0.0;
        locals.var_uc_i_rv = 0.0;

        let assign2470_e3419: f64 = (locals.var_bin_l * p.p407);
        let assign2470_e3420: f64 = (p.p404 + assign2470_e3419);
        let assign2470_e3423: f64 = (locals.var_bin_w * p.p408);
        let assign2470_e3424: f64 = (assign2470_e3420 + assign2470_e3423);
        let assign2470_e3427: f64 = (locals.var_bin_wl * p.p409);
        let assign2470_e3428: f64 = (assign2470_e3424 + assign2470_e3427);
        locals.var_pclm_i = assign2470_e3428;
        locals.var_pclm_i_dn3 = 0.0;
        locals.var_pclm_i_dn4 = 0.0;
        locals.var_pclm_i_dn5 = 0.0;
        locals.var_pclm_i_dn6 = 0.0;
        locals.var_pclm_i_dn7 = 0.0;
        locals.var_pclm_i_dn8 = 0.0;
        locals.var_pclm_i_dn9 = 0.0;
        locals.var_pclm_i_dn10 = 0.0;
        locals.var_pclm_i_dn11 = 0.0;
        locals.var_pclm_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2480_e3432: f64 = (locals.var_bin_l * p.p418);
        let assign2480_e3433: f64 = (p.p415 + assign2480_e3432);
        let assign2480_e3436: f64 = (locals.var_bin_w * p.p419);
        let assign2480_e3437: f64 = (assign2480_e3433 + assign2480_e3436);
        let assign2480_e3440: f64 = (locals.var_bin_wl * p.p420);
        let assign2480_e3441: f64 = (assign2480_e3437 + assign2480_e3440);
        locals.var_pclmcv_i = assign2480_e3441;
        locals.var_pclmcv_i_rv = 0.0;

        let assign2490_e3445: f64 = (locals.var_bin_l * p.p458);
        let assign2490_e3446: f64 = (p.p457 + assign2490_e3445);
        let assign2490_e3449: f64 = (locals.var_bin_w * p.p459);
        let assign2490_e3450: f64 = (assign2490_e3446 + assign2490_e3449);
        let assign2490_e3453: f64 = (locals.var_bin_wl * p.p460);
        let assign2490_e3454: f64 = (assign2490_e3450 + assign2490_e3453);
        locals.var_rsw_i = assign2490_e3454;
        locals.var_rsw_i_rv = 0.0;

        let assign2500_e3458: f64 = (locals.var_bin_l * p.p468);
        let assign2500_e3459: f64 = (p.p467 + assign2500_e3458);
        let assign2500_e3462: f64 = (locals.var_bin_w * p.p469);
        let assign2500_e3463: f64 = (assign2500_e3459 + assign2500_e3462);
        let assign2500_e3466: f64 = (locals.var_bin_wl * p.p470);
        let assign2500_e3467: f64 = (assign2500_e3463 + assign2500_e3466);
        locals.var_rdw_i = assign2500_e3467;
        locals.var_rdw_i_rv = 0.0;

        let assign2510_e3471: f64 = (locals.var_bin_l * p.p440);
        let assign2510_e3472: f64 = (p.p439 + assign2510_e3471);
        let assign2510_e3475: f64 = (locals.var_bin_w * p.p441);
        let assign2510_e3476: f64 = (assign2510_e3472 + assign2510_e3475);
        let assign2510_e3479: f64 = (locals.var_bin_wl * p.p442);
        let assign2510_e3480: f64 = (assign2510_e3476 + assign2510_e3479);
        locals.var_prwg_i = assign2510_e3480;
        locals.var_prwg_i_rv = 0.0;

        let assign2520_e3484: f64 = (locals.var_bin_l * p.p444);
        let assign2520_e3485: f64 = (p.p443 + assign2520_e3484);
        let assign2520_e3488: f64 = (locals.var_bin_w * p.p445);
        let assign2520_e3489: f64 = (assign2520_e3485 + assign2520_e3488);
        let assign2520_e3492: f64 = (locals.var_bin_wl * p.p446);
        let assign2520_e3493: f64 = (assign2520_e3489 + assign2520_e3492);
        locals.var_prwb_i = assign2520_e3493;
        locals.var_prwb_i_rv = 0.0;

        let assign2530_e3497: f64 = (locals.var_bin_l * p.p450);
        let assign2530_e3498: f64 = (p.p449 + assign2530_e3497);
        let assign2530_e3501: f64 = (locals.var_bin_w * p.p451);
        let assign2530_e3502: f64 = (assign2530_e3498 + assign2530_e3501);
        let assign2530_e3505: f64 = (locals.var_bin_wl * p.p452);
        let assign2530_e3506: f64 = (assign2530_e3502 + assign2530_e3505);
        locals.var_wr_i = assign2530_e3506;
        locals.var_wr_i_rv = 0.0;

        let assign2540_e3510: f64 = (locals.var_bin_l * p.p454);
        let assign2540_e3511: f64 = (p.p453 + assign2540_e3510);
        let assign2540_e3514: f64 = (locals.var_bin_w * p.p455);
        let assign2540_e3515: f64 = (assign2540_e3511 + assign2540_e3514);
        let assign2540_e3518: f64 = (locals.var_bin_wl * p.p456);
        let assign2540_e3519: f64 = (assign2540_e3515 + assign2540_e3518);
        locals.var_rswmin_i = assign2540_e3519;
        locals.var_rswmin_i_rv = 0.0;

        let assign2550_e3523: f64 = (locals.var_bin_l * p.p464);
        let assign2550_e3524: f64 = (p.p463 + assign2550_e3523);
        let assign2550_e3527: f64 = (locals.var_bin_w * p.p465);
        let assign2550_e3528: f64 = (assign2550_e3524 + assign2550_e3527);
        let assign2550_e3531: f64 = (locals.var_bin_wl * p.p466);
        let assign2550_e3532: f64 = (assign2550_e3528 + assign2550_e3531);
        locals.var_rdwmin_i = assign2550_e3532;
        locals.var_rdwmin_i_rv = 0.0;

        let assign2560_e3536: f64 = (locals.var_bin_l * p.p480);
        let assign2560_e3537: f64 = (p.p477 + assign2560_e3536);
        let assign2560_e3540: f64 = (locals.var_bin_w * p.p481);
        let assign2560_e3541: f64 = (assign2560_e3537 + assign2560_e3540);
        let assign2560_e3544: f64 = (locals.var_bin_wl * p.p482);
        let assign2560_e3545: f64 = (assign2560_e3541 + assign2560_e3544);
        locals.var_rdsw_i = assign2560_e3545;
        locals.var_rdsw_i_rv = 0.0;

        let assign2570_e3549: f64 = (locals.var_bin_l * p.p474);
        let assign2570_e3550: f64 = (p.p473 + assign2570_e3549);
        let assign2570_e3553: f64 = (locals.var_bin_w * p.p475);
        let assign2570_e3554: f64 = (assign2570_e3550 + assign2570_e3553);
        let assign2570_e3557: f64 = (locals.var_bin_wl * p.p476);
        let assign2570_e3558: f64 = (assign2570_e3554 + assign2570_e3557);
        locals.var_rdswmin_i = assign2570_e3558;
        locals.var_rdswmin_i_rv = 0.0;

        let assign2580_e3562: f64 = (locals.var_bin_l * p.p499);
        let assign2580_e3563: f64 = (p.p498 + assign2580_e3562);
        let assign2580_e3566: f64 = (locals.var_bin_w * p.p500);
        let assign2580_e3567: f64 = (assign2580_e3563 + assign2580_e3566);
        let assign2580_e3570: f64 = (locals.var_bin_wl * p.p501);
        let assign2580_e3571: f64 = (assign2580_e3567 + assign2580_e3570);
        locals.var_ptwg_i = assign2580_e3571;
        locals.var_ptwg_i_dn3 = 0.0;
        locals.var_ptwg_i_dn4 = 0.0;
        locals.var_ptwg_i_dn5 = 0.0;
        locals.var_ptwg_i_dn6 = 0.0;
        locals.var_ptwg_i_dn7 = 0.0;
        locals.var_ptwg_i_dn8 = 0.0;
        locals.var_ptwg_i_dn9 = 0.0;
        locals.var_ptwg_i_dn10 = 0.0;
        locals.var_ptwg_i_dn11 = 0.0;
        locals.var_ptwg_i_rv = 0.0;

        let assign2590_e3575: f64 = (locals.var_bin_l * p.p533);
        let assign2590_e3576: f64 = (p.p530 + assign2590_e3575);
        let assign2590_e3579: f64 = (locals.var_bin_w * p.p534);
        let assign2590_e3580: f64 = (assign2590_e3576 + assign2590_e3579);
        let assign2590_e3583: f64 = (locals.var_bin_wl * p.p535);
        let assign2590_e3584: f64 = (assign2590_e3580 + assign2590_e3583);
        locals.var_pdiblc_i = assign2590_e3584;
        locals.var_pdiblc_i_dn3 = 0.0;
        locals.var_pdiblc_i_dn4 = 0.0;
        locals.var_pdiblc_i_dn5 = 0.0;
        locals.var_pdiblc_i_dn6 = 0.0;
        locals.var_pdiblc_i_dn7 = 0.0;
        locals.var_pdiblc_i_dn8 = 0.0;
        locals.var_pdiblc_i_dn9 = 0.0;
        locals.var_pdiblc_i_dn10 = 0.0;
        locals.var_pdiblc_i_dn11 = 0.0;
        locals.var_pdiblc_i_rv = 0.0;

        let assign2600_e3588: f64 = (locals.var_bin_l * p.p541);
        let assign2600_e3589: f64 = (p.p540 + assign2600_e3588);
        let assign2600_e3592: f64 = (locals.var_bin_w * p.p542);
        let assign2600_e3593: f64 = (assign2600_e3589 + assign2600_e3592);
        let assign2600_e3596: f64 = (locals.var_bin_wl * p.p543);
        let assign2600_e3597: f64 = (assign2600_e3593 + assign2600_e3596);
        locals.var_pdiblcb_i = assign2600_e3597;
        locals.var_pdiblcb_i_rv = 0.0;

        let assign2610_e3601: f64 = (locals.var_bin_l * p.p422);
        let assign2610_e3602: f64 = (p.p421 + assign2610_e3601);
        let assign2610_e3605: f64 = (locals.var_bin_w * p.p423);
        let assign2610_e3606: f64 = (assign2610_e3602 + assign2610_e3605);
        let assign2610_e3609: f64 = (locals.var_bin_wl * p.p424);
        let assign2610_e3610: f64 = (assign2610_e3606 + assign2610_e3609);
        locals.var_pscbe1_i = assign2610_e3610;
        locals.var_pscbe1_i_rv = 0.0;

        let assign2620_e3614: f64 = (locals.var_bin_l * p.p426);
        let assign2620_e3615: f64 = (p.p425 + assign2620_e3614);
        let assign2620_e3618: f64 = (locals.var_bin_w * p.p427);
        let assign2620_e3619: f64 = (assign2620_e3615 + assign2620_e3618);
        let assign2620_e3622: f64 = (locals.var_bin_wl * p.p428);
        let assign2620_e3623: f64 = (assign2620_e3619 + assign2620_e3622);
        locals.var_pscbe2_i = assign2620_e3623;
        locals.var_pscbe2_i_rv = 0.0;

        let assign2630_e3627: f64 = (locals.var_bin_l * p.p430);
        let assign2630_e3628: f64 = (p.p429 + assign2630_e3627);
        let assign2630_e3631: f64 = (locals.var_bin_w * p.p431);
        let assign2630_e3632: f64 = (assign2630_e3628 + assign2630_e3631);
        let assign2630_e3635: f64 = (locals.var_bin_wl * p.p432);
        let assign2630_e3636: f64 = (assign2630_e3632 + assign2630_e3635);
        locals.var_pdits_i = assign2630_e3636;
        locals.var_pdits_i_rv = 0.0;

        let assign2640_e3640: f64 = (locals.var_bin_l * p.p435);
        let assign2640_e3641: f64 = (p.p434 + assign2640_e3640);
        let assign2640_e3644: f64 = (locals.var_bin_w * p.p436);
        let assign2640_e3645: f64 = (assign2640_e3641 + assign2640_e3644);
        let assign2640_e3648: f64 = (locals.var_bin_wl * p.p437);
        let assign2640_e3649: f64 = (assign2640_e3645 + assign2640_e3648);
        locals.var_pditsd_i = assign2640_e3649;
        locals.var_pditsd_i_rv = 0.0;

        let assign2650_e3653: f64 = (locals.var_bin_l * p.p551);
        let assign2650_e3654: f64 = (p.p548 + assign2650_e3653);
        let assign2650_e3657: f64 = (locals.var_bin_w * p.p552);
        let assign2650_e3658: f64 = (assign2650_e3654 + assign2650_e3657);
        let assign2650_e3661: f64 = (locals.var_bin_wl * p.p553);
        let assign2650_e3662: f64 = (assign2650_e3658 + assign2650_e3661);
        locals.var_fprout_i = assign2650_e3662;
        locals.var_fprout_i_rv = 0.0;

        let assign2660_e3666: f64 = (locals.var_bin_l * p.p545);
        let assign2660_e3667: f64 = (p.p544 + assign2660_e3666);
        let assign2660_e3670: f64 = (locals.var_bin_w * p.p546);
        let assign2660_e3671: f64 = (assign2660_e3667 + assign2660_e3670);
        let assign2660_e3674: f64 = (locals.var_bin_wl * p.p547);
        let assign2660_e3675: f64 = (assign2660_e3671 + assign2660_e3674);
        locals.var_pvag_i = assign2660_e3675;
        locals.var_pvag_i_rv = 0.0;

        let assign2670_e3679: f64 = (locals.var_bin_l * p.p296);
        let assign2670_e3680: f64 = (p.p295 + assign2670_e3679);
        let assign2670_e3683: f64 = (locals.var_bin_w * p.p297);
        let assign2670_e3684: f64 = (assign2670_e3680 + assign2670_e3683);
        let assign2670_e3687: f64 = (locals.var_bin_wl * p.p298);
        let assign2670_e3688: f64 = (assign2670_e3684 + assign2670_e3687);
        locals.var_vsat_i = assign2670_e3688;
        locals.var_vsat_i_dn3 = 0.0;
        locals.var_vsat_i_dn4 = 0.0;
        locals.var_vsat_i_dn5 = 0.0;
        locals.var_vsat_i_dn6 = 0.0;
        locals.var_vsat_i_dn7 = 0.0;
        locals.var_vsat_i_dn8 = 0.0;
        locals.var_vsat_i_dn9 = 0.0;
        locals.var_vsat_i_dn10 = 0.0;
        locals.var_vsat_i_dn11 = 0.0;
        locals.var_vsat_i_rv = 0.0;

        let assign2680_e3692: f64 = (locals.var_bin_l * p.p511);
        let assign2680_e3693: f64 = (p.p510 + assign2680_e3692);
        let assign2680_e3696: f64 = (locals.var_bin_w * p.p512);
        let assign2680_e3697: f64 = (assign2680_e3693 + assign2680_e3696);
        let assign2680_e3700: f64 = (locals.var_bin_wl * p.p513);
        let assign2680_e3701: f64 = (assign2680_e3697 + assign2680_e3700);
        locals.var_ksativ_i = assign2680_e3701;
        locals.var_ksativ_i_rv = 0.0;

        let assign2690_e3705: f64 = (locals.var_bin_l * p.p326);
        let assign2690_e3706: f64 = (p.p325 + assign2690_e3705);
        let assign2690_e3709: f64 = (locals.var_bin_w * p.p327);
        let assign2690_e3710: f64 = (assign2690_e3706 + assign2690_e3709);
        let assign2690_e3713: f64 = (locals.var_bin_wl * p.p328);
        let assign2690_e3714: f64 = (assign2690_e3710 + assign2690_e3713);
        locals.var_thesat_i = assign2690_e3714;
        locals.var_thesat_i_rv = 0.0;

        let assign2700_e3718: f64 = (p.p330 * locals.var_bin_l);
        let assign2700_e3719: f64 = (p.p329 + assign2700_e3718);
        let assign2700_e3722: f64 = (p.p331 * locals.var_bin_w);
        let assign2700_e3723: f64 = (assign2700_e3719 + assign2700_e3722);
        let assign2700_e3726: f64 = (p.p332 * locals.var_bin_wl);
        let assign2700_e3727: f64 = (assign2700_e3723 + assign2700_e3726);
        locals.var_lpe1_i = assign2700_e3727;
        locals.var_lpe1_i_rv = 0.0;

        let assign2710_e3731: f64 = (locals.var_bin_l * p.p484);
        let assign2710_e3732: f64 = (p.p483 + assign2710_e3731);
        let assign2710_e3735: f64 = (locals.var_bin_w * p.p485);
        let assign2710_e3736: f64 = (assign2710_e3732 + assign2710_e3735);
        let assign2710_e3739: f64 = (locals.var_bin_wl * p.p486);
        let assign2710_e3740: f64 = (assign2710_e3736 + assign2710_e3739);
        locals.var_psat_i = assign2710_e3740;
        locals.var_psat_i_rv = 0.0;

        let assign2720_e3744: f64 = (locals.var_bin_l * p.p316);
        let assign2720_e3745: f64 = (p.p315 + assign2720_e3744);
        let assign2720_e3748: f64 = (locals.var_bin_w * p.p317);
        let assign2720_e3749: f64 = (assign2720_e3745 + assign2720_e3748);
        let assign2720_e3752: f64 = (locals.var_bin_wl * p.p318);
        let assign2720_e3753: f64 = (assign2720_e3749 + assign2720_e3752);
        locals.var_vsatcv_i = assign2720_e3753;
        locals.var_vsatcv_i_dn3 = 0.0;
        locals.var_vsatcv_i_dn4 = 0.0;
        locals.var_vsatcv_i_dn5 = 0.0;
        locals.var_vsatcv_i_dn6 = 0.0;
        locals.var_vsatcv_i_dn7 = 0.0;
        locals.var_vsatcv_i_dn8 = 0.0;
        locals.var_vsatcv_i_dn9 = 0.0;
        locals.var_vsatcv_i_dn10 = 0.0;
        locals.var_vsatcv_i_dn11 = 0.0;
        locals.var_vsatcv_i_rv = 0.0;

        let assign2730_e3757: f64 = (locals.var_bin_l * p.p868);
        let assign2730_e3758: f64 = (p.p867 + assign2730_e3757);
        let assign2730_e3761: f64 = (locals.var_bin_w * p.p869);
        let assign2730_e3762: f64 = (assign2730_e3758 + assign2730_e3761);
        let assign2730_e3765: f64 = (locals.var_bin_wl * p.p870);
        let assign2730_e3766: f64 = (assign2730_e3762 + assign2730_e3765);
        locals.var_cf_i = assign2730_e3766;
        locals.var_cf_i_rv = 0.0;

        let assign2740_e3770: f64 = (locals.var_bin_l * p.p876);
        let assign2740_e3771: f64 = (p.p875 + assign2740_e3770);
        let assign2740_e3774: f64 = (locals.var_bin_w * p.p877);
        let assign2740_e3775: f64 = (assign2740_e3771 + assign2740_e3774);
        let assign2740_e3778: f64 = (locals.var_bin_wl * p.p878);
        let assign2740_e3779: f64 = (assign2740_e3775 + assign2740_e3778);
        locals.var_cgsl_i = assign2740_e3779;
        locals.var_cgsl_i_rv = 0.0;

        let assign2750_e3783: f64 = (locals.var_bin_l * p.p880);
        let assign2750_e3784: f64 = (p.p879 + assign2750_e3783);
        let assign2750_e3787: f64 = (locals.var_bin_w * p.p881);
        let assign2750_e3788: f64 = (assign2750_e3784 + assign2750_e3787);
        let assign2750_e3791: f64 = (locals.var_bin_wl * p.p882);
        let assign2750_e3792: f64 = (assign2750_e3788 + assign2750_e3791);
        locals.var_cgdl_i = assign2750_e3792;
        locals.var_cgdl_i_rv = 0.0;

        let assign2760_e3796: f64 = (locals.var_bin_l * p.p884);
        let assign2760_e3797: f64 = (p.p883 + assign2760_e3796);
        let assign2760_e3800: f64 = (locals.var_bin_w * p.p885);
        let assign2760_e3801: f64 = (assign2760_e3797 + assign2760_e3800);
        let assign2760_e3804: f64 = (locals.var_bin_wl * p.p886);
        let assign2760_e3805: f64 = (assign2760_e3801 + assign2760_e3804);
        locals.var_ckappas_i = assign2760_e3805;
        locals.var_ckappas_i_rv = 0.0;

        let assign2770_e3809: f64 = (locals.var_bin_l * p.p888);
        let assign2770_e3810: f64 = (p.p887 + assign2770_e3809);
        let assign2770_e3813: f64 = (locals.var_bin_w * p.p889);
        let assign2770_e3814: f64 = (assign2770_e3810 + assign2770_e3813);
        let assign2770_e3817: f64 = (locals.var_bin_wl * p.p890);
        let assign2770_e3818: f64 = (assign2770_e3814 + assign2770_e3817);
        locals.var_ckappad_i = assign2770_e3818;
        locals.var_ckappad_i_rv = 0.0;

        let assign2780_e3822: f64 = (locals.var_bin_l * p.p604);
        let assign2780_e3823: f64 = (p.p601 + assign2780_e3822);
        let assign2780_e3826: f64 = (locals.var_bin_w * p.p605);
        let assign2780_e3827: f64 = (assign2780_e3823 + assign2780_e3826);
        let assign2780_e3830: f64 = (locals.var_bin_wl * p.p606);
        let assign2780_e3831: f64 = (assign2780_e3827 + assign2780_e3830);
        locals.var_alpha0_i = assign2780_e3831;
        locals.var_alpha0_i_rv = 0.0;

        let assign2790_e3835: f64 = (locals.var_bin_l * p.p608);
        let assign2790_e3836: f64 = (p.p607 + assign2790_e3835);
        let assign2790_e3839: f64 = (locals.var_bin_w * p.p609);
        let assign2790_e3840: f64 = (assign2790_e3836 + assign2790_e3839);
        let assign2790_e3843: f64 = (locals.var_bin_wl * p.p610);
        let assign2790_e3844: f64 = (assign2790_e3840 + assign2790_e3843);
        locals.var_beta0_i = assign2790_e3844;
        locals.var_beta0_i_rv = 0.0;

        let assign2800_e3848: f64 = (locals.var_bin_l * p.p612);
        let assign2800_e3849: f64 = (p.p611 + assign2800_e3848);
        let assign2800_e3852: f64 = (locals.var_bin_w * p.p613);
        let assign2800_e3853: f64 = (assign2800_e3849 + assign2800_e3852);
        let assign2800_e3856: f64 = (locals.var_bin_wl * p.p614);
        let assign2800_e3857: f64 = (assign2800_e3853 + assign2800_e3856);
        locals.var_beta1_i = assign2800_e3857;
        locals.var_beta1_i_rv = 0.0;

        let assign2810_e3861: f64 = (locals.var_bin_l * p.p616);
        let assign2810_e3862: f64 = (p.p615 + assign2810_e3861);
        let assign2810_e3865: f64 = (locals.var_bin_w * p.p617);
        let assign2810_e3866: f64 = (assign2810_e3862 + assign2810_e3865);
        let assign2810_e3869: f64 = (locals.var_bin_wl * p.p618);
        let assign2810_e3870: f64 = (assign2810_e3866 + assign2810_e3869);
        locals.var_beta2_i = assign2810_e3870;
        locals.var_beta2_i_rv = 0.0;

        let assign2820_e3874: f64 = (locals.var_bin_l * p.p620);
        let assign2820_e3875: f64 = (p.p619 + assign2820_e3874);
        let assign2820_e3878: f64 = (locals.var_bin_w * p.p621);
        let assign2820_e3879: f64 = (assign2820_e3875 + assign2820_e3878);
        let assign2820_e3882: f64 = (locals.var_bin_wl * p.p622);
        let assign2820_e3883: f64 = (assign2820_e3879 + assign2820_e3882);
        locals.var_lii_i = assign2820_e3883;
        locals.var_lii_i_rv = 0.0;

        let assign2830_e3887: f64 = (locals.var_bin_l * p.p624);
        let assign2830_e3888: f64 = (p.p623 + assign2830_e3887);
        let assign2830_e3891: f64 = (locals.var_bin_w * p.p625);
        let assign2830_e3892: f64 = (assign2830_e3888 + assign2830_e3891);
        let assign2830_e3895: f64 = (locals.var_bin_wl * p.p626);
        let assign2830_e3896: f64 = (assign2830_e3892 + assign2830_e3895);
        locals.var_sii0_i = assign2830_e3896;
        locals.var_sii0_i_rv = 0.0;

        let assign2840_e3900: f64 = (locals.var_bin_l * p.p628);
        let assign2840_e3901: f64 = (p.p627 + assign2840_e3900);
        let assign2840_e3904: f64 = (locals.var_bin_w * p.p629);
        let assign2840_e3905: f64 = (assign2840_e3901 + assign2840_e3904);
        let assign2840_e3908: f64 = (locals.var_bin_wl * p.p630);
        let assign2840_e3909: f64 = (assign2840_e3905 + assign2840_e3908);
        locals.var_sii1_i = assign2840_e3909;
        locals.var_sii1_i_rv = 0.0;

        let assign2850_e3913: f64 = (locals.var_bin_l * p.p632);
        let assign2850_e3914: f64 = (p.p631 + assign2850_e3913);
        let assign2850_e3917: f64 = (locals.var_bin_w * p.p633);
        let assign2850_e3918: f64 = (assign2850_e3914 + assign2850_e3917);
        let assign2850_e3921: f64 = (locals.var_bin_wl * p.p634);
        let assign2850_e3922: f64 = (assign2850_e3918 + assign2850_e3921);
        locals.var_sii2_i = assign2850_e3922;
        locals.var_sii2_i_rv = 0.0;

        let assign2860_e3926: f64 = (locals.var_bin_l * p.p636);
        let assign2860_e3927: f64 = (p.p635 + assign2860_e3926);
        let assign2860_e3930: f64 = (locals.var_bin_w * p.p637);
        let assign2860_e3931: f64 = (assign2860_e3927 + assign2860_e3930);
        let assign2860_e3934: f64 = (locals.var_bin_wl * p.p638);
        let assign2860_e3935: f64 = (assign2860_e3931 + assign2860_e3934);
        locals.var_siid_i = assign2860_e3935;
        locals.var_siid_i_rv = 0.0;

        let assign2870_e3939: f64 = (p.p597 * locals.var_bin_l);
        let assign2870_e3940: f64 = (p.p596 + assign2870_e3939);
        let assign2870_e3943: f64 = (p.p598 * locals.var_bin_w);
        let assign2870_e3944: f64 = (assign2870_e3940 + assign2870_e3943);
        let assign2870_e3947: f64 = (p.p599 * locals.var_bin_wl);
        let assign2870_e3948: f64 = (assign2870_e3944 + assign2870_e3947);
        locals.var_vdsatii0_i = assign2870_e3948;
        locals.var_vdsatii0_i_rv = 0.0;

        let assign2880_e3952: f64 = (locals.var_bin_l * p.p640);
        let assign2880_e3953: f64 = (p.p639 + assign2880_e3952);
        let assign2880_e3956: f64 = (locals.var_bin_w * p.p641);
        let assign2880_e3957: f64 = (assign2880_e3953 + assign2880_e3956);
        let assign2880_e3960: f64 = (locals.var_bin_wl * p.p642);
        let assign2880_e3961: f64 = (assign2880_e3957 + assign2880_e3960);
        locals.var_esatii_i = assign2880_e3961;
        locals.var_esatii_i_rv = 0.0;

        let assign2900_e3978: f64 = (locals.var_bin_l * p.p655);
        let assign2900_e3979: f64 = (p.p650 + assign2900_e3978);
        let assign2900_e3982: f64 = (locals.var_bin_w * p.p658);
        let assign2900_e3983: f64 = (assign2900_e3979 + assign2900_e3982);
        let assign2900_e3986: f64 = (locals.var_bin_wl * p.p661);
        let assign2900_e3987: f64 = (assign2900_e3983 + assign2900_e3986);
        locals.var_ebjtii_i = assign2900_e3987;
        locals.var_ebjtii_i_rv = 0.0;

        let assign2910_e3991: f64 = (locals.var_bin_l * p.p654);
        let assign2910_e3992: f64 = (p.p651 + assign2910_e3991);
        let assign2910_e3995: f64 = (locals.var_bin_w * p.p657);
        let assign2910_e3996: f64 = (assign2910_e3992 + assign2910_e3995);
        let assign2910_e3999: f64 = (locals.var_bin_wl * p.p660);
        let assign2910_e4000: f64 = (assign2910_e3996 + assign2910_e3999);
        locals.var_cbjtii_i = assign2910_e4000;
        locals.var_cbjtii_i_rv = 0.0;

        let assign2920_e4004: f64 = (locals.var_bin_l * p.p653);
        let assign2920_e4005: f64 = (p.p652 + assign2920_e4004);
        let assign2920_e4008: f64 = (locals.var_bin_w * p.p656);
        let assign2920_e4009: f64 = (assign2920_e4005 + assign2920_e4008);
        let assign2920_e4012: f64 = (locals.var_bin_wl * p.p659);
        let assign2920_e4013: f64 = (assign2920_e4009 + assign2920_e4012);
        locals.var_abjtii_i = assign2920_e4013;
        locals.var_abjtii_i_rv = 0.0;

        let assign2930_e4017: f64 = (locals.var_bin_l * p.p663);
        let assign2930_e4018: f64 = (p.p662 + assign2930_e4017);
        let assign2930_e4021: f64 = (locals.var_bin_w * p.p664);
        let assign2930_e4022: f64 = (assign2930_e4018 + assign2930_e4021);
        let assign2930_e4025: f64 = (locals.var_bin_wl * p.p665);
        let assign2930_e4026: f64 = (assign2930_e4022 + assign2930_e4025);
        locals.var_vbci_i = assign2930_e4026;
        locals.var_vbci_i_rv = 0.0;

        let assign2940_e4030: f64 = (locals.var_bin_l * p.p668);
        let assign2940_e4031: f64 = (p.p667 + assign2940_e4030);
        let assign2940_e4034: f64 = (locals.var_bin_w * p.p669);
        let assign2940_e4035: f64 = (assign2940_e4031 + assign2940_e4034);
        let assign2940_e4038: f64 = (locals.var_bin_wl * p.p670);
        let assign2940_e4039: f64 = (assign2940_e4035 + assign2940_e4038);
        locals.var_mbjtii_i = assign2940_e4039;
        locals.var_mbjtii_i_rv = 0.0;

        let assign2950_e4043: f64 = (locals.var_bin_l * p.p1362);
        let assign2950_e4044: f64 = (p.p1361 + assign2950_e4043);
        let assign2950_e4047: f64 = (locals.var_bin_w * p.p1363);
        let assign2950_e4048: f64 = (assign2950_e4044 + assign2950_e4047);
        let assign2950_e4051: f64 = (locals.var_bin_wl * p.p1364);
        let assign2950_e4052: f64 = (assign2950_e4048 + assign2950_e4051);
        locals.var_ub_i = assign2950_e4052;
        locals.var_ub_i_rv = 0.0;

        let assign2960_e4056: f64 = (locals.var_bin_l * p.p1366);
        let assign2960_e4057: f64 = (p.p1365 + assign2960_e4056);
        let assign2960_e4060: f64 = (locals.var_bin_w * p.p1367);
        let assign2960_e4061: f64 = (assign2960_e4057 + assign2960_e4060);
        let assign2960_e4064: f64 = (locals.var_bin_wl * p.p1368);
        let assign2960_e4065: f64 = (assign2960_e4061 + assign2960_e4064);
        locals.var_ubte_i = assign2960_e4065;
        locals.var_ubte_i_rv = 0.0;

        let assign2970_e4069: f64 = (locals.var_bin_l * p.p1370);
        let assign2970_e4070: f64 = (p.p1369 + assign2970_e4069);
        let assign2970_e4073: f64 = (locals.var_bin_w * p.p1371);
        let assign2970_e4074: f64 = (assign2970_e4070 + assign2970_e4073);
        let assign2970_e4077: f64 = (locals.var_bin_wl * p.p1372);
        let assign2970_e4078: f64 = (assign2970_e4074 + assign2970_e4077);
        locals.var_neff_i = assign2970_e4078;
        locals.var_neff_i_rv = 0.0;

        let assign2980_e4082: f64 = (p.p929 * locals.var_bin_l);
        let assign2980_e4083: f64 = (p.p928 + assign2980_e4082);
        let assign2980_e4086: f64 = (p.p930 * locals.var_bin_w);
        let assign2980_e4087: f64 = (assign2980_e4083 + assign2980_e4086);
        let assign2980_e4090: f64 = (p.p931 * locals.var_bin_wl);
        let assign2980_e4091: f64 = (assign2980_e4087 + assign2980_e4090);
        locals.var_xdif_i = assign2980_e4091;
        locals.var_xdif_i_rv = 0.0;

        let assign2990_e4095: f64 = (p.p934 * locals.var_bin_l);
        let assign2990_e4096: f64 = (p.p932 + assign2990_e4095);
        let assign2990_e4099: f64 = (p.p936 * locals.var_bin_w);
        let assign2990_e4100: f64 = (assign2990_e4096 + assign2990_e4099);
        let assign2990_e4103: f64 = (p.p938 * locals.var_bin_wl);
        let assign2990_e4104: f64 = (assign2990_e4100 + assign2990_e4103);
        locals.var_isdif_i = assign2990_e4104;
        locals.var_isdif_i_rv = 0.0;

        let assign3000_e4108: f64 = (p.p935 * locals.var_bin_l);
        let assign3000_e4109: f64 = (p.p933 + assign3000_e4108);
        let assign3000_e4112: f64 = (p.p937 * locals.var_bin_w);
        let assign3000_e4113: f64 = (assign3000_e4109 + assign3000_e4112);
        let assign3000_e4116: f64 = (p.p939 * locals.var_bin_wl);
        let assign3000_e4117: f64 = (assign3000_e4113 + assign3000_e4116);
        locals.var_iddif_i = assign3000_e4117;
        locals.var_iddif_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3010_e4121: f64 = (p.p941 * locals.var_bin_l);
        let assign3010_e4122: f64 = (p.p940 + assign3010_e4121);
        let assign3010_e4125: f64 = (p.p942 * locals.var_bin_w);
        let assign3010_e4126: f64 = (assign3010_e4122 + assign3010_e4125);
        let assign3010_e4129: f64 = (p.p943 * locals.var_bin_wl);
        let assign3010_e4130: f64 = (assign3010_e4126 + assign3010_e4129);
        locals.var_nrecf0_i = assign3010_e4130;
        locals.var_nrecf0_i_rv = 0.0;

        let assign3020_e4134: f64 = (p.p945 * locals.var_bin_l);
        let assign3020_e4135: f64 = (p.p944 + assign3020_e4134);
        let assign3020_e4138: f64 = (p.p946 * locals.var_bin_w);
        let assign3020_e4139: f64 = (assign3020_e4135 + assign3020_e4138);
        let assign3020_e4142: f64 = (p.p947 * locals.var_bin_wl);
        let assign3020_e4143: f64 = (assign3020_e4139 + assign3020_e4142);
        locals.var_nrecr0_i = assign3020_e4143;
        locals.var_nrecr0_i_rv = 0.0;

        let assign3030_e4147: f64 = (p.p949 * locals.var_bin_l);
        let assign3030_e4148: f64 = (p.p948 + assign3030_e4147);
        let assign3030_e4151: f64 = (p.p950 * locals.var_bin_w);
        let assign3030_e4152: f64 = (assign3030_e4148 + assign3030_e4151);
        let assign3030_e4155: f64 = (p.p951 * locals.var_bin_wl);
        let assign3030_e4156: f64 = (assign3030_e4152 + assign3030_e4155);
        locals.var_xrec_i = assign3030_e4156;
        locals.var_xrec_i_rv = 0.0;

        let assign3040_e4160: f64 = (p.p954 * locals.var_bin_l);
        let assign3040_e4161: f64 = (p.p952 + assign3040_e4160);
        let assign3040_e4164: f64 = (p.p956 * locals.var_bin_w);
        let assign3040_e4165: f64 = (assign3040_e4161 + assign3040_e4164);
        let assign3040_e4168: f64 = (p.p958 * locals.var_bin_wl);
        let assign3040_e4169: f64 = (assign3040_e4165 + assign3040_e4168);
        locals.var_isrec_i = assign3040_e4169;
        locals.var_isrec_i_rv = 0.0;

        let assign3050_e4173: f64 = (p.p955 * locals.var_bin_l);
        let assign3050_e4174: f64 = (p.p953 + assign3050_e4173);
        let assign3050_e4177: f64 = (p.p957 * locals.var_bin_w);
        let assign3050_e4178: f64 = (assign3050_e4174 + assign3050_e4177);
        let assign3050_e4181: f64 = (p.p959 * locals.var_bin_wl);
        let assign3050_e4182: f64 = (assign3050_e4178 + assign3050_e4181);
        locals.var_idrec_i = assign3050_e4182;
        locals.var_idrec_i_rv = 0.0;

        let assign3060_e4186: f64 = (p.p962 * locals.var_bin_l);
        let assign3060_e4187: f64 = (p.p960 + assign3060_e4186);
        let assign3060_e4190: f64 = (p.p964 * locals.var_bin_w);
        let assign3060_e4191: f64 = (assign3060_e4187 + assign3060_e4190);
        let assign3060_e4194: f64 = (p.p966 * locals.var_bin_wl);
        let assign3060_e4195: f64 = (assign3060_e4191 + assign3060_e4194);
        locals.var_ntrecf_i = assign3060_e4195;
        locals.var_ntrecf_i_rv = 0.0;

        let assign3070_e4199: f64 = (p.p963 * locals.var_bin_l);
        let assign3070_e4200: f64 = (p.p961 + assign3070_e4199);
        let assign3070_e4203: f64 = (p.p965 * locals.var_bin_w);
        let assign3070_e4204: f64 = (assign3070_e4200 + assign3070_e4203);
        let assign3070_e4207: f64 = (p.p967 * locals.var_bin_wl);
        let assign3070_e4208: f64 = (assign3070_e4204 + assign3070_e4207);
        locals.var_ntrecr_i = assign3070_e4208;
        locals.var_ntrecr_i_rv = 0.0;

        let assign3080_e4212: f64 = (p.p970 * locals.var_bin_l);
        let assign3080_e4213: f64 = (p.p968 + assign3080_e4212);
        let assign3080_e4216: f64 = (p.p972 * locals.var_bin_w);
        let assign3080_e4217: f64 = (assign3080_e4213 + assign3080_e4216);
        let assign3080_e4220: f64 = (p.p974 * locals.var_bin_wl);
        let assign3080_e4221: f64 = (assign3080_e4217 + assign3080_e4220);
        locals.var_istun_i = assign3080_e4221;
        locals.var_istun_i_rv = 0.0;

        let assign3090_e4225: f64 = (p.p971 * locals.var_bin_l);
        let assign3090_e4226: f64 = (p.p969 + assign3090_e4225);
        let assign3090_e4229: f64 = (p.p973 * locals.var_bin_w);
        let assign3090_e4230: f64 = (assign3090_e4226 + assign3090_e4229);
        let assign3090_e4233: f64 = (p.p975 * locals.var_bin_wl);
        let assign3090_e4234: f64 = (assign3090_e4230 + assign3090_e4233);
        locals.var_idtun_i = assign3090_e4234;
        locals.var_idtun_i_rv = 0.0;

        let assign3100_e4238: f64 = (p.p978 * locals.var_bin_l);
        let assign3100_e4239: f64 = (p.p976 + assign3100_e4238);
        let assign3100_e4242: f64 = (p.p980 * locals.var_bin_w);
        let assign3100_e4243: f64 = (assign3100_e4239 + assign3100_e4242);
        let assign3100_e4246: f64 = (p.p982 * locals.var_bin_wl);
        let assign3100_e4247: f64 = (assign3100_e4243 + assign3100_e4246);
        locals.var_xtun_i = assign3100_e4247;
        locals.var_xtun_i_rv = 0.0;

        let assign3110_e4251: f64 = (p.p979 * locals.var_bin_l);
        let assign3110_e4252: f64 = (p.p977 + assign3110_e4251);
        let assign3110_e4255: f64 = (p.p981 * locals.var_bin_w);
        let assign3110_e4256: f64 = (assign3110_e4252 + assign3110_e4255);
        let assign3110_e4259: f64 = (p.p983 * locals.var_bin_wl);
        let assign3110_e4260: f64 = (assign3110_e4256 + assign3110_e4259);
        locals.var_xtund_i = assign3110_e4260;
        locals.var_xtund_i_rv = 0.0;

        let assign3120_e4264: f64 = (p.p986 * locals.var_bin_l);
        let assign3120_e4265: f64 = (p.p984 + assign3120_e4264);
        let assign3120_e4268: f64 = (p.p988 * locals.var_bin_w);
        let assign3120_e4269: f64 = (assign3120_e4265 + assign3120_e4268);
        let assign3120_e4272: f64 = (p.p990 * locals.var_bin_wl);
        let assign3120_e4273: f64 = (assign3120_e4269 + assign3120_e4272);
        locals.var_ntun_i = assign3120_e4273;
        locals.var_ntun_i_rv = 0.0;

        let assign3130_e4277: f64 = (p.p987 * locals.var_bin_l);
        let assign3130_e4278: f64 = (p.p985 + assign3130_e4277);
        let assign3130_e4281: f64 = (p.p989 * locals.var_bin_w);
        let assign3130_e4282: f64 = (assign3130_e4278 + assign3130_e4281);
        let assign3130_e4285: f64 = (p.p991 * locals.var_bin_wl);
        let assign3130_e4286: f64 = (assign3130_e4282 + assign3130_e4285);
        locals.var_ntund_i = assign3130_e4286;
        locals.var_ntund_i_rv = 0.0;

        let assign3140_e4290: f64 = (p.p994 * locals.var_bin_l);
        let assign3140_e4291: f64 = (p.p992 + assign3140_e4290);
        let assign3140_e4294: f64 = (p.p996 * locals.var_bin_w);
        let assign3140_e4295: f64 = (assign3140_e4291 + assign3140_e4294);
        let assign3140_e4298: f64 = (p.p998 * locals.var_bin_wl);
        let assign3140_e4299: f64 = (assign3140_e4295 + assign3140_e4298);
        locals.var_vtun0_i = assign3140_e4299;
        locals.var_vtun0_i_rv = 0.0;

        let assign3150_e4303: f64 = (p.p995 * locals.var_bin_l);
        let assign3150_e4304: f64 = (p.p993 + assign3150_e4303);
        let assign3150_e4307: f64 = (p.p997 * locals.var_bin_w);
        let assign3150_e4308: f64 = (assign3150_e4304 + assign3150_e4307);
        let assign3150_e4311: f64 = (p.p999 * locals.var_bin_wl);
        let assign3150_e4312: f64 = (assign3150_e4308 + assign3150_e4311);
        locals.var_vtun0d_i = assign3150_e4312;
        locals.var_vtun0d_i_rv = 0.0;

        let assign3160_e4316: f64 = (p.p1002 * locals.var_bin_l);
        let assign3160_e4317: f64 = (p.p1000 + assign3160_e4316);
        let assign3160_e4320: f64 = (p.p1004 * locals.var_bin_w);
        let assign3160_e4321: f64 = (assign3160_e4317 + assign3160_e4320);
        let assign3160_e4324: f64 = (p.p1006 * locals.var_bin_wl);
        let assign3160_e4325: f64 = (assign3160_e4321 + assign3160_e4324);
        locals.var_vrec0_i = assign3160_e4325;
        locals.var_vrec0_i_rv = 0.0;

        let assign3170_e4329: f64 = (p.p1003 * locals.var_bin_l);
        let assign3170_e4330: f64 = (p.p1001 + assign3170_e4329);
        let assign3170_e4333: f64 = (p.p1005 * locals.var_bin_w);
        let assign3170_e4334: f64 = (assign3170_e4330 + assign3170_e4333);
        let assign3170_e4337: f64 = (p.p1007 * locals.var_bin_wl);
        let assign3170_e4338: f64 = (assign3170_e4334 + assign3170_e4337);
        locals.var_vrec0d_i = assign3170_e4338;
        locals.var_vrec0d_i_rv = 0.0;

        let assign3180_e4342: f64 = (p.p556 * locals.var_bin_l);
        let assign3180_e4343: f64 = (p.p555 + assign3180_e4342);
        let assign3180_e4346: f64 = (p.p557 * locals.var_bin_w);
        let assign3180_e4347: f64 = (assign3180_e4343 + assign3180_e4346);
        let assign3180_e4350: f64 = (p.p558 * locals.var_bin_wl);
        let assign3180_e4351: f64 = (assign3180_e4347 + assign3180_e4350);
        locals.var_vabjt_i = assign3180_e4351;
        locals.var_vabjt_i_rv = 0.0;

        let assign3190_e4355: f64 = (p.p560 * locals.var_bin_l);
        let assign3190_e4356: f64 = (p.p559 + assign3190_e4355);
        let assign3190_e4359: f64 = (p.p561 * locals.var_bin_w);
        let assign3190_e4360: f64 = (assign3190_e4356 + assign3190_e4359);
        let assign3190_e4363: f64 = (p.p562 * locals.var_bin_wl);
        let assign3190_e4364: f64 = (assign3190_e4360 + assign3190_e4363);
        locals.var_aely_i = assign3190_e4364;
        locals.var_aely_i_rv = 0.0;

        let assign3200_e4368: f64 = (locals.var_bin_l * p.p565);
        let assign3200_e4369: f64 = (p.p563 + assign3200_e4368);
        let assign3200_e4372: f64 = (locals.var_bin_w * p.p567);
        let assign3200_e4373: f64 = (assign3200_e4369 + assign3200_e4372);
        let assign3200_e4376: f64 = (p.p569 * locals.var_bin_wl);
        let assign3200_e4377: f64 = (assign3200_e4373 + assign3200_e4376);
        locals.var_ahli_i = assign3200_e4377;
        locals.var_ahli_i_rv = 0.0;

        let assign3210_e4381: f64 = (locals.var_bin_l * p.p566);
        let assign3210_e4382: f64 = (p.p564 + assign3210_e4381);
        let assign3210_e4385: f64 = (locals.var_bin_w * p.p568);
        let assign3210_e4386: f64 = (assign3210_e4382 + assign3210_e4385);
        let assign3210_e4389: f64 = (p.p570 * locals.var_bin_wl);
        let assign3210_e4390: f64 = (assign3210_e4386 + assign3210_e4389);
        locals.var_ahlid_i = assign3210_e4390;
        locals.var_ahlid_i_rv = 0.0;

        let assign3220_e4394: f64 = (p.p572 * locals.var_bin_l);
        let assign3220_e4395: f64 = (p.p571 + assign3220_e4394);
        let assign3220_e4398: f64 = (p.p573 * locals.var_bin_w);
        let assign3220_e4399: f64 = (assign3220_e4395 + assign3220_e4398);
        let assign3220_e4402: f64 = (p.p574 * locals.var_bin_wl);
        let assign3220_e4403: f64 = (assign3220_e4399 + assign3220_e4402);
        locals.var_xbjt_i = assign3220_e4403;
        locals.var_xbjt_i_rv = 0.0;

        let assign3230_e4407: f64 = (p.p576 * locals.var_bin_l);
        let assign3230_e4408: f64 = (p.p575 + assign3230_e4407);
        let assign3230_e4411: f64 = (p.p577 * locals.var_bin_w);
        let assign3230_e4412: f64 = (assign3230_e4408 + assign3230_e4411);
        let assign3230_e4415: f64 = (p.p578 * locals.var_bin_wl);
        let assign3230_e4416: f64 = (assign3230_e4412 + assign3230_e4415);
        locals.var_ndiode_i = assign3230_e4416;
        locals.var_ndiode_i_rv = 0.0;

        let assign3240_e4420: f64 = (p.p582 * locals.var_bin_l);
        let assign3240_e4421: f64 = (p.p579 + assign3240_e4420);
        let assign3240_e4424: f64 = (p.p581 * locals.var_bin_w);
        let assign3240_e4425: f64 = (assign3240_e4421 + assign3240_e4424);
        let assign3240_e4428: f64 = (p.p580 * locals.var_bin_wl);
        let assign3240_e4429: f64 = (assign3240_e4425 + assign3240_e4428);
        locals.var_isbjt_i = assign3240_e4429;
        locals.var_isbjt_i_rv = 0.0;

        let assign3250_e4433: f64 = (p.p584 * locals.var_bin_l);
        let assign3250_e4434: f64 = (p.p583 + assign3250_e4433);
        let assign3250_e4437: f64 = (p.p585 * locals.var_bin_w);
        let assign3250_e4438: f64 = (assign3250_e4434 + assign3250_e4437);
        let assign3250_e4441: f64 = (p.p586 * locals.var_bin_wl);
        let assign3250_e4442: f64 = (assign3250_e4438 + assign3250_e4441);
        locals.var_idbjt_i = assign3250_e4442;
        locals.var_idbjt_i_rv = 0.0;

        let assign3260_e4446: f64 = (p.p588 * locals.var_bin_l);
        let assign3260_e4447: f64 = (p.p587 + assign3260_e4446);
        let assign3260_e4450: f64 = (p.p590 * locals.var_bin_w);
        let assign3260_e4451: f64 = (assign3260_e4447 + assign3260_e4450);
        let assign3260_e4454: f64 = (p.p592 * locals.var_bin_wl);
        let assign3260_e4455: f64 = (assign3260_e4451 + assign3260_e4454);
        locals.var_nbjt_i = assign3260_e4455;
        locals.var_nbjt_i_rv = 0.0;

        let assign3270_e4459: f64 = (p.p589 * locals.var_bin_l);
        let assign3270_e4460: f64 = (p.p594 + assign3270_e4459);
        let assign3270_e4463: f64 = (p.p591 * locals.var_bin_w);
        let assign3270_e4464: f64 = (assign3270_e4460 + assign3270_e4463);
        let assign3270_e4467: f64 = (p.p593 * locals.var_bin_wl);
        let assign3270_e4468: f64 = (assign3270_e4464 + assign3270_e4467);
        locals.var_lbjt0_i = assign3270_e4468;
        locals.var_lbjt0_i_rv = 0.0;

        let assign3280_e4472: f64 = (p.p922 * locals.var_bin_l);
        let assign3280_e4473: f64 = (p.p921 + assign3280_e4472);
        let assign3280_e4476: f64 = (p.p923 * locals.var_bin_w);
        let assign3280_e4477: f64 = (assign3280_e4473 + assign3280_e4476);
        let assign3280_e4480: f64 = (p.p924 * locals.var_bin_wl);
        let assign3280_e4481: f64 = (assign3280_e4477 + assign3280_e4480);
        locals.var_ndif_i = assign3280_e4481;
        locals.var_ndif_i_rv = 0.0;

        let assign3290_e4485: f64 = (locals.var_bin_l * p.p1126);
        let assign3290_e4486: f64 = (p.p1125 + assign3290_e4485);
        let assign3290_e4489: f64 = (locals.var_bin_w * p.p1127);
        let assign3290_e4490: f64 = (assign3290_e4486 + assign3290_e4489);
        let assign3290_e4493: f64 = (locals.var_bin_wl * p.p1128);
        let assign3290_e4494: f64 = (assign3290_e4490 + assign3290_e4493);
        locals.var_kvth0we_i = assign3290_e4494;
        locals.var_kvth0we_i_rv = 0.0;

        let assign3300_e4498: f64 = (locals.var_bin_l * p.p1130);
        let assign3300_e4499: f64 = (p.p1129 + assign3300_e4498);
        let assign3300_e4502: f64 = (locals.var_bin_w * p.p1131);
        let assign3300_e4503: f64 = (assign3300_e4499 + assign3300_e4502);
        let assign3300_e4506: f64 = (locals.var_bin_wl * p.p1132);
        let assign3300_e4507: f64 = (assign3300_e4503 + assign3300_e4506);
        locals.var_k2we_i = assign3300_e4507;
        locals.var_k2we_i_rv = 0.0;

        let assign3310_e4511: f64 = (locals.var_bin_l * p.p1134);
        let assign3310_e4512: f64 = (p.p1133 + assign3310_e4511);
        let assign3310_e4515: f64 = (locals.var_bin_w * p.p1135);
        let assign3310_e4516: f64 = (assign3310_e4512 + assign3310_e4515);
        let assign3310_e4519: f64 = (locals.var_bin_wl * p.p1136);
        let assign3310_e4520: f64 = (assign3310_e4516 + assign3310_e4519);
        locals.var_ku0we_i = assign3310_e4520;
        locals.var_ku0we_i_rv = 0.0;

        let assign3320_e4524: f64 = (locals.var_bin_l * p.p802);
        let assign3320_e4525: f64 = (p.p799 + assign3320_e4524);
        let assign3320_e4528: f64 = (locals.var_bin_w * p.p803);
        let assign3320_e4529: f64 = (assign3320_e4525 + assign3320_e4528);
        let assign3320_e4532: f64 = (locals.var_bin_wl * p.p804);
        let assign3320_e4533: f64 = (assign3320_e4529 + assign3320_e4532);
        locals.var_agidl_i = assign3320_e4533;
        locals.var_agidl_i_rv = 0.0;

        let assign3330_e4537: f64 = (locals.var_bin_l * p.p807);
        let assign3330_e4538: f64 = (p.p805 + assign3330_e4537);
        let assign3330_e4541: f64 = (locals.var_bin_w * p.p808);
        let assign3330_e4542: f64 = (assign3330_e4538 + assign3330_e4541);
        let assign3330_e4545: f64 = (locals.var_bin_wl * p.p809);
        let assign3330_e4546: f64 = (assign3330_e4542 + assign3330_e4545);
        locals.var_bgidl_i = assign3330_e4546;
        locals.var_bgidl_i_rv = 0.0;

        let assign3340_e4550: f64 = (p.p810 * locals.var_bin_l);
        let assign3340_e4551: f64 = (p.p806 + assign3340_e4550);
        let assign3340_e4554: f64 = (p.p811 * locals.var_bin_w);
        let assign3340_e4555: f64 = (assign3340_e4551 + assign3340_e4554);
        let assign3340_e4558: f64 = (p.p812 * locals.var_bin_wl);
        let assign3340_e4559: f64 = (assign3340_e4555 + assign3340_e4558);
        locals.var_bgidl1_i = assign3340_e4559;
        locals.var_bgidl1_i_rv = 0.0;

        let assign3350_e4563: f64 = (locals.var_bin_l * p.p814);
        let assign3350_e4564: f64 = (p.p813 + assign3350_e4563);
        let assign3350_e4567: f64 = (locals.var_bin_w * p.p815);
        let assign3350_e4568: f64 = (assign3350_e4564 + assign3350_e4567);
        let assign3350_e4571: f64 = (locals.var_bin_wl * p.p816);
        let assign3350_e4572: f64 = (assign3350_e4568 + assign3350_e4571);
        locals.var_cgidl_i = assign3350_e4572;
        locals.var_cgidl_i_rv = 0.0;

        let assign3360_e4576: f64 = (locals.var_bin_l * p.p818);
        let assign3360_e4577: f64 = (p.p817 + assign3360_e4576);
        let assign3360_e4580: f64 = (locals.var_bin_w * p.p819);
        let assign3360_e4581: f64 = (assign3360_e4577 + assign3360_e4580);
        let assign3360_e4584: f64 = (locals.var_bin_wl * p.p820);
        let assign3360_e4585: f64 = (assign3360_e4581 + assign3360_e4584);
        locals.var_egidl_i = assign3360_e4585;
        locals.var_egidl_i_rv = 0.0;

        let assign3370_e4589: f64 = (locals.var_bin_l * p.p824);
        let assign3370_e4590: f64 = (p.p821 + assign3370_e4589);
        let assign3370_e4593: f64 = (locals.var_bin_w * p.p825);
        let assign3370_e4594: f64 = (assign3370_e4590 + assign3370_e4593);
        let assign3370_e4597: f64 = (locals.var_bin_wl * p.p826);
        let assign3370_e4598: f64 = (assign3370_e4594 + assign3370_e4597);
        locals.var_agisl_i = assign3370_e4598;
        locals.var_agisl_i_rv = 0.0;

        let assign3380_e4602: f64 = (locals.var_bin_l * p.p829);
        let assign3380_e4603: f64 = (p.p827 + assign3380_e4602);
        let assign3380_e4606: f64 = (locals.var_bin_w * p.p830);
        let assign3380_e4607: f64 = (assign3380_e4603 + assign3380_e4606);
        let assign3380_e4610: f64 = (locals.var_bin_wl * p.p831);
        let assign3380_e4611: f64 = (assign3380_e4607 + assign3380_e4610);
        locals.var_bgisl_i = assign3380_e4611;
        locals.var_bgisl_i_rv = 0.0;

        let assign3390_e4615: f64 = (p.p832 * locals.var_bin_l);
        let assign3390_e4616: f64 = (p.p828 + assign3390_e4615);
        let assign3390_e4619: f64 = (p.p833 * locals.var_bin_w);
        let assign3390_e4620: f64 = (assign3390_e4616 + assign3390_e4619);
        let assign3390_e4623: f64 = (p.p834 * locals.var_bin_wl);
        let assign3390_e4624: f64 = (assign3390_e4620 + assign3390_e4623);
        locals.var_bgisl1_i = assign3390_e4624;
        locals.var_bgisl1_i_rv = 0.0;

        let assign3400_e4628: f64 = (locals.var_bin_l * p.p836);
        let assign3400_e4629: f64 = (p.p835 + assign3400_e4628);
        let assign3400_e4632: f64 = (locals.var_bin_w * p.p837);
        let assign3400_e4633: f64 = (assign3400_e4629 + assign3400_e4632);
        let assign3400_e4636: f64 = (locals.var_bin_wl * p.p838);
        let assign3400_e4637: f64 = (assign3400_e4633 + assign3400_e4636);
        locals.var_cgisl_i = assign3400_e4637;
        locals.var_cgisl_i_rv = 0.0;

        let assign3410_e4641: f64 = (locals.var_bin_l * p.p840);
        let assign3410_e4642: f64 = (p.p839 + assign3410_e4641);
        let assign3410_e4645: f64 = (locals.var_bin_w * p.p841);
        let assign3410_e4646: f64 = (assign3410_e4642 + assign3410_e4645);
        let assign3410_e4649: f64 = (locals.var_bin_wl * p.p842);
        let assign3410_e4650: f64 = (assign3410_e4646 + assign3410_e4649);
        locals.var_egisl_i = assign3410_e4650;
        locals.var_egisl_i_rv = 0.0;

        let assign3420_e4654: f64 = (locals.var_bin_l * p.p856);
        let assign3420_e4655: f64 = (p.p855 + assign3420_e4654);
        let assign3420_e4658: f64 = (locals.var_bin_w * p.p857);
        let assign3420_e4659: f64 = (assign3420_e4655 + assign3420_e4658);
        let assign3420_e4662: f64 = (locals.var_bin_wl * p.p858);
        let assign3420_e4663: f64 = (assign3420_e4659 + assign3420_e4662);
        locals.var_rgisl_i = assign3420_e4663;
        locals.var_rgisl_i_rv = 0.0;

        let assign3430_e4667: f64 = (locals.var_bin_l * p.p844);
        let assign3430_e4668: f64 = (p.p843 + assign3430_e4667);
        let assign3430_e4671: f64 = (locals.var_bin_w * p.p845);
        let assign3430_e4672: f64 = (assign3430_e4668 + assign3430_e4671);
        let assign3430_e4675: f64 = (locals.var_bin_wl * p.p846);
        let assign3430_e4676: f64 = (assign3430_e4672 + assign3430_e4675);
        locals.var_rgidl_i = assign3430_e4676;
        locals.var_rgidl_i_rv = 0.0;

        let assign3440_e4680: f64 = (locals.var_bin_l * p.p860);
        let assign3440_e4681: f64 = (p.p859 + assign3440_e4680);
        let assign3440_e4684: f64 = (locals.var_bin_w * p.p861);
        let assign3440_e4685: f64 = (assign3440_e4681 + assign3440_e4684);
        let assign3440_e4688: f64 = (locals.var_bin_wl * p.p862);
        let assign3440_e4689: f64 = (assign3440_e4685 + assign3440_e4688);
        locals.var_kgisl_i = assign3440_e4689;
        locals.var_kgisl_i_rv = 0.0;

        let assign3450_e4693: f64 = (locals.var_bin_l * p.p848);
        let assign3450_e4694: f64 = (p.p847 + assign3450_e4693);
        let assign3450_e4697: f64 = (locals.var_bin_w * p.p849);
        let assign3450_e4698: f64 = (assign3450_e4694 + assign3450_e4697);
        let assign3450_e4701: f64 = (locals.var_bin_wl * p.p850);
        let assign3450_e4702: f64 = (assign3450_e4698 + assign3450_e4701);
        locals.var_kgidl_i = assign3450_e4702;
        locals.var_kgidl_i_rv = 0.0;

        let assign3460_e4706: f64 = (locals.var_bin_l * p.p864);
        let assign3460_e4707: f64 = (p.p863 + assign3460_e4706);
        let assign3460_e4710: f64 = (locals.var_bin_w * p.p865);
        let assign3460_e4711: f64 = (assign3460_e4707 + assign3460_e4710);
        let assign3460_e4714: f64 = (locals.var_bin_wl * p.p866);
        let assign3460_e4715: f64 = (assign3460_e4711 + assign3460_e4714);
        locals.var_fgisl_i = assign3460_e4715;
        locals.var_fgisl_i_rv = 0.0;

        let assign3470_e4719: f64 = (locals.var_bin_l * p.p852);
        let assign3470_e4720: f64 = (p.p851 + assign3470_e4719);
        let assign3470_e4723: f64 = (locals.var_bin_w * p.p853);
        let assign3470_e4724: f64 = (assign3470_e4720 + assign3470_e4723);
        let assign3470_e4727: f64 = (locals.var_bin_wl * p.p854);
        let assign3470_e4728: f64 = (assign3470_e4724 + assign3470_e4727);
        locals.var_fgidl_i = assign3470_e4728;
        locals.var_fgidl_i_rv = 0.0;

        let assign3480_e4732: f64 = (locals.var_bin_l * p.p1033);
        let assign3480_e4733: f64 = (p.p1032 + assign3480_e4732);
        let assign3480_e4736: f64 = (locals.var_bin_w * p.p1034);
        let assign3480_e4737: f64 = (assign3480_e4733 + assign3480_e4736);
        let assign3480_e4740: f64 = (locals.var_bin_wl * p.p1035);
        let assign3480_e4741: f64 = (assign3480_e4737 + assign3480_e4740);
        locals.var_ute_i = assign3480_e4741;
        locals.var_ute_i_rv = 0.0;

        let assign3490_e4745: f64 = (locals.var_bin_l * p.p1038);
        let assign3490_e4746: f64 = (p.p1037 + assign3490_e4745);
        let assign3490_e4749: f64 = (locals.var_bin_w * p.p1039);
        let assign3490_e4750: f64 = (assign3490_e4746 + assign3490_e4749);
        let assign3490_e4753: f64 = (locals.var_bin_wl * p.p1040);
        let assign3490_e4754: f64 = (assign3490_e4750 + assign3490_e4753);
        locals.var_ua1_i = assign3490_e4754;
        locals.var_ua1_i_rv = 0.0;

        let assign3500_e4758: f64 = (locals.var_bin_l * p.p1043);
        let assign3500_e4759: f64 = (p.p1042 + assign3500_e4758);
        let assign3500_e4762: f64 = (locals.var_bin_w * p.p1044);
        let assign3500_e4763: f64 = (assign3500_e4759 + assign3500_e4762);
        let assign3500_e4766: f64 = (locals.var_bin_wl * p.p1045);
        let assign3500_e4767: f64 = (assign3500_e4763 + assign3500_e4766);
        locals.var_uc1_i = assign3500_e4767;
        locals.var_uc1_i_rv = 0.0;

        let assign3510_e4771: f64 = (locals.var_bin_l * p.p1047);
        let assign3510_e4772: f64 = (p.p1046 + assign3510_e4771);
        let assign3510_e4775: f64 = (locals.var_bin_w * p.p1048);
        let assign3510_e4776: f64 = (assign3510_e4772 + assign3510_e4775);
        let assign3510_e4779: f64 = (locals.var_bin_wl * p.p1049);
        let assign3510_e4780: f64 = (assign3510_e4776 + assign3510_e4779);
        locals.var_ud1_i = assign3510_e4780;
        locals.var_ud1_i_rv = 0.0;

        let assign3520_e4784: f64 = (locals.var_bin_l * p.p1052);
        let assign3520_e4785: f64 = (p.p1051 + assign3520_e4784);
        let assign3520_e4788: f64 = (locals.var_bin_w * p.p1053);
        let assign3520_e4789: f64 = (assign3520_e4785 + assign3520_e4788);
        let assign3520_e4792: f64 = (locals.var_bin_wl * p.p1054);
        let assign3520_e4793: f64 = (assign3520_e4789 + assign3520_e4792);
        locals.var_eu1_i = assign3520_e4793;
        locals.var_eu1_i_rv = 0.0;

        let assign3530_e4797: f64 = (locals.var_bin_l * p.p1056);
        let assign3530_e4798: f64 = (p.p1055 + assign3530_e4797);
        let assign3530_e4801: f64 = (locals.var_bin_w * p.p1057);
        let assign3530_e4802: f64 = (assign3530_e4798 + assign3530_e4801);
        let assign3530_e4805: f64 = (locals.var_bin_wl * p.p1058);
        let assign3530_e4806: f64 = (assign3530_e4802 + assign3530_e4805);
        locals.var_ucste_i = assign3530_e4806;
        locals.var_ucste_i_rv = 0.0;

        let assign3540_e4810: f64 = (locals.var_bin_l * p.p1061);
        let assign3540_e4811: f64 = (p.p1060 + assign3540_e4810);
        let assign3540_e4814: f64 = (locals.var_bin_w * p.p1062);
        let assign3540_e4815: f64 = (assign3540_e4811 + assign3540_e4814);
        let assign3540_e4818: f64 = (locals.var_bin_wl * p.p1063);
        let assign3540_e4819: f64 = (assign3540_e4815 + assign3540_e4818);
        locals.var_prt_i = assign3540_e4819;
        locals.var_prt_i_rv = 0.0;

        let assign3550_e4823: f64 = (locals.var_bin_l * p.p1065);
        let assign3550_e4824: f64 = (p.p1064 + assign3550_e4823);
        let assign3550_e4827: f64 = (locals.var_bin_w * p.p1066);
        let assign3550_e4828: f64 = (assign3550_e4824 + assign3550_e4827);
        let assign3550_e4831: f64 = (locals.var_bin_wl * p.p1067);
        let assign3550_e4832: f64 = (assign3550_e4828 + assign3550_e4831);
        locals.var_at_i = assign3550_e4832;
        locals.var_at_i_rv = 0.0;

        let assign3560_e4836: f64 = (locals.var_bin_l * p.p1071);
        let assign3560_e4837: f64 = (p.p1070 + assign3560_e4836);
        let assign3560_e4840: f64 = (locals.var_bin_w * p.p1072);
        let assign3560_e4841: f64 = (assign3560_e4837 + assign3560_e4840);
        let assign3560_e4844: f64 = (locals.var_bin_wl * p.p1073);
        let assign3560_e4845: f64 = (assign3560_e4841 + assign3560_e4844);
        locals.var_ptwgt_i = assign3560_e4845;
        locals.var_ptwgt_i_rv = 0.0;

    }
}
